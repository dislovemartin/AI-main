/// Generates a simple ASCII bar chart for a given dataset.
///
/// # Arguments
/// * `data` - A slice of numerical data points.
///
/// # Returns
/// A string representation of the bar chart.
pub fn generate_bar_chart(data: &[f64]) -> String {
    let max_value = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let scale = if max_value > 0.0 {
        50.0 / max_value
    } else {
        1.0
    };

    let mut chart = String::new();
    for &value in data {
        let bar = (value * scale).round() as usize;
        chart.push_str(&format!(
            "{:>5}: {}
",
            value,
            "*".repeat(bar)
        ));
    }
    chart
}

/// Displays a trend line for a dataset.
///
/// # Arguments
/// * `data` - A slice of numerical data points.
///
/// # Returns
/// A string representation of the trend line.
pub fn display_trend_line(data: &[f64]) -> String {
    if data.is_empty() {
        return "No data to display".to_string();
    }

    let trend: Vec<String> = data
        .iter()
        .map(|&value| if value > 0.0 { "+" } else { "-" })
        .collect();

    trend.join(" ")
}
