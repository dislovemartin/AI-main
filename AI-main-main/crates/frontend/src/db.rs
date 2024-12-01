use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::error;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new().max_connections(5).connect(database_url).await.map_err(|e| {
        error!("Failed to create database pool: {}", e);
        anyhow::anyhow!("Database connection error: {}", e)
    })
}

pub async fn store_feedback(pool: &PgPool, user_id: &str, comments: &str) -> Result<i32> {
    sqlx::query!(
        r#"
        INSERT INTO feedback (user_id, comments)
        VALUES ($1, $2)
        RETURNING id
        "#,
        user_id,
        comments
    )
    .fetch_one(pool)
    .await
    .map(|record| record.id)
    .map_err(|e| {
        error!("Database error while storing feedback: {}", e);
        anyhow::anyhow!("Failed to store feedback: {}", e)
    })
}
