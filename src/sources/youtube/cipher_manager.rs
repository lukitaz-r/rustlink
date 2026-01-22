use serde_json::{json, Value};
use crate::utils::{make_request, logger};
use crate::types::http::{RequestOptions, NodelinkMock};

pub struct CipherManager {
    // config...
}

impl CipherManager {
    pub fn new() -> Self { Self {} }

    pub async fn get_timestamp(&self, _player_url: &str, _nodelink: &NodelinkMock) -> Result<String, String> {
        // Mock
        Ok("12345".to_string())
    }

    pub async fn resolve_url(&self, stream_url: &str, _nodelink: &NodelinkMock) -> Result<String, String> {
        // Mock
        Ok(stream_url.to_string())
    }
}
