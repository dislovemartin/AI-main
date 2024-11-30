use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a CSV file and returns the data as a vector of vectors of f64.
pub fn read_csv(filepath: &str) -> Result<Vec<Vec<f64>>, io::Error> {
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(record) = line {
                let values =
                    record.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect();
                data.push(values);
            }
        }
    }
    Ok(data)
}

/// Helper function to read lines from a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Logs a message to the console with a timestamp.
pub fn log_message(message: &str) {
    let timestamp = chrono::Local::now();
    println!("[{}] {}", timestamp.format("%Y-%m-%d %H:%M:%S"), message);
}
