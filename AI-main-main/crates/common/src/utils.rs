use anyhow::{anyhow, Result};
use uuid::Uuid;

pub fn process_data(input: &str) -> Result<&str> {
    if input.is_empty() {
        return Err(anyhow!("Input data is empty"));
    }

    let processed = input.trim();
    Ok(processed)
}

pub fn generate_run_id() -> String {
    Uuid::new_v4().to_string()
}
