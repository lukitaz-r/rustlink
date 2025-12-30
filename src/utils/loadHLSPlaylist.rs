use super::http1_make_request::http1_make_request;
use super::load_hls::load_hls;
use super::types::http::RequestOptions;
use reqwest::Method;
use serde_json::Value;

pub async fn load_hls_playlist(url: &str, stream: &mut Vec<u8>) -> bool {
     let mut options = RequestOptions::default();
     options.method = Method::GET;
     
     if let Ok(res) = http1_make_request(url, options).await {
          if let Some(Value::String(body_str)) = res.body {
               let lines: Vec<&str> = body_str.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
               
               if lines.iter().any(|l| l.starts_with("#EXTINF")) {
                   return load_hls(url, stream, false, true).await;
               }
               
               return true;
          }
     }
     false
}
