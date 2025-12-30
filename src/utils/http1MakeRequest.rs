use super::types::http::{RequestOptions, ResponseResult};
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Method};
use serde_json::Value;
use std::io::Write;
use std::time::Duration;

pub async fn http1_make_request(
    url_str: &str,
    options: RequestOptions
) -> Result<ResponseResult, Box<dyn std::error::Error + Send + Sync>> {
    
    if options.redirects_followed >= options.max_redirects {
        return Err(format!("Too many redirects ({}) for {}", options.max_redirects, url_str).into());
    }

    let url = reqwest::Url::parse(url_str)?;
    
    let client_builder = Client::builder()
        .timeout(Duration::from_millis(options.timeout_ms))
        .redirect(reqwest::redirect::Policy::none());

    let client = client_builder.build()?;

    let mut final_headers = options.headers.clone();
    if !final_headers.contains_key(USER_AGENT) {
        final_headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"));
    }
    final_headers.insert("Accept-Encoding", HeaderValue::from_static("br, gzip, deflate"));

    let mut req_builder = client.request(options.method.clone(), url.clone())
        .headers(final_headers);

    if let Some(body) = &options.body {
        if !options.disable_body_compression && options.method != Method::GET && options.method != Method::HEAD {
             let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
             if let Ok(_) = encoder.write_all(body.as_bytes()) {
                 if let Ok(compressed) = encoder.finish() {
                     req_builder = req_builder.body(compressed);
                     req_builder = req_builder.header("Content-Encoding", "gzip");
                 } else {
                      req_builder = req_builder.body(body.clone());
                 }
             } else {
                  req_builder = req_builder.body(body.clone());
             }
        } else {
            req_builder = req_builder.body(body.clone());
        }
    }
    
    let result = req_builder.send().await?;
    let status = result.status();
    
    // Redirects
    if status.is_redirection() {
         if let Some(location) = result.headers().get("location") {
            let next_url = location.to_str()?;
            let new_url = url.join(next_url)?.to_string(); 
            
            let mut new_options = options.clone();
            new_options.redirects_followed += 1;
            
            let status_code = status.as_u16();
            if status_code == 301 || status_code == 302 || status_code == 303 {
                 if status_code == 303 || (status_code != 307 && status_code != 308 && options.method != Method::HEAD) {
                      new_options.method = Method::GET;
                      new_options.body = None;
                 }
            }
            return Box::pin(http1_make_request(&new_url, new_options)).await;
         }
    }

    if options.stream_only {
         return Ok(ResponseResult {
             status,
             headers: result.headers().clone(),
             body: None,
             raw_body: None 
         });
    }

    let headers = result.headers().clone();
    let bytes = result.bytes().await?;

    let is_json = headers.get(CONTENT_TYPE)
       .and_then(|v| v.to_str().ok())
       .map(|s| s.to_lowercase().starts_with("application/json"))
       .unwrap_or(false);

    let body_val = if is_json {
         serde_json::from_slice(&bytes).ok()
    } else {
         String::from_utf8(bytes.to_vec()).ok().map(Value::String)
    };

    Ok(ResponseResult {
        status,
        headers,
        body: body_val,
        raw_body: Some(bytes.to_vec())
    })
}
