# 🦊 Rubah RSS Reader - Official Windows PowerShell Installer

$ErrorActionPreference = "Stop"

# Enable TLS 1.2 / TLS 1.3 protocols in PowerShell 5.1 & 7
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
} catch {}

Write-Host ""
Write-Host "  🦊 Rubah - Ruang Baca Harian (Windows Installer)" -ForegroundColor Yellow
Write-Host "  =================================================" -ForegroundColor DarkGray

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ExePath = Join-Path $InstallDir "baca.exe"
$PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v0.3.1/rubah-windows-amd64.exe"
$LatestUrl = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

Write-Host "--> Mengunduh binary 'baca.exe'..." -ForegroundColor Cyan

$downloadSuccess = $false

# 1. Try built-in curl.exe (built-in on Windows 10 & 11)
if (Get-Command "curl.exe" -ErrorAction SilentlyContinue) {
    try {
        & curl.exe -sL -o $ExePath $PrimaryUrl
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

# 2. Try Invoke-WebRequest on direct v0.3.1 URL
if (-not $downloadSuccess) {
    try {
        Invoke-WebRequest -Uri $PrimaryUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

# 3. Try Invoke-WebRequest on latest URL
if (-not $downloadSuccess) {
    try {
        Invoke-WebRequest -Uri $LatestUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 1000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    Write-Host "--> Gagal mengunduh binary 'baca.exe'. Silakan unduh manual dari GitHub Releases." -ForegroundColor Red
    exit 1
}

# Add InstallDir to User Environment PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
}

Write-Host ""
Write-Host "  ===========================================================" -ForegroundColor Green
Write-Host "  🎉 Instalasi Rubah Berhasil Selesai!" -ForegroundColor Green
Write-Host "  ===========================================================" -ForegroundColor Green
Write-Host ""
Write-Host "Jalankan aplikasi di PowerShell atau CMD dengan mengetik:" -ForegroundColor Yellow
Write-Host "  baca" -ForegroundColor White
Write-Host ""
