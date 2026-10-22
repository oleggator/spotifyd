#Requires -RunAsAdministrator
# Stops and removes the spotifyd Windows service. The config and cache stay on disk.

$ErrorActionPreference = 'Stop'

Stop-Service spotifyd
# Remove-Service is missing in Windows PowerShell 5.1, so use sc.exe.
sc.exe delete spotifyd
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
