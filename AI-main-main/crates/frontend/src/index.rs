use actix_web::{HttpResponse, Responder, get};
use log::info;

/// Handler for the index route.
///
/// Returns a welcome message.
#[get("/")]
async fn index() -> impl Responder {
    info!("Index route accessed");
    HttpResponse::Ok().body("Welcome to the frontend!")
}
