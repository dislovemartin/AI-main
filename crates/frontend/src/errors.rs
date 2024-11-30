use actix_web::{HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            AppError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            error: self.to_string(),
            timestamp: Utc::now(),
        })
    }
}
