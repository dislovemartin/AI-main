use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use anyhow::Result;
use tch::Tensor;
use std::sync::Arc;
use redis::Client as RedisClient;

use crate::models::{User, Item};

#[async_trait]
pub trait RecommendationRepository: Send + Sync {
    async fn get_user(&self, user_id: &str) -> Result<User>;
    async fn get_candidate_items(&self, user: &User, limit: usize) -> Result<Vec<Item>>;
    async fn get_training_data(&self) -> Result<Vec<(Tensor, Vec<Tensor>, Tensor)>>;
    async fn update_user_interactions(&self, user_id: &str, interactions: &[String]) -> Result<()>;
    async fn save_model_metrics(&self, metrics: &crate::models::ModelMetrics) -> Result<()>;
}

pub struct PostgresRepository {
    pool: Pool<Postgres>,
    redis: Arc<RedisClient>,
}

impl PostgresRepository {
    pub async fn new(database_url: &str, redis_url: &str) -> Result<Self> {
        let pool = Pool::connect(database_url).await?;
        let redis = Arc::new(RedisClient::open(redis_url)?);

        Ok(Self { pool, redis })
    }

    async fn get_user_features(&self, user_id: &str) -> Result<serde_json::Value> {
        // Try to get from cache first
        let redis_conn = self.redis.get_async_connection().await?;
        let cache_key = format!("user_features:{}", user_id);

        if let Ok(cached) = redis::cmd("GET")
            .arg(&cache_key)
            .query_async::<_, Option<String>>(&mut redis_conn)
            .await
        {
            if let Some(data) = cached {
                return Ok(serde_json::from_str(&data)?);
            }
        }

        // If not in cache, get from database
        let features = sqlx::query!(
            r#"
            SELECT features
            FROM user_features
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .features;

        // Cache the result
        let _ = redis::cmd("SETEX")
            .arg(&cache_key)
            .arg(3600) // Cache for 1 hour
            .arg(features.to_string())
            .query_async::<_, ()>(&mut redis_conn)
            .await;

        Ok(features)
    }

    async fn get_item_features(&self, item_ids: &[String]) -> Result<Vec<serde_json::Value>> {
        let features = sqlx::query!(
            r#"
            SELECT features
            FROM item_features
            WHERE item_id = ANY($1)
            "#,
            item_ids
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| row.features)
        .collect();

        Ok(features)
    }
}

#[async_trait]
impl RecommendationRepository for PostgresRepository {
    async fn get_user(&self, user_id: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_candidate_items(&self, user: &User, limit: usize) -> Result<Vec<Item>> {
        // Get items based on user preferences and popularity
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT i.*
            FROM items i
            LEFT JOIN user_item_interactions uii ON i.id = uii.item_id AND uii.user_id = $1
            WHERE uii.item_id IS NULL
            ORDER BY i.popularity_score DESC
            LIMIT $2
            "#,
            user.id,
            limit as i32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    async fn get_training_data(&self) -> Result<Vec<(Tensor, Vec<Tensor>, Tensor)>> {
        // Get training data from database
        let raw_data = sqlx::query!(
            r#"
            SELECT user_id, item_id, interaction_type, features
            FROM training_data
            LIMIT 1000000
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert to tensors
        // This is a placeholder - actual implementation would need to properly
        // convert the data into the correct tensor format
        unimplemented!("Training data conversion not implemented")
    }

    async fn update_user_interactions(&self, user_id: &str, interactions: &[String]) -> Result<()> {
        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Insert interactions
        for item_id in interactions {
            sqlx::query!(
                r#"
                INSERT INTO user_item_interactions (user_id, item_id, timestamp)
                VALUES ($1, $2, NOW())
                ON CONFLICT (user_id, item_id) 
                DO UPDATE SET timestamp = NOW()
                "#,
                user_id,
                item_id
            )
            .execute(&mut tx)
            .await?;
        }

        // Update user features
        sqlx::query!(
            r#"
            UPDATE user_features
            SET features = features || $2
            WHERE user_id = $1
            "#,
            user_id,
            serde_json::json!({
                "last_interaction_time": chrono::Utc::now(),
                "interaction_count": interactions.len(),
            })
        )
        .execute(&mut tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        // Invalidate cache
        let mut redis_conn = self.redis.get_async_connection().await?;
        let cache_key = format!("user_features:{}", user_id);
        redis::cmd("DEL")
            .arg(&cache_key)
            .query_async::<_, ()>(&mut redis_conn)
            .await?;

        Ok(())
    }

    async fn save_model_metrics(&self, metrics: &crate::models::ModelMetrics) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO model_metrics (
                precision_score,
                recall_score,
                ndcg_score,
                timestamp
            )
            VALUES ($1, $2, $3, $4)
            "#,
            metrics.precision,
            metrics.recall,
            metrics.ndcg,
            metrics.timestamp,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 