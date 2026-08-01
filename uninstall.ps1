# 🦊 Rubah [Ruang Baca Harian] - Windows Uninstaller

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "--> 🦊 Rubah [Ruang Baca Harian] Uninstaller" -ForegroundColor Cyan

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

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

Draw-Progress 25 "Menghapus file program & binary executable..."
if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
}
Start-Sleep -Milliseconds 100
Show-Step "Menghapus file program & binary executable..."

Draw-Progress 60 "Menghapus data konfigurasi & database..."
if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
}
if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
}
Start-Sleep -Milliseconds 100
Show-Step "Menghapus data konfigurasi, cache & database..."

Draw-Progress 90 "Membersihkan PATH Environment..."
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}

Draw-Progress 100 "Uninstall selesai!"
Start-Sleep -Milliseconds 100
Show-Step "Membersihkan PATH Environment..."
Write-Host ""

Write-Host "[✔] Aplikasi Rubah berhasil di-uninstall dari sistem Anda." -ForegroundColor Green
Write-Host "Terima kasih telah menggunakan Rubah [Ruang Baca Harian]." -ForegroundColor White
Write-Host "Sampai jumpa kembali! 🦊" -ForegroundColor Cyan
Write-Host ""
