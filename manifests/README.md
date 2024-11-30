# Kubernetes Deployment for AI Services

This directory contains the Kubernetes manifests required to deploy the AI services on a Kubernetes cluster, such as NVIDIA DGX Cloud.

## Deployment Instructions

1. **Prerequisites**:
   - Kubernetes cluster with NVIDIA GPU support.
   - `kubectl` CLI installed and configured.
   - NVIDIA GPU Operator installed on the cluster.

2. **Deployment Steps**:
   - Apply the manifests:
     ```bash
     kubectl apply -f .
     ```

3. **Verify Deployment**:
   - Check the status of pods and services:
     ```bash
     kubectl get pods -n ai-services
     kubectl get services -n ai-services
     ```

4. **Access Services**:
   - Use the `NodePort` assigned to each service to access them.
   - Example:
     ```
     http://<node-ip>:<node-port>
     ```

## Customization

- Modify `replicas` in the Deployment files to scale the services.
- Adjust GPU limits and resources based on workload requirements.

## Included Services

- Supply Chain AI
- Predictive Analytics
- AI Chatbot
- Frontend
- Content Creation AI

## Monitoring and Logging

### Structured Logging

Enhance logging in your Rust services using `tracing` and `tracing-subscriber`.

**Example Enhancement in `crates/ai_consulting/src/main.rs`:**
