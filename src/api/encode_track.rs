use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use crate::utils::{encode_track, send_response};
use crate::types::tracks::TrackInfo;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct EncodeTrackQuery {
    track: String,
}

pub async fn handler(
    headers: HeaderMap,
    Query(params): Query<EncodeTrackQuery>,
) -> Response {
    let track_info: Result<TrackInfo, _> = serde_json::from_str(&params.track);

    match track_info {
        Ok(info) => {
            match encode_track(&info) {
                Ok(encoded) => send_response(&headers, Some(json!(encoded)), StatusCode::OK, false),
                Err(e) => send_response(
                    &headers,
                    Some(json!({ "error": e.to_string() })),
                    StatusCode::INTERNAL_SERVER_ERROR,
                    false
                )
            }
        },
        Err(e) => send_response(
            &headers,
            Some(json!({ "error": "Invalid track JSON", "details": e.to_string() })),
            StatusCode::BAD_REQUEST,
            false
        )
    }
}
