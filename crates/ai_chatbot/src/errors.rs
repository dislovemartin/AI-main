use std::io;
use shared::SharedError;
use thiserror::Error;
use serde_json::json;
use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};

#[derive(Error, Debug)]
pub enum ChatbotError {
    // Model-related errors
    #[error("Failed to initialize model: {0}")]
    ModelInitializationError(String),

    #[error("Error during model inference: {0}")]
    ModelError(String),

    #[error("Model not loaded")]
    ModelNotLoaded,

    #[error("Context window exceeded")]
    ContextWindowExceeded,

    // Request/Response errors
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Empty response from model")]
    EmptyResponse,

    // Rate limiting
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Infrastructure errors
    #[error("Failed to connect to service: {0}")]
    ConnectionError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    // Cache errors
    #[error("Cache error: {0}")]
    CacheError(String),

    // Configuration and authentication
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    // Standard error types
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    // Fallback errors
    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    Shared(#[from] SharedError),
}

impl ResponseError for ChatbotError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            // 400 Bad Request
            Self::InvalidRequest(_) | Self::InvalidInput(_) => 
                (StatusCode::BAD_REQUEST, "BAD_REQUEST"),

            // 401 Unauthorized
            Self::AuthError(_) => 
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),

            // 429 Too Many Requests
            Self::RateLimitExceeded => 
                (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMIT_EXCEEDED"),

            // 422 Unprocessable Entity
            Self::ModelError(_) | Self::ConfigError(_) | Self::ContextWindowExceeded => 
                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY"),

            // 503 Service Unavailable
            Self::ConnectionError(_) | Self::ModelNotLoaded | Self::ModelInitializationError(_) => 
                (StatusCode::SERVICE_UNAVAILABLE, "SERVICE_UNAVAILABLE"),

            // 500 Internal Server Error
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        HttpResponse::build(status).json(json!({
            "error": {
                "message": self.to_string(),
                "type": error_type,
                "details": format!("{:?}", self)
            }
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidRequest(_) | Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::AuthError(_) => StatusCode::UNAUTHORIZED,
            Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            Self::ModelError(_) | Self::ConfigError(_) | Self::ContextWindowExceeded => 
                StatusCode::UNPROCESSABLE_ENTITY,
            Self::ConnectionError(_) | Self::ModelNotLoaded | Self::ModelInitializationError(_) => 
                StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// From implementations for external error types
impl From<reqwest::Error> for ChatbotError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            ChatbotError::ConnectionError("Request timeout".to_string())
        } else if error.is_connect() {
            ChatbotError::ConnectionError("Connection failed".to_string())
        } else {
            ChatbotError::NetworkError(error.to_string())
        }
    }
}

impl From<redis::RedisError> for ChatbotError {
    fn from(error: redis::RedisError) -> Self {
        ChatbotError::CacheError(error.to_string())
    }
}

// Convert internal errors to ChatbotError
impl From<anyhow::Error> for ChatbotError {
    fn from(error: anyhow::Error) -> Self {
        ChatbotError::Unknown(error.to_string())
    }
}
