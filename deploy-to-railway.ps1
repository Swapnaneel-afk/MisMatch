Write-Host "Starting Railway deployment..." -ForegroundColor Green

# Navigate to chat-backend directory
Set-Location -Path .\chat-backend

# Initialize a new Railway project
Write-Host "Initializing Railway project..." -ForegroundColor Cyan
railway init

# Deploy the backend to Railway
Write-Host "Deploying to Railway..." -ForegroundColor Cyan
railway up

# Return to the main directory
Set-Location -Path ..

Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "Deployment completed! Check the Railway dashboard to see your application." -ForegroundColor Green
Write-Host "To add a PostgreSQL database, run:" -ForegroundColor Cyan
Write-Host "railway add" -ForegroundColor Yellow
Write-Host "and select PostgreSQL from the list." -ForegroundColor Yellow
Write-Host "===============================================" -ForegroundColor Cyan 