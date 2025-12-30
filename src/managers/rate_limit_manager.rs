use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct RateLimitEntry {
    requests: Vec<u128>,
}

pub struct RateLimitManager {
    store: HashMap<String, RateLimitEntry>,
    enabled: bool,
}

impl RateLimitManager {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            enabled: false,
        }
    }

    pub fn check(&mut self, _identifier: &str) -> bool {
        if !self.enabled {
            return true;
        }
        // Logic ...
        true
    }
}
