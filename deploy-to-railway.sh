#!/bin/bash
set -e

echo "Starting Railway deployment..."

# Navigate to chat-backend directory
cd ./chat-backend

# Initialize a new Railway project
echo "Initializing Railway project..."
railway init

# Deploy the backend to Railway
echo "Deploying to Railway..."
railway up

# Return to the main directory
cd ..

echo "==============================================="
echo "Deployment completed! Check the Railway dashboard to see your application."
echo "To add a PostgreSQL database, run:"
echo "railway add"
echo "and select PostgreSQL from the list."
echo "===============================================" 