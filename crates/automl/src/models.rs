use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMLConfig {
    pub task_type: TaskType,
    pub optimization_config: OptimizationConfig,
    pub model_config: ModelConfig,
    pub training_config: TrainingConfig,
    pub hardware_config: HardwareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    BinaryClassification,
    MultiClassification,
    Regression,
    TimeSeries,
    Clustering,
    Ranking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub n_trials: usize,
    pub timeout_seconds: Option<u64>,
    pub n_jobs: usize,
    pub optimization_direction: OptimizationDirection,
    pub search_space: SearchSpace,
    pub pruner_config: PrunerConfig,
    pub sampler_config: SamplerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationDirection {
    Minimize,
    Maximize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpace {
    pub parameters: HashMap<String, ParameterRange>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterRange {
    Continuous { low: f64, high: f64, log: bool },
    Discrete { low: i64, high: i64, step: i64 },
    Categorical { choices: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub expression: String,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    LessThan(f64),
    GreaterThan(f64),
    Equal(f64),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrunerConfig {
    pub pruner_type: PrunerType,
    pub n_warmup_steps: usize,
    pub n_min_trials: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrunerType {
    MedianPruner,
    PercentilePruner { percentile: f64 },
    HyperbandPruner { min_resource: u64, reduction_factor: u64 },
    ThresholdPruner { lower: f64, upper: f64 },
    NopPruner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerConfig {
    pub sampler_type: SamplerType,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SamplerType {
    TPE { n_ei_candidates: usize },
    RandomSearch,
    GridSearch,
    CmaEs { sigma: f64 },
    Sobol,
    QMC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_type: ModelType,
    pub architecture_search: bool,
    pub feature_selection: bool,
    pub ensemble_config: Option<EnsembleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    NeuralNetwork(NNConfig),
    LightGBM(LightGBMConfig),
    XGBoost(XGBoostConfig),
    RandomForest(RandomForestConfig),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NNConfig {
    pub max_layers: usize,
    pub max_units: usize,
    pub activation_functions: Vec<String>,
    pub dropout_range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightGBMConfig {
    pub max_depth: (i32, i32),
    pub num_leaves: (i32, i32),
    pub learning_rate: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XGBoostConfig {
    pub max_depth: (i32, i32),
    pub eta: (f64, f64),
    pub gamma: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestConfig {
    pub n_estimators: (i32, i32),
    pub max_depth: (i32, i32),
    pub min_samples_split: (i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    pub ensemble_type: EnsembleType,
    pub n_models: usize,
    pub voting_method: VotingMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnsembleType {
    Stacking,
    Bagging,
    Boosting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VotingMethod {
    Hard,
    Soft,
    Weighted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub batch_size_range: (usize, usize),
    pub epochs_range: (usize, usize),
    pub early_stopping_patience: usize,
    pub validation_split: f64,
    pub cross_validation_folds: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub n_gpus: usize,
    pub n_cpu_threads: usize,
    pub memory_limit_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialResult {
    pub trial_id: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub value: f64,
    pub state: TrialState,
    pub datetime_start: DateTime<Utc>,
    pub datetime_complete: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrialState {
    Running,
    Completed,
    Pruned,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyResult {
    pub study_id: String,
    pub task_type: TaskType,
    pub best_trial: TrialResult,
    pub best_model_path: String,
    pub trials: Vec<TrialResult>,
    pub optimization_history: Vec<f64>,
    pub datetime_start: DateTime<Utc>,
    pub datetime_complete: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub accuracy: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
    pub auc_roc: Option<f64>,
    pub mse: Option<f64>,
    pub rmse: Option<f64>,
    pub mae: Option<f64>,
    pub r2_score: Option<f64>,
    pub custom_metrics: HashMap<String, f64>,
}
