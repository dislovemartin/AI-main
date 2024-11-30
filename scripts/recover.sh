#!/bin/bash

set -euo pipefail

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "'.env' file not found. Exiting..."
    exit 1
fi

BACKUP_DIR=$1

if [ -z "$BACKUP_DIR" ]; then
    echo "Usage: $0 <backup-directory>"
    exit 1
fi

# Restore PostgreSQL
psql -h localhost -U "$POSTGRES_USER" "$POSTGRES_DB" < "$BACKUP_DIR/$POSTGRES_DB.sql"

# Restore configuration files
cp "$BACKUP_DIR/config.toml" config/

echo "Recovery completed successfully from $BACKUP_DIR" 