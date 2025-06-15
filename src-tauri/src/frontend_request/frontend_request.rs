use serde::{Deserialize, Serialize};
use tauri::command;

use crate::app_state::app_state::AppState;

use crate::frontend_request::types::BackendResponse;
use crate::frontend_request::types::FrontendRequest;

//
use crate::frontend_request::chats_request::handle_get_chat_messages_count;
use crate::frontend_request::chats_request::handle_say_hello;
use crate::frontend_request::chats_request::handle_send_chat_message;
use crate::frontend_request::settings_request::handle_request_settings;

#[command]
pub async fn handle_frontend_request(
    req: FrontendRequest,
    app_state: tauri::State<'_, AppState>,
) -> Result<BackendResponse, String> {
    match req.category.as_str() {
        "sayHello" => handle_say_hello(req).await,
        "sendChatMessageToBackend" => handle_send_chat_message(req).await,
        "getChatMessagesCountFromBackend" => handle_get_chat_messages_count(req, app_state).await,
        "requestSettings" => handle_request_settings().await,
        _ => handle_unknown_category().await,
    }
}
async fn handle_unknown_category() -> Result<BackendResponse, String> {
    Ok(BackendResponse {
        status: "error".into(),
        status_description: "Unknown category".into(),
        payload: serde_json::json!({ "additional_text": "Unknown category" }),
    })
}

/*
//async fn increment_count(state: tauri::State<'_, AppState>) -> Result<u32, String> {
pub async fn handle_frontend_request(req: FrontendRequest,app_state: tauri::State<'_, AppState>) -> Result<BackendResponse, String>  {
    match req.category.as_str() {
        //
        "sayHello" => {
            if let Some(name) = req.payload.get("name").and_then(|v| v.as_str()) {
                Ok(BackendResponse {
                    status: "success".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "additional_text": format!("Hello, {}!", name) }),
                })
            } else {
                Ok(BackendResponse {
                    status: "error".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "additional_text": "Missing 'name' field" }),
                })
            }
        }
        //const res = await callBackend<{ response_text: string }>('sendChatMessageToBackend', {  chat_message: input  });
        "sendChatMessageToBackend" => {
            if let Some(chat_message_text) = req.payload.get("chat_message_text").and_then(|v| v.as_str()) {
                Ok(BackendResponse {
                    status: "success".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "response_text": format!("What do you mean {}!", chat_message_text) }),
                })
            } else {
                Ok(BackendResponse {
                    status: "error".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "additional_text": "Missing 'chat_message_text' field" }),
                })
            }
        }
        "getChatMessagesCountFromBackend" => {
            if let Some(chat_id) = req.payload.get("chat_id").and_then(|v| v.as_str()) {
                let mut tmp_user_chats = app_state.shared_user_chats.lock().await;
                let tmp_value=tmp_user_chats.test_variable;
                Ok(BackendResponse {
                    status: "success".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "chat_messages_count": tmp_value }),
                })
            } else {
                Ok(BackendResponse {
                    status: "error".into(),
                    status_description:"".into(),
                    payload: serde_json::json!({ "additional_text": "Missing 'chat_id' field" }),
                })
            }
        }
        //
        "requestSettings" => {
            let settings = serde_json::json!({
                "theme": "dark",
                "notifications": true,
                "version": "1.0.0",
            });

            Ok(BackendResponse {
                status: "success".into(),
                status_description:"".into(),
                payload: settings,
            })
        }
        //
        _ => Ok(BackendResponse {
            status: "error".into(),
            status_description:"Unknown category".into(),
            payload: serde_json::json!({ "additional_text": "Unknown category" }),
        }),
    }
}
*/
