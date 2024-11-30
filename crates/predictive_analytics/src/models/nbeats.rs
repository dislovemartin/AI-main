use std::sync::Arc;
use tch::{nn, Device, Tensor, Kind};
use anyhow::Result;
use tracing::{info, error};

use crate::models::{NBEATSModel, NBEATSStackType, TimeSeries, PredictionMetrics, TrainingMetrics};

pub struct NBEATSBlock {
    layers: Vec<Arc<nn::Linear>>,
    backcast_layer: Arc<nn::Linear>,
    forecast_layer: Arc<nn::Linear>,
}

impl NBEATSBlock {
    fn new(
        vs: &nn::Path,
        input_size: i64,
        layer_width: i64,
        expansion_coefficient_dim: i64,
    ) -> Self {
        let mut layers = Vec::new();
        let mut current_size = input_size;

        // Fully connected layers
        for _ in 0..4 {
            let layer = Arc::new(nn::linear(vs, current_size, layer_width, Default::default()));
            layers.push(layer);
            current_size = layer_width;
        }

        // Output layers for backcast and forecast
        let backcast_layer = Arc::new(nn::linear(
            vs,
            layer_width,
            expansion_coefficient_dim,
            Default::default(),
        ));
        let forecast_layer = Arc::new(nn::linear(
            vs,
            layer_width,
            expansion_coefficient_dim,
            Default::default(),
        ));

        Self {
            layers,
            backcast_layer,
            forecast_layer,
        }
    }

    fn forward(&self, x: &Tensor) -> (Tensor, Tensor) {
        let mut current = x.shallow_clone();

        // Forward through fully connected layers with ReLU activation
        for layer in &self.layers {
            current = layer.forward(&current).relu();
        }

        // Generate backcast and forecast
        let backcast = self.backcast_layer.forward(&current);
        let forecast = self.forecast_layer.forward(&current);

        (backcast, forecast)
    }
}

pub struct NBEATSStack {
    blocks: Vec<NBEATSBlock>,
    stack_type: NBEATSStackType,
}

impl NBEATSStack {
    fn new(
        vs: &nn::Path,
        num_blocks: usize,
        input_size: i64,
        layer_width: i64,
        expansion_coefficient_dim: i64,
        stack_type: NBEATSStackType,
    ) -> Self {
        let blocks = (0..num_blocks)
            .map(|_| {
                NBEATSBlock::new(vs, input_size, layer_width, expansion_coefficient_dim)
            })
            .collect();

        Self { blocks, stack_type }
    }

    fn forward(&self, x: &Tensor) -> (Tensor, Tensor) {
        let mut current = x.shallow_clone();
        let mut total_forecast = Tensor::zeros_like(x);

        for block in &self.blocks {
            let (backcast, forecast) = block.forward(&current);
            current = &current - &backcast;
            total_forecast += &forecast;
        }

        (current, total_forecast)
    }
}

pub struct NBEATSNet {
    stacks: Vec<NBEATSStack>,
    device: Device,
    var_store: nn::VarStore,
    config: NBEATSModel,
}

impl NBEATSNet {
    pub fn new(config: &NBEATSModel) -> Result<Self> {
        let device = Device::Cpu; // Use GPU if available
        let mut var_store = nn::VarStore::new(device);
        let vs = var_store.root();

        let stacks = config.stack_types
            .iter()
            .map(|&stack_type| {
                NBEATSStack::new(
                    &vs,
                    config.num_blocks,
                    config.backcast_length as i64,
                    config.layer_width as i64,
                    config.expansion_coefficient_dim as i64,
                    stack_type.clone(),
                )
            })
            .collect();

        Ok(Self {
            stacks,
            device,
            var_store,
            config: config.clone(),
        })
    }

    pub fn forward(&self, x: &Tensor) -> Tensor {
        let mut current = x.shallow_clone();
        let mut total_forecast = Tensor::zeros_like(x);

        for stack in &self.stacks {
            let (residuals, forecast) = stack.forward(&current);
            current = residuals;
            total_forecast += &forecast;
        }

        total_forecast
    }

    pub fn train_step(
        &self,
        optimizer: &mut nn::Optimizer,
        batch: (Tensor, Tensor),
    ) -> Result<f64> {
        let (x, y) = batch;
        optimizer.zero_grad();

        let output = self.forward(&x);
        let loss = output.mse_loss(&y, tch::Reduction::Mean);
        
        loss.backward();
        optimizer.step();

        Ok(f64::from(loss))
    }

    pub fn predict(&self, x: &Tensor) -> Tensor {
        self.forward(x)
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

pub struct NBEATSTrainer {
    model: NBEATSNet,
    optimizer: nn::Optimizer,
}

impl NBEATSTrainer {
    pub fn new(config: &NBEATSModel) -> Result<Self> {
        let model = NBEATSNet::new(config)?;
        let optimizer = nn::Adam::default().build(&model.var_store, 0.001)?;

        Ok(Self {
            model,
            optimizer,
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

    pub fn predict(&self, series: &TimeSeries) -> Result<Vec<f32>> {
        let x = Tensor::of_slice(&series.values)
            .view([-1, self.model.config.backcast_length as i64]);

        let predictions = self.model.predict(&x);
        let predictions: Vec<f32> = Vec::from(predictions.flatten(0, 1));

        Ok(predictions)
    }
} 