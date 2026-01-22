use tokio::sync::mpsc;
use serde_json::{Value, json};
use crate::types::audio_engine::AudioEngineCommand;

#[derive(Clone)]
pub struct PlayerManager {
    sender: mpsc::Sender<AudioEngineCommand>,
    session_id: String,
}

impl PlayerManager {
    pub fn new(sender: mpsc::Sender<AudioEngineCommand>, session_id: String) -> Self {
        Self {
            sender,
            session_id,
        }
    }

    pub async fn create(&self, guild_id: String, user_id: String, voice: Option<Value>) -> Result<Value, String> {
        let (tx, rx) = mpsc::channel(1); // One-shot response via mpsc for now, or use oneshot
        // AudioEngine uses mpsc::Sender<Value> for response.
        
        let command = AudioEngineCommand::CreatePlayer {
            session_id: self.session_id.clone(),
            guild_id,
            user_id,
            voice,
            resp: tx,
        };

        self.sender.send(command).await.map_err(|e| e.to_string())?;
        
        let mut rx = rx;
        rx.recv().await.ok_or("No response from AudioEngine".to_string())
    }

    pub async fn destroy(&self, guild_id: String) -> Result<Value, String> {
        let (tx, rx) = mpsc::channel(1);
        let command = AudioEngineCommand::DestroyPlayer {
            guild_id,
            resp: tx,
        };
        self.sender.send(command).await.map_err(|e| e.to_string())?;
        let mut rx = rx;
        rx.recv().await.ok_or("No response from AudioEngine".to_string())
    }

    pub async fn play(&self, guild_id: String, track_payload: Value) -> Result<Value, String> {
        self.send_command(guild_id, "play", vec![track_payload]).await
    }

    pub async fn stop(&self, guild_id: String) -> Result<Value, String> {
        self.send_command(guild_id, "stop", vec![]).await
    }

    pub async fn pause(&self, guild_id: String, should_pause: bool) -> Result<Value, String> {
        self.send_command(guild_id, "pause", vec![serde_json::Value::Bool(should_pause)]).await
    }

    pub async fn seek(&self, guild_id: String, position: i64) -> Result<Value, String> {
        self.send_command(guild_id, "seek", vec![serde_json::Value::Number(position.into())]).await
    }

    pub async fn set_volume(&self, guild_id: String, level: u64) -> Result<Value, String> {
        self.send_command(guild_id, "volume", vec![serde_json::Value::Number(level.into())]).await
    }
    
        pub async fn set_filters(&self, guild_id: String, filters: Value) -> Result<Value, String> {
            self.send_command(guild_id, "setFilters", vec![filters]).await
        }
    
        pub async fn set_voice(&self, guild_id: String, session_id: String, token: String, endpoint: String) -> Result<Value, String> {
            self.send_command(guild_id, "updateVoice", vec![json!({
                "sessionId": session_id,
                "token": token,
                "endpoint": endpoint
            })]).await
        }
    
        async fn send_command(&self, guild_id: String, command: &str, args: Vec<Value>) -> Result<Value, String> {
                let (tx, rx) = mpsc::channel(1);

            let cmd = AudioEngineCommand::PlayerCommand {

                guild_id,

                command: command.to_string(),

                args,

                resp: tx,

            };

            self.sender.send(cmd).await.map_err(|e| e.to_string())?;

            let mut rx = rx;

            rx.recv().await.ok_or("No response from AudioEngine".to_string())

        }

    

        pub async fn get_player(&self, guild_id: String) -> Option<Value> {

            let (tx, mut rx) = mpsc::channel(1);

            let cmd = AudioEngineCommand::GetPlayer {

                guild_id,

                resp: tx,

            };

            if self.sender.send(cmd).await.is_err() {

                return None;

            }

            let res = rx.recv().await;

            if let Some(val) = res {

                if val.is_null() {

                    None

                } else {

                    Some(val)

                }

            } else {

                None

            }

        }

    

        pub async fn get_players(&self) -> Vec<Value> {

            let (tx, mut rx) = mpsc::channel(1);

            let cmd = AudioEngineCommand::GetPlayers {

                resp: tx,

            };

            if self.sender.send(cmd).await.is_err() {

                return Vec::new();

            }

            rx.recv().await.unwrap_or_default()

        }

    }

    