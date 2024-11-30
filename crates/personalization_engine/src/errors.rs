use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecommendationError {
    #[error("Failed to initialize model: {0}")]
    ModelInitializationError(String),

    #[error("Error during model inference: {0}")]
    ModelError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("Feature extraction error: {0}")]
    FeatureExtractionError(String),

    #[error("Training data error: {0}")]
    TrainingDataError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("SQL error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for RecommendationError {
    fn from(error: anyhow::Error) -> Self {
        RecommendationError::Unknown(error.to_string())
    }
}

// Helper function to convert errors to API responses
pub fn error_to_response(error: RecommendationError) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use actix_web::http::StatusCode;

    let status = match error {
        RecommendationError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        RecommendationError::UserNotFound(_) => StatusCode::NOT_FOUND,
        RecommendationError::ItemNotFound(_) => StatusCode::NOT_FOUND,
        RecommendationError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
        RecommendationError::CacheError(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status).json(serde_json::json!({
        "error": error.to_string(),
        "error_type": format!("{:?}", error),
    }))
}

// Implement conversion for common error types
impl From<tch::TchError> for RecommendationError {
    fn from(error: tch::TchError) -> Self {
        RecommendationError::ModelError(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for RecommendationError {
    fn from(error: std::num::ParseFloatError) -> Self {
        RecommendationError::InvalidInput(error.to_string())
    }
}

impl From<std::num::ParseIntError> for RecommendationError {
    fn from(error: std::num::ParseIntError) -> Self {
        RecommendationError::InvalidInput(error.to_string())
    }
}

// Custom result type for recommendation operations
pub type RecommendationResult<T> = Result<T, RecommendationError>;
