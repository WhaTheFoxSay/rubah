# 🦊 Rubah RSS Reader - Official Windows PowerShell Uninstaller

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "  🗑️ Rubah - Uninstaller (Windows)" -ForegroundColor Yellow
Write-Host "  =================================================" -ForegroundColor DarkGray

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
    Write-Host "--> Menghapus folder program: $InstallDir" -ForegroundColor Cyan
}

if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
    Write-Host "--> Menghapus data konfigurasi: $ConfigDir" -ForegroundColor Cyan
}

if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
    Write-Host "--> Menghapus data lokal: $LocalConfigDir" -ForegroundColor Cyan
}

# Remove InstallDir from User Environment PATH
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "--> Menghapus $InstallDir dari User PATH" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "  ===========================================================" -ForegroundColor Green
Write-Host "  ✅ Aplikasi Rubah dan seluruh datanya berhasil di-uninstall!" -ForegroundColor Green
Write-Host "  ===========================================================" -ForegroundColor Green
Write-Host ""
