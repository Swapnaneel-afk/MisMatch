# MisMatch Chat Application

A real-time chat application with rooms and persistent messaging built with Rust (backend) and React (frontend).

## Features

- Real-time messaging using WebSockets
- User presence indicators (online/offline status)
- Typing indicators
- Persistent message storage in PostgreSQL
- Chat rooms (public, private, and password-protected)
- Message history

## Architecture

### Backend (Rust)
- **Framework**: Actix Web + Actix WebSockets
- **Database**: PostgreSQL with indexed tables for better performance
- **Authentication**: Simple username-based system (for local development)

### Frontend (React)
- **Framework**: React with TypeScript
- **WebSocket**: Native WebSocket implementation
- **Styling**: CSS modules

## Local Development Setup

### Prerequisites
- Docker and Docker Compose
- Node.js and npm (if running the frontend separately)
- Rust and Cargo (if running the backend separately)

### Running with Docker Compose (Recommended)

The easiest way to get started is using Docker Compose, which sets up PostgreSQL, the backend, and the frontend:

```bash
# Start all services
docker-compose up

# Or in detached mode
docker-compose up -d
```

The application will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- WebSocket: ws://localhost:8080/ws

### Running Locally (Without Docker)

#### Backend

1. Install PostgreSQL and create a database:
   ```bash
   # Create a chat_db database
   createdb chat_db
   ```

2. Configure environment variables:
   ```bash
   # Create a .env file in the chat-backend directory
   echo "DATABASE_URL=postgresql://postgres:password@localhost:5432/chat_db" > chat-backend/.env
   echo "RUST_LOG=info" >> chat-backend/.env
   ```

3. Run the backend:
   ```bash
   cd chat-backend
   cargo run
   ```

#### Frontend

1. Configure the WebSocket URL:
   ```bash
   # Create a .env file in the chat-frontend directory
   echo "REACT_APP_WS_URL=ws://localhost:8080/ws" > chat-frontend/.env
   ```

2. Install dependencies and start the development server:
   ```bash
   cd chat-frontend
   npm install
   npm start
   ```

## Using the Chat Application

1. Open the application in your browser at http://localhost:3000
2. Enter your username to join the global chat
3. Use the room controls to:
   - Create a new room
   - Join an existing room
   - Leave a room
4. Send messages in the current room or global chat

## API Endpoints

- `GET /health` - Health check endpoint
- `GET /ws?username={username}` - WebSocket connection endpoint
- `GET /api/rooms` - List all available rooms
- `POST /api/rooms` - Create a new room

## Database Schema

The PostgreSQL database includes the following tables:

- `users` - User information
- `rooms` - Chat room definitions
- `room_members` - User membership in rooms
- `messages` - Chat messages with room association

All tables are properly indexed for optimal performance.

## Future Improvements

- User authentication with JWT tokens
- User profile management
- File uploads and sharing
- Message reactions
- Direct messaging between users

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
