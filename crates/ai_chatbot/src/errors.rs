use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use std::io;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Failed to connect to AI service: {0}")]
    ConnectionError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Service configuration error: {0}")]
    ConfigError(String),

    #[error("AI model error: {0}")]
    ModelError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for ChatError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ChatError::ConnectionError(_) => {
                HttpResponse::ServiceUnavailable().json(self.to_string())
            }
            ChatError::InvalidRequest(_) => {
                HttpResponse::BadRequest().json(self.to_string())
            }
            ChatError::ConfigError(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            ChatError::ModelError(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            ChatError::RateLimitError(_) => {
                HttpResponse::TooManyRequests().json(self.to_string())
            }
            _ => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum ChatbotError {
    #[error("Failed to initialize model: {0}")]
    ModelInitializationError(String),

    #[error("Error during model inference: {0}")]
    ModelError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Empty response from model")]
    EmptyResponse,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Model not loaded")]
    ModelNotLoaded,

    #[error("Context window exceeded")]
    ContextWindowExceeded,

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for ChatbotError {
    fn from(error: reqwest::Error) -> Self {
        ChatbotError::NetworkError(error.to_string())
    }
}

impl From<redis::RedisError> for ChatbotError {
    fn from(error: redis::RedisError) -> Self {
        ChatbotError::CacheError(error.to_string())
    }
}

// Implement conversion from anyhow::Error
impl From<anyhow::Error> for ChatbotError {
    fn from(error: anyhow::Error) -> Self {
        ChatbotError::Unknown(error.to_string())
    }
}

// Helper function to convert errors to API responses
pub fn error_to_response(error: ChatbotError) -> actix_web::HttpResponse {
    use actix_web::http::StatusCode;
    use actix_web::HttpResponse;

    let status = match error {
        ChatbotError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        ChatbotError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
        ChatbotError::AuthError(_) => StatusCode::UNAUTHORIZED,
        ChatbotError::ModelNotLoaded => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status)
        .json(serde_json::json!({
            "error": error.to_string(),
            "error_type": format!("{:?}", error),
        }))
}
