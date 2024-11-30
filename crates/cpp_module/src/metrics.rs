use lazy_static::lazy_static;
use prometheus::{Encoder, IntCounterVec, TextEncoder, register_int_counter_vec};

lazy_static! {
    pub static ref CPP_METRIC_COUNTER: IntCounterVec = register_int_counter_vec!(
        "cpp_module_metrics_total",
        "Total metrics processed by the C++ module",
        &["metric_type"]
    )
    .expect("Failed to create CPP_METRIC_COUNTER");
}

pub fn increment_cpp_metric(metric_type: &str) {
    CPP_METRIC_COUNTER.with_label_values(&[metric_type]).inc();
}

pub fn gather_cpp_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
