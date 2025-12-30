#[derive(Debug)]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
    pub url: Option<String>,
    pub codename: Option<String>,
    pub release_date: Option<String>,
}
