#!/bin/bash

# Check if DB host is set
if [ -z "$DB_HOST" ]; then
  echo "ERROR: DB_HOST not found"
  exit 1
fi

# Check if DB port is set
if [ -z "$PG_DB_PORT" ]; then
  echo "ERROR: PG_DB_PORT not found"
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

# Define docker container name
CONTAINER_NAME=postgres-db

# Create the database as a docker container
docker run --name $CONTAINER_NAME  -e POSTGRES_PASSWORD=$DB_PASSWORD -e POSTGRES_DB=$DB_NAME -e POSTGRES_USER=$DB_USER -p $PG_DB_PORT:5432 -d postgres
