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

The application is designed to be easily deployed on Railway. For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

## License

MIT
