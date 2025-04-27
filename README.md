# MisMatch

A real-time chat application with a React frontend and Rust backend.

## Project Structure

- **chat-frontend**: React 19 application with Material UI
- **chat-backend**: Rust Actix-Web server with WebSocket support

## Quick Start

You can run the entire application using Docker Compose:

```bash
docker-compose up
```

This will start:
- PostgreSQL database
- Rust backend server on port 8080
- React frontend on port 3000

## Development

See [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for a detailed project overview.

## Deployment

### Railway Deployment

We've configured the project to deploy to Railway. However, Railway has limits on free resources.

### Alternative Free Deployment Options

We've provided configurations for multiple free hosting options:

1. **Render.com**: 
   - Configuration: `chat-backend/render.yaml`
   - Deployment guide: `DEPLOYMENT-ALTERNATIVES.md`
   - Deploy script: `deploy-to-render.ps1`

2. **Fly.io**:
   - Configuration: `chat-backend/fly.toml`
   - Deployment guide: `DEPLOYMENT-ALTERNATIVES.md`

3. **Shuttle.rs** (Rust-specific):
   - Configuration: `chat-backend/Shuttle.toml`
   - Deployment guide: `DEPLOYMENT-ALTERNATIVES.md`

4. **Heroku**:
   - Deployment guide: `DEPLOYMENT-ALTERNATIVES.md`

For detailed instructions, see [DEPLOYMENT-ALTERNATIVES.md](DEPLOYMENT-ALTERNATIVES.md).

## Frontend Deployment

The frontend can be deployed separately to:
- Vercel
- Netlify
- GitHub Pages
- Render.com

Make sure to set the `REACT_APP_WS_URL` environment variable to point to your deployed backend's WebSocket URL.

## License

MIT
