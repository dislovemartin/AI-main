use std::fs::File;
use std::io::{self, Write};

/// Generates a text-based report for a dataset and saves it to a file.
///
/// # Arguments
/// * `data` - A slice of numerical data points.
/// * `file_path` - The path to save the report.
///
/// # Returns
/// An `io::Result` indicating success or failure.
pub fn generate_report(data: &[f64], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    writeln!(file, "Predictive Analytics Report")?;
    writeln!(file, "--------------------------")?;
    writeln!(file, "Data Points: {:?}", data)?;

    if let Some(mean) = data.iter().cloned().reduce(|a, b| a + b).map(|sum| sum / data.len() as f64)
    {
        writeln!(file, "Mean Value: {:.2}", mean)?;
    } else {
        writeln!(file, "Mean Value: N/A")?;
    }

    writeln!(file, "Number of Data Points: {}", data.len())?;

    Ok(())
}
