use serde::{Deserialize, Serialize};
use tauri::command;

use crate::app_state::app_state::AppState;
use crate::frontend_request::types::BackendResponse;
use crate::frontend_request::types::FrontendRequest;

pub async fn handle_request_settings() -> Result<BackendResponse, String> {
    let settings = serde_json::json!({
        "theme": "dark",
        "notifications": true,
        "version": "1.0.0",
    });

    Ok(BackendResponse {
        status: "success".into(),
        status_description: "".into(),
        payload: settings,
    })
}
