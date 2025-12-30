use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Semver {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub pre: Option<String>,
}
