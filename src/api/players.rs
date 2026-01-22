use crate::types::stats::RustlinkMock;
use crate::utils::{decode_track, send_response};
use crate::managers::player_manager::PlayerManager;
use axum::{
    Json,
    extract::{State, Path},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{Response, IntoResponse},
};
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};

pub async fn handler(
    state: State<Arc<RwLock<RustlinkMock>>>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Option<Json<Value>>,
) -> Response {
    let path = uri.path();
    let parts: Vec<&str> = path.split('/').collect();

    let session_id = parts.get(3).copied();
    let guild_id: Option<String> = parts.get(5).map(|s| s.to_string());

    let player_manager = {
        let rustlink_read = state.read().unwrap();
        session_id.and_then(|sid| rustlink_read.sessions.get(sid).map(|s| s.player_manager.clone()))
    };

    let player_manager = match player_manager {
        Some(pm) => pm,
        None => {
            return send_response(
                &headers,
                Some(json!({
                    "timestamp": Utc::now().timestamp_millis(),
                    "status": 404,
                    "error": "Not Found",
                    "message": "The provided sessionId doesn't exist.",
                    "path": path
                })),
                StatusCode::NOT_FOUND,
                false,
            ).into_response();
        }
    };

    match (guild_id, method) {
        (None, Method::GET) => {
            let players = player_manager.get_players().await;
            send_response(&headers, Some(json!(players)), StatusCode::OK, false).into_response()
        }
        (Some(gid), Method::GET) => {
            let player = player_manager.get_player(gid.clone()).await.unwrap_or_else(|| {
                // Simplified: return empty or logic to create
                Value::Null
            });
            send_response(&headers, Some(player), StatusCode::OK, false).into_response()
        }
        (Some(gid), Method::DELETE) => {
            match player_manager.destroy(gid).await {
                Ok(_) => send_response(&headers, None, StatusCode::NO_CONTENT, false).into_response(),
                Err(e) => send_error(&headers, &e, StatusCode::NOT_FOUND, path).into_response(),
            }
        }
        (Some(gid), Method::PATCH) => {
            if let Some(Json(payload)) = body {
                // Process PATCH
                let _ = player_manager.set_filters(gid.clone(), payload).await;
                let player = player_manager.get_player(gid).await.unwrap_or(Value::Null);
                send_response(&headers, Some(player), StatusCode::OK, false).into_response()
            } else {
                send_error(&headers, "Missing body", StatusCode::BAD_REQUEST, path).into_response()
            }
        }
        _ => send_error(&headers, "Method not allowed", StatusCode::METHOD_NOT_ALLOWED, path).into_response()
    }
}

fn send_error(headers: &HeaderMap, message: &str, status: StatusCode, path: &str) -> Response {
    send_response(
        headers,
        Some(json!({
            "timestamp": Utc::now().timestamp_millis(),
            "status": status.as_u16(),
            "error": status.canonical_reason().unwrap_or("Unknown"),
            "message": message,
            "path": path
        })),
        status,
        false,
    )
}
