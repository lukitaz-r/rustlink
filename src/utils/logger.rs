use chrono::Utc;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Sources,
    Started,
    Network,
}

impl LogLevel {
    fn from_str(s: &str) -> Self {
        match s {
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            "sources" => LogLevel::Sources,
            "started" => LogLevel::Started,
            "network" => LogLevel::Network,
            _ => LogLevel::Info,
        }
    }
    fn details(&self) -> (&'static str, &'static str) {
        match self {
            LogLevel::Info => ("INFO", "\x1b[1m\x1b[3;42m"),
            LogLevel::Warn => ("WARN", "\x1b[1m\x1b[3;43m"),
            LogLevel::Error => ("ERROR", "\x1b[1m\x1b[3;41m"),
            LogLevel::Debug => ("DEBUG", "\x1b[1m\x1b[3;45m"),
            LogLevel::Sources => ("SOURCES", "\x1b[1m\x1b[3;46m"),
            LogLevel::Started => ("STARTED", "\x1b[1m\x1b[3;44m"),
            LogLevel::Network => ("NETWORK", "\x1b[1m\x1b[3;44m"),
        }
    }

    fn index(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info | LogLevel::Sources | LogLevel::Started | LogLevel::Network => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        }
    }
}

pub static LOG_FILE: Lazy<Arc<Mutex<Option<File>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
use serde_json::Value;

pub fn logger(level_str: &str, category: &str, message: &str, data: Option<&Value>) {
    let level = LogLevel::from_str(level_str);
    let (label, color_code) = level.details();
    let reset_color = "\x1b[0m";
    let time = Utc::now().format("%H:%M:%S.%3f").to_string();

    let formatted_category = if !category.is_empty() {
        format!(": {} >", category)
    } else {
        String::new()
    };

    let msg = if let Some(d) = data {
        format!("{} {}", message, d)
    } else {
        message.to_string()
    };

    println!(
        "[{}] {}[{}]{} >{} {}",
        time, color_code, label, reset_color, formatted_category, msg
    );
    if let Ok(mut file_lock) = LOG_FILE.lock() {
        if let Some(file) = file_lock.as_mut() {
            let file_output = format!(
                "[{}] [{}] {} {}\n",
                Utc::now().to_rfc3339(),
                label,
                formatted_category,
                msg
            );
            let _ = file.write_all(file_output.as_bytes());
        }
    }
}
