use actix_web::{HttpResponse, Responder};
use prometheus::{Encoder, TextEncoder};
use lazy_static::lazy_static;
use prometheus::{Registry, Counter, Histogram, Opts, CounterVec, HistogramOpts};
use tracing::{info, error};

lazy_static! {
    static ref REQUEST_COUNTER: Counter = Counter::new("requests_total", "Total number of requests")
        .expect("Failed to create REQUEST_COUNTER");
    static ref RESPONSE_TIME_HISTOGRAM: Histogram = Histogram::with_opts(
        HistogramOpts::new("response_time_seconds", "Response times in seconds")
    ).expect("Failed to create RESPONSE_TIME_HISTOGRAM");
    static ref CUSTOM_REGISTRY: Registry = {
        let registry = Registry::new();
        registry.register(Box::new(REQUEST_COUNTER.clone())).unwrap();
        registry.register(Box::new(RESPONSE_TIME_HISTOGRAM.clone())).unwrap();
        registry
    };
}

/// Custom Prometheus Metrics Handler
pub async fn prometheus_metrics() -> impl Responder {
    info!("Fetching Prometheus metrics");
    let encoder = TextEncoder::new();
    let metric_families = CUSTOM_REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}
