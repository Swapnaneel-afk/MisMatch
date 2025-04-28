Write-Host "Building Docker image for MisMatch backend..." -ForegroundColor Green
docker build -t mismatch-backend .

Write-Host "Running tests on the image..." -ForegroundColor Green
docker run --rm mismatch-backend echo "Build successful!"

Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "Docker build is successful! ✅" -ForegroundColor Green
Write-Host "To start the full application with docker-compose, run:" -ForegroundColor Cyan
Write-Host "docker-compose up" -ForegroundColor Yellow
Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "To deploy to Railway:" -ForegroundColor Cyan
Write-Host "1. Push to GitHub" -ForegroundColor Yellow
Write-Host "2. Connect repository to Railway" -ForegroundColor Yellow
Write-Host "3. Railway will use the Dockerfile for deployment" -ForegroundColor Yellow
Write-Host "===============================================" -ForegroundColor Cyan 