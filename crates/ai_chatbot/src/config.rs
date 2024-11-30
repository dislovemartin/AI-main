use serde::Deserialize;
use std::env;
use crate::errors::ChatError;

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    /// The API endpoint for the AI service
    pub api_endpoint: String,
    /// API key for authentication
    pub api_key: String,
    /// Organization ID if applicable
    pub org_id: Option<String>,
    /// Default model configuration
    pub default_model: String,
    /// Maximum allowed tokens in response
    pub max_tokens: usize,
    /// Default temperature for responses
    pub temperature: f32,
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    /// Maximum concurrent requests
    pub max_concurrent: u32,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, ChatError> {
        Ok(Self {
            api_endpoint: env::var("AI_SERVICE_ENDPOINT")
                .map_err(|_| ChatError::ConfigError("AI_SERVICE_ENDPOINT not set".to_string()))?,
            api_key: env::var("AI_SERVICE_API_KEY")
                .map_err(|_| ChatError::ConfigError("AI_SERVICE_API_KEY not set".to_string()))?,
            org_id: env::var("AI_SERVICE_ORG_ID").ok(),
            default_model: env::var("AI_SERVICE_MODEL")
                .unwrap_or_else(|_| "gpt-4".to_string()),
            max_tokens: env::var("AI_SERVICE_MAX_TOKENS")
                .unwrap_or_else(|_| "2048".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_SERVICE_MAX_TOKENS".to_string()))?,
            temperature: env::var("AI_SERVICE_TEMPERATURE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_SERVICE_TEMPERATURE".to_string()))?,
            rate_limit: RateLimitConfig {
                requests_per_minute: env::var("AI_SERVICE_RATE_LIMIT")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .map_err(|_| ChatError::ConfigError("Invalid AI_SERVICE_RATE_LIMIT".to_string()))?,
                max_concurrent: env::var("AI_SERVICE_MAX_CONCURRENT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .map_err(|_| ChatError::ConfigError("Invalid AI_SERVICE_MAX_CONCURRENT".to_string()))?,
            },
        })
    }
} 