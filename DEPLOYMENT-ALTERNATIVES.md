# Alternative Deployment Options

Since Railway has a limit on free resources, here are some alternative platforms that offer more generous free tiers for your chat application.

## Option 1: Render.com

Render offers a generous free tier and works well with Rust applications.

### Deployment Steps:

1. Sign up at [Render.com](https://render.com)
2. Connect your GitHub repository
3. Click "New Web Service"
4. Select your repository and the `chat-backend` directory
5. Configure as:
   - Environment: Rust
   - Build Command: `cargo build --release`
   - Start Command: `./target/release/chat-backend`
6. Add environment variables:
   - `PORT=10000`
   - `RUST_LOG=info`
   - Database variables (if using Render's PostgreSQL)

### Database:

Create a PostgreSQL database in Render and link it to your service.

### Frontend Deployment:

Deploy your frontend to a Static Site service on Render:
1. Create a new Static Site
2. Point to your `chat-frontend` directory
3. Build Command: `npm ci && npm run build`
4. Publish Directory: `build`
5. Set `REACT_APP_WS_URL=wss://your-backend-url/ws`

## Option 2: Fly.io

Fly.io has an excellent free tier and global distribution.

### Prerequisites:

1. Install the Fly CLI:
   ```bash
   curl -L https://fly.io/install.sh | sh
   ```
2. Login to Fly:
   ```bash
   fly auth login
   ```

### Backend Deployment:

1. Navigate to your backend directory:
   ```bash
   cd chat-backend
   ```
2. Launch your app:
   ```bash
   fly launch
   ```
3. Deploy:
   ```bash
   fly deploy
   ```

### Database:

Create a PostgreSQL database:
```bash
fly postgres create --name mismatch-db
```

Connect to your app:
```bash
fly postgres attach --app mismatch-backend mismatch-db
```

### Frontend Deployment:

You can deploy the frontend to Netlify, Vercel, or as a separate Fly app.

## Option 3: Shuttle.rs (Rust-specific)

[Shuttle.rs](https://shuttle.rs) is specifically designed for Rust applications and has a generous free tier.

### Deployment Steps:

1. Sign up for an account
2. Install Shuttle CLI:
   ```bash
   cargo install cargo-shuttle
   ```
3. Login:
   ```bash
   cargo shuttle login
   ```
4. Initialize your project:
   ```bash
   cd chat-backend
   cargo shuttle init
   ```
5. Deploy:
   ```bash
   cargo shuttle deploy
   ```

## Option 4: Heroku (Reduced Free Tier)

While Heroku has reduced its free tier, it's still an option for short-term deployment.

### Backend Deployment:

1. Create a `Procfile` in your backend directory:
   ```
   web: ./target/release/chat-backend
   ```
2. Create a Heroku app:
   ```bash
   heroku create mismatch-backend
   ```
3. Add PostgreSQL:
   ```bash
   heroku addons:create heroku-postgresql:hobby-dev
   ```
4. Deploy:
   ```bash
   git subtree push --prefix chat-backend heroku main
   ```

## Connecting the Frontend

For all options, deploy the frontend separately and set the WebSocket URL environment variable to point to your backend:

```
REACT_APP_WS_URL=wss://your-backend-url/ws
```

Remember to update the CORS settings in your backend to allow connections from your frontend domain. 