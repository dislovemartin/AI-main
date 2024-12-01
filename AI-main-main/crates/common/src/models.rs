
/// A trait for machine learning models.
pub trait Model {
    /// Trains the model with the provided features and labels.
    fn train(&mut self, features: &[Vec<f64>], labels: &[f64]);

    /// Makes predictions using the provided features.
    fn predict(&self, features: &[Vec<f64>]) -> Vec<f64>;
}

/// A simple stub implementation of a model for testing purposes.
pub struct DummyModel {
    pub bias: f64,
}

impl DummyModel {
    /// Creates a new DummyModel with the given bias.
    pub fn new(bias: f64) -> Self {
        Self { bias }
    }
}

impl Model for DummyModel {
    fn train(&mut self, _features: &[Vec<f64>], _labels: &[f64]) {
        println!("DummyModel: Training is a no-op.");
    }

    fn predict(&self, features: &[Vec<f64>]) -> Vec<f64> {
        features.iter().map(|_| self.bias).collect()
    }
}
