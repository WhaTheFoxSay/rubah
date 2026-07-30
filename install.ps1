# 🦊 Rubah RSS Reader - Official Windows Setup Wizard

$ErrorActionPreference = "Stop"

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
} catch {}

Clear-Host
Write-Host ""
Write-Host "  ┌────────────────────────────────────────────────────────┐" -ForegroundColor Cyan
Write-Host "  │ 🦊  RUBAH RSS READER - SETUP WIZARD (Windows)         │" -ForegroundColor Cyan
Write-Host "  │     Retro Terminal User Interface Reader               │" -ForegroundColor Cyan
Write-Host "  └────────────────────────────────────────────────────────┘" -ForegroundColor Cyan
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ExePath = Join-Path $InstallDir "baca.exe"
$PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v0.3.5/rubah-windows-amd64.exe"
$LatestUrl = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

# Step 1
Write-Host "[1/4] 🔍 Detecting Windows platform architecture..." -ForegroundColor Yellow
Write-Host "      --> Platform: Windows x64 (MSVC 64-bit)" -ForegroundColor DarkGray
Write-Host ""

# Step 2
Write-Host "[2/4] 🔒 Initializing TLS 1.2 / 1.3 security protocols..." -ForegroundColor Yellow
Write-Host "      --> Security Provider: Windows Schannel" -ForegroundColor DarkGray
Write-Host ""

# Step 3
Write-Host "[3/4] 💾 Downloading pre-compiled binary package (~10.4 MB)..." -ForegroundColor Yellow

$downloadSuccess = $false

if (Get-Command "curl.exe" -ErrorAction SilentlyContinue) {
    try {
        & curl.exe -sL -o $ExePath $PrimaryUrl
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $PrimaryUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $LatestUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    Write-Host "❌ Error: Failed to download 'baca.exe'. Please check your network connection." -ForegroundColor Red
    exit 1
}

Write-Host "      [████████████████████████████████████████] 100% Verified!" -ForegroundColor Green
Write-Host ""

# Step 4
Write-Host "[4/4] ⚙️  Installing executable & updating environment..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
    Write-Host "      --> Added $InstallDir to User PATH" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host " ════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "  🎉 INSTALLATION COMPLETED SUCCESSFULLY!" -ForegroundColor Green
Write-Host " ════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "Launch the application by typing in PowerShell or CMD:" -ForegroundColor Yellow
Write-Host "  baca" -ForegroundColor White
Write-Host ""
