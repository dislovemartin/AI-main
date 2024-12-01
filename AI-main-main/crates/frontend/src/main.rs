use crate::errors::AppError;
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use anyhow::Result;
use env_logger::Env;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tracing::{error, info};

mod chat;
mod db;
mod errors;
mod index;

#[derive(Serialize, Deserialize)]
struct Feedback {
    user_id: String,
    comments: String,
}

#[post("/feedback")]
async fn feedback(
    feedback: web::Json<Feedback>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {
    info!("Received feedback from user ID: {}", feedback.user_id);

    let id = db::store_feedback(&db_pool, &feedback.user_id, &feedback.comments).await?;

    info!("Stored feedback with id: {}", id);
    Ok(HttpResponse::Ok().json(json!({
        "status": "Feedback received",
        "id": id
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database pool
    let pool = db::create_pool(&database_url).await.expect("Failed to create database pool");

    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index::index)
            .service(chat::chat_page)
            .service(chat::chat)
            .service(feedback)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_feedback_handler() {
        let mut app = test::init_service(App::new().service(feedback)).await;

        let payload =
            Feedback { user_id: "user123".to_string(), comments: "Great service!".to_string() };

        let req = test::TestRequest::post().uri("/feedback").set_json(&payload).to_request();

        let resp: serde_json::Value = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp["status"], "Feedback received");
    }
}
