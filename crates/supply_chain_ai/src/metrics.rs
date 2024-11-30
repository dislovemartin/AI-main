use prometheus::{Encoder, TextEncoder, register_counter_vec, CounterVec};
use anyhow::Result;

/// Define and register a counter metric.
lazy_static::lazy_static! {
    pub static ref REQUEST_COUNTER: CounterVec = register_counter_vec!(
        "ai_fast_core_requests_total",
        "Total number of requests",
        &["service", "status"]
    ).unwrap();
}

/// Increments the request counter.
pub fn increment_counter(service: &str, status: &str) {
    REQUEST_COUNTER.with_label_values(&[service, status]).inc();
}

/// Gathers all metrics and encodes them into a string.
pub fn gather_metrics() -> Result<String> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    let encoded = String::from_utf8(buffer)?;
    Ok(encoded)
}

/// Initializes Prometheus metrics.
pub fn init_metrics() {
    // #TODO Initialize any additional metrics here
}
