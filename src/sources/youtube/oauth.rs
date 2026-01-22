use serde_json::{json, Value};
use crate::utils::{make_request, logger};
use crate::types::http::{RequestOptions, NodelinkMock};

pub struct OAuth {
    refresh_token: Option<String>,
    access_token: Option<String>,
}

impl OAuth {
    pub fn new() -> Self {
        Self {
            refresh_token: None,
            access_token: None,
        }
    }

    pub async fn get_access_token(&mut self, _nodelink: &NodelinkMock) -> Option<String> {
        // Mock
        self.access_token.clone()
    }
}
