/// Predicts demand using a simple linear regression model.
///
/// # Arguments
/// * `historical_data` - A slice of historical demand data.
///
/// # Returns
/// The predicted demand for the next period.
pub fn linear_regression_forecast(historical_data: &[f64]) -> Option<f64> {
    if historical_data.len() < 2 {
        return None; // Not enough data to perform regression
    }

    let n = historical_data.len() as f64;
    let mean_x = (0..historical_data.len()).map(|x| x as f64).sum::<f64>() / n;
    let mean_y = historical_data.iter().sum::<f64>() / n;

    let numerator: f64 = (0..historical_data.len())
        .map(|i| (i as f64 - mean_x) * (historical_data[i] - mean_y))
        .sum();

    let denominator: f64 = (0..historical_data.len()).map(|i| (i as f64 - mean_x).powi(2)).sum();

    if denominator == 0.0 {
        return None; // Avoid division by zero
    }

    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;

    Some(slope * n + intercept) // Predicted value for the next period
}
