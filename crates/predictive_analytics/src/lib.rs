pub mod api;
pub mod data_integration;
pub mod forecasting;
pub mod models;
pub mod reporting;
pub mod visualization;

/// Initializes the predictive analytics library.
pub fn initialize_predictive_analytics() {
    println!("Predictive Analytics library initialized!");
}

use actix_web::{App, HttpServer, web};
use common::config::Config;

pub async fn run_server(config: Config) -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(api::config))
        .bind((config.server.host.as_str(), config.server.port))?
        .run()
        .await
}
