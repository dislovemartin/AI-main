use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("Failed to initialize model: {0}")]
    ModelInitializationError(String),

    #[error("Error during model inference: {0}")]
    ModelError(String),

    #[error("Model not initialized")]
    ModelNotInitialized,

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Series not found: {0}")]
    SeriesNotFound(String),

    #[error("Feature extraction error: {0}")]
    FeatureExtractionError(String),

    #[error("Data preprocessing error: {0}")]
    PreprocessingError(String),

    #[error("Invalid time frequency: {0}")]
    InvalidFrequency(String),

    #[error("Insufficient data: {0}")]
    InsufficientData(String),

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

impl From<anyhow::Error> for PredictionError {
    fn from(error: anyhow::Error) -> Self {
        PredictionError::Unknown(error.to_string())
    }
}

// Helper function to convert errors to API responses
pub fn error_to_response(error: PredictionError) -> actix_web::HttpResponse {
    use actix_web::http::StatusCode;
    use actix_web::HttpResponse;

    let status = match error {
        PredictionError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        PredictionError::SeriesNotFound(_) => StatusCode::NOT_FOUND,
        PredictionError::ModelNotInitialized => StatusCode::SERVICE_UNAVAILABLE,
        PredictionError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
        PredictionError::CacheError(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status)
        .json(serde_json::json!({
            "error": error.to_string(),
            "error_type": format!("{:?}", error),
        }))
}

// Implement conversion for common error types
impl From<tch::TchError> for PredictionError {
    fn from(error: tch::TchError) -> Self {
        PredictionError::ModelError(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for PredictionError {
    fn from(error: std::num::ParseFloatError) -> Self {
        PredictionError::InvalidInput(error.to_string())
    }
}

impl From<std::num::ParseIntError> for PredictionError {
    fn from(error: std::num::ParseIntError) -> Self {
        PredictionError::InvalidInput(error.to_string())
    }
}

impl From<chrono::ParseError> for PredictionError {
    fn from(error: chrono::ParseError) -> Self {
        PredictionError::InvalidInput(error.to_string())
    }
}

// Custom result type for prediction operations
pub type PredictionResult<T> = Result<T, PredictionError>; 