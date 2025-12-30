use axum::response::IntoResponse;
use axum::http::{header, StatusCode};
use crate::utils::get_version;

pub async fn handler() -> impl IntoResponse {
    let version = get_version(false).to_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        version,
    )
}
