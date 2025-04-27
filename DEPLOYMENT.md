# Deployment Guide

This guide walks through deploying the MisMatch chat application on Railway, a platform that simplifies container-based deployments.

## Prerequisites

1. [Railway account](https://railway.app/)
2. [Railway CLI](https://docs.railway.app/develop/cli) (optional but recommended)
3. [GitHub account](https://github.com/) (for source integration)

## Preparing the Backend

The backend is already containerized with a Dockerfile. Let's make some adjustments to ensure it's ready for production deployment:

1. Make sure the backend listens on the port provided by Railway:

In `chat-backend/src/main.rs`, we need to update the binding address to use the PORT environment variable provided by Railway.

2. Update CORS settings to allow your deployed frontend URL.

## Preparing the Frontend

We need to create a Dockerfile for the frontend:

```bash
# Create a Dockerfile in the frontend directory
touch chat-frontend/Dockerfile
```

## Deployment Steps

### 1. Push to GitHub

First, push your code to GitHub:

```bash
git add .
git commit -m "Prepare for Railway deployment"
git push origin main
```

### 2. Deploy Backend on Railway

1. Go to [Railway Dashboard](https://railway.app/dashboard)
2. Click "New Project" → "Deploy from GitHub repo"
3. Select your repository
4. Choose the backend directory (`chat-backend`)
5. Add the following environment variables:
   - `PORT=8080`
   - `DB_HOST` - Your PostgreSQL host
   - `DB_PORT` - Your PostgreSQL port (usually 5432)
   - `DB_NAME` - Your database name
   - `DB_USER` - Your database username
   - `DB_PASSWORD` - Your database password

Railway will automatically detect the Dockerfile and build it.

### 3. Set Up PostgreSQL on Railway

1. In your project, click "New" → "Database" → "PostgreSQL"
2. Railway will provision a PostgreSQL database and generate the connection details
3. Go to your backend service and add the connection details to your environment variables (or use Railway's variable linking)

### 4. Deploy Frontend on Railway

1. In your project dashboard, click "New" → "Service"
2. Choose "GitHub Repo" again and select the same repository
3. This time, specify the frontend directory (`chat-frontend`)
4. Add the following environment variables:
   - `REACT_APP_WS_URL=wss://your-backend-service-url` (use your backend service domain)

### 5. Configure Domains (Optional)

1. In each service, go to "Settings" → "Domains"
2. Click "Generate Domain" or add a custom domain

## Testing the Deployment

After deployment completes:

1. Visit your frontend domain
2. Open the app in multiple browser windows
3. Test real-time chat functionality

## Troubleshooting

- **Connection Issues**: Make sure the WebSocket URL in the frontend is correctly pointing to your backend service
- **Database Errors**: Verify the database connection environment variables
- **CORS Errors**: Ensure the backend CORS settings include your frontend domain

## Monitoring and Logs

Railway provides logs for each service:

1. Go to your service in the Railway dashboard
2. Click "Logs" to view real-time logs
3. Use these logs to diagnose any issues 