//! Module: Errors
//! Provides custom error types and utilities for the workspace.

use thiserror::Error;

/// Custom error type for the workspace.
#[derive(Debug, Error)]
pub enum SharedError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    #[error("Unknown error occurred.")]
    Unknown,
}

/// Converts a generic string error into a SharedError.
impl From<String> for SharedError {
    fn from(err: String) -> Self {
        SharedError::OperationFailed(err)
    }
}

/// Converts a &str error into a SharedError.
impl From<&str> for SharedError {
    fn from(err: &str) -> Self {
        SharedError::OperationFailed(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_error() {
        let error = SharedError::InvalidInput("Invalid data".into());
        assert_eq!(format!("{}", error), "Invalid input: Invalid data");
    }

    #[test]
    fn test_operation_failed_error() {
        let error = SharedError::OperationFailed("Failure reason".into());
        assert_eq!(format!("{}", error), "Operation failed: Failure reason");
    }

    #[test]
    fn test_unknown_error() {
        let error = SharedError::Unknown;
        assert_eq!(format!("{}", error), "Unknown error occurred.");
    }

    #[test]
    fn test_string_conversion() {
        let error: SharedError = "A string error".into();
        assert_eq!(format!("{}", error), "Operation failed: A string error");
    }
}
