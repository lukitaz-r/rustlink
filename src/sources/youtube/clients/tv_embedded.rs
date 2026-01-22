use serde_json::{json, Value};
use crate::types::http::NodelinkMock;

pub struct TvEmbeddedClient;

impl TvEmbeddedClient {
    pub fn new() -> Self { Self }

    pub fn get_client_context(&self, hl: &str, gl: &str) -> Value {
        json!({
            "client": {
                "clientName": "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
                "clientVersion": "2.0",
                "userAgent": "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
                "hl": hl,
                "gl": gl
            },
            "user": { "lockedSafetyMode": false },
            "request": { "useSsl": true },
            "thirdParty": { "embedUrl": "https://www.youtube.com" }
        })
    }

    pub async fn resolve(&self, _url: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }
}
