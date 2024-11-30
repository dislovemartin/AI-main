use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use thiserror::Error;

/// Application-wide errors.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized access")]
    Unauthorized,
}

/// Standardized error response for JSON serialization.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    pub timestamp: String,
}

impl ErrorResponse {
    /// Creates a new ErrorResponse instance.
    pub fn new(message: String, code: u16) -> Self {
        Self { error: message, code, timestamp: chrono::Utc::now().to_rfc3339() }
    }
}

/// Map application errors to HTTP responses.
impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let status_code = self.status_code().as_u16();
        let error_response = ErrorResponse::new(error_message, status_code);
        HttpResponse::build(self.status_code()).json(error_response)
    }
}
