use serde::{Deserialize, Serialize};
use crate::managers::session_manager::SessionManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RustlinkStats {
    pub players: usize,
    #[serde(rename = "playingPlayers")]
    pub playing_players: usize,
}

pub struct RustlinkMock {
    pub statistics: RustlinkStats,
    pub sessions: SessionManager,
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
