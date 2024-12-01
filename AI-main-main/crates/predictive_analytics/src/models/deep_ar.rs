use std::sync::Arc;
use tch::{nn, Device, Tensor, Kind};
use anyhow::Result;
use tracing::{info, error};

use crate::models::{
    DeepARModel, LikelihoodType, TimeSeries, PredictionMetrics,
    TrainingConfig, TrainingMetrics,
};

pub struct DeepARNet {
    lstm: Arc<nn::LSTM>,
    projection: Arc<nn::Linear>,
    device: Device,
    var_store: nn::VarStore,
    config: DeepARModel,
}

impl DeepARNet {
    pub fn new(config: &DeepARModel) -> Result<Self> {
        let device = Device::Cpu; // Use GPU if available
        let mut var_store = nn::VarStore::new(device);
        let root = var_store.root();

        // LSTM for sequence modeling
        let lstm = Arc::new(nn::lstm(
            &root,
            config.hidden_size as i64,
            config.hidden_size as i64,
            nn::LSTMConfig {
                num_layers: config.num_layers as i64,
                dropout: config.dropout,
                ..Default::default()
            },
        ));

        // Projection layer for parameters of the likelihood function
        let output_size = match config.likelihood {
            LikelihoodType::Gaussian => 2, // mean and std
            LikelihoodType::NegativeBinomial => 2, // mean and alpha
            LikelihoodType::StudentT => 3, // mean, std, and df
            LikelihoodType::Poisson => 1, // lambda
        };

        let projection = Arc::new(nn::linear(
            &root,
            config.hidden_size as i64,
            output_size,
            Default::default(),
        ));

        Ok(Self {
            lstm,
            projection,
            device,
            var_store,
            config: config.clone(),
        })
    }

    pub fn forward(&self, x: &Tensor, hidden: Option<(Tensor, Tensor)>) -> (Tensor, (Tensor, Tensor)) {
        let (output, new_hidden) = self.lstm.forward(x, hidden);
        let params = self.projection.forward(&output);
        (params, new_hidden)
    }

    pub fn likelihood_loss(&self, params: &Tensor, targets: &Tensor) -> Tensor {
        match self.config.likelihood {
            LikelihoodType::Gaussian => {
                let (mean, std) = params.chunk(2, -1);
                let std = std.exp() + 1e-6;
                -((-0.5 * ((targets - mean) / std).pow(2.0) - std.log() - 0.5 * (2.0 * std::f64::consts::PI).ln()).mean())
            }
            LikelihoodType::Poisson => {
                let lambda = params.exp();
                -(targets * lambda.log() - lambda - targets.lgamma()).mean()
            }
            _ => unimplemented!("Other likelihood types not implemented yet"),
        }
    }

    pub fn train_step(
        &self,
        optimizer: &mut nn::Optimizer,
        batch: (Tensor, Tensor),
    ) -> Result<f64> {
        let (x, y) = batch;
        optimizer.zero_grad();

        let (params, _) = self.forward(&x, None);
        let loss = self.likelihood_loss(&params, &y);
        
        loss.backward();
        optimizer.step();

        Ok(f64::from(loss))
    }

    pub fn predict(
        &self,
        context_window: &Tensor,
        num_samples: usize,
        prediction_length: usize,
    ) -> Result<Tensor> {
        let mut predictions = Vec::new();
        
        for _ in 0..num_samples {
            let mut current_input = context_window.copy();
            let mut hidden = None;
            let mut sample_path = Vec::new();

            for _ in 0..prediction_length {
                let (params, new_hidden) = self.forward(&current_input, hidden);
                hidden = Some(new_hidden);

                // Sample from the predicted distribution
                let sample = match self.config.likelihood {
                    LikelihoodType::Gaussian => {
                        let (mean, std) = params.chunk(2, -1);
                        let std = std.exp() + 1e-6;
                        mean + std * Tensor::randn_like(&mean)
                    }
                    LikelihoodType::Poisson => {
                        let lambda = params.exp();
                        Tensor::poisson_like(&lambda)
                    }
                    _ => unimplemented!("Other likelihood types not implemented yet"),
                };

                sample_path.push(sample.copy());
                current_input = sample.view([-1, 1, 1]);
            }

            let path = Tensor::stack(&sample_path, 0);
            predictions.push(path);
        }

        Ok(Tensor::stack(&predictions, 0))
    }

    pub fn save(&self, path: &str) -> Result<()> {
        self.var_store.save(path)?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        self.var_store.load(path)?;
        Ok(())
    }
}

pub struct DeepARTrainer {
    model: DeepARNet,
    optimizer: nn::Optimizer,
    training_config: TrainingConfig,
}

impl DeepARTrainer {
    pub fn new(
        model_config: &DeepARModel,
        training_config: TrainingConfig,
    ) -> Result<Self> {
        let model = DeepARNet::new(model_config)?;
        let optimizer = nn::Adam::default().build(&model.var_store, model_config.learning_rate)?;

        Ok(Self {
            model,
            optimizer,
            training_config,
        })
    }

    pub fn train_epoch(&mut self, dataset: &[(Tensor, Tensor)]) -> Result<TrainingMetrics> {
        let start_time = std::time::Instant::now();
        let mut total_loss = 0.0;
        let mut batch_count = 0;

        for batch in dataset {
            let loss = self.model.train_step(&mut self.optimizer, batch.clone())?;
            total_loss += loss;
            batch_count += 1;
        }

        let avg_loss = total_loss / batch_count as f64;
        let training_time = start_time.elapsed().as_secs_f32();

        info!("Epoch complete. Average loss: {}", avg_loss);

        Ok(TrainingMetrics {
            loss_history: vec![avg_loss as f32],
            validation_metrics: PredictionMetrics {
                mse: 0.0,
                rmse: 0.0,
                mae: 0.0,
                mape: 0.0,
                r2_score: 0.0,
            },
            training_time,
            timestamp: chrono::Utc::now(),
        })
    }

    pub fn predict(
        &self,
        series: &TimeSeries,
        prediction_length: usize,
        num_samples: usize,
    ) -> Result<Vec<Vec<f32>>> {
        let context_window = Tensor::of_slice(&series.values)
            .view([-1, 1, 1]);

        let predictions = self.model.predict(&context_window, num_samples, prediction_length)?;
        let predictions: Vec<Vec<f32>> = predictions
            .size2()?
            .iter()
            .map(|&(i, j)| {
                predictions.i((i, j)).into()
            })
            .collect();

        Ok(predictions)
    }
} 