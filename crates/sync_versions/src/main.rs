use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use toml::Value;

const DEPENDENCIES: &[(&str, &str)] = &[
    ("serde", "1.0.215"),
    ("serde_json", "1.0.133"),
    ("tokio", "1.41.1"),
    ("anyhow", "1.0.48"),
    ("thiserror", "2.0.3"),
    ("actix-web", "4.4"),
    ("reqwest", "0.11"),
    ("tracing", "0.1.40"),
    ("tracing-subscriber", "0.3"),
    ("redis", "0.24"),
    ("prometheus", "0.13.4"),
    ("chrono", "0.4.38"),
    ("async-trait", "0.1.83"),
    ("futures", "0.3"),
    ("linfa", "0.5.1"),
    ("ndarray", "0.15"),
    ("ndarray-stats", "0.5"),
    ("linregress", "0.5"),
    ("statrs", "0.16"),
    ("smartcore", "0.3"),
    ("hyperopt", "0.0.17"),
    ("rand", "0.8"),
];

fn main() -> Result<()> {
    let workspace_members = vec![
        "crates/ai_chatbot",
        "crates/ai_consulting",
        "crates/automl",
        "crates/common",
        "crates/content_creation_ai",
        "crates/cpp_module",
        "crates/cybersecurity_ai",
        "crates/frontend",
        "crates/go_recommender_service",
        "crates/healthcare_ai",
        "crates/metrics",
        "crates/personalization_engine",
        "crates/predictive_analytics",
        "crates/python_service",
        "crates/rust_anomaly_detector",
        "crates/rust_service",
        "crates/shared",
        "crates/supply_chain_ai",
        "crates/vr_ar_ai",
    ];

    for member in workspace_members {
        let cargo_toml_path = format!("{}/Cargo.toml", member);
        if !Path::new(&cargo_toml_path).exists() {
            println!("Skipping non-existent path: {}", cargo_toml_path);
            continue;
        }

        let content = fs::read_to_string(&cargo_toml_path)
            .with_context(|| format!("Failed to read {}", cargo_toml_path))?;

        let mut toml_val: Value = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", cargo_toml_path))?;
        let mut updated = false;

        for (dep, ver) in DEPENDENCIES {
            // Update [dependencies]
            if let Some(deps) = toml_val.get_mut("dependencies") {
                if deps.get(*dep).is_some() {
                    deps[*dep] = Value::String(ver.to_string());
                    updated = true;
                }
            }

            // Update [dev-dependencies]
            if let Some(deps) = toml_val.get_mut("dev-dependencies") {
                if deps.get(*dep).is_some() {
                    deps[*dep] = Value::String(ver.to_string());
                    updated = true;
                }
            }

            // Update [build-dependencies]
            if let Some(deps) = toml_val.get_mut("build-dependencies") {
                if deps.get(*dep).is_some() {
                    deps[*dep] = Value::String(ver.to_string());
                    updated = true;
                }
            }
        }

        if updated {
            let new_content = toml::to_string_pretty(&toml_val)
                .with_context(|| format!("Failed to serialize {}", cargo_toml_path))?;
            fs::write(&cargo_toml_path, new_content)
                .with_context(|| format!("Failed to write {}", cargo_toml_path))?;
            println!("Updated dependencies in {}", cargo_toml_path);
        } else {
            println!("No updates needed for {}", cargo_toml_path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_dependency_update() -> Result<()> {
        let dir = tempdir()?;
        let cargo_toml_path = dir.path().join("Cargo.toml");

        let test_toml = r#"
[package]
name = "test_crate"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.0"
tokio = { version = "1.0.0", features = ["full"] }
"#;

        let mut file = File::create(&cargo_toml_path)?;
        file.write_all(test_toml.as_bytes())?;

        // TODO: Add test implementation
        Ok(())
    }
}
