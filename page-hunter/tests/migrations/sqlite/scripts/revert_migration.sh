#!/bin/bash

# Check if DB path is set
if [ -z "$SQLITE_DB_PATH" ]; then
  echo "ERROR: The SQLITE_DB_PATH not found"
  exit 1
fi

# Check if migrations path is set
if [ -z "$SQLITE_MIGRATIONS_PATH" ]; then
  echo "ERROR: SQLITE_MIGRATIONS_PATH not found"
  exit 1
fi

# Revert the migration
sqlx migrate revert --source $SQLITE_MIGRATIONS_PATH --database-url sqlite:$SQLITE_DB_PATH
