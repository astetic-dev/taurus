# Start a TEST instance of Taurus next to your running Taurus.
#
# Why this script exists: otherwise both instances read the same
# %APPDATA%\Taurus, and the second one resumes your RUNNING sessions and then
# overwrites them as well. TAURUS_CONFIG_DIR moves the whole config folder, so the
# test instance has its own projects/hosts/sessions/peers.
#
# The window is titled "... TEST" so you can tell them apart.
#
# -Exe points at another build. Needed once this test instance is running itself: it
# keeps target\release\taurus.exe locked, so a new build has to go to a separate
# folder (cargo build --release --target-dir target\fixbuild). This way you can start
# that one without copying it first.
param(
    [string]$Exe
)
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$cfg  = Join-Path $env:APPDATA 'Taurus-TEST'

# Without -Exe: take the NEWEST of the known build locations, not blindly
# target\release. Once this test instance runs it keeps that exe locked, so the next
# build goes to target\fixbuild - and this script then silently started the OLD
# build again. That cost a round of finding out why a change was missing.
# Include every build folder under target\, not a fixed list: once a test instance
# runs its exe is locked and the next build goes to yet another folder. With a glob
# this script doesn't need to know about that.
$candidates = @(
    (Join-Path $root 'src-tauri\target\release\taurus.exe'),
    (Join-Path $root 'src-tauri\target\*\release\taurus.exe')
)
if ($Exe) {
    $exe = $Exe
} else {
    $found = @(Get-Item $candidates -EA 0 | Sort-Object LastWriteTime -Descending)
    if ($found.Count -eq 0) {
        Write-Host "No build found. First run in src-tauri: cargo build --release"
        return
    }
    $exe = $found[0].FullName
    if ($found.Count -gt 1) {
        Write-Host "Several builds found; picked the newest:"
        foreach ($f in $found) {
            Write-Host ("  {0}  {1}  {2}" -f $f.LastWriteTime.ToString('yyyy-MM-dd HH:mm'), $f.VersionInfo.FileVersion, $f.FullName)
        }
    }
}

if (-not (Test-Path $exe)) {
    Write-Host "No build found at $exe - first run: cargo build --release (in src-tauri)"
    return
}

# Two test instances would otherwise share the same Taurus-TEST folder, and then do
# to each other exactly what this script prevents between test and real.
$al = @(Get-Process taurus -EA 0 | Where-Object { $_.Path -and $_.Path -like "$root*" })
if ($al.Count -gt 0) {
    Write-Host "A test instance is already running (pid $($al.Id -join ', ')). Close it first - both would use $cfg."
    return
}
New-Item -ItemType Directory -Force -Path $cfg | Out-Null

# Safety net: if this ever points at the real folder, stop. A test instance that
# takes over your real sessions is exactly what we prevent here.
if ($cfg -ieq (Join-Path $env:APPDATA 'Taurus')) {
    throw "TAURUS_CONFIG_DIR points at the REAL config folder - stopped."
}

# Show the version, because "which build is this, actually" is exactly the question
# you ask when a change seems to be missing.
$ver = (Get-Item $exe).VersionInfo.FileVersion
Write-Host "Config    : $cfg"
Write-Host "Binary    : $exe"
Write-Host "Version   : $ver  ($((Get-Item $exe).LastWriteTime.ToString('yyyy-MM-dd HH:mm')))"
Write-Host "Sessions  : $(if (Test-Path (Join-Path $cfg 'sessions.json')) { 'own sessions.json present' } else { 'none - starts empty, leaves your real sessions alone' })"

$env:TAURUS_CONFIG_DIR = $cfg
Start-Process $exe
Write-Host "Started. The title bar says 'TEST'."
