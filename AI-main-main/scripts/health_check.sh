#!/bin/bash

# Usage: ./health_check.sh <environment> <service-name>
# Example: ./health_check.sh production ai-chatbot

set -e

ENVIRONMENT=$1
SERVICE_NAME=$2
MAX_RETRIES=3
SLEEP_TIME=5

if [ -z "$ENVIRONMENT" ] || [ -z "$SERVICE_NAME" ]; then
    echo "Error: Both environment and service name are required"
    echo "Usage: $0 <environment> <service-name>"
    exit 1
fi

# Load environment-specific configuration
case "$ENVIRONMENT" in
    production)
        BASE_URL="https://ai-platform.example.com"
        ;;
    staging)
        BASE_URL="https://staging.example.com"
        ;;
    *)
        echo "Error: Invalid environment. Must be 'production' or 'staging'"
        exit 1
        ;;
esac

# Service-specific health check endpoints
declare -A ENDPOINTS
ENDPOINTS=(
    ["ai-chatbot"]="/api/v1/health"
    ["predictive-analytics"]="/api/v1/health"
    ["personalization"]="/api/v1/health"
    ["automl"]="/api/v1/health"
)

if [ -z "${ENDPOINTS[$SERVICE_NAME]}" ]; then
    echo "Error: Unknown service '$SERVICE_NAME'"
    exit 1
fi

HEALTH_URL="$BASE_URL${ENDPOINTS[$SERVICE_NAME]}"
echo "Checking health of $SERVICE_NAME at $HEALTH_URL"

for i in $(seq 1 $MAX_RETRIES); do
    RESPONSE=$(curl -s -w "\n%{http_code}" "$HEALTH_URL")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | head -n-1)
    
    if [ "$HTTP_CODE" == "200" ]; then
        echo "Health check passed for $SERVICE_NAME"
        
        # Check response body for detailed health status
        if echo "$BODY" | jq -e '.status == "healthy"' > /dev/null 2>&1; then
            echo "Service reports healthy status"
            exit 0
        else
            echo "Warning: Service returned 200 but may not be fully healthy"
            echo "Response: $BODY"
        fi
    else
        echo "Attempt $i/$MAX_RETRIES: Health check failed with status $HTTP_CODE"
        echo "Response: $BODY"
        
        if [ $i -eq $MAX_RETRIES ]; then
            echo "Error: Health check failed after $MAX_RETRIES attempts"
            exit 1
        fi
        
        sleep $SLEEP_TIME
    fi
done 