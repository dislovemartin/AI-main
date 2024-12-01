use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tracing::{info, warn, error, Level};
use tracing_subscriber::prelude::*;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use metrics_exporter_prometheus::PrometheusBuilder;
use dotenv::dotenv;

mod errors;
mod handlers;
mod models;
mod optimization;
mod repository;
mod services;
mod config;
mod telemetry;

use config::Config;
use errors::AutoMLError;
use handlers::AppState;
use repository::AutoMLRepository;
use services::AutoMLOptimizer;
use telemetry::{init_telemetry, shutdown_telemetry};

#[actix_web::main]
async fn main() -> Result<(), AutoMLError> {
    // Load environment variables
    dotenv().ok();

    // Initialize configuration
    let config = Config::new().map_err(|e| AutoMLError::ConfigError(e.to_string()))?;

    // Initialize telemetry (tracing, metrics, etc.)
    init_telemetry(&config)?;

    info!("Starting AutoML service with config: {:?}", config);

    // Initialize metrics endpoint
    let metrics_builder = PrometheusBuilder::new();
    metrics_builder
        .install()
        .map_err(|e| AutoMLError::ConfigError(format!("Failed to install metrics: {}", e)))?;

    // Initialize database connection
    let repository = AutoMLRepository::new(&config.database_url, &config.redis_url)
        .await
        .map_err(|e| AutoMLError::DatabaseError(format!("Failed to initialize repository: {}", e)))?;

    // Initialize optimizer service
    let optimizer = AutoMLOptimizer::new(Arc::new(repository))
        .await
        .map_err(|e| AutoMLError::ModelInitializationError(format!("Failed to initialize optimizer: {}", e)))?;

    // Create application state
    let app_state = web::Data::new(AppState::new(Arc::new(optimizer)));

    // Start HTTP server
    info!("Starting HTTP server on {}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(telemetry::TracingMiddleware::default())
            .service(
                web::scope("/api/v1")
                    .route("/optimize", web::post().to(handlers::optimize_model))
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/metrics", web::get().to(handlers::metrics))
                    .route("/studies/{study_id}", web::get().to(handlers::get_study_info))
                    .route(
                        "/studies/{study_id}/best_model",
                        web::get().to(handlers::get_best_model),
                    ),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .workers(config.workers.unwrap_or_else(num_cpus::get))
    .shutdown_timeout(config.shutdown_timeout);

    // Handle shutdown gracefully
    let server_handle = server.run();
    
    tokio::spawn(async move {
        if let Err(e) = server_handle.await {
            error!("Server error: {}", e);
        }
    });

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    warn!("Received shutdown signal, starting graceful shutdown");

    // Cleanup
    shutdown_telemetry();
    info!("AutoML service shutdown complete");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use test_log::test;

    #[actix_rt::test]
    async fn test_server_startup() {
        let config = Config::new().expect("Failed to load test config");
        
        let repository = AutoMLRepository::new(&config.database_url, &config.redis_url)
            .await
            .expect("Failed to initialize repository");

        let optimizer = AutoMLOptimizer::new(Arc::new(repository))
            .await
            .expect("Failed to initialize optimizer");

        let app_state = web::Data::new(AppState::new(Arc::new(optimizer)));

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .wrap(middleware::Logger::default())
                .route("/api/v1/health", web::get().to(handlers::health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[test]
    fn test_config_loading() {
        let config = Config::new();
        assert!(config.is_ok(), "Should load test configuration successfully");
    }
} 