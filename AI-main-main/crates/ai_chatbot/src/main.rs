use actix_web::{App, HttpServer, web};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod api;
mod config;
mod data_processing;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
mod validators;

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

    info!("Starting AI Chatbot service...");

    // Initialize application state
    let app_state =
        handlers::AppState::new().await.expect("Failed to initialize application state");
    let app_data = web::Data::new(app_state);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Cors::default())
            .service(
                web::scope("/api/v1")
                    .route("/chat", web::post().to(handlers::chat))
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/model-info", web::get().to(handlers::model_info)),
            )
    })
    .bind("0.0.0.0:8080")?
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
        let app_state = handlers::AppState::new().await.unwrap();
        let app_data = web::Data::new(app_state);

        let app = test::init_service(
            App::new()
                .app_data(app_data)
                .route("/api/v1/health", web::get().to(handlers::health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
