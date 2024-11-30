use anyhow::{anyhow, Result};
use common::{Run, RunStatus};

pub struct DataProcessor {
    run: Run,
    //#Todo add other fields as necessary
}

impl DataProcessor {
    pub fn new(user: String) -> Self {
        Self {
            run: Run::new(user),
            // Initialize other fields
        }
    }

    pub fn process_data(&mut self, data: Vec<u8>) -> Result<()> {
        if !self.run.start_run() {
            return Err(anyhow!("Failed to start data processing run"));
        }

        // Implement data processing logic here

        if self.run.complete_run(&self.run.get_user()) {
            Ok(())
        } else {
            Err(anyhow!("Failed to complete data processing run"))
        }
    }

    pub fn require_additional_processing(&mut self) -> Result<()> {
        if !self.run.require_action(&self.run.get_user()) {
            return Err(anyhow!("Failed to mark run as requiring additional processing"));
        }
        Ok(())
    }

    pub fn submit_additional_processing(&mut self, additional_data: Vec<u8>) -> Result<()> {
        if !self.run.submit_action(&self.run.get_user()) {
            return Err(anyhow!("Failed to submit additional processing data"));
        }

        // Implement additional processing logic here

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_data_processor_lifecycle() {
        let mut processor = DataProcessor::new("test_user".to_string());

        // Mock data processing
        assert!(processor.process_data(vec![1, 2, 3]).is_ok());

        // Mock requiring additional processing
        assert!(processor.require_additional_processing().is_ok());

        // Mock submitting additional processing
        assert!(processor.submit_additional_processing(vec![4, 5, 6]).is_ok());
    }

    #[test]
    fn test_unauthorized_run_completion() {
        let mut processor = DataProcessor::new("test_user".to_string());
        assert!(!processor.run.complete_run("unauthorized_user"));
    }

    #[test]
    fn test_run_expiration() {
        let run = Run {
            status: Mutex::new(RunStatus::InProgress),
            created_at: SystemTime::now() - Duration::from_secs(7200), // 2 hours ago
            expiration_time: Duration::from_secs(3600),                // 1 hour
            user: "test_user".to_string(),
        };
        assert!(run.check_expiration());
    }

    #[test]
    fn test_submit_additional_processing_success() -> Result<()> {
        let mut processor = DataProcessor::new("user1".to_string());
        let additional_data = vec![1, 2, 3];
        assert!(processor.submit_additional_processing(additional_data)?.is_ok());
        Ok(())
    }

    #[test]
    fn test_submit_additional_processing_failure() {
        let mut processor = DataProcessor::new("user1".to_string());
        // Simulate failure by mocking `run.submit_action` if possible
        // For simplicity, assuming it returns an error
        // This requires refactoring for better testability
        // Here we directly check the error handling
        let additional_data = vec![1, 2, 3];
        let result = processor.submit_additional_processing(additional_data);
        assert!(result.is_err());
    }
}
