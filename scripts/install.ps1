# ==============================================================================
# Oride — One-Line Windows PowerShell Installer
#
# Usage:
#   irm https://raw.githubusercontent.com/ori-team/oride/main/scripts/install.ps1 | iex
#
# Parameters (can be passed via environment variables):
#   $env:ORIDE_VERSION      - Specific version to install (default: "v0.2.0")
#   $env:ORIDE_INSTALL_DIR  - Custom install directory (default: "$HOME\.oride\bin")
# ==============================================================================

$ErrorActionPreference = "Stop"

$Repo = "ori-team/oride"
$DefaultVersion = "v0.2.0"
$InstallDir = if ($env:ORIDE_INSTALL_DIR) { $env:ORIDE_INSTALL_DIR } else { "$HOME\.oride\bin" }

function Write-Banner {
    Write-Host ""
    Write-Host "  ╭────────────────────────────────────────╮" -ForegroundColor Cyan
    Write-Host "  │     Oride — Terminal Code Editor       │" -ForegroundColor Cyan
    Write-Host "  │   Windows One-Line Installer (v0.2.0)  │" -ForegroundColor Cyan
    Write-Host "  ╰────────────────────────────────────────╯" -ForegroundColor Cyan
    Write-Host ""
}

function Resolve-Arch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        "x86"   { return "i686" }
        default {
            Write-Warning "Unrecognized architecture '$arch', defaulting to x86_64."
            return "x86_64"
        }
    }
}

function Resolve-Version {
    if ($env:ORIDE_VERSION) {
        return $env:ORIDE_VERSION
    }
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing -TimeoutSec 5
        if ($release.tag_name) {
            return $release.tag_name
        }
    } catch {
        # Fallback to default version if API is rate-limited or offline
    }
    return $DefaultVersion
}

function Add-ToPath ($Directory) {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -split ";" -contains $Directory) {
        return
    }

    Write-Host "==> Adding '$Directory' to User PATH environment variable..." -ForegroundColor Yellow
    $newPath = "$userPath;$Directory"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = "$env:Path;$Directory"
    Write-Host "==> PATH updated successfully!" -ForegroundColor Green
}

Write-Banner

$arch = Resolve-Arch
$version = Resolve-Version
if (-not ($version.StartsWith("v"))) {
    $version = "v$version"
}

$target = "$arch-pc-windows-msvc"
$zipName = "oride-$version-$target.zip"
$downloadUrl = "https://github.com/$Repo/releases/download/$version/$zipName"

Write-Host "==> Architecture: $arch ($target)" -ForegroundColor Cyan
Write-Host "==> Target Version: $version" -ForegroundColor Cyan
Write-Host "==> Install Location: $InstallDir" -ForegroundColor Cyan

$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
$zipPath = Join-Path $tempDir $zipName

try {
    Write-Host "==> Downloading $zipName from GitHub Releases..." -ForegroundColor Green
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing

    Write-Host "==> Extracting files..." -ForegroundColor Green
    Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null

    $exePath = Join-Path $tempDir "oride.exe"
    if (Test-Path $exePath) {
        Copy-Item -Path $exePath -Destination $InstallDir -Force
    } else {
        # Search recursively inside extracted folder
        $found = Get-ChildItem -Path $tempDir -Filter "oride.exe" -Recurse | Select-Object -First 1
        if ($found) {
            Copy-Item -Path $found.FullName -Destination $InstallDir -Force
        } else {
            throw "oride.exe was not found inside the downloaded archive."
        }
    }

    Add-ToPath $InstallDir

    Write-Host ""
    Write-Host "==> Oride installed successfully to: $(Join-Path $InstallDir 'oride.exe')" -ForegroundColor Green
    
    $installedExe = Join-Path $InstallDir "oride.exe"
    if (Test-Path $installedExe) {
        $verOutput = & $installedExe --version
        Write-Host "    Installed: $verOutput" -ForegroundColor Cyan
    }

    Write-Host ""
    Write-Host "  Getting Started:" -ForegroundColor White
    Write-Host "    oride .             # Open current directory in Oride"
    Write-Host "    oride main.rs       # Open specific file"
    Write-Host "    oride --help        # View commands and options"
    Write-Host ""

} catch {
    Write-Error "Failed to install Oride: $_"
    exit 1
} finally {
    if (Test-Path $tempDir) {
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
