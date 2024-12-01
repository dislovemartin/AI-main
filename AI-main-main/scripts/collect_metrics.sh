#!/bin/bash

# Usage: ./collect_metrics.sh <service-name>
# Example: ./collect_metrics.sh ai-chatbot

set -e

SERVICE_NAME=$1
PROMETHEUS_URL=${PROMETHEUS_URL:-"https://prometheus.example.com"}

if [ -z "$SERVICE_NAME" ]; then
    echo "Error: Service name is required"
    echo "Usage: $0 <service-name>"
    exit 1
fi

# Service-specific metrics queries
declare -A METRICS
METRICS=(
    ["ai-chatbot"]="
        request_count{service='ai-chatbot'}[5m]
        request_latency_seconds{service='ai-chatbot'}
        error_count{service='ai-chatbot'}[5m]
        model_inference_time{service='ai-chatbot'}
    "
    ["predictive-analytics"]="
        prediction_count{service='predictive-analytics'}[5m]
        model_accuracy{service='predictive-analytics'}
        training_time{service='predictive-analytics'}
    "
    ["personalization"]="
        recommendation_count{service='personalization'}[5m]
        user_interaction_count{service='personalization'}
        cache_hit_ratio{service='personalization'}
    "
    ["automl"]="
        training_jobs_count{service='automl'}
        model_evaluation_time{service='automl'}
        optimization_progress{service='automl'}
    "
)

if [ -z "${METRICS[$SERVICE_NAME]}" ]; then
    echo "Error: Unknown service '$SERVICE_NAME'"
    exit 1
fi

echo "Collecting metrics for $SERVICE_NAME..."

# Create temporary file for metrics
TEMP_FILE=$(mktemp)
echo "{" > "$TEMP_FILE"
echo "  \"service\": \"$SERVICE_NAME\"," >> "$TEMP_FILE"
echo "  \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"," >> "$TEMP_FILE"
echo "  \"metrics\": {" >> "$TEMP_FILE"

# Collect each metric
FIRST=true
for query in ${METRICS[$SERVICE_NAME]}; do
    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        echo "," >> "$TEMP_FILE"
    fi
    
    RESPONSE=$(curl -s -G --data-urlencode "query=$query" "$PROMETHEUS_URL/api/v1/query")
    
    # Extract metric name from query
    METRIC_NAME=$(echo "$query" | cut -d'{' -f1)
    
    echo "    \"$METRIC_NAME\": $(echo "$RESPONSE" | jq '.data.result')" >> "$TEMP_FILE"
done

echo "  }" >> "$TEMP_FILE"
echo "}" >> "$TEMP_FILE"

# Output the collected metrics
cat "$TEMP_FILE"
rm "$TEMP_FILE" 