use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub features: UserFeatures,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeatures {
    pub demographics: Demographics,
    pub behavioral: BehavioralFeatures,
    pub preferences: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    pub age_group: String,
    pub location: String,
    pub language: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralFeatures {
    pub interaction_count: u32,
    pub avg_session_duration: f32,
    pub last_interactions: Vec<Interaction>,
    pub category_preferences: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub item_id: String,
    pub interaction_type: InteractionType,
    pub timestamp: DateTime<Utc>,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    View,
    Click,
    Like,
    Share,
    Purchase,
    Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub features: ItemFeatures,
    pub metadata: ItemMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFeatures {
    pub category: String,
    pub tags: Vec<String>,
    pub embedding: Vec<f32>,
    pub popularity_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMetadata {
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub user_id: String,
    pub context: RecommendationContext,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationContext {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub device_type: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub items: Vec<RecommendedItem>,
    pub request_id: String,
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedItem {
    pub item: Item,
    pub score: f32,
    pub explanation: String,
}

// Wide & Deep Model Structures
#[derive(Debug)]
pub struct WideAndDeepModel {
    pub wide_features: Vec<String>,
    pub deep_features: Vec<String>,
    pub embedding_dim: usize,
    pub hidden_layers: Vec<usize>,
    pub learning_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub precision: f32,
    pub recall: f32,
    pub ndcg: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub id: String,
    pub model_params: HashMap<String, String>,
    pub feature_config: FeatureConfig,
    pub training_config: TrainingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub wide_features: Vec<String>,
    pub deep_features: Vec<String>,
    pub embedding_dimensions: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub batch_size: usize,
    pub epochs: usize,
    pub learning_rate: f32,
    pub optimizer: String,
    pub loss_function: String,
}
