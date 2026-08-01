# 🦊 Rubah [Ruang Baca Harian] - Windows Uninstaller

$ErrorActionPreference = "Stop"

function Step {
    param ([string]$Label, [string]$Detail)
    Write-Host "  " -NoNewline
    Write-Host "✔ " -ForegroundColor Green -NoNewline
    Write-Host ("{0,-25}" -f $Label) -NoNewline
    Write-Host " $Detail" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "  🦊 RUBAH " -ForegroundColor Yellow -NoNewline
Write-Host "[Ruang Baca Harian] Uninstaller" -ForegroundColor White
Write-Host "  High-Performance RSS Feed Reader TUI" -ForegroundColor DarkGray
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
}
Step "Binary & program files" "$InstallDir terhapus"

if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
}
if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
}
Step "Config & database" "$ConfigDir terhapus"

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}
Step "PATH Environment" "User PATH reset"

Write-Host ""
Write-Host "  ✔ Aplikasi Rubah berhasil di-uninstall dari sistem Anda." -ForegroundColor Green
Write-Host "  Terima kasih telah menggunakan Rubah [Ruang Baca Harian]." -ForegroundColor White
Write-Host "  Sampai jumpa kembali! 🦊" -ForegroundColor Yellow
Write-Host ""
