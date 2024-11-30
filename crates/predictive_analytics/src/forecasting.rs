/// Predicts future values using a moving average model.
///
/// # Arguments
/// * `data` - A slice of historical data points.
/// * `window_size` - The size of the moving average window.
///
/// # Returns
/// A vector of predicted values.
pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if data.len() < window_size || window_size == 0 {
        return vec![]; // Insufficient data or invalid window size
    }

    let mut predictions = vec![];
    for i in 0..(data.len() - window_size + 1) {
        let window = &data[i..(i + window_size)];
        let average = window.iter().sum::<f64>() / window.len() as f64;
        predictions.push(average);
    }

    predictions
}

/// Forecasts future trends using linear regression.
///
/// # Arguments
/// * `data` - A slice of historical data points.
///
/// # Returns
/// The predicted value for the next period.
pub fn linear_trend_forecast(data: &[f64]) -> Option<f64> {
    if data.len() < 2 {
        return None; // Not enough data for trend analysis
    }

    let n = data.len() as f64;
    let mean_x = (0..data.len()).map(|x| x as f64).sum::<f64>() / n;
    let mean_y = data.iter().sum::<f64>() / n;

    let numerator: f64 = (0..data.len())
        .map(|i| (i as f64 - mean_x) * (data[i] - mean_y))
        .sum();

    let denominator: f64 = (0..data.len()).map(|i| (i as f64 - mean_x).powi(2)).sum();

    if denominator == 0.0 {
        return None; // Avoid division by zero
    }

    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;

    Some(slope * n + intercept) // Forecast for the next period
}
