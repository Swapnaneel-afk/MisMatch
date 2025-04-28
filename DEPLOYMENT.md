# Deployment Guide for MisMatch

This guide walks through deploying the MisMatch application on Railway and alternative platforms.

## Railway Deployment with Docker (Recommended)

Railway offers a simple deployment platform with PostgreSQL integration. We've configured a Docker-based deployment for maximum reliability.

### Prerequisites

1. [Railway account](https://railway.app/)
2. [GitHub account](https://github.com/) (for repository connection)

### Deployment Steps

1. Push your code to a GitHub repository
2. Log in to Railway Dashboard at [https://railway.app/dashboard](https://railway.app/dashboard)
3. Click "New Project" → "Deploy from GitHub repo"
4. Select your repository
5. Railway will automatically detect the Dockerfile and railway.toml configuration
6. The backend will be built and deployed using the Dockerfile

### Database Setup

1. In your project dashboard, click "New" → "Database" → "PostgreSQL"
2. Railway will automatically create a PostgreSQL database
3. Copy the connection details from the Variables tab

### Environment Variables

Set these environment variables in your Railway project:
- `PORT`: (Railway sets this automatically)
- `DB_HOST`: PostgreSQL host from the database connection
- `DB_PORT`: PostgreSQL port (usually 5432)
- `DB_NAME`: PostgreSQL database name
- `DB_USER`: PostgreSQL username
- `DB_PASSWORD`: PostgreSQL password

### Troubleshooting Docker Builds

If your Docker build fails:

1. Check the build logs in Railway dashboard
2. Make sure all necessary files are included in the Docker build context
3. Try building the Docker image locally to test:
   ```bash
   docker build -t mismatch-backend .
   ```

## Frontend Deployment Options

### Option 1: Vercel (Recommended for Frontend)

1. Push your code to GitHub
2. Sign up at [Vercel](https://vercel.com/)
3. Create a new project and connect to your repository
4. Set the "Root Directory" to "chat-frontend"
5. Add the environment variable:
   ```
   REACT_APP_WS_URL=wss://your-backend-railway-url/ws
   ```
6. Deploy

### Option 2: Railway for Frontend

1. In your Railway dashboard, create a new service
2. Connect to the same GitHub repository
3. Set the root directory to `chat-frontend`
4. Add build command: `npm install && npm run build`
5. Add start command: `npx serve -s build`
6. Add the environment variable:
   ```
   REACT_APP_WS_URL=wss://your-backend-railway-url/ws
   ```

## Alternative Deployment: Render.com

If you encounter issues with Railway, Render.com is an excellent alternative.

### Deployment Steps

1. Push your code to GitHub
2. Sign up at [Render.com](https://render.com/)
3. Create a new "Blueprint" and connect to your repository
4. Render will use the `render.yaml` configuration to set up:
   - Backend Rust web service
   - PostgreSQL database
   - Environment variables

Or deploy manually:

1. Create a new Web Service for the backend
   - Set root directory to `chat-backend`
   - Build command: `cargo build --release`
   - Start command: `./target/release/chat-backend`

2. Create a PostgreSQL database

3. Create a Static Site for the frontend
   - Set root directory to `chat-frontend`
   - Build command: `npm install && npm run build`
   - Publish directory: `build`

## Connection Issues

If the frontend can't connect to the backend:

1. Verify the `REACT_APP_WS_URL` is set correctly
2. Check CORS settings in `main.rs`
3. Ensure the backend is running and accessible 