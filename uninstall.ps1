# 🦊 Rubah RSS Reader - Official Windows Uninstall Wizard

$ErrorActionPreference = "Stop"

Clear-Host
Write-Host ""
Write-Host "  ┌────────────────────────────────────────────────────────┐" -ForegroundColor Cyan
Write-Host "  │ 🦊  RUBAH RSS READER - UNINSTALL WIZARD (Windows)     │" -ForegroundColor Cyan
Write-Host "  └────────────────────────────────────────────────────────┘" -ForegroundColor Cyan
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

Write-Host "[1/3] 🗑️  Removing program files..." -ForegroundColor Yellow
if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
    Write-Host "      --> Removed $InstallDir" -ForegroundColor DarkGray
}

Write-Host "[2/3] 📂 Cleaning local configuration & database storage..." -ForegroundColor Yellow
if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
    Write-Host "      --> Purged $ConfigDir" -ForegroundColor DarkGray
}
if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
    Write-Host "      --> Purged $LocalConfigDir" -ForegroundColor DarkGray
}

Write-Host "[3/3] 🧹 Purging User Environment PATH entries..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "      --> Removed $InstallDir from User PATH" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host " ════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "  👋 UNINSTALL COMPLETED SUCCESSFULLY!" -ForegroundColor Green
Write-Host " ════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "Thank you for using Rubah RSS Reader!" -ForegroundColor White
Write-Host "We hope to see you again soon. 🦊✨" -ForegroundColor Cyan
Write-Host ""
