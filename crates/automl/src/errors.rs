use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum AutoMLError {
    #[error("Failed to initialize model: {0}")]
    ModelInitializationError(String),

    #[error("Error during model inference: {0}")]
    ModelError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Neural architecture error: {0}")]
    ArchitectureError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Feature extraction error: {0}")]
    FeatureExtractionError(String),

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("Hardware error: {0}")]
    HardwareError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("SQL error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Optuna error: {0}")]
    OptunaError(#[from] optuna::OptunaError),

    #[error("PyTorch error: {0}")]
    TorchError(#[from] tch::TchError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for AutoMLError {
    fn from(error: anyhow::Error) -> Self {
        AutoMLError::Unknown(error.to_string())
    }
}

// Helper function to convert errors to API responses
pub fn error_to_response(error: AutoMLError) -> actix_web::HttpResponse {
    use actix_web::http::StatusCode;
    use actix_web::HttpResponse;

    let status = match error {
        AutoMLError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        AutoMLError::ResourceExhausted(_) => StatusCode::TOO_MANY_REQUESTS,
        AutoMLError::HardwareError(_) => StatusCode::SERVICE_UNAVAILABLE,
        AutoMLError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
        AutoMLError::CacheError(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status)
        .json(serde_json::json!({
            "error": error.to_string(),
            "error_type": format!("{:?}", error),
        }))
}

// Implement conversion for common error types
impl From<std::num::ParseFloatError> for AutoMLError {
    fn from(error: std::num::ParseFloatError) -> Self {
        AutoMLError::InvalidInput(error.to_string())
    }
}

impl From<std::num::ParseIntError> for AutoMLError {
    fn from(error: std::num::ParseIntError) -> Self {
        AutoMLError::InvalidInput(error.to_string())
    }
}

impl From<hyperopt::Error> for AutoMLError {
    fn from(error: hyperopt::Error) -> Self {
        AutoMLError::OptimizationError(error.to_string())
    }
}

// Custom result type for automl operations
pub type AutoMLResult<T> = Result<T, AutoMLError>; 