use actix_web::{get, HttpResponse, Responder};
use log::info;

/// Handler for the index route.
///
/// Returns a welcome message.
#[get("/")]
async fn index() -> impl Responder {
    info!("Index route accessed");
    HttpResponse::Ok().body("Welcome to the frontend!")
}
