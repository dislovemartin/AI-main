/// Handles hyperparameter tuning for machine learning models.
use crate::errors::AutoMLError;
use crate::models::{OptimizationConfig, ParameterRange, TrialResult};
use async_trait::async_trait;
use rand::Rng;
use std::collections::HashMap;
use tracing::{info, warn};

#[async_trait]
pub trait HyperParameterTuner: Send + Sync {
    async fn optimize(&self, config: &OptimizationConfig) -> Result<TrialResult, AutoMLError>;
    async fn suggest_parameters(&self) -> Result<HashMap<String, f64>, AutoMLError>;
}

pub struct BasicTuner {
    config: OptimizationConfig,
}

impl BasicTuner {
    pub fn new(config: OptimizationConfig) -> Self {
        Self { config }
    }

    fn sample_parameter(&self, range: &ParameterRange) -> Result<f64, AutoMLError> {
        let mut rng = rand::thread_rng();
        match range {
            ParameterRange::Continuous { low, high, log } => {
                if *log {
                    let log_low = low.ln();
                    let log_high = high.ln();
                    Ok((rng.gen_range(log_low..log_high)).exp())
                } else {
                    Ok(rng.gen_range(*low..*high))
                }
            }
            ParameterRange::Discrete { low, high, step } => {
                let steps = (high - low) / step;
                Ok((rng.gen_range(0..=steps as i64) * step + low) as f64)
            }
            ParameterRange::Categorical { choices } => {
                let idx = rng.gen_range(0..choices.len());
                Ok(idx as f64)
            }
        }
    }
}

#[async_trait]
impl HyperParameterTuner for BasicTuner {
    async fn optimize(&self, config: &OptimizationConfig) -> Result<TrialResult, AutoMLError> {
        info!("Starting hyperparameter optimization");

        let mut best_trial = None;
        let mut best_value = f64::INFINITY;

        for trial_id in 0..config.max_trials {
            let parameters = self.suggest_parameters().await?;

            // Simulate model training with random value
            let mut rng = rand::thread_rng();
            let value = rng.gen_range(0.0..1.0);

            if value < best_value {
                best_value = value;
                best_trial = Some(TrialResult {
                    trial_id: trial_id.to_string(),
                    parameters: parameters
                        .into_iter()
                        .map(|(k, v)| (k, serde_json::Value::from(v)))
                        .collect(),
                    value,
                    state: crate::models::TrialState::Completed,
                    datetime_start: chrono::Utc::now(),
                    datetime_complete: Some(chrono::Utc::now()),
                });
                info!("New best trial found with value: {}", value);
            }
        }

        best_trial.ok_or_else(|| AutoMLError::OptimizationError("No successful trials".to_string()))
    }

    async fn suggest_parameters(&self) -> Result<HashMap<String, f64>, AutoMLError> {
        let mut parameters = HashMap::new();
        for (name, param_config) in &self.config.parameters {
            parameters.insert(name.clone(), self.sample_parameter(&param_config.parameter_type)?);
        }
        Ok(parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ParameterConfig;

    #[tokio::test]
    async fn test_basic_tuner() {
        let mut parameters = HashMap::new();
        parameters.insert("learning_rate".to_string(), ParameterConfig {
            parameter_type: ParameterRange::Continuous { low: 0.0001, high: 0.1, log: true },
            range: (0.0001, 0.1),
            step: None,
        });

        let config = OptimizationConfig {
            max_trials: 10,
            timeout_seconds: 3600,
            objective: "loss".to_string(),
            parameters,
        };

        let tuner = BasicTuner::new(config.clone());
        let result = tuner.optimize(&config).await;
        assert!(result.is_ok());

        let trial = result.unwrap();
        assert!(trial.value >= 0.0 && trial.value <= 1.0);
        assert!(trial.parameters.contains_key("learning_rate"));
    }

    #[tokio::test]
    async fn test_parameter_sampling() {
        let config = OptimizationConfig {
            max_trials: 1,
            timeout_seconds: 3600,
            objective: "loss".to_string(),
            parameters: HashMap::new(),
        };

        let tuner = BasicTuner::new(config);

        // Test continuous parameter sampling
        let continuous = ParameterRange::Continuous { low: 0.0, high: 1.0, log: false };
        let value = tuner.sample_parameter(&continuous).unwrap();
        assert!(value >= 0.0 && value <= 1.0);

        // Test discrete parameter sampling
        let discrete = ParameterRange::Discrete { low: 0, high: 10, step: 2 };
        let value = tuner.sample_parameter(&discrete).unwrap();
        assert!(value >= 0.0 && value <= 10.0);
        assert_eq!(value % 2.0, 0.0);

        // Test categorical parameter sampling
        let categorical = ParameterRange::Categorical {
            choices: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };
        let value = tuner.sample_parameter(&categorical).unwrap();
        assert!(value >= 0.0 && value < 3.0);
    }
}
