use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use prometheus::{HistogramOpts, HistogramVec};

pub fn setup_metrics() -> PrometheusMetrics {
    let feedback_histogram = HistogramVec::new(
        HistogramOpts::new(
            "feedback_processing_duration",
            "Time spent processing feedback"
        ),
        &["status"]
    ).unwrap();

    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .histogram(feedback_histogram.clone())
        .build()
}
