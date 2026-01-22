use async_trait::async_trait;
use serde_json::Value;
use crate::types::http::NodelinkMock;

pub mod youtube;
pub mod soundcloud;
pub mod spotify;
pub mod deezer;
pub mod bandcamp;
pub mod http;
pub mod local;
pub mod twitch;
pub mod nicovideo;
pub mod reddit;
pub mod instagram;
pub mod kwai;
pub mod lastfm;
pub mod google_tts;
pub mod tidal;

#[async_trait]
pub trait Source: Send + Sync {
    async fn search(&self, query: &str, nodelink: &NodelinkMock) -> Value;
    async fn resolve(&self, url: &str, nodelink: &NodelinkMock) -> Value;
    // ... other methods like load_stream
}
