use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::Response;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use super::logger::logger;
use super::send_response::send_response;

pub fn verify_method(
    uri: &Uri,
    method: &Method,
    req_headers: &HeaderMap,
    expected: &[Method],
    client_address: &str,
    trace: bool,
) -> Result<(), Response> {
    if !expected.contains(method) {
        let path = uri.path();
        logger(
            "warn",
            "Server",
            &format!(
                "Method not allowed: {} {} from {}",
                method, path, client_address
            ),
            None,
        );

        let methods_str = expected
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let data = json!({
            "timestamp": timestamp,
            "status": 405,
            "error": "Method Not Allowed",
            "message": format!("Method must be one of {}", methods_str),
            "path": path,
            "trace": if trace { "Full stack trace would be here".to_string() } else { "".to_string() }
        });

        return Err(send_response(
            req_headers,
            Some(data),
            StatusCode::METHOD_NOT_ALLOWED,
            trace,
        ));
    }
    Ok(())
}
