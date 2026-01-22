use serde_json::{json, Value};
use crate::utils::{make_request, logger};
use crate::types::http::{RequestOptions, NodelinkMock};
use crate::sources::youtube::common::{build_track, check_url_type, YoutubeUrlType};
use reqwest::Method;

pub struct AndroidClient {
    // config...
}

impl AndroidClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_client_context(&self, hl: &str, gl: &str, visitor_data: Option<&str>) -> Value {
        json!({
            "client": {
                "clientName": "ANDROID",
                "clientVersion": "20.38.37",
                "userAgent": "com.google.android.youtube/20.38.37 (Linux; U; Android 14) gzip",
                "deviceMake": "Google",
                "deviceModel": "Pixel 6",
                "osName": "Android",
                "osVersion": "14",
                "androidSdkVersion": "30",
                "hl": hl,
                "gl": gl,
                "visitorData": visitor_data
            },
            "user": { "lockedSafetyMode": false },
            "request": { "useSsl": true }
        })
    }

    pub async fn search(&self, query: &str, nodelink: &NodelinkMock) -> Value {
        let context = self.get_client_context("en", "US", None); // Default hl/gl
        let request_body = json!({
            "context": context,
            "query": query,
            "params": "EgIQAQ%3D%3D"
        });

        let mut options = RequestOptions::default();
        options.method = Method::POST;
        options.headers.insert("User-Agent", context["client"]["userAgent"].as_str().unwrap().parse().unwrap());
        options.headers.insert("X-Goog-Api-Format-Version", "2".parse().unwrap());
        options.body = Some(request_body.to_string());
        options.disable_body_compression = true;

        match make_request("https://youtubei.googleapis.com/youtubei/v1/search", options, nodelink).await {
            Ok(res) => {
                let body = res.body.unwrap_or(Value::Null);
                if body["error"].is_object() {
                    return json!({ "loadType": "error", "data": { "message": body["error"]["message"] } });
                }

                let mut tracks = Vec::new();
                if let Some(contents) = body["contents"]["sectionListRenderer"]["contents"].as_array() {
                    if let Some(last_section) = contents.last() {
                        if let Some(videos) = last_section["itemSectionRenderer"]["contents"].as_array() {
                            for video_data in videos {
                                if let Some(track) = build_track(video_data, "youtube", None) {
                                    tracks.push(track);
                                }
                            }
                        }
                    }
                }

                if tracks.is_empty() {
                    json!({ "loadType": "empty", "data": {} })
                } else {
                    json!({ "loadType": "search", "data": tracks })
                }
            }
            Err(e) => {
                json!({ "loadType": "error", "data": { "message": e.to_string() } })
            }
        }
    }

    // Resolve and other methods...
}
