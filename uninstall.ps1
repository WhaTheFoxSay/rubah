# 🦊 Rubah - Windows Uninstaller (English Default & Indonesian Support)

param ([string]$Lang = "en")

$ErrorActionPreference = "Stop"

function Step {
    param ([string]$Label, [string]$Detail)
    Write-Host "  " -NoNewline
    Write-Host "✔ " -ForegroundColor Green -NoNewline
    Write-Host ("{0,-25}" -f $Label) -NoNewline
    Write-Host " $Detail" -ForegroundColor DarkGray
}

if ($Lang.ToLower() -eq "id" -or $env:LANG -like "id*") {
    $SubTitle = "Ruang Baca Harian"
    $BinDetail = "$InstallDir terhapus"
    $CfgDetail = "$ConfigDir terhapus"
    $PathLabel = "PATH Environment"
    $PathDetail = "User PATH reset"
    $DoneMsg = "Aplikasi Rubah berhasil di-uninstall dari sistem Anda."
    $ThanksMsg = "Terima kasih telah menggunakan Rubah [Ruang Baca Harian]."
    $ByeMsg = "Sampai jumpa kembali! 🦊"
} else {
    $SubTitle = "Daily Reading Space"
    $BinDetail = "$InstallDir deleted"
    $CfgDetail = "$ConfigDir deleted"
    $PathLabel = "PATH Environment"
    $PathDetail = "User PATH reset"
    $DoneMsg = "Rubah application successfully uninstalled from your system."
    $ThanksMsg = "Thank you for using Rubah [Daily Reading Space]."
    $ByeMsg = "See you again! 🦊"
}

Write-Host ""
Write-Host "  🦊 RUBAH " -ForegroundColor Yellow -NoNewline
Write-Host "[$SubTitle] Uninstaller" -ForegroundColor White
Write-Host "  High-Performance RSS Feed Reader TUI" -ForegroundColor DarkGray
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
$ConfigDir = "$env:APPDATA\rubah"
$LocalConfigDir = "$env:LOCALAPPDATA\rubah"

if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir | Out-Null
}
Step "Binary & program files" $BinDetail

if (Test-Path $ConfigDir) {
    Remove-Item -Recurse -Force $ConfigDir | Out-Null
}
if (Test-Path $LocalConfigDir) {
    Remove-Item -Recurse -Force $LocalConfigDir | Out-Null
}
Step "Config & database" $CfgDetail

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}
Step $PathLabel $PathDetail

Write-Host ""
Write-Host "  ✔ $DoneMsg" -ForegroundColor Green
Write-Host "  $ThanksMsg" -ForegroundColor White
Write-Host "  $ByeMsg" -ForegroundColor Yellow
Write-Host ""
