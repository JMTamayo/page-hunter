#!/bin/bash

# Check if DB host is set
if [ -z "$DB_HOST" ]; then
  echo "ERROR: DB_HOST not found"
  exit 1
fi

# Check if DB port is set
if [ -z "$MYSQL_DB_PORT" ]; then
  echo "ERROR: MYSQL_DB_PORT not found"
  exit 1
fi

# Check if DB user is set
if [ -z "$DB_USER" ]; then
  echo "ERROR: DB_USER not found"
  exit 1
fi

# Check if DB password is set
if [ -z "$DB_PASSWORD" ]; then
  echo "ERROR: DB_PASSWORD not found"
  exit 1
fi

# Check if DB name is set
if [ -z "$DB_NAME" ]; then
  echo "ERROR: DB_NAME not found"
  exit 1
fi

# Check if migrations path is set
if [ -z "$MYSQL_MIGRATIONS_PATH" ]; then
  echo "ERROR: MYSQL_MIGRATIONS_PATH not found"
  exit 1
fi

# revert migration
sqlx migrate revert --source $MYSQL_MIGRATIONS_PATH --database-url mysql://$DB_USER:$DB_PASSWORD@$DB_HOST:$MYSQL_DB_PORT/$DB_NAME