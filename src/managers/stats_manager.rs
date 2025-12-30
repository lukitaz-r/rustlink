use std::sync::{Arc, Mutex};
use serde_json::json;

#[derive(Clone)]
pub struct StatsManager {
    stats: Arc<Mutex<serde_json::Value>>,
}

impl StatsManager {
    pub fn new() -> Self {
        let initial_stats = json!({
            "api": {
                "requests": {},
                "errors": {}
            },
            "sources": {},
            "playback": {
                "events": {}
            }
        });
        
        Self {
            stats: Arc::new(Mutex::new(initial_stats)),
        }
    }

    pub fn get_snapshot(&self) -> serde_json::Value {
        self.stats.lock().unwrap().clone()
    }
    
    // Increment methods stubbed
}
