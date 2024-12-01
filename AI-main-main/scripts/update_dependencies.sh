#!/bin/bash

# List of crates to update
declare -A CRATE_FEATURES=(
    ["ai_chatbot"]="serialization async_runtime error_handling web_framework logging database utils"
    ["supply_chain_ai"]="serialization async_runtime error_handling web_framework logging utils machine_learning"
    ["vr_ar_ai"]="serialization async_runtime error_handling web_framework logging utils machine_learning"
    ["python_service"]="serialization async_runtime error_handling web_framework logging utils"
    ["cpp_module"]="serialization async_runtime error_handling web_framework logging utils"
    ["rust_service"]="serialization async_runtime error_handling web_framework logging utils"
    ["content_creation_ai"]="serialization async_runtime error_handling web_framework logging utils machine_learning"
    ["cybersecurity_ai"]="serialization async_runtime error_handling web_framework logging database utils"
    ["frontend"]="serialization async_runtime error_handling web_framework logging utils"
    ["rust_anomaly_detector"]="serialization async_runtime error_handling web_framework logging utils machine_learning"
    ["healthcare_ai"]="serialization async_runtime error_handling web_framework logging database utils machine_learning"
    ["personalization_engine"]="serialization async_runtime error_handling web_framework logging database utils machine_learning"
    ["predictive_analytics"]="serialization async_runtime error_handling web_framework logging utils machine_learning"
)

# Update each crate's Cargo.toml
for crate in "${!CRATE_FEATURES[@]}"; do
    echo "Updating $crate..."
    features=${CRATE_FEATURES[$crate]}
    
    # Convert space-separated features into array format
    feature_array=""
    for feature in $features; do
        feature_array="${feature_array}    \"${feature}\",\n"
    done
    
    # Remove the trailing comma and newline
    feature_array=$(echo -e "$feature_array" | sed '$ s/,\\n$//')
    
    cat > "crates/$crate/Cargo.toml" << EOL
[package]
name = "$crate"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { path = "../common", features = [
${feature_array}
] }

# Add any crate-specific dependencies below
EOL
done

# Update sync_versions crate separately
cat > "crates/sync_versions/Cargo.toml" << EOL
[package]
name = "sync_versions"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.48"
toml = "0.8.8"
EOL

echo "Dependencies updated successfully!" 