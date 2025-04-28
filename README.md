# MisMatch

A real-time chat application with a React frontend and Rust backend.

## Project Structure

- **chat-frontend**: React application with Material UI
- **chat-backend**: Rust Actix-Web server with WebSocket support

## Features

- Real-time messaging with WebSockets
- User join/leave notifications
- Online user list
- Typing indicators
- Message timestamps
- Light/dark theme toggle
- Avatar generation based on username

## Local Development

### Backend

1. Navigate to the backend directory:
   ```
   cd chat-backend
   ```

2. Run the Rust server:
   ```
   cargo run
   ```

### Frontend

1. Navigate to the frontend directory:
   ```
   cd chat-frontend
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Start the development server:
   ```
   npm start
   ```

## Docker Deployment

You can build and run the application using Docker:

```bash
# Build the backend
docker build -t mismatch-backend .

# Run the backend
docker run -p 8080:8080 mismatch-backend
```

## Railway Deployment

This project is configured for deployment on Railway using Docker:

1. Push this repository to GitHub
2. Connect your Railway account to your GitHub repository
3. Railway will automatically detect the Dockerfile and railway.toml configuration
4. The backend will be built and deployed using the Dockerfile

### Environment Variables

Configure these environment variables in Railway:
- `PORT`: Port for the backend server (Railway sets this automatically)
- `DB_HOST`: PostgreSQL database host
- `DB_PORT`: PostgreSQL database port
- `DB_NAME`: PostgreSQL database name
- `DB_USER`: PostgreSQL database username
- `DB_PASSWORD`: PostgreSQL database password

### Frontend Deployment

For the frontend, you can deploy to:
- Vercel
- Netlify 
- Railway (as a separate service)

Set the environment variable in your frontend deployment:
```
REACT_APP_WS_URL=wss://your-backend-service.up.railway.app/ws
```

## License

MIT
