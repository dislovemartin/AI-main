use crate::errors::AppError;
use crate::utils::process_chat_message;
use anyhow::{anyhow, Result};
use chrono::Duration;
use shared::RunError;

#[derive(Default)]
pub struct Run;

impl Run {
    pub fn start_run(&self) -> Result<(), RunError> {
        // Implementation
        Ok(())
    }

    pub fn complete_run(&self) -> Result<(), RunError> {
        // Implementation
        Ok(())
    }

    pub fn require_action(&self) -> Result<(), RunError> {
        // Implementation
        Ok(())
    }

    pub fn submit_action(&self) -> Result<(), RunError> {
        // Implementation
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        // Implementation
        false
    }
}

pub struct ChatModel {
    run: Run,
}

impl ChatModel {
    pub fn new() -> Self {
        Self { run: Run::default() }
    }

    pub fn start_run(&mut self) -> Result<(), AppError> {
        self.run.start_run()?;
        Ok(())
    }

    pub fn complete_run(&mut self) -> Result<(), AppError> {
        self.run.complete_run()?;
        Ok(())
    }

    pub fn require_action(&mut self) -> Result<(), AppError> {
        self.run.require_action()?;
        Ok(())
    }

    pub fn submit_action(&mut self) -> Result<(), AppError> {
        self.run.submit_action()?;
        Ok(())
    }

    pub fn require_client_input(&mut self, _response: String) -> Result<()> {
        // Implementation
        Ok(())
    }

    pub fn chat_session(&mut self) -> Result<(), AppError> {
        if let Err(e) = self.run.start_run() {
            return Err(anyhow!("Failed to start chat session run: {}", e).into());
        }

        // Additional logic...

        if let Err(e) = self.run.complete_run() {
            return Err(anyhow!("Failed to complete chat session run: {}", e).into());
        }

        Ok(())
    }
}

impl From<RunError> for AppError {
    fn from(err: RunError) -> Self {
        match err {
            RunError::Failed(msg) => AppError::InternalError(msg),
            RunError::Expired => AppError::BadRequest("Run expired".to_string()),
        }
    }
}

// Define SessionData if it's used elsewhere
#[derive(Default)]
pub struct SessionData {
    // Session data fields
}
