use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

use crate::models::{
    TimeSeries, PredictionRequest, PredictionResponse, ModelConfig,
    ModelType, TrainingMetrics, ModelArtifact,
};
use crate::models::deep_ar::{DeepARTrainer, DeepARModel};
use crate::models::nbeats::{NBEATSTrainer, NBEATSModel};
use crate::repository::TimeSeriesRepository;
use crate::errors::PredictionError;

#[async_trait]
pub trait PredictionService: Send + Sync {
    async fn predict(
        &self,
        request: PredictionRequest,
    ) -> Result<PredictionResponse, PredictionError>;
    
    async fn train_model(
        &self,
        config: ModelConfig,
    ) -> Result<TrainingMetrics, PredictionError>;
    
    async fn get_model_info(&self) -> Result<ModelArtifact, PredictionError>;
}

pub struct TimeSeriesPredictor {
    deep_ar: Option<Arc<RwLock<DeepARTrainer>>>,
    nbeats: Option<Arc<RwLock<NBEATSTrainer>>>,
    repository: Arc<dyn TimeSeriesRepository>,
    current_model: ModelType,
}

impl TimeSeriesPredictor {
    pub async fn new(
        repository: Arc<dyn TimeSeriesRepository>,
        config: ModelConfig,
    ) -> Result<Self, PredictionError> {
        let (deep_ar, nbeats) = match &config.model_type {
            ModelType::DeepAR(deep_ar_config) => {
                let model_config = DeepARModel {
                    hidden_size: deep_ar_config.hidden_size,
                    num_layers: deep_ar_config.num_layers,
                    dropout: deep_ar_config.dropout,
                    learning_rate: config.training_config.learning_rate,
                    likelihood: deep_ar_config.likelihood.clone(),
                    context_length: 100,  // Configure as needed
                    prediction_length: 24, // Configure as needed
                };
                let trainer = DeepARTrainer::new(&model_config, config.training_config.clone())?;
                (Some(Arc::new(RwLock::new(trainer))), None)
            }
            ModelType::NBEATS(nbeats_config) => {
                let model_config = NBEATSModel {
                    stack_types: nbeats_config.stack_types.clone(),
                    num_blocks: nbeats_config.num_blocks,
                    num_layers: nbeats_config.num_layers,
                    layer_width: nbeats_config.layer_width,
                    expansion_coefficient_dim: 5,  // Configure as needed
                    backcast_length: 100,         // Configure as needed
                    forecast_length: 24,          // Configure as needed
                };
                let trainer = NBEATSTrainer::new(&model_config)?;
                (None, Some(Arc::new(RwLock::new(trainer))))
            }
            ModelType::Ensemble(_) => {
                unimplemented!("Ensemble models not implemented yet")
            }
        };

        Ok(Self {
            deep_ar,
            nbeats,
            repository,
            current_model: config.model_type,
        })
    }

    async fn get_series(&self, series_id: &str) -> Result<TimeSeries, PredictionError> {
        self.repository
            .get_series(series_id)
            .await
            .map_err(|e| PredictionError::DatabaseError(e.to_string()))
    }

    async fn prepare_prediction_response(
        &self,
        series: &TimeSeries,
        predictions: Vec<f32>,
        request: &PredictionRequest,
    ) -> Result<PredictionResponse, PredictionError> {
        let timestamps = self.repository
            .generate_future_timestamps(
                series.timestamps.last().unwrap(),
                request.horizon,
                &request.frequency,
            )
            .await?;

        Ok(PredictionResponse {
            series_id: series.id.clone(),
            predictions,
            confidence_intervals: None, // Implement if needed
            timestamps,
            metrics: Default::default(), // Calculate proper metrics
        })
    }
}

#[async_trait]
impl PredictionService for TimeSeriesPredictor {
    async fn predict(
        &self,
        request: PredictionRequest,
    ) -> Result<PredictionResponse, PredictionError> {
        info!("Generating predictions for series: {}", request.series_id);

        let series = self.get_series(&request.series_id).await?;

        let predictions = match (&self.current_model, &self.deep_ar, &self.nbeats) {
            (ModelType::DeepAR(_), Some(trainer), _) => {
                let trainer = trainer.read().await;
                trainer.predict(&series, request.horizon as usize, 100)?
                    .into_iter()
                    .map(|v| v[0])
                    .collect()
            }
            (ModelType::NBEATS(_), _, Some(trainer)) => {
                let trainer = trainer.read().await;
                trainer.predict(&series)?
            }
            _ => return Err(PredictionError::ModelNotInitialized),
        };

        self.prepare_prediction_response(&series, predictions, &request).await
    }

    async fn train_model(
        &self,
        config: ModelConfig,
    ) -> Result<TrainingMetrics, PredictionError> {
        info!("Starting model training with config: {:?}", config);

        let training_data = self.repository
            .get_training_data()
            .await
            .map_err(|e| PredictionError::DatabaseError(e.to_string()))?;

        match (&self.current_model, &self.deep_ar, &self.nbeats) {
            (ModelType::DeepAR(_), Some(trainer), _) => {
                let mut trainer = trainer.write().await;
                trainer.train_epoch(&training_data)
                    .map_err(|e| PredictionError::TrainingError(e.to_string()))
            }
            (ModelType::NBEATS(_), _, Some(trainer)) => {
                let mut trainer = trainer.write().await;
                trainer.train_epoch(&training_data)
                    .map_err(|e| PredictionError::TrainingError(e.to_string()))
            }
            _ => Err(PredictionError::ModelNotInitialized),
        }
    }

    async fn get_model_info(&self) -> Result<ModelArtifact, PredictionError> {
        Ok(ModelArtifact {
            model_id: Uuid::new_v4().to_string(),
            model_type: self.current_model.clone(),
            created_at: chrono::Utc::now(),
            metrics: TrainingMetrics {
                loss_history: vec![],
                validation_metrics: Default::default(),
                training_time: 0.0,
                timestamp: chrono::Utc::now(),
            },
            config: ModelConfig {
                model_type: self.current_model.clone(),
                training_config: Default::default(),
                feature_config: Default::default(),
            },
        })
    }
} 