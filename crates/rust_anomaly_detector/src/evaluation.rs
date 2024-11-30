/// Calculates precision, recall, and F1-score for anomaly detection.
///
/// # Arguments
/// * `predicted` - A slice of predicted labels (true for anomalies, false otherwise).
/// * `actual` - A slice of actual labels (true for anomalies, false otherwise).
///
/// # Returns
/// A tuple containing (precision, recall, F1-score).
pub fn precision_recall_f1(predicted: &[bool], actual: &[bool]) -> (f64, f64, f64) {
    let mut true_positive = 0;
    let mut false_positive = 0;
    let mut false_negative = 0;

    for (&p, &a) in predicted.iter().zip(actual.iter()) {
        match (p, a) {
            (true, true) => true_positive += 1,
            (true, false) => false_positive += 1,
            (false, true) => false_negative += 1,
            _ => (),
        }
    }

    let precision = if true_positive + false_positive > 0 {
        true_positive as f64 / (true_positive + false_positive) as f64
    } else {
        0.0
    };

    let recall = if true_positive + false_negative > 0 {
        true_positive as f64 / (true_positive + false_negative) as f64
    } else {
        0.0
    };

    let f1_score = if precision + recall > 0.0 {
        2.0 * (precision * recall) / (precision + recall)
    } else {
        0.0
    };

    (precision, recall, f1_score)
}
