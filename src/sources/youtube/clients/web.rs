use serde_json::{json, Value};
use crate::types::http::NodelinkMock;

pub struct WebClient;

impl WebClient {
    pub fn new() -> Self { Self }

    pub fn get_client_context(&self, hl: &str, gl: &str) -> Value {
        json!({
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20250403.01.00",
                "platform": "DESKTOP",
                "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
                "hl": hl,
                "gl": gl
            },
            "user": { "lockedSafetyMode": false },
            "request": { "useSsl": true }
        })
    }

    pub async fn search(&self, _query: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }
}
