# Railway Deployment Instructions

## Important Notes

1. This app is pre-built for deployment. The binary is already compiled in the `target/release` directory.
2. Use the included Dockerfile for deployment.
3. Add these environment variables in the Railway dashboard:
   - `PORT`: 8080
   - `RUST_LOG`: info
   - `DB_HOST`: [Your PostgreSQL host]
   - `DB_PORT`: 5432
   - `DB_NAME`: [Your database name]
   - `DB_USER`: [Your database user]
   - `DB_PASSWORD`: [Your database password]

## Steps for Deployment

1. Create a new service in Railway
2. Connect your GitHub repository
3. Select the `chat-backend` directory as the source
4. Set Railway to use the Dockerfile
5. Add the environment variables listed above
6. Add a PostgreSQL database from the Railway add-ons
7. Configure the health check path to `/health`

## Testing Locally

```
cargo build --release
./target/release/chat-backend
```

## Troubleshooting

If the deployment fails, check these common issues:

1. Make sure the binary is built for a compatible target
2. Verify the environment variables are correctly set
3. Check the application logs for specific errors
4. Ensure the database connection string is correct
5. Try running with `RUST_BACKTRACE=1` for more detailed error output 