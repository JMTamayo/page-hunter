#!/bin/bash

# Check if DB path is set
if [ -z "$SQLITE_DB_PATH" ]; then
  echo "ERROR: The SQLITE_DB_PATH not found"
  exit 1
fi

# Create the database file
touch $SQLITE_DB_PATH
