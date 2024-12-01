use common::process_data;
use anyhow::Result;

/// Processes input data using the common `process_data` function.
pub fn process_data_wrapper(input: &str) -> Result<String> {
    process_data(input)
}
