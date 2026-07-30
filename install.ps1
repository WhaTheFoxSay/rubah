# 🦊 Rubah RSS Reader - Official Windows PowerShell Installer

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "  🦊 Rubah - Ruang Baca Harian (Windows Installer)" -ForegroundColor Yellow
Write-Host "  =================================================" -ForegroundColor DarkGray

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ExePath = Join-Path $InstallDir "baca.exe"
$Url = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

Write-Host "--> Mengunduh binary 'baca.exe'..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $Url -OutFile $ExePath -UseBasicParsing

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
