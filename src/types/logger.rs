pub struct FileConfig {
    pub enabled: bool,
    pub path: Option<String>,
}

pub struct LoggingConfig {
    pub file: Option<FileConfig>,
    pub level: Option<String>,
}
pub struct Config {
    pub logging: Option<LoggingConfig>,
}
