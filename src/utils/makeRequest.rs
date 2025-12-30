use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rand::Rng;
use serde_json::{json, Value};
use reqwest::{header::*, Method, StatusCode, Client};
use std::time::Duration;

use super::types::http::{RequestOptions, ResponseResult, NodelinkMock};
use super::logger::logger;
use super::http1_make_request::http1_make_request;

static HTTP2_FAILED_HOSTS: Lazy<Arc<Mutex<HashSet<String>>>> = Lazy::new(|| Arc::new(Mutex::new(HashSet::new())));
const REDIRECT_STATUS_CODES: [u16; 5] = [301, 302, 303, 307, 308];

pub async fn make_request(
    url_string: &str,
    mut options: RequestOptions,
    nodelink: &NodelinkMock,
) -> Result<ResponseResult, Box<dyn std::error::Error + Send + Sync>> {
    let log_id = hex::encode(rand::thread_rng().r#gen::<[u8; 4]>());
    
    // Logging (Mocking loggingConfig check for now)
    logger("debug", "Network", &format!("[{}] Request: {} {}", log_id, options.method, url_string), None);
    
    let mut redacted_headers = HeaderMap::new();
    for (key, value) in &options.headers {
        let key_str = key.as_str().to_lowercase();
        if key_str.contains("authorization") || key_str.contains("cookie") {
            redacted_headers.insert(key.clone(), HeaderValue::from_static("[REDACTED]"));
        } else {
            redacted_headers.insert(key.clone(), value.clone());
        }
    }
    logger("debug", "Network", &format!("[{}] Headers: {:?}", log_id, redacted_headers), None);

    if let Some(body) = &options.body {
        let body_snippet = if body.len() > 200 {
            format!("{}...", &body[..200])
        } else {
            body.clone()
        };
        logger("debug", "Network", &format!("[{}] Body: {}", log_id, body_snippet), None);
    }

    if options.redirects_followed >= options.max_redirects {
        return Err(format!("Too many redirects ({}) for {}", options.max_redirects, url_string).into());
    }

    let local_address = nodelink.route_planner.as_ref().and_then(|rp| rp.get_ip());
    options.local_address = local_address.clone();

    let url = match reqwest::Url::parse(url_string) {
        Ok(u) => u,
        Err(_) => return http1_make_request(url_string, options).await,
    };

    let host = url.host_str().unwrap_or("").to_string();
    {
        let failed_hosts = HTTP2_FAILED_HOSTS.lock().unwrap();
        if failed_hosts.contains(&host) {
            return http1_make_request(url_string, options).await;
        }
    }

    // Attempt HTTP/2 request
    let client = match Client::builder()
        .http2_prior_knowledge() // Force H2 to see if it works
        .timeout(Duration::from_millis(options.timeout_ms))
        .redirect(reqwest::redirect::Policy::none())
        .build() {
            Ok(c) => c,
            Err(_) => return http1_make_request(url_string, options).await,
        };

    let mut req_builder = client.request(options.method.clone(), url.clone())
        .headers(options.headers.clone());
    
    // Add default H2 headers from Node.js logic
    if !options.headers.contains_key(USER_AGENT) {
        req_builder = req_builder.header(USER_AGENT, "Mozilla/5.0 (Node.js Http2Client)");
    }
    req_builder = req_builder.header(ACCEPT_ENCODING, "br, gzip, deflate");
    req_builder = req_builder.header("dnt", "1");

    if let Some(body) = &options.body {
        if options.method != Method::GET && options.method != Method::HEAD {
            if !options.headers.contains_key(CONTENT_TYPE) {
                // Determine content type if not present
                if body.starts_with('{') || body.starts_with('[') {
                    req_builder = req_builder.header(CONTENT_TYPE, "application/json");
                }
            }
            // Body compression (gzip) is handled in http1_make_request, 
            // but the Node.js code does it here too.
            // For simplicity and since reqwest doesn't automatically gzip request bodies unless configured,
            // we can just pass the body. If the user wants manual compression like in the Node.js code:
            if !options.disable_body_compression {
                use flate2::write::GzEncoder;
                use flate2::Compression;
                use std::io::Write;
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                if encoder.write_all(body.as_bytes()).is_ok() {
                    if let Ok(compressed) = encoder.finish() {
                        req_builder = req_builder.body(compressed);
                        req_builder = req_builder.header(CONTENT_ENCODING, "gzip");
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
    }

    let result = match req_builder.send().await {
        Ok(res) => res,
        Err(e) => {
            // If it's a protocol error or connection error, add to failed hosts
            if e.is_connect() || e.is_timeout() || e.to_string().contains("protocol error") {
                HTTP2_FAILED_HOSTS.lock().unwrap().insert(host);
            }
            return http1_make_request(url_string, options).await;
        }
    };

    let status = result.status();
    let status_u16 = status.as_u16();

    if status_u16 == 429 {
        if let Some(rp) = &nodelink.route_planner {
            rp.ban_ip(local_address);
        }
    }

    // Handle Redirects
    if REDIRECT_STATUS_CODES.contains(&status_u16) {
        if let Some(location) = result.headers().get(LOCATION) {
            if let Ok(loc_str) = location.to_str() {
                let new_location = url.join(loc_str)?.to_string();
                let mut next_method = options.method.clone();
                let mut next_body = options.body.clone();

                if (status_u16 == 301 || status_u16 == 302) && 
                   [Method::POST, Method::PUT, Method::DELETE].contains(&options.method) {
                    next_method = Method::GET;
                    next_body = None;
                } else if status_u16 == 303 {
                    next_method = Method::GET;
                    next_body = None;
                }

                let mut new_options = options.clone();
                new_options.method = next_method;
                new_options.body = next_body;
                new_options.redirects_followed += 1;
                new_options.disable_body_compression = if new_options.body.is_some() { options.disable_body_compression } else { false };

                return Box::pin(make_request(&new_location, new_options, nodelink)).await;
            }
        }
    }

    if options.method == Method::HEAD {
        return Ok(ResponseResult {
            status,
            headers: result.headers().clone(),
            body: None,
            raw_body: None,
        });
    }

    if options.stream_only {
        return Ok(ResponseResult {
            status,
            headers: result.headers().clone(),
            body: None,
            raw_body: None, // In a real app, you'd pass the stream here, but ResponseResult doesn't have it yet
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

    // Log response
    let res_body_snippet = match &body_val {
        Some(Value::String(s)) => if s.len() > 200 { format!("{}...", &s[..200]) } else { s.clone() },
        Some(v) => {
            let s = v.to_string();
            if s.len() > 200 { format!("{}...", &s[..200]) } else { s }
        },
        None => "".to_string(),
    };
    logger("debug", "Network", &format!("[{}] Response Status: {}", log_id, status_u16), None);
    logger("debug", "Network", &format!("[{}] Response Body: {}", log_id, res_body_snippet), None);

    Ok(ResponseResult {
        status,
        headers,
        body: body_val,
        raw_body: Some(bytes.to_vec()),
    })
}
