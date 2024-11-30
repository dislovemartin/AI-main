use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::{RecommendationError, error_to_response};
use crate::models::{RecommendationRequest, RecommendationResponse};
use crate::services::RecommendationService;

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
pub struct GetRecommendationsRequest {
    user_id: String,
    limit: Option<usize>,
    context: Option<serde_json::Value>,
}

pub struct AppState {
    recommender: Arc<dyn RecommendationService>,
}

impl AppState {
    pub fn new(recommender: Arc<dyn RecommendationService>) -> Self {
        Self { recommender }
    }
}

pub async fn get_recommendations(
    data: web::Data<AppState>,
    request: web::Json<GetRecommendationsRequest>,
) -> Result<HttpResponse, RecommendationError> {
    info!("Generating recommendations for user: {}", request.user_id);

    let rec_request = RecommendationRequest {
        user_id: request.user_id.clone(),
        context: request.context.clone().unwrap_or_default(),
        limit: request.limit.unwrap_or(10),
    };

    match data.recommender.get_recommendations(rec_request).await {
        Ok(response) => {
            info!("Successfully generated recommendations");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Error generating recommendations: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    interactions: Vec<String>,
}

pub async fn update_preferences(
    data: web::Data<AppState>,
    user_id: web::Path<String>,
    request: web::Json<UpdatePreferencesRequest>,
) -> Result<HttpResponse, RecommendationError> {
    info!("Updating preferences for user: {}", user_id);

    data.recommender
        .update_user_preferences(user_id.to_string(), request.interactions.clone())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Preferences updated successfully"
    })))
}

pub async fn train_model(data: web::Data<AppState>) -> Result<HttpResponse, RecommendationError> {
    info!("Starting model training");

    match data.recommender.train_model().await {
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        RecommendationService {}

        #[async_trait]
        impl RecommendationService for RecommendationService {
            async fn get_recommendations(
                &self,
                request: RecommendationRequest,
            ) -> Result<RecommendationResponse, RecommendationError>;

            async fn train_model(&self) -> Result<crate::models::ModelMetrics, RecommendationError>;

            async fn update_user_preferences(
                &self,
                user_id: String,
                interactions: Vec<String>,
            ) -> Result<(), RecommendationError>;
        }
    }

    #[actix_rt::test]
    async fn test_get_recommendations() {
        let mut mock_service = MockRecommendationService::new();
        mock_service.expect_get_recommendations().returning(|_| {
            Ok(RecommendationResponse {
                items: vec![],
                request_id: Uuid::new_v4().to_string(),
                model_version: "test".to_string(),
            })
        });

        let app_state = web::Data::new(AppState { recommender: Arc::new(mock_service) });

        let request = GetRecommendationsRequest {
            user_id: "test_user".to_string(),
            limit: Some(5),
            context: None,
        };

        let resp = get_recommendations(app_state, web::Json(request)).await;

        assert!(resp.is_ok());
    }

    #[actix_rt::test]
    async fn test_update_preferences() {
        let mut mock_service = MockRecommendationService::new();
        mock_service.expect_update_user_preferences().returning(|_, _| Ok(()));

        let app_state = web::Data::new(AppState { recommender: Arc::new(mock_service) });

        let request = UpdatePreferencesRequest {
            interactions: vec!["item1".to_string(), "item2".to_string()],
        };

        let resp = update_preferences(
            app_state,
            web::Path::from("test_user".to_string()),
            web::Json(request),
        )
        .await;

        assert!(resp.is_ok());
    }
}
