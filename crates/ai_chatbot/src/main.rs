use actix_middleware_cache::Cache;
use actix_rate_limit::{MemoryStore, RateLimiter};
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use ai_chatbot::api::config as api_config;
use ai_chatbot::errors::AppError;
use ai_chatbot::middleware::{CorrelationId, ErrorHandler, RequestResponseLogger};
use common::metrics::{record_request, record_response_time};
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber;

mod errors;
mod handlers;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing and logging
    tracing_subscriber::fmt::init();

    // Initialize Actix Web server
    HttpServer::new(|| {
        App::new()
            .wrap(CorrelationId)
            .wrap(ErrorHandler)
            .wrap(RequestResponseLogger)
            .wrap(PrometheusMetrics::new("api", Some("/metrics"), None))
            .configure(api_config)
            // Add Rate Limiter
            .wrap(
                RateLimiter::new(MemoryStore::new())
                    .with_period(Duration::from_secs(60))
                    .with_max(100),
            )
            // Add Cache Middleware with new implementation
            .wrap(Cache::new().max_age(Duration::from_secs(60)))
            .route("/health", web::get().to(handlers::health_check))
            .route("/chat", web::post().to(handlers::chat_endpoint))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
