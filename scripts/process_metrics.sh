#!/bin/bash

# Usage: ./process_metrics.sh <metrics-file>
# Example: ./process_metrics.sh metrics.json

set -e

METRICS_FILE=$1
THRESHOLD_FILE=${2:-"config/thresholds.json"}

if [ -z "$METRICS_FILE" ]; then
    echo "Error: Metrics file is required"
    echo "Usage: $0 <metrics-file> [threshold-file]"
    exit 1
fi

if [ ! -f "$METRICS_FILE" ]; then
    echo "Error: Metrics file '$METRICS_FILE' not found"
    exit 1
fi

# Generate report header
cat << EOF
# Service Health Report
Generated at: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

## Overview
EOF

# Process each service's metrics
jq -r '.[] | select(.service)' "$METRICS_FILE" | while read -r service_data; do
    SERVICE_NAME=$(echo "$service_data" | jq -r '.service')
    TIMESTAMP=$(echo "$service_data" | jq -r '.timestamp')
    
    echo -e "\n### $SERVICE_NAME"
    echo "Last updated: $TIMESTAMP"
    echo -e "\nMetrics:"
    
    # Process metrics based on service type
    case "$SERVICE_NAME" in
        "ai-chatbot")
            # Process AI Chatbot specific metrics
            REQUEST_COUNT=$(echo "$service_data" | jq -r '.metrics.request_count[0].value[1]')
            ERROR_RATE=$(echo "$service_data" | jq -r '.metrics.error_count[0].value[1]')
            LATENCY=$(echo "$service_data" | jq -r '.metrics.request_latency_seconds[0].value[1]')
            
            echo "- Request Count (5m): $REQUEST_COUNT"
            echo "- Error Rate (5m): $ERROR_RATE%"
            echo "- Average Latency: ${LATENCY}s"
            ;;
            
        "predictive-analytics")
            # Process Predictive Analytics specific metrics
            PREDICTION_COUNT=$(echo "$service_data" | jq -r '.metrics.prediction_count[0].value[1]')
            MODEL_ACCURACY=$(echo "$service_data" | jq -r '.metrics.model_accuracy[0].value[1]')
            
            echo "- Predictions (5m): $PREDICTION_COUNT"
            echo "- Model Accuracy: $MODEL_ACCURACY%"
            ;;
            
        "personalization")
            # Process Personalization Engine specific metrics
            RECOMMENDATION_COUNT=$(echo "$service_data" | jq -r '.metrics.recommendation_count[0].value[1]')
            CACHE_HIT_RATIO=$(echo "$service_data" | jq -r '.metrics.cache_hit_ratio[0].value[1]')
            
            echo "- Recommendations (5m): $RECOMMENDATION_COUNT"
            echo "- Cache Hit Ratio: $CACHE_HIT_RATIO%"
            ;;
            
        "automl")
            # Process AutoML specific metrics
            TRAINING_JOBS=$(echo "$service_data" | jq -r '.metrics.training_jobs_count[0].value[1]')
            EVAL_TIME=$(echo "$service_data" | jq -r '.metrics.model_evaluation_time[0].value[1]')
            
            echo "- Active Training Jobs: $TRAINING_JOBS"
            echo "- Average Evaluation Time: ${EVAL_TIME}s"
            ;;
    esac
    
    # Check for alerts
    if [ -f "$THRESHOLD_FILE" ]; then
        echo -e "\nAlerts:"
        jq -r --arg service "$SERVICE_NAME" '.[$service].thresholds[]' "$THRESHOLD_FILE" | while read -r threshold; do
            METRIC=$(echo "$threshold" | jq -r '.metric')
            VALUE=$(echo "$service_data" | jq -r ".metrics.$METRIC[0].value[1]")
            MAX=$(echo "$threshold" | jq -r '.max')
            
            if (( $(echo "$VALUE > $MAX" | bc -l) )); then
                echo "⚠️ $METRIC exceeds threshold: $VALUE > $MAX"
            fi
        done
    fi
done

# Generate summary
echo -e "\n## Summary"
echo "Total services monitored: $(jq -r '[.[] | select(.service)] | length' "$METRICS_FILE")"
echo "Services with alerts: $(grep -c "⚠️" "$METRICS_FILE" || true)" 