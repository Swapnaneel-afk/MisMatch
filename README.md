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

### Railway Deployment (Backend Only)

We've configured the project to deploy only the backend to Railway:

1. Make sure the `railway.toml`, `Procfile`, and `nixpacks.toml` files are in the root directory
2. Push your code to GitHub
3. Deploy from the Railway dashboard by connecting to your GitHub repository
4. Railway will use these configuration files to build and deploy only the backend

For detailed instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

### Frontend Deployment

The frontend can be deployed separately to:
- Vercel
- Netlify
- Another Railway service
- Any other frontend hosting platform

Make sure to set the `REACT_APP_WS_URL` environment variable to point to your deployed backend's WebSocket URL.

## License

MIT
