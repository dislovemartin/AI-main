/// Computes the accuracy of predictions.
///
/// # Arguments
/// * `predicted` - A vector of predicted labels.
/// * `actual` - A vector of actual labels.
///
/// # Returns
/// The accuracy as a percentage (0.0 to 100.0).
pub fn accuracy(predicted: &[String], actual: &[String]) -> f64 {
    if predicted.len() != actual.len() || predicted.is_empty() {
        return 0.0;
    }
    let correct = predicted.iter().zip(actual.iter()).filter(|(p, a)| p == a).count();
    (correct as f64 / predicted.len() as f64) * 100.0
}

/// Computes precision and recall for a binary classification task.
///
/// # Arguments
/// * `predicted` - A vector of predicted binary labels.
/// * `actual` - A vector of actual binary labels.
///
/// # Returns
/// A tuple of (precision, recall).
pub fn precision_recall(predicted: &[bool], actual: &[bool]) -> (f64, f64) {
    if predicted.len() != actual.len() || predicted.is_empty() {
        return (0.0, 0.0);
    }
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

    (precision, recall)
}

/// Computes the mean squared error (MSE) for regression tasks.
///
/// # Arguments
/// * `predicted` - A vector of predicted values.
/// * `actual` - A vector of actual values.
///
/// # Returns
/// The mean squared error.
pub fn mean_squared_error(predicted: &[f64], actual: &[f64]) -> f64 {
    if predicted.len() != actual.len() || predicted.is_empty() {
        return 0.0;
    }
    let sum_squared_error: f64 =
        predicted.iter().zip(actual.iter()).map(|(p, a)| (p - a).powi(2)).sum();
    sum_squared_error / predicted.len() as f64
}
