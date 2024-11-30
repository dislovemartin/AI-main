use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a chat message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The role of the message sender (e.g., "user", "assistant")
    pub role: String,
    /// The content of the message
    pub content: String,
    /// Optional metadata about the message
    #[serde(default)]
    pub metadata: Option<MessageMetadata>,
}

/// Metadata associated with a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// When the message was created
    pub timestamp: DateTime<Utc>,
    /// Optional user ID if authenticated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Optional session ID for tracking conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Response from the chat service
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The response message
    pub message: ChatMessage,
    /// Metadata about the response
    pub metadata: ResponseMetadata,
}

/// Metadata about the chat response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// When the response was generated
    pub timestamp: DateTime<Utc>,
    /// Version of the model used
    pub model_version: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Optional confidence score (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// Configuration for the chat service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    /// The model to use (e.g., "gpt-4")
    pub model: String,
    /// Maximum tokens in the response
    pub max_tokens: usize,
    /// Temperature for response generation
    pub temperature: f32,
    /// Whether to use streaming responses
    pub stream: bool,
}
