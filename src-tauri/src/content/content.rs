use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use mime_guess::from_path;
use thiserror::Error;
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone)]
struct ContentState {
    content_dir: PathBuf,
}

pub async fn run_content_localhost(data_path: PathBuf) {
    let content_dir = data_path.join("Content");
    let content_dir = match content_dir.canonicalize() {
        Ok(path) => path,
        Err(_) => content_dir,
    };

    println!("Content directory: {}", content_dir.display());

    if !content_dir.exists() {
        if let Err(e) = tokio::fs::create_dir_all(&content_dir).await {
            eprintln!("Failed to create content directory: {}", e);
            return;
        }
    }

    let state = ContentState { content_dir };
    
    // Set up CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE]);
    
    let app = Router::new()
        .route("/*path", get(serve_file))
        .layer(cors)
        .with_state(state);

    match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => {
            println!("Content server running on http://localhost:3000");
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Content server error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to start content server: {}", e);
        }
    }
}

async fn serve_file(
    Path(path): Path<String>,
    State(state): State<ContentState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut path_buf = state.content_dir.clone();
    
    for part in path.split('/') {
        if part == ".." || part.contains('\\') {
            return Err((StatusCode::FORBIDDEN, "Path traversal not allowed".to_string()));
        }
        path_buf.push(part);
    }

    let metadata = match tokio::fs::metadata(&path_buf).await {
        Ok(m) => m,
        Err(e) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", e))),
    };

    if metadata.is_dir() {
        return Err((StatusCode::FORBIDDEN, "Directory listing not allowed".to_string()));
    }

    serve_file_impl(path_buf).await
}



async fn serve_file_impl(
    file_path: PathBuf,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file = File::open(&file_path).await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("File not found: {}", e)))?;

    let stream = ReaderStream::new(file);
    let mime_type = from_path(&file_path).first_or_octet_stream();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, mime_type.to_string().parse().unwrap());
    headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());

    if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
        headers.insert(header::CONTENT_LENGTH, metadata.len().into());
    }

    Ok((headers, axum::body::Body::from_stream(stream)))
}