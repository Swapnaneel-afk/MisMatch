# MisMatch - Real-time Chat Application

MisMatch is a real-time chat application with a React frontend and Rust backend. It supports multiple chat rooms, user typing indicators, and message history.

## Project Structure

- **Frontend**: React with Material UI
- **Backend**: Rust with Actix Web, WebSockets, and PostgreSQL

## Features

- User registration
- Multiple chat rooms
- Real-time messaging with WebSockets
- Typing indicators
- Message history
- Dark/light theme
- Responsive design

## Requirements

### For Local Development

- Node.js (v16+)
- Rust (2021 edition)
- PostgreSQL

### For Deployment

- Vercel account (frontend)
- Railway account (backend and database)

## Local Development Setup

### Backend Setup

1. Install Rust if you haven't already:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install PostgreSQL and create a database:
   ```
   psql -U postgres
   CREATE DATABASE chat_db;
   ```

3. Create a `.env` file in the `chat-backend` directory:
   ```
   # Server Configuration
   HOST=127.0.0.1
   PORT=8080
   FRONTEND_URL=http://localhost:3000

   # Database Configuration
   DB_HOST=localhost
   DB_PORT=5432
   DB_NAME=chat_db
   DB_USER=postgres
   DB_PASSWORD=your_password
   ```

4. Run the backend:
   ```
   cd chat-backend
   cargo run
   ```

### Frontend Setup

1. Install dependencies:
   ```
   cd chat-frontend
   npm install
   ```

2. Start the development server:
   ```
   npm start
   ```

3. Open http://localhost:3000 in your browser

## Deployment Instructions

### Backend Deployment (Railway)

1. Create a Railway account at https://railway.app/

2. Install the Railway CLI:
   ```
   npm i -g @railway/cli
   ```

3. Login to Railway:
   ```
   railway login
   ```

4. Initialize Railway in the backend directory:
   ```
   cd chat-backend
   railway init
   ```

5. Create a PostgreSQL database in Railway:
   ```
   railway add
   ```
   Select PostgreSQL from the list of plugins.

6. Configure environment variables in Railway:
   - `HOST=0.0.0.0`
   - `PORT=8080`
   - `FRONTEND_URL=https://your-vercel-app-name.vercel.app`
   - `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_USER`, and `DB_PASSWORD` will be automatically set by the PostgreSQL plugin

7. Deploy the backend:
   ```
   railway up
   ```

8. Make note of your backend's domain for configuring the frontend (e.g., `https://mismatch-production.up.railway.app`)

### Frontend Deployment (Vercel)

1. Create a Vercel account at https://vercel.com/

2. Install the Vercel CLI:
   ```
   npm i -g vercel
   ```

3. Login to Vercel:
   ```
   vercel login
   ```

4. Configure the `.env` file in the frontend directory to use your backend URL:
   ```
   REACT_APP_API_URL=https://your-railway-app-name.up.railway.app
   ```

5. Deploy the frontend:
   ```
   cd chat-frontend
   vercel
   ```

6. After deployment, get your frontend URL (e.g., `https://mismatch.vercel.app`)

7. Update the `FRONTEND_URL` environment variable in your Railway backend to match your Vercel frontend URL to ensure CORS works correctly.

## Development Roadmap

For future improvements to the application, see [implementation.md](implementation.md).

## License

MIT

