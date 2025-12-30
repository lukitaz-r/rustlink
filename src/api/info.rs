use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};
use crate::utils::{get_version, send_response};
use serde_json::json;

pub async fn handler(headers: HeaderMap) -> Response {
    let version_str = get_version(false).to_string();
    
    let response = json!({
        "version": {
            "semver": version_str,
            "major": 0,
            "minor": 1,
            "patch": 0,
            "preRelease": null
        },
        "buildTime": -1,
        "git": {
            "commit": "unknown",
            "branch": "unknown"
        },
        "node": "rust", 
        "voice": {
            "name": "rustlink-voice",
            "version": "0.1.0"
        },
        "sourceManagers": [],
        "filters": [],
        "plugins": []
    });

    send_response(&headers, Some(response), StatusCode::OK, false)
}
