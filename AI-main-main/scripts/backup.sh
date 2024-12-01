#!/bin/bash

set -euo pipefail

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "'.env' file not found. Exiting..."
    exit 1
fi

BACKUP_DIR="backups/$(date +'%Y-%m-%d')"
mkdir -p "$BACKUP_DIR"

# Backup PostgreSQL
pg_dump -h localhost -U "$POSTGRES_USER" "$POSTGRES_DB" > "$BACKUP_DIR/$POSTGRES_DB.sql"

# Backup configuration files
cp config/config.toml "$BACKUP_DIR/"

echo "Backup completed successfully at $BACKUP_DIR" 