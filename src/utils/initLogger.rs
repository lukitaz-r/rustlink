use super::types::logger::{Config, LoggingConfig};
use chrono::Utc;
use rand::random;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use sysinfo::{System, SystemExt};

pub fn init_file_logger(config: &LoggingConfig) -> Option<File> {
    let file_config = config.file.as_ref()?;
    if !file_config.enabled {
        return None;
    }
    let default_path = "logs".to_string();
    let log_dir_str = file_config.path.as_ref().unwrap_or(&default_path);
    let log_dir = Path::new(log_dir_str);
    if !log_dir.exists() {
        fs::create_dir_all(log_dir).expect("Error creando directorio de logs");
    }
    let timestamp = Utc::now().to_rfc3339().replace(":", "-").replace(".", "-");
    let random_bytes: [u8; 4] = random();
    let random_id = hex::encode(random_bytes);
    let log_file_name = format!("{}-{}.log", timestamp, random_id);
    let log_file_path = log_dir.join(log_file_name);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Error abriendo archivo de log");
    let mut sys = System::new_all();
    sys.refresh_all();
    let os_info = format!(
        "{} {}",
        sys.name().unwrap_or_default(),
        sys.os_version().unwrap_or_default()
    );
    let initial_info = format!(
        "\n--- NodeLink Log ---\nTimestamp: {}\nVersion: 0.0.1\nOS: {}\n--------------------\n",
        Utc::now().to_rfc3339(),
        os_info
    );
    if let Err(e) = file.write_all(initial_info.as_bytes()) {
        eprintln!("No se pudo escribir el header del log: {}", e);
    }
    Some(file)
}

pub fn init_logger(config: Config) -> Option<File> {
    let logging_config = config.logging.unwrap_or(LoggingConfig {
        file: None,
        level: Some("info".to_string()),
    });

    init_file_logger(&logging_config)
}
