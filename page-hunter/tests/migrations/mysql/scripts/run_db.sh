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

# Define docker container name
CONTAINER_NAME=mysql-db

# Run the database as a docker container
docker run --name $CONTAINER_NAME -e MYSQL_ROOT_PASSWORD=$DB_PASSWORD -e MYSQL_DATABASE=$DB_NAME -e MYSQL_USER=$DB_USER -e MYSQL_PASSWORD=$DB_PASSWORD -p $MYSQL_DB_PORT:3306 -d mysql