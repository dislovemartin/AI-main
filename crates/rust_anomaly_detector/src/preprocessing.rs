/// Normalizes a dataset to the range [0, 1].
///
/// # Arguments
/// * `data` - A slice of data points to normalize.
///
/// # Returns
/// A vector of normalized data points.
pub fn min_max_normalize(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter().map(|x| (x - min) / (max - min)).collect()
}

/// Standardizes a dataset to have zero mean and unit variance.
///
/// # Arguments
/// * `data` - A slice of data points to standardize.
///
/// # Returns
/// A vector of standardized data points.
pub fn standardize(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let std_dev = variance.sqrt();
    data.iter().map(|x| (x - mean) / std_dev).collect()
}
