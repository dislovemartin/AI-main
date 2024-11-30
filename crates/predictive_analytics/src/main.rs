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
use models::{ModelConfig, ModelType, DeepARConfig, LikelihoodType};
use repository::TimeSeriesRepository;
use services::TimeSeriesPredictor;

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

    info!("Starting Predictive Analytics service...");

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/timeseries".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let repository = TimeSeriesRepository::new(&database_url, &redis_url)
        .await
        .expect("Failed to initialize repository");

    // Initialize model configuration
    let model_config = ModelConfig {
        model_type: ModelType::DeepAR(DeepARConfig {
            hidden_size: 64,
            num_layers: 2,
            dropout: 0.1,
            likelihood: LikelihoodType::Gaussian,
        }),
        training_config: models::TrainingConfig {
            batch_size: 32,
            epochs: 100,
            learning_rate: 0.001,
            early_stopping_patience: 5,
            validation_split: 0.2,
        },
        feature_config: models::FeatureConfig {
            use_time_features: true,
            use_holiday_features: true,
            custom_features: vec![],
            scaling_method: models::ScalingMethod::StandardScaler,
        },
    };

    // Initialize predictor service
    let predictor = TimeSeriesPredictor::new(Arc::new(repository), model_config)
        .await
        .expect("Failed to initialize predictor");

    // Create application state
    let app_state = web::Data::new(AppState::new(Arc::new(predictor)));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .route("/predict", web::post().to(handlers::get_prediction))
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/train", web::post().to(handlers::train_model))
                    .route("/model-info", web::get().to(handlers::get_model_info)),
            )
    })
    .bind("0.0.0.0:8082")?
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
        let database_url = "postgres://postgres:postgres@localhost:5432/timeseries_test";
        let redis_url = "redis://localhost:6379";

        let repository = TimeSeriesRepository::new(database_url, redis_url)
            .await
            .expect("Failed to initialize repository");

        let model_config = ModelConfig {
            model_type: ModelType::DeepAR(DeepARConfig {
                hidden_size: 32,
                num_layers: 1,
                dropout: 0.1,
                likelihood: LikelihoodType::Gaussian,
            }),
            training_config: models::TrainingConfig {
                batch_size: 32,
                epochs: 10,
                learning_rate: 0.001,
                early_stopping_patience: 3,
                validation_split: 0.2,
            },
            feature_config: models::FeatureConfig {
                use_time_features: true,
                use_holiday_features: false,
                custom_features: vec![],
                scaling_method: models::ScalingMethod::StandardScaler,
            },
        };

        let predictor = TimeSeriesPredictor::new(Arc::new(repository), model_config)
            .await
            .expect("Failed to initialize predictor");

        let app_state = web::Data::new(AppState::new(Arc::new(predictor)));

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