use super::http1_make_request::http1_make_request;
use super::types::http::RequestOptions;
use reqwest::{Method, Url};
use serde_json::Value;

pub async fn load_hls(url: &str, _stream: &mut Vec<u8> /* Mocked stream */, _once_ended: bool, _should_end: bool) -> bool {
    let mut options = RequestOptions::default();
    options.method = Method::GET;
    
    if let Ok(res) = http1_make_request(url, options).await {
         if let Some(Value::String(body_str)) = res.body {
             let lines: Vec<&str> = body_str.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
             
             if !lines.iter().any(|l| l.starts_with("#EXTINF")) {
                  return !_should_end;
             }
             
             let base = Url::parse(url).ok();
             let mut _segs = Vec::new(); // Muted unused var for now
             let mut saw_end = false;
             
             for (i, line) in lines.iter().enumerate() {
                 if line.starts_with("#EXTINF") {
                     if let Some(uri) = lines.get(i+1) {
                         if !uri.starts_with("#") {
                             if let Some(base_url) = &base {
                                 if let Ok(joined) = base_url.join(uri) {
                                     _segs.push(joined.to_string());
                                 }
                             }
                         }
                     }
                 }
                 if line.starts_with("#EXT-X-ENDLIST") {
                     saw_end = true;
                 }
             }
             
             return !saw_end;
         }
    }
    false
}
