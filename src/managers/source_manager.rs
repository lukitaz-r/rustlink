use std::collections::HashMap;
use serde_json::Value;
use crate::sources::{Source, youtube::youtube::YoutubeSource, soundcloud::SoundcloudSource, spotify::SpotifySource};
use crate::types::http::NodelinkMock;

pub struct SourceManager {
    sources: HashMap<String, Box<dyn Source>>,
}

impl SourceManager {
    pub fn new() -> Self {
        let mut sources: HashMap<String, Box<dyn Source>> = HashMap::new();
        
        sources.insert("youtube".to_string(), Box::new(YoutubeSource::new()));
        sources.insert("soundcloud".to_string(), Box::new(SoundcloudSource));
        sources.insert("spotify".to_string(), Box::new(SpotifySource));
        // Add others...

        Self {
            sources,
        }
    }

    pub async fn load_folder(&mut self) {
        // In Rust, we manually register or use a registry.
    }

    pub async fn search(&self, query: &str, nodelink: &NodelinkMock) -> Value {
        // Simplified search logic
        if let Some(source) = self.sources.get("youtube") {
            return source.search(query, nodelink).await;
        }
        serde_json::json!({
            "loadType": "empty",
            "data": {}
        })
    }
    
    pub async fn resolve(&self, url: &str, nodelink: &NodelinkMock) -> Value {
        for source in self.sources.values() {
            let res = source.resolve(url, nodelink).await;
            if res["loadType"] != "empty" && res["loadType"] != "error" {
                return res;
            }
        }
         serde_json::json!({
            "loadType": "empty",
            "data": {}
        })
    }
}