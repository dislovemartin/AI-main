use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// Define the error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Define custom application errors
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
    // TODO: Add other variants as needed
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();

        HttpResponse::build(status_code).json(ErrorResponse {
            error: self.to_string(),
            timestamp: chrono::Utc::now(),
        })
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type CommonResult<T> = Result<T, CommonError>;
