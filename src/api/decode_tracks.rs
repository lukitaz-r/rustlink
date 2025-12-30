use axum::{
    extract::Json,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use crate::utils::{decode_track, send_response};
use serde_json::json;

pub async fn handler(
    headers: HeaderMap,
    Json(encoded_tracks): Json<Vec<String>>,
) -> Response {
    let mut decoded_tracks = Vec::new();
    for encoded in encoded_tracks {
        if let Ok(decoded) = decode_track(&encoded) {
            decoded_tracks.push(decoded);
        }
    }
    send_response(&headers, Some(json!(decoded_tracks)), StatusCode::OK, false)
}
