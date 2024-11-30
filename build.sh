#!/bin/bash

set -e

echo "Building all crates in the workspace..."
cargo build --release --workspace

echo "Building Docker images..."
docker build -f dockerfiles/crates_content_creation_ai_Dockerfile -t your-registry/content_creation-ai:latest crates/content_creation_ai
# Repeat for other services

echo "Build and Docker process completed successfully." 