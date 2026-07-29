use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    // Minecraft section is the only required section
    // with game_version and java_version specified
    pub minecraft: MinecraftConfig,
    #[serde(default)]
    pub cors: CorsConfig,
    #[serde(default)]
    pub rcon: RconConfig,
}

// TODO: Error handling
// Custom Error type - ApiError
impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        Self::apply_defaults(&mut config);
        Ok(config)
    }

    fn apply_defaults(&mut self) {
        if self.server.log_level.is_none() {
            self.server.log_level = Some(if self.server.dev_mode {LogLevel::Debug} else {LogLevel::Info});
        }

        if self.cors.allowed_origins.is_none() {
            self.cors.allowed_origins = Some(if self.server.dev_mode {
                vec![
                    "http://localhost:*".to_string(),
                    "tauri://localhost".to_string(),
                ]
            } else {
                Vec::new()
            });
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub port: u16,
    pub dev_mode: bool,
    pub log_level: Option<LogLevel>,
    pub log_format: LogFormat,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            dev_mode: false,
            log_level: None,
            log_format: LogFormat::Json,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
        }
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warning => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Plain,
    Json,
}

impl LogFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogFormat::Plain => "plain",
            LogFormat::Json => "json",
        }
    }
}

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Plain
    }
}

#[derive(Debug, Deserialize)]
pub struct MinecraftConfig {
    pub game_version: String,
    pub java_version: String,
    #[serde(default = "default_instance_path")]
    pub instance_path: PathBuf,
}

fn default_instance_path() -> PathBuf {
    PathBuf::from("./instance/default.zip")
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CorsConfig {
    pub allowed_origins: Option<Vec<String>>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RconConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl Default for RconConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "127.0.0.1".to_string(),
            port: 25575,
            password: String::new(),
        }
    }
}
