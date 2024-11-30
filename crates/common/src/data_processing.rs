use crate::errors::CommonError;
use serde_json::from_str;
use serde_json::Value;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

/// Validates the input JSON string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the JSON data to validate.
///
/// # Returns
///
/// * `Result<Value, CommonError>` - The parsed JSON value or an error if validation fails.
pub fn validate_input(input: &str) -> Result<Value, CommonError> {
    let parsed: Value = from_str(input)
        .map_err(|_| CommonError::InvalidFormatError("Failed to parse JSON".to_string()))?;

    if parsed.is_object() {
        Ok(parsed)
    } else {
        Err(CommonError::InvalidFormatError(
            "Invalid format".to_string(),
        ))
    }
}

pub fn preprocess_text(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Sanitizes the input by removing non-alphanumeric and non-whitespace characters.
///
/// # Arguments
///
/// * `input` - A string slice to sanitize.
///
/// # Returns
///
/// * `String` - The sanitized input string.
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

pub trait DataProcessor {
    fn process_data(&self, input: &str) -> Result<String>;
    fn validate_input(&self, input: &str) -> Result<bool>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonDataProcessor;

impl DataProcessor for CommonDataProcessor {
    fn process_data(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(anyhow!("Input data is empty"));
        }
        Ok(input.trim().to_lowercase())
    }

    fn validate_input(&self, input: &str) -> Result<bool> {
        Ok(!input.trim().is_empty())
    }
}
