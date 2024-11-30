use sqlx::postgres::{PgPool, PgPoolOptions};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub max_lifetime: Option<std::time::Duration>,
    pub idle_timeout: Option<std::time::Duration>,
    pub connect_timeout: std::time::Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://localhost/app".to_string(),
            max_connections: 5,
            min_connections: 1,
            max_lifetime: Some(std::time::Duration::from_secs(30 * 60)),
            idle_timeout: Some(std::time::Duration::from_secs(10 * 60)),
            connect_timeout: std::time::Duration::from_secs(3),
        }
    }
}

pub async fn create_pool(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .max_lifetime(config.max_lifetime)
        .idle_timeout(config.idle_timeout)
        .connect_timeout(config.connect_timeout)
        .connect(&config.url)
        .await
        .map_err(Into::into)
}
