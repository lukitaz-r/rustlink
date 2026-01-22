use serde_json::{json, Value};
use crate::types::http::NodelinkMock;

pub struct TvClient;

impl TvClient {
    pub fn new() -> Self { Self }

    pub fn get_client_context(&self, hl: &str, gl: &str) -> Value {
        json!({
            "client": {
                "clientName": "TVHTML5",
                "clientVersion": "7.20250319.10.00",
                "userAgent": "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
                "hl": hl,
                "gl": gl
            },
            "user": { "lockedSafetyMode": false },
            "request": { "useSsl": true }
        })
    }

    pub async fn resolve(&self, _url: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }
}
