use rust_anomaly_detector::{
    algorithms::{isolation_forest, z_score},
    evaluation::precision_recall_f1,
    preprocessing::{min_max_normalize, standardize},
};

#[test]
fn test_anomaly_detection_algorithms() {
    let data = vec![1.0, 2.0, 3.0, 100.0, 4.0, 5.0];
    let threshold = 2.0;

    let anomalies = z_score(&data, threshold);
    assert!(anomalies.contains(&true)); // Should detect the outlier (100.0)

    let scores = isolation_forest(&data);
    assert_eq!(scores.len(), data.len());
}

#[test]
fn test_preprocessing() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let normalized = min_max_normalize(&data);
    assert!(normalized.iter().all(|&x| x >= 0.0 && x <= 1.0));

    let standardized = standardize(&data);
    let mean: f64 = standardized.iter().sum::<f64>() / standardized.len() as f64;
    assert!((mean - 0.0).abs() < 1e-10); // Mean should be approximately 0
}

#[test]
fn test_evaluation_metrics() {
    let predicted = vec![true, false, true, true];
    let actual = vec![true, false, false, true];

    let (precision, recall, f1) = precision_recall_f1(&predicted, &actual);
    assert!(precision >= 0.0 && precision <= 1.0);
    assert!(recall >= 0.0 && recall <= 1.0);
    assert!(f1 >= 0.0 && f1 <= 1.0);
}
