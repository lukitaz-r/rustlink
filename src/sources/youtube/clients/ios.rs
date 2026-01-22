use serde_json::{json, Value};
use crate::types::http::NodelinkMock;

pub struct IosClient;

impl IosClient {
    pub fn new() -> Self { Self }

    pub fn get_client_context(&self, hl: &str, gl: &str, visitor_data: Option<&str>) -> Value {
        json!({
            "client": {
                "clientName": "IOS",
                "clientVersion": "19.47.7",
                "userAgent": "com.google.ios.youtube/19.47.7 (iPhone16,2; U; CPU iOS 17_5_1 like Mac OS X;)",
                "deviceMake": "Apple",
                "deviceModel": "iPhone16,2",
                "osName": "iPhone",
                "osVersion": "17.5.1.21F90",
                "hl": hl,
                "gl": gl,
                "visitorData": visitor_data
            },
            "user": { "lockedSafetyMode": false },
            "request": { "useSsl": true }
        })
    }

    pub async fn search(&self, _query: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }
}
