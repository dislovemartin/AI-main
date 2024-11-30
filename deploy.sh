#!/bin/bash

set -e

echo "Deploying services to Kubernetes..."

kubectl apply -f manifests/

echo "Deployment completed successfully." 