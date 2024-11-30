use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// Define the error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

/// Define custom application errors
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    InternalError(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error(transparent)]
    Common(#[from] shared::errors::CommonError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Common(e) => e.status_code(),
            AppError::Other(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status)
            .json(ErrorResponse { code: status.as_u16(), message: self.to_string() })
    }
}
