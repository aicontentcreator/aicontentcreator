use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendRequest {
    pub category: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendResponse {
    pub status: String,
    pub status_description: String,
    pub payload: serde_json::Value,
}
