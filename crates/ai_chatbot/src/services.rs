use anyhow::Result;
use async_trait::async_trait;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error};

use crate::errors::ChatbotError;
use crate::models::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatbotResponse {
    pub response: String,
    pub confidence: f32,
    pub model_name: String,
}

#[async_trait]
pub trait ChatbotService: Send + Sync {
    async fn generate_response(&self, message: &Message) -> Result<ChatbotResponse, ChatbotError>;
    async fn get_model_info(&self) -> Result<String, ChatbotError>;
}

pub struct HuggingFaceChatbot {
    model: Arc<RwLock<TextGenerationModel>>,
    config: TextGenerationConfig,
    model_name: String,
}

impl HuggingFaceChatbot {
    pub async fn new(model_name: &str) -> Result<Self, ChatbotError> {
        info!("Initializing HuggingFace chatbot with model: {}", model_name);
        
        let config = TextGenerationConfig {
            max_length: 100,
            num_beams: 5,
            temperature: 0.7,
            top_k: 50,
            top_p: 0.9,
            ..Default::default()
        };

        let model = TextGenerationModel::new(Default::default())?;
        
        Ok(Self {
            model: Arc::new(RwLock::new(model)),
            config,
            model_name: model_name.to_string(),
        })
    }

    async fn preprocess_message(&self, message: &Message) -> String {
        // Add any necessary preprocessing here
        message.content.clone()
    }
}

#[async_trait]
impl ChatbotService for HuggingFaceChatbot {
    async fn generate_response(&self, message: &Message) -> Result<ChatbotResponse, ChatbotError> {
        let input_text = self.preprocess_message(message).await;
        
        let model = self.model.read().await;
        let output = model
            .generate(&[input_text], Some(&self.config))
            .map_err(|e| {
                error!("Model generation error: {:?}", e);
                ChatbotError::ModelError(e.to_string())
            })?;

        if let Some(response) = output.first() {
            Ok(ChatbotResponse {
                response: response.clone(),
                confidence: 0.95, // This should be calculated based on model output
                model_name: self.model_name.clone(),
            })
        } else {
            Err(ChatbotError::EmptyResponse)
        }
    }

    async fn get_model_info(&self) -> Result<String, ChatbotError> {
        Ok(format!("HuggingFace Model: {}", self.model_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_chatbot_initialization() {
        let result = block_on(HuggingFaceChatbot::new("gpt2"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_response() {
        let chatbot = block_on(HuggingFaceChatbot::new("gpt2")).unwrap();
        let message = Message {
            content: "Hello, how are you?".to_string(),
            timestamp: chrono::Utc::now(),
            user_id: "test_user".to_string(),
        };
        
        let response = block_on(chatbot.generate_response(&message));
        assert!(response.is_ok());
    }
} 