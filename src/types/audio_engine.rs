use tokio::sync::mpsc;
use serde_json::Value;

#[derive(Debug)]
pub enum AudioEngineCommand {
    CreatePlayer {
        session_id: String,
        guild_id: String,
        user_id: String,
        voice: Option<Value>,
        resp: mpsc::Sender<Value>,
    },
    DestroyPlayer {
        guild_id: String,
        resp: mpsc::Sender<Value>,
    },
    PlayerCommand {
        guild_id: String,
        command: String,
        args: Vec<Value>,
        resp: mpsc::Sender<Value>,
    },
    LoadTracks {
        identifier: String,
        resp: mpsc::Sender<Value>,
    },
    LoadLyrics {
        decoded_track: Value,
        resp: mpsc::Sender<Value>,
    },
    GetSources {
        resp: mpsc::Sender<Vec<String>>,
    },
    GetPlayer {
        guild_id: String,
        resp: mpsc::Sender<Value>,
    },
    GetPlayers {
        resp: mpsc::Sender<Vec<Value>>,
    },
}
