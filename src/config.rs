#![allow(dead_code)]

use crate::error::Result;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub api: ApiConfig,
    pub minecraft: MinecraftConfig,
    pub cors: CorsConfig,
    pub rcon: RconConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let raw: RawConfig = toml::from_str(&content)?;
        Ok(raw.into_config())
    }
}

#[derive(Debug)]
pub struct ApiConfig {
    pub port: u16,
    pub dev_mode: bool,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
}

#[derive(Debug)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftConfig {
    pub active_season: String,
    #[serde(default = "default_sync_path")]
    pub sync_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct SeasonManifest {
    pub name: String,
    pub description: String,
    pub game_version: String,
    pub java_version: String,
    pub server_ip: String,
    pub server_port: u16,
}

pub fn default_sync_path() -> PathBuf {
    PathBuf::from("./sync")
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
            LogLevel::Warning => "warn",
            LogLevel::Error => "error",
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
struct RawConfig {
    #[serde(default)]
    api: RawApiConfig,
    minecraft: MinecraftConfig,
    #[serde(default)]
    cors: RawCorsConfig,
    #[serde(default)]
    rcon: RconConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct RawApiConfig {
    port: u16,
    dev_mode: bool,
    log_level: Option<LogLevel>,
    log_format: LogFormat,
}

impl Default for RawApiConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            dev_mode: false,
            log_level: None,
            log_format: LogFormat::Json,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct RawCorsConfig {
    allowed_origins: Option<Vec<String>>,
}

impl RawConfig {
    fn into_config(self) -> Config {
        let dev_mode = self.api.dev_mode;

        let log_level = self.api.log_level.unwrap_or_else(|| {
            if dev_mode { LogLevel::Debug } else { LogLevel::Info }
        });

        let mut allowed_origins = self.cors.allowed_origins.unwrap_or(Vec::new());
        if dev_mode {
            allowed_origins.push("http://localhost:*".to_string());
            allowed_origins.push("tauri://localhost".to_string());
        }

        Config {
            api: ApiConfig {
                port: self.api.port,
                dev_mode,
                log_level,
                log_format: self.api.log_format,
            },
            minecraft: self.minecraft,
            cors: CorsConfig { allowed_origins },
            rcon: self.rcon,
        }
    }
}
