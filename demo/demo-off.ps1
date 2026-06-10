# Restore your real Taurus projects after the demo.
$ErrorActionPreference = "Stop"
$cfgDir = Join-Path $env:APPDATA "Taurus"
$live   = Join-Path $cfgDir "projects.json"
$backup = Join-Path $cfgDir "projects.real.json"
if (Test-Path $backup) {
    Copy-Item $backup $live -Force
    Remove-Item $backup -Force
    Write-Host "Real projects restored. Click 'Reload' in Taurus."
} else {
    Write-Host "No backup (projects.real.json) found. Nothing restored."
}
