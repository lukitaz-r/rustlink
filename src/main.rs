use axum::{
    routing::{get, patch, post},
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

mod api;
mod types;
mod utils;

use crate::types::stats::{RustlinkMock, RustlinkStats};

#[tokio::main]
async fn main() {
    println!("\nVersion: {}", utils::get_version(false));

    let rustlink_state = Arc::new(RwLock::new(RustlinkMock {
        statistics: RustlinkStats {
            players: 0,
            playing_players: 0,
        },
        sessions: HashMap::new(),
    }));

    let app = Router::new()
        .route("/v4/decodetrack", get(api::handler_decode_track))
        .route("/v4/decodetracks", post(api::handler_decode_tracks))
        .route("/v4/encodetrack", get(api::handler_encode_track))
        .route("/v4/info", get(api::handler_info))
        .route("/v4/stats", get(api::handler_stats))
        .route("/version", get(api::handler_version))
        .route("/v4/sessions/:sessionId", patch(api::handler_sessions))
        // players handler handles multiple methods and paths?
        // players.rs signature: (State, Method, Headers, Uri, Body)
        // It manually parses path to distinguish guildId presence.
        // So we route both to it.
        .route("/v4/sessions/:sessionId/players", get(api::handler_players))
        .route("/v4/sessions/:sessionId/players/:guildId", get(api::handler_players).patch(api::handler_players).delete(api::handler_players))
        .with_state(rustlink_state.clone());

    // Fix for stats handler needing state
    // api/stats.rs: pub async fn handler(headers: HeaderMap, rustlink: &RustlinkMock)
    // This is not a valid Axum handler signature directly with .with_state().
    // I should create a wrapper here or modify api/stats.rs.
    // I'll modify api/stats.rs to take State in a separate step.
    // actually, let's fix api/stats.rs NOW before running.

    let addr = SocketAddr::from(([0, 0, 0, 0], 2333));
    println!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}