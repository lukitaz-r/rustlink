use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct IpData {
    count: u32,
    last_reset: u128,
    blocked_until: u128,
}

pub struct DosProtectionManager {
    ip_request_counts: HashMap<String, IpData>,
    enabled: bool,
}

impl DosProtectionManager {
    pub fn new() -> Self {
        Self {
            ip_request_counts: HashMap::new(),
            enabled: false, // Default from config
        }
    }

    pub fn check(&mut self, remote_address: &str) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        
        let entry = self.ip_request_counts.entry(remote_address.to_string()).or_insert(IpData {
            count: 0,
            last_reset: now,
            blocked_until: 0,
        });

        if now < entry.blocked_until {
            return Err("Forbidden".to_string());
        }

        // Logic ...
        
        Ok(())
    }
}
