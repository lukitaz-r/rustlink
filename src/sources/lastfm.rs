use async_trait::async_trait;
use serde_json::{json, Value};
use crate::types::http::NodelinkMock;
use super::Source;

pub struct LastfmSource;

#[async_trait]
impl Source for LastfmSource {
    async fn search(&self, _query: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }

    async fn resolve(&self, _url: &str, _nodelink: &NodelinkMock) -> Value {
        json!({ "loadType": "empty", "data": {} })
    }
}
