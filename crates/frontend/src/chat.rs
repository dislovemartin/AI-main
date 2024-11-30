use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use serde_json::json;

/// Represents a chat message submitted by a user.
#[derive(Deserialize)]
struct ChatMessage {
    user_id: String,
    message: String,
}

/// Handler for the chat page.
///
/// Returns a welcome message for the chat page.
#[get("/chat")]
async fn chat_page() -> impl Responder {
    info!("Chat page accessed");
    HttpResponse::Ok().body("Welcome to the chat page!")
}

/// Handler for receiving chat messages.
///
/// Stores or broadcasts the received chat message.
#[post("/chat")]
async fn chat(message: web::Json<ChatMessage>) -> impl Responder {
    // Log the chat message details
    info!(
        "Chat message from user {}: {}",
        message.user_id, message.message
    );

    // #TODO: Implement storage or broadcasting of the chat message

    HttpResponse::Ok().json(json!({"status": "Message received"}))
}
