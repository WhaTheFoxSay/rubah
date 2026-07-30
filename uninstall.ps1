# 🦊 Rubah [Ruang Baca Harian] - Windows Uninstaller

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "--> 🦊 Rubah [Ruang Baca Harian] Uninstaller" -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

Write-Host "--> Menghapus file program..." -ForegroundColor Yellow
if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
}

Write-Host "--> Menghapus data konfigurasi & database..." -ForegroundColor Yellow
if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
}
if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
}

Write-Host "--> Menghapus PATH environment..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}

Write-Host "--> Uninstall berhasil selesai." -ForegroundColor Green
Write-Host "Terima kasih telah menggunakan Rubah [Ruang Baca Harian]." -ForegroundColor White
Write-Host "Sampai jumpa kembali! 🦊" -ForegroundColor Cyan
Write-Host ""
