use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::RecommendationError;
use crate::models::{
    Item, ModelMetrics, RecommendationRequest, RecommendationResponse, RecommendedItem, User,
    WideAndDeepModel, WideAndDeepTrainer,
};
use crate::repository::RecommendationRepository;

#[async_trait]
pub trait RecommendationService: Send + Sync {
    async fn get_recommendations(
        &self,
        request: RecommendationRequest,
    ) -> Result<RecommendationResponse, RecommendationError>;

    async fn train_model(&self) -> Result<ModelMetrics, RecommendationError>;

    async fn update_user_preferences(
        &self,
        user_id: String,
        interactions: Vec<String>,
    ) -> Result<(), RecommendationError>;
}

pub struct WideAndDeepRecommender {
    model: Arc<RwLock<WideAndDeepTrainer>>,
    repository: Arc<dyn RecommendationRepository>,
    config: WideAndDeepModel,
}

impl WideAndDeepRecommender {
    pub async fn new(
        repository: Arc<dyn RecommendationRepository>,
        config: WideAndDeepModel,
    ) -> Result<Self, RecommendationError> {
        let trainer = WideAndDeepTrainer::new(&config)
            .map_err(|e| RecommendationError::ModelInitializationError(e.to_string()))?;

        Ok(Self { model: Arc::new(RwLock::new(trainer)), repository, config })
    }

    async fn get_user(&self, user_id: &str) -> Result<User, RecommendationError> {
        self.repository
            .get_user(user_id)
            .await
            .map_err(|e| RecommendationError::DatabaseError(e.to_string()))
    }

    async fn get_candidate_items(
        &self,
        user: &User,
        limit: usize,
    ) -> Result<Vec<Item>, RecommendationError> {
        self.repository
            .get_candidate_items(user, limit)
            .await
            .map_err(|e| RecommendationError::DatabaseError(e.to_string()))
    }

    async fn score_items(
        &self,
        user: &User,
        items: &[Item],
    ) -> Result<Vec<f32>, RecommendationError> {
        let model = self.model.read().await;
        model.predict(user, items).map_err(|e| RecommendationError::ModelError(e.to_string()))
    }

    async fn create_response(
        &self,
        items: Vec<Item>,
        scores: Vec<f32>,
    ) -> Result<RecommendationResponse, RecommendationError> {
        let mut recommended_items: Vec<RecommendedItem> = items
            .into_iter()
            .zip(scores)
            .map(|(item, score)| RecommendedItem {
                item,
                score,
                explanation: format!("Score: {:.3}", score),
            })
            .collect();

        // Sort by score in descending order
        recommended_items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        Ok(RecommendationResponse {
            items: recommended_items,
            request_id: Uuid::new_v4().to_string(),
            model_version: "wide_and_deep_v1".to_string(),
        })
    }
}

#[async_trait]
impl RecommendationService for WideAndDeepRecommender {
    async fn get_recommendations(
        &self,
        request: RecommendationRequest,
    ) -> Result<RecommendationResponse, RecommendationError> {
        info!("Generating recommendations for user: {}", request.user_id);

        // Get user data
        let user = self.get_user(&request.user_id).await?;

        // Get candidate items
        let candidate_items = self.get_candidate_items(&user, request.limit).await?;

        // Score items
        let scores = self.score_items(&user, &candidate_items).await?;

        // Create response
        self.create_response(candidate_items, scores).await
    }

    async fn train_model(&self) -> Result<ModelMetrics, RecommendationError> {
        info!("Starting model training");

        // Get training data
        let training_data = self
            .repository
            .get_training_data()
            .await
            .map_err(|e| RecommendationError::DatabaseError(e.to_string()))?;

        // Train model
        let mut model = self.model.write().await;
        let metrics = model
            .train_epoch(&training_data)
            .map_err(|e| RecommendationError::ModelError(e.to_string()))?;

        info!("Model training completed. Metrics: {:?}", metrics);

        Ok(metrics)
    }

    async fn update_user_preferences(
        &self,
        user_id: String,
        interactions: Vec<String>,
    ) -> Result<(), RecommendationError> {
        info!("Updating preferences for user: {}", user_id);

        self.repository
            .update_user_interactions(&user_id, &interactions)
            .await
            .map_err(|e| RecommendationError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
