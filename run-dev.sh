#!/bin/bash

# Start PostgreSQL in Docker if not already running
POSTGRES_CONTAINER="mismatch_postgres"
POSTGRES_RUNNING=$(docker ps -q -f name=$POSTGRES_CONTAINER)

if [ -z "$POSTGRES_RUNNING" ]; then
    echo "Starting PostgreSQL container..."
    docker run --name $POSTGRES_CONTAINER \
        -e POSTGRES_USER=postgres \
        -e POSTGRES_PASSWORD=password \
        -e POSTGRES_DB=chat_db \
        -p 5432:5432 \
        -d postgres:14-alpine
    
    # Wait for PostgreSQL to be ready
    echo "Waiting for PostgreSQL to be ready..."
    sleep 5
else
    echo "PostgreSQL container is already running."
fi

# Set environment variables for the backend
export DATABASE_URL="postgresql://postgres:password@localhost:5432/chat_db"
export RUST_LOG="info"
export PORT=8080

# Change to backend directory
cd ./chat-backend

# Run the backend
echo "Starting the backend..."
cargo run 