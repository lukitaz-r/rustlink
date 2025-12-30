use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use super::send_response::send_response;

pub fn send_error_response(
    req_headers: &HeaderMap,
    status: StatusCode,
    error: &str,
    message: &str,
    path: &str,
    trace: bool,
) -> Response {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let error_payload = json!({
        "timestamp": timestamp,
        "status": status.as_u16(),
        "error": error,
        "message": message,
        "path": path,
        "trace": if trace { "Full stack trace would be here".to_string() } else { "".to_string() }
    });

    send_response(req_headers, Some(error_payload), status, trace)
}
