use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

use crate::models::{
    AutoMLConfig, TaskType, ModelConfig, ModelType,
    StudyResult, TrialResult, ModelMetrics,
};
use crate::optimization::{
    optuna::OptunaOptimizer,
    nas::NeuralArchitectureSearch,
};
use crate::repository::AutoMLRepository;
use crate::errors::AutoMLError;

#[async_trait]
pub trait AutoMLService: Send + Sync {
    async fn optimize_model(
        &self,
        config: AutoMLConfig,
    ) -> Result<StudyResult, AutoMLError>;
    
    async fn get_study_info(
        &self,
        study_id: String,
    ) -> Result<StudyResult, AutoMLError>;
    
    async fn get_best_model(
        &self,
        study_id: String,
    ) -> Result<ModelConfig, AutoMLError>;
}

pub struct AutoMLOptimizer {
    repository: Arc<dyn AutoMLRepository>,
    current_study: Option<Arc<RwLock<OptunaOptimizer>>>,
    nas: Option<Arc<RwLock<NeuralArchitectureSearch>>>,
}

impl AutoMLOptimizer {
    pub async fn new(repository: Arc<dyn AutoMLRepository>) -> Result<Self, AutoMLError> {
        Ok(Self {
            repository,
            current_study: None,
            nas: None,
        })
    }

    async fn initialize_study(&mut self, config: &AutoMLConfig) -> Result<(), AutoMLError> {
        let optimizer = OptunaOptimizer::new(config.clone()).await?;
        self.current_study = Some(Arc::new(RwLock::new(optimizer)));

        if config.model_config.architecture_search {
            match &config.model_config.model_type {
                ModelType::NeuralNetwork(nn_config) => {
                    let nas = NeuralArchitectureSearch::new(nn_config.clone())?;
                    self.nas = Some(Arc::new(RwLock::new(nas)));
                }
                _ => {
                    return Err(AutoMLError::ConfigError(
                        "Architecture search is only supported for neural networks".to_string()
                    ));
                }
            }
        }

        Ok(())
    }

    async fn evaluate_model(
        &self,
        model_config: &ModelConfig,
        data: &[(tch::Tensor, tch::Tensor)],
    ) -> Result<ModelMetrics, AutoMLError> {
        match &model_config.model_type {
            ModelType::NeuralNetwork(nn_config) => {
                if let Some(nas) = &self.nas {
                    let nas = nas.read().await;
                    let architecture = nas.generate_architecture()?;
                    let model = nas.build_model(&architecture)?;
                    nas.evaluate_architecture(&model, data)
                } else {
                    Err(AutoMLError::ConfigError("NAS not initialized".to_string()))
                }
            }
            ModelType::LightGBM(config) => {
                // Implement LightGBM evaluation
                unimplemented!("LightGBM evaluation not implemented yet")
            }
            ModelType::XGBoost(config) => {
                // Implement XGBoost evaluation
                unimplemented!("XGBoost evaluation not implemented yet")
            }
            ModelType::RandomForest(config) => {
                // Implement RandomForest evaluation
                unimplemented!("RandomForest evaluation not implemented yet")
            }
            ModelType::Custom(name) => {
                Err(AutoMLError::ConfigError(format!("Custom model type {} not supported", name)))
            }
        }
    }

    async fn save_study_result(&self, result: &StudyResult) -> Result<(), AutoMLError> {
        self.repository
            .save_study_result(result)
            .await
            .map_err(|e| AutoMLError::DatabaseError(e.to_string()))
    }

    async fn load_study_result(&self, study_id: &str) -> Result<StudyResult, AutoMLError> {
        self.repository
            .get_study_result(study_id)
            .await
            .map_err(|e| AutoMLError::DatabaseError(e.to_string()))
    }
}

#[async_trait]
impl AutoMLService for AutoMLOptimizer {
    async fn optimize_model(
        &self,
        config: AutoMLConfig,
    ) -> Result<StudyResult, AutoMLError> {
        info!("Starting model optimization with config: {:?}", config);

        // Initialize study and NAS if needed
        let mut this = self.clone();
        this.initialize_study(&config).await?;

        let study = this.current_study.as_ref()
            .ok_or_else(|| AutoMLError::ConfigError("Study not initialized".to_string()))?;

        // Get training data
        let training_data = this.repository
            .get_training_data()
            .await
            .map_err(|e| AutoMLError::DatabaseError(e.to_string()))?;

        // Define objective function
        let objective = move |trial: &optuna::Trial| {
            // Sample hyperparameters
            let model_config = match &config.model_config.model_type {
                ModelType::NeuralNetwork(nn_config) => {
                    // Sample neural network hyperparameters
                    ModelConfig {
                        model_type: ModelType::NeuralNetwork(nn_config.clone()),
                        architecture_search: config.model_config.architecture_search,
                        feature_selection: config.model_config.feature_selection,
                        ensemble_config: config.model_config.ensemble_config.clone(),
                    }
                }
                // Add other model types here
                _ => return Err(AutoMLError::ConfigError("Unsupported model type".to_string())),
            };

            // Evaluate model
            let metrics = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(this.evaluate_model(&model_config, &training_data))?;

            // Return objective value based on task type
            match config.task_type {
                TaskType::BinaryClassification | TaskType::MultiClassification => {
                    Ok(1.0 - metrics.accuracy.unwrap_or(0.0))
                }
                TaskType::Regression => {
                    Ok(metrics.mse.unwrap_or(f64::INFINITY))
                }
                _ => Err(AutoMLError::ConfigError("Unsupported task type".to_string())),
            }
        };

        // Run optimization
        let result = study.read().await.optimize(objective).await?;

        // Save results
        this.save_study_result(&result).await?;

        Ok(result)
    }

    async fn get_study_info(
        &self,
        study_id: String,
    ) -> Result<StudyResult, AutoMLError> {
        info!("Retrieving study info for: {}", study_id);
        self.load_study_result(&study_id).await
    }

    async fn get_best_model(
        &self,
        study_id: String,
    ) -> Result<ModelConfig, AutoMLError> {
        info!("Retrieving best model for study: {}", study_id);
        
        let study_result = self.load_study_result(&study_id).await?;
        let model_config = self.repository
            .load_model_config(&study_result.best_model_path)
            .await
            .map_err(|e| AutoMLError::DatabaseError(e.to_string()))?;

        Ok(model_config)
    }
} 