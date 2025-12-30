use crate::types::stats::RustlinkMock;
use crate::utils::{get_stats, send_response};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use std::sync::{Arc, RwLock};

pub async fn handler(
    State(rustlink): State<Arc<RwLock<RustlinkMock>>>,
    headers: HeaderMap,
) -> Response {
    let rustlink_read = rustlink.read().unwrap();
    let payload = get_stats(&rustlink_read);

    let detailed_stats = serde_json::json!({});

    let final_payload = serde_json::json!({
        "players": payload.players,
        "playingPlayers": payload.playing_players,
        "uptime": payload.uptime,
        "memory": {
            "free": payload.memory.free,
            "used": payload.memory.used,
            "allocated": payload.memory.allocated,
            "reservable": payload.memory.reservable,
        },
        "cpu": {
            "cores": payload.cpu.cores,
            "systemLoad": payload.cpu.system_load,
            "rustlinkLoad": payload.cpu.rustlink_load,
        },
        "frameStats": payload.frame_stats.as_ref().map(|fs| {
            serde_json::json!({
                "sent": fs.sent,
                "nulled": fs.nulled,
                "deficit": fs.deficit,
                "expected": fs.expected,
            })
        }),
        "detailedStats": detailed_stats
    });

    send_response(&headers, Some(final_payload), StatusCode::OK, false)
}
