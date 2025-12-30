use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use crate::utils::{decode_track, send_response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct DecodeTrackQuery {
    #[serde(rename = "encodedTrack")]
    encoded_track: String,
}

pub async fn handler(
    headers: HeaderMap,
    Query(params): Query<DecodeTrackQuery>,
) -> Response {
    match decode_track(&params.encoded_track) {
        Ok(decoded) => send_response(&headers, Some(json!(decoded)), StatusCode::OK, false),
        Err(e) => send_response(
            &headers,
            Some(json!({
                "timestamp": chrono::Utc::now().timestamp_millis(),
                "status": 500,
                "error": "Internal Server Error",
                "message": e.to_string(),
                "path": "/v4/decodetrack"
            })),
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
        ),
    }
}
