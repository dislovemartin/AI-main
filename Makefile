.PHONY: all setup build test lint clean deploy self-improve self-fix

# Variables
CARGO := cargo
DOCKER_COMPOSE := docker-compose
SETUP_SCRIPT := ./scripts/setup.sh

# Default target
all: setup build

# Setup environment
setup:
	$(SETUP_SCRIPT) setup

# Build all crates
build:
	$(CARGO) build --workspace --all-features

# Testing
test:
	$(CARGO) test --workspace --all-features -- --nocapture

# Linting
lint:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --workspace --all-features -- -D warnings

# Security audit
audit:
	$(CARGO) audit

# Build Docker images
docker-build:
	$(SETUP_SCRIPT) build_docker_images

# Deploy to Kubernetes
deploy:
	$(SETUP_SCRIPT) deploy_kubernetes

# Cleanup
clean:
	$(CARGO) clean
	$(DOCKER_COMPOSE) down -v
	rm -rf target/ coverage/ docs/

# Self-improvement
self-improve:
	$(SETUP_SCRIPT) self_improve

# Self-fixing
self-fix:
	$(SETUP_SCRIPT) self_fix 