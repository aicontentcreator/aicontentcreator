use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{Manager, AppHandle};
use tokio::time::{sleep, Duration};
use utility::system::time::{format_timestamp_to_gmt_string, timestamp_now};

////////
// should be in a separate module
use tauri::{command, State};
use tokio::sync::Mutex;
use daemon::daemon_proxy::daemon_proxy::DaemonProxy;
pub struct AppState {
    pub shared_app_daemon_proxy: Mutex<DaemonProxy>,
}
//


#[derive(Deserialize)]
pub struct FrontendRequest {
    action: String,
    payload: Value,
}

#[derive(Serialize)]
pub struct BackendResponse {
    status: String,
    description: String,
    data: Option<Value>,
}

// 🔹 Greet function
fn greet(payload: &Value) -> BackendResponse {
    let name = payload["name"].as_str().unwrap_or("Unknown");
    BackendResponse {
        status: "success".to_string(),
        description: format!("Hello, {}!", name),
        data: None,
    }
}

// 🔹 Calculate function
fn calculate(payload: &Value) -> BackendResponse {
    let a = payload["a"].as_f64().unwrap_or(0.0);
    let b = payload["b"].as_f64().unwrap_or(0.0);
    BackendResponse {
        status: "success".to_string(),
        description: "Calculation complete".to_string(),
        data: Some(serde_json::json!({ "result": a + b })),
    }
}

// 🔹 Async Get Time function
/*
async fn get_time_string() -> BackendResponse {
    sleep(Duration::from_secs(1)).await; // Simulate async delay
    let current_time = format_timestamp_to_gmt_string(timestamp_now());
    
    BackendResponse {
        status: "success".to_string(),
        description: "Current time fetched".to_string(),
        data: Some(serde_json::json!({ "time": current_time })),
    }
}
*/
/*
// 🔹 Async Get Time function
async fn get_daemon_operational_situation(app_daemon_proxy:DaemonProxy) -> BackendResponse {
    //sleep(Duration::from_secs(1)).await; // Simulate async delay
    //let current_time = format_timestamp_to_gmt_string(timestamp_now());
    
    println!("app_daemon_proxy.get_operational_situation().await.unwrap() {}", app_daemon_proxy.get_operational_situation().await.unwrap());
    BackendResponse {
        status: "success".to_string(),
        description: "Current time fetched".to_string(),
        data: Some(serde_json::json!({ "daemon_operational_situation": "current_time" })),
    }
}
*/
async fn get_daemon_operational_situation(app_daemon_proxy: DaemonProxy) -> BackendResponse {
    match app_daemon_proxy.get_operational_situation().await {
        Ok(tmp_operational_situation) => {
            //app_daemon_proxy.get_operational_situation().await.unwrap()
            println!("app_daemon_proxy.get_operational_situation().await.unwrap() {}",tmp_operational_situation);
            BackendResponse {
                status: "success".to_string(),
                description: "Daemon status fetched".to_string(),
                data: Some(serde_json::json!({ "daemon_operational_situation": tmp_operational_situation })),
            }
        },
        Err(e) => {
            println!("Error fetching daemon status: {:?}", e);
            BackendResponse {
                status: "error".to_string(),
                description: "Failed to fetch daemon status".to_string(),
                data: None,
            }
        }
    }
}


/*
#[command]
fn greet(name: String, state: State<AppState>) -> String {
    let prefix = state.greeting_prefix.lock().unwrap();
    format!("{}, {}!", *prefix, name)
}
*/
// 🔹 Handle incoming frontend requests
#[tauri::command]
pub async fn handle_frontend_request(app: AppHandle, frontend_request: FrontendRequest, app_state:State<'_, AppState>)-> Result<BackendResponse, String> {

    let app_daemon_proxy = app_state.shared_app_daemon_proxy.lock().await;
    /*
    let response = match frontend_request.action.as_str() {
        "greet" => greet(&frontend_request.payload),
        "calculate" => calculate(&frontend_request.payload),
        "get_time_string" => get_time_string(app_daemon_proxy).await,
        _ => BackendResponse {
            status: "error".to_string(),
            description: "Unknown action".to_string(),
            data: None,
        },
    };

    app.emit_all("backend_response", &response).unwrap();
    */
    let response = match frontend_request.action.as_str() {
        "greet" => greet(&frontend_request.payload),
        "calculate" => calculate(&frontend_request.payload),
        "get_daemon_operational_situation" => {
            let current_daemon_operational_situation = get_daemon_operational_situation(app_daemon_proxy.clone()).await;
            BackendResponse {
                status: "success".to_string(),
                description: "Current time fetched".to_string(),
                data: Some(serde_json::json!({ "daemon_operational_situation": current_daemon_operational_situation })),
            }
        },

        /*
                "get_time_string" => {
            let current_time = format_timestamp_to_gmt_string(timestamp_now());
            BackendResponse {
                status: "success".to_string(),
                description: "Current time fetched".to_string(),
                data: Some(serde_json::json!({ "time": current_time })),
            }
        },
        */
        _ => BackendResponse {
            status: "error".to_string(),
            description: "Unknown action".to_string(),
            data: None,
        },
    };

    //println!("🕒 Sending time response: {:?}", response); // DEBUG LOG
    Ok(response) // ✅ Return instead of emitting
}
