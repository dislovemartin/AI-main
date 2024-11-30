use anyhow::Result;
use common::{Run, RunError, RunStatus};

pub struct PredictiveModel {
    run: Run,
    model_parameters: ModelParams,
}

impl PredictiveModel {
    pub fn new(user: String, params: ModelParams) -> Self {
        Self {
            run: Run::new(user),
            model_parameters: params,
        }
    }

    pub async fn train_model(&mut self, training_data: TrainingData) -> Result<()> {
        self.run.start_run(RunStatus::InProgress)?;

        // Training logic here

        self.run.complete_run()?;
        Ok(())
    }

    pub async fn predict(&self, prediction_input: PredictionInput) -> Result<PredictionOutput> {
        if self.run.is_expired() {
            return Err(RunError::Expired.into());
        }

        // Prediction logic here

        Ok(PredictionOutput { prediction: 42.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[tokio::test]
    async fn test_model_training_success() {
        let mut model = PredictiveModel::new(
            "test_user".to_string(),
            ModelParams {
                learning_rate: 0.01,
                epochs: 10,
            },
        );

        let training_data = TrainingData {
            features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            labels: vec![5.0, 6.0],
        };

        let result = model.train_model(training_data).await;
        assert!(result.is_ok());
        assert_eq!(model.run.status(), RunStatus::Completed);
    }

    #[tokio::test]
    async fn test_model_training_failure_to_start_run() {
        let mut model = PredictiveModel::new("test_user".to_string(), ModelParams::default());
        model.run = Run {
            status: Mutex::new(RunStatus::Failed),
            created_at: SystemTime::now(),
            expiration_time: Duration::from_secs(3600),
            user: "test_user".to_string(),
        };

        let training_data = TrainingData {
            features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            labels: vec![5.0, 6.0],
        };

        let result = model.train_model(training_data).await;
        assert!(result.is_err());
        assert_eq!(model.run.status(), RunStatus::Failed);
    }

    #[tokio::test]
    async fn test_prediction_after_expiration() {
        let mut run = Run {
            status: Mutex::new(RunStatus::InProgress),
            created_at: SystemTime::now() - Duration::from_secs(7200), // 2 hours ago
            expiration_time: Duration::from_secs(3600),                // 1 hour
            user: "test_user".to_string(),
        };
        let model = PredictiveModel {
            run,
            model_parameters: ModelParams::default(),
        };

        let prediction_input = PredictionInput {
            data: vec![7.0, 8.0],
        };

        let prediction = model.predict(prediction_input).await;
        assert!(prediction.is_err());
    }
}
