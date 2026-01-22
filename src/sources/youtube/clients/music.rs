use serde_json::{json, Value};
use crate::types::http::NodelinkMock;

pub struct MusicClient;

impl MusicClient {
    pub fn new() -> Self { Self }

    pub fn get_client_context(&self, hl: &str, gl: &str) -> Value {
        json!({
            "client": {
                "clientName": "ANDROID_MUSIC",
                "clientVersion": "7.27.52",
                "userAgent": "com.google.android.apps.youtube.music/7.27.52 (Linux; U; Android 14 gzip)",
                "osName": "Android",
                "osVersion": "14",
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
