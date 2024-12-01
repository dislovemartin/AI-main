#!/bin/bash

# Usage: ./wait-for-service.sh <service-name>
# Example: ./wait-for-service.sh ai-chatbot-service

set -e

SERVICE_NAME=$1
MAX_ATTEMPTS=30
SLEEP_TIME=10

if [ -z "$SERVICE_NAME" ]; then
    echo "Error: Service name is required"
    echo "Usage: $0 <service-name>"
    exit 1
fi

echo "Waiting for service $SERVICE_NAME to be ready..."

for i in $(seq 1 $MAX_ATTEMPTS); do
    # Get service status from AWS ECS
    STATUS=$(aws ecs describe-services \
        --cluster ${CLUSTER_NAME:-production-cluster} \
        --services $SERVICE_NAME \
        --query 'services[0].deployments[0].status' \
        --output text)
    
    if [ "$STATUS" == "PRIMARY" ]; then
        # Check if desired count matches running count
        DESIRED=$(aws ecs describe-services \
            --cluster ${CLUSTER_NAME:-production-cluster} \
            --services $SERVICE_NAME \
            --query 'services[0].desiredCount' \
            --output text)
        
        RUNNING=$(aws ecs describe-services \
            --cluster ${CLUSTER_NAME:-production-cluster} \
            --services $SERVICE_NAME \
            --query 'services[0].runningCount' \
            --output text)
        
        if [ "$DESIRED" == "$RUNNING" ]; then
            echo "Service $SERVICE_NAME is ready!"
            exit 0
        fi
    fi
    
    echo "Attempt $i/$MAX_ATTEMPTS: Service not ready yet (Status: $STATUS, Running: $RUNNING/$DESIRED)"
    
    if [ $i -eq $MAX_ATTEMPTS ]; then
        echo "Error: Service $SERVICE_NAME failed to become ready within timeout"
        exit 1
    fi
    
    sleep $SLEEP_TIME
done 