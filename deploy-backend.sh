#!/bin/bash

# Make the script exit on any error
set -e

echo "Deploying chat-backend to Railway..."

# Check if Railway CLI is installed
if ! command -v railway &> /dev/null; then
    echo "Railway CLI not found. Please install it first:"
    echo "npm install -g @railway/cli"
    exit 1
fi

# Ensure the user is logged in
railway login

# Create a temporary directory for deployment
TEMP_DIR=$(mktemp -d)
echo "Created temporary directory: $TEMP_DIR"

# Copy only the backend code
cp -r chat-backend $TEMP_DIR/
cp railway.toml $TEMP_DIR/
cp Procfile $TEMP_DIR/
cp nixpacks.toml $TEMP_DIR/

# Move to the temporary directory
cd $TEMP_DIR

# Initialize Railway
echo "Initializing Railway project..."
railway init

# Deploy the project
echo "Deploying to Railway..."
railway up

echo "Deployment complete! Check your Railway dashboard for details."
echo "Don't forget to set the required environment variables in the Railway dashboard." 