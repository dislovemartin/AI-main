#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub shutdown_timeout: u64,
    pub log_level: String,
    pub environment: Environment,
    pub jaeger_endpoint: Option<String>,
    pub metrics_port: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::builder();

        // Add default configuration
        settings = settings.add_source(config::File::with_name("config/default"));

        // Add environment-specific configuration
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        settings = settings.add_source(
            config::File::with_name(&format!("config/{}", env))
                .required(false),
        );

        // Add local configuration override
        settings = settings.add_source(
            config::File::with_name("config/local")
                .required(false),
        );

        // Add environment variables with prefix "AUTOML_"
        settings = settings.add_source(
            config::Environment::with_prefix("AUTOML")
                .separator("_"),
        );

        // Build configuration
        let settings = settings.build()?;

        Ok(Config {
            database_url: settings.get_string("database.url")?,
            redis_url: settings.get_string("redis.url")?,
            host: settings.get_string("server.host")?,
            port: settings.get_int("server.port")? as u16,
            workers: settings.get_int("server.workers").ok().map(|w| w as usize),
            shutdown_timeout: settings.get_int("server.shutdown_timeout")? as u64,
            log_level: settings.get_string("logging.level")?,
            environment: match settings.get_string("environment")?.as_str() {
                "production" => Environment::Production,
                "staging" => Environment::Staging,
                _ => Environment::Development,
            },
            jaeger_endpoint: settings.get_string("telemetry.jaeger_endpoint").ok(),
            metrics_port: settings.get_int("metrics.port")? as u16,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        std::env::set_var("AUTOML_DATABASE_URL", "postgres://test:test@localhost/test");
        std::env::set_var("AUTOML_REDIS_URL", "redis://localhost:6379");
        
        let config = Config::new().expect("Failed to load config");
        
        assert_eq!(config.environment, Environment::Development);
        assert!(!config.is_production());
    }

    #[test]
    fn test_environment_override() {
        std::env::set_var("AUTOML_ENVIRONMENT", "production");
        
        let config = Config::new().expect("Failed to load config");
        
        assert_eq!(config.environment, Environment::Production);
        assert!(config.is_production());
    }
} 