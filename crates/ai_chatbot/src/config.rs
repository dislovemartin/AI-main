use serde::Deserialize;
use std::env;
use crate::errors::ChatError;
use crate::services::HuggingFaceConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    /// The API endpoint for the AI service
    pub api_endpoint: String,
    /// API key for authentication
    pub api_key: String,
    /// Organization ID if applicable
    pub org_id: Option<String>,
    /// Model-specific configuration
    pub model_config: ModelConfig,
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    /// HuggingFace model configuration
    pub huggingface: HuggingFaceConfig,
    /// Whether to use streaming responses
    pub stream: bool,
    /// Cache configuration
    pub cache_config: CacheConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    /// Whether to enable response caching
    pub enabled: bool,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Maximum cache size in MB
    pub max_size_mb: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    /// Maximum concurrent requests
    pub max_concurrent: u32,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_seconds: 3600,
            max_size_mb: 1024,
        }
    }
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, ChatError> {
        let huggingface_config = HuggingFaceConfig {
            model_name: env::var("AI_MODEL_NAME")
                .unwrap_or_else(|_| "gpt2".to_string()),
            max_length: env::var("AI_MAX_LENGTH")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_MAX_LENGTH".to_string()))?,
            num_beams: env::var("AI_NUM_BEAMS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_NUM_BEAMS".to_string()))?,
            temperature: env::var("AI_TEMPERATURE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_TEMPERATURE".to_string()))?,
            top_k: env::var("AI_TOP_K")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_TOP_K".to_string()))?,
            top_p: env::var("AI_TOP_P")
                .unwrap_or_else(|_| "0.9".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_TOP_P".to_string()))?,
            do_sample: env::var("AI_DO_SAMPLE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_DO_SAMPLE".to_string()))?,
            repetition_penalty: env::var("AI_REPETITION_PENALTY")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse()
                .map_err(|_| ChatError::ConfigError("Invalid AI_REPETITION_PENALTY".to_string()))?,
        };

        Ok(Self {
            api_endpoint: env::var("AI_SERVICE_ENDPOINT")
                .map_err(|_| ChatError::ConfigError("AI_SERVICE_ENDPOINT not set".to_string()))?,
            api_key: env::var("AI_SERVICE_API_KEY")
                .map_err(|_| ChatError::ConfigError("AI_SERVICE_API_KEY not set".to_string()))?,
            org_id: env::var("AI_SERVICE_ORG_ID").ok(),
            model_config: ModelConfig {
                huggingface: huggingface_config,
                stream: env::var("AI_STREAM_ENABLED")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .map_err(|_| ChatError::ConfigError("Invalid AI_STREAM_ENABLED".to_string()))?,
                cache_config: CacheConfig::default(),
            },
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