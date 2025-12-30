use reqwest::{header::HeaderMap, Method, StatusCode};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Option<String>,
    pub timeout_ms: u64,
    pub stream_only: bool,
    pub disable_body_compression: bool,
    pub max_redirects: usize,
    pub redirects_followed: usize,
    pub local_address: Option<String>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            method: Method::GET,
            headers: HeaderMap::new(),
            body: None,
            timeout_ms: 30000,
            stream_only: false,
            disable_body_compression: false,
            max_redirects: 5,
            redirects_followed: 0,
            local_address: None,
        }
    }
}

pub struct ResponseResult {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Option<Value>,
    pub raw_body: Option<Vec<u8>>,
}

// Mocks for Nodelink
pub struct RoutePlannerMock {
    // fields
}
impl RoutePlannerMock {
    pub fn get_ip(&self) -> Option<String> {
        None
    }
    pub fn ban_ip(&self, _ip: Option<String>) {}
}

pub struct NodelinkMock {
    pub route_planner: Option<RoutePlannerMock>,
}
