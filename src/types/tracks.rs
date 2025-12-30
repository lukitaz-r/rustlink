use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // Clone es útil si necesitas copias
pub struct TrackInfo {
    pub title: String,
    pub author: String,
    pub length: i64,
    pub identifier: String,
    pub is_seekable: bool,
    pub is_stream: bool,
    pub uri: Option<String>,
    pub artwork_url: Option<String>,
    pub isrc: Option<String>,
    pub source_name: String,
    pub position: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedTrack {
    pub encoded: String,
    pub info: TrackInfo,
}
