use std::collections::HashMap;
use serde_json::Value;

use crate::playback::player::Player;

pub struct PlayerManager {
    players: HashMap<String, Player>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn get(&mut self, guild_id: &str) -> Option<&mut Player> {
        self.players.get_mut(guild_id)
    }

    pub fn create(&mut self, guild_id: String) -> &mut Player {
        self.players.entry(guild_id.clone()).or_insert_with(|| Player::new(guild_id))
    }

    pub fn destroy(&mut self, guild_id: &str) {
        if let Some(player) = self.players.get_mut(guild_id) {
            player.destroy();
        }
        self.players.remove(guild_id);
    }
}
