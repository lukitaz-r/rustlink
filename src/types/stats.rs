use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustlinkStats {
    pub players: u32,
    pub playing_players: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConnectionStats {
    pub packets_sent: u64,
    pub packets_lost: u64,
    pub packets_expect: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConnection {
    pub statistics: PlayerConnectionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub guild_id: String,
    pub connection: Option<PlayerConnection>,
}

#[derive(Debug, Clone)]
pub struct SessionPlayers {
    pub players: HashMap<String, Player>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub players: SessionPlayers,
}

#[derive(Debug, Clone)]
pub struct RustlinkMock {
    pub statistics: RustlinkStats,
    pub sessions: HashMap<String, Session>,
}

// Estructuras de Salida (DTOs)
#[derive(Debug, Serialize)]
pub struct FrameStats {
    pub sent: u64,
    pub nulled: u64,
    pub deficit: u64,
    pub expected: u64,
}

#[derive(Debug, Serialize)]
pub struct MemoryStats {
    pub free: u64,
    pub used: u64,
    pub allocated: u64,
    pub reservable: u64,
}

#[derive(Debug, Serialize)]
pub struct CpuStats {
    pub cores: usize,
    pub system_load: f64,
    pub rustlink_load: f64,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    pub players: u32,
    pub playing_players: u32,
    pub uptime: u64,
    pub memory: MemoryStats,
    pub cpu: CpuStats,
    pub frame_stats: Option<FrameStats>,
}
