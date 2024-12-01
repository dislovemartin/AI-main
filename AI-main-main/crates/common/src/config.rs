use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub auth: AuthConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub idle_timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub redis_url: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiration_hours: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub metrics_enabled: bool,
    pub database_url: Option<String>,
}

impl ServiceConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn validate(&self) -> Result<()> {
        // Add validation logic
        Ok(())
    }
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
}
