use axum::{
    routing::{get, patch, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

mod api;
mod types;
mod utils;
mod playback;
mod managers;
mod sources;

use crate::types::stats::{RustlinkMock, RustlinkStats};
use crate::playback::audio_engine::AudioEngine;
use crate::managers::session_manager::SessionManager;

#[tokio::main]
async fn main() {
    let version = utils::get_version(false);
    
    let ascii = format!(r#"
   ▄   ████▄ ██▄   ▄███▄   █    ▄█    ▄   █  █▀ 
    █  █   █ █  █  █▀   ▀  █    ██     █  █▄█   
██   █ █   █ █   █ ██▄▄    █    ██ ██   █ █▀▄   Rust Mode
█ █  █ ▀████ █  █  █▄   ▄▀ ███▄ ▐█ █ █  █ █  █  v{}
█  █ █       ███▀  ▀███▀       ▀ ▐ █  █ █   █   Powered by PerformanC;
█   ██                             █   ██  ▀    transpiled by Gemini CLI;
"#, version);
    
    println!("\x1b[32m{}\x1b[0m", ascii);

    // Initialize Audio Engine (Successor to Worker)
    let (tx, rx) = mpsc::channel(100);
    let mut audio_engine = AudioEngine::new(rx);
    tokio::spawn(async move {
        audio_engine.run().await;
    });

    let rustlink_state = Arc::new(RwLock::new(RustlinkMock {
        statistics: RustlinkStats {
            players: 0,
            playing_players: 0,
        },
        sessions: SessionManager::new(tx),
    }));

    let app = Router::new()
        .route("/v4/decodetrack", get(api::handler_decode_track))
        .route("/v4/decodetracks", post(api::handler_decode_tracks))
        .route("/v4/encodetrack", get(api::handler_encode_track))
        .route("/v4/info", get(api::handler_info))
        .route("/v4/stats", get(api::handler_stats))
        .route("/version", get(api::handler_version))
        .route("/v4/sessions/{sessionId}", patch(api::handler_sessions))
        .route("/v4/sessions/{sessionId}/players", get(api::handler_players))
        .route("/v4/sessions/{sessionId}/players/{guildId}", get(api::handler_players).patch(api::handler_players).delete(api::handler_players))
        .with_state(rustlink_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}