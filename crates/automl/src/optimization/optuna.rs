use anyhow::Result;
use chrono::{DateTime, Utc};
use optuna::{Study, StudyDirection, Trial};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::errors::AutoMLError;
use crate::models::{
    AutoMLConfig, ModelMetrics, OptimizationConfig, ParameterRange, SearchSpace, StudyResult,
    TaskType, TrialResult,
};

pub struct OptunaOptimizer {
    study: Arc<RwLock<Study>>,
    config: AutoMLConfig,
}

impl OptunaOptimizer {
    pub async fn new(config: AutoMLConfig) -> Result<Self, AutoMLError> {
        let study_direction = match config.optimization_config.optimization_direction {
            crate::models::OptimizationDirection::Minimize => StudyDirection::Minimize,
            crate::models::OptimizationDirection::Maximize => StudyDirection::Maximize,
        };

        let study = Study::create(study_direction)
            .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?;

        Ok(Self { study: Arc::new(RwLock::new(study)), config })
    }

    pub async fn optimize<F>(&self, objective: F) -> Result<StudyResult, AutoMLError>
    where
        F: Fn(&Trial) -> Result<f64, AutoMLError> + Send + Sync,
    {
        let study = self.study.read().await;
        let n_trials = self.config.optimization_config.n_trials;

        // Configure sampler
        self.configure_sampler(&study).await?;

        // Configure pruner
        self.configure_pruner(&study).await?;

        // Run optimization
        let start_time = chrono::Utc::now();
        let mut trials = Vec::new();

        for trial_number in 0..n_trials {
            info!("Starting trial {}/{}", trial_number + 1, n_trials);

            let trial_start = chrono::Utc::now();
            let trial = study.ask().map_err(|e| AutoMLError::OptimizationError(e.to_string()))?;

            let result = match objective(&trial) {
                Ok(value) => {
                    study
                        .tell(trial.id, value)
                        .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?;

                    TrialResult {
                        trial_id: trial.id.to_string(),
                        parameters: self.get_trial_params(&trial)?,
                        value,
                        state: crate::models::TrialState::Completed,
                        datetime_start: trial_start,
                        datetime_complete: Some(chrono::Utc::now()),
                    }
                }
                Err(e) => {
                    study
                        .tell_failed(trial.id)
                        .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?;

                    TrialResult {
                        trial_id: trial.id.to_string(),
                        parameters: self.get_trial_params(&trial)?,
                        value: f64::NAN,
                        state: crate::models::TrialState::Failed(e.to_string()),
                        datetime_start: trial_start,
                        datetime_complete: Some(chrono::Utc::now()),
                    }
                }
            };

            trials.push(result);
        }

        // Get best trial
        let best_trial =
            study.best_trial().map_err(|e| AutoMLError::OptimizationError(e.to_string()))?;

        let best_trial_result = TrialResult {
            trial_id: best_trial.id.to_string(),
            parameters: self.get_trial_params(&best_trial)?,
            value: best_trial.value,
            state: crate::models::TrialState::Completed,
            datetime_start: start_time,
            datetime_complete: Some(chrono::Utc::now()),
        };

        Ok(StudyResult {
            study_id: study.id().to_string(),
            task_type: self.config.task_type.clone(),
            best_trial: best_trial_result,
            best_model_path: format!("models/best_model_{}.pt", study.id()),
            trials,
            optimization_history: study
                .trials()
                .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?
                .iter()
                .map(|t| t.value)
                .collect(),
            datetime_start: start_time,
            datetime_complete: Some(chrono::Utc::now()),
            metadata: Default::default(),
        })
    }

    async fn configure_sampler(&self, study: &Study) -> Result<(), AutoMLError> {
        use optuna::samplers::*;

        match &self.config.optimization_config.sampler_config.sampler_type {
            crate::models::SamplerType::TPE { n_ei_candidates } => {
                study.set_sampler(
                    TPESampler::new()
                        .with_n_ei_candidates(*n_ei_candidates)
                        .with_seed(self.config.optimization_config.sampler_config.seed),
                );
            }
            crate::models::SamplerType::RandomSearch => {
                study.set_sampler(RandomSampler::new(
                    self.config.optimization_config.sampler_config.seed,
                ));
            }
            crate::models::SamplerType::GridSearch => {
                // Implement grid search sampler
                unimplemented!("Grid search sampler not implemented yet");
            }
            crate::models::SamplerType::CmaEs { sigma } => {
                study.set_sampler(
                    CmaEsSampler::new()
                        .with_sigma(*sigma)
                        .with_seed(self.config.optimization_config.sampler_config.seed),
                );
            }
            _ => {
                // Default to TPE
                study.set_sampler(
                    TPESampler::new()
                        .with_seed(self.config.optimization_config.sampler_config.seed),
                );
            }
        }

        Ok(())
    }

    async fn configure_pruner(&self, study: &Study) -> Result<(), AutoMLError> {
        use optuna::pruners::*;

        match &self.config.optimization_config.pruner_config.pruner_type {
            crate::models::PrunerType::MedianPruner => {
                study.set_pruner(
                    MedianPruner::new()
                        .with_n_warmup_steps(
                            self.config.optimization_config.pruner_config.n_warmup_steps,
                        )
                        .with_n_min_trials(
                            self.config.optimization_config.pruner_config.n_min_trials,
                        ),
                );
            }
            crate::models::PrunerType::PercentilePruner { percentile } => {
                study.set_pruner(
                    PercentilePruner::new(*percentile)
                        .with_n_warmup_steps(
                            self.config.optimization_config.pruner_config.n_warmup_steps,
                        )
                        .with_n_min_trials(
                            self.config.optimization_config.pruner_config.n_min_trials,
                        ),
                );
            }
            crate::models::PrunerType::HyperbandPruner { min_resource, reduction_factor } => {
                study.set_pruner(
                    HyperbandPruner::new()
                        .with_min_resource(*min_resource)
                        .with_reduction_factor(*reduction_factor),
                );
            }
            crate::models::PrunerType::ThresholdPruner { lower, upper } => {
                study.set_pruner(ThresholdPruner::new(*lower, *upper));
            }
            crate::models::PrunerType::NopPruner => {
                study.set_pruner(NopPruner::new());
            }
        }

        Ok(())
    }

    fn get_trial_params(
        &self,
        trial: &Trial,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, AutoMLError> {
        let mut params = std::collections::HashMap::new();

        for (name, range) in &self.config.optimization_config.search_space.parameters {
            let value = match range {
                ParameterRange::Continuous { low, high, log } => if *log {
                    trial.suggest_log_float(name, *low, *high)
                } else {
                    trial.suggest_float(name, *low, *high)
                }
                .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?,
                ParameterRange::Discrete { low, high, step } => trial
                    .suggest_int(name, *low, *high, *step)
                    .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?,
                ParameterRange::Categorical { choices } => trial
                    .suggest_categorical(name, choices.as_slice())
                    .map_err(|e| AutoMLError::OptimizationError(e.to_string()))?,
            };

            params.insert(name.clone(), serde_json::to_value(value)?);
        }

        Ok(params)
    }
}
