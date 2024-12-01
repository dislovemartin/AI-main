/// A trait defining the basic operations for any machine learning model.
pub trait Model {
    /// Trains the model using the given features and labels.
    fn train(&mut self, features: &[Vec<f64>], labels: &[f64]);

    /// Makes predictions for the given features.
    fn predict(&self, features: &[Vec<f64>]) -> Vec<f64>;
}

/// A simple linear regression model.
pub struct LinearRegression {
    weights: Vec<f64>,
    bias: f64,
}

impl LinearRegression {
    /// Creates a new instance of the LinearRegression model.
    pub fn new(num_features: usize) -> Self {
        Self { weights: vec![0.0; num_features], bias: 0.0 }
    }
}

impl Model for LinearRegression {
    fn train(&mut self, features: &[Vec<f64>], labels: &[f64]) {
        // Placeholder: Add gradient descent or another optimization logic here
        println!("Training linear regression model with {} samples", features.len());
        self.bias = labels.iter().sum::<f64>() / labels.len() as f64; // Simple mean as bias
    }

    fn predict(&self, features: &[Vec<f64>]) -> Vec<f64> {
        features
            .iter()
            .map(|f| f.iter().zip(self.weights.iter()).map(|(x, w)| x * w).sum::<f64>() + self.bias)
            .collect()
    }
}
