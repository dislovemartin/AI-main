use crate::errors::AutoMLError;
use crate::models::{AutoMLConfig, ModelConfig, ModelType, TaskType};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

#[async_trait]
pub trait ModelSearcher: Send + Sync {
    async fn search(&self, config: &AutoMLConfig) -> Result<Vec<ModelConfig>, AutoMLError>;
    async fn evaluate_model(&self, model: &ModelConfig) -> Result<f64, AutoMLError>;
}

pub struct AutoModelSearcher {
    task_type: TaskType,
    available_models: Vec<ModelType>,
    evaluation_metric: String,
}

impl AutoModelSearcher {
    pub fn new(task_type: TaskType, evaluation_metric: String) -> Self {
        let available_models = match task_type {
            TaskType::BinaryClassification | TaskType::MultiClassification => vec![
                ModelType::LightGBM(Default::default()),
                ModelType::XGBoost(Default::default()),
                ModelType::RandomForest(Default::default()),
                ModelType::NeuralNetwork(Default::default()),
            ],
            TaskType::Regression => vec![
                ModelType::LightGBM(Default::default()),
                ModelType::XGBoost(Default::default()),
                ModelType::RandomForest(Default::default()),
                ModelType::NeuralNetwork(Default::default()),
            ],
            _ => vec![ModelType::NeuralNetwork(Default::default())],
        };

        Self { task_type, available_models, evaluation_metric }
    }

    fn generate_model_configs(&self) -> Vec<ModelConfig> {
        self.available_models
            .iter()
            .map(|model_type| ModelConfig {
                model_type: model_type.clone(),
                architecture_search: true,
                feature_selection: true,
                ensemble_config: None,
            })
            .collect()
    }
}

#[async_trait]
impl ModelSearcher for AutoModelSearcher {
    async fn search(&self, config: &AutoMLConfig) -> Result<Vec<ModelConfig>, AutoMLError> {
        info!("Starting model search for task type: {:?}", self.task_type);

        let model_configs = self.generate_model_configs();
        let mut evaluated_models = Vec::new();

        for model_config in model_configs {
            match self.evaluate_model(&model_config).await {
                Ok(score) => {
                    info!(
                        "Model {:?} achieved score {} on metric {}",
                        model_config.model_type, score, self.evaluation_metric
                    );
                    evaluated_models.push((model_config, score));
                }
                Err(e) => {
                    warn!("Failed to evaluate model {:?}: {}", model_config.model_type, e);
                }
            }
        }

        // Sort models by score
        evaluated_models.sort_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap());

        Ok(evaluated_models.into_iter().map(|(config, _)| config).collect())
    }

    async fn evaluate_model(&self, model: &ModelConfig) -> Result<f64, AutoMLError> {
        // Placeholder for actual model evaluation
        // In a real implementation, this would:
        // 1. Train the model with cross-validation
        // 2. Compute the specified evaluation metric
        // 3. Return the average score

        match &model.model_type {
            ModelType::LightGBM(_) => Ok(0.85),
            ModelType::XGBoost(_) => Ok(0.86),
            ModelType::RandomForest(_) => Ok(0.83),
            ModelType::NeuralNetwork(_) => Ok(0.87),
            _ => Err(AutoMLError::ModelError("Unsupported model type".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_search() {
        let searcher =
            AutoModelSearcher::new(TaskType::BinaryClassification, "accuracy".to_string());

        let config = AutoMLConfig {
            task_type: TaskType::BinaryClassification,
            optimization_config: Default::default(),
            model_config: Default::default(),
            training_config: Default::default(),
            hardware_config: Default::default(),
        };

        let result = searcher.search(&config).await;
        assert!(result.is_ok());

        let models = result.unwrap();
        assert!(!models.is_empty());
    }
}
