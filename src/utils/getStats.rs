use super::types::stats::{CpuStats, FrameStats, MemoryStats, RustlinkMock, Stats};
use sysinfo::{System, SystemExt};

pub fn get_stats(rustlink: &RustlinkMock) -> Stats {
    let players = rustlink.statistics.players;
    let playing_players = rustlink.statistics.playing_players;

    let mut frame_stats = if players > 0 {
        Some(FrameStats {
            sent: 0,
            nulled: 0,
            deficit: 0,
            expected: 0,
        })
    } else {
        None
    };
    if let Some(fs) = &mut frame_stats {
        for session in rustlink.sessions.values() {
            for player in session.players.players.values() {
                if let Some(conn) = &player.connection {
                    let sent = conn.statistics.packets_sent;
                    let nulled = conn.statistics.packets_lost;
                    let expected = conn.statistics.packets_expect;
                    fs.sent += sent;
                    fs.nulled += nulled;
                    fs.expected += expected;
                }
            }
        }
        fs.deficit = fs.expected.saturating_sub(fs.sent);
    }
    let mut sys = System::new_all();
    sys.refresh_all();
    let uptime = sys.uptime() * 1000; // Segundos a ms

    let memory = MemoryStats {
        free: sys.free_memory(),
        used: sys.used_memory(),
        allocated: sys.total_memory(), // Aproximación a heapTotal
        reservable: sys.total_memory(),
    };
    let cores = sys.cpus().len();
    let load = sys.load_average().one; // Load avg 1 min

    let cpu = CpuStats {
        cores,
        system_load: load,
        rustlink_load: (load / cores as f64 * 100.0).round() / 100.0, // Format to 2 decimal places aprox
    };
    Stats {
        players,
        playing_players,
        uptime,
        memory,
        cpu,
        frame_stats,
    }
}
