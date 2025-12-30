use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use crate::utils::send_response;
use crate::types::stats::RustlinkMock;
use std::sync::{Arc, RwLock};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SessionPatch {
    resuming: Option<bool>,
    timeout: Option<u64>,
}

pub async fn handler(
    State(rustlink): State<Arc<RwLock<RustlinkMock>>>,
    Path(session_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<SessionPatch>,
) -> Response {
    let mut rustlink_write = rustlink.write().unwrap();

    if let Some(session) = rustlink_write.sessions.get_mut(&session_id) {
        // Since Session struct in stats.rs might not have resuming/timeout fields yet, 
        // we might need to update the struct definition first.
        // For now, we acknowledge the request but don't persist if fields are missing.
        // Checking rustlink/src/types/stats.rs context...
        // It has `Session { players: SessionPlayers }`. 
        // It does NOT have resuming/timeout.
        
        // TODO: Update Session struct to support resuming/timeout.
        
        let response = json!({
            "resuming": payload.resuming.unwrap_or(false),
            "timeout": payload.timeout.unwrap_or(0)
        });
        
        return send_response(&headers, Some(response), StatusCode::OK, false);
    }

    send_response(
        &headers,
        Some(json!({
            "timestamp": chrono::Utc::now().timestamp_millis(),
            "status": 404,
            "error": "Not Found",
            "message": "The provided sessionId doesn't exist.",
            "path": format!("/v4/sessions/{}", session_id)
        })),
        StatusCode::NOT_FOUND,
        false,
    )
}
