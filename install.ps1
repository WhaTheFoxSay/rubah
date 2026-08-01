# 🦊 Rubah - Windows Installer (English Default & Indonesian Support)

param ([string]$Lang = "en")

$ErrorActionPreference = "Stop"

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
} catch {}

function Step {
    param ([string]$Label, [string]$Detail)
    Write-Host "  " -NoNewline
    Write-Host "✔ " -ForegroundColor Green -NoNewline
    Write-Host ("{0,-25}" -f $Label) -NoNewline
    Write-Host " $Detail" -ForegroundColor DarkGray
}

try {
    $ReleaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/WhaTheFoxSay/rubah/releases/latest" -UserAgent "RubahInstaller/1.0"
    $LatestTag = $ReleaseInfo.tag_name
    if ($LatestTag -and $LatestTag.StartsWith("v")) {
        $Version = $LatestTag.Substring(1)
    } else {
        $Version = "1.3.0"
    }
} catch {
    $Version = "1.3.0"
}

if ($Lang.ToLower() -eq "id" -or $env:LANG -like "id*") {
    $SubTitle = "Ruang Baca Harian"
    $LabelSys = "Lingkungan sistem"
    $LabelDl = "Unduh executable"
    $LabelInst = "Pasang biner & PATH"
    $MsgSuccess = "Rubah v$Version berhasil terinstall!"
    $MsgRun = "Jalankan aplikasi di PowerShell atau CMD dengan mengetik:"
} else {
    $SubTitle = "Daily Reading Space"
    $LabelSys = "System environment"
    $LabelDl = "Download executable"
    $LabelInst = "Install binary & PATH"
    $MsgSuccess = "Rubah v$Version successfully installed!"
    $MsgRun = "Run the application in PowerShell or CMD by typing:"
}

Write-Host ""
Write-Host "  🦊 RUBAH " -ForegroundColor Yellow -NoNewline
Write-Host "[$SubTitle] " -ForegroundColor White -NoNewline
Write-Host "v$Version" -ForegroundColor DarkGray
Write-Host "  High-Performance RSS Feed Reader TUI" -ForegroundColor DarkGray
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Programs\Rubah"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ExePath = Join-Path $InstallDir "baca.exe"
$PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v$Version/rubah-windows-amd64.exe"
$LatestUrl = "https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe"

Step $LabelSys "windows (amd64)"

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
    Write-Host "  Error: Failed to download 'baca.exe'. Please check your internet connection." -ForegroundColor Red
    exit 1
}

$SizeMB = [math]::Round((Get-Item $ExePath).Length / 1MB, 1)
Step $LabelDl "v$Version (${SizeMB} MB)"

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
}

Step $LabelInst "$ExePath"

Write-Host ""
Write-Host "  ✔ $MsgSuccess" -ForegroundColor Green
Write-Host "  $MsgRun" -ForegroundColor White
Write-Host "    baca" -ForegroundColor Yellow
Write-Host ""
