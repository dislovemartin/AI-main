use lazy_static::lazy_static;
use prometheus::{register_int_counter_vec, Encoder, IntCounterVec, TextEncoder};

lazy_static! {
    pub static ref REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "rust_service_requests_total",
        "Total number of requests to the Rust service",
        &["service", "status"]
    )
    .expect("Failed to create REQUEST_COUNTER");
}

pub fn increment_request_counter(service: &str, status: &str) {
    REQUEST_COUNTER.with_label_values(&[service, status]).inc();
}

pub fn gather_rust_service_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
