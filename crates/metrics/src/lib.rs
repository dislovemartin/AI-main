pub mod exporter;
pub mod recorder;

/// Initializes the metrics system.
///
/// # Errors
///
/// Returns `MetricsError` if the initialization fails.
pub fn initialize_metrics() -> Result<(), MetricsError> {
    exporter::setup_exporter()?;
    println!("Metrics library initialized!");
    Ok(())
}

/// Records a metric with the given name and value.
///
/// # Arguments
///
/// * `name` - The name of the metric.
/// * `value` - The value of the metric.
///
/// # Errors
///
/// Returns `MetricsError` if recording the metric fails.
pub fn record_metric(name: &str, value: f64) -> Result<(), MetricsError> {
    recorder::record(name, value)?;
    Ok(())
}

/// Custom error type for metrics operations.
#[derive(Debug)]
pub enum MetricsError {
    InitializationError(String),
    RecordingError(String),
    // #TODO add more error variants as needed.
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricsError::InitializationError(msg) => write!(f, "Initialization Error: {}", msg),
            MetricsError::RecordingError(msg) => write!(f, "Recording Error: {}", msg),
        }
    }
}

impl std::error::Error for MetricsError {}
