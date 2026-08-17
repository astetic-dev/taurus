# Start een TESTexemplaar van Taurus naast je draaiende Taurus.
#
# Waarom dit script bestaat: beide exemplaren lezen anders dezelfde
# %APPDATA%\Taurus, en dan hervat de tweede je LOPENDE sessies en overschrijft
# hij ze daarna ook nog. TAURUS_CONFIG_DIR verlegt de hele configmap, zodat het
# testexemplaar zijn eigen projects/hosts/sessions/peers heeft.
#
# Het venster heet "... TEST" zodat je ze uit elkaar houdt.
#
# -Exe wijst naar een andere build. Nodig zodra dit testexemplaar zelf draait: het
# houdt target\release\taurus.exe vergrendeld, dus een nieuwe build moet dan naar
# een aparte map (cargo build --release --target-dir target\fixbuild). Zo kun je die
# starten zonder eerst te kopieren.
param(
    [string]$Exe
)
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$cfg  = Join-Path $env:APPDATA 'Taurus-TEST'

# Zonder -Exe: pak de NIEUWSTE van de bekende buildplekken, niet blind
# target\release. Zodra dit testexemplaar draait houdt het die exe vergrendeld, dus
# gaat een volgende build naar target\fixbuild - en dan startte dit script daarna
# stil de OUDE build weer. Dat kostte een ronde uitzoeken waarom een wijziging er
# niet in zat.
$candidates = @(
    (Join-Path $root 'src-tauri\target\release\taurus.exe'),
    (Join-Path $root 'src-tauri\target\fixbuild\release\taurus.exe')
)
if ($Exe) {
    $exe = $Exe
} else {
    $found = @(Get-Item $candidates -EA 0 | Sort-Object LastWriteTime -Descending)
    if ($found.Count -eq 0) {
        Write-Host "Geen build gevonden. Draai eerst in src-tauri: cargo build --release"
        return
    }
    $exe = $found[0].FullName
    if ($found.Count -gt 1) {
        Write-Host "Meerdere builds gevonden; de nieuwste gekozen:"
        foreach ($f in $found) {
            Write-Host ("  {0}  {1}  {2}" -f $f.LastWriteTime.ToString('yyyy-MM-dd HH:mm'), $f.VersionInfo.FileVersion, $f.FullName)
        }
    }
}

if (-not (Test-Path $exe)) {
    Write-Host "Geen build gevonden op $exe - draai eerst: cargo build --release (in src-tauri)"
    return
}

# Twee testexemplaren delen anders dezelfde Taurus-TEST-map, en dan doen ze elkaar
# precies aan wat dit script voorkomt tussen test en echt.
$al = @(Get-Process taurus -EA 0 | Where-Object { $_.Path -and $_.Path -like "$root*" })
if ($al.Count -gt 0) {
    Write-Host "Er draait al een testexemplaar (pid $($al.Id -join ', ')). Sluit dat eerst - beide zouden $cfg gebruiken."
    return
}
New-Item -ItemType Directory -Force -Path $cfg | Out-Null

# Vangnet: als dit ooit naar de echte map wijst, stop. Een testexemplaar dat je
# echte sessies overneemt is precies wat we hier voorkomen.
if ($cfg -ieq (Join-Path $env:APPDATA 'Taurus')) {
    throw "TAURUS_CONFIG_DIR wijst naar de ECHTE configmap - gestopt."
}

# De versie erbij, want "welke build draait hier eigenlijk" is precies de vraag die
# je je stelt als een wijziging er niet in lijkt te zitten.
$ver = (Get-Item $exe).VersionInfo.FileVersion
Write-Host "Configmap : $cfg"
Write-Host "Binary    : $exe"
Write-Host "Versie    : $ver  ($((Get-Item $exe).LastWriteTime.ToString('yyyy-MM-dd HH:mm')))"
Write-Host "Sessies   : $(if (Test-Path (Join-Path $cfg 'sessions.json')) { 'eigen sessions.json aanwezig' } else { 'geen - start leeg, raakt je echte sessies niet aan' })"

$env:TAURUS_CONFIG_DIR = $cfg
Start-Process $exe
Write-Host "Gestart. Titelbalk zegt 'TEST'."
