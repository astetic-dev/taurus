# Start een TESTexemplaar van Taurus naast je draaiende Taurus.
#
# Waarom dit script bestaat: beide exemplaren lezen anders dezelfde
# %APPDATA%\Taurus, en dan hervat de tweede je LOPENDE sessies en overschrijft
# hij ze daarna ook nog. TAURUS_CONFIG_DIR verlegt de hele configmap, zodat het
# testexemplaar zijn eigen projects/hosts/sessions/peers heeft.
#
# Het venster heet "... TEST" zodat je ze uit elkaar houdt.
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$exe  = Join-Path $root 'src-tauri\target\release\taurus.exe'
$cfg  = Join-Path $env:APPDATA 'Taurus-TEST'

if (-not (Test-Path $exe)) {
    Write-Host "Geen build gevonden op $exe - draai eerst: cargo build --release (in src-tauri)"
    return
}
New-Item -ItemType Directory -Force -Path $cfg | Out-Null

# Vangnet: als dit ooit naar de echte map wijst, stop. Een testexemplaar dat je
# echte sessies overneemt is precies wat we hier voorkomen.
if ($cfg -ieq (Join-Path $env:APPDATA 'Taurus')) {
    throw "TAURUS_CONFIG_DIR wijst naar de ECHTE configmap - gestopt."
}

Write-Host "Configmap : $cfg"
Write-Host "Binary    : $exe"
Write-Host "Sessies   : $(if (Test-Path (Join-Path $cfg 'sessions.json')) { 'eigen sessions.json aanwezig' } else { 'geen - start leeg, raakt je echte sessies niet aan' })"

$env:TAURUS_CONFIG_DIR = $cfg
Start-Process $exe
Write-Host "Gestart. Titelbalk zegt 'TEST'."
