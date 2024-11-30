use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramVec, IntCounterVec, TextEncoder, register_histogram_vec,
    register_int_counter_vec,
};

lazy_static! {
    pub static ref REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "ai_consulting_requests_total",
        "Total number of requests to the AI consulting service",
        &["service", "status"]
    )
    .expect("Failed to create REQUEST_COUNTER");
    pub static ref RESPONSE_TIME_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "ai_consulting_response_time_seconds",
        "Histogram for response times of the AI consulting service",
        &["service"]
    )
    .expect("Failed to create RESPONSE_TIME_HISTOGRAM");
}

/// Increments the request counter for a given service and status.
pub fn increment_request_counter(service: &str, status: &str) {
    REQUEST_COUNTER.with_label_values(&[service, status]).inc();
}

/// Observes the response time for a specific service.
pub fn observe_response_time(service: &str, duration: f64) {
    RESPONSE_TIME_HISTOGRAM.with_label_values(&[service]).observe(duration);
}

/// Exports all metrics as a Prometheus-compatible string.
pub fn export_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).expect("Failed to encode metrics");
    String::from_utf8(buffer).expect("Failed to convert metrics to String")
}
