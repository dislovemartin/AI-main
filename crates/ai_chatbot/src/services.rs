use anyhow::Result;
use async_trait::async_trait;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, warn, debug};

use crate::errors::ChatbotError;
use crate::models::{ChatMessage, ChatResponse, ResponseMetadata, ChatConfig};

/// Configuration for the HuggingFace chatbot model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceConfig {
    pub model_name: String,
    pub max_length: usize,
    pub num_beams: usize,
    pub temperature: f32,
    pub top_k: usize,
    pub top_p: f32,
    pub do_sample: bool,
    pub repetition_penalty: f32,
}

impl Default for HuggingFaceConfig {
    fn default() -> Self {
        Self {
            model_name: "gpt2".to_string(),
            max_length: 100,
            num_beams: 5,
            temperature: 0.7,
            top_k: 50,
            top_p: 0.9,
            do_sample: true,
            repetition_penalty: 1.0,
        }
    }
}

/// Trait defining the interface for chatbot services
#[async_trait]
pub trait ChatbotService: Send + Sync {
    /// Generate a response for the given message
    async fn generate_response(&self, message: &ChatMessage) -> Result<ChatResponse, ChatbotError>;
    /// Get information about the model being used
    async fn get_model_info(&self) -> Result<String, ChatbotError>;
    /// Update the model configuration
    async fn update_config(&self, config: ChatConfig) -> Result<(), ChatbotError>;
}

/// Implementation of the HuggingFace-based chatbot
pub struct HuggingFaceChatbot {
    model: Arc<RwLock<TextGenerationModel>>,
    config: Arc<RwLock<HuggingFaceConfig>>,
}

impl HuggingFaceChatbot {
    /// Create a new instance of the HuggingFace chatbot
    pub async fn new(config: Option<HuggingFaceConfig>) -> Result<Self, ChatbotError> {
        let config = config.unwrap_or_default();
        info!("Initializing HuggingFace chatbot with model: {}", config.model_name);
        
        let generation_config = TextGenerationConfig {
            max_length: config.max_length,
            num_beams: config.num_beams,
            temperature: config.temperature,
            top_k: config.top_k,
            top_p: config.top_p,
            do_sample: config.do_sample,
            repetition_penalty: config.repetition_penalty,
            ..Default::default()
        };

        let model = TextGenerationModel::new(Default::default())
            .map_err(|e| {
                error!("Failed to initialize model: {:?}", e);
                ChatbotError::ModelInitializationError(e.to_string())
            })?;
        
        Ok(Self {
            model: Arc::new(RwLock::new(model)),
            config: Arc::new(RwLock::new(config)),
        })
    }

    /// Preprocess the input message before generation
    async fn preprocess_message(&self, message: &ChatMessage) -> String {
        debug!("Preprocessing message: {:?}", message);
        // Add any necessary preprocessing steps here
        message.content.clone()
    }

    /// Convert model output to a structured response
    async fn format_response(&self, output: String, processing_time: u64) -> ChatResponse {
        let config = self.config.read().await;
        ChatResponse {
            message: ChatMessage {
                role: "assistant".to_string(),
                content: output,
                metadata: None,
            },
            metadata: ResponseMetadata {
                timestamp: chrono::Utc::now(),
                model_version: config.model_name.clone(),
                processing_time_ms: processing_time,
                confidence: Some(0.95), // TODO: Implement proper confidence calculation
            },
        }
    }
}

#[async_trait]
impl ChatbotService for HuggingFaceChatbot {
    async fn generate_response(&self, message: &ChatMessage) -> Result<ChatResponse, ChatbotError> {
        let start_time = std::time::Instant::now();
        let input_text = self.preprocess_message(message).await;
        
        debug!("Generating response for input: {}", input_text);
        let model = self.model.read().await;
        let config = self.config.read().await;
        
        let generation_config = TextGenerationConfig {
            max_length: config.max_length,
            num_beams: config.num_beams,
            temperature: config.temperature,
            top_k: config.top_k,
            top_p: config.top_p,
            do_sample: config.do_sample,
            repetition_penalty: config.repetition_penalty,
            ..Default::default()
        };

        let output = model
            .generate(&[input_text], Some(&generation_config))
            .map_err(|e| {
                error!("Model generation error: {:?}", e);
                ChatbotError::ModelError(e.to_string())
            })?;

        if let Some(response) = output.first() {
            let processing_time = start_time.elapsed().as_millis() as u64;
            debug!("Response generated in {}ms", processing_time);
            Ok(self.format_response(response.clone(), processing_time).await)
        } else {
            warn!("Model generated empty response");
            Err(ChatbotError::EmptyResponse)
        }
    }

    async fn get_model_info(&self) -> Result<String, ChatbotError> {
        let config = self.config.read().await;
        Ok(format!("HuggingFace Model: {}", config.model_name))
    }

    async fn update_config(&self, chat_config: ChatConfig) -> Result<(), ChatbotError> {
        let mut config = self.config.write().await;
        config.temperature = chat_config.temperature;
        config.max_length = chat_config.max_tokens;
        info!("Updated model configuration: {:?}", config);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_chatbot_initialization() {
        let result = HuggingFaceChatbot::new(None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_response() {
        let chatbot = HuggingFaceChatbot::new(None).await.unwrap();
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
            metadata: None,
        };
        
        let response = chatbot.generate_response(&message).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_config_update() {
        let chatbot = HuggingFaceChatbot::new(None).await.unwrap();
        let new_config = ChatConfig {
            model: "gpt2".to_string(),
            max_tokens: 150,
            temperature: 0.8,
            stream: false,
        };
        
        let result = chatbot.update_config(new_config).await;
        assert!(result.is_ok());
    }
} 