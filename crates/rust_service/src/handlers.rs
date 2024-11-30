use actix_web::{web, HttpResponse};
use crate::database::connect_database;
use anyhow::Result;

pub async fn get_data() -> Result<HttpResponse, actix_web::Error> {
    let client = connect_database().await.map_err(|e| {
        error!("Error connecting to database: {}", e);
        HttpResponse::InternalServerError().json({"error": "Database connection error"})
    })?;

    // Proceed with database operations
    Ok(HttpResponse::Ok().json({"data": "Your data"}))
}
