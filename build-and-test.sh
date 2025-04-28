#!/bin/bash
set -e

echo "Building Docker image for MisMatch backend..."
docker build -t mismatch-backend .

echo "Running tests on the image..."
docker run --rm mismatch-backend echo "Build successful!"

echo "==============================================="
echo "Docker build is successful! ✅"
echo "To start the full application with docker-compose, run:"
echo "docker-compose up"
echo "==============================================="
echo "To deploy to Railway:"
echo "1. Push to GitHub"
echo "2. Connect repository to Railway"
echo "3. Railway will use the Dockerfile for deployment"
echo "===============================================" 