use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod errors;
mod handlers;
mod models;
mod repository;
mod services;

use handlers::AppState;
use models::WideAndDeepModel;
use repository::PostgresRepository;
use services::WideAndDeepRecommender;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();

    info!("Starting Personalization Engine service...");

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/recommender".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let repository = PostgresRepository::new(&database_url, &redis_url)
        .await
        .expect("Failed to initialize repository");

    // Initialize model configuration
    let model_config = WideAndDeepModel {
        wide_features: vec!["category".to_string(), "tags".to_string()],
        deep_features: vec!["user_embedding".to_string(), "item_embedding".to_string()],
        embedding_dim: 64,
        hidden_layers: vec![128, 64, 32],
        learning_rate: 0.001,
    };

    // Initialize recommender service
    let recommender = WideAndDeepRecommender::new(Arc::new(repository), model_config)
        .await
        .expect("Failed to initialize recommender");

    // Create application state
    let app_state = web::Data::new(AppState::new(Arc::new(recommender)));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .route("/recommendations", web::post().to(handlers::get_recommendations))
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/train", web::post().to(handlers::train_model))
                    .route(
                        "/users/{user_id}/preferences",
                        web::put().to(handlers::update_preferences),
                    ),
            )
    })
    .bind("0.0.0.0:8081")?
    .workers(num_cpus::get())
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_server_startup() {
        let database_url = "postgres://postgres:postgres@localhost:5432/recommender_test";
        let redis_url = "redis://localhost:6379";

        let repository = PostgresRepository::new(database_url, redis_url)
            .await
            .expect("Failed to initialize repository");

        let model_config = WideAndDeepModel {
            wide_features: vec!["category".to_string()],
            deep_features: vec!["embedding".to_string()],
            embedding_dim: 32,
            hidden_layers: vec![64, 32],
            learning_rate: 0.001,
        };

        let recommender = WideAndDeepRecommender::new(Arc::new(repository), model_config)
            .await
            .expect("Failed to initialize recommender");

        let app_state = web::Data::new(AppState::new(Arc::new(recommender)));

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .route("/api/v1/health", web::get().to(handlers::health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
} 