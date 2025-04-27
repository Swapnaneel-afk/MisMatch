# Railway Deployment Guide

This guide walks through deploying the MisMatch chat application's backend on Railway.

## Project Setup

We've added several configuration files to ensure Railway properly builds and deploys only the backend:

1. `railway.toml` - Main configuration file for Railway
2. `Procfile` - Alternative way to specify how to run the app
3. `nixpacks.toml` - Controls the build environment and process
4. `buildpack.yml` - Specifies the project path
5. `backend-railway.json` - Railway-specific configuration with more options

## Deployment Options

### Option 1: Deploy via GitHub Integration

1. Push your code to GitHub with all the configuration files
2. Go to [Railway Dashboard](https://railway.app/dashboard)
3. Click "New Project" → "Deploy from GitHub repo"
4. Select your repository
5. Railway will use the configuration files to build and run the backend

### Option 2: Deploy via Railway CLI

1. Install the Railway CLI:
   ```bash
   npm install -g @railway/cli
   ```

2. Login to Railway:
   ```bash
   railway login
   ```

3. Initialize a new project:
   ```bash
   railway init
   ```

4. Deploy the project:
   ```bash
   railway up
   ```

### Option 3: Use the provided deploy script

We've created a deployment script that will:
1. Create a temporary directory with just the necessary files
2. Deploy only the backend to Railway

```bash
# On Unix/Linux/Mac
chmod +x deploy-backend.sh
./deploy-backend.sh

# On Windows
# Run deploy-backend.sh with PowerShell or Git Bash
```

## Environment Variables

After deployment, set these environment variables in your Railway project:

- `PORT=8080`
- `DB_HOST` - Your PostgreSQL host
- `DB_PORT` - Your PostgreSQL port (usually 5432)
- `DB_NAME` - Your database name
- `DB_USER` - Your database username
- `DB_PASSWORD` - Your database password

## Database Setup

1. In your Railway project, click "New" → "Database" → "PostgreSQL"
2. Railway will provision a PostgreSQL database and generate connection details
3. Link the database variables to your backend service

## Frontend Deployment

For the frontend, deploy to a service like:
- Vercel
- Netlify
- GitHub Pages
- Another Railway service

Set the WebSocket URL in your frontend deployment:
```
REACT_APP_WS_URL=wss://your-backend-railway-url/ws
```

## Troubleshooting

- **Build Errors**: Check Railway logs for specific build errors
- **Missing Configuration**: Ensure all config files are in the root directory
- **Connection Issues**: Verify the WebSocket URL and CORS settings
- **Database Issues**: Check the database connection environment variables

## Monitoring and Logs

Railway provides logs for each service:

1. Go to your service in the Railway dashboard
2. Click "Logs" to view real-time logs
3. Use these logs to diagnose any issues 