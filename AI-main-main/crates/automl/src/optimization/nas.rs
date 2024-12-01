use anyhow::Result;
use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use rand::Rng;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use tch::{Device, Tensor, nn};
use tracing::{error, info};

use crate::errors::AutoMLError;
use crate::models::{ModelMetrics, NNConfig};

#[derive(Debug, Clone)]
pub enum LayerType {
    Linear { in_features: i64, out_features: i64 },
    Conv2d { in_channels: i64, out_channels: i64, kernel_size: i64 },
    MaxPool2d { kernel_size: i64 },
    Dropout { p: f64 },
    BatchNorm1d { num_features: i64 },
    BatchNorm2d { num_features: i64 },
    Activation { name: String },
}

pub struct NeuralArchitectureSearch {
    config: NNConfig,
    device: Device,
    var_store: nn::VarStore,
}

impl NeuralArchitectureSearch {
    pub fn new(config: NNConfig) -> Result<Self, AutoMLError> {
        let device = Device::Cpu; // Use GPU if available
        let var_store = nn::VarStore::new(device);

        Ok(Self { config, device, var_store })
    }

    pub fn generate_architecture(&self) -> Result<DiGraph<LayerType, ()>, AutoMLError> {
        let mut graph = DiGraph::new();
        let mut last_node = None;

        // Input layer
        let input_size = 784; // Example for MNIST
        let mut current_size = input_size;

        // Generate hidden layers
        for layer_idx in 0..self.config.max_layers {
            // Randomly choose number of units
            let n_units = rand::random::<usize>() % self.config.max_units + 64;

            // Add linear layer
            let linear_node = graph.add_node(LayerType::Linear {
                in_features: current_size,
                out_features: n_units as i64,
            });

            if let Some(prev) = last_node {
                graph.add_edge(prev, linear_node, ());
            }

            // Add batch normalization
            let bn_node = graph.add_node(LayerType::BatchNorm1d { num_features: n_units as i64 });
            graph.add_edge(linear_node, bn_node, ());

            // Add activation
            let activation_name = self.config.activation_functions
                [rand::random::<usize>() % self.config.activation_functions.len()]
            .clone();
            let activation_node = graph.add_node(LayerType::Activation { name: activation_name });
            graph.add_edge(bn_node, activation_node, ());

            // Add dropout with random probability
            let dropout_prob = rand::random::<f64>()
                * (self.config.dropout_range.1 - self.config.dropout_range.0)
                + self.config.dropout_range.0;
            let dropout_node = graph.add_node(LayerType::Dropout { p: dropout_prob });
            graph.add_edge(activation_node, dropout_node, ());

            last_node = Some(dropout_node);
            current_size = n_units as i64;
        }

        // Output layer
        let output_node = graph.add_node(LayerType::Linear {
            in_features: current_size,
            out_features: 10, // Example for MNIST
        });
        if let Some(prev) = last_node {
            graph.add_edge(prev, output_node, ());
        }

        Ok(graph)
    }

    pub fn build_model(
        &self,
        architecture: &DiGraph<LayerType, ()>,
    ) -> Result<Box<dyn nn::Module>, AutoMLError> {
        let mut layers = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        // Find input node (node with no incoming edges)
        let input_node = architecture
            .node_indices()
            .find(|&node| architecture.neighbors_directed(node, Direction::Incoming).count() == 0)
            .ok_or_else(|| AutoMLError::ArchitectureError("No input node found".to_string()))?;

        queue.push_back(input_node);

        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }

            let layer = match &architecture[node] {
                LayerType::Linear { in_features, out_features } => Box::new(nn::linear(
                    &self.var_store.root(),
                    *in_features,
                    *out_features,
                    Default::default(),
                ))
                    as Box<dyn nn::Module>,
                LayerType::Conv2d { in_channels, out_channels, kernel_size } => {
                    Box::new(nn::conv2d(
                        &self.var_store.root(),
                        *in_channels,
                        *out_channels,
                        *kernel_size,
                        Default::default(),
                    )) as Box<dyn nn::Module>
                }
                LayerType::MaxPool2d { kernel_size } => {
                    Box::new(nn::max_pool2d(*kernel_size, *kernel_size, 0, 1, true))
                        as Box<dyn nn::Module>
                }
                LayerType::Dropout { p } => Box::new(nn::dropout(*p, false)) as Box<dyn nn::Module>,
                LayerType::BatchNorm1d { num_features } => Box::new(nn::batch_norm1d(
                    &self.var_store.root(),
                    *num_features,
                    Default::default(),
                ))
                    as Box<dyn nn::Module>,
                LayerType::BatchNorm2d { num_features } => Box::new(nn::batch_norm2d(
                    &self.var_store.root(),
                    *num_features,
                    Default::default(),
                ))
                    as Box<dyn nn::Module>,
                LayerType::Activation { name } => match name.as_str() {
                    "relu" => Box::new(nn::func(|xs| xs.relu())) as Box<dyn nn::Module>,
                    "tanh" => Box::new(nn::func(|xs| xs.tanh())) as Box<dyn nn::Module>,
                    "sigmoid" => Box::new(nn::func(|xs| xs.sigmoid())) as Box<dyn nn::Module>,
                    _ => {
                        return Err(AutoMLError::ArchitectureError(format!(
                            "Unknown activation function: {}",
                            name
                        )));
                    }
                },
            };

            layers.push(layer);
            visited.insert(node);

            // Add successors to queue
            for neighbor in architecture.neighbors(node) {
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        Ok(Box::new(nn::seq::Sequential::new(layers)))
    }

    pub fn evaluate_architecture(
        &self,
        model: &Box<dyn nn::Module>,
        data: &[(Tensor, Tensor)],
    ) -> Result<ModelMetrics, AutoMLError> {
        let mut total_loss = 0.0;
        let mut correct = 0;
        let mut total = 0;

        for (x, y) in data {
            let output = model.forward(x);
            let loss = output.cross_entropy_loss(y, None, tch::Reduction::Mean, -100, 0.0);
            total_loss += f64::from(loss);

            let pred = output.argmax(-1, false);
            correct += i64::from(pred.eq(y).sum());
            total += x.size()[0];
        }

        let accuracy = correct as f64 / total as f64;
        let avg_loss = total_loss / data.len() as f64;

        Ok(ModelMetrics {
            accuracy: Some(accuracy),
            precision: None, // Calculate if needed
            recall: None,    // Calculate if needed
            f1_score: None,  // Calculate if needed
            auc_roc: None,   // Calculate if needed
            mse: Some(avg_loss),
            rmse: Some(avg_loss.sqrt()),
            mae: None,      // Calculate if needed
            r2_score: None, // Calculate if needed
            custom_metrics: Default::default(),
        })
    }
}
