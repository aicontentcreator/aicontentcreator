use serde::{Deserialize, Serialize};
use tauri::command;

use crate::app_state::app_state::AppState;

use crate::frontend_request::types::BackendResponse;
use crate::frontend_request::types::FrontendRequest;

pub async fn handle_say_hello(req: FrontendRequest) -> Result<BackendResponse, String> {
    if let Some(name) = req.payload.get("name").and_then(|v| v.as_str()) {
        Ok(BackendResponse {
            status: "success".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "additional_text": format!("Hello, {}!", name) }),
        })
    } else {
        Ok(BackendResponse {
            status: "error".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "additional_text": "Missing 'name' field" }),
        })
    }
}

pub async fn handle_send_chat_message(req: FrontendRequest) -> Result<BackendResponse, String> {
    if let Some(chat_message_text) = req
        .payload
        .get("chat_message_text")
        .and_then(|v| v.as_str())
    {
        Ok(BackendResponse {
            status: "success".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "response_text": format!("What do you mean {}!", chat_message_text) }),
        })
    } else {
        Ok(BackendResponse {
            status: "error".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "additional_text": "Missing 'chat_message_text' field" }),
        })
    }
}

pub async fn handle_get_chat_messages_count(
    req: FrontendRequest,
    app_state: tauri::State<'_, AppState>,
) -> Result<BackendResponse, String> {
    if let Some(_chat_id) = req.payload.get("chat_id").and_then(|v| v.as_str()) {
        let tmp_user_chats = app_state.shared_user_chats.lock().await;
        let tmp_value = tmp_user_chats.test_variable;

        Ok(BackendResponse {
            status: "success".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "chat_messages_count": tmp_value }),
        })
    } else {
        Ok(BackendResponse {
            status: "error".into(),
            status_description: "".into(),
            payload: serde_json::json!({ "additional_text": "Missing 'chat_id' field" }),
        })
    }
}
