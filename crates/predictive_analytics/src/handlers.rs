use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

use crate::errors::{PredictionError, error_to_response};
use crate::models::{PredictionRequest, ModelConfig};
use crate::services::PredictionService;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Debug, Deserialize)]
pub struct GetPredictionRequest {
    series_id: String,
    horizon: u32,
    frequency: String,
    include_history: Option<bool>,
    confidence_level: Option<f32>,
}

pub struct AppState {
    predictor: Arc<dyn PredictionService>,
}

impl AppState {
    pub fn new(predictor: Arc<dyn PredictionService>) -> Self {
        Self { predictor }
    }
}

pub async fn get_prediction(
    data: web::Data<AppState>,
    request: web::Json<GetPredictionRequest>,
) -> Result<HttpResponse, PredictionError> {
    info!("Generating predictions for series: {}", request.series_id);

    let prediction_request = PredictionRequest {
        series_id: request.series_id.clone(),
        horizon: request.horizon,
        frequency: request.frequency.parse()?,
        include_history: request.include_history.unwrap_or(false),
        confidence_level: request.confidence_level,
    };

    match data.predictor.predict(prediction_request).await {
        Ok(response) => {
            info!("Successfully generated predictions");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Error generating predictions: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TrainModelRequest {
    config: ModelConfig,
}

pub async fn train_model(
    data: web::Data<AppState>,
    request: web::Json<TrainModelRequest>,
) -> Result<HttpResponse, PredictionError> {
    info!("Starting model training with config: {:?}", request.config);

    match data.predictor.train_model(request.config.clone()).await {
        Ok(metrics) => {
            info!("Model training completed successfully");
            Ok(HttpResponse::Ok().json(metrics))
        }
        Err(e) => {
            error!("Error during model training: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

pub async fn get_model_info(
    data: web::Data<AppState>,
) -> Result<HttpResponse, PredictionError> {
    info!("Retrieving model information");

    match data.predictor.get_model_info().await {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(e) => {
            error!("Error retrieving model info: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        PredictionService {}

        #[async_trait]
        impl PredictionService for PredictionService {
            async fn predict(
                &self,
                request: PredictionRequest,
            ) -> Result<crate::models::PredictionResponse, PredictionError>;

            async fn train_model(
                &self,
                config: ModelConfig,
            ) -> Result<crate::models::TrainingMetrics, PredictionError>;

            async fn get_model_info(
                &self,
            ) -> Result<crate::models::ModelArtifact, PredictionError>;
        }
    }

    #[actix_rt::test]
    async fn test_get_prediction() {
        let mut mock_service = MockPredictionService::new();
        mock_service
            .expect_predict()
            .returning(|_| {
                Ok(crate::models::PredictionResponse {
                    series_id: "test".to_string(),
                    predictions: vec![1.0, 2.0, 3.0],
                    confidence_intervals: None,
                    timestamps: vec![],
                    metrics: Default::default(),
                })
            });

        let app_state = web::Data::new(AppState {
            predictor: Arc::new(mock_service),
        });

        let request = GetPredictionRequest {
            series_id: "test_series".to_string(),
            horizon: 24,
            frequency: "hourly".to_string(),
            include_history: None,
            confidence_level: None,
        };

        let resp = get_prediction(
            app_state,
            web::Json(request),
        ).await;

        assert!(resp.is_ok());
    }

    #[actix_rt::test]
    async fn test_train_model() {
        let mut mock_service = MockPredictionService::new();
        mock_service
            .expect_train_model()
            .returning(|_| {
                Ok(crate::models::TrainingMetrics {
                    loss_history: vec![],
                    validation_metrics: Default::default(),
                    training_time: 0.0,
                    timestamp: chrono::Utc::now(),
                })
            });

        let app_state = web::Data::new(AppState {
            predictor: Arc::new(mock_service),
        });

        let request = TrainModelRequest {
            config: ModelConfig {
                model_type: crate::models::ModelType::DeepAR(Default::default()),
                training_config: Default::default(),
                feature_config: Default::default(),
            },
        };

        let resp = train_model(
            app_state,
            web::Json(request),
        ).await;

        assert!(resp.is_ok());
    }
} 