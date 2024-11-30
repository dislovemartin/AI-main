pub mod api;
pub mod data_processing;
pub mod metrics;
pub mod models;
pub mod utils;

use actix_web::{App, HttpServer, web};
use common::config::Config;

pub async fn run_server(config: Config) -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(api::config))
        .bind((config.server.host.as_str(), config.server.port))?
        .run()
        .await
}
