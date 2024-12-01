use crate::errors::ApiError;
use actix_web::{HttpResponse, ResponseError, http::StatusCode, web};
use serde::Serialize;
use thiserror::Error;

/// Define API-related error types
#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Prediction failed: {0}")]
    PredictionFailed(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApiError::PredictionFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string(),
            "code": self.status_code().as_u16()
        }))
    }
}
