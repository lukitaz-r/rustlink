use std::collections::HashMap;
use serde_json::Value;
use tokio::sync::mpsc;

use crate::types::audio_engine::AudioEngineCommand;
use super::player_manager::PlayerManager;

pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub client_info: Option<Value>,
    pub player_manager: PlayerManager,
}

pub struct SessionManager {
    pub sessions: HashMap<String, Session>,
    audio_engine_sender: mpsc::Sender<AudioEngineCommand>,
}

impl SessionManager {
    pub fn new(audio_engine_sender: mpsc::Sender<AudioEngineCommand>) -> Self {
        Self {
            sessions: HashMap::new(),
            audio_engine_sender,
        }
    }

    pub fn create(&mut self, _request: &Value, _client_info: Option<Value>) -> String {
        let session_id = "mock_session_id".to_string(); // In real app generate random
        let player_manager = PlayerManager::new(self.audio_engine_sender.clone(), session_id.clone());
        
        let session = Session {
            id: session_id.clone(),
            user_id: None, // Extract from request
            client_info: _client_info,
            player_manager,
        };
        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    pub fn get(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }
    
    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    pub fn delete(&mut self, session_id: &str) {
        // Here we should probably also tell AudioEngine to destroy players for this session.
        // But for now, just remove session.
        self.sessions.remove(session_id);
    }
}