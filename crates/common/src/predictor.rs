
use async_trait::async_trait;
use std::error::Error;
use std::fmt;

/// Custom error type for prediction failures.
#[derive(Debug)]
pub struct PredictionError {
    details: String,
}

impl PredictionError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PredictionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PredictionError {}

/// Predictor trait for making predictions asynchronously.
#[async_trait]
pub trait Predictor {
    /// Perform a prediction based on the given input.
    ///
    /// # Arguments
    /// * `input` - The input data as a string.
    ///
    /// # Returns
    /// A `Result` containing the prediction as a `f64` or a `PredictionError`.
    async fn predict(&self, input: &str) -> Result<f64, PredictionError>;
}

/// A mock implementation of the `Predictor` trait for testing.
pub struct MockPredictor {
    pub fixed_value: f64,
}

#[async_trait]
impl Predictor for MockPredictor {
    async fn predict(&self, _input: &str) -> Result<f64, PredictionError> {
        Ok(self.fixed_value)
    }
}

/// Example implementation of the `Predictor` trait.
pub struct SimplePredictor;

#[async_trait]
impl Predictor for SimplePredictor {
    async fn predict(&self, input: &str) -> Result<f64, PredictionError> {
        input
            .parse::<f64>()
            .map_err(|_| PredictionError::new("Failed to parse input to f64."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_predictor() {
        let predictor = MockPredictor { fixed_value: 42.0 };
        let result = predictor.predict("any input").await.unwrap();
        assert_eq!(result, 42.0);
    }

    #[tokio::test]
    async fn test_simple_predictor_valid_input() {
        let predictor = SimplePredictor;
        let result = predictor.predict("123.45").await.unwrap();
        assert_eq!(result, 123.45);
    }

    #[tokio::test]
    async fn test_simple_predictor_invalid_input() {
        let predictor = SimplePredictor;
        let result = predictor.predict("invalid").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Failed to parse input to f64.");
    }
}
