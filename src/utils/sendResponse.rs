use axum::{
    body::Body,
    http::{
        header::{HeaderMap, ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_TYPE},
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use flate2::{
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
use serde_json::Value;
use std::io::Write;

pub fn send_response(
    req_headers: &HeaderMap,
    data: Option<Value>,
    status: StatusCode,
    trace: bool,
) -> Response {
    let mut response_headers = HeaderMap::new();

    let mut final_data = match data {
        Some(v) => v,
        None => return (status, response_headers).into_response(),
    };

    // Filter "trace" if trace is false
    if let Value::Object(ref mut map) = final_data {
        if map.contains_key("trace") && !trace {
            map.remove("trace");
        }
    }

    response_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let json_data = serde_json::to_string(&final_data).unwrap_or_default();
    let encoding_header = req_headers
        .get(ACCEPT_ENCODING)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Compression logic
    if encoding_header.contains("br") {
        let mut compressor = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        if compressor.write_all(json_data.as_bytes()).is_ok() {
            // brotli's into_inner returns the inner writer directly (Vec<u8>)
            let result = compressor.into_inner();
            response_headers.insert(CONTENT_ENCODING, "br".parse().unwrap());
            return (status, response_headers, Body::from(result)).into_response();
        }
    } else if encoding_header.contains("gzip") {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(json_data.as_bytes()).is_ok() {
            if let Ok(result) = encoder.finish() {
                response_headers.insert(CONTENT_ENCODING, "gzip".parse().unwrap());
                return (status, response_headers, Body::from(result)).into_response();
            }
        }
    } else if encoding_header.contains("deflate") {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(json_data.as_bytes()).is_ok() {
            if let Ok(result) = encoder.finish() {
                response_headers.insert(CONTENT_ENCODING, "deflate".parse().unwrap());
                return (status, response_headers, Body::from(result)).into_response();
            }
        }
    }

    // Fallback: No compression
    (status, response_headers, Body::from(json_data)).into_response()
}
