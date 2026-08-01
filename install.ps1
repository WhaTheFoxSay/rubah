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
$PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v0.9.0/rubah-windows-amd64.exe"
$LatestUrl = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

function Draw-Progress {
    param ([int]$Percent, [string]$StepName)
    $Width = 24
    $Filled = [math]::Floor($Percent * $Width / 100)
    $Empty = $Width - $Filled
    $Bar = ("█" * $Filled) + ("░" * $Empty)
    Write-Host -NoNewline "`r  [$Bar] $Percent% | $StepName"
}

function Show-Step {
    param ([string]$StepName)
    Write-Host "  [✔] $StepName" -ForegroundColor Green
}

$Version = "0.9.0"

Draw-Progress 10 "Menyiapkan direktori sistem..."
Start-Sleep -Milliseconds 100
Show-Step "Menyiapkan direktori sistem ($InstallDir)..."

Draw-Progress 30 "Mengunduh binary 'baca.exe' v$Version..."
$downloadSuccess = $false

if (Get-Command "curl.exe" -ErrorAction SilentlyContinue) {
    try {
        & curl.exe -sL -o $ExePath $PrimaryUrl
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 3000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $PrimaryUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 3000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    try {
        $ApiUrl = "https://api.github.com/repos/WhaTheFoxSay/rubah/releases/tags/v$Version"
        $ReleaseInfo = Invoke-RestMethod -Uri $ApiUrl -UserAgent "RubahInstaller/1.0"
        $Asset = $ReleaseInfo.assets | Where-Object { $_.name -eq "rubah-windows-amd64.exe" }
        if ($Asset -and $Asset.url) {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $Asset.url -Headers @{ "Accept" = "application/octet-stream" } -OutFile $ExePath -UseBasicParsing -UserAgent "RubahInstaller/1.0"
            if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 3000000)) {
                $downloadSuccess = $true
            }
        }
    } catch {}
}

if (-not $downloadSuccess) {
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $LatestUrl -OutFile $ExePath -UseBasicParsing -MaximumRedirection 10
        if ((Test-Path $ExePath) -and ((Get-Item $ExePath).Length -gt 3000000)) {
            $downloadSuccess = $true
        }
    } catch {}
}

if (-not $downloadSuccess) {
    Write-Host ""
    Write-Host "Error: Gagal mengunduh 'baca.exe'. Silakan periksa koneksi internet Anda." -ForegroundColor Red
    exit 1
}

Draw-Progress 70 "Memverifikasi binary & environment..."
Show-Step "Mengunduh binary 'baca.exe' v$Version..."

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
}

Draw-Progress 100 "Instalasi selesai!"
Start-Sleep -Milliseconds 100
Show-Step "Mengatur PATH Environment User..."
Write-Host ""

Write-Host "[✔] Rubah v$Version berhasil terinstall di sistem Anda!" -ForegroundColor Green
Write-Host "Jalankan aplikasi di PowerShell atau CMD dengan mengetik:" -ForegroundColor White
Write-Host "  baca" -ForegroundColor Yellow
Write-Host ""
