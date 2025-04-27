# PowerShell script for deploying to Render
Write-Host "Preparing deployment to Render.com..." -ForegroundColor Green

# Check for render.yaml
if (-Not (Test-Path -Path "chat-backend/render.yaml")) {
    Write-Host "render.yaml not found in chat-backend directory. Creating it..." -ForegroundColor Yellow
    
    # Create render.yaml content
    $renderYaml = @"
services:
  - type: web
    name: mismatch-backend
    env: rust
    buildCommand: cargo build --release
    startCommand: ./target/release/chat-backend
    envVars:
      - key: PORT
        value: 10000
      - key: RUST_LOG
        value: info
"@
    
    # Write to file
    $renderYaml | Out-File -FilePath "chat-backend/render.yaml" -Encoding utf8
    Write-Host "render.yaml created successfully." -ForegroundColor Green
}

Write-Host "To deploy to Render.com:" -ForegroundColor Green
Write-Host "1. Go to https://dashboard.render.com/select-repo?type=web" -ForegroundColor Cyan
Write-Host "2. Connect your GitHub repository" -ForegroundColor Cyan
Write-Host "3. Select the 'chat-backend' directory as the root directory" -ForegroundColor Cyan
Write-Host "4. Configure as:" -ForegroundColor Cyan
Write-Host "   - Environment: Rust" -ForegroundColor Cyan
Write-Host "   - Build Command: cargo build --release" -ForegroundColor Cyan
Write-Host "   - Start Command: ./target/release/chat-backend" -ForegroundColor Cyan
Write-Host "5. Add environment variables for your database connection" -ForegroundColor Cyan

Write-Host "`nFor the frontend, you can deploy to:" -ForegroundColor Green
Write-Host "1. Render.com as a Static Site" -ForegroundColor Cyan
Write-Host "2. Netlify (https://netlify.com)" -ForegroundColor Cyan
Write-Host "3. Vercel (https://vercel.com)" -ForegroundColor Cyan
Write-Host "4. GitHub Pages" -ForegroundColor Cyan

Write-Host "`nDon't forget to set the WebSocket URL in your frontend:" -ForegroundColor Yellow
Write-Host "REACT_APP_WS_URL=wss://your-backend-url/ws" -ForegroundColor Cyan 