//! Module: Metrics
//! Provides Prometheus-based metrics for monitoring application performance.

use prometheus::{
    Encoder, HistogramVec, IntCounterVec, TextEncoder, register_histogram_vec,
    register_int_counter_vec,
};

/// MetricsCollector handles Prometheus metrics collection.
pub struct MetricsCollector {
    pub counter: IntCounterVec,
    pub histogram: HistogramVec,
}

impl MetricsCollector {
    /// Creates a new MetricsCollector instance.
    pub fn new(service_name: &str) -> Self {
        let counter = register_int_counter_vec!(
            format!("{}_requests_total", service_name),
            format!("Total requests for {}", service_name),
            &["endpoint", "status"]
        )
        .expect("Failed to create IntCounterVec");

        let histogram = register_histogram_vec!(
            format!("{}_response_duration_seconds", service_name),
            format!("Response duration for {}", service_name),
            &["endpoint"]
        )
        .expect("Failed to create HistogramVec");

        Self { counter, histogram }
    }

    /// Records a request with endpoint and status labels.
    pub fn record_request(&self, endpoint: &str, status: &str) {
        self.counter.with_label_values(&[endpoint, status]).inc();
    }

    /// Records response duration for a specific endpoint.
    pub fn record_duration(&self, endpoint: &str, duration: f64) {
        self.histogram.with_label_values(&[endpoint]).observe(duration);
    }

    /// Exports metrics in Prometheus text format.
    pub fn export_metrics() -> String {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).expect("Failed to encode metrics");
        String::from_utf8(buffer).expect("Failed to convert metrics to String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::Encoder;

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new("test_service");
        assert!(collector.counter.collect().len() > 0);
        assert!(collector.histogram.collect().len() > 0);
    }

    #[test]
    fn test_record_request() {
        let collector = MetricsCollector::new("test_service");
        collector.record_request("endpoint1", "200");
        let metric_families = prometheus::gather();
        let counter_metric = metric_families
            .iter()
            .find(|m| m.get_name() == "test_service_requests_total")
            .expect("Metric not found");
        assert_eq!(counter_metric.get_metric().len(), 1);
    }

    #[test]
    fn test_record_duration() {
        let collector = MetricsCollector::new("test_service");
        collector.record_duration("endpoint1", 1.23);
        let metric_families = prometheus::gather();
        let histogram_metric = metric_families
            .iter()
            .find(|m| m.get_name() == "test_service_response_duration_seconds")
            .expect("Metric not found");
        assert_eq!(histogram_metric.get_metric().len(), 1);
    }

    #[test]
    fn test_export_metrics() {
        let _collector = MetricsCollector::new("test_service");
        let exported = MetricsCollector::export_metrics();
        assert!(exported.contains("test_service_requests_total"));
    }
}
