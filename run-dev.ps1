# Start PostgreSQL in Docker if not already running
$postgres_container = "mismatch_postgres"
$postgres_running = docker ps -q -f name=$postgres_container

if (!$postgres_running) {
    Write-Host "Starting PostgreSQL container..."
    docker run --name $postgres_container `
        -e POSTGRES_USER=postgres `
        -e POSTGRES_PASSWORD=password `
        -e POSTGRES_DB=chat_db `
        -p 5432:5432 `
        -d postgres:14-alpine
    
    # Wait for PostgreSQL to be ready
    Write-Host "Waiting for PostgreSQL to be ready..."
    Start-Sleep -Seconds 5
} else {
    Write-Host "PostgreSQL container is already running."
}

# Set environment variables for the backend
$env:DATABASE_URL = "postgresql://postgres:password@localhost:5432/chat_db"
$env:RUST_LOG = "info"
$env:PORT = 8080

# Change to backend directory
Set-Location -Path .\chat-backend

# Run the backend
Write-Host "Starting the backend..."
cargo run 