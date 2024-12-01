use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct ChatMessage {
    #[validate(length(min = 1, message = "User ID cannot be empty"))]
    pub user: String,

    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}
