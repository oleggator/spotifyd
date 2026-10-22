#Requires -RunAsAdministrator
# Registers spotifyd as a Windows service that starts on boot, then starts it.
param(
    [string]$BinaryPath = (Join-Path $PSScriptRoot '..\..\target\release\spotifyd.exe'),
    [string]$ConfigPath = (Join-Path $env:ProgramData 'spotifyd\spotifyd.conf'),
    [string]$CachePath = (Join-Path $env:ProgramData 'spotifyd\cache')
)

$ErrorActionPreference = 'Stop'

$BinaryPath = (Resolve-Path $BinaryPath).Path
New-Item -ItemType Directory -Force -Path $CachePath | Out-Null

if (-not (Test-Path $ConfigPath)) {
    Write-Warning "No config file at $ConfigPath, spotifyd will use default values."
}

New-Service -Name spotifyd `
    -DisplayName 'spotifyd' `
    -Description 'A Spotify Connect daemon' `
    -BinaryPathName "`"$BinaryPath`" --windows-service --config-path `"$ConfigPath`" --cache-path `"$CachePath`"" `
    -StartupType Automatic | Out-Null

# Windows shows no firewall prompt for a service, so allow Spotify Connect discovery explicitly.
New-NetFirewallRule -DisplayName 'spotifyd' -Direction Inbound -Action Allow `
    -Program $BinaryPath -RemoteAddress LocalSubnet | Out-Null

Start-Service spotifyd
Get-Service spotifyd
