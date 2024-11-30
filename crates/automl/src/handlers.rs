use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

use crate::errors::{AutoMLError, error_to_response};
use crate::models::{AutoMLConfig, ModelConfig};
use crate::services::AutoMLService;

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
pub struct OptimizeModelRequest {
    config: AutoMLConfig,
}

pub struct AppState {
    optimizer: Arc<dyn AutoMLService>,
}

impl AppState {
    pub fn new(optimizer: Arc<dyn AutoMLService>) -> Self {
        Self { optimizer }
    }
}

pub async fn optimize_model(
    data: web::Data<AppState>,
    request: web::Json<OptimizeModelRequest>,
) -> Result<HttpResponse, AutoMLError> {
    info!("Starting model optimization with config: {:?}", request.config);

    match data.optimizer.optimize_model(request.config.clone()).await {
        Ok(result) => {
            info!("Model optimization completed successfully");
            Ok(HttpResponse::Ok().json(result))
        }
        Err(e) => {
            error!("Error during model optimization: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

pub async fn get_study_info(
    data: web::Data<AppState>,
    study_id: web::Path<String>,
) -> Result<HttpResponse, AutoMLError> {
    info!("Retrieving study info for: {}", study_id);

    match data.optimizer.get_study_info(study_id.to_string()).await {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(e) => {
            error!("Error retrieving study info: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

pub async fn get_best_model(
    data: web::Data<AppState>,
    study_id: web::Path<String>,
) -> Result<HttpResponse, AutoMLError> {
    info!("Retrieving best model for study: {}", study_id);

    match data.optimizer.get_best_model(study_id.to_string()).await {
        Ok(model) => Ok(HttpResponse::Ok().json(model)),
        Err(e) => {
            error!("Error retrieving best model: {:?}", e);
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
        AutoMLService {}

        #[async_trait]
        impl AutoMLService for AutoMLService {
            async fn optimize_model(
                &self,
                config: AutoMLConfig,
            ) -> Result<crate::models::StudyResult, AutoMLError>;

            async fn get_study_info(
                &self,
                study_id: String,
            ) -> Result<crate::models::StudyResult, AutoMLError>;

            async fn get_best_model(
                &self,
                study_id: String,
            ) -> Result<ModelConfig, AutoMLError>;
        }
    }

    #[actix_rt::test]
    async fn test_optimize_model() {
        let mut mock_service = MockAutoMLService::new();
        mock_service
            .expect_optimize_model()
            .returning(|_| {
                Ok(crate::models::StudyResult {
                    study_id: "test".to_string(),
                    task_type: crate::models::TaskType::BinaryClassification,
                    best_trial: Default::default(),
                    best_model_path: "models/test.pt".to_string(),
                    trials: vec![],
                    optimization_history: vec![],
                    datetime_start: chrono::Utc::now(),
                    datetime_complete: Some(chrono::Utc::now()),
                    metadata: Default::default(),
                })
            });

        let app_state = web::Data::new(AppState {
            optimizer: Arc::new(mock_service),
        });

        let request = OptimizeModelRequest {
            config: AutoMLConfig {
                task_type: crate::models::TaskType::BinaryClassification,
                optimization_config: Default::default(),
                model_config: Default::default(),
                training_config: Default::default(),
                hardware_config: Default::default(),
            },
        };

        let resp = optimize_model(
            app_state,
            web::Json(request),
        ).await;

        assert!(resp.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_study_info() {
        let mut mock_service = MockAutoMLService::new();
        mock_service
            .expect_get_study_info()
            .returning(|_| {
                Ok(crate::models::StudyResult {
                    study_id: "test".to_string(),
                    task_type: crate::models::TaskType::BinaryClassification,
                    best_trial: Default::default(),
                    best_model_path: "models/test.pt".to_string(),
                    trials: vec![],
                    optimization_history: vec![],
                    datetime_start: chrono::Utc::now(),
                    datetime_complete: Some(chrono::Utc::now()),
                    metadata: Default::default(),
                })
            });

        let app_state = web::Data::new(AppState {
            optimizer: Arc::new(mock_service),
        });

        let resp = get_study_info(
            app_state,
            web::Path::from("test_study".to_string()),
        ).await;

        assert!(resp.is_ok());
    }
} 