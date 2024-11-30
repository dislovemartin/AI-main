#!/bin/bash

set -euo pipefail  # Enable strict error handling

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m'
RED='\033[0;31m'
YELLOW='\033[1;33m'

# Logging functions
log() { echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"; }
error() { echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}" >&2; }
warn() { echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"; }

# Service configuration
declare -A SERVICE_CONFIG=(
    # Format: [service_name]="port gpu_required memory_limit cpu_limit"
    ["ai_chatbot"]="8083 1 2Gi 1000m"
    ["ai_consulting"]="8080 1 2Gi 1000m"
    ["content_creation_ai"]="8084 0 1Gi 500m"
    ["healthcare_ai"]="8089 1 2Gi 1000m"
    ["supply_chain_ai"]="8080 1 2Gi 1000m"
    ["predictive_analytics"]="8082 1 2Gi 1000m"
    ["frontend"]="8081 0 512Mi 200m"
)

setup_environment() {
    log "Setting up environment..."
    
    # Create necessary directories
    mkdir -p logs configs temp
    
    # Set up environment variables
    export RUST_LOG=${RUST_LOG:-info}
    export DOCKER_BUILDKIT=1
    
    # Verify kubernetes connection
    if ! kubectl cluster-info &>/dev/null; then
        error "Cannot connect to Kubernetes cluster"
        exit 1
    fi
}

install_system_dependencies() {
    log "Installing system dependencies..."
    
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            gcc \
            clang \
            llvm \
            lld \
            pkg-config \
            libssl-dev \
            curl \
            git \
            make \
            cmake
    elif command -v yum &> /dev/null; then
        sudo yum -y update
        sudo yum -y install \
            gcc \
            gcc-c++ \
            clang \
            llvm \
            lld \
            openssl-devel \
            curl \
            git \
            make \
            cmake
    else
        error "Unsupported package manager. Please install dependencies manually."
        exit 1
    fi

    # Verify installations
    for cmd in gcc clang lld pkg-config make cmake; do
        if ! command -v $cmd &> /dev/null; then
            error "$cmd is not installed properly. Please install it manually."
            exit 1
        fi
    done
}

install_rust() {
    if ! command -v rustc &> /dev/null; then
        log "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        log "Rust is already installed. Updating to the latest stable version..."
        rustup update stable
        rustup default stable
    fi
}

install_docker() {
    if ! command -v docker &> /dev/null; then
        log "Installing Docker..."
        curl -fsSL https://get.docker.com -o get-docker.sh
        sudo sh get-docker.sh
        sudo usermod -aG docker "$USER"
        rm get-docker.sh
        
        # Start Docker service
        sudo systemctl enable docker
        sudo systemctl start docker
    else
        log "Docker is already installed. Ensuring Docker is running..."
        sudo systemctl enable docker
        sudo systemctl start docker
    fi
}

setup_rust_environment() {
    log "Setting up Rust environment..."
    
    # Add necessary components
    rustup component add clippy rustfmt
    
    # Install cargo tools
    cargo install --locked \
        cargo-audit \
        cargo-watch \
        cargo-edit \
        cargo-tarpaulin \
        cargo-expand \
        cargo-outdated || true
}

build_rust_services() {
    log "Building Rust services..."
    
    # First build core dependencies
    for core in common shared metrics; do
        log "Building core: $core"
        (cd "crates/$core" && cargo build --release) || {
            error "Failed to build $core"
            return 1
        }
    done
    
    # Then build services
    for service in "${!SERVICE_CONFIG[@]}"; do
        log "Building service: $service"
        (cd "crates/$service" && cargo build --release) || {
            error "Failed to build $service"
            return 1
        }
    done
}

build_docker_images() {
    log "Building Docker images..."
    
    # Build base image first
    docker build -f dockerfiles/CrateDockerfile \
        --target builder \
        -t rust-base:latest .
    
    # Build service images
    for service in "${!SERVICE_CONFIG[@]}"; do
        read -r port gpu mem cpu <<< "${SERVICE_CONFIG[$service]}"
        
        log "Building Docker image for $service (Port: $port, GPU: $gpu)"
        
        docker build -f "dockerfiles/crates_${service}_Dockerfile" \
            --build-arg SERVICE_NAME="$service" \
            --build-arg SERVICE_PORT="$port" \
            --cache-from rust-base:latest \
            -t "ai-platform/$service:latest" . || {
            error "Failed to build Docker image for $service"
            return 1
        }
    done
}

deploy_kubernetes_resources() {
    log "Deploying Kubernetes resources..."
    
    # Create namespace if it doesn't exist
    kubectl create namespace ai-services --dry-run=client -o yaml | kubectl apply -f -
    
    # Apply ConfigMap first
    kubectl apply -f manifests/configmap.yaml
    
    # Deploy core services first
    for core in common shared metrics; do
        if [[ -f "manifests/${core}-deployment.yaml" ]]; then
            kubectl apply -f "manifests/${core}-deployment.yaml"
            kubectl apply -f "manifests/${core}-service.yaml"
            wait_for_deployment "$core"
        fi
    done
    
    # Deploy application services
    for service in "${!SERVICE_CONFIG[@]}"; do
        read -r port gpu mem cpu <<< "${SERVICE_CONFIG[$service]}"
        
        # Generate deployment manifest
        generate_deployment_manifest "$service" "$port" "$gpu" "$mem" "$cpu" | \
            kubectl apply -f -
        
        # Generate service manifest
        generate_service_manifest "$service" "$port" | \
            kubectl apply -f -
        
        # Wait for deployment
        wait_for_deployment "$service"
    done
    
    # Apply HPA last
    kubectl apply -f manifests/hpa.yaml
}

generate_deployment_manifest() {
    local service=$1
    local port=$2
    local gpu=$3
    local mem=$4
    local cpu=$5
    
    cat <<EOF
apiVersion: apps/v1
kind: Deployment
metadata:
 name: $service
  namespace: ai-services
spec:
  replicas: 1
  selector:
    matchLabels:
      app: $service
  template:
    metadata:
      labels:
        app: $service
    spec:
      containers:
      - name: $service
        image: ai-platform/$service:latest
        ports:
        - containerPort: $port
        resources:
          limits:
            cpu: $cpu
            memory: $mem
            nvidia.com/gpu: $gpu
          requests:
            cpu: ${cpu%m}m
            memory: ${mem%i}i
        readinessProbe:
          httpGet:
            path: /health
            port: $port
          initialDelaySeconds: 5
          periodSeconds: 10
        livenessProbe:
          httpGet:
            path: /health
            port: $port
          initialDelaySeconds: 15
          periodSeconds: 20
EOF
}

generate_service_manifest() {
    local service=$1
    local port=$2
    
    cat <<EOF
apiVersion: v1
kind: Service
metadata:
  name: $service
  namespace: ai-services
spec:
  selector:
    app: $service
  ports:
  - protocol: TCP
    port: $port
    targetPort: $port
  type: NodePort
EOF
}

verify_deployment() {
    log "Verifying deployment..."
    
    for service in "${!SERVICE_CONFIG[@]}"; do
        local ready_replicas=$(kubectl get deployment "$service" \
            -n ai-services \
            -o jsonpath='{.status.readyReplicas}')
        
        if [[ "$ready_replicas" != "1" ]]; then
            error "Service $service is not ready (replicas: $ready_replicas)"
            return 1
        fi
        
        # Check service endpoints
        local nodeport=$(kubectl get svc "$service" \
            -n ai-services \
            -o jsonpath='{.spec.ports[0].nodePort}')
        
        log "Service $service is available at port $nodeport"
    done
}

setup_monitoring() {
    log "Setting up monitoring..."
    
    # Deploy Prometheus
    kubectl apply -f manifests/prometheus.yaml
    
    # Deploy Grafana
    kubectl apply -f manifests/grafana.yaml
    
    # Wait for monitoring stack
    wait_for_deployment "prometheus"
    wait_for_deployment "grafana"
}

wait_for_deployment() {
    local deployment=$1
    log "Waiting for deployment $deployment..."
    
    kubectl rollout status deployment "$deployment" \
        -n ai-services \
        --timeout=300s || {
        error "Deployment $deployment failed to roll out"
        return 1
    }
}

cleanup() {
    log "Cleaning up..."
    rm -rf temp/*
}

main() {
    log "Starting deployment process..."
    
    setup_environment || exit 1
    
    install_system_dependencies || exit 1
    
    install_rust || exit 1
    
    install_docker || exit 1
    
    setup_rust_environment || exit 1
    
    build_rust_services || exit 1
    
    build_docker_images || exit 1
    
    deploy_kubernetes_resources || exit 1
    
    setup_monitoring || exit 1
    
    verify_deployment || exit 1
    
    cleanup
    
    log "Deployment completed successfully!"
    
    # Print access information
    log "Access Information:"
    for service in "${!SERVICE_CONFIG[@]}"; do
        local nodeport=$(kubectl get svc "$service" \
            -n ai-services \
            -o jsonpath='{.spec.ports[0].nodePort}')
        echo "- $service: http://localhost:$nodeport"
    done
}

# Execute with error handling
main "$@"