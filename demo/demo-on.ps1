# Activate the DEMO projects (Porter + ASTRID) in Taurus.
# Backs up your real projects.json once, then writes a demo config whose paths
# are derived from THIS folder (portable). The demo agents run a fake Claude
# (mock-claude.mjs) so no real data or MCP is ever touched.
# Note: keep the clone path free of spaces for the mock command to launch.
$ErrorActionPreference = "Stop"
$cfgDir = Join-Path $env:APPDATA "Taurus"
New-Item -ItemType Directory -Force $cfgDir | Out-Null
$live   = Join-Path $cfgDir "projects.json"
$backup = Join-Path $cfgDir "projects.real.json"
if ((Test-Path $live) -and -not (Test-Path $backup)) {
    Copy-Item $live $backup -Force
    Write-Host "Backed up your real config -> $backup"
}

$root = $PSScriptRoot
$mock = Join-Path $root "mock-claude.mjs"
$projects = @(
    [ordered]@{ id = "porter"; label = "Porter"; path = (Join-Path $root "Porter"); title = "Porter"; task = ""; accent = "#1144df"; mode = "default"; command = "node $mock" }
    [ordered]@{ id = "astrid"; label = "ASTRID"; path = (Join-Path $root "ASTRID"); title = "ASTRID"; task = ""; accent = "#d57aff"; mode = "plan"; command = "node $mock" }
)
$projects | ConvertTo-Json -Depth 5 | Set-Content -Path $live -Encoding UTF8
Write-Host "DEMO active (Porter + ASTRID). In Taurus: click 'Reload' (or restart) to see them."
