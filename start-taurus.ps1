# Start Taurus - Agent Launcher
# Draait de gebouwde .exe als die bestaat, anders dev-mode.
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$exe  = Join-Path $root 'src-tauri\target\release\Taurus.exe'

if (Test-Path $exe) {
    Start-Process $exe
    return
}

# Dev-mode: zorg dat cargo in PATH staat
$cargoBin = Join-Path $env:USERPROFILE '.cargo\bin'
if (Test-Path $cargoBin) { $env:Path = "$cargoBin;$env:Path" }

Push-Location $root
try {
    npm run tauri dev
} finally {
    Pop-Location
}
