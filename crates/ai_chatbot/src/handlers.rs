
//! Module: Handlers
//! Handles API requests for the AI Chatbot.
//! 
//! This module defines functions like `chat_endpoint` and `health_check`.
//! These functions process incoming requests and delegate tasks to other modules.

use actix_web::{HttpResponse, Responder};

/// Handles chat requests.
/// Processes incoming messages and generates a chatbot response.
pub async fn chat_endpoint() -> impl Responder {
    // TODO: Implement chat processing logic.
    HttpResponse::Ok().body("Chat endpoint is under construction.")
}

/// Health check endpoint.
/// Confirms service availability.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Service is healthy.")
}

/// Handles chat requests.
/// Processes incoming messages using utility and processing modules, and generates a chatbot response.
pub async fn chat_endpoint(msg: String) -> impl Responder {
    // Preprocess input
    let preprocessed_msg = crate::data_processing::preprocess_data(&msg);

    // Process chat message
    let response = crate::utils::process_chat_message(&preprocessed_msg);

    // Return response
    HttpResponse::Ok().json(response)
}

/// Health check endpoint.
/// Confirms service availability and provides basic diagnostics.
pub async fn health_check() -> impl Responder {
    // Collect basic diagnostics
    let diagnostics = serde_json::json!({
        "status": "healthy",
        "uptime": format!("{} seconds", chrono::Utc::now().timestamp()), // Simplified uptime example
        "version": "0.1.0"
    });

    // Return diagnostics as a JSON response
    HttpResponse::Ok().json(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_chat_endpoint() {
        let app = test::init_service(App::new().route("/chat", web::post().to(chat_endpoint))).await;
        let req = test::TestRequest::post().uri("/chat").set_payload(r#""Hello""#).to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: String = test::read_body(resp).await;
        assert!(body.contains("Echo: Hello"));
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().route("/health", web::get().to(health_check))).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "healthy");
        assert!(body["uptime"].as_str().unwrap_or("").contains("seconds"));
        assert_eq!(body["version"], "0.1.0");
    }
}
