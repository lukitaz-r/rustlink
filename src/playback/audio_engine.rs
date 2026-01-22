use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::managers::connection_manager::ConnectionManager;
use crate::managers::lyrics_manager::LyricsManager;
use crate::managers::route_planner_manager::RoutePlannerManager;
use crate::managers::source_manager::SourceManager;
use crate::managers::stats_manager::StatsManager;
use crate::playback::player::Player;
use crate::types::audio_engine::AudioEngineCommand;
use crate::types::http::NodelinkMock;

pub struct AudioEngine {
    players: HashMap<String, Player>,
    stats_manager: StatsManager,
    source_manager: SourceManager,
    lyrics_manager: LyricsManager,
    route_planner: RoutePlannerManager,
    connection_manager: ConnectionManager,
    receiver: mpsc::Receiver<AudioEngineCommand>,
    nodelink: NodelinkMock,
}

impl AudioEngine {
    pub fn new(receiver: mpsc::Receiver<AudioEngineCommand>) -> Self {
        Self {
            players: HashMap::new(),
            stats_manager: StatsManager::new(),
            source_manager: SourceManager::new(),
            lyrics_manager: LyricsManager::new(),
            route_planner: RoutePlannerManager::new(),
            connection_manager: ConnectionManager::new(),
            receiver,
            nodelink: NodelinkMock {
                route_planner: None,
            },
        }
    }

    pub async fn run(&mut self) {
        // Initialize managers
        self.source_manager.load_folder().await;
        self.lyrics_manager.load_folder().await;
        self.connection_manager.start().await;

        println!("AudioEngine (Worker) started.");

        while let Some(command) = self.receiver.recv().await {
            match command {
                AudioEngineCommand::CreatePlayer {
                    session_id: _,
                    guild_id,
                    user_id: _,
                    voice,
                    resp,
                } => {
                    let result = if self.players.contains_key(&guild_id) {
                        serde_json::json!({ "created": false, "reason": "Player already exists" })
                    } else {
                        let mut player = Player::new(guild_id.clone());
                        if let Some(v) = voice {
                            // Mock voice update
                            let _ = player.update_voice(
                                v.get("sessionId").and_then(|s| s.as_str()).unwrap_or(""),
                                v.get("token").and_then(|s| s.as_str()).unwrap_or(""),
                                v.get("endpoint").and_then(|s| s.as_str()).unwrap_or(""),
                            );
                        }
                        self.players.insert(guild_id, player);
                        serde_json::json!({ "created": true })
                    };
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::DestroyPlayer { guild_id, resp } => {
                    let result = if let Some(player) = self.players.get_mut(&guild_id) {
                        player.destroy();
                        self.players.remove(&guild_id);
                        serde_json::json!({ "destroyed": true })
                    } else {
                        serde_json::json!({ "destroyed": false, "reason": "Player not found" })
                    };
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::PlayerCommand {
                    guild_id,
                    command,
                    args,
                    resp,
                } => {
                    let result = if let Some(player) = self.players.get_mut(&guild_id) {
                        match command.as_str() {
                            "play" => {
                                // args[0] = { encoded, info, ... }
                                if let Some(track_payload) = args.get(0) {
                                    let encoded = track_payload
                                        .get("encoded")
                                        .and_then(|s| s.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let info = track_payload
                                        .get("info")
                                        .cloned()
                                        .unwrap_or(serde_json::Value::Null);
                                    let no_replace = track_payload
                                        .get("noReplace")
                                        .and_then(|b| b.as_bool())
                                        .unwrap_or(false);
                                    serde_json::json!(player.play(encoded, info, no_replace))
                                } else {
                                    serde_json::json!(false)
                                }
                            }
                            "stop" => serde_json::json!(player.stop()),
                            "pause" => {
                                let should_pause =
                                    args.get(0).and_then(|b| b.as_bool()).unwrap_or(true);
                                serde_json::json!(player.pause(should_pause))
                            }
                            "seek" => {
                                let position = args.get(0).and_then(|v| v.as_i64()).unwrap_or(0);
                                serde_json::json!(player.seek(position))
                            }
                            "volume" => {
                                let level =
                                    args.get(0).and_then(|v| v.as_u64()).unwrap_or(100) as u32;
                                serde_json::json!(player.set_volume(level))
                            }
                            "setFilters" => {
                                let filters = args.get(0).cloned().unwrap_or(serde_json::json!({}));
                                serde_json::json!(player.set_filters(filters))
                            }
                            // ... other commands
                            _ => serde_json::json!({ "error": "Unknown command" }),
                        }
                    } else {
                        serde_json::json!({ "error": "Player not found" })
                    };
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::LoadTracks { identifier, resp } => {
                    let result = if identifier.starts_with("http") {
                        self.source_manager
                            .resolve(&identifier, &self.nodelink)
                            .await
                    } else {
                        self.source_manager
                            .search(&identifier, &self.nodelink)
                            .await
                    };
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::LoadLyrics {
                    decoded_track,
                    resp,
                } => {
                    let result = self.lyrics_manager.load_lyrics(&decoded_track).await;
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::GetSources { resp } => {
                    let _ = resp
                        .send(vec!["youtube".to_string(), "soundcloud".to_string()])
                        .await;
                }
                AudioEngineCommand::GetPlayer { guild_id, resp } => {
                    let result = if let Some(player) = self.players.get(&guild_id) {
                        player.to_json()
                    } else {
                        serde_json::Value::Null
                    };
                    let _ = resp.send(result).await;
                }
                AudioEngineCommand::GetPlayers { resp } => {
                    let players: Vec<serde_json::Value> =
                        self.players.values().map(|p| p.to_json()).collect();
                    let _ = resp.send(players).await;
                }
            }
        }
    }
}
