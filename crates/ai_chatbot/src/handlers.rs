//! Module: Handlers
//! Handles API requests for the AI Chatbot.
//! 
//! This module defines functions like `chat_endpoint` and `health_check`.
//! These functions process incoming requests and delegate tasks to other modules.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

use crate::errors::{ChatbotError, error_to_response};
use crate::models::Message;
use crate::services::{ChatbotService, HuggingFaceChatbot};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    message: String,
    user_id: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    response: String,
    confidence: f32,
    model: String,
}

pub struct AppState {
    chatbot: Arc<dyn ChatbotService>,
}

impl AppState {
    pub async fn new() -> Result<Self, ChatbotError> {
        let chatbot = HuggingFaceChatbot::new("gpt2").await?;
        Ok(Self {
            chatbot: Arc::new(chatbot),
        })
    }
}

pub async fn chat(
    data: web::Data<AppState>,
    request: web::Json<ChatRequest>,
) -> Result<HttpResponse, ChatbotError> {
    info!("Received chat request from user: {}", request.user_id);

    let message = Message {
        content: request.message.clone(),
        user_id: request.user_id.clone(),
        timestamp: chrono::Utc::now(),
    };

    match data.chatbot.generate_response(&message).await {
        Ok(response) => {
            info!("Generated response for user: {}", request.user_id);
            Ok(HttpResponse::Ok().json(ChatResponse {
                response: response.response,
                confidence: response.confidence,
                model: response.model_name,
            }))
        }
        Err(e) => {
            error!("Error generating response: {:?}", e);
            Ok(error_to_response(e))
        }
    }
}

pub async fn model_info(data: web::Data<AppState>) -> Result<HttpResponse, ChatbotError> {
    let info = data.chatbot.get_model_info().await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "model_info": info
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_health_check() {
        let resp = health_check().await;
        let resp = test::call_service(
            &test::init_service(
                actix_web::App::new().service(web::resource("/health").to(health_check))
            ).await,
            test::TestRequest::get().uri("/health").to_request(),
        ).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_chat_endpoint() {
        let app_state = AppState::new().await.unwrap();
        let app_data = web::Data::new(app_state);

        let request = ChatRequest {
            message: "Hello".to_string(),
            user_id: "test_user".to_string(),
        };

        let resp = chat(
            app_data,
            web::Json(request),
        ).await;

        assert!(resp.is_ok());
    }
}
