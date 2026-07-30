# 🦊 Rubah [Ruang Baca Harian] - Windows Installer

$ErrorActionPreference = "Stop"

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
} catch {}

Write-Host ""
Write-Host "--> 🦊 Rubah [Ruang Baca Harian]" -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ExePath = Join-Path $InstallDir "baca.exe"
$PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v0.3.7/rubah-windows-amd64.exe"
$LatestUrl = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

Write-Host "--> Mengunduh binary 'baca.exe'..." -ForegroundColor Yellow

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
    Write-Host "Error: Gagal mengunduh 'baca.exe'. Silakan periksa koneksi internet Anda." -ForegroundColor Red
    exit 1
}

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
}

Write-Host "--> Instalasi selesai!" -ForegroundColor Green
Write-Host ""
Write-Host "Jalankan aplikasi di PowerShell atau CMD dengan mengetik:" -ForegroundColor Yellow
Write-Host "  baca" -ForegroundColor White
Write-Host ""
