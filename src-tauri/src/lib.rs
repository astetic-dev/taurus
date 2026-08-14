// Taurus - Agent Launcher
// Draait Claude Code-agents als ingebedde terminals (ConPTY) MET tabs in het
// eigen venster. Elke sessie = een pseudo-terminal die `claude -n <titel>` draait
// in de juiste map (lokaal C: of netwerk X:), zodat je nooit hoeft te twijfelen.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use tauri::{AppHandle, Emitter, Manager, State};

// Taurus als SSH-host: inkomende sessies met toestemming in de GUI (#121).
mod discovery;
mod sshhost;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Project {
    id: String,
    label: String,
    path: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    task: String,
    #[serde(default)]
    accent: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    command: String,
    // Welke agent-CLI start dit project: "" / "claude" (default) of "agy".
    #[serde(default)]
    agent: String,
    // Model voor de agent (vrije tekst). Leeg = de eigen default van de agent.
    #[serde(default)]
    model: String,
    // Op welke machine draait deze agent; leeg = deze computer. Hoort bij de
    // AGENT en niet bij het starten: een kaart is een precieze werkplek, en dan
    // is `path` een pad op DIE machine (#98).
    #[serde(default)]
    host_id: String,
}

// Lege defaults: een verse installatie start zonder projecten. De gebruiker
// voegt ze toe via de in-app editor ("Projecten"). Zo bevat een gedeelde build
// geen machine-specifieke paden.
fn default_projects() -> Vec<Project> {
    Vec::new()
}

// Per-gebruiker config: %APPDATA%\Taurus\projects.json (schrijfbaar, geen
// hardcoded dev-pad, werkt na installatie).
//
// TAURUS_CONFIG_DIR verlegt de HELE configmap. Bedoeld om een testexemplaar
// naast een draaiende Taurus te zetten: die deelt anders sessions.json, en dan
// hervat de tweede jouw lopende sessies en overschrijft ze daarna ook nog.
// Bewust een eigen variabele en niet APPDATA verbuigen: APPDATA erven de
// agents mee, en dan zoekt claude zijn credentials op de verkeerde plek.
fn config_dir() -> std::path::PathBuf {
    resolve_config_dir(
        std::env::var("TAURUS_CONFIG_DIR").ok(),
        std::env::var("APPDATA").ok(),
    )
}

// Apart gehouden zodat de keuze te testen is zonder aan de omgeving te zitten:
// env-variabelen zijn procesbreed en tests draaien parallel.
fn resolve_config_dir(override_dir: Option<String>, appdata: Option<String>) -> std::path::PathBuf {
    if let Some(dir) = override_dir.filter(|s| !s.trim().is_empty()) {
        return std::path::PathBuf::from(dir.trim());
    }
    appdata
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("Taurus")
}
fn config_path() -> std::path::PathBuf {
    config_dir().join("projects.json")
}

// Zorgt dat de config bestaat. Eerste start: migreer eenmalig een oude
// projects.json (oude dev-map of naast de exe); anders schrijf een lege lijst.
fn ensure_config() -> std::path::PathBuf {
    let p = config_path();
    if p.is_file() {
        return p;
    }
    let _ = std::fs::create_dir_all(config_dir());
    // Migreer eenmalig een projects.json die naast de exe is meegeleverd.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let legacy = dir.join("projects.json");
            if legacy.is_file() {
                if let Ok(txt) = std::fs::read_to_string(&legacy) {
                    let _ = std::fs::write(&p, txt);
                    return p;
                }
            }
        }
    }
    let _ = std::fs::write(&p, "[]");
    p
}

// Parse mislukt maar het bestand bestaat: zet het veilig opzij. Zonder backup
// toont de UI een lege lijst en overschrijft de eerstvolgende save het
// origineel — een herstelbare tikfout (hand-editen) werd zo dataverlies (#74).
fn backup_invalid(p: &Path) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let name = format!(
        "{}.invalid-{}",
        p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
        ts
    );
    if let Some(dir) = p.parent() {
        let _ = std::fs::copy(p, dir.join(name));
    }
}

#[tauri::command]
fn get_projects() -> Vec<Project> {
    let p = ensure_config();
    if let Ok(txt) = std::fs::read_to_string(&p) {
        match serde_json::from_str::<Vec<Project>>(&txt) {
            Ok(list) => return list,
            Err(_) => backup_invalid(&p),
        }
    }
    default_projects()
}

#[tauri::command]
fn save_projects(projects: Vec<Project>) -> Result<(), String> {
    let _ = std::fs::create_dir_all(config_dir());
    let txt = serde_json::to_string_pretty(&projects).map_err(|e| e.to_string())?;
    std::fs::write(config_path(), txt).map_err(|e| e.to_string())?;
    Ok(())
}

// ---------- machines: een machine is geen route (#124) ----------

// Waaronder een route wordt gegroepeerd. Expliciet gezette `machine` wint;
// anders het adres, want twee routes naar hetzelfde adres zijn per definitie
// dezelfde computer.
fn machine_key(h: &Host) -> String {
    let m = h.machine.trim();
    if m.is_empty() {
        h.hostname.trim().to_lowercase()
    } else {
        m.to_lowercase()
    }
}

// Eén regel per machine, met de routes eronder. De naam van de machine is de
// KORTSTE bijnaam van zijn routes: die bevat de onderscheidende toevoeging niet
// ("ursu" i.p.v. "ursu (Taurus-host)"), en juist die toevoeging wilden we kwijt.
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MachineView {
    key: String,
    label: String,
    routes: Vec<Host>,
    // De route die Taurus zelf zou kiezen: de Taurus-host als die er is, want
    // die vraagt geen sleuteluitwisseling en geen sshd op de andere machine.
    preferred: String,
}

const TAURUS_PORT: u16 = sshhost::DEFAULT_PORT;

fn preferred_route(routes: &[Host]) -> String {
    routes
        .iter()
        .find(|h| h.port == TAURUS_PORT)
        .or_else(|| routes.first())
        .map(|h| h.id.clone())
        .unwrap_or_default()
}

fn group_machines(hosts: Vec<Host>) -> Vec<MachineView> {
    let mut order: Vec<String> = Vec::new();
    let mut buckets: HashMap<String, Vec<Host>> = HashMap::new();
    for h in hosts {
        let k = machine_key(&h);
        if !buckets.contains_key(&k) {
            order.push(k.clone());
        }
        buckets.entry(k).or_default().push(h);
    }
    order
        .into_iter()
        .map(|k| {
            let routes = buckets.remove(&k).unwrap_or_default();
            let label = routes
                .iter()
                .map(|h| h.nickname.trim())
                .filter(|n| !n.is_empty())
                .min_by_key(|n| n.chars().count())
                .unwrap_or(k.as_str())
                .to_string();
            let preferred = preferred_route(&routes);
            MachineView { key: k, label, routes, preferred }
        })
        .collect()
}

#[tauri::command]
fn machines() -> Vec<MachineView> {
    group_machines(get_hosts())
}

// ---------- remote hosts (#98) ----------

// Een machine waarop een tab een agent kan draaien. Bewaard in
// %APPDATA%\Taurus\hosts.json, naast projects.json en sessions.json.
// Debug bevat geen geheim: key_path is een pad, nooit de sleutel zelf.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Host {
    id: String,
    nickname: String,
    hostname: String,
    // Welke FYSIEKE machine dit is (#124). Een `Host` is namelijk geen machine
    // maar een ROUTE ernaartoe: dezelfde computer staat er drie keer in zodra je
    // hem over sshd, over WSL en over een Taurus-host kunt bereiken. Daardoor
    // moest de naam het onderscheid dragen ("ursu (Taurus-host)"), en dat lekte
    // door naar tabbadges en agentkaarten.
    //
    // Leeg = val terug op `hostname`, zodat een bestaande hosts.json vanzelf
    // samenvalt tot een rij per adres zonder dat iemand iets hoeft te herschrijven.
    #[serde(default)]
    machine: String,
    #[serde(default)]
    user: String,
    #[serde(default = "default_ssh_port")]
    port: u16,
    // Pad naar de private key. Leeg = laat ssh zelf kiezen (~/.ssh/config of de
    // agent). Alles loopt via resolve_key, zodat een latere sleutelbron (vault)
    // maar een plek raakt.
    #[serde(default)]
    key_path: String,
    // Werkmap op de host als een project er geen eigen remote pad bij zet.
    #[serde(default)]
    default_project: String,
    // Hieronder: ingevuld door de probe bij "Add & test", niet met de hand.
    // "linux" | "windows" | "" (nog niet getest).
    #[serde(default)]
    os: String,
    // Wat houdt de sessie in leven als de verbinding wegvalt:
    // "herdr" | "tmux" | "psmux" | "taurus-agent" | "none". Bij "none" is het
    // transcript de enige persistentie (claude --resume) en sterft een lopende
    // beurt.
    #[serde(default)]
    mux: String,
    // Volgt `mux` de probe, of heb je hem zelf vastgezet? Een hertest mag een
    // eigen keuze niet stilletjes terugdraaien -- die test doe je juist vaak
    // NADAT je iets geinstalleerd hebt. Een oude hosts.json zonder dit veld
    // gedraagt zich als voorheen: automatisch.
    #[serde(default = "default_true")]
    mux_auto: bool,
    // Versie van de taurus-agent op de host; leeg als hij er niet staat.
    #[serde(default)]
    agent_version: String,
    // "" = de agent draait rechtstreeks op de host. "wsl" = hij draait in WSL op
    // een Windows-host. Dat is de goedkope route naar echte persistentie op
    // Windows: WSL heeft tmux, Windows niet. De prijs is dat de agent dan in
    // Linux zit en Windows-paden via /mnt/c ziet, en dat WSL zijn VM afsluit bij
    // inactiviteit -- wat een "persistente" sessie alsnog kan beeindigen.
    #[serde(default)]
    via: String,
}

// Onder welk OS draait de agent uiteindelijk? Bij `via: wsl` is de host Windows
// maar is alles daarbinnen Linux: paden, quoting en de naam van de binary.
fn effective_os(host: &Host) -> &str {
    if host.via == "wsl" {
        "linux"
    } else {
        &host.os
    }
}

fn default_ssh_port() -> u16 {
    22
}

fn default_true() -> bool {
    true
}

fn hosts_path() -> std::path::PathBuf {
    config_dir().join("hosts.json")
}

#[tauri::command]
fn get_hosts() -> Vec<Host> {
    let p = hosts_path();
    if let Ok(txt) = std::fs::read_to_string(&p) {
        match serde_json::from_str::<Vec<Host>>(&txt) {
            Ok(list) => return list,
            Err(_) => backup_invalid(&p),
        }
    }
    Vec::new()
}

#[tauri::command]
fn save_hosts(hosts: Vec<Host>) -> Result<(), String> {
    let _ = std::fs::create_dir_all(config_dir());
    let txt = serde_json::to_string_pretty(&hosts).map_err(|e| e.to_string())?;
    std::fs::write(hosts_path(), txt).map_err(|e| e.to_string())?;
    Ok(())
}

// Bereikbaarheid = kale TCP-connect naar de SSH-poort. Geen ICMP-ping: die
// vraagt op Windows verhoogde rechten en zegt bovendien niets over sshd.
const REACH_TIMEOUT_MS: u64 = 2000;

#[derive(serde::Deserialize)]
struct HostRef {
    id: String,
    hostname: String,
    #[serde(default = "default_ssh_port")]
    port: u16,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct HostStatus {
    id: String,
    reachable: bool,
    // Hoe lang de check duurde; de UI kan er een trage host mee aanduiden.
    ms: u64,
}

// Alle adressen van een naam TEGELIJK proberen, eerste succes wint.
//
// Serieel proberen lijkt logisch maar is fout: een machinenaam op een netwerk met
// IPv6 levert er zo zeven op (link-local, ULA, globaal, plus het IPv4-adres), en
// de niet-routeerbare adressen lopen elk in een timeout. Op volgorde is de hele
// deadline dan op voordat het adres dat WEL werkt aan de beurt is, en meldt de
// picker een bereikbare host als onbereikbaar. Zo gemeten op een echte host met
// zeven adressen waarvan alleen de laatste antwoordde.
//
// Het aantal adressen is begrensd, zodat een pathologische DNS-uitkomst geen
// tientallen threads oplevert.
const MAX_PROBE_ADDRS: usize = 8;

fn probe_tcp(hostname: &str, port: u16, timeout_ms: u64) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    use std::sync::mpsc;

    let timeout = std::time::Duration::from_millis(timeout_ms);
    let addrs: Vec<_> = match format!("{}:{}", hostname, port).to_socket_addrs() {
        Ok(a) => a.take(MAX_PROBE_ADDRS).collect(),
        Err(_) => return false, // onbekende naam telt als onbereikbaar
    };
    if addrs.is_empty() {
        return false;
    }

    let (tx, rx) = mpsc::channel();
    for addr in addrs {
        let tx = tx.clone();
        std::thread::spawn(move || {
            // Faalt de send, dan is de ontvanger al klaar (iemand anders was
            // eerder of de deadline verliep) -- dat is geen fout.
            let _ = tx.send(TcpStream::connect_timeout(&addr, timeout).is_ok());
        });
    }
    drop(tx); // anders blokkeert recv tot in de eeuwigheid op onze eigen kopie

    let deadline = std::time::Instant::now() + timeout;
    loop {
        let left = deadline.saturating_duration_since(std::time::Instant::now());
        match rx.recv_timeout(left) {
            Ok(true) => return true,
            Ok(false) => continue,       // dit adres niet, wacht op de rest
            Err(_) => return false,      // alle adressen op, of tijd om
        }
    }
}

// Alle hosts tegelijk, want de host-picker checkt de hele lijst bij openen.
// LET OP het `(async)`: een gewone synchrone #[tauri::command] draait bij Tauri
// op de main thread, dus een blokkerende connect van 2s zou het venster laten
// bevriezen. Met (async) gaat de aanroep naar de threadpool, en de threads
// hieronder doen de hosts naast elkaar in plaats van na elkaar.
#[tauri::command(async)]
fn check_hosts(hosts: Vec<HostRef>) -> Vec<HostStatus> {
    let handles: Vec<_> = hosts
        .into_iter()
        .map(|h| {
            std::thread::spawn(move || {
                let t0 = std::time::Instant::now();
                let reachable = probe_tcp(&h.hostname, h.port, REACH_TIMEOUT_MS);
                HostStatus {
                    id: h.id,
                    reachable,
                    ms: t0.elapsed().as_millis() as u64,
                }
            })
        })
        .collect();
    // Een gepanicte thread levert geen status op i.p.v. de hele lijst te slopen.
    handles.into_iter().filter_map(|h| h.join().ok()).collect()
}

// Wat "Add & test" over een host te weten komt. Serialize-only, camelCase voor
// de frontend -- net als SessionState/SttStatus.
#[derive(serde::Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct HostProbe {
    reachable: bool,
    auth_ok: bool,
    os: String,
    // Beste multiplexer die op de host gevonden is; "none" als er geen is.
    mux: String,
    // Alles wat er gevonden is, in voorkeursvolgorde. Nodig omdat de gebruiker
    // een andere dan de beste mag kiezen: zonder deze lijst kan het formulier
    // niet zien of die keuze op de host bestaat.
    muxes: Vec<String>,
    // Versieregel van de agent-CLI, leeg als hij ontbreekt.
    claude: String,
    // Uitgaand HTTPS naar api.anthropic.com. Zonder dit kan er geen agent
    // draaien, hoe goed de rest ook werkt -- dus expliciet meten.
    outbound: bool,
    // Alleen voor Windows-hosts: heeft WSL zowel een multiplexer als een
    // agent-CLI? Sinds herdr is `via: wsl` niet meer de enige route naar
    // sessiepersistentie op zo'n machine, maar wel nog steeds een geldige.
    wsl_usable: bool,
    // Welke multiplexer dat dan is ("herdr" of "tmux"), zodat een via:wsl-host
    // niet blind op tmux wordt gezet terwijl daar herdr staat.
    wsl_mux: String,
    error: String,
}

// Welke van de gevonden multiplexers Taurus zou kiezen. Herdr eerst: dezelfde
// persistentie als tmux, plus een status die je kunt uitlezen, en hij bestaat op
// alle drie de platforms. taurus-agent staat er nog in omdat een oude
// hosts.json hem kan bevatten, maar hij is nooit gebouwd -- daarom laatst.
fn best_mux(found: &[String]) -> String {
    for pref in ["herdr", "tmux", "psmux", "taurus-agent"] {
        if found.iter().any(|m| m == pref) {
            return pref.to_string();
        }
    }
    "none".to_string()
}

// ssh-argumenten tot en met het doel, gedeeld door de launch en de probe.
// `batch` hoort AAN bij een probe en UIT bij een launch: zonder BatchMode blijft
// ssh op een wachtwoordprompt staan die niemand beantwoordt, en dan hangt "Add &
// test" tot de timeout. Bij een echte sessie wil je die prompt juist wel -- daar
// zit een pty en kun je een passphrase intypen.
fn ssh_base_args(host: &Host, batch: bool) -> Result<Vec<String>, String> {
    if host.hostname.trim().is_empty() {
        return Err("host heeft geen hostname".into());
    }
    let mut a: Vec<String> = Vec::new();
    if host.port != 22 {
        a.push("-p".into());
        a.push(host.port.to_string());
    }
    if let KeySource::Path(k) = resolve_key(host)? {
        a.push("-i".into());
        a.push(k);
    }
    a.push("-o".into());
    a.push("StrictHostKeyChecking=accept-new".into());
    a.push("-o".into());
    a.push("ConnectTimeout=10".into());
    if batch {
        a.push("-o".into());
        a.push("BatchMode=yes".into());
    }
    a.push(if host.user.trim().is_empty() {
        host.hostname.trim().to_string()
    } else {
        format!("{}@{}", host.user.trim(), host.hostname.trim())
    });
    Ok(a)
}

// Een achtergrondproces (ssh/scp) zonder consolevenster. Zonder deze vlag flitst
// er bij elke probe een zwart venster op -- en "Opnieuw testen" opende er twee,
// een per ssh-ronde. Dezelfde vlag die ps_encoded al gebruikt.
fn quiet_command(program: &str) -> std::process::Command {
    let mut c = std::process::Command::new(program);
    c.stdin(std::process::Stdio::null()); // anders wacht de remote kant op invoer
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    c
}

// Eén ssh-aanroep; geeft (stdout, gelukt).
fn ssh_run(host: &Host, remote_cmd: &str) -> Result<(String, bool), String> {
    let mut args = ssh_base_args(host, true)?;
    args.push(remote_cmd.to_string());
    let out = quiet_command(&ssh_program())
        .args(&args)
        .output()
        .map_err(|e| format!("ssh starten mislukte: {}", e))?;
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    if s.trim().is_empty() {
        s = String::from_utf8_lossy(&out.stderr).into_owned();
    }
    Ok((s, out.status.success()))
}

// Alles wat we van een host willen weten, in twee ssh-rondes: eerst het OS,
// daarna een OS-specifieke inventarisatie. Duur genoeg (handshake) om los te
// staan van de goedkope TCP-check van check_hosts.
#[tauri::command(async)]
fn probe_host(host: Host) -> HostProbe {
    let mut p = HostProbe::default();
    p.reachable = probe_tcp(host.hostname.trim(), host.port, REACH_TIMEOUT_MS);
    if !p.reachable {
        p.error = format!("Geen SSH-server bereikbaar op {}:{}", host.hostname.trim(), host.port);
        return p;
    }

    // uname bestaat niet op Windows; dat het faalt is zelf het signaal.
    match ssh_run(&host, "uname -sm") {
        Err(e) => {
            p.error = e;
            return p;
        }
        Ok((out, ok)) if ok && out.to_lowercase().contains("linux") => {
            p.auth_ok = true;
            p.os = "linux".into();
        }
        Ok((out, _)) => {
            // Auth-fouten herkennen we aan de ssh-melding; anders is het Windows.
            let low = out.to_lowercase();
            if low.contains("permission denied") || low.contains("publickey") {
                p.error = "Key-auth geweigerd. Let op: voor een account in de Administrators-groep hoort de key in C:\\ProgramData\\ssh\\administrators_authorized_keys, niet in %USERPROFILE%\\.ssh\\authorized_keys.".into();
                return p;
            }
            if low.contains("connection") && low.contains("refused") {
                p.error = out.trim().to_string();
                return p;
            }
            p.auth_ok = true;
            p.os = "windows".into();
        }
    }

    // Ronde 2: inventarisatie. Op Windows via base64 UTF-16LE, zodat cmd.exe geen
    // enkel metateken te zien krijgt (dezelfde reden als bij de launch).
    let (out, _) = if p.os == "linux" {
        // Alles rapporteren wat er staat, niet alleen de beste: de gebruiker mag
        // in het formulier een andere kiezen, en dan moet te zien zijn of die
        // keuze bestaat. Deze shell is GEEN login shell, dus ~/.local/bin staat
        // niet op PATH -- het expliciete -x is daarom geen luxe maar de normale
        // situatie.
        let script = "M=''; for c in herdr tmux psmux; do \
                      if command -v \"$c\" >/dev/null 2>&1 || [ -x \"$HOME/.local/bin/$c\" ]; then M=\"$M$c,\"; fi; done; \
                      echo \"MUXES=$M\"; \
                      (claude --version 2>/dev/null || echo '') | head -1 | sed 's/^/CLAUDE=/'; \
                      curl -sS -o /dev/null -w 'HTTP=%{http_code}' --max-time 10 https://api.anthropic.com/ 2>/dev/null || echo HTTP=000";
        match ssh_run(&host, script) {
            Ok(v) => v,
            Err(e) => {
                p.error = e;
                return p;
            }
        }
    } else {
        // Zelfde lijst als op POSIX. De herdr-installer zet zijn map alleen in de
        // PATH van NIEUWE sessies, dus Get-Command alleen is niet genoeg.
        let ps = r#"$f = @()
if ((Get-Command herdr -EA 0) -or (Test-Path "$env:LOCALAPPDATA\Programs\Herdr\bin\herdr.exe")) { $f += 'herdr' }
if (Get-Command psmux -EA 0) { $f += 'psmux' }
if (Test-Path "$env:USERPROFILE\.taurus\bin\taurus-agent.exe") { $f += 'taurus-agent' }
'MUXES=' + ($f -join ',')
$c = Get-Command claude -EA 0
if ($c) { 'CLAUDE=' + (& claude --version 2>&1 | Select-Object -First 1) } else { 'CLAUDE=' }
try { $r = Invoke-WebRequest -Uri https://api.anthropic.com/ -Method Head -TimeoutSec 10 -UseBasicParsing; 'HTTP=' + $r.StatusCode } catch { 'HTTP=' + [int]$_.Exception.Response.StatusCode }"#;
        let enc = b64(&utf16le(ps));
        match ssh_run(&host, &format!("powershell -NoProfile -EncodedCommand {}", enc)) {
            Ok(v) => v,
            Err(e) => {
                p.error = e;
                return p;
            }
        }
    };

    for line in out.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("MUXES=") {
            p.muxes = v
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        } else if let Some(v) = line.strip_prefix("CLAUDE=") {
            p.claude = v.trim().to_string();
        } else if let Some(v) = line.strip_prefix("HTTP=") {
            // Elke HTTP-status betekent dat er verbinding was; 000 is curl's
            // "geen antwoord". 401/404 tellen dus als goed.
            let code: u32 = v.trim().parse().unwrap_or(0);
            p.outbound = code > 0;
        }
    }
    p.mux = best_mux(&p.muxes);

    // Windows zonder multiplexer: kijk of WSL er wel een heeft. Dat is de enige
    // route naar heraanhaken op zo'n machine zonder iets te installeren. Losse
    // ronde, want het script moet door cmd.exe EN door sh heen -- vandaar base64.
    if p.os == "windows" {
        let inner = "if command -v herdr >/dev/null 2>&1 || [ -x \"$HOME/.local/bin/herdr\" ]; then M=herdr; \
                     elif command -v tmux >/dev/null 2>&1; then M=tmux; else exit 1; fi; \
                     { command -v claude >/dev/null 2>&1 || [ -x \"$HOME/.local/bin/claude\" ]; } || exit 1; \
                     echo WSLOK=$M";
        let cmd = format!(
            "wsl -e sh -c \"echo {} | base64 -d | sh\"",
            b64(inner.as_bytes())
        );
        if let Ok((out, _)) = ssh_run(&host, &cmd) {
            if let Some(m) = out
                .lines()
                .find_map(|l| l.trim().strip_prefix("WSLOK="))
                .map(|m| m.trim().to_string())
            {
                p.wsl_usable = !m.is_empty();
                p.wsl_mux = m;
            }
        }
    }
    p
}

// scp zit naast ssh in System32; zelfde voorkeur voor het systeempad, want een
// Git-for-Windows scp gedraagt zich anders rond Windows-paden.
fn scp_program() -> String {
    if let Ok(root) = std::env::var("SystemRoot") {
        let p = std::path::PathBuf::from(&root).join("System32\\OpenSSH\\scp.exe");
        if p.is_file() {
            return p.to_string_lossy().into_owned();
        }
    }
    "scp.exe".into()
}

// Voeg een padcomponent toe met de scheidingstekens van de DOEL-machine, niet
// die van ons. Een backslash in een Linux-pad is een gewoon teken, geen map.
fn remote_join(os: &str, dir: &str, name: &str) -> String {
    let d = dir.trim_end_matches(['/', '\\']);
    if os == "windows" {
        format!("{}\\{}", d, name)
    } else {
        format!("{}/{}", d, name)
    }
}

// De DROPZONE op een remote sessie: kopieer het bestand naar <werkmap>/input op
// de host en geef het pad DAAR terug, zodat de prompt een pad krijgt dat de
// agent ook echt kan openen.
#[tauri::command(async)]
fn scp_to_host(host_id: String, src: String, remote_cwd: String) -> Result<String, String> {
    let host = lookup_host(&host_id)?.ok_or_else(|| format!("onbekende host: {}", host_id))?;
    if host.via == "wsl" {
        // De scp-server draait op Windows en kan niet in het ext4 van WSL
        // schrijven; dat zou via \\wsl.localhost\<distro>\... moeten en dat is
        // een eigen puzzel. Liever een duidelijke melding dan een bestand dat
        // ergens anders landt dan de agent verwacht.
        return Err(
            "Bestanden overzetten naar een agent in WSL kan nog niet. Zet het bestand met de hand neer, of gebruik een host zonder WSL-route.".into(),
        );
    }
    let name = Path::new(&src)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| format!("geen bestandsnaam in {}", src))?;
    let remote_dir = remote_join(&host.os, remote_cwd.trim(), "input");

    // De input-map hoeft nog niet te bestaan; scp maakt hem niet aan.
    let mkdir = if host.os == "windows" {
        let ps = format!(
            "New-Item -ItemType Directory -Force -Path {} | Out-Null",
            ps_quote(&remote_dir)
        );
        format!("powershell -NoProfile -EncodedCommand {}", b64(&utf16le(&ps)))
    } else {
        format!("mkdir -p {}", shell_quote_posix(&remote_dir))
    };
    let (out, ok) = ssh_run(&host, &mkdir)?;
    if !ok {
        return Err(format!("kon {} niet aanmaken op de host:\n{}", remote_dir, out.trim()));
    }

    // scp gebruikt -P voor de poort waar ssh -p gebruikt; met -p zou scp de
    // tijdstempels willen behouden en zwijgend op de standaardpoort verbinden.
    let mut args: Vec<String> = Vec::new();
    if host.port != 22 {
        args.push("-P".into());
        args.push(host.port.to_string());
    }
    if let KeySource::Path(k) = resolve_key(&host)? {
        args.push("-i".into());
        args.push(k);
    }
    args.push("-o".into());
    args.push("StrictHostKeyChecking=accept-new".into());
    args.push("-o".into());
    args.push("BatchMode=yes".into());
    args.push(src.clone());
    // NIET quoten achter de dubbele punt. Sinds OpenSSH 9 draait scp standaard
    // over SFTP, en dan is het pad een letterlijke string in plaats van iets dat
    // een remote shell nog parseert: quotes komen er dan als tekens aan en scp
    // meldt `dest open "'C:/...'": No such file or directory`. Zonder quotes
    // werkt een pad met spaties gewoon -- getest tegen een map "my input".
    let target_dir = if host.os == "windows" {
        // De Windows-kant wil forward slashes achter de dubbele punt.
        remote_dir.replace('\\', "/")
    } else {
        remote_dir.clone()
    };
    let user_at = if host.user.trim().is_empty() {
        host.hostname.trim().to_string()
    } else {
        format!("{}@{}", host.user.trim(), host.hostname.trim())
    };
    args.push(format!("{}:{}/", user_at, target_dir.trim_end_matches('/')));

    let out = quiet_command(&scp_program())
        .args(&args)
        .output()
        .map_err(|e| format!("scp starten mislukte: {}", e))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("scp mislukte: {}", err.trim()));
    }
    Ok(remote_join(&host.os, &remote_dir, &name))
}

// ---------- werkmap verplaatsen tussen machines (#102) ----------

// Mappen die werk bevatten in plaats van broncode. Ze worden apart geteld zodat
// je per stuk kunt kiezen of ze meegaan -- een output-map van 4 GB wil je zelden
// meesturen, een input-map met net aangeleverde bestanden juist wel.
const WORK_DIRS: [&str; 5] = ["input", "output", "log", "logs", "review"];
// Mappen die groot en reproduceerbaar zijn. Standaard uit: ze zijn meestal het
// leeuwendeel van de omvang en aan de andere kant zo weer opgebouwd.
const BULK_DIRS: [&str; 6] = [".git", "node_modules", "target", "dist", ".venv", "__pycache__"];

#[derive(serde::Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct DirEntrySize {
    name: String,
    bytes: u64,
    files: u64,
}

#[derive(serde::Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSurvey {
    // De rest: alles buiten de apart genoemde mappen. Dat gaat altijd mee.
    core_bytes: u64,
    core_files: u64,
    // Dezelfde items als losse namen. Nodig om terug te halen: scp kan aan de
    // andere kant geen map uitlezen, dus de lijst moet expliciet zijn -- een
    // wildcard zou juist ook meenemen wat je hebt uitgevinkt.
    core: Vec<String>,
    work: Vec<DirEntrySize>,
    bulk: Vec<DirEntrySize>,
    // Bestaat de map, en wanneer is er voor het laatst iets in gewijzigd
    // (unix-seconden, 0 = onbekend)? Daarmee kan de UI zeggen welke kant de
    // nieuwste versie heeft in plaats van botweg te weigeren.
    exists: bool,
    newest: u64,
    error: String,
}

// De nieuwste wijziging in een boom, als unix-seconden.
fn newest_mtime(path: &Path) -> u64 {
    let mut newest = 0u64;
    let rd = match std::fs::read_dir(path) {
        Ok(r) => r,
        Err(_) => return 0,
    };
    for e in rd.flatten() {
        let meta = match std::fs::symlink_metadata(e.path()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_symlink() {
            continue;
        }
        let t = meta
            .modified()
            .ok()
            .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        newest = newest.max(t);
        if meta.is_dir() {
            newest = newest.max(newest_mtime(&e.path()));
        }
    }
    newest
}

// Tel een map recursief. Symlinks worden niet gevolgd: een lus zou hier anders
// oneindig doorlopen, en een gevolgde link telt bestanden dubbel.
fn dir_size(path: &Path) -> (u64, u64) {
    let mut bytes = 0u64;
    let mut files = 0u64;
    let rd = match std::fs::read_dir(path) {
        Ok(r) => r,
        Err(_) => return (0, 0),
    };
    for e in rd.flatten() {
        let meta = match std::fs::symlink_metadata(e.path()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_symlink() {
            continue;
        }
        if meta.is_dir() {
            let (b, f) = dir_size(&e.path());
            bytes += b;
            files += f;
        } else {
            bytes += meta.len();
            files += 1;
        }
    }
    (bytes, files)
}

// Wat zit er in deze werkmap, en hoe groot is het? Vóór het overzetten, zodat
// een wachttijd van tien minuten geen verrassing is.
#[tauri::command(async)]
fn survey_workspace(path: String) -> WorkspaceSurvey {
    let mut s = WorkspaceSurvey::default();
    let root = Path::new(&path);
    if !root.is_dir() {
        // Geen fout: een nog niet bestaande doelmap is het normale geval bij een
        // eerste overzetting. De UI onderscheidt dat aan `exists`.
        return s;
    }
    s.exists = true;
    s.newest = newest_mtime(root);
    let named: Vec<&str> = WORK_DIRS.iter().chain(BULK_DIRS.iter()).copied().collect();
    let rd = match std::fs::read_dir(root) {
        Ok(r) => r,
        Err(e) => {
            s.error = e.to_string();
            return s;
        }
    };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let meta = match std::fs::symlink_metadata(e.path()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_symlink() {
            continue;
        }
        if meta.is_dir() {
            let lower = name.to_lowercase();
            let (bytes, files) = dir_size(&e.path());
            if named.iter().any(|n| *n == lower) {
                let entry = DirEntrySize { name: name.clone(), bytes, files };
                if WORK_DIRS.contains(&lower.as_str()) {
                    s.work.push(entry);
                } else {
                    s.bulk.push(entry);
                }
            } else {
                s.core_bytes += bytes;
                s.core_files += files;
                s.core.push(name);
            }
        } else {
            s.core_bytes += meta.len();
            s.core_files += 1;
            s.core.push(name);
        }
    }
    s.work.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    s.bulk.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    s
}

// Dezelfde meting als survey_workspace, maar op een host. Nodig voor twee
// dingen: terughalen naar deze computer (dan is de host de BRON), en kunnen
// zeggen welke kant recenter is bijgewerkt voordat er iets overschreven wordt.
//
// Het script schrijft regels `T|naam|bytes|files|mtime`; dat parseert eenvoudig
// en overleeft een motd of andere ruis op de verbinding.
#[tauri::command(async)]
fn survey_remote_workspace(host_id: String, path: String) -> WorkspaceSurvey {
    let mut s = WorkspaceSurvey::default();
    let host = match lookup_host(&host_id) {
        Ok(Some(h)) => h,
        Ok(None) => {
            s.error = format!("onbekende host: {}", host_id);
            return s;
        }
        Err(e) => {
            s.error = e;
            return s;
        }
    };

    let cmd = if effective_os(&host) == "windows" {
        let ps = format!(
            r#"$r = {p}
if (-not (Test-Path $r)) {{ 'MISSING'; exit }}
'EXISTS'
Get-ChildItem $r -Force -EA 0 | ForEach-Object {{
  if ($_.PSIsContainer) {{
    $f = Get-ChildItem $_.FullName -Recurse -File -Force -EA 0
    $b = ($f | Measure-Object -Property Length -Sum).Sum
    $n = ($f | Measure-Object -Property LastWriteTimeUtc -Maximum).Maximum
    $t = if ($n) {{ [int][double]::Parse((Get-Date $n -UFormat %s)) }} else {{ 0 }}
    'D|' + $_.Name + '|' + [int64]$b + '|' + $f.Count + '|' + $t
  }} else {{
    $t = [int][double]::Parse((Get-Date $_.LastWriteTimeUtc -UFormat %s))
    'F|' + $_.Name + '|' + $_.Length + '|1|' + $t
  }}
}}"#,
            p = ps_quote(&path)
        );
        format!("powershell -NoProfile -EncodedCommand {}", b64(&utf16le(&ps)))
    } else {
        let inner = format!(
            r#"r={p}
[ -d "$r" ] || {{ echo MISSING; exit 0; }}
echo EXISTS
for e in "$r"/* "$r"/.*; do
  b=$(basename "$e")
  [ "$b" = "." ] || [ "$b" = ".." ] || [ ! -e "$e" ] || {{
    if [ -d "$e" ]; then
      sz=$(du -sb "$e" 2>/dev/null | cut -f1)
      n=$(find "$e" -type f 2>/dev/null | wc -l)
      t=$(find "$e" -printf '%T@\n' 2>/dev/null | sort -n | tail -1 | cut -d. -f1)
      echo "D|$b|${{sz:-0}}|${{n:-0}}|${{t:-0}}"
    else
      sz=$(stat -c%s "$e" 2>/dev/null)
      t=$(stat -c%Y "$e" 2>/dev/null)
      echo "F|$b|${{sz:-0}}|1|${{t:-0}}"
    fi
  }}
done"#,
            p = shell_quote_posix(&path)
        );
        if host.via == "wsl" {
            format!("wsl -e sh -c \"echo {} | base64 -d | sh\"", b64(inner.as_bytes()))
        } else {
            format!("echo {} | base64 -d | sh", b64(inner.as_bytes()))
        }
    };

    let (out, _) = match ssh_run(&host, &cmd) {
        Ok(v) => v,
        Err(e) => {
            s.error = e;
            return s;
        }
    };
    if !out.contains("EXISTS") {
        return s; // bestaat nog niet -- geen fout
    }
    s.exists = true;
    for line in out.lines() {
        let p: Vec<&str> = line.trim().split('|').collect();
        if p.len() != 5 || (p[0] != "D" && p[0] != "F") {
            continue;
        }
        let name = p[1].to_string();
        let bytes: u64 = p[2].parse().unwrap_or(0);
        let files: u64 = p[3].parse().unwrap_or(0);
        let mtime: u64 = p[4].parse().unwrap_or(0);
        s.newest = s.newest.max(mtime);
        let lower = name.to_lowercase();
        if p[0] == "D" && WORK_DIRS.contains(&lower.as_str()) {
            s.work.push(DirEntrySize { name, bytes, files });
        } else if p[0] == "D" && BULK_DIRS.contains(&lower.as_str()) {
            s.bulk.push(DirEntrySize { name, bytes, files });
        } else {
            s.core_bytes += bytes;
            s.core_files += files;
            s.core.push(name);
        }
    }
    s.work.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    s.bulk.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    s
}

// Zet de werkmap over naar een host. `skip` bevat de mapnamen die niet meegaan.
// Kopieert; de bron blijft staan -- zie #102 voor waarom dat een bewuste keuze is.
// Voortgang van een overzetting (#107). 1,7 GB duurt minuten, en zonder dit
// staat er alleen "Synchroniseren…" terwijl niemand kan zien of het opschiet.
//
// Wat WEL meegaat: welk item aan de beurt is en hoeveel er zijn. Wat NIET
// meegaat: bytes. De frontend heeft de groottes per item al uit de survey
// staan -- daar rekent hij het percentage zelf uit, en dat scheelt hier een
// tweede recursieve telling die net zo lang duurt als de survey zelf.
//
// Twee fasen, want met alleen "done" zou het eerste item -- vaak juist het
// grootste -- er dood uitzien tot het klaar is.
fn emit_transfer(app: &AppHandle, phase: &str, name: &str, index: u32, total: u32) {
    let _ = app.emit("transfer-progress", (phase, name, index, total));
}

#[tauri::command(async)]
fn push_workspace(
    app: AppHandle,
    host_id: String,
    local_path: String,
    remote_path: String,
    skip: Vec<String>,
) -> Result<String, String> {
    let host = lookup_host(&host_id)?.ok_or_else(|| format!("onbekende host: {}", host_id))?;
    if host.via == "wsl" {
        return Err(
            "Overzetten naar een agent in WSL kan nog niet: de scp-server draait op Windows en komt niet in het bestandssysteem van WSL.".into(),
        );
    }
    let src = Path::new(&local_path);
    if !src.is_dir() {
        return Err(format!("Map bestaat niet:\n{}", local_path));
    }

    // Doelmap aanmaken; scp -r maakt hem zelf niet aan.
    let mkdir = if host.os == "windows" {
        let ps = format!(
            "New-Item -ItemType Directory -Force -Path {} | Out-Null",
            ps_quote(&remote_path)
        );
        format!("powershell -NoProfile -EncodedCommand {}", b64(&utf16le(&ps)))
    } else {
        format!("mkdir -p {}", shell_quote_posix(&remote_path))
    };
    let (out, ok) = ssh_run(&host, &mkdir)?;
    if !ok {
        return Err(format!("kon {} niet aanmaken:\n{}", remote_path, out.trim()));
    }

    // Per top-level item apart, zodat overslaan mogelijk is. scp -r op de hele
    // map zou alles meenemen, inclusief een node_modules van een halve gigabyte.
    let skip_lower: Vec<String> = skip.iter().map(|s| s.to_lowercase()).collect();
    let mut skipped = 0u32;
    let target_dir = if host.os == "windows" {
        remote_path.replace('\\', "/")
    } else {
        remote_path.clone()
    };
    let user_at = host_target(&host);

    // Eerst de lijst, dan versturen: voor "item 3 van 12" moet het totaal
    // bekend zijn voordat het eerste item begint.
    let mut todo: Vec<(String, std::path::PathBuf)> = Vec::new();
    for e in std::fs::read_dir(src).map_err(|e| e.to_string())?.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if skip_lower.contains(&name.to_lowercase()) {
            skipped += 1;
            continue;
        }
        todo.push((name, e.path()));
    }
    let total = todo.len() as u32;

    for (i, (name, path)) in todo.iter().enumerate() {
        let index = i as u32 + 1;
        emit_transfer(&app, "start", name, index, total);

        let mut args = scp_base_args(&host)?;
        args.push(path.to_string_lossy().into_owned());
        // Niet quoten achter de dubbele punt: scp loopt sinds OpenSSH 9 over
        // SFTP, en dan is het pad letterlijk (zie scp_to_host).
        args.push(format!("{}:{}/", user_at, target_dir.trim_end_matches('/')));

        let out = quiet_command(&scp_program())
            .args(&args)
            .output()
            .map_err(|err| format!("scp starten mislukte: {}", err))?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("overzetten van '{}' mislukte:\n{}", name, err.trim()));
        }
        emit_transfer(&app, "done", name, index, total);
    }
    Ok(format!("{} overgezet, {} overgeslagen", total, skipped))
}

// Gedeelde scp-opties voor beide richtingen.
fn scp_base_args(host: &Host) -> Result<Vec<String>, String> {
    let mut a: Vec<String> = vec!["-r".into()];
    if host.port != 22 {
        // scp gebruikt -P waar ssh -p gebruikt; -p zou hier "behoud tijdstempels"
        // betekenen en stil op de standaardpoort verbinden.
        a.push("-P".into());
        a.push(host.port.to_string());
    }
    if let KeySource::Path(k) = resolve_key(host)? {
        a.push("-i".into());
        a.push(k);
    }
    a.push("-o".into());
    a.push("StrictHostKeyChecking=accept-new".into());
    a.push("-o".into());
    a.push("BatchMode=yes".into());
    Ok(a)
}

fn host_target(host: &Host) -> String {
    if host.user.trim().is_empty() {
        host.hostname.trim().to_string()
    } else {
        format!("{}@{}", host.user.trim(), host.hostname.trim())
    }
}

// Haal de werkmap van een host terug naar deze computer. Spiegelbeeld van
// push_workspace; bestaande bestanden worden overschreven, want de aangevinkte
// kant is de bron van waarheid.
#[tauri::command(async)]
fn pull_workspace(
    app: AppHandle,
    host_id: String,
    remote_path: String,
    local_path: String,
    items: Vec<String>,
) -> Result<String, String> {
    let host = lookup_host(&host_id)?.ok_or_else(|| format!("onbekende host: {}", host_id))?;
    if host.via == "wsl" {
        return Err(
            "Terughalen uit WSL kan nog niet: de scp-server draait op Windows en komt niet in het bestandssysteem van WSL.".into(),
        );
    }
    std::fs::create_dir_all(&local_path)
        .map_err(|e| format!("kon {} niet aanmaken:\n{}", local_path, e))?;

    let remote_base = if host.os == "windows" {
        remote_path.replace('\\', "/")
    } else {
        remote_path.clone()
    };
    let user_at = host_target(&host);
    let total = items.len() as u32;
    for (i, name) in items.iter().enumerate() {
        let index = i as u32 + 1;
        emit_transfer(&app, "start", name, index, total);

        let mut args = scp_base_args(&host)?;
        // Niet quoten achter de dubbele punt: scp loopt sinds OpenSSH 9 over
        // SFTP en behandelt het pad letterlijk.
        args.push(format!("{}:{}/{}", user_at, remote_base.trim_end_matches('/'), name));
        args.push(local_path.clone());
        let out = quiet_command(&scp_program())
            .args(&args)
            .output()
            .map_err(|e| format!("scp starten mislukte: {}", e))?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("ophalen van '{}' mislukte:\n{}", name, err.trim()));
        }
        emit_transfer(&app, "done", name, index, total);
    }
    Ok(format!("{} opgehaald", total))
}

// Map-kiezer (bladeren-knop in de project-editor).
#[tauri::command]
fn pick_folder(app: AppHandle) -> Option<String> {
    use tauri_plugin_dialog::DialogExt;
    app.dialog()
        .file()
        .blocking_pick_folder()
        .and_then(|p| p.into_path().ok())
        .map(|p| p.to_string_lossy().into_owned())
}

// Bestand-kiezer voor de dropzone-+. Start in `start_dir` (de input-map); die
// maken we zo nodig eerst aan zodat de dialoog daar echt opent. Geeft het gekozen
// absolute pad terug (None bij annuleren).
#[tauri::command]
fn pick_file(app: AppHandle, start_dir: String) -> Option<String> {
    use tauri_plugin_dialog::DialogExt;
    // ~ expanderen: de sleutelkiezer vraagt om "~/.ssh", en zonder dit zou
    // create_dir_all een relatieve map naast de exe aanmaken.
    let start_dir = expand_home(&start_dir);
    let _ = std::fs::create_dir_all(&start_dir);
    let mut b = app.dialog().file();
    if Path::new(&start_dir).is_dir() {
        b = b.set_directory(&start_dir);
    }
    b.blocking_pick_file()
        .and_then(|p| p.into_path().ok())
        .map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
fn path_exists(path: String) -> bool {
    Path::new(&path).is_dir()
}

// Versie van de app (uit Cargo.toml) -> discreet in de UI getoond.
#[tauri::command]
fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Heeft de gekozen map een CLAUDE.md (hoofdletterongevoelig)? Een ad-hoc map
// openen mag altijd, maar de UI geeft een hint als die ontbreekt -- "optimaal"
// is een map met projectinstructies voor Claude.
#[tauri::command]
fn has_claude_md(path: String) -> bool {
    let rd = match std::fs::read_dir(&path) {
        Ok(r) => r,
        Err(_) => return false,
    };
    for entry in rd.flatten() {
        if entry.file_name().to_string_lossy().eq_ignore_ascii_case("claude.md") {
            return true;
        }
    }
    false
}

// ===== Persistente sessies =====
// Een opgeslagen sessie: genoeg om bij de volgende start `claude --resume <uuid>`
// te doen. De `id` is een vluchtige UI-handle; de `uuid` is het anker.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct PersistedSession {
    id: String,
    uuid: String,
    path: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    accent: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    agent: String,
    #[serde(default)]
    model: String,
    // Op welke host draaide deze sessie; leeg = lokaal. Bestaande sessions.json
    // zonder dit veld blijven daardoor gewoon lokale sessies.
    #[serde(default)]
    host_id: String,
    // Van welke agentkaart komt deze sessie (#90); leeg = losse sessie. Nodig om
    // de tabbundeling een herstart te laten overleven. Een bestaande
    // sessions.json zonder dit veld levert lege waarden -- die sessies komen dan
    // eenmalig als losse sessies terug.
    #[serde(default)]
    project_id: String,
}

fn sessions_path() -> std::path::PathBuf {
    config_dir().join("sessions.json")
}

#[tauri::command]
fn save_sessions(sessions: Vec<PersistedSession>) -> Result<(), String> {
    let _ = std::fs::create_dir_all(config_dir());
    let txt = serde_json::to_string_pretty(&sessions).map_err(|e| e.to_string())?;
    std::fs::write(sessions_path(), txt).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_sessions() -> Vec<PersistedSession> {
    let p = sessions_path();
    if let Ok(txt) = std::fs::read_to_string(&p) {
        match serde_json::from_str::<Vec<PersistedSession>>(&txt) {
            Ok(list) => return list,
            // Corrupt: opzij zetten, anders wist persistSessionsToDisk het zo (#74).
            Err(_) => backup_invalid(&p),
        }
    }
    Vec::new()
}

// ---------- sessiegeschiedenis (#129) ----------
//
// GEMETEN na een herstart voor een driverinstallatie (2026-08-14): van vier sessies
// kwamen er twee terug en waren de andere twee niet "niet hersteld" maar VERDWENEN.
// Dat is geen pech maar de code: restoreSessions slaat over wat het niet kan
// hervatten, en persistSessionsToDisk schrijft daarna sessions.json opnieuw uit de
// tabs die dan open staan. Het enige spoor dat Taurus bijhield was daarmee weg.
//
// Deze lijst is daarom een ANDER bestand met een andere regel: er wordt aan
// toegevoegd en bijgewerkt, en een mislukte herstart haalt er nooit iets uit.
// sessions.json blijft wat het is -- "wat stond er open" -- en dit is "wat is er
// geweest".
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct HistoryEntry {
    uuid: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    accent: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    agent: String,
    #[serde(default)]
    model: String,
    #[serde(default)]
    host_id: String,
    #[serde(default)]
    project_id: String,
    // Seconden sinds epoch, net als de rest van de tree (geen chrono).
    #[serde(default)]
    created: u64,
    #[serde(default)]
    last_seen: u64,
    // Stond hij open toen Taurus voor het laatst sloot? Dat bepaalt of hij bij het
    // opstarten voorgevinkt staat -- niet of hij bewaard blijft.
    #[serde(default)]
    was_open: bool,
}

fn history_path() -> std::path::PathBuf {
    config_dir().join("history.json")
}

fn read_history() -> Vec<HistoryEntry> {
    let p = history_path();
    match std::fs::read_to_string(&p) {
        Ok(txt) => match serde_json::from_str::<Vec<HistoryEntry>>(&txt) {
            Ok(v) => v,
            Err(_) => {
                backup_invalid(&p);
                Vec::new()
            }
        },
        Err(_) => Vec::new(),
    }
}

fn write_history(list: &[HistoryEntry]) -> Result<(), String> {
    let _ = std::fs::create_dir_all(config_dir());
    let txt = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    std::fs::write(history_path(), txt).map_err(|e| e.to_string())
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// Bijwerken op uuid, nooit verwijderen. `created` van de eerste keer blijft staan;
// dat is wat "wanneer begon dit werk" betekent.
fn merge_history(mut list: Vec<HistoryEntry>, mut e: HistoryEntry, now: u64) -> Vec<HistoryEntry> {
    if e.uuid.trim().is_empty() {
        return list;
    }
    e.last_seen = now;
    match list.iter_mut().find(|x| x.uuid == e.uuid) {
        Some(old) => {
            e.created = if old.created == 0 { now } else { old.created };
            *old = e;
        }
        None => {
            if e.created == 0 {
                e.created = now;
            }
            list.push(e);
        }
    }
    // Nieuwste bovenaan: dat is de volgorde waarin je ernaar zoekt.
    list.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
    list
}

#[tauri::command]
fn session_history() -> Vec<HistoryEntry> {
    read_history()
}

#[tauri::command]
fn history_record(entry: HistoryEntry) -> Result<(), String> {
    let list = merge_history(read_history(), entry, now_secs());
    write_history(&list)
}

// Welke sessies stonden er open toen Taurus sloot. Apart van `history_record`,
// want dit is een eigenschap van het MOMENT en niet van de sessie: hij wordt in
// één keer voor de hele lijst gezet, zodat een tab die je sluit ook echt afvalt.
#[tauri::command]
fn history_mark_open(uuids: Vec<String>) -> Result<(), String> {
    let mut list = read_history();
    for e in list.iter_mut() {
        e.was_open = uuids.iter().any(|u| u == &e.uuid);
    }
    write_history(&list)
}

// Met de hand vergeten. De enige manier waarop hier iets uit verdwijnt -- niet
// omdat een herstart niet lukte.
#[tauri::command]
fn history_forget(uuid: String) -> Result<Vec<HistoryEntry>, String> {
    let mut list = read_history();
    list.retain(|e| e.uuid != uuid);
    write_history(&list)?;
    Ok(list)
}

// Pad waar Claude Code het transcript van een sessie bewaart:
// %USERPROFILE%\.claude\projects\<map-encoded>\<uuid>.jsonl
// De map-encoding vervangt elk niet-alfanumeriek teken door '-'
// (bv. C:\Users\AST -> C--Users-AST).
fn claude_session_file(path: &str, uuid: &str) -> std::path::PathBuf {
    let enc: String = path
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    std::path::PathBuf::from(home)
        .join(".claude")
        .join("projects")
        .join(enc)
        .join(format!("{}.jsonl", uuid))
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionState {
    exists: bool,
    age_secs: u64,
}

// Bestaat het transcript nog, en hoe oud (seconden sinds laatste wijziging)?
// Claude ruimt oude sessies zelf op; ontbreekt het bestand -> niet herstartbaar,
// dan proberen we het bij het opstarten niet eens.
#[tauri::command]
fn session_state(path: String, uuid: String) -> SessionState {
    let f = claude_session_file(&path, &uuid);
    match std::fs::metadata(&f) {
        Ok(meta) => {
            let age = meta
                .modified()
                .ok()
                .and_then(|t| t.elapsed().ok())
                .map(|d| d.as_secs())
                .unwrap_or(u64::MAX);
            SessionState {
                exists: true,
                age_secs: age,
            }
        }
        Err(_) => SessionState {
            exists: false,
            age_secs: 0,
        },
    }
}

// Welk uitvoerbaar bestand hoort bij deze agent? Leeg/"claude" -> claude.exe,
// "agy" -> agy.exe (de Gemini-agent-CLI).
fn agent_exe(agent: &str) -> &'static str {
    match agent {
        "agy" => "agy.exe",
        _ => "claude.exe",
    }
}

// Zoek <base>.exe, dan .cmd, dan .bat in de opgegeven PATH-string (PATHEXT-
// volgorde: een native exe wint altijd van een shim). Een npm-installatie van
// Claude Code zet alleen een claude.cmd op PATH (#40); CreateProcess kan een
// .cmd/.bat niet direct starten, dus die komt terug als cmd.exe + "/c <pad>"-
// prefix waar de agent-args achteraan komen.
fn resolve_in_paths(base: &str, paths: &str) -> Option<(String, Vec<String>)> {
    for ext in ["exe", "cmd", "bat"] {
        for p in std::env::split_paths(paths) {
            let cand = p.join(format!("{}.{}", base, ext));
            if cand.is_file() {
                let full = cand.to_string_lossy().into_owned();
                return Some(if ext == "exe" {
                    (full, Vec::new())
                } else {
                    ("cmd.exe".to_string(), vec!["/c".into(), full])
                });
            }
        }
    }
    None
}

// Zoek het programma van de agent via PATH, met fallback naar de kale exe-naam.
// Geeft (programma, prefix-args): leeg voor een exe, "/c <pad>" voor een shim.
fn resolve_program(agent: &str) -> (String, Vec<String>) {
    let exe = agent_exe(agent);
    let base = exe.trim_end_matches(".exe");
    if let Ok(paths) = std::env::var("PATH") {
        if let Some(hit) = resolve_in_paths(base, &paths) {
            return hit;
        }
    }
    (exe.to_string(), Vec::new())
}

// Welk subcommando somt de modellen van deze agent op? Alleen agy heeft er een
// (`agy models`, één label per regel). claude heeft het niet nodig: daar wijzen
// de aliassen (fable/opus/sonnet/haiku) altijd naar het nieuwste model (#92).
fn model_list_subcommand(agent: &str) -> Option<&'static str> {
    match agent {
        "agy" => Some("models"),
        _ => None,
    }
}

// Zet de stdout van het list-commando om in modelnamen. Puur, dus testbaar:
// trimmen, lege regels en CR weg, ontdubbelen met behoud van volgorde (de CLI
// zet het nieuwste bovenaan), en een plafond zodat onverwachte uitvoer -- een
// hulptekst of een foutmelding op stdout -- de datalist niet volspamt.
fn parse_model_list(stdout: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let name = line.trim();
        // Modelnamen zijn korte labels; alles daarbuiten is geen modelregel.
        if name.is_empty() || name.len() > 120 || out.iter().any(|s| s == name) {
            continue;
        }
        out.push(name.to_string());
        if out.len() == 64 {
            break;
        }
    }
    out
}

// Vraag de agent-CLI welke modellen beschikbaar zijn, zodat een nieuw model in
// de suggestielijst verschijnt zonder dat Taurus mee hoeft te updaten (#92).
// Faalt dit (agent niet geïnstalleerd, oudere CLI zonder `models`, timeout),
// dan valt de frontend terug op zijn ingebouwde lijst.
#[tauri::command]
fn list_agent_models(agent: String) -> Result<Vec<String>, String> {
    let sub = model_list_subcommand(&agent).ok_or("agent has no model list command")?;
    // Zelfde programma-resolutie als bij het starten van een sessie, dus de
    // cmd.exe-shim voor een npm-installatie werkt hier ook (#40).
    let (program, mut args) = resolve_program(&agent);
    args.push(sub.to_string());
    let mut c = std::process::Command::new(&program);
    c.args(&args)
        // Geen stdin: een CLI die onverhoopt om invoer vraagt hangt anders tot
        // de timeout i.p.v. meteen af te breken.
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut child = c
        .spawn()
        .map_err(|e| format!("{} {}: {}", agent, sub, e))?;
    // Wachten met plafond: de uitvoer is een handvol regels, dus de pipe loopt
    // niet vol en pollen op try_wait volstaat.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("{} {} timed out", agent, sub));
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(e) => return Err(format!("{} {}: {}", agent, sub, e)),
        }
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("{} {}: {}", agent, sub, e))?;
    if !out.status.success() {
        return Err(format!("{} {} exited with {}", agent, sub, out.status));
    }
    let models = parse_model_list(&String::from_utf8_lossy(&out.stdout));
    if models.is_empty() {
        return Err(format!("{} {} returned no models", agent, sub));
    }
    Ok(models)
}

// Windows Job Object met kill-on-close rond het agent-proces. child.kill() is
// TerminateProcess op alleen het directe kind; door de agent aan een job te
// hangen ruimt Windows de HELE boom (claude + gespawnde MCP-servers) op zodra
// de laatste job-handle sluit — ook bij een crash van Taurus zelf (#77).
#[cfg(windows)]
mod job {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
        SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE};

    pub struct Job(HANDLE);
    // HANDLEs zijn gewoon over threads te dragen; alleen de wrapper mist de markers.
    unsafe impl Send for Job {}
    unsafe impl Sync for Job {}
    impl Drop for Job {
        fn drop(&mut self) {
            // Laatste handle dicht -> kill-on-close ruimt de procesboom op.
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }

    // Best effort: faalt de toewijzing (bv. proces al weg), dan gedraagt de
    // sessie zich als voorheen (alleen child.kill op het directe kind).
    pub fn assign(pid: u32) -> Option<Job> {
        unsafe {
            let hjob = CreateJobObjectW(None, None).ok()?;
            let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            if SetInformationJobObject(
                hjob,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const core::ffi::c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
            .is_err()
            {
                let _ = CloseHandle(hjob);
                return None;
            }
            let hproc = match OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid) {
                Ok(h) => h,
                Err(_) => {
                    let _ = CloseHandle(hjob);
                    return None;
                }
            };
            let ok = AssignProcessToJobObject(hjob, hproc).is_ok();
            let _ = CloseHandle(hproc);
            if ok {
                Some(Job(hjob))
            } else {
                let _ = CloseHandle(hjob);
                None
            }
        }
    }
}

// Een actieve terminal-sessie: de PTY-master (voor resize), de writer (stdin)
// en het kindproces (om te kunnen afsluiten). De job-handle (kill-on-close)
// neemt bij drop de hele procesboom mee.
struct Session {
    writer: Box<dyn Write + Send>,
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    #[cfg(windows)]
    _job: Option<job::Job>,
}

// ---------- een lokale sessie ook naar buiten laten meelezen (#125) ----------
//
// De vraagmodus belooft dat het werk in de sessie blijft van degene die vastloopt:
// je kijkt mee in ZIJN terminal, je krijgt er geen tweede naast. Daarvoor moet de
// uitvoer van een lokale sessie naar een tweede lezer kunnen -- precies wat #121's
// join al doet, maar dan de andere kant op.
//
// Een globale registry en geen veld op `Session`, omdat de lees-thread in start_pty
// alleen een geleende `&Mutex<..>` heeft en die niet kan vasthouden. De sleutel is
// het sessie-id; een sessie zonder abonnee kost hier niets.
type OfferSink = std::sync::mpsc::Sender<Vec<u8>>;
static OFFERS: std::sync::OnceLock<Mutex<HashMap<String, Vec<OfferSink>>>> =
    std::sync::OnceLock::new();

fn offers() -> &'static Mutex<HashMap<String, Vec<OfferSink>>> {
    OFFERS.get_or_init(|| Mutex::new(HashMap::new()))
}

// Dezelfde bytes als naar het venster gaan, ook naar wie meeleest. Weggevallen
// abonnees ruimen we hier op: een dichte channel is het einde van dat meekijken en
// geen reden om de sessie zelf iets te laten merken.
fn fan_out(id: &str, data: &[u8]) {
    let mut map = offers().lock().unwrap();
    let Some(list) = map.get_mut(id) else { return };
    list.retain(|tx| tx.send(data.to_vec()).is_ok());
    if list.is_empty() {
        map.remove(id);
    }
}

fn offer_subscribe(id: &str) -> std::sync::mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = std::sync::mpsc::channel();
    offers().lock().unwrap().entry(id.to_string()).or_default().push(tx);
    rx
}

struct AppState {
    sessions: Mutex<HashMap<String, Session>>,
    // STT-opname: commando-kanaal naar de audio-thread + zichtbare status.
    stt: SttState,
    // Inkomende SSH: listener, wachtende popups en draaiende sessies (#121).
    ssh: std::sync::Arc<sshhost::HostState>,
    // Aankondigen en zoeken op het vertrouwde netwerk (#125).
    discovery: std::sync::Arc<discovery::Discovery>,
    // De openstaande hulpvraag, als die er is. Eén tegelijk: een hand die je
    // opsteekt wijst naar één agent, en twee handen tegelijk is geen vraag maar
    // ruis op het netwerk.
    asking: Mutex<Option<HelpRequest>>,
}

// Start een claude-proces in een ConPTY en registreer de sessie onder `id`.
// `gen` is de generatieteller van de frontend: pty-output/pty-exit dragen hem
// mee, zodat een VERLAAT event van een gekild proces (herstart hergebruikt het
// id) nooit de nieuwe incarnatie als "beëindigd" kan markeren (#71).
fn start_pty(
    app: &AppHandle,
    sessions: &Mutex<HashMap<String, Session>>,
    id: String,
    gen: u64,
    program: String,
    path: &str,
    args: Vec<String>,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    if !Path::new(path).is_dir() {
        return Err(format!("Map bestaat niet of is niet bereikbaar:\n{}", path));
    }

    let pty_system = NativePtySystem::default();
    let pair = pty_system
        .openpty(PtySize {
            rows: rows.max(1),
            cols: cols.max(1),
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("openpty: {}", e))?;

    let mut cmd = CommandBuilder::new(program);
    for a in &args {
        cmd.arg(a);
    }
    cmd.cwd(path);
    // Erf de omgeving van het werkstation (PATH, USERPROFILE, APPDATA ...) zodat
    // claude zijn config/credentials vindt -- maar niet de variabelen waarmee een
    // agent zijn EIGEN sessie beschrijft (#101).
    let from_agent = std::env::var("CLAUDECODE").is_ok();
    for (k, v) in std::env::vars() {
        if inherits_session_marker(&k, from_agent) {
            // LET OP: overslaan is hier niet genoeg. CommandBuilder::new() vult
            // zichzelf met get_base_env(), dus de variabele staat er al in en
            // blijft staan als we hem alleen niet opnieuw zetten. Hij moet er
            // expliciet uit.
            cmd.env_remove(&k);
            continue;
        }
        cmd.env(k, v);
    }

    let child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("kon claude niet starten: {}", e))?;

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("reader: {}", e))?;
    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("writer: {}", e))?;

    let app2 = app.clone();
    let id2 = id.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    // Wie meeleest krijgt dezelfde bytes (#125). Eerst, zodat een
                    // trage abonnee het venster niet ophoudt -- send() op een
                    // channel blokkeert niet.
                    fan_out(&id2, &buf[..n]);
                    // Base64 i.p.v. Vec<u8>: Tauri serialiseert events als JSON,
                    // en een byte-array wordt dan een array van getallen (3-4x
                    // zo groot + parse-kosten) op het heetste pad van de app (#73).
                    if app2.emit("pty-output", (id2.clone(), gen, b64(&buf[..n]))).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        let _ = app2.emit("pty-exit", (id2.clone(), gen));
    });

    // Agent aan een Job Object hangen zodat de hele procesboom opgeruimd
    // wordt bij close/restart/crash (#77). Best effort: zonder PID of bij een
    // mislukte toewijzing draait de sessie gewoon zonder job.
    #[cfg(windows)]
    let job_handle = child.process_id().and_then(job::assign);

    sessions.lock().unwrap().insert(
        id,
        Session {
            writer,
            master: pair.master,
            child,
            #[cfg(windows)]
            _job: job_handle,
        },
    );
    Ok(())
}

// Beschrijft deze variabele de sessie van de agent die TAURUS startte? Zo ja,
// hoort hij niet mee naar een nieuwe agent: die is geen kindsessie maar een
// zelfstandige sessie. Erven we ze toch, dan zet claude zijn transcript uit
// ("inherited CLAUDE_CODE_CHILD_SESSION marker") en werkt --resume niet meer --
// zonder dat iets uitlegt waarom (#101).
//
// NO_COLOR alleen weglaten als de omgeving van een agent komt: los is het een
// legitieme voorkeur van de gebruiker, maar naast CLAUDECODE is het gezet voor
// een subproces van een agent, en Taurus levert juist een kleurenterminal.
fn inherits_session_marker(key: &str, from_agent: bool) -> bool {
    matches!(
        key,
        "CLAUDECODE"
            | "CLAUDE_CODE_CHILD_SESSION"
            | "CLAUDE_CODE_SESSION_ID"
            | "CLAUDE_CODE_ENTRYPOINT"
            | "CLAUDE_PID"
    ) || (from_agent && key == "NO_COLOR")
}

fn norm_title(title: &str) -> String {
    if title.trim().is_empty() { "agent".into() } else { title.trim().into() }
}

// Instructie die we (optioneel) aan claude meegeven zodat hij altijd volledige
// paden toont -> die zijn dan klikbaar in de HTML-preview.
const FULL_PATH_PROMPT: &str = "When you create, write, save, or reference any file, always print its full absolute Windows path (for example C:\\Users\\you\\dir\\file.html), not just the file name, so it can be opened directly.";

// Start een verse sessie of hervat een bestaande? Bepaalt welke vlaggen per agent
// gebruikt worden (claude --session-id vs --resume; agy verse start vs --continue).
enum LaunchKind {
    Create,
    Resume,
}

// Splits een commando-override in tokens; dubbele quotes bewaren spaties, zodat
// ook "C:\Program Files\tool.exe" --flag "een arg" werkt. split_whitespace
// brak elk programma-pad met een spatie (#76).
fn split_command(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    for c in s.chars() {
        match c {
            '"' => in_q = !in_q,
            c if c.is_whitespace() && !in_q => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

// Commando-override (bijv. nep-Claude voor de demo) -> (programma, argumenten).
// Gedeeld door create_session en restart_session.
fn parse_override(command: &str) -> Result<(String, Vec<String>), String> {
    let mut toks = split_command(command);
    if toks.is_empty() {
        return Err("commando-override is leeg".into());
    }
    let prog = toks.remove(0);
    Ok((prog, toks))
}

// Bouw (programma, argumenten) voor de gekozen agent. De `command`-escape-hatch
// wordt door de aanroeper afgehandeld; hier gaat het puur om claude/agy. De twee
// CLIs verschillen sterk in vlaggen, dus we bouwen ze apart op i.p.v. Claude's
// vlaggen voor beide aan te nemen.
// Hoe heet de agent-binary OP EEN ANDERE MACHINE? Daar mag niets van dit
// werkstation in meeliften: resolve_program levert een absoluut lokaal pad
// (C:\Users\<mij>\.local\bin\claude.exe), en dat bestaat op de doelhost niet --
// die heeft zijn eigen gebruiker en zijn eigen installatie. De host zoekt de
// naam zelf op via zijn PATH. Op Windows expliciet met .exe, want de agent start
// het programma via CreateProcess en die vult PATHEXT niet aan.
fn remote_agent_program(agent: &str, os: &str) -> String {
    let base = match agent {
        "agy" => "agy",
        _ => "claude",
    };
    if os == "windows" {
        format!("{}.exe", base)
    } else {
        base.to_string()
    }
}

fn build_command(
    agent: &str,
    kind: LaunchKind,
    session_id: &str,
    title: &str,
    task: &str,
    mode: &str,
    model: &str,
    full_paths: bool,
    // Some(os) = dit commando draait op een remote host, niet hier.
    remote_os: Option<&str>,
) -> (String, Vec<String>) {
    // `a` start met de eventuele cmd.exe-shim-prefix ("/c <pad>", #40); de
    // agent-vlaggen komen daarachter. Remote is er geen shim: daar bestaat het
    // lokale pad niet en resolvet de host zelf.
    let (program, mut a) = match remote_os {
        Some(os) => (remote_agent_program(agent, os), Vec::new()),
        None => resolve_program(agent),
    };
    match agent {
        // agy (Antigravity/Gemini-agent): kent geen --session-id / -n /
        // --permission-mode / --append-system-prompt. Modus: auto -> alle tools
        // auto-goedkeuren; sandbox -> beperkte terminalrechten. full_paths heeft
        // geen equivalent en wordt overgeslagen. Het model is de volledige
        // agy-label-string (bijv. "Gemini 3.5 Flash (Medium)").
        // LET OP (#81): --continue hervat het LAATSTE gesprek in de werkmap,
        // niet een specifieke sessie (agy heeft geen sessie-id's). Meerdere
        // bewaarde agy-sessies in dezelfde map hervatten dus allemaal
        // hetzelfde (meest recente) gesprek.
        "agy" => {
            if let LaunchKind::Resume = kind {
                a.push("--continue".into());
            }
            if !model.trim().is_empty() {
                a.push("--model".into());
                a.push(model.trim().into());
            }
            match mode {
                "auto" => a.push("--dangerously-skip-permissions".into()),
                // "plan" blijft als alias voor sandbox staan (oudere configs).
                "sandbox" | "plan" => a.push("--sandbox".into()),
                _ => {}
            }
            // Een taak start agy interactief met die prompt -- alleen bij een
            // verse start; bij --continue zou een losse prompt het hervatten van
            // het gesprek verstoren.
            if let LaunchKind::Create = kind {
                if !task.trim().is_empty() {
                    a.push("--prompt-interactive".into());
                    a.push(task.trim().into());
                }
            }
        }
        // claude (default): ongewijzigde vlaggen, plus --model wanneer gezet.
        _ => {
            match kind {
                LaunchKind::Create => {
                    a.push("--session-id".into());
                    a.push(session_id.into());
                }
                LaunchKind::Resume => {
                    a.push("--resume".into());
                    a.push(session_id.into());
                }
            }
            a.push("-n".into());
            a.push(norm_title(title));
            if !mode.is_empty() && mode != "default" {
                a.push("--permission-mode".into());
                a.push(mode.into());
            }
            if !model.trim().is_empty() {
                a.push("--model".into());
                a.push(model.trim().into());
            }
            if full_paths {
                a.push("--append-system-prompt".into());
                a.push(FULL_PATH_PROMPT.into());
            }
            // Taak alleen bij een verse start meesturen; --resume hervat het gesprek.
            if let LaunchKind::Create = kind {
                if !task.trim().is_empty() {
                    a.push(task.trim().into());
                }
            }
        }
    }
    (program, a)
}

// ---------- remote uitvoering (#98) ----------
//
// Een remote tab is een gewone tab waarin het programma toevallig ssh.exe heet:
// start_pty spawnt elk programma in een ConPTY, dus de Session-struct, resize,
// write, het Job Object en het (id, gen, base64)-eventcontract blijven precies
// hetzelfde. wrap_remote wikkelt om de UITKOMST van parse_override/build_command
// heen, niet ernaast -- daardoor werkt remote met beide agents en met een
// commando-override, zonder een derde stille uitzondering (#93).

// Waar de agent-binary op de host staat. Relatief aan de home-map, want dat is
// de werkmap waarin sshd een commando start -- op Linux en Windows allebei. Dat
// scheelt het expanderen van ~ of %USERPROFILE% in een vreemde shell.
const AGENT_REL_UNIX: &str = ".taurus/bin/taurus-agent";
const AGENT_REL_WIN: &str = ".taurus\\bin\\taurus-agent.exe";

// Waar de private key vandaan komt. Vandaag alleen een pad of "laat ssh kiezen";
// een vault-bron (bw serve) wordt hier later een derde variant, zodat die keuze
// deze ene functie raakt en niet de hele launch-keten.
#[derive(Debug, PartialEq)]
enum KeySource {
    Path(String),
    // Geen -i meesturen: ssh valt terug op ~/.ssh/config en de agent.
    SshConfig,
}

fn resolve_key(host: &Host) -> Result<KeySource, String> {
    let raw = host.key_path.trim();
    if raw.is_empty() {
        return Ok(KeySource::SshConfig);
    }
    let expanded = expand_home(raw);
    if !Path::new(&expanded).is_file() {
        return Err(format!("SSH-key niet gevonden:\n{}", expanded));
    }
    Ok(KeySource::Path(expanded))
}

// ~ of ~/ aan het begin -> %USERPROFILE%. Het issue toont "~/.ssh/id_ed25519"
// als voorbeeld, en dat moet op Windows gewoon werken.
fn expand_home(p: &str) -> String {
    if p == "~" || p.starts_with("~/") || p.starts_with("~\\") {
        if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
            return format!("{}{}", home, &p[1..]);
        }
    }
    p.to_string()
}

// POSIX-shellquoting: alles tussen enkele quotes, en een enkele quote zelf wordt
// afgesloten-geescaped-heropend. Nodig omdat ssh de payload als EEN string aan
// de remote shell geeft, die hem opnieuw parseert -- een taak met een spatie of
// een apostrof zou anders in losse argumenten uiteenvallen.
fn shell_quote_posix(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

// Hoe een POSIX-payload op de host gestart wordt. Twee dingen zitten hierin die
// allebei uit een echte fout komen:
//
// 1. Het script gaat naar een BESTAND en start daarna met exec, niet via een
//    pipe (`... | sh`). Met een pipe krijgt die shell zijn stdin van de pipe in
//    plaats van de terminal, en dan weigert tmux met "open terminal failed: not
//    a terminal". Na de pipeline erft exec de stdin van de buitenste sh -- de
//    pty van ssh -t.
// 2. `sh -l`, een LOGIN shell. `ssh host "commando"` draait geen login shell,
//    dus ~/.profile wordt niet gelezen en ~/.local/bin staat niet op PATH. Een
//    claude die daar geinstalleerd is, wordt dan niet gevonden: de sessie
//    eindigt meteen met alleen "[exited]" en niets legt uit waarom. Met -l
//    krijgt de agent de omgeving die de gebruiker op die machine heeft
//    (PATH, nvm, pyenv).
fn posix_launcher(session: &str, inner: &str) -> String {
    format!(
        "echo {} | base64 -d > /tmp/{}.sh && exec sh -l /tmp/{}.sh",
        b64(inner.as_bytes()),
        session,
        session
    )
}

// PowerShell-quoting: alles tussen enkele quotes, en een enkele quote wordt
// verdubbeld. Binnen '...' expandeert PowerShell niets, dus $ en ` zijn veilig.
fn ps_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

// UTF-16LE, want dat verwacht powershell -EncodedCommand.
fn utf16le(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect()
}

// Kleine FNV-1a; genoeg om afgekapte paden uit elkaar te houden.
fn fnv1a(s: &str) -> u32 {
    let mut h: u32 = 0x811c9dc5;
    for b in s.as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h
}

// Deterministische sessienaam per (host, project): dezelfde combinatie moet
// dezelfde naam opleveren, anders haakt Taurus niet aan maar start hij een
// tweede agent in dezelfde map. Alleen [a-z0-9-], want tmux gebruikt '.' en ':'
// als scheidingstekens in target-namen. Het pad wordt afgekapt maar krijgt een
// hash van het VOLLEDIGE pad mee, zodat twee lange paden met hetzelfde begin
// niet stil op dezelfde sessie uitkomen.
fn mux_session_name(host_id: &str, project_path: &str) -> String {
    fn slug(s: &str, max: usize) -> String {
        let mut out = String::new();
        let mut last_dash = true; // voorkomt een leidend streepje
        for c in s.chars() {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_alphanumeric() {
                out.push(c);
                last_dash = false;
            } else if !last_dash {
                out.push('-');
                last_dash = true;
            }
            if out.len() >= max {
                break;
            }
        }
        while out.ends_with('-') {
            out.pop();
        }
        out
    }
    format!(
        "taurus-{}-{}-{:08x}",
        slug(host_id, 16),
        slug(project_path, 24),
        fnv1a(project_path)
    )
}

// Herdr kent geen attach-or-create in een commando zoals `tmux new -A -s`:
// `workspace create` is niet idempotent (tweemaal hetzelfde label levert twee
// workspaces op) en workspace-id's worden niet hergebruikt, dus na sluiten van
// w1 heet de volgende w2. Terugvinden op label zou betekenen dat een shell-
// payload JSON moet parsen, en dat wil je niet.
//
// Wat herdr wel heeft is een SESSIE: een eigen namespace met een eigen server en
// socket. Een eigen sessie per (host, project) -- dezelfde eenheid die tmux hier
// al gebruikt -- maakt de eerste pane deterministisch w1:p1. Daarmee is de hele
// handshake een reeks bestaanschecks op exitcode, zonder een teken JSON:
//
//   server draait?   zo niet: losgekoppeld starten en wachten tot de socket er is
//   pane bestaat?    zo niet: workspace aanmaken met de juiste werkmap
//   agent draait?    zo niet: het agent-commando in die pane starten
//   attach
//
// Elke stap is los idempotent, dus een half opgezette sessie (server leeft nog,
// agent gestopt) herstelt zichzelf bij de volgende start in plaats van een
// tweede agent naast de eerste te zetten.
const HERDR_PANE: &str = "w1:p1";

// Waar herdr staat als hij niet op PATH staat. De probe draait geen login shell,
// en op Windows zet de installer alleen de user-PATH voor NIEUWE sessies.
const HERDR_POSIX_FALLBACK: &str = "$HOME/.local/bin/herdr";
const HERDR_WIN_FALLBACK: &str = r"Programs\Herdr\bin\herdr.exe";

fn herdr_posix_script(session: &str, cwd: &str, program: &str, args: &[String]) -> String {
    // Twee quoting-lagen: eerst de commandoregel zoals de shell IN de pane hem
    // moet lezen, daarna nog een keer omdat herdr die regel als EEN argument krijgt.
    let mut inner = shell_quote_posix(program);
    for a in args {
        inner.push(' ');
        inner.push_str(&shell_quote_posix(a));
    }
    let s = shell_quote_posix(session);
    // setsid bestaat niet op macOS; nohup alleen is daar genoeg om SIGHUP bij het
    // sluiten van de ssh-verbinding te overleven.
    [
        "H=herdr".to_string(),
        format!("command -v herdr >/dev/null 2>&1 || H=\"{}\"", HERDR_POSIX_FALLBACK),
        format!("run() {{ \"$H\" --session {} \"$@\"; }}", s),
        "if ! run status 2>/dev/null | grep -q 'status: running'; then".to_string(),
        format!(
            "  if command -v setsid >/dev/null 2>&1; then setsid nohup \"$H\" --session {s} server >/dev/null 2>&1 </dev/null & else nohup \"$H\" --session {s} server >/dev/null 2>&1 </dev/null & fi",
            s = s
        ),
        "  i=0".to_string(),
        "  while [ $i -lt 20 ]; do run status 2>/dev/null | grep -q 'status: running' && break; i=$((i+1)); sleep 1; done".to_string(),
        "fi".to_string(),
        format!(
            "run pane get {p} >/dev/null 2>&1 || run workspace create --cwd {c} --label {s} --no-focus >/dev/null",
            p = HERDR_PANE,
            c = shell_quote_posix(cwd),
            s = s
        ),
        format!(
            "if ! run agent get {p} >/dev/null 2>&1; then",
            p = HERDR_PANE
        ),
        format!(
            "  run pane run {p} {cmd} >/dev/null",
            p = HERDR_PANE,
            cmd = shell_quote_posix(&inner)
        ),
        // Gemeten: herdr herkent een agent ongeveer een seconde na de start --
        // hij moet eerst iets getekend hebben. Zonder deze lus komt attach
        // milliseconden te vroeg en faalt hij met "agent target not found".
        format!(
            "  i=0; while [ $i -lt 8 ]; do run agent get {p} >/dev/null 2>&1 && break; i=$((i+1)); sleep 1; done",
            p = HERDR_PANE
        ),
        "fi".to_string(),
        // Rechtstreeks aan de agent-terminal hangen geeft een kale tab, zonder
        // herdr's eigen tabbalk. Herkent herdr het programma niet (agy, of een
        // command-override), dan is er geen agent om aan te hangen en is de
        // sessie-TUI de terugval -- een werkende tab met wat randwerk eromheen
        // is beter dan een tab die niet opent.
        format!(
            "if run agent get {p} >/dev/null 2>&1; then exec \"$H\" --session {s} agent attach {p}; fi",
            p = HERDR_PANE,
            s = s
        ),
        format!("exec \"$H\" --session {}", s),
    ]
    .join("\n")
}

fn herdr_windows_script(session: &str, cwd: &str, program: &str, args: &[String]) -> String {
    // In de pane draait PowerShell, dus dezelfde vorm als de kale Windows-start:
    // de call-operator, want een gequote pad is anders geen commando.
    let mut inner = format!("& {}", ps_quote(program));
    for a in args {
        inner.push(' ');
        inner.push_str(&ps_quote(a));
    }
    [
        "$h = (Get-Command herdr -EA 0).Source".to_string(),
        format!(
            "if (-not $h) {{ $h = Join-Path $env:LOCALAPPDATA '{}' }}",
            HERDR_WIN_FALLBACK
        ),
        "if (-not (Test-Path $h)) { Write-Host 'herdr staat niet op deze machine'; exit 1 }".to_string(),
        format!("$s = {}", ps_quote(session)),
        "if (-not (& $h --session $s status 2>$null | Select-String 'status: running')) {".to_string(),
        // Gemeten: een server die je met Start-Process vanuit een sshd-sessie start,
        // wordt gekild zodra die verbinding sluit. Win32_Process.Create herparent
        // hem buiten de sessie, en dan overleeft hij het wel.
        // ShowWindow = 0 (SW_HIDE): zonder dit verschijnt er een consolevenster
        // op het BUREAUBLAD van de host. Via sshd viel dat niet op -- die sessie
        // is niet de interactieve desktop -- maar een Taurus-host (#121) draait
        // wel in de sessie van de ingelogde gebruiker, en dan staat er ineens
        // een zwart venster over zijn werk heen.
        "  $si = New-CimInstance -ClassName Win32_ProcessStartup -ClientOnly -Property @{ ShowWindow = [uint16]0 }".to_string(),
        "  Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{ CommandLine = ('\"' + $h + '\" --session ' + $s + ' server'); ProcessStartupInformation = $si } | Out-Null".to_string(),
        "  for ($i = 0; $i -lt 20; $i++) { if (& $h --session $s status 2>$null | Select-String 'status: running') { break }; Start-Sleep -Seconds 1 }".to_string(),
        "}".to_string(),
        format!("& $h --session $s pane get {} 2>$null | Out-Null", HERDR_PANE),
        format!(
            "if ($LASTEXITCODE -ne 0) {{ & $h --session $s workspace create --cwd {} --label $s --no-focus | Out-Null }}",
            ps_quote(cwd)
        ),
        format!("& $h --session $s agent get {} 2>$null | Out-Null", HERDR_PANE),
        format!(
            "if ($LASTEXITCODE -ne 0) {{ & $h --session $s pane run {} {} | Out-Null }}",
            HERDR_PANE,
            ps_quote(&inner)
        ),
        // Gemeten tegen herdr 0.8.0-preview: `agent attach` antwoordt op Windows
        // met "direct terminal attach is not supported on Windows yet". Daar is
        // de sessie-TUI de enige manier om aan te haken -- dat kost een tabbalk
        // in beeld, die de host desgewenst kan wegzetten met
        // hide_tab_bar_when_single_tab in zijn eigen config.toml. Op POSIX
        // hangen we wel rechtstreeks aan de agent en is die balk er niet.
        "& $h --session $s".to_string(),
    ]
    .join("\n")
}

// De opdracht voor de taurus-agent gaat als base64-JSON mee in plaats van als
// shell-argumenten. Dat haalt de remote shell volledig uit de keten: base64 is
// [A-Za-z0-9+/=] en overleeft zowel /bin/sh als cmd.exe zonder quoting. Zonder
// dit zou een taak met een " of een & op een Windows-host de aanroep breken.
fn agent_payload_b64(cwd: &str, program: &str, args: &[String]) -> String {
    let json = serde_json::json!({ "cwd": cwd, "program": program, "args": args });
    b64(json.to_string().as_bytes())
}

// De string die de remote shell te zien krijgt.
// `via: wsl` bouwt de Linux-payload en zet die in WSL. De payload gaat als
// base64 door cmd.exe: hij bevat quotes en pipes, en die zouden er onderweg
// anders uit gehaald worden. Deze exacte vorm is tegen een echte host getest.
fn build_remote_payload_via(
    mux: &str,
    os: &str,
    via: &str,
    session: &str,
    cwd: &str,
    program: &str,
    args: &[String],
) -> Result<String, String> {
    if via == "wsl" {
        let inner = build_remote_payload_inner(mux, "linux", session, cwd, program, args)?;
        return Ok(format!("wsl -e sh -c \"{}\"", posix_launcher(session, &inner)));
    }
    // Een Linux-host rechtstreeks: dezelfde login-shell-val, dus dezelfde
    // starter. Windows loopt via PowerShell en heeft dit niet nodig.
    if os == "linux" {
        let inner = build_remote_payload_inner(mux, os, session, cwd, program, args)?;
        return Ok(posix_launcher(session, &inner));
    }
    build_remote_payload_inner(mux, os, session, cwd, program, args)
}

// De kale payload, zonder de starter eromheen.
fn build_remote_payload_inner(
    mux: &str,
    os: &str,
    session: &str,
    cwd: &str,
    program: &str,
    args: &[String],
) -> Result<String, String> {
    match mux {
        // Herdr: eigen sessie per (host, project), zie de toelichting bij
        // HERDR_PANE. Werkt op Windows, Linux en macOS met hetzelfde model --
        // dit is de enige waarde die heraanhaken op een Windows-host geeft
        // zonder de omweg via WSL.
        "herdr" => {
            if os == "windows" {
                let script = herdr_windows_script(session, cwd, program, args);
                Ok(format!(
                    "powershell -NoProfile -EncodedCommand {}",
                    b64(&utf16le(&script))
                ))
            } else {
                Ok(herdr_posix_script(session, cwd, program, args))
            }
        }
        // Eigen agent: geen shellquoting nodig, en hij bestaat op beide OS'en.
        "taurus-agent" => {
            let bin = if os == "windows" { AGENT_REL_WIN } else { AGENT_REL_UNIX };
            Ok(format!(
                "{} run -s {} --b64 {}",
                bin,
                session,
                agent_payload_b64(cwd, program, args)
            ))
        }
        // tmux (en psmux, dat dezelfde commandotaal spreekt). -A = aanhaken als
        // de sessie bestaat, anders aanmaken; -c geldt alleen bij aanmaken, dus
        // heraanhaken laat de bestaande werkmap met rust.
        "tmux" | "psmux" => {
            if os == "windows" && mux == "tmux" {
                return Err("tmux bestaat niet op Windows; kies taurus-agent of psmux".into());
            }
            let mut s = format!(
                "{} new-session -A -s {} -c {} --",
                mux,
                shell_quote_posix(session),
                shell_quote_posix(cwd)
            );
            s.push(' ');
            s.push_str(&shell_quote_posix(program));
            for a in args {
                s.push(' ');
                s.push_str(&shell_quote_posix(a));
            }
            Ok(s)
        }
        // Geen multiplexer: kaal starten. Er is dan geen scrollback en geen
        // heraanhaken; of de sessie een verbindingsbreuk overleeft hangt van de
        // host af. Gemeten: Windows' sshd kapt de procesboom NIET af, dus daar
        // loopt de agent door; op Linux sterft hij zonder tmux wel, en is het
        // transcript (claude --resume) de enige persistentie.
        "" | "none" => {
            if os == "windows" {
                // De remote shell is cmd.exe, en een taak met " of & zou de
                // aanroep slopen. -EncodedCommand neemt base64 UTF-16LE, dus
                // [A-Za-z0-9+/=]: cmd.exe ziet geen enkel metateken meer.
                let mut script = format!("Set-Location {}; & {}", ps_quote(cwd), ps_quote(program));
                for a in args {
                    script.push(' ');
                    script.push_str(&ps_quote(a));
                }
                return Ok(format!(
                    "powershell -NoProfile -EncodedCommand {}",
                    b64(&utf16le(&script))
                ));
            }
            let mut s = format!("cd {} && exec {}", shell_quote_posix(cwd), shell_quote_posix(program));
            for a in args {
                s.push(' ');
                s.push_str(&shell_quote_posix(a));
            }
            Ok(s)
        }
        other => Err(format!("onbekende multiplexer: {}", other)),
    }
}

// Een sessienaam komt van de HOST terug en gaat daarna in een commando dat daar
// weer geparseerd wordt, plus in een bestandsnaam onder /tmp. Alleen tekens waar
// geen shell iets mee doet. Dit is geen bescherming van dit werkstation -- de
// payload draait ginds -- maar het houdt een rare naam (spatie, puntkomma,
// backtick) uit een commando dat anders stil iets anders doet dan bedoeld.
fn valid_session_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

// Aanhaken aan een sessie die er AL is. Bewust een andere payload dan die van een
// nieuwe sessie: hier wordt niets aangemaakt en niets gestart. Een `pane run` op
// een sessie waar de agent middenin een beurt zit, zou die beurt overschrijven.
fn build_attach_payload(host: &Host, session: &str) -> Result<String, String> {
    if !valid_session_name(session) {
        return Err(format!(
            "Ongeldige sessienaam: {}. Alleen letters, cijfers, - _ en .",
            session
        ));
    }
    let os = effective_os(host);
    let via_wsl = host.via == "wsl";

    if host.mux == "herdr" && os == "windows" && !via_wsl {
        // Zoals bij de launch: `agent attach` bestaat nog niet op Windows, dus de
        // sessie-TUI haakt aan.
        let script = [
            "$h = (Get-Command herdr -EA 0).Source".to_string(),
            format!(
                "if (-not $h) {{ $h = Join-Path $env:LOCALAPPDATA '{}' }}",
                HERDR_WIN_FALLBACK
            ),
            "if (-not (Test-Path $h)) { Write-Host 'herdr staat niet op deze machine'; exit 1 }"
                .to_string(),
            format!("$s = {}", ps_quote(session)),
            "& $h --session $s".to_string(),
        ]
        .join("\n");
        return Ok(format!(
            "powershell -NoProfile -EncodedCommand {}",
            b64(&utf16le(&script))
        ));
    }

    let inner = match host.mux.as_str() {
        "herdr" => {
            let s = shell_quote_posix(session);
            [
                "H=herdr".to_string(),
                format!("command -v herdr >/dev/null 2>&1 || H=\"{}\"", HERDR_POSIX_FALLBACK),
                format!(
                    "if \"$H\" --session {s} agent get {p} >/dev/null 2>&1; then exec \"$H\" --session {s} agent attach {p}; fi",
                    s = s,
                    p = HERDR_PANE
                ),
                // Geen agent in de pane (afgesloten, of iets wat herdr niet kent):
                // de sessie-TUI werkt nog steeds en je kunt er iets starten.
                format!("exec \"$H\" --session {}", s),
            ]
            .join("\n")
        }
        "tmux" | "psmux" => {
            if os == "windows" && host.mux == "tmux" {
                return Err("tmux bestaat niet op Windows".into());
            }
            // attach-session, niet new-session: bestaat hij niet meer, dan hoort
            // dat een fout te zijn en geen tweede lege sessie.
            format!(
                "exec {} attach-session -t {}",
                host.mux,
                shell_quote_posix(session)
            )
        }
        _ => {
            return Err(
                "Deze machine bewaart geen sessies. Zet persistentie op herdr of tmux.".into(),
            )
        }
    };

    let launcher = posix_launcher(session, &inner);
    if via_wsl {
        Ok(format!("wsl -e sh -c \"{}\"", launcher))
    } else {
        Ok(launcher)
    }
}

// Herdr's TUI tekent een eigen sidebar met workspaces en agents, plus een
// tabbalk. In een Taurus-tab is dat dubbelop -- het venster HEEFT al tabs en een
// agentlijst -- en het vangt bovendien de muis, zodat je in dat deel niets kunt
// selecteren. Alleen een probleem op Windows: daar bestaat `agent attach` nog
// niet, dus daar haakt de tab aan de sessie-TUI in plaats van rechtstreeks aan
// de agent-terminal.
//
// Drie voorwaarden voordat dit aan een configbestand van iemand anders komt:
// alleen als de sleutel er nog niet staat (jouw eigen keuze wint), altijd met
// een back-up, en achteraf gevalideerd met herdrs eigen `config check` -- klopt
// het niet, dan gaat de back-up terug. De sectiekop [ui] is niet optioneel:
// zonder die kop wijst herdr de sleutels af als onbekend.
#[tauri::command(async)]
fn tune_herdr(host: Host) -> Result<String, String> {
    if host.mux != "herdr" {
        return Ok("skip".into());
    }
    // GEMETEN op ursu-wsl: de aanname dat er op Linux/macOS "geen chrome te
    // verbergen is" klopt alleen zolang er een agent in de pane draait. Zodra
    // die weg is, valt build_attach_payload terug op de sessie-TUI -- en dan
    // krijg je daar precies dezelfde dubbele sidebar en tabbalk als op Windows.
    if effective_os(&host) != "windows" || host.via == "wsl" {
        return tune_herdr_posix(&host);
    }
    let ps = r#"$p = Join-Path $env:APPDATA 'herdr\config.toml'
$h = (Get-Command herdr -EA 0).Source
if (-not $h) { $h = Join-Path $env:LOCALAPPDATA 'Programs\Herdr\bin\herdr.exe' }
if (-not (Test-Path $h)) { 'TUNE=skip'; exit 0 }
if (-not (Test-Path $p)) { New-Item -ItemType File -Path $p -Force | Out-Null }
$c = @(Get-Content $p -EA SilentlyContinue)
if ($c -match 'sidebar_start_collapsed') { 'TUNE=skip'; exit 0 }
Copy-Item $p ($p + '.taurus.bak') -Force
$add = @('sidebar_start_collapsed = true', 'sidebar_collapsed_mode = "hidden"', 'hide_tab_bar_when_single_tab = true')
$idx = -1
for ($i = 0; $i -lt $c.Count; $i++) { if ($c[$i].Trim() -eq '[ui]') { $idx = $i; break } }
if ($idx -ge 0) {
  $new = @()
  for ($i = 0; $i -lt $c.Count; $i++) { $new += $c[$i]; if ($i -eq $idx) { $new += $add } }
} else {
  $new = $c + @('', '# Taurus: this tab already has tabs and an agent list of its own, so the', '# herdr chrome would be a second copy of both. This leaves just the pane.', '[ui]') + $add
}
Set-Content -Path $p -Value $new -Encoding ASCII
if ((& $h config check 2>&1) -match 'issues found') { Copy-Item ($p + '.taurus.bak') $p -Force; 'TUNE=fail'; exit 0 }
'TUNE=ok'"#;
    let (out, _) = ssh_run(
        &host,
        &format!("powershell -NoProfile -EncodedCommand {}", b64(&utf16le(ps))),
    )?;
    match out.lines().find_map(|l| l.trim().strip_prefix("TUNE=")) {
        Some("ok") => Ok("ok".into()),
        Some("skip") => Ok("skip".into()),
        Some("fail") => Err("herdr wees de configuratie af; de back-up is teruggezet.".into()),
        _ => Err(format!("onverwacht antwoord van de host:\n{}", out.trim())),
    }
}

// Dezelfde ingreep op een POSIX-host. Config staat daar in
// $XDG_CONFIG_HOME/herdr/config.toml (gemeten: ~/.config/herdr/config.toml).
//
// Het script gaat base64-gecodeerd de lijn over. Anders moet elke quote drie
// parse-rondes overleven (PowerShell -> ssh -> remote sh) en dat is precies
// waar de payload-code elders ook al op stukliep; zo is er maar EEN vorm.
fn tune_herdr_posix(host: &Host) -> Result<String, String> {
    let (out, _) = ssh_run(host, &herdr_tune_posix_command())?;
    match out.lines().find_map(|l| l.trim().strip_prefix("TUNE=")) {
        Some("ok") => Ok("ok".into()),
        Some("skip") => Ok("skip".into()),
        Some("fail") => Err("herdr wees de configuratie af; de back-up is teruggezet.".into()),
        _ => Err(format!("onverwacht antwoord van de host:\n{}", out.trim())),
    }
}

// Los van de aanroep zodat de vorm te testen is zonder een host nodig te hebben.
fn herdr_tune_posix_command() -> String {
    let script = r#"if command -v herdr >/dev/null 2>&1; then H=herdr
elif [ -x "$HOME/.local/bin/herdr" ]; then H="$HOME/.local/bin/herdr"
else echo TUNE=skip; exit 0
fi
D="${XDG_CONFIG_HOME:-$HOME/.config}/herdr"
P="$D/config.toml"
mkdir -p "$D"
[ -f "$P" ] || : > "$P"
if grep -q sidebar_start_collapsed "$P"; then echo TUNE=skip; exit 0; fi
cp "$P" "$P.taurus.bak"
if grep -q "^\[ui\]" "$P"; then
  awk 'BEGIN{d=0} {print} /^\[ui\]/ && d==0 {print "sidebar_start_collapsed = true"; print "sidebar_collapsed_mode = \"hidden\""; print "hide_tab_bar_when_single_tab = true"; d=1}' "$P" > "$P.taurus.new" && mv "$P.taurus.new" "$P"
else
  printf '\n[ui]\nsidebar_start_collapsed = true\nsidebar_collapsed_mode = "hidden"\nhide_tab_bar_when_single_tab = true\n' >> "$P"
fi
if "$H" config check 2>&1 | grep -qi "issues found"; then cp "$P.taurus.bak" "$P"; echo TUNE=fail; exit 0; fi
echo TUNE=ok"#;
    // macOS' base64 kent -d niet altijd; even proberen welke variant werkt.
    format!(
        "if echo | base64 -d >/dev/null 2>&1; then DEC=\"base64 -d\"; else DEC=\"base64 -D\"; fi; echo {} | $DEC | sh",
        b64(script.as_bytes())
    )
}

// Wat er op een host draait. Leeg agent-veld = een sessie zonder (herkende) agent;
// die is nog steeds bruikbaar, dus wel tonen.
#[derive(serde::Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct RemoteSession {
    name: String,
    status: String,
    agent: String,
    agent_status: String,
    cwd: String,
}

// De host stuurt regels, Rust parseert. Bewust geen JSON-verwerking in de shell:
// `pane list` is JSON en serde staat hier toch al.
fn parse_herdr_sessions(out: &str) -> Vec<RemoteSession> {
    let mut list: Vec<RemoteSession> = Vec::new();
    for line in out.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("SESS|") {
            let mut it = rest.splitn(2, '|');
            let name = it.next().unwrap_or("").trim().to_string();
            let status = it.next().unwrap_or("").trim().to_string();
            if name.is_empty() {
                continue;
            }
            list.push(RemoteSession { name, status, ..Default::default() });
        } else if let Some(rest) = line.strip_prefix("JSON|") {
            let mut it = rest.splitn(2, '|');
            let name = it.next().unwrap_or("").trim().to_string();
            let json = it.next().unwrap_or("");
            let Some(s) = list.iter_mut().find(|s| s.name == name) else { continue };
            let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else { continue };
            let Some(p) = v.pointer("/result/panes/0") else { continue };
            let get = |k: &str| p.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
            s.cwd = get("cwd");
            s.agent = get("agent");
            // "unknown" is herdr's "nog niets gezien"; als status is dat ruis.
            let st = get("agent_status");
            s.agent_status = if st == "unknown" { String::new() } else { st };
        }
    }
    list
}

fn parse_tmux_sessions(out: &str) -> Vec<RemoteSession> {
    out.lines()
        .filter_map(|l| {
            let mut it = l.trim().split('|');
            let name = it.next()?.trim().to_string();
            if name.is_empty() {
                return None;
            }
            let cwd = it.next().unwrap_or("").trim().to_string();
            let attached = it.next().unwrap_or("").trim() == "1";
            Some(RemoteSession {
                name,
                status: if attached { "attached".into() } else { "running".into() },
                cwd,
                ..Default::default()
            })
        })
        .collect()
}

// Wat draait er op die machine? Eén ssh-ronde per keer verversen.
#[tauri::command(async)]
fn remote_sessions(host_id: String) -> Result<Vec<RemoteSession>, String> {
    let host = lookup_host(&host_id)?
        .ok_or_else(|| "Sessies opsommen kan alleen op een andere machine.".to_string())?;
    let os = effective_os(&host);
    let via_wsl = host.via == "wsl";

    let posix_script = match host.mux.as_str() {
        "herdr" => Some(
            "H=herdr; command -v herdr >/dev/null 2>&1 || H=\"$HOME/.local/bin/herdr\"; \
             \"$H\" session list 2>/dev/null | awk 'NR>1 && NF>=2 {print $1\" \"$2}' | \
             while read -r n s; do echo \"SESS|$n|$s\"; \
             if [ \"$s\" = running ]; then printf 'JSON|%s|' \"$n\"; \
             \"$H\" --session \"$n\" pane list 2>/dev/null | tr -d '\\r\\n'; echo \"\"; fi; done"
                .to_string(),
        ),
        "tmux" | "psmux" => Some(format!(
            "{} ls -F '#{{session_name}}|#{{session_path}}|#{{session_attached}}' 2>/dev/null || true",
            host.mux
        )),
        _ => None,
    };

    let (out, _) = if host.mux == "herdr" && os == "windows" && !via_wsl {
        let ps = r#"$h = (Get-Command herdr -EA 0).Source
if (-not $h) { $h = Join-Path $env:LOCALAPPDATA 'Programs\Herdr\bin\herdr.exe' }
if (-not (Test-Path $h)) { 'ERR=herdr staat niet op deze machine'; exit 0 }
$lines = & $h session list 2>$null
foreach ($l in ($lines | Select-Object -Skip 1)) {
  $p = ($l.Trim() -split '\s+')
  if ($p.Count -lt 2) { continue }
  $n = $p[0]; $st = $p[1]
  'SESS|' + $n + '|' + $st
  if ($st -eq 'running') { 'JSON|' + $n + '|' + ((& $h --session $n pane list 2>$null) -join '') }
}"#;
        ssh_run(&host, &format!("powershell -NoProfile -EncodedCommand {}", b64(&utf16le(ps))))?
    } else {
        let script = posix_script.ok_or_else(|| {
            "Deze machine bewaart geen sessies. Zet persistentie op herdr of tmux.".to_string()
        })?;
        // Zelfde base64-truc als elders: de payload gaat door cmd.exe en/of sh,
        // en bevat quotes, pipes en accolades.
        let cmd = if via_wsl {
            format!("wsl -e sh -c \"echo {} | base64 -d | sh\"", b64(script.as_bytes()))
        } else {
            format!("echo {} | base64 -d | sh", b64(script.as_bytes()))
        };
        ssh_run(&host, &cmd)?
    };

    if let Some(err) = out.lines().find_map(|l| l.trim().strip_prefix("ERR=")) {
        return Err(err.to_string());
    }
    Ok(if host.mux == "herdr" {
        parse_herdr_sessions(&out)
    } else {
        parse_tmux_sessions(&out)
    })
}

// ---------- agents op een machine van jezelf (#128) ----------
//
// DE REGEL: Taurus toont AGENTS. ssh, tmux en herdr maken de weg vrij zodat er een
// agent kan starten -- ze zijn leidingwerk en nooit iets wat je kiest. Geen agent
// betekent dat er niets is om mee te verbinden; dat is geen keuze met een
// waarschuwingslabel erop, het is geen keuze.
//
// Een agent kan er op twee manieren zijn, en voor wie kijkt is dat hetzelfde ding:
//   - Taurus startte hem hier vandaan over ssh; dan weet herdr ervan.
//   - Hij draait in de Taurus OP die machine; dan staat hij in de sessions.json daar.
// GEMETEN op ursu: herdr kende drie sessies (nul agents) terwijl er twee agents
// draaiden die alleen in die sessions.json stonden. Wie alleen herdr vraagt, ziet
// dus precies het verkeerde.
#[derive(serde::Serialize, Clone, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RemoteAgent {
    // De mux-sessie waaraan je kunt aanhaken. Leeg voor een agent die in de Taurus
    // daar draait: die heeft er geen, en daarom kun je hem (nog) niet overnemen.
    session: String,
    title: String,
    agent: String,
    cwd: String,
    status: String,
    // "herdr" of "taurus" -- een detail op de regel, geen tweede lijst.
    origin: String,
    attachable: bool,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct MachineAgents {
    agents: Vec<RemoteAgent>,
    // Sessies zonder agent. Geen keuze, wel opruimwerk -- ze bestaan echt.
    empty: Vec<String>,
    // Kon de Taurus op die machine bevraagd worden? "Nee" is iets anders dan
    // "die draait daar niets".
    taurus_seen: bool,
}

// Wat de laatste regel van de andere Taurus zegt over zijn eigen sessies. Alleen de
// velden die hier iets betekenen; de rest van dat bestand gaat ons niet aan.
#[derive(serde::Deserialize)]
struct PeerSession {
    #[serde(default)]
    title: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    agent: String,
    // Leeg = lokaal op die machine. Een sessie die dáár naar een derde machine
    // wijst is niet van deze machine en hoort hier niet in de lijst.
    #[serde(default)]
    host_id: String,
}

// De twee bronnen samenvoegen. Puur, zodat de regel "geen agent is geen keuze"
// toetsbaar is zonder machine ernaast.
fn collect_agents(sessions: Vec<RemoteSession>, peer_json: Option<&str>) -> MachineAgents {
    let mut out = MachineAgents::default();
    for s in sessions {
        if s.agent.trim().is_empty() {
            // Geen agent: bestaat wel, is geen keuze.
            out.empty.push(s.name);
            continue;
        }
        // De mapnaam leest prettiger dan een sessienaam met een hash erachter.
        let leaf = s
            .cwd
            .rsplit(['\\', '/'])
            .find(|p| !p.is_empty())
            .unwrap_or(&s.name)
            .to_string();
        out.agents.push(RemoteAgent {
            title: leaf,
            agent: s.agent,
            cwd: s.cwd,
            status: if s.agent_status.is_empty() { s.status } else { s.agent_status },
            session: s.name,
            origin: "herdr".into(),
            attachable: true,
        });
    }
    let Some(txt) = peer_json else { return out };
    let Ok(peers) = serde_json::from_str::<Vec<PeerSession>>(txt) else {
        return out;
    };
    out.taurus_seen = true;
    for p in peers {
        // Alleen wat op DIE machine zelf draait.
        if !p.host_id.trim().is_empty() {
            continue;
        }
        // Draait hij al als mux-sessie, dan is het dezelfde agent en niet een tweede.
        if out.agents.iter().any(|a| paths_equal(&a.cwd, &p.path)) {
            continue;
        }
        out.agents.push(RemoteAgent {
            title: p.title,
            agent: if p.agent.is_empty() { "claude".into() } else { p.agent },
            cwd: p.path,
            status: String::new(),
            session: String::new(),
            origin: "taurus".into(),
            // Er is geen kanaal naar een sessie die in die Taurus zelf leeft. Tonen
            // dus wel, aanbieden nog niet -- en dat moet de UI ook zo zeggen.
            attachable: false,
        });
    }
    out
}

// Windows-paden verschillen in hoofdletters en in de slash die je toevallig typte.
fn paths_equal(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.trim().trim_end_matches(['\\', '/']).replace('/', "\\").to_lowercase();
    !a.trim().is_empty() && norm(a) == norm(b)
}

// De sessions.json van de Taurus aan de andere kant uitlezen. Aparte ssh-ronde in
// plaats van de bestaande sessie-scripts uitbreiden: die zijn precies afgeregeld en
// gaan door twee quoting-lagen, en dit is een lijst die alleen op verzoek wordt
// opgehaald -- correctheid weegt daar zwaarder dan een seconde.
fn peer_sessions_command(host: &Host) -> Option<String> {
    if effective_os(host) != "windows" || host.via == "wsl" {
        // Op POSIX heeft config_dir geen stabiele plek (geen APPDATA), dus daar valt
        // niets betrouwbaars te lezen. Liever niets dan een gokpad.
        return None;
    }
    let ps = "$p = Join-Path $env:APPDATA 'Taurus\\sessions.json'\n\
              if (Test-Path $p) { Get-Content $p -Raw } else { '[]' }";
    Some(format!(
        "powershell -NoProfile -EncodedCommand {}",
        b64(&utf16le(ps))
    ))
}

#[tauri::command]
fn remote_agents(host_id: String) -> Result<MachineAgents, String> {
    let host = lookup_host(&host_id)?
        .ok_or_else(|| "Agents opsommen kan alleen op een andere machine.".to_string())?;
    let sessions = remote_sessions(host_id)?;
    let peer = peer_sessions_command(&host)
        .and_then(|cmd| ssh_run(&host, &cmd).ok())
        .map(|(out, _)| out);
    Ok(collect_agents(sessions, peer.as_deref()))
}

// Het commando dat een sessie op de andere machine beëindigt (#124). Apart van de
// tauri-command, want dit is de kant die te toetsen valt zonder machine ernaast.
//
// GEMETEN tegen herdr op ursu, en de eerste versie hiervan was fout. Die deed
// `herdr --session <naam> server stop`, naar analogie van hoe Taurus een server
// START. Dat gaf exitcode 0 en veranderde niets -- de sessie stond daarna nog
// gewoon in de lijst -- en het startte als bijeffect de default-sessie op die
// machine. De juiste vorm neemt de naam als ARGUMENT:
//
//   herdr session stop <naam>     stopt de server, laat de sessie staan
//   herdr session delete <naam>   haalt hem echt weg  ("deleted session ...")
//
// Allebei, in die volgorde: de knop staat er voor een sessie die weg moet, en een
// entry die blijft staan is precies de klacht waarvoor hij gebouwd is.
fn stop_session_command(host: &Host, session: &str) -> Result<String, String> {
    let os = effective_os(host);
    let via_wsl = host.via == "wsl";
    let posix = |script: String| {
        if via_wsl {
            format!("wsl -e sh -c \"echo {} | base64 -d | sh\"", b64(script.as_bytes()))
        } else {
            format!("echo {} | base64 -d | sh", b64(script.as_bytes()))
        }
    };
    match host.mux.as_str() {
        "herdr" if os == "windows" && !via_wsl => {
            let ps = format!(
                "$h = (Get-Command herdr -EA 0).Source\n\
                 if (-not $h) {{ $h = Join-Path $env:LOCALAPPDATA '{fb}' }}\n\
                 if (-not (Test-Path $h)) {{ 'ERR=herdr staat niet op deze machine'; exit 0 }}\n\
                 & $h session stop {s} 2>&1 | Out-Null\n\
                 & $h session delete {s} 2>&1 | Out-Null\n\
                 if ($LASTEXITCODE -ne 0) {{ 'ERR=herdr kon de sessie niet verwijderen' }}",
                fb = HERDR_WIN_FALLBACK,
                s = ps_quote(session),
            );
            Ok(format!(
                "powershell -NoProfile -EncodedCommand {}",
                b64(&utf16le(&ps))
            ))
        }
        "herdr" => Ok(posix(format!(
            "H=herdr; command -v herdr >/dev/null 2>&1 || H=\"{fb}\"; \
             \"$H\" session stop {s} >/dev/null 2>&1; \
             \"$H\" session delete {s} >/dev/null 2>&1 || echo 'ERR=herdr kon de sessie niet verwijderen'",
            fb = HERDR_POSIX_FALLBACK,
            s = shell_quote_posix(session),
        ))),
        "tmux" | "psmux" => Ok(posix(format!(
            "{mux} kill-session -t {s} 2>&1 || echo 'ERR=tmux kon de sessie niet stoppen'",
            mux = host.mux,
            s = shell_quote_posix(session),
        ))),
        _ => Err("Deze machine bewaart geen sessies, dus er valt niets te stoppen.".to_string()),
    }
}

// Een sessie op een andere machine beëindigen. Zonder dit is een verweesde sessie --
// eentje waarvan de agent weg is -- alleen op te ruimen door naar die machine toe te
// lopen, terwijl Taurus hem wel toont en er een oude `--resume` in laat mislukken.
#[tauri::command]
fn stop_remote_session(host_id: String, session: String) -> Result<(), String> {
    let host = lookup_host(&host_id)?
        .ok_or_else(|| "Een sessie stoppen kan alleen op een andere machine.".to_string())?;
    let cmd = stop_session_command(&host, &session)?;
    let (out, _) = ssh_run(&host, &cmd)?;
    if let Some(err) = out.lines().find_map(|l| l.trim().strip_prefix("ERR=")) {
        return Err(err.to_string());
    }
    Ok(())
}

// Open een tab op een sessie die al draait. Geen uuid, geen agent-vlaggen: Taurus
// heeft dit commando niet gebouwd en kan het dus ook niet hervatten.
#[tauri::command]
fn attach_remote_session(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    gen: u64,
    host_id: String,
    session: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let host = lookup_host(&host_id)?
        .ok_or_else(|| "Aanhaken kan alleen op een andere machine.".to_string())?;
    let payload = build_attach_payload(&host, &session)?;
    let (program, args) = ssh_interactive(&host, payload)?;
    start_pty(
        &app,
        &state.sessions,
        id,
        gen,
        program,
        &local_cwd_for_remote(),
        args,
        cols,
        rows,
    )
}

// ssh.exe uit System32 heeft de voorkeur boven wat er toevallig in PATH staat:
// een Git-for-Windows of WSL-ssh in PATH gedraagt zich anders rond paden en pty.
fn ssh_program() -> String {
    if let Ok(root) = std::env::var("SystemRoot") {
        let p = std::path::PathBuf::from(&root).join("System32\\OpenSSH\\ssh.exe");
        if p.is_file() {
            return p.to_string_lossy().into_owned();
        }
    }
    "ssh.exe".into()
}

// (programma, argumenten) voor een remote sessie.
fn wrap_remote(
    host: &Host,
    cwd: &str,
    program: String,
    args: Vec<String>,
) -> Result<(String, Vec<String>), String> {
    if host.hostname.trim().is_empty() {
        return Err("host heeft geen hostname".into());
    }
    let session = mux_session_name(&host.id, cwd);
    let payload = build_remote_payload_via(
        &host.mux,
        effective_os(host),
        &host.via,
        &session,
        cwd,
        &program,
        &args,
    )?;

    ssh_interactive(host, payload)
}

// De ssh-aanroep om een payload met een pty op de host te draaien. Gedeeld door
// een nieuwe sessie en door aanhaken, zodat allebei dezelfde key- en
// host-key-behandeling krijgen.
fn ssh_interactive(host: &Host, payload: String) -> Result<(String, Vec<String>), String> {
    // -t forceert een pty aan de andere kant; zonder pty tekent de agent-TUI niet.
    let mut a = vec!["-t".to_string()];
    // batch=false: bij een echte sessie MOET een passphrase- of wachtwoordprompt
    // zichtbaar zijn, want er is een pty om hem in te typen. Alleen de probe
    // draait met BatchMode.
    a.extend(ssh_base_args(host, false)?);
    a.push(payload);
    Ok((ssh_program(), a))
}

// De lokale werkmap van ssh.exe doet er niet toe -- de echte werkmap zit in de
// payload. start_pty eist wel een bestaande map, dus we geven de home van het
// werkstation; die bestaat altijd.
fn local_cwd_for_remote() -> String {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".into())
}

// Start een nieuwe agent-sessie. session_id is een vooraf bepaalde UUID, zodat we
// later kunnen herstarten met `claude --resume <uuid>`.
#[tauri::command]
fn create_session(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    gen: u64,
    path: String,
    title: String,
    task: String,
    session_id: String,
    mode: String,
    full_paths: bool,
    command: String,
    agent: String,
    model: String,
    host_id: String,
    cols: u16,
    rows: u16,
    // Wegwerpsessie: geen persistentie, ook niet als de machine herdr heeft (#124).
    //
    // GEMETEN nadat de Connect-knop een paar keer was gebruikt: op ursu stonden drie
    // herdr-sessies zonder agent, waarvan twee door Connect gemaakt. "Geen kaart in
    // de zijbalk" was dus maar de helft van het verhaal -- aan de andere kant bleef
    // er wél iets staan, en die lege sessies zetten je bij het aanhaken in een kale
    // shell of in een mislukte `claude --resume`. Wat weg mag zijn, moet ook echt
    // verdwijnen als je de tab sluit.
    ephemeral: Option<bool>,
) -> Result<(), String> {
    // De host moet BEKEND zijn voordat het commando gebouwd wordt: remote levert
    // een andere programmanaam op dan lokaal.
    let host = lookup_host(&host_id)?.map(|h| without_persistence(h, ephemeral.unwrap_or(false)));
    let (program, args) = if !command.trim().is_empty() {
        // Commando-override (bijv. nep-Claude voor de demo): voer dit programma
        // uit i.p.v. de agent, zonder agent-vlaggen.
        parse_override(&command)?
    } else {
        build_command(
            &agent,
            LaunchKind::Create,
            &session_id,
            &title,
            &task,
            &mode,
            &model,
            full_paths,
            host.as_ref().map(|h| effective_os(h)),
        )
    };
    let (program, args, cwd) = apply_host(host.as_ref(), &path, program, args)?;
    start_pty(&app, &state.sessions, id, gen, program, &cwd, args, cols, rows)
}

// Een wegwerpsessie krijgt een host-kopie zonder persistentie: dan maakt de andere
// kant geen herdr-sessie aan en blijft er dus ook niets staan. `mux_auto` gaat mee
// uit, anders zou een latere hertest hem alsnog invullen. De opgeslagen host in
// hosts.json verandert niet -- dit geldt alleen voor deze ene start.
fn without_persistence(h: Host, ephemeral: bool) -> Host {
    if ephemeral {
        Host { mux: "none".to_string(), mux_auto: false, ..h }
    } else {
        h
    }
}

// Leeg host_id = lokaal. Een onbekend id is een fout en geen stille terugval op
// lokaal: dan zou een remote tab ongemerkt op het werkstation starten.
fn lookup_host(host_id: &str) -> Result<Option<Host>, String> {
    if host_id.trim().is_empty() {
        return Ok(None);
    }
    get_hosts()
        .into_iter()
        .find(|h| h.id == host_id)
        .map(Some)
        .ok_or_else(|| format!("onbekende host: {}", host_id))
}

// Lokaal blijft alles zoals het was; remote wikkelt ssh eromheen en verschuift
// de werkmap: `path` is dan de map OP DE HOST, en de lokale werkmap van ssh.exe
// is de home van het werkstation (start_pty eist een bestaande lokale map).
fn apply_host(
    host: Option<&Host>,
    path: &str,
    program: String,
    args: Vec<String>,
) -> Result<(String, Vec<String>, String), String> {
    let host = match host {
        None => return Ok((program, args, path.to_string())),
        Some(h) => h,
    };
    let remote_cwd = if path.trim().is_empty() {
        host.default_project.trim().to_string()
    } else {
        path.trim().to_string()
    };
    if remote_cwd.is_empty() {
        return Err(format!(
            "Geen werkmap voor host '{}'. Vul een projectpad in, of een standaard-projectpad bij de host.",
            host.nickname
        ));
    }
    let (program, args) = wrap_remote(host, &remote_cwd, program, args)?;
    Ok((program, args, local_cwd_for_remote()))
}

// Herstart een sessie: stop het huidige claude-proces en hervat hetzelfde gesprek
// met `claude --resume <uuid>` (bijv. na een MCP-server-update).
#[tauri::command]
fn restart_session(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    gen: u64,
    path: String,
    title: String,
    session_id: String,
    mode: String,
    full_paths: bool,
    command: String,
    agent: String,
    model: String,
    host_id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    {
        if let Some(mut s) = state.sessions.lock().unwrap().remove(&id) {
            let _ = s.child.kill();
        }
    }
    let host = lookup_host(&host_id)?;
    let (program, args) = if !command.trim().is_empty() {
        parse_override(&command)?
    } else {
        build_command(
            &agent,
            LaunchKind::Resume,
            &session_id,
            &title,
            "",
            &mode,
            &model,
            full_paths,
            host.as_ref().map(|h| effective_os(h)),
        )
    };
    // Remote herstart: het kappen hierboven doodt alleen de lokale ssh.exe. Draait
    // er een multiplexer op de host, dan leeft de agent daar gewoon door en haakt
    // `new-session -A` er weer aan -- dan is dit een heraanhaak-actie in plaats van
    // een herstart. Bestaat de sessie niet meer, dan start hij vers met --resume.
    let (program, args, cwd) = apply_host(host.as_ref(), &path, program, args)?;
    start_pty(&app, &state.sessions, id, gen, program, &cwd, args, cols, rows)
}

#[tauri::command]
fn write_session(state: State<AppState>, id: String, data: String) -> Result<(), String> {
    let mut map = state.sessions.lock().unwrap();
    if let Some(s) = map.get_mut(&id) {
        s.writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        s.writer.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn resize_session(state: State<AppState>, id: String, cols: u16, rows: u16) -> Result<(), String> {
    let map = state.sessions.lock().unwrap();
    if let Some(s) = map.get(&id) {
        s.master
            .resize(PtySize {
                rows: rows.max(1),
                cols: cols.max(1),
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn close_session(state: State<AppState>, id: String) {
    if let Some(mut s) = state.sessions.lock().unwrap().remove(&id) {
        let _ = s.child.kill();
    }
}

#[derive(serde::Serialize)]
struct HtmlFile {
    path: String,
    name: String,
    mtime: u64,
}

fn scan_html(dir: &Path, depth: i32, out: &mut Vec<HtmlFile>) {
    if depth < 0 {
        return;
    }
    let skip = ["node_modules", ".git", "target", "dist", ".next", "vendor", "bin", "obj"];
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if skip.contains(&name.as_str()) || name.starts_with('.') {
                continue;
            }
            scan_html(&p, depth - 1, out);
        } else if p
            .extension()
            .map(|e| {
                e.eq_ignore_ascii_case("html")
                    || e.eq_ignore_ascii_case("htm")
                    || e.eq_ignore_ascii_case("md")
            })
            .unwrap_or(false)
        {
            let mtime = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            out.push(HtmlFile {
                path: p.to_string_lossy().into_owned(),
                name: p
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                mtime,
            });
        }
    }
}

// Vind .html-bestanden in de werkmap (max 3 niveaus diep), nieuwste eerst.
#[tauri::command]
fn list_html(dir: String) -> Vec<HtmlFile> {
    let mut out = Vec::new();
    scan_html(Path::new(&dir), 3, &mut out);
    out.sort_by(|a, b| b.mtime.cmp(&a.mtime));
    out.truncate(80);
    out
}

// Preview-plafond aan de Rust-kant: voorheen werd het hele bestand gelezen en
// over de IPC gestuurd en pas in JS op 2 MB gecontroleerd (#72). De frontend
// herkent "file too large" en toont de preview_toobig-melding.
const MAX_PREVIEW_BYTES: u64 = 2_000_000;

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    if meta.len() > MAX_PREVIEW_BYTES {
        return Err(format!("file too large for preview ({} bytes)", meta.len()));
    }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ===== File-dropper: gedropte bestanden/mappen -> <cwd>\input =====
// De sidebar-dropper laat een bestand of map verplaatsen/kopieren naar een
// "input"-submap van de werkmap van de actieve sessie (of het geselecteerde
// project). Zo landt een bestand op een voorspelbare plek en is het pad direct
// bruikbaar voor de agent. "Alleen pad" doet geen bestandsactie (frontend post
// het bestaande pad); geplakte objecten lopen via save_clipboard_to_input.

// Geef een bestemmingspad dat nog niet bestaat: "naam.ext", dan "naam (2).ext",
// "naam (3).ext", ... zodat een drop nooit een bestaand bestand overschrijft.
fn unique_path(dest: std::path::PathBuf) -> std::path::PathBuf {
    if !dest.exists() {
        return dest;
    }
    let parent = dest.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let stem = dest
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ext = dest.extension().map(|e| e.to_string_lossy().into_owned());
    let mut n = 2u32;
    loop {
        let name = match &ext {
            Some(e) => format!("{} ({}).{}", stem, n, e),
            None => format!("{} ({})", stem, n),
        };
        let cand = parent.join(name);
        if !cand.exists() {
            return cand;
        }
        n += 1;
    }
}

// Ligt de (nog niet bestaande) bestemming BINNEN de bron? Dan zou
// copy_recursive de map in zichzelf blijven kopieren tot de schijf vol is
// (bv. de werkmap zelf op de dropzone slepen: dest = <src>\input\<naam>).
// We canonicaliseren beide kanten zodat relatieve paden, symlinks en
// verschillend hoofdlettergebruik geen vals negatief opleveren (#70).
fn dest_inside_src(src: &Path, dest: &Path) -> bool {
    let src_c = match std::fs::canonicalize(src) {
        Ok(p) => p,
        Err(_) => return false,
    };
    // dest bestaat nog niet; de ouder (de input-map) wel — die is net aangemaakt.
    dest.parent()
        .and_then(|p| std::fs::canonicalize(p).ok())
        .map(|p| p.starts_with(&src_c))
        .unwrap_or(false)
}

// Kopieer een bestand of (recursief) een hele map naar `dest`.
fn copy_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        std::fs::create_dir_all(dest)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            copy_recursive(&entry.path(), &dest.join(entry.file_name()))?;
        }
        Ok(())
    } else {
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src, dest)?;
        Ok(())
    }
}

// Verplaats of kopieer een gedropt bestand/map naar <cwd>\input. `mode` is
// "move" of "copy". Geeft het absolute bestemmingspad terug.
#[tauri::command]
fn save_dropped_path(src: String, cwd: String, mode: String) -> Result<String, String> {
    let src_path = Path::new(&src);
    if !src_path.exists() {
        return Err(format!("bron bestaat niet: {}", src));
    }
    let name = src_path
        .file_name()
        .ok_or_else(|| "bron heeft geen naam".to_string())?;
    let input_dir = Path::new(&cwd).join("input");
    std::fs::create_dir_all(&input_dir).map_err(|e| e.to_string())?;
    let dest = unique_path(input_dir.join(name));
    if dest_inside_src(src_path, &dest) {
        return Err(format!(
            "bestemming ligt binnen de bron (map zou in zichzelf gekopieerd worden): {}",
            src
        ));
    }

    match mode.as_str() {
        "move" => {
            // Snel pad: rename werkt op hetzelfde volume voor bestanden EN mappen.
            // Ander volume (C: -> X:) laat rename falen -> recursief kopieren en
            // daarna de bron wissen.
            if std::fs::rename(src_path, &dest).is_err() {
                copy_recursive(src_path, &dest).map_err(|e| e.to_string())?;
                let rm = if src_path.is_dir() {
                    std::fs::remove_dir_all(src_path)
                } else {
                    std::fs::remove_file(src_path)
                };
                rm.map_err(|e| e.to_string())?;
            }
        }
        "copy" => copy_recursive(src_path, &dest).map_err(|e| e.to_string())?,
        other => return Err(format!("onbekende modus: {}", other)),
    }
    Ok(dest.to_string_lossy().into_owned())
}

// Lees de op het klembord gekopieerde bestandspaden (Verkenner Ctrl+C zet een
// CF_HDROP-lijst op het klembord). De clipboard-manager-plugin kent alleen
// tekst/HTML/afbeelding, dus dit gaat rechtstreeks via de Win32-klembord-API.
#[cfg(windows)]
fn clipboard_file_paths() -> Vec<String> {
    use windows::Win32::System::DataExchange::{
        CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    };
    use windows::Win32::UI::Shell::{DragQueryFileW, HDROP};
    const CF_HDROP: u32 = 15;
    let mut out: Vec<String> = Vec::new();
    unsafe {
        if IsClipboardFormatAvailable(CF_HDROP).is_err() {
            return out;
        }
        if OpenClipboard(None).is_err() {
            return out;
        }
        if let Ok(handle) = GetClipboardData(CF_HDROP) {
            let hdrop = HDROP(handle.0);
            let count = DragQueryFileW(hdrop, 0xFFFF_FFFF, None);
            for i in 0..count {
                let len = DragQueryFileW(hdrop, i, None);
                if len == 0 {
                    continue;
                }
                let mut buf = vec![0u16; (len as usize) + 1];
                let got = DragQueryFileW(hdrop, i, Some(buf.as_mut_slice()));
                if got > 0 {
                    out.push(String::from_utf16_lossy(&buf[..got as usize]));
                }
            }
        }
        let _ = CloseClipboard();
    }
    out
}

#[cfg(not(windows))]
fn clipboard_file_paths() -> Vec<String> {
    Vec::new()
}

// Sla de inhoud van het klembord op als bestand(en) in <cwd>\input en geef de
// absolute paden terug. Volgorde: eerst gekopieerde BESTANDEN (Ctrl+C in
// Verkenner -> die kopieren we in), dan een AFBEELDING (als PNG), dan TEKST
// (.txt). Zo werkt "Plak object" ook voor een gekopieerd bestand -- niet alleen
// voor tekst/afbeelding die nog geen bestand zijn.
#[tauri::command]
fn save_clipboard_to_input(app: AppHandle, cwd: String) -> Result<Vec<String>, String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    let input_dir = Path::new(&cwd).join("input");
    std::fs::create_dir_all(&input_dir).map_err(|e| e.to_string())?;

    // 1) Gekopieerde bestanden/mappen (CF_HDROP) -> kopieer ze in de input-map.
    let files = clipboard_file_paths();
    if !files.is_empty() {
        let mut saved = Vec::new();
        for f in files {
            let src = Path::new(&f);
            if !src.exists() {
                continue;
            }
            let name = match src.file_name() {
                Some(n) => n,
                None => continue,
            };
            let dest = unique_path(input_dir.join(name));
            if dest_inside_src(src, &dest) {
                return Err(format!(
                    "bestemming ligt binnen de bron (map zou in zichzelf gekopieerd worden): {}",
                    f
                ));
            }
            copy_recursive(src, &dest).map_err(|e| e.to_string())?;
            saved.push(dest.to_string_lossy().into_owned());
        }
        if !saved.is_empty() {
            return Ok(saved);
        }
    }

    // 2) Afbeelding op het klembord -> PNG.
    if let Ok(img) = app.clipboard().read_image() {
        let (w, h) = (img.width(), img.height());
        let rgba = img.rgba();
        if w > 0 && h > 0 && rgba.len() as u32 == w * h * 4 {
            let dest = unique_path(input_dir.join("pasted.png"));
            let file = std::fs::File::create(&dest).map_err(|e| e.to_string())?;
            let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut writer = enc.write_header().map_err(|e| e.to_string())?;
            writer.write_image_data(rgba).map_err(|e| e.to_string())?;
            return Ok(vec![dest.to_string_lossy().into_owned()]);
        }
    }

    // 3) Tekst.
    let text = app.clipboard().read_text().map_err(|e| e.to_string())?;
    if text.is_empty() {
        return Err("klembord bevat geen bestand, afbeelding of tekst".to_string());
    }
    let dest = unique_path(input_dir.join("pasted.txt"));
    std::fs::write(&dest, text).map_err(|e| e.to_string())?;
    Ok(vec![dest.to_string_lossy().into_owned()])
}

// Schrijf tekst naar het Windows-klembord via native code i.p.v. de WebView2
// browser-API. Een WebView2/Edge-update kan navigator.clipboard.writeText IN de
// webview blokkeren (kopieren-bij-selectie en Ctrl+Shift+C deden niets meer),
// terwijl het OS-klembord en elke shell buiten Taurus prima blijven werken. Deze
// native weg schrijft op dezelfde laag als die externe shells en omzeilt het
// schrijf-beleid van de webview. Eigen command (geen ACL-permissie nodig), net
// als pick_folder.
#[tauri::command]
fn copy_to_clipboard(app: AppHandle, text: String) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    // Het Windows-klembord kan kortstondig vergrendeld zijn door een ander
    // proces (clipboard-manager, RDP, Office); OpenClipboard faalt dan. Een
    // paar korte herpogingen vangt die contentie op i.p.v. stil opgeven.
    let mut last = String::new();
    for attempt in 0..4 {
        match app.clipboard().write_text(text.clone()) {
            Ok(()) => return Ok(()),
            Err(e) => {
                last = e.to_string();
                if attempt < 3 {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            }
        }
    }
    Err(last)
}

// Lichtgewicht diagnoselog voor klembord-events. Schrijft alleen METADATA
// (event, lengte, ok/fout) naar %APPDATA%\Taurus\clipboard.log -- nooit de
// inhoud van het klembord (privacy). Gebruikt om intermitterende kopieer/plak-
// problemen te debuggen zonder devtools.
#[tauri::command]
fn debug_log(line: String) {
    use std::io::Write;
    let p = config_dir().join("clipboard.log");
    let _ = std::fs::create_dir_all(config_dir());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "{}", line);
    }
}

// ===== White-label branding =====
// Branding is een configuratie-item, geen code: een optioneel branding.json in
// de per-gebruiker config-map (%APPDATA%\Taurus\, naast projects.json) bepaalt
// app-naam, ondertitel, logo, venstertitel en CSS-variabele-overrides. Ontbreekt
// het bestand, dan komen lege velden terug en blijft de UI gewoon Taurus. Zo kan
// iedereen rebranden zonder de code te wijzigen, en blijft de default-build
// identiek. Bedrijfsspecifieke branding hoort puur in dat lokale bestand thuis.
// De JSON-sleutels zijn camelCase (appName, windowTitle) -- net als
// branding.example.json, de README en de Branding-uitvoer hieronder. Zonder deze
// rename werden appName/windowTitle stil genegeerd (serde zocht app_name/
// window_title), waardoor de titel op de default "Taurus" bleef staan (#31).
#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct BrandingConfig {
    #[serde(default)]
    app_name: String,
    #[serde(default)]
    subtitle: String,
    #[serde(default)]
    logo: String, // absoluut pad naar een afbeelding
    #[serde(default)]
    window_title: String,
    #[serde(default)]
    theme: HashMap<String, String>, // CSS-variabele -> waarde, bv. "--accent": "#3b82f6"
    #[serde(default)]
    skin: String, // optionele default-skin (bv. "winxp"); leeg = geen default
    #[serde(default)]
    skin_name: String, // label voor de merk-skin in de Thema-lijst (bv. "NEXUS"); leeg = val terug op appName
    #[serde(default)]
    font: String, // optioneel UI-lettertype (bv. "'IBM Plex Mono', monospace")
    #[serde(default)]
    garble: Option<bool>, // garble-hover forceren aan/uit; None = default (aan bij merk-skin)
}

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Branding {
    app_name: String,
    subtitle: String,
    logo_data_uri: String, // "" als er geen logo is
    window_title: String,
    theme: HashMap<String, String>,
    skin: String,
    skin_name: String,
    font: String,
    garble: Option<bool>,
}

// Minimale standaard-base64 (geen extra dependency): het logo wordt als data-URI
// teruggegeven zodat het binnen de WebView2-sandbox laadt (een file:-pad zou door
// de CSP/asset-laag geweigerd worden).
fn b64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for c in data.chunks(3) {
        let b0 = c[0];
        let b1 = *c.get(1).unwrap_or(&0);
        let b2 = *c.get(2).unwrap_or(&0);
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if c.len() > 1 { T[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if c.len() > 2 { T[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}

fn logo_mime(path: &str) -> &'static str {
    let p = path.to_lowercase();
    if p.ends_with(".svg") {
        "image/svg+xml"
    } else if p.ends_with(".jpg") || p.ends_with(".jpeg") {
        "image/jpeg"
    } else if p.ends_with(".gif") {
        "image/gif"
    } else if p.ends_with(".webp") {
        "image/webp"
    } else {
        "image/png"
    }
}

// Vind branding.json: eerst de per-gebruiker config-map (%APPDATA%\Taurus\),
// anders naast de exe -- zo werkt een portable, voorgebrande map zonder setup.
// Geeft ook de map terug waarin het bestand stond, zodat een relatief logo-pad
// daartegen opgelost kan worden (portable: "logo": "logo.png" naast de exe).
fn load_branding() -> (BrandingConfig, std::path::PathBuf) {
    let mut candidates: Vec<std::path::PathBuf> = vec![config_dir().join("branding.json")];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("branding.json"));
        }
    }
    for p in candidates {
        if let Ok(txt) = std::fs::read_to_string(&p) {
            if let Ok(cfg) = serde_json::from_str::<BrandingConfig>(&txt) {
                let base = p.parent().map(|d| d.to_path_buf()).unwrap_or_default();
                return (cfg, base);
            }
        }
    }
    (BrandingConfig::default(), std::path::PathBuf::new())
}

#[tauri::command]
fn branding() -> Branding {
    let (cfg, base_dir) = load_branding();
    let logo_data_uri = {
        let raw = cfg.logo.trim();
        if raw.is_empty() {
            String::new()
        } else {
            // Absoluut pad blijft zoals het is; een relatief pad lost op t.o.v.
            // de map waarin branding.json gevonden is (portable distributie).
            let p = std::path::Path::new(raw);
            let full = if p.is_absolute() { p.to_path_buf() } else { base_dir.join(raw) };
            match std::fs::read(&full) {
                Ok(bytes) => format!("data:{};base64,{}", logo_mime(raw), b64(&bytes)),
                Err(_) => String::new(),
            }
        }
    };
    Branding {
        app_name: cfg.app_name,
        subtitle: cfg.subtitle,
        logo_data_uri,
        window_title: cfg.window_title,
        theme: cfg.theme,
        skin: cfg.skin,
        skin_name: cfg.skin_name,
        font: cfg.font,
        garble: cfg.garble,
    }
}

// ===== Spraak (TTS + STT) =====
// TTS: Windows-native stemmen via System.Speech (SAPI) in een PowerShell-
// kindproces -- geen downloads, geen cloud, geen extra Rust-deps. Het script
// gaat als -EncodedCommand (base64/UTF-16LE) mee zodat er geen enkel
// quoting-probleem is met arbitraire tekst.
fn ps_encoded(script: &str) -> std::process::Command {
    let utf16: Vec<u8> = script.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    let mut c = std::process::Command::new("powershell.exe");
    c.args(["-NoProfile", "-NonInteractive", "-EncodedCommand", &b64(&utf16)]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    c
}

// Draai een PS-script dat per stem "<taal>\t<naam>" print en voeg elke stem toe als
// "<engine>|<taal>|<naam>" (taal = BCP-47, bijv. nl-NL). De frontend toont de taal
// leesbaar en groepeert; speak_text kiest de engine op de tag (taal wordt genegeerd).
fn push_voices(out: &mut Vec<String>, engine: &str, script: &str) {
    if let Ok(o) = ps_encoded(script).output() {
        for l in String::from_utf8_lossy(&o.stdout).lines() {
            let mut it = l.trim_end().splitn(2, '\t');
            let lang = it.next().unwrap_or("").trim();
            let name = it.next().unwrap_or("").trim();
            if !name.is_empty() {
                out.push(format!("{}|{}|{}", engine, lang, name));
            }
        }
    }
}

// Stemmen getagd als "engine|taal|naam". WinRT/OneCore eerst (de rijkere set: bevat
// natuurlijke stemmen en andere talen zoals het Nederlandse 'Microsoft Frank', dat
// alleen in OneCore staat), SAPI als backup. Niet filteren op "Natural" -- dan
// zouden juist die stemmen wegvallen.
#[tauri::command]
fn list_tts_voices() -> Vec<String> {
    let mut out = Vec::new();
    push_voices(
        &mut out,
        "winrt",
        "$null=[Windows.Media.SpeechSynthesis.SpeechSynthesizer,Windows.Media,ContentType=WindowsRuntime]; \
         [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices | ForEach-Object { \"{0}`t{1}\" -f $_.Language, $_.DisplayName }",
    );
    push_voices(
        &mut out,
        "sapi",
        "Add-Type -AssemblyName System.Speech; \
         (New-Object System.Speech.Synthesis.SpeechSynthesizer).GetInstalledVoices() \
         | ForEach-Object { \"{0}`t{1}\" -f $_.VoiceInfo.Culture.Name, $_.VoiceInfo.Name }",
    );
    out
}

// Spreek tekst uit (asynchroon kindproces; blokkeert de UI nooit). Tekst wordt
// afgekapt en PS-single-quote-ge-escaped; een onbekende stem valt stil terug
// op de default.
#[tauri::command]
fn speak_text(text: String, voice: String, rate: i32) -> Result<(), String> {
    let t: String = text.chars().take(2000).collect::<String>().replace('\'', "''");
    if t.trim().is_empty() {
        return Ok(());
    }
    let r = rate.clamp(-10, 10);
    // Tag "engine|taal|naam" (taal genegeerd bij spreken). Terugval: oud "engine|naam"
    // (2 velden) en ongetagd = klassieke SAPI.
    let parts: Vec<&str> = voice.splitn(3, '|').collect();
    let (engine, name) = match parts.as_slice() {
        [e, _lang, n] => (*e, *n),
        [e, n] => (*e, *n),
        _ => ("sapi", voice.as_str()),
    };
    let name_esc = name.replace('\'', "''");
    let script = if engine == "winrt" {
        // Natuurlijke stem via WinRT: synthese -> WAV-stream -> tijdelijke .wav ->
        // synchroon afspelen. Bij ELKE fout terugval op de klassieke SAPI-default,
        // zodat een selectie nooit stil blijft. WinRT SpeakingRate: 0.5..~2.0 (1.0 =
        // normaal); we mappen -10..10 daarop.
        let rate_w = (1.0 + (r as f64) * 0.08).max(0.5);
        let tmpl = r#"
try {
  Add-Type -AssemblyName System.Runtime.WindowsRuntime
  $null=[Windows.Media.SpeechSynthesis.SpeechSynthesizer,Windows.Media,ContentType=WindowsRuntime]
  $null=[Windows.Storage.Streams.DataReader,Windows.Storage.Streams,ContentType=WindowsRuntime]
  function Await($t,$ty){ $m=([System.WindowsRuntimeSystemExtensions].GetMethods()|?{$_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1'})[0].MakeGenericMethod($ty); $nt=$m.Invoke($null,@($t)); [void]$nt.Wait(-1); $nt.Result }
  $s=New-Object Windows.Media.SpeechSynthesis.SpeechSynthesizer
  $v=[Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices|?{$_.DisplayName -eq '__NAME__'}|Select-Object -First 1
  if($v){$s.Voice=$v}
  $s.Options.SpeakingRate=[double]__RATEW__
  $st=Await ($s.SynthesizeTextToStreamAsync('__TEXT__')) ([Windows.Media.SpeechSynthesis.SpeechSynthesisStream])
  $rd=New-Object Windows.Storage.Streams.DataReader($st)
  [void](Await ($rd.LoadAsync([uint32]$st.Size)) ([uint32]))
  $b=New-Object byte[] ([int]$st.Size)
  $rd.ReadBytes($b)
  $p='__WAV__'
  [System.IO.File]::WriteAllBytes($p,$b)
  (New-Object System.Media.SoundPlayer($p)).PlaySync()
  Remove-Item -LiteralPath $p -ErrorAction SilentlyContinue
} catch {
  Add-Type -AssemblyName System.Speech
  $f=New-Object System.Speech.Synthesis.SpeechSynthesizer
  $f.Rate=__RATES__
  $f.Speak('__TEXT__')
}
"#;
        // Uniek WAV-pad per aanroep (PID + teller): een vaste naam liet twee
        // overlappende speaks of twee Taurus-instanties elkaars bestand
        // overschrijven tijdens het afspelen (#80). Het script ruimt het
        // bestand zelf op na PlaySync.
        static TTS_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = TTS_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let wav = std::env::temp_dir().join(format!("taurus-tts-{}-{}.wav", std::process::id(), n));
        let wav_esc = wav.to_string_lossy().replace('\'', "''");
        tmpl.replace("__NAME__", &name_esc)
            .replace("__TEXT__", &t)
            .replace("__RATEW__", &format!("{:.2}", rate_w))
            .replace("__RATES__", &r.to_string())
            .replace("__WAV__", &wav_esc)
    } else {
        let sel = if name_esc.trim().is_empty() {
            String::new()
        } else {
            format!("try {{ $s.SelectVoice('{}') }} catch {{}}; ", name_esc)
        };
        format!(
            "Add-Type -AssemblyName System.Speech; \
             $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
             {}$s.Rate = {}; $s.Speak('{}')",
            sel, r, t
        )
    };
    ps_encoded(&script).spawn().map(|_| ()).map_err(|e| e.to_string())
}

// --- STT: opname op een eigen audio-thread (cpal Streams zijn !Send) ---
enum RecCmd {
    // Antwoord: lukte het starten van de opname? Zonder dit kanaal bleef een
    // mislukte start (geen microfoon) stil: de widget toonde "Luisteren…"
    // terwijl er niets opgenomen werd, en de fout kwam pas bij stop (#75).
    Start(std::sync::mpsc::Sender<Result<(), String>>),
    // Antwoord: pad van de geschreven WAV, of een fout.
    Stop(std::sync::mpsc::Sender<Result<std::path::PathBuf, String>>),
}

struct SttState {
    tx: std::sync::mpsc::Sender<RecCmd>,
    recording: std::sync::atomic::AtomicBool,
    // Live piekniveau van de microfoon (0..1) tijdens opname; de cpal-callback
    // werkt het bij, de frontend pollt het via stt_level voor de equalizer.
    level: std::sync::Arc<Mutex<f32>>,
}

fn stt_dir() -> std::path::PathBuf {
    config_dir().join("stt")
}

// PID in de naam: twee Taurus-instanties schreven anders over elkaars
// opname heen (#80). Binnen één proces is er hooguit één opname tegelijk.
fn stt_wav_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("taurus-stt-{}.wav", std::process::id()))
}

// Zet het gedeelde piekniveau (0..1) op de grootste absolute sample in dit blok.
fn set_peak(lvl: &Mutex<f32>, peak: f32) {
    if let Ok(mut l) = lvl.lock() {
        *l = peak;
    }
}

fn start_capture(
    level: std::sync::Arc<Mutex<f32>>,
) -> Result<(cpal::Stream, std::sync::Arc<Mutex<Vec<f32>>>, u32, u16), String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let device = cpal::default_host()
        .default_input_device()
        .ok_or("no microphone found")?;
    let cfg = device.default_input_config().map_err(|e| e.to_string())?;
    let rate = cfg.sample_rate().0;
    let channels = cfg.channels();
    let buf = std::sync::Arc::new(Mutex::new(Vec::<f32>::new()));
    let b2 = buf.clone();
    let lvl = level;
    let err_fn = |_e| {};
    let stream = match cfg.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &cfg.into(),
            move |data: &[f32], _| {
                b2.lock().unwrap().extend_from_slice(data);
                set_peak(&lvl, data.iter().fold(0f32, |m, &s| m.max(s.abs())));
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &cfg.into(),
            move |data: &[i16], _| {
                let mut g = b2.lock().unwrap();
                let mut peak = 0f32;
                for s in data {
                    let v = *s as f32 / 32768.0;
                    g.push(v);
                    peak = peak.max(v.abs());
                }
                set_peak(&lvl, peak);
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &cfg.into(),
            move |data: &[u16], _| {
                let mut g = b2.lock().unwrap();
                let mut peak = 0f32;
                for s in data {
                    let v = (*s as f32 - 32768.0) / 32768.0;
                    g.push(v);
                    peak = peak.max(v.abs());
                }
                set_peak(&lvl, peak);
            },
            err_fn,
            None,
        ),
        f => return Err(format!("unsupported sample format: {:?}", f)),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok((stream, buf, rate, channels))
}

// Multichannel middelen naar mono en als 16-bit PCM WAV wegschrijven op de
// native samplerate (sherpa-onnx resamplet zelf).
fn write_wav(buf: &[f32], rate: u32, channels: u16) -> Result<std::path::PathBuf, String> {
    let path = stt_wav_path();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(&path, spec).map_err(|e| e.to_string())?;
    let ch = channels.max(1) as usize;
    for frame in buf.chunks(ch) {
        let mono: f32 = frame.iter().sum::<f32>() / ch as f32;
        w.write_sample((mono.clamp(-1.0, 1.0) * 32767.0) as i16)
            .map_err(|e| e.to_string())?;
    }
    w.finalize().map_err(|e| e.to_string())?;
    Ok(path)
}

fn audio_thread(rx: std::sync::mpsc::Receiver<RecCmd>, level: std::sync::Arc<Mutex<f32>>) {
    let mut current: Option<(cpal::Stream, std::sync::Arc<Mutex<Vec<f32>>>, u32, u16)> = None;
    for cmd in rx {
        match cmd {
            RecCmd::Start(reply) => {
                let res = if current.is_some() {
                    Ok(())
                } else {
                    match start_capture(level.clone()) {
                        Ok(c) => {
                            current = Some(c);
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                };
                let _ = reply.send(res);
            }
            RecCmd::Stop(reply) => {
                let res = match current.take() {
                    Some((stream, buf, rate, ch)) => {
                        drop(stream); // stopt de opname
                        set_peak(&level, 0.0); // meter terug naar nul
                        let samples = buf.lock().unwrap();
                        if samples.len() < 1600 {
                            Err("recording too short".to_string())
                        } else {
                            write_wav(&samples, rate, ch)
                        }
                    }
                    None => Err("no recording in progress".to_string()),
                };
                let _ = reply.send(res);
            }
        }
    }
}

// Zoek een bestand (op naam-predicaat) onder de stt-map, max `depth` niveaus.
fn find_under(dir: &Path, pred: &dyn Fn(&str) -> bool, depth: i32) -> Option<std::path::PathBuf> {
    if depth < 0 {
        return None;
    }
    let rd = std::fs::read_dir(dir).ok()?;
    let mut dirs = Vec::new();
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_lowercase();
        if p.is_file() && pred(&name) {
            return Some(p);
        }
        if p.is_dir() {
            dirs.push(p);
        }
    }
    for d in dirs {
        if let Some(hit) = find_under(&d, pred, depth - 1) {
            return Some(hit);
        }
    }
    None
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SttStatus {
    engine: bool,
    model: bool,
    downloading: bool,
    recording: bool,
}

fn stt_paths() -> (Option<std::path::PathBuf>, Option<std::path::PathBuf>) {
    let d = stt_dir();
    let exe = find_under(&d, &|n| n == "sherpa-onnx-offline.exe", 3);
    let tokens = find_under(&d, &|n| n == "tokens.txt", 3);
    (exe, tokens)
}

#[tauri::command]
fn stt_status(state: State<AppState>) -> SttStatus {
    let (exe, tokens) = stt_paths();
    SttStatus {
        engine: exe.is_some(),
        model: tokens.is_some(),
        downloading: stt_dir().join(".downloading").is_file(),
        recording: state
            .stt
            .recording
            .load(std::sync::atomic::Ordering::Relaxed),
    }
}

// Is de .downloading-marker een wees? De marker bevat het PID van het
// PowerShell-downloadproces; leeft dat niet meer (crash, kill, reboot), dan
// is de marker oud vuil en mag een nieuwe download gewoon starten.
fn download_marker_stale(d: &Path) -> bool {
    let pid = match std::fs::read_to_string(d.join(".downloading")) {
        Ok(s) => s.trim().to_string(),
        Err(_) => return false,
    };
    if pid.is_empty() || !pid.chars().all(|c| c.is_ascii_digit()) {
        return true; // marker van een oude build zonder PID
    }
    let mut c = std::process::Command::new("tasklist");
    c.args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000);
    }
    match c.output() {
        Ok(o) => !String::from_utf8_lossy(&o.stdout).contains(&format!("\"{}\"", pid)),
        Err(_) => false, // bij twijfel een lopende download niet overrulen
    }
}

fn dl_log(d: &Path, msg: &str) {
    use std::io::Write as _;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(d.join("download.log"))
    {
        let _ = writeln!(f, "{}", msg);
    }
}

// Download één bestand met PowerShell (blokkerend; wij zitten al op een
// worker-thread). Eerst naar .part, daarna atomisch hernoemen zodat een
// afgebroken download nooit voor een compleet bestand doorgaat.
fn ps_fetch(url: &str, dest: &Path) -> Result<(), String> {
    let part = dest.with_extension("part");
    let u = url.replace('\'', "''");
    let p = part.to_string_lossy().replace('\'', "''");
    let script = format!(
        "$ErrorActionPreference = 'Stop'; \
         [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; \
         Invoke-WebRequest -Uri '{u}' -OutFile '{p}' -UseBasicParsing"
    );
    let status = ps_encoded(&script).status().map_err(|e| e.to_string())?;
    if !status.success() {
        let _ = std::fs::remove_file(&part);
        return Err(format!("download failed ({}): {}", status, url));
    }
    std::fs::rename(&part, dest).map_err(|e| e.to_string())
}

// SHA-256 van een bestand (streaming, dus ook het ~460 MB-model kan zonder
// alles in het geheugen te laden).
fn file_sha256(path: &Path) -> Result<String, String> {
    use sha2::Digest;
    let mut f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = sha2::Sha256::new();
    std::io::copy(&mut f, &mut hasher).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", hasher.finalize()))
}

// Verifieer een gedownload archief tegen zijn gepinde SHA-256. Mismatch ->
// bestand weg (het is niet te vertrouwen) en een duidelijke fout. De engine
// is een exe die we UITVOEREN; zonder deze check zou een kwaadaardige
// registry-URL of een MITM directe code-executie opleveren (#69).
fn verify_sha256(path: &Path, expected: &str) -> Result<(), String> {
    let want = expected.trim().to_lowercase();
    if want.len() != 64 || !want.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("invalid sha256 for {}: {:?}", path.display(), expected));
    }
    let got = file_sha256(path)?;
    if got != want {
        let _ = std::fs::remove_file(path);
        return Err(format!(
            "sha256 mismatch for {} (expected {}, got {}) — file removed",
            path.display(),
            want,
            got
        ));
    }
    Ok(())
}

// .tar.bz2 in-process uitpakken. NIET de systeem-tar: die is niet overal met
// bzip2 gebouwd (een zlib-only bsdtar hangt er stil op, gezien in het wild).
fn extract_tar_bz2(archive: &Path, dest: &Path) -> Result<(), String> {
    let f = std::fs::File::open(archive).map_err(|e| e.to_string())?;
    let dec = bzip2::read::BzDecoder::new(std::io::BufReader::new(f));
    tar::Archive::new(dec).unpack(dest).map_err(|e| e.to_string())
}

// Download engine + model en pak ze uit, op een eigen worker-thread. De
// marker (met ons eigen PID) maakt de voortgang pollbaar via stt_status en
// een wees-marker (app gecrasht/afgesloten) zelfherstellend bij de volgende
// download-klik. Alles komt in download.log terecht.
#[tauri::command]
fn stt_download(
    app: AppHandle,
    engine_url: String,
    engine_sha256: String,
    model_url: String,
    model_sha256: String,
) -> Result<(), String> {
    for u in [&engine_url, &model_url] {
        if !u.starts_with("https://") {
            return Err(format!("https URLs only: {}", u));
        }
    }
    // Checksums zijn verplicht (ook voor registry-modellen): de engine wordt
    // uitgevoerd, dus zonder pin is elke download blind vertrouwen.
    for s in [&engine_sha256, &model_sha256] {
        let t = s.trim();
        if t.len() != 64 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("model entry is missing a valid sha256 checksum".into());
        }
    }
    let d = stt_dir();
    std::fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    let marker = d.join(".downloading");
    if marker.is_file() {
        if download_marker_stale(&d) {
            let _ = std::fs::remove_file(&marker);
        } else {
            return Err("a download is already running".into());
        }
    }
    std::fs::write(&marker, std::process::id().to_string()).map_err(|e| e.to_string())?;
    std::thread::spawn(move || {
        let res = (|| -> Result<(), String> {
            for (url, sha) in [(engine_url, engine_sha256), (model_url, model_sha256)] {
                let name = url.rsplit('/').next().unwrap_or("archive.tar.bz2");
                let file = d.join(name);
                if !file.is_file() {
                    dl_log(&d, &format!("downloading {}", url));
                    ps_fetch(&url, &file)?;
                }
                // Ook een eerder gedownload (gecached) archief verifieren:
                // pas na een geldige checksum wordt er uitgepakt.
                dl_log(&d, &format!("verifying sha256 of {}", name));
                verify_sha256(&file, &sha)?;
                dl_log(&d, &format!("extracting {}", name));
                extract_tar_bz2(&file, &d)?;
            }
            Ok(())
        })();
        match res {
            Ok(()) => {
                dl_log(&d, "done");
                // STT is nu bruikbaar: F9 alsnog registreren (bij het opstarten
                // is dat overgeslagen omdat engine/model nog ontbraken, #79).
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                let _ = app.global_shortcut().register("F9");
            }
            Err(e) => dl_log(&d, &format!("FAILED: {}", e)),
        }
        let _ = std::fs::remove_file(d.join(".downloading"));
    });
    Ok(())
}

// Toggle: eerste aanroep start de opname, tweede stopt hem, transcribeert de
// WAV met de sherpa-onnx sidecar (NeMo-transducer zoals Parakeet v3) en geeft
// de tekst terug. De frontend plaatst die in de actieve terminal.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SttToggle {
    recording: bool,
    text: Option<String>,
}

#[tauri::command]
fn stt_toggle(state: State<AppState>) -> Result<SttToggle, String> {
    use std::sync::atomic::Ordering;
    if !state.stt.recording.load(Ordering::Relaxed) {
        let (exe, tokens) = stt_paths();
        if exe.is_none() || tokens.is_none() {
            return Err("STT model not installed — download it under Settings → Voice".into());
        }
        // Wacht op het antwoord van de audio-thread: een mislukte start (geen
        // microfoon) komt zo METEEN terug i.p.v. pas bij stop (#75).
        let (reply_tx, reply_rx) = std::sync::mpsc::channel();
        state
            .stt
            .tx
            .send(RecCmd::Start(reply_tx))
            .map_err(|e| e.to_string())?;
        reply_rx.recv().map_err(|e| e.to_string())??;
        state.stt.recording.store(true, Ordering::Relaxed);
        return Ok(SttToggle { recording: true, text: None });
    }
    let (reply_tx, reply_rx) = std::sync::mpsc::channel();
    state
        .stt
        .tx
        .send(RecCmd::Stop(reply_tx))
        .map_err(|e| e.to_string())?;
    state.stt.recording.store(false, Ordering::Relaxed);
    let wav = reply_rx.recv().map_err(|e| e.to_string())??;
    let res = transcribe(&wav).map(|text| SttToggle { recording: false, text: Some(text) });
    let _ = std::fs::remove_file(&wav); // opname is verwerkt; niets in temp laten slingeren
    res
}

// Live microfoon-piekniveau (0..1) tijdens opname; de frontend pollt dit voor de
// equalizer rond de opnameknop. 0 als er niet wordt opgenomen.
#[tauri::command]
fn stt_level(state: State<AppState>) -> f32 {
    state.stt.level.lock().map(|l| *l).unwrap_or(0.0)
}

fn transcribe(wav: &Path) -> Result<String, String> {
    let (exe, tokens) = stt_paths();
    let (exe, tokens) = (exe.ok_or("engine missing")?, tokens.ok_or("model missing")?);
    let model_dir = tokens.parent().ok_or("model folder missing")?.to_path_buf();
    let onnx = |frag: &str| {
        find_under(&model_dir, &|n| n.contains(frag) && n.ends_with(".onnx"), 1)
            .ok_or_else(|| format!("{}*.onnx not found in the model folder", frag))
    };
    let (enc, dec, joi) = (onnx("encoder")?, onnx("decoder")?, onnx("joiner")?);
    let mut c = std::process::Command::new(&exe);
    c.arg(format!("--encoder={}", enc.display()))
        .arg(format!("--decoder={}", dec.display()))
        .arg(format!("--joiner={}", joi.display()))
        .arg(format!("--tokens={}", tokens.display()))
        .arg("--model-type=nemo_transducer")
        .arg(wav);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000);
    }
    let out = c.output().map_err(|e| e.to_string())?;
    let all = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    // sherpa-onnx-offline print per bestand een JSON-regel met o.a. "text".
    for line in all.lines() {
        let l = line.trim();
        if l.starts_with('{') {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                if let Some(t) = v.get("text").and_then(|t| t.as_str()) {
                    return Ok(t.trim().to_string());
                }
            }
        }
    }
    Err(format!(
        "no transcript in sidecar output: {}",
        all.chars().take(400).collect::<String>()
    ))
}

// Zet op Windows de WebView2 browser-accelerator-keys uit (F5, Ctrl+R,
// Ctrl+Shift+R, Ctrl+Shift+I/devtools enz.). Die toetsen herladen of
// onderbreken de webview en wissen daarmee alle agent-tabs uit beeld terwijl de
// Claude-processen als zombies in de backend achterblijven. De JS-handler vangt
// F5/Ctrl+R ook af; dit is de waterdichte laag eronder.
#[cfg(target_os = "windows")]
fn disable_accelerator_keys(app: &tauri::AppHandle) {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.with_webview(|webview| {
            use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings3;
            use windows::core::Interface;
            unsafe {
                if let Ok(core) = webview.controller().CoreWebView2() {
                    if let Ok(settings) = core.Settings() {
                        if let Ok(s3) = settings.cast::<ICoreWebView2Settings3>() {
                            let _ = s3.SetAreBrowserAcceleratorKeysEnabled(false);
                        }
                    }
                }
            }
        });
    }
}

// Stop alle nog draaiende claude-processen. Wordt aangeroepen als het venster
// sluit (X-knop, Alt+F4), zodat er geen agents als zombie blijven draaien.
fn kill_all_sessions(app: &tauri::AppHandle) {
    use tauri::Manager;
    let state = app.state::<AppState>();
    let mut map = state.sessions.lock().unwrap();
    for (_, s) in map.iter_mut() {
        let _ = s.child.kill();
    }
    map.clear();
}

// --------------------------------------------------------------------------
// Taurus als SSH-host (#121)
// --------------------------------------------------------------------------

#[derive(serde::Serialize)]
struct SshHostStatus {
    // `desired` is het vinkje, `running` is of de deur ook echt open staat.
    // Die twee lopen uiteen op een niet-vertrouwd netwerk, en dat verschil moet
    // zichtbaar zijn -- anders lijkt het vinkje zichzelf te hebben uitgezet.
    desired: bool,
    running: bool,
    port: u16,
    // De fingerprint van ONZE hostkey: die kan een collega vergelijken met wat
    // zijn ssh-client bij de eerste verbinding toont.
    fingerprint: String,
    networks: Vec<sshhost::netgate::NetInfo>,
}

fn ssh_status_of(state: &AppState) -> SshHostStatus {
    SshHostStatus {
        desired: state.ssh.is_desired(),
        running: state.ssh.is_running(),
        port: *state.ssh.port.lock().unwrap(),
        fingerprint: sshhost::host_key_fingerprint(),
        networks: sshhost::netgate::current_networks(),
    }
}

// Draait dit exemplaar op een eigen configmap? De frontend zet dat in de
// titelbalk. Niet vanuit Rust doen: branding zet de venstertitel na de start
// opnieuw en wist zo'n markering meteen weer uit.
#[tauri::command]
fn is_test_instance() -> bool {
    std::env::var("TAURUS_CONFIG_DIR").map(|s| !s.trim().is_empty()).unwrap_or(false)
}

#[tauri::command]
fn ssh_host_status(state: State<AppState>) -> SshHostStatus {
    ssh_status_of(&state)
}

// "Vertrouw dit netwerk". Pas daarna gaat de listener open -- en bij een
// wisseling naar een onbekend netwerk vanzelf weer dicht.
#[tauri::command]
fn ssh_network_trust(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    trusted: bool,
) -> Result<SshHostStatus, String> {
    sshhost::netgate::set_trusted(&id, trusted)?;
    // Meteen laten meebewegen in plaats van tot de volgende ronde wachten.
    if state.ssh.is_desired() {
        let port = *state.ssh.port.lock().unwrap();
        sshhost::set_enabled(app.clone(), state.ssh.clone(), true, port)?;
    }
    // Een ander netwerk betekent een ander adres om op aan te kondigen -- of geen.
    sync_announcement(&app);
    Ok(ssh_status_of(&state))
}

// Het vinkje in Instellingen. Uit is de default en blijft de default: dit zet
// een deur open op het netwerk en dat hoort een bewuste handeling te zijn.
#[tauri::command]
fn ssh_host_set(
    app: AppHandle,
    state: State<AppState>,
    enabled: bool,
    port: Option<u16>,
) -> Result<SshHostStatus, String> {
    let p = port.unwrap_or(sshhost::DEFAULT_PORT);
    sshhost::set_enabled(app.clone(), state.ssh.clone(), enabled, p)?;
    sync_announcement(&app);
    Ok(ssh_status_of(&state))
}

// De aankondiging volgt zowel de listener als de vraag: er valt niets aan te
// kondigen als de deur dicht staat, en er valt niets aan te kondigen als niemand
// iets vraagt. Zodra een van beide wegvalt, verdwijnt de vraag van het netwerk.
fn sync_announcement(app: &AppHandle) {
    let state = app.state::<AppState>();
    let ask = state.asking.lock().unwrap().clone();
    let Some(ask) = ask.filter(|_| state.ssh.is_running()) else {
        state.discovery.unannounce();
        return;
    };
    let port = *state.ssh.port.lock().unwrap();
    let user = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_default();
    // Lukt aankondigen niet, dan is dat geen reden om de vraag in te trekken -- de
    // deur werkt ook als niemand hem kan vinden; het scherm meldt het verschil.
    let _ = state.discovery.announce(
        port,
        &user,
        &sshhost::host_key_fingerprint(),
        &ask.title,
        &ask.cwd,
        &ask.token,
    );
}

// ---------- de hand opsteken (#125) ----------
//
// Een verzoek wijst altijd naar ÉÉN agent. Dat is niet netter maar noodzakelijk:
// zonder agent kom je uit "op een computer", en dan kan het antwoord een kale
// prompt zijn. Met een agent erbij kan dat per constructie niet.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct HelpRequest {
    // Het lokale sessie-id waar de helper aan gaat meelezen.
    session: String,
    title: String,
    cwd: String,
    // Eenmalig; de uitnodiging is de toestemming, dus dit is het hele bewijs.
    token: String,
}

// Een token dat niet te raden hoeft te zijn om te tellen, maar het wel is: hij
// reist alleen over het vertrouwde netwerk en leeft zolang de vraag open staat.
fn new_token() -> String {
    // rand 0.10: `Rng` is de trait met fill_bytes, `RngExt` die met random_range.
    use rand::Rng;
    const T: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut bytes = [0u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| T[*b as usize % T.len()] as char).collect()
}

#[tauri::command]
fn help_ask(
    app: AppHandle,
    state: State<AppState>,
    session: String,
    title: String,
    cwd: String,
) -> Result<HelpRequest, String> {
    if !state.ssh.is_running() {
        return Err(
            "Zet eerst 'bereikbaar' aan op een vertrouwd netwerk -- anders kan niemand je vraag beantwoorden."
                .to_string(),
        );
    }
    let req = HelpRequest { session, title, cwd, token: new_token() };
    *state.asking.lock().unwrap() = Some(req.clone());
    sync_announcement(&app);
    Ok(req)
}

#[tauri::command]
fn help_withdraw(app: AppHandle, state: State<AppState>) {
    *state.asking.lock().unwrap() = None;
    sync_announcement(&app);
}

// Klopt dit token bij de openstaande vraag? Alleen kijken, niet innemen: dit is de
// auth-fase, en er is nog geen kanaal om aan te hangen.
pub(crate) fn help_token_matches(app: &AppHandle, token: &str) -> bool {
    let state = app.state::<AppState>();
    let ask = state.asking.lock().unwrap();
    ask.as_ref()
        .map(|a| !a.token.trim().is_empty() && a.token == token)
        .unwrap_or(false)
}

// Het token inwisselen, vanuit de SSH-host. De vraag gaat er meteen af: wie hem
// beantwoordt, heeft hem beantwoord, en een hand die omhoog blijft nadat er iemand
// gekomen is nodigt de rest van de gang voor niets uit.
pub(crate) fn claim_help_offer(app: &AppHandle, token: &str) -> Option<String> {
    let state = app.state::<AppState>();
    let ask = state.asking.lock().unwrap().clone()?;
    if ask.token.trim().is_empty() || ask.token != token {
        return None;
    }
    *state.asking.lock().unwrap() = None;
    sync_announcement(app);
    let _ = app.emit("help-answered", ());
    Some(ask.session)
}

// Meelezen met een lokale sessie: dezelfde bytes die naar het venster gaan.
pub(crate) fn subscribe_local_session(id: &str) -> std::sync::mpsc::Receiver<Vec<u8>> {
    offer_subscribe(id)
}

// En de terugweg: wat de helper typt gaat de LOKALE pty in, dezelfde terminal waar
// de vrager in zit. Twee toetsenborden op een agent, net als de join uit #121 maar
// dan andersom.
pub(crate) fn write_local_session(app: &AppHandle, id: &str, data: &[u8]) {
    let state = app.state::<AppState>();
    let mut map = state.sessions.lock().unwrap();
    if let Some(s) = map.get_mut(id) {
        let _ = s.writer.write_all(data);
        let _ = s.writer.flush();
    }
}

#[tauri::command]
fn help_asking(state: State<AppState>) -> Option<HelpRequest> {
    state.asking.lock().unwrap().clone()
}

// ---------- machines vinden op het vertrouwde netwerk (#125) ----------

// Een gevonden machine, plus of hij al bekend is. Bekend = niet nog een keer in
// de gevonden-lijst: hij staat dan al boven, met zijn routes en zijn knoppen.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FoundMachine {
    #[serde(flatten)]
    found: discovery::Found,
    known: bool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DiscoveryView {
    machines: Vec<FoundMachine>,
    // Waarom de lijst leeg is, als hij leeg is. Een lege lijst die "geblokkeerd"
    // betekent leest als "er is niemand", en dat is de verkeerde conclusie.
    problem: String,
    // Kondigen we zelf aan? Zo niet, dan ziet niemand ONS -- ook goed om te weten
    // op een scherm dat over vindbaarheid gaat.
    announcing: bool,
}

// Zoeken loopt alleen terwijl het machinescherm open staat. Dat is de hele
// invulling van "discovery is passief": geen melding, geen badge, geen popup.
#[tauri::command]
fn discovery_start(state: State<AppState>) -> Result<(), String> {
    state.discovery.browse_start()
}

#[tauri::command]
fn discovery_stop(state: State<AppState>) {
    state.discovery.browse_stop();
}

#[tauri::command]
fn discovered_machines(state: State<AppState>) -> DiscoveryView {
    let hosts = get_hosts();
    let machines = state
        .discovery
        .list()
        .into_iter()
        .map(|f| {
            let known = hosts
                .iter()
                .any(|h| h.hostname.eq_ignore_ascii_case(&f.address));
            FoundMachine { found: f, known }
        })
        .collect();
    DiscoveryView {
        machines,
        problem: state.discovery.problem(),
        announcing: sshhost::read_pref().enabled && sshhost::netgate::on_trusted_network(),
    }
}

// Een hulpvraag beantwoorden (#125). Er wordt niets gestart en niets opgeslagen:
// dit opent een tab op de sessie die de ander al draait.
//
// De machine hoeft niet in hosts.json te staan en er is geen sleutel nodig -- het
// token in de gebruikersnaam is het bewijs, en het geldt alleen voor deze vraag.
// Daarom ook geen `adopt`: iemand helpen maakt hem nog niet tot een van jouw
// machines.
#[tauri::command]
fn answer_help_request(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    gen: u64,
    found: discovery::Found,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    if found.token.trim().is_empty() {
        return Err("Dit verzoek heeft geen token meer; vraag de ander opnieuw.".to_string());
    }
    let host = Host {
        id: "help".into(),
        nickname: found.name.clone(),
        hostname: found.address.clone(),
        machine: found.address.clone(),
        // Het token reist als gebruikersnaam: de andere kant herkent het in de
        // auth-fase en laat precies dit ene pad toe.
        user: format!("taurus-help-{}", found.token),
        port: found.port,
        key_path: String::new(),
        default_project: String::new(),
        via: String::new(),
        os: found.os.clone(),
        mux: "none".into(),
        agent_version: String::new(),
        mux_auto: false,
    };
    let (program, args) = ssh_interactive(&host, format!("TAURUS-JOIN {}", found.token))?;
    start_pty(
        &app,
        &state.sessions,
        id,
        gen,
        program,
        &local_cwd_for_remote(),
        args,
        cols,
        rows,
    )
}

// ---------- de twee firewall-uitzonderingen, in één handeling (#125) ----------
//
// GEMETEN: elke bestaande allow-regel voor mDNS op deze machine is PROGRAMMA-
// gebonden (svchost voor Windows' eigen responder, msedgewebview2 voor Edge).
// Taurus valt daar niet onder, dus `taurus.exe` op UDP 5353 heeft een eigen regel
// nodig -- naast de poortregel voor TCP 8287 die #121 al vraagt.
//
// Ze samen aanbieden bij het aanzetten van "bereikbaar" is het verschil tussen één
// bewuste handeling en twee verrassingen op twee momenten: de tweede zou pas komen
// bovendrijven wanneer iemand zich afvraagt waarom niemand hem ziet staan.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FirewallStatus {
    // Staan ONZE eigen regels er: aan, inbound, toestaan?
    tcp: bool,
    udp: bool,
    // Regels die deze exe expliciet BLOKKEREN. Geen bijzaak: een block-regel wint in
    // Windows Firewall van elke allow-regel, dus zolang die er staat is Taurus dicht,
    // hoeveel uitzonderingen je er ook naast zet.
    //
    // GEMETEN op dit werkstation: er stonden er twee, "TCP Query User{...}" en
    // "UDP Query User{...}", allebei op het pad van taurus.exe. Dat zijn de
    // automatisch aangemaakte regels van een weggeklikte Defender-prompt -- precies
    // wat er tijdens de spike gebeurde en toen met de hand weg moest.
    blocked: u32,
    // Kon het überhaupt nagekeken worden? Op een niet-Windows-machine, of als de
    // cmdlet ontbreekt, is "nee" iets anders dan "geen regel".
    checked: bool,
}

const FW_TCP_RULE: &str = "Taurus";
const FW_UDP_RULE: &str = "Taurus mDNS";

fn this_exe() -> String {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

// Twee vragen die snel te stellen zijn: staan onze eigen regels er, en blokkeert
// iets deze exe?
//
// De derde vraag -- "laat een WILLEKEURIGE regel deze poort door?" -- is geprobeerd
// en weer weggegooid, om twee gemeten redenen:
//
//   1. `Get-NetFirewallPortFilter` als LOSSE lijst is niet op InstanceID te koppelen
//      aan `Get-NetFirewallRule`. De 599 objecten die hij teruggaf bevatten de
//      InstanceID van onze eigen regel niet, terwijl de associatie
//      (`$rule | Get-NetFirewallPortFilter`) hem wél geeft. Een eerdere versie
//      koppelde zo en antwoordde daardoor ALTIJD "nee" -- ook nadat de regels
//      aantoonbaar waren aangemaakt. (De app-filters koppelen wél gewoon op
//      InstanceID; alleen de poortfilters niet.)
//   2. Het via de associatie in bulk doen klopt wel, maar duurde 39,7 seconden.
//
// Deze versie doet er 6. Een handgemaakte regel onder een andere naam telt niet mee;
// daarom zegt de tekst "een eigen uitzondering", en het ergste gevolg is een
// overbodige regel erbij.
fn firewall_script(exe: &str) -> String {
    format!(
        "$ErrorActionPreference='SilentlyContinue'\n\
         $exe = '{exe}'\n\
         function Mine($naam) {{\n\
         \x20 $r = Get-NetFirewallRule -DisplayName $naam -EA 0 | Where-Object {{ $_.Enabled -eq 'True' -and $_.Direction -eq 'Inbound' -and $_.Action -eq 'Allow' }}\n\
         \x20 return [bool]$r\n\
         }}\n\
         'TCP=' + (Mine '{tcp}')\n\
         'UDP=' + (Mine '{udp}')\n\
         $apps = @{{}}\n\
         foreach ($a in Get-NetFirewallApplicationFilter) {{ $apps[$a.InstanceID] = $a.Program }}\n\
         $n = 0\n\
         foreach ($r in Get-NetFirewallRule -Direction Inbound -Action Block -Enabled True) {{\n\
         \x20 $p = $apps[$r.InstanceID]\n\
         \x20 if ($p -and [Environment]::ExpandEnvironmentVariables($p) -eq $exe) {{ $n = $n + 1 }}\n\
         }}\n\
         'BLOCKED=' + $n\n",
        exe = exe.replace('\'', "''"),
        tcp = FW_TCP_RULE,
        udp = FW_UDP_RULE,
    )
}

#[tauri::command]
fn firewall_status(port: Option<u16>) -> FirewallStatus {
    let _ = port; // de poort zit in de regelnaam, niet in de vraag
    let dicht = FirewallStatus { tcp: false, udp: false, blocked: 0, checked: false };
    let Ok(o) = ps_encoded(&firewall_script(&this_exe())).output() else {
        return dicht;
    };
    let out = String::from_utf8_lossy(&o.stdout);
    let flag = |k: &str| {
        out.lines()
            .find_map(|l| l.trim().strip_prefix(k))
            .map(|v| v.trim().eq_ignore_ascii_case("true"))
    };
    let count = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("BLOCKED="))
        .and_then(|v| v.trim().parse::<u32>().ok());
    match (flag("TCP="), flag("UDP="), count) {
        (Some(t), Some(u), Some(b)) => FirewallStatus { tcp: t, udp: u, blocked: b, checked: true },
        _ => dicht,
    }
}

// Alles wat nodig is in ÉÉN UAC-prompt: de ontbrekende regels erbij, en de
// block-regels tegen deze exe eraf. Dat laatste hoort erbij en niet apart -- een
// allow-regel is zinloos zolang er een block-regel naast staat, en wie "maak de
// regels" aanklikt bedoelt "zorg dat Taurus erdoor kan".
//
// Bewust smal: alleen INBOUND, alleen BLOCK, en alleen als het programmafilter
// precies deze exe is. Er wordt niets anders aangeraakt.
#[tauri::command]
fn firewall_allow(port: Option<u16>) -> Result<(), String> {
    let p = port.unwrap_or(sshhost::DEFAULT_PORT);
    let exe = this_exe();
    let st = firewall_status(Some(p));
    if st.checked && st.tcp && st.udp && st.blocked == 0 {
        return Ok(());
    }
    // Windows PowerShell 5.1, ASCII: dit draait op de machine van de gebruiker en
    // niet per se op pwsh 7. De verhoogde kant kijkt zélf opnieuw wat er moet
    // gebeuren; namen van buiten meegeven zou quoting-gevoelig zijn, want een
    // "TCP Query User{...}C:\pad\taurus.exe" zit vol accolades en backslashes.
    let inner = format!(
        "$ErrorActionPreference='Stop'\n\
         $exe = '{exe}'\n\
         if (-not (Get-NetFirewallRule -DisplayName '{tcp}' -EA 0)) {{ New-NetFirewallRule -DisplayName '{tcp}' -Direction Inbound -Action Allow -Protocol TCP -LocalPort {p} | Out-Null }}\n\
         if (-not (Get-NetFirewallRule -DisplayName '{udp}' -EA 0)) {{ New-NetFirewallRule -DisplayName '{udp}' -Direction Inbound -Action Allow -Protocol UDP -LocalPort 5353 | Out-Null }}\n\
         $apps = @{{}}\n\
         foreach ($a in Get-NetFirewallApplicationFilter) {{ $apps[$a.InstanceID] = $a.Program }}\n\
         foreach ($r in Get-NetFirewallRule -Direction Inbound -Action Block) {{\n\
         \x20 $prog = $apps[$r.InstanceID]\n\
         \x20 if ($prog -and [Environment]::ExpandEnvironmentVariables($prog) -eq $exe) {{\n\
         \x20   Remove-NetFirewallRule -Name $r.Name -EA 0\n\
         \x20 }}\n\
         }}\n",
        exe = exe.replace('\'', "''"),
        tcp = FW_TCP_RULE,
        udp = FW_UDP_RULE,
    );
    let utf16: Vec<u8> = inner.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    // -WindowStyle Hidden: anders knippert er een consolevenster over het scherm.
    let launcher = format!(
        "try {{ $p = Start-Process powershell -Verb RunAs -WindowStyle Hidden -Wait -PassThru -ArgumentList '-NoProfile','-EncodedCommand','{}'; 'CODE=' + $p.ExitCode }} catch {{ 'ERR=' + $_.Exception.Message }}",
        b64(&utf16)
    );
    let o = ps_encoded(&launcher)
        .output()
        .map_err(|e| format!("PowerShell starten: {e}"))?;
    let out = String::from_utf8_lossy(&o.stdout);
    if let Some(err) = out.lines().find_map(|l| l.trim().strip_prefix("ERR=")) {
        // De meest voorkomende: de UAC-prompt is weggeklikt.
        return Err(format!("De firewall-regels zijn niet aangepast ({err})."));
    }
    let code = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("CODE="))
        .and_then(|v| v.trim().parse::<i32>().ok())
        .unwrap_or(-1);
    if code != 0 {
        return Err(format!("De firewall-regels zijn niet aangepast (afsluitcode {code})."));
    }
    // Meteen nakijken of het ook echt gelukt is. Zonder deze controle meldt de knop
    // succes zodra de prompt is weggeklikt, ook als er niets veranderde -- en dat is
    // precies hoe je een melding krijgt die blijft staan zonder uitleg.
    let na = firewall_status(Some(p));
    if na.checked && (!na.tcp || !na.udp || na.blocked > 0) {
        return Err(
            "De prompt is doorlopen, maar de firewall staat er nog hetzelfde bij. Kijk of beleid van de organisatie deze regels terugzet."
                .to_string(),
        );
    }
    Ok(())
}


// Antwoord op een popup: "deny" | "allow" | "join" | "block" | "always".
#[tauri::command]
fn ssh_consent_reply(state: State<AppState>, id: String, decision: String) {
    state.ssh.consents.reply(&id, &decision);
}

#[tauri::command]
fn ssh_peers() -> Vec<sshhost::Peer> {
    sshhost::read_peers()
}

// Een gekoppelde collega alsnog blokkeren trekt ook de koppeling in: anders zou
// "geblokkeerd" nog steeds een auto-allow met zich meedragen.
#[tauri::command]
fn ssh_peer_set(
    fingerprint: String,
    blocked: Option<bool>,
    auto_allow: Option<bool>,
) -> Result<Vec<sshhost::Peer>, String> {
    let mut peers = sshhost::read_peers();
    if let Some(p) = peers.iter_mut().find(|p| p.fingerprint == fingerprint) {
        if let Some(b) = blocked {
            p.blocked = b;
            if b {
                p.auto_allow = false;
            }
        }
        if let Some(a) = auto_allow {
            p.auto_allow = a;
        }
    }
    sshhost::write_peers(&peers)?;
    Ok(peers)
}

#[tauri::command]
fn ssh_peer_forget(fingerprint: String) -> Result<Vec<sshhost::Peer>, String> {
    let mut peers = sshhost::read_peers();
    peers.retain(|p| p.fingerprint != fingerprint);
    sshhost::write_peers(&peers)?;
    Ok(peers)
}

#[tauri::command]
fn ssh_inbound_sessions(state: State<AppState>) -> Vec<sshhost::InboundSession> {
    state.ssh.sessions.lock().unwrap().values().cloned().collect()
}

// JOIN: de lokale tab typt in dezelfde terminal als de collega. Twee
// toetsenborden op één agent -- dat is precies de bedoeling, dus hier zit geen
// "wie is aan de beurt"-logica.
#[tauri::command]
fn ssh_mirror_write(state: State<AppState>, id: String, data: String) {
    let io = state.ssh.io.lock().unwrap().get(&id).cloned();
    if let Some(io) = io {
        io.write(data.as_bytes());
    }
}

// Twee vensters op één terminal: de kleinste maat wint (zoals tmux).
#[tauri::command]
fn ssh_mirror_resize(state: State<AppState>, id: String, cols: u16, rows: u16) {
    let io = state.ssh.io.lock().unwrap().get(&id).cloned();
    if let Some(io) = io {
        io.set_local_size(cols, rows);
    }
}

// De spiegel-tab sluiten stopt alleen het meekijken; de sessie van de collega
// loopt door. Afkappen is een aparte, expliciete handeling (ssh_kill_session).
#[tauri::command]
fn ssh_mirror_detach(state: State<AppState>, id: String) {
    let io = state.ssh.io.lock().unwrap().get(&id).cloned();
    if let Some(io) = io {
        io.drop_local();
    }
}

#[tauri::command]
fn ssh_kill_session(app: AppHandle, state: State<AppState>, id: String) {
    let io = state.ssh.io.lock().unwrap().get(&id).cloned();
    if let Some(io) = io {
        sshhost::audit(&app, "session-kill", &id, "door de lokale gebruiker");
        io.kill();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Audio-thread voor STT: cpal-streams zijn !Send, dus één eigen thread
    // bezit de stream; commands praten er via een kanaal mee.
    let (stt_tx, stt_rx) = std::sync::mpsc::channel();
    // NB: niet 'stt_level' noemen -- dat zou de gelijknamige command-functie
    // schaduwen in generate_handler! (E0618).
    let mic_level = std::sync::Arc::new(Mutex::new(0.0f32));
    let mic_level_thread = mic_level.clone();
    std::thread::spawn(move || audio_thread(stt_rx, mic_level_thread));

    tauri::Builder::default()
        .manage(AppState {
            sessions: Mutex::new(HashMap::new()),
            stt: SttState {
                tx: stt_tx,
                recording: std::sync::atomic::AtomicBool::new(false),
                level: mic_level,
            },
            ssh: std::sync::Arc::new(sshhost::HostState::default()),
            discovery: std::sync::Arc::new(discovery::Discovery::default()),
            asking: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            // F9 = push-to-talk-toggle, ook zonder vensterfocus. Registratie
            // aan de Rust-kant; de frontend krijgt een event en doet de rest.
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    use tauri_plugin_global_shortcut::ShortcutState;
                    // F9 = push-to-talk-INHOUDEN: indrukken start de opname, loslaten
                    // stopt + transcribeert. De frontend doet de reconciliatie zodat
                    // een korte tik geen halve opname achterlaat.
                    if shortcut.matches(tauri_plugin_global_shortcut::Modifiers::empty(), tauri_plugin_global_shortcut::Code::F9) {
                        match event.state() {
                            ShortcutState::Pressed => { let _ = app.emit("stt-ptt-down", ()); }
                            ShortcutState::Released => { let _ = app.emit("stt-ptt-up", ()); }
                            #[allow(unreachable_patterns)]
                            _ => {}
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "windows")]
            disable_accelerator_keys(app.handle());
            // Stond de SSH-host aan toen je Taurus afsloot? Dan weer aan -- de
            // netwerk-gate beslist alsnog of er echt geluisterd wordt. Zonder
            // dit moest je na elke start opnieuw aanvinken.
            {
                use tauri::Manager;
                let pref = sshhost::read_pref();
                if pref.enabled {
                    let st = app.state::<AppState>();
                    if let Err(e) =
                        sshhost::set_enabled(app.handle().clone(), st.ssh.clone(), true, pref.port)
                    {
                        debug_log(format!("ssh-host bij opstarten: {e}"));
                    }
                }
            }
            {
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                // F9 alleen systeembreed claimen als STT ook echt bruikbaar is
                // (engine + model geinstalleerd); anders blijft de toets vrij
                // voor andere programma's (#79). Na een geslaagde download
                // registreert stt_download hem alsnog. Mislukte registratie
                // (F9 elders in gebruik) is geen ramp: de mic-knop blijft werken.
                let (exe, tokens) = stt_paths();
                if exe.is_some() && tokens.is_some() {
                    let _ = app.global_shortcut().register("F9");
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            use tauri::Manager;
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                kill_all_sessions(window.app_handle());
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_projects,
            save_projects,
            get_hosts,
            machines,
            remote_agents,
            session_history,
            history_record,
            history_mark_open,
            history_forget,
            stop_remote_session,
            discovery_start,
            discovery_stop,
            help_ask,
            help_withdraw,
            help_asking,
            answer_help_request,
            discovered_machines,
            firewall_status,
            firewall_allow,
            save_hosts,
            check_hosts,
            probe_host,
            remote_sessions,
            attach_remote_session,
            tune_herdr,
            scp_to_host,
            survey_workspace,
            survey_remote_workspace,
            push_workspace,
            pull_workspace,
            pick_folder,
            pick_file,
            path_exists,
            app_version,
            has_claude_md,
            save_sessions,
            get_sessions,
            session_state,
            create_session,
            list_agent_models,
            restart_session,
            write_session,
            resize_session,
            close_session,
            list_html,
            read_file,
            open_folder,
            save_dropped_path,
            save_clipboard_to_input,
            copy_to_clipboard,
            branding,
            list_tts_voices,
            speak_text,
            stt_status,
            stt_download,
            stt_toggle,
            stt_level,
            is_test_instance,
            ssh_host_status,
            ssh_host_set,
            ssh_network_trust,
            ssh_consent_reply,
            ssh_peers,
            ssh_peer_set,
            ssh_peer_forget,
            ssh_inbound_sessions,
            ssh_mirror_write,
            ssh_mirror_resize,
            ssh_mirror_detach,
            ssh_kill_session,
            debug_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Unit-tests voor de pure helpers (#82). De grotere opsplitsing in modules
// (en JS-tests na de ES-module-split) blijft in issue #82 openstaan.
#[cfg(test)]
mod tests {
    use super::*;

    fn test_host() -> Host {
        Host {
            id: "h1".into(),
            nickname: "Support".into(),
            hostname: "support01".into(),
            machine: String::new(),
            user: "arjen".into(),
            port: 22,
            key_path: String::new(),
            default_project: String::new(),
            os: "linux".into(),
            mux: "tmux".into(),
            mux_auto: true,
            agent_version: String::new(),
            via: String::new(),
        }
    }

    #[test]
    fn via_wsl_builds_a_linux_payload_and_wraps_it_for_cmd() {
        let mut h = test_host();
        h.os = "windows".into();
        h.via = "wsl".into();
        h.mux = "tmux".into();
        assert_eq!(effective_os(&h), "linux", "binnen WSL is alles Linux");

        let (_, args) = wrap_remote(&h, "/home/arjen/proj", "claude".into(), vec!["-n".into()]).unwrap();
        let payload = args.last().unwrap();
        assert!(payload.starts_with("wsl -e sh -c \""));
        // Alles tussen de dubbele quotes moet shell-neutraal zijn: de echte
        // payload zit als base64 verpakt, want hij bevat zelf quotes en pipes
        // die cmd.exe er anders uit haalt.
        assert!(
            payload.contains("> /tmp/") && payload.contains("&& exec sh -l /tmp/"),
            "moet via een bestand en een LOGIN shell starten -- anders heeft tmux geen tty en staat ~/.local/bin niet op PATH: {}",
            payload
        );
        let inner = payload
            .trim_start_matches("wsl -e sh -c \"echo ")
            .split(' ')
            .next()
            .unwrap();
        assert!(
            inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='),
            "payload is niet zuiver base64: {}",
            inner
        );
        // En het is de tmux-vorm, niet de PowerShell-vorm van een Windows-host.
        let expected = b64(
            build_remote_payload_inner("tmux", "linux", &mux_session_name(&h.id, "/home/arjen/proj"),
                                       "/home/arjen/proj", "claude", &["-n".to_string()])
                .unwrap()
                .as_bytes(),
        );
        assert_eq!(inner, expected);
    }

    #[test]
    fn via_wsl_uses_the_unix_agent_name() {
        // Zonder dit zou "claude.exe" in WSL gezocht worden.
        let mut h = test_host();
        h.os = "windows".into();
        h.via = "wsl".into();
        assert_eq!(remote_agent_program("claude", effective_os(&h)), "claude");
    }

    #[test]
    fn removing_a_marker_actually_takes_it_out_of_the_command() {
        // De valkuil die de eerste poging tot deze fix liet mislukken:
        // CommandBuilder::new() vult zichzelf met de HELE omgeving
        // (get_base_env), dus een marker alleen NIET opnieuw zetten laat de
        // geerfde waarde gewoon staan. Alleen env_remove haalt hem eruit.
        std::env::set_var("TAURUS_TEST_MARKER_KEEP", "yes");
        let mut cmd = CommandBuilder::new("cmd.exe");
        assert!(
            cmd.get_env("TAURUS_TEST_MARKER_KEEP").is_some(),
            "de builder hoort de omgeving te erven"
        );
        cmd.env_remove("TAURUS_TEST_MARKER_KEEP");
        assert!(cmd.get_env("TAURUS_TEST_MARKER_KEEP").is_none());
        std::env::remove_var("TAURUS_TEST_MARKER_KEEP");
    }

    #[test]
    fn session_markers_do_not_leak_into_a_new_agent() {
        // Deze beschrijven de sessie van de agent die Taurus startte; erven zou
        // de nieuwe agent tot kindsessie maken en zijn transcript uitzetten.
        for k in [
            "CLAUDECODE",
            "CLAUDE_CODE_CHILD_SESSION",
            "CLAUDE_CODE_SESSION_ID",
            "CLAUDE_CODE_ENTRYPOINT",
            "CLAUDE_PID",
        ] {
            assert!(inherits_session_marker(k, true), "{} moet gefilterd worden", k);
            assert!(inherits_session_marker(k, false), "{} ook zonder CLAUDECODE", k);
        }
        // Wat de agent WEL nodig heeft blijft staan.
        for k in ["PATH", "USERPROFILE", "APPDATA", "HOME", "CLAUDE_CODE_GIT_BASH_PATH"] {
            assert!(!inherits_session_marker(k, true), "{} mag niet verdwijnen", k);
        }
        // NO_COLOR: alleen weg als de omgeving van een agent komt.
        assert!(inherits_session_marker("NO_COLOR", true));
        assert!(!inherits_session_marker("NO_COLOR", false), "los is het een gebruikersvoorkeur");
    }

    #[test]
    fn survey_separates_work_and_bulk_from_the_rest() {
        let dir = std::env::temp_dir().join(format!("taurus-survey-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("input")).unwrap();
        std::fs::create_dir_all(dir.join("node_modules").join("pkg")).unwrap();
        std::fs::write(dir.join("README.md"), "x".repeat(10)).unwrap();
        std::fs::write(dir.join("src").join("a.rs"), "y".repeat(20)).unwrap();
        std::fs::write(dir.join("input").join("data.csv"), "z".repeat(50)).unwrap();
        std::fs::write(dir.join("node_modules").join("pkg").join("i.js"), "q".repeat(100)).unwrap();

        let s = survey_workspace(dir.to_string_lossy().into_owned());
        assert_eq!(s.error, "");
        // De kern is alles buiten de apart genoemde mappen: README + src.
        assert_eq!(s.core_bytes, 30);
        assert_eq!(s.core_files, 2);
        let work: Vec<_> = s.work.iter().map(|w| (w.name.as_str(), w.bytes)).collect();
        assert_eq!(work, vec![("input", 50)]);
        let bulk: Vec<_> = s.bulk.iter().map(|b| (b.name.as_str(), b.bytes)).collect();
        assert_eq!(bulk, vec![("node_modules", 100)]);
        // De kern moet ook als LOSSE namen komen: terughalen stuurt een expliciete
        // lijst aan scp, want een wildcard zou het uitgevinkte alsnog meenemen.
        let mut core = s.core.clone();
        core.sort();
        assert_eq!(core, vec!["README.md".to_string(), "src".to_string()]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn survey_distinguishes_a_missing_directory_from_an_empty_one() {
        // Een doelmap die nog niet bestaat is het NORMALE geval bij een eerste
        // overzetting, dus geen fout. Maar hij moet wel te onderscheiden zijn
        // van een lege map: anders kan de UI niet zeggen of er iets overschreven
        // gaat worden.
        let missing = survey_workspace(r"C:\taurus-does-not-exist-1234".into());
        assert!(!missing.exists);
        assert_eq!(missing.error, "");
        assert_eq!(missing.newest, 0);

        let dir = std::env::temp_dir().join(format!("taurus-empty-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let empty = survey_workspace(dir.to_string_lossy().into_owned());
        assert!(empty.exists, "een lege map bestaat wel");
        assert_eq!(empty.core_files, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn survey_reports_the_newest_change_in_the_tree() {
        // Hiermee kan de UI zeggen WELKE kant recenter is in plaats van botweg te
        // weigeren als het doel al bestaat.
        let dir = std::env::temp_dir().join(format!("taurus-mtime-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("deep.txt"), "x").unwrap();
        let s = survey_workspace(dir.to_string_lossy().into_owned());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(s.newest > 0, "moet een tijdstempel vinden");
        // Een bestand dat we net schreven ligt hooguit een paar seconden terug --
        // en het moet uit de SUBmap komen, niet alleen van de root.
        assert!(now - s.newest < 120, "onverwacht oude mtime: {} vs {}", s.newest, now);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn remote_join_uses_the_targets_separator_not_ours() {
        // Een backslash in een Linux-pad is een gewoon teken, geen map -- dus de
        // scheiding moet van de DOEL-machine komen, niet van dit werkstation.
        assert_eq!(remote_join("windows", r"C:\proj", "input"), r"C:\proj\input");
        assert_eq!(remote_join("linux", "/home/a/proj", "input"), "/home/a/proj/input");
        // Een al aanwezige scheiding aan het eind mag niet verdubbelen.
        assert_eq!(remote_join("windows", r"C:\proj\", "input"), r"C:\proj\input");
        assert_eq!(remote_join("linux", "/home/a/proj/", "input"), "/home/a/proj/input");
    }

    #[test]
    fn shell_quote_posix_survives_spaces_quotes_and_expansion() {
        assert_eq!(shell_quote_posix("plain"), "'plain'");
        assert_eq!(shell_quote_posix("with space"), "'with space'");
        assert_eq!(shell_quote_posix(""), "''");
        // $ en ` mogen NIET expanderen aan de andere kant.
        assert_eq!(shell_quote_posix("$HOME `id`"), "'$HOME `id`'");
        // De enkele quote zelf: sluiten, geescapet toevoegen, heropenen.
        assert_eq!(shell_quote_posix("it's"), r"'it'\''s'");
        // Een taak die de shell zou kunnen slopen blijft een enkel argument.
        assert_eq!(
            shell_quote_posix("fix; rm -rf / && echo hi"),
            "'fix; rm -rf / && echo hi'"
        );
    }

    #[test]
    fn mux_session_name_is_deterministic_and_tmux_safe() {
        let a = mux_session_name("h1", "/home/arjen/proj");
        assert_eq!(a, mux_session_name("h1", "/home/arjen/proj"), "moet stabiel zijn");
        assert!(a.starts_with("taurus-h1-"));
        // tmux gebruikt '.' en ':' als scheidingstekens in target-namen.
        assert!(
            a.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "onverwacht teken in {}",
            a
        );
        // Andere host of ander project -> andere sessie.
        assert_ne!(a, mux_session_name("h2", "/home/arjen/proj"));
        assert_ne!(a, mux_session_name("h1", "/home/arjen/other"));
    }

    #[test]
    fn mux_session_name_separates_long_paths_that_share_a_prefix() {
        // Precies het geval waarvoor de hash er zit: na afkappen zijn deze twee
        // identiek, en zonder hash zou Taurus aan de verkeerde sessie aanhaken.
        let a = mux_session_name("h1", "/home/arjen/werk/klant-alpha/heel/diep/project-een");
        let b = mux_session_name("h1", "/home/arjen/werk/klant-alpha/heel/diep/project-twee");
        assert_ne!(a, b);
    }

    #[test]
    fn build_remote_payload_wraps_tmux_with_attach_or_create() {
        let args = vec!["-n".to_string(), "my session".to_string()];
        let s = build_remote_payload_inner("tmux", "linux", "taurus-h1-p-0", "/home/a/p", "claude", &args)
            .unwrap();
        assert_eq!(
            s,
            "tmux new-session -A -s 'taurus-h1-p-0' -c '/home/a/p' -- 'claude' '-n' 'my session'"
        );
    }

    #[test]
    fn a_posix_launch_uses_a_login_shell() {
        // `ssh host "cmd"` draait geen login shell, dus ~/.profile blijft
        // ongelezen en ~/.local/bin staat niet op PATH. Een claude die daar
        // staat wordt dan niet gevonden en de sessie eindigt meteen met alleen
        // "[exited]" -- precies wat er tegen een echte host gebeurde.
        let s = build_remote_payload_via("tmux", "linux", "", "sess", "/home/a/p", "claude", &[]).unwrap();
        assert!(s.contains("exec sh -l /tmp/sess.sh"), "moet een login shell zijn: {}", s);
        assert!(s.starts_with("echo "), "payload hoort als base64 mee te reizen: {}", s);
        // Windows loopt via PowerShell en heeft deze starter niet.
        let w = build_remote_payload_via("none", "windows", "", "sess", r"C:\p", "claude", &[]).unwrap();
        assert!(w.starts_with("powershell -NoProfile -EncodedCommand "));
    }

    #[test]
    fn herdr_posix_bootstraps_in_steps_that_are_each_idempotent() {
        let args = vec!["-n".to_string(), "my session".to_string()];
        let s = herdr_posix_script("taurus-h1-p-0", "/home/a/p", "claude", &args);
        // Elke stap moet zelfstandig overslaan wat er al is; anders zet een half
        // opgezette sessie er een tweede agent naast in plaats van aan te haken.
        assert!(s.contains("run pane get w1:p1 >/dev/null 2>&1 || run workspace create --cwd '/home/a/p'"));
        assert!(s.contains("if ! run agent get w1:p1 >/dev/null 2>&1; then"));
        // Attach mag pas als de agent herkend IS; anders komt hij een seconde te
        // vroeg. En wordt hij nooit herkend (agy, command-override), dan is de
        // sessie-TUI de terugval in plaats van een tab die niet opent.
        assert!(s.contains("while [ $i -lt 8 ]; do run agent get w1:p1"));
        assert!(s.contains("if run agent get w1:p1 >/dev/null 2>&1; then exec \"$H\" --session 'taurus-h1-p-0' agent attach w1:p1; fi"));
        assert!(s.trim_end().ends_with("exec \"$H\" --session 'taurus-h1-p-0'"));
        // De sessie is de eenheid van heraanhaken, net als bij tmux.
        assert!(s.contains("--session 'taurus-h1-p-0'"));
        // De pane-shell moet de taak met spatie als EEN argument zien; die regel
        // gaat op zijn beurt als EEN argument naar herdr, dus twee lagen quoting.
        assert!(
            s.contains(r#"run pane run w1:p1 ''\''claude'\'' '\''-n'\'' '\''my session'\''"#),
            "taak met spatie valt uiteen: {}",
            s
        );
        // macOS heeft geen setsid; dan moet nohup alleen het werk doen.
        assert!(s.contains("command -v setsid") && s.contains("else nohup"));
    }

    #[test]
    fn herdr_windows_detaches_the_server_from_the_ssh_session() {
        let s = herdr_windows_script("taurus-h1-p-0", r"C:\proj", "claude", &[]);
        // Gemeten op een echte host: met Start-Process is de server weg zodra de
        // ssh-verbinding sluit. Win32_Process.Create herparent hem erbuiten.
        assert!(s.contains("Win32_Process"), "server moet losgekoppeld starten: {}", s);
        assert!(!s.contains("Start-Process"));
        assert!(s.contains("--cwd 'C:\\proj'"));
        // Gemeten: `agent attach` bestaat nog niet op Windows, dus daar haakt de
        // sessie-TUI aan. Zou dit stilletjes de POSIX-vorm worden, dan opent de
        // tab met een foutmelding in plaats van met een agent.
        assert!(!s.contains("agent attach"), "attach werkt niet op Windows: {}", s);
        assert!(s.trim_end().ends_with("& $h --session $s"));
        // De pane draait PowerShell: zonder de call-operator is een gequote pad
        // geen commando maar een string die alleen wordt afgedrukt.
        assert!(s.contains("pane run w1:p1 '& ''claude'''"), "geen call-operator: {}", s);
    }

    #[test]
    fn herdr_works_on_both_platforms_unlike_tmux() {
        // Dit is de hele reden voor #115: Windows kon niet heraanhaken.
        let w = build_remote_payload_via("herdr", "windows", "", "s", r"C:\p", "claude", &[]).unwrap();
        assert!(w.starts_with("powershell -NoProfile -EncodedCommand "));
        let enc = w.rsplit(' ').next().unwrap();
        assert_eq!(enc, &b64(&utf16le(&herdr_windows_script("s", r"C:\p", "claude", &[]))));
        // POSIX gaat door dezelfde login-shell-starter als tmux.
        let l = build_remote_payload_via("herdr", "linux", "", "s", "/p", "claude", &[]).unwrap();
        assert!(l.starts_with("echo ") && l.contains("exec sh -l /tmp/s.sh"));
        // En via WSL op een Windows-host is het gewoon de POSIX-vorm.
        let v = build_remote_payload_via("herdr", "windows", "wsl", "s", "/p", "claude", &[]).unwrap();
        assert!(v.starts_with("wsl -e sh -c "));
    }

    #[test]
    fn attaching_creates_nothing() {
        // Het verschil met de launch-payload: aanhaken mag geen workspace maken
        // en geen agent starten. Een `pane run` op een sessie die middenin een
        // beurt zit, zou die beurt overschrijven.
        let mut h = test_host();
        h.mux = "herdr".into();
        h.os = "linux".into();
        let s = build_attach_payload(&h, "taurus-h1-p-0").unwrap();
        let inner = String::from_utf8(
            b64_decode_for_test(s.split_whitespace().nth(1).unwrap()),
        )
        .unwrap();
        assert!(!inner.contains("workspace create"), "attach maakt niets aan: {}", inner);
        assert!(!inner.contains("pane run"), "attach start niets: {}", inner);
        assert!(inner.contains("agent attach w1:p1"));
        // Geen agent (afgesloten of onbekend programma): de sessie-TUI werkt nog.
        assert!(inner.trim_end().ends_with("exec \"$H\" --session 'taurus-h1-p-0'"));

        // tmux haakt aan, en maakt niet stiekem een tweede lege sessie.
        h.mux = "tmux".into();
        let t = build_attach_payload(&h, "werk").unwrap();
        let ti = String::from_utf8(b64_decode_for_test(t.split_whitespace().nth(1).unwrap())).unwrap();
        assert_eq!(ti, "exec tmux attach-session -t 'werk'");
        assert!(!ti.contains("new-session"));

        // Een host zonder multiplexer heeft niets om aan te haken.
        h.mux = "none".into();
        assert!(build_attach_payload(&h, "werk").is_err());
    }

    #[test]
    fn a_session_name_from_the_host_cannot_carry_shell_syntax() {
        let mut h = test_host();
        h.mux = "herdr".into();
        for bad in ["a b", "a;rm -rf /", "$(id)", "`id`", "a|b", "../etc", ""] {
            assert!(
                build_attach_payload(&h, bad).is_err(),
                "had geweigerd moeten worden: {:?}",
                bad
            );
        }
        assert!(build_attach_payload(&h, "taurus-ursu-c-users-arjen-28f85e64").is_ok());
    }

    #[test]
    fn herdr_session_lines_become_rows_with_agent_and_cwd() {
        let out = "SESS|taurus-a|running\n\
                   JSON|taurus-a|{\"result\":{\"panes\":[{\"agent\":\"claude\",\"agent_status\":\"blocked\",\"cwd\":\"/srv/p\",\"pane_id\":\"w1:p1\"}]}}\n\
                   SESS|default|stopped\n";
        let list = parse_herdr_sessions(out);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "taurus-a");
        assert_eq!(list[0].agent, "claude");
        assert_eq!(list[0].agent_status, "blocked");
        assert_eq!(list[0].cwd, "/srv/p");
        // Een gestopte sessie heeft geen pane-JSON en hoort er toch te staan.
        assert_eq!(list[1].name, "default");
        assert_eq!(list[1].status, "stopped");
        assert!(list[1].agent.is_empty());
    }

    #[test]
    fn a_pane_without_an_agent_is_still_a_row() {
        // herdr zet "unknown" zolang hij niets gezien heeft; dat is geen status
        // om in de UI te zetten, maar de sessie is wel bruikbaar.
        let out = "SESS|kaal|running\n\
                   JSON|kaal|{\"result\":{\"panes\":[{\"agent_status\":\"unknown\",\"cwd\":\"/home/a\"}]}}\n";
        let list = parse_herdr_sessions(out);
        assert_eq!(list.len(), 1);
        assert!(list[0].agent.is_empty() && list[0].agent_status.is_empty());
        assert_eq!(list[0].cwd, "/home/a");
    }

    #[test]
    fn tmux_lines_become_rows() {
        let list = parse_tmux_sessions("werk|/home/a/p|1\nlos|/tmp|0\n\n");
        assert_eq!(list.len(), 2);
        assert_eq!((list[0].name.as_str(), list[0].status.as_str()), ("werk", "attached"));
        assert_eq!((list[1].name.as_str(), list[1].cwd.as_str()), ("los", "/tmp"));
    }

    #[test]
    fn an_older_hosts_json_keeps_following_the_probe() {
        // Zonder mux_auto is een host van vóór deze versie: die volgde altijd de
        // probe, en moet dat blijven doen. Stond het veld op false, dan zou een
        // hertest zijn mux nooit meer bijwerken -- precies verkeerd om.
        let old = r#"[{"id":"h","nickname":"h","hostname":"x","mux":"tmux"}]"#;
        let list: Vec<Host> = serde_json::from_str(old).expect("oude vorm moet leesbaar blijven");
        assert!(list[0].mux_auto, "ontbrekend veld = automatisch");
        let pinned = r#"[{"id":"h","nickname":"h","hostname":"x","mux":"none","mux_auto":false}]"#;
        let list: Vec<Host> = serde_json::from_str(pinned).unwrap();
        assert!(!list[0].mux_auto);
    }

    // GEMETEN op een echte opstelling: dezelfde computer stond er drie keer in
    // (sshd, WSL, Taurus-host), waardoor de bijnaam het onderscheid moest dragen
    // en "(Taurus-host)" doorlekte naar tabbadges en agentkaarten (#124).
    #[test]
    fn routes_to_one_address_collapse_into_one_machine() {
        let mk = |id: &str, nick: &str, port: u16| {
            let mut h = test_host();
            h.id = id.into();
            h.nickname = nick.into();
            h.hostname = "192.168.2.9".into();
            h.port = port;
            h
        };
        let m = group_machines(vec![
            mk("ursu", "ursu", 22),
            mk("ursu-wsl", "ursu-wsl", 2223),
            mk("ursu-taurus", "ursu (Taurus-host)", 8287),
        ]);
        assert_eq!(m.len(), 1, "een adres is een machine");
        // De kortste bijnaam wint: juist de toevoeging wilden we kwijt.
        assert_eq!(m[0].label, "ursu");
        assert_eq!(m[0].routes.len(), 3);
        // De Taurus-route heeft voorkeur: geen sleutelruil, geen sshd nodig.
        assert_eq!(m[0].preferred, "ursu-taurus");
    }

    // Zonder Taurus-route valt de voorkeur terug op wat er wel is.
    #[test]
    fn without_a_taurus_route_the_first_one_is_used() {
        let mut a = test_host();
        a.id = "ursu".into();
        a.hostname = "192.168.2.9".into();
        a.port = 22;
        let m = group_machines(vec![a]);
        assert_eq!(m[0].preferred, "ursu");
    }

    // Twee echt verschillende machines blijven twee regels; en een expliciete
    // `machine` wint van het adres (zelfde computer achter twee adressen).
    #[test]
    fn different_machines_stay_apart_and_machine_field_wins() {
        let mk = |id: &str, host: &str, machine: &str| {
            let mut h = test_host();
            h.id = id.into();
            h.hostname = host.into();
            h.machine = machine.into();
            h
        };
        assert_eq!(group_machines(vec![mk("a", "10.0.0.1", ""), mk("b", "10.0.0.2", "")]).len(), 2);
        assert_eq!(group_machines(vec![mk("a", "10.0.0.1", "ursu"), mk("b", "10.0.0.2", "ursu")]).len(), 1);
        // Hoofdlettergebruik mag geen tweede machine opleveren.
        assert_eq!(group_machines(vec![mk("a", "URSU", ""), mk("b", "ursu", "")]).len(), 1);
    }

    // Wat de firewallcheck op DEZE machine antwoordt. Genegeerd in de gewone run,
    // want de uitkomst hangt af van hoe deze machine is ingericht -- maar het is
    // wel de enige manier om te zien of hij de waarheid vertelt, en een eerdere
    // versie deed dat aantoonbaar niet:
    //
    //   cargo test --lib -- --ignored --nocapture toon_firewall
    #[test]
    #[ignore]
    fn toon_firewall() {
        // Standaard de testbinary; met TAURUS_TEST_EXE kijk je naar de echte. De
        // check is namelijk PER EXE, en dat is precies het punt: een block-regel op
        // taurus.exe zegt niets over dit testproces, en andersom ook niet.
        let exe = std::env::var("TAURUS_TEST_EXE").unwrap_or_else(|_| this_exe());
        let out = ps_encoded(&firewall_script(&exe))
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default();
        println!("exe: {exe}\n{}", out.trim());
    }

    // Tegen een ECHTE machine uit je eigen hosts.json. Genegeerd in de gewone run
    // (hij doet twee ssh-rondes en hangt dus aan een netwerk en een sleutel), maar
    // dit is de enige manier om te zien of de twee bronnen samen kloppen zonder de
    // GUI te starten:
    //
    //   cargo test --lib -- --ignored --nocapture toon_agents_op
    //
    // Zet TAURUS_TEST_HOST op het host-id; zonder die variabele doet hij niets.
    #[test]
    #[ignore]
    fn toon_agents_op() {
        let Ok(id) = std::env::var("TAURUS_TEST_HOST") else {
            println!("zet TAURUS_TEST_HOST=<host-id> om dit te draaien");
            return;
        };
        match remote_agents(id.clone()) {
            Err(e) => println!("{id}: FOUT {e}"),
            Ok(v) => {
                println!("{id}: {} agent(s), {} lege sessie(s), taurus gezien: {}",
                    v.agents.len(), v.empty.len(), v.taurus_seen);
                for a in &v.agents {
                    println!("  [{}] {} | {} | {} | aanhaakbaar={}",
                        a.origin, a.title, a.agent, a.cwd, a.attachable);
                }
                for e in &v.empty {
                    println!("  (leeg) {e}");
                }
            }
        }
    }

    // De hele reden dat dit bestand bestaat: een mislukte herstart mag een sessie
    // niet uit de geschiedenis halen. GEMETEN: na een herstart kwamen 2 van de 4
    // sessies terug en waren de andere twee helemaal weg uit Taurus.
    #[test]
    fn history_updates_and_never_loses_the_first_start() {
        let e = |uuid: &str, title: &str| HistoryEntry {
            uuid: uuid.into(),
            title: title.into(),
            ..Default::default()
        };
        let list = merge_history(Vec::new(), e("u1", "Ontwikkel"), 100);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].created, 100);

        // Zelfde sessie later opnieuw gezien: bijgewerkt, niet verdubbeld, en
        // "wanneer begon dit werk" blijft staan.
        let list = merge_history(list, e("u1", "Ontwikkel hernoemd"), 500);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].created, 100, "created is van de eerste keer");
        assert_eq!(list[0].last_seen, 500);
        assert_eq!(list[0].title, "Ontwikkel hernoemd");

        // Een tweede sessie komt erbij, nieuwste bovenaan.
        let list = merge_history(list, e("u2", "random"), 900);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].uuid, "u2");
    }

    // Een lege uuid is geen sessie om te onthouden; die zou bij elke start een
    // nieuwe regel opleveren.
    #[test]
    fn history_ignores_an_entry_without_a_uuid() {
        let list = merge_history(Vec::new(), HistoryEntry::default(), 1);
        assert!(list.is_empty());
    }

    // GEMETEN op ursu, en precies de situatie die de klacht opleverde: herdr kende
    // drie sessies waarvan NUL een agent had, terwijl er twee agents draaiden die
    // alleen de Taurus daar kende. Alle drie de herdr-regels werden als keuze
    // aangeboden en zetten je in een cmd-prompt.
    #[test]
    fn only_agents_are_choices_and_the_rest_is_cleanup() {
        let sess = |name: &str, agent: &str, cwd: &str| RemoteSession {
            name: name.into(),
            status: "running".into(),
            agent: agent.into(),
            agent_status: String::new(),
            cwd: cwd.into(),
        };
        let peer = r#"[
          {"title":"Ontwikkel","path":"C:\\Users\\arjen\\ontwikkelmap","agent":"claude","host_id":""},
          {"title":"random","path":"C:\\Users\\arjen","agent":"claude","host_id":""},
          {"title":"elders","path":"/srv/x","agent":"claude","host_id":"andere-machine"}
        ]"#;
        let got = collect_agents(
            vec![
                sess("default", "", ""),
                sess("taurus-ursu-c-users-arjen-28f85e64", "", r"C:\Users\arjen"),
                sess("taurus-ursu-taurus-c-users-arjen-28f85e64", "", ""),
            ],
            Some(peer),
        );
        // Geen van de drie herdr-sessies is een keuze; ze zijn opruimwerk.
        assert_eq!(got.empty.len(), 3);
        // Wel de twee agents die er echt draaien.
        let namen: Vec<&str> = got.agents.iter().map(|a| a.title.as_str()).collect();
        assert_eq!(namen, vec!["Ontwikkel", "random"]);
        // Een sessie die op DIE machine naar een derde machine wijst is niet van haar.
        assert!(!got.agents.iter().any(|a| a.title == "elders"));
        // Zonder mux-sessie is er niets om aan te haken, en dat zegt het model ook.
        assert!(got.agents.iter().all(|a| !a.attachable && a.origin == "taurus"));
        assert!(got.taurus_seen);
    }

    // Een herdr-sessie MET agent is wel gewoon een keuze, en de mapnaam leest
    // prettiger dan een sessienaam met een hash erachter.
    #[test]
    fn a_herdr_session_with_an_agent_is_a_real_choice() {
        let got = collect_agents(
            vec![RemoteSession {
                name: "taurus-ursu-c-users-arjen-proj-1a2b".into(),
                status: "running".into(),
                agent: "claude".into(),
                agent_status: "werkt".into(),
                cwd: r"C:\Users\arjen\proj".into(),
                }],
            None,
        );
        assert_eq!(got.agents.len(), 1);
        assert_eq!(got.agents[0].title, "proj");
        assert!(got.agents[0].attachable);
        assert_eq!(got.agents[0].origin, "herdr");
        assert_eq!(got.agents[0].status, "werkt");
        // Geen antwoord van de Taurus daar is iets anders dan "hij draait niets".
        assert!(!got.taurus_seen);
    }

    // Dezelfde agent langs twee kanten gezien blijft één regel: de Taurus daar kent
    // hem ook, maar het is dezelfde map en dus hetzelfde werk.
    #[test]
    fn the_same_agent_seen_twice_stays_one_row() {
        let got = collect_agents(
            vec![RemoteSession {
                name: "taurus-ursu-x".into(),
                status: "running".into(),
                agent: "claude".into(),
                agent_status: String::new(),
                cwd: r"C:\Users\arjen\proj".into(),
            }],
            Some(r#"[{"title":"proj","path":"c:/users/arjen/proj/","agent":"claude","host_id":""}]"#),
        );
        assert_eq!(got.agents.len(), 1, "{:?}", got.agents);
        assert!(got.agents[0].attachable, "de aanhaakbare kant wint");
    }

    // GEMETEN nadat Connect een paar keer gebruikt was: op de andere machine
    // stonden drie herdr-sessies zonder agent, twee daarvan door Connect gemaakt.
    // "Wegwerp" moet aan beide kanten gelden, anders stapelen lege sessies zich op
    // die bij het aanhaken in een kale shell of een mislukte resume eindigen.
    #[test]
    fn a_throwaway_session_leaves_no_persistent_session_behind() {
        let mut h = test_host();
        h.mux = "herdr".into();
        h.mux_auto = true;
        let weg = without_persistence(h.clone(), true);
        assert_eq!(weg.mux, "none");
        // Anders vult de eerste hertest hem alsnog in.
        assert!(!weg.mux_auto);
        // Een gewone start houdt de persistentie die de machine biedt.
        assert_eq!(without_persistence(h, false).mux, "herdr");
    }

    // De sessienaam gaat een shell in, dus hij hoort gequote te zijn -- en het moet
    // `session delete` zijn, niet `server stop`.
    //
    // GEMETEN tegen herdr op ursu: `--session <naam> server stop` gaf exitcode 0,
    // liet de sessie gewoon in de lijst staan en startte en passant de
    // default-sessie. `session delete <naam>` antwoordt "deleted session ..." en
    // haalt hem er echt uit.
    #[test]
    fn stopping_a_session_deletes_it_by_name_and_quotes_it() {
        let mut h = test_host();
        h.mux = "herdr".into();
        h.os = "linux".into();
        let cmd = stop_session_command(&h, "werk 2").unwrap();
        // De payload gaat als base64 door de shell; decodeer hem om te kijken.
        let b64part = cmd.split_whitespace().nth(1).unwrap();
        let raw = String::from_utf8(b64_decode_for_test(b64part)).unwrap();
        assert!(raw.contains("session stop 'werk 2'"), "{raw}");
        assert!(raw.contains("session delete 'werk 2'"), "{raw}");
        // Juist NIET de vorm die niets deed.
        assert!(!raw.contains("server stop"), "{raw}");
    }

    // Op een Windows-host gaat het door PowerShell, met dezelfde herdr-terugval
    // als de rest: een pad in LOCALAPPDATA als `herdr` niet in PATH staat.
    #[test]
    fn stopping_a_session_on_windows_goes_through_powershell() {
        let mut h = test_host();
        h.mux = "herdr".into();
        h.os = "windows".into();
        let cmd = stop_session_command(&h, "werk").unwrap();
        assert!(cmd.starts_with("powershell -NoProfile -EncodedCommand "), "{cmd}");
    }

    // tmux heeft zijn eigen werkwoord; psmux spreekt dezelfde taal.
    #[test]
    fn stopping_a_tmux_session_uses_kill_session() {
        let mut h = test_host();
        h.mux = "tmux".into();
        h.os = "linux".into();
        let cmd = stop_session_command(&h, "werk").unwrap();
        let b64part = cmd.split_whitespace().nth(1).unwrap();
        let raw = String::from_utf8(b64_decode_for_test(b64part)).unwrap();
        assert!(raw.contains("tmux kill-session -t 'werk'"), "{raw}");
    }

    // Zonder persistentie is er geen sessie die blijft staan, dus ook niets om te
    // stoppen. Een commando verzinnen zou hier een fout uit een shell opleveren.
    #[test]
    fn stopping_a_session_needs_a_mux() {
        let mut h = test_host();
        h.mux = "none".into();
        assert!(stop_session_command(&h, "werk").is_err());
    }

    // Zonder herdr is er geen chrome om te verbergen, en dan hoort Taurus van
    // andermans configbestand af te blijven.
    #[test]
    fn tuning_is_only_for_herdr_hosts() {
        let mut h = test_host();
        h.mux = "tmux".into();
        assert_eq!(tune_herdr(h.clone()).unwrap(), "skip");
        h.mux = "none".into();
        assert_eq!(tune_herdr(h).unwrap(), "skip");
    }

    // Handmatig: `cargo test -- --ignored --nocapture toon_herdr` drukt het
    // commando af dat naar een POSIX-host gaat, zodat je het tegen een echte
    // machine kunt houden zonder de app te starten.
    #[test]
    #[ignore]
    fn toon_herdr_tune_commando() {
        println!("{}", herdr_tune_posix_command());
    }

    // GEMETEN op ursu-wsl: een POSIX-host valt WEL terug op de sessie-TUI zodra
    // er geen agent in de pane draait, en dan staat daar dezelfde dubbele
    // sidebar als op Windows. Deze test legt vast dat de ingreep daar nu ook
    // gebeurt -- en dat hij net zo voorzichtig is als de Windows-variant.
    #[test]
    fn posix_tuning_is_reversible_and_validated() {
        let cmd = herdr_tune_posix_command();
        // Alles na de laatste "echo " tot de pipe is de base64-payload.
        let payload = cmd
            .rsplit("echo ")
            .next()
            .and_then(|rest| rest.split(' ').next())
            .expect("base64-deel");
        let script = String::from_utf8(b64_decode_for_test(payload)).expect("script is tekst");

        assert!(script.contains("sidebar_start_collapsed = true"), "{script}");
        assert!(script.contains("sidebar_collapsed_mode = \"hidden\""), "{script}");
        assert!(script.contains("hide_tab_bar_when_single_tab = true"), "{script}");
        // Eigen keuze van de gebruiker wint: al ingesteld = niet aankomen.
        assert!(script.contains("if grep -q sidebar_start_collapsed"), "{script}");
        // Back-up voor, terugzetten als herdr de config afkeurt.
        assert!(script.contains("taurus.bak"), "{script}");
        assert!(script.contains("config check"), "{script}");
        assert!(script.contains("TUNE=fail"), "{script}");
        // Een bestaande [ui]-tabel krijgt de sleutels erbij; een tweede [ui]
        // zou ongeldige TOML zijn.
        assert!(script.contains("grep -q \"^\\[ui\\]\""), "{script}");
    }

    #[test]
    fn best_mux_prefers_herdr_and_falls_back_to_none() {
        let v = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(best_mux(&v(&["tmux", "herdr"])), "herdr");
        assert_eq!(best_mux(&v(&["tmux", "psmux"])), "tmux");
        assert_eq!(best_mux(&v(&["psmux"])), "psmux");
        assert_eq!(best_mux(&[]), "none");
    }

    // Kleine base64-decoder, alleen voor de tests: de payloads reizen als base64
    // en een assert op de bytes leest niet.
    fn b64_decode_for_test(s: &str) -> Vec<u8> {
        const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut acc: u32 = 0;
        let mut bits = 0;
        let mut out = Vec::new();
        for c in s.bytes().filter(|c| *c != b'=') {
            let v = T.iter().position(|t| *t == c).expect("geen base64") as u32;
            acc = (acc << 6) | v;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push((acc >> bits) as u8);
            }
        }
        out
    }

    #[test]
    fn build_remote_payload_uses_base64_for_the_taurus_agent() {
        let args = vec!["-n".to_string(), r#"a "quoted" & risky task"#.to_string()];
        let s = build_remote_payload_via("taurus-agent", "windows", "", "sess1", r"C:\p", "claude", &args)
            .unwrap();
        let expected_b64 = b64(
            serde_json::json!({ "cwd": r"C:\p", "program": "claude", "args": args })
                .to_string()
                .as_bytes(),
        );
        assert_eq!(
            s,
            format!(".taurus\\bin\\taurus-agent.exe run -s sess1 --b64 {}", expected_b64)
        );
        // Het hele commando moet shell-neutraal zijn: geen teken waar cmd.exe of
        // /bin/sh iets mee doet. Dat is de reden dat de payload base64 is.
        assert!(
            !s.contains('"') && !s.contains('&') && !s.contains('<') && !s.contains('>'),
            "payload mag geen shell-metatekens bevatten: {}",
            s
        );
    }

    #[test]
    fn build_remote_payload_refuses_combinations_that_cannot_work() {
        // tmux bestaat niet op Windows.
        assert!(build_remote_payload_via("tmux", "windows", "", "s", "C:\\p", "claude", &[]).is_err());
        assert!(build_remote_payload_via("zellij", "linux", "", "s", "/p", "claude", &[]).is_err());
        // Zonder multiplexer op Linux: kaal, met exec.
        let s = build_remote_payload_inner("none", "linux", "s", "/p", "claude", &[]).unwrap();
        assert_eq!(s, "cd '/p' && exec 'claude'");
    }

    #[test]
    fn build_remote_payload_encodes_a_bare_windows_launch_for_powershell() {
        // De remote shell is cmd.exe. Een taak met aanhalingstekens, een & en
        // een apostrof is precies wat een naief samengesteld commando sloopt.
        let args = vec![
            "-n".to_string(),
            r#"fix "the" thing & report it; it's urgent"#.to_string(),
        ];
        let s = build_remote_payload_via("none", "windows", "", "s", r"C:\proj", "claude", &args).unwrap();
        assert!(s.starts_with("powershell -NoProfile -EncodedCommand "));
        let enc = s.rsplit(' ').next().unwrap();
        // Alleen base64-tekens: cmd.exe ziet geen enkel metateken.
        assert!(
            enc.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='),
            "niet-base64 teken in payload: {}",
            enc
        );
        // En het decodeert naar het bedoelde PowerShell-script.
        let expected = b64(&utf16le(
            "Set-Location 'C:\\proj'; & 'claude' '-n' 'fix \"the\" thing & report it; it''s urgent'",
        ));
        assert_eq!(enc, expected);
    }

    #[test]
    fn ps_quote_doubles_the_single_quote_and_blocks_expansion() {
        assert_eq!(ps_quote("plain"), "'plain'");
        assert_eq!(ps_quote("it's"), "'it''s'");
        // Binnen '...' expandeert PowerShell niets.
        assert_eq!(ps_quote("$env:PATH `id`"), "'$env:PATH `id`'");
        assert_eq!(ps_quote(""), "''");
    }

    #[test]
    fn utf16le_is_what_encodedcommand_expects() {
        // "Hi" -> 48 00 69 00
        assert_eq!(utf16le("Hi"), vec![0x48, 0x00, 0x69, 0x00]);
        assert_eq!(utf16le(""), Vec::<u8>::new());
    }

    #[test]
    fn wrap_remote_builds_the_ssh_argument_list() {
        let (prog, args) = wrap_remote(&test_host(), "/home/a/p", "claude".into(), vec![]).unwrap();
        assert!(prog.to_lowercase().ends_with("ssh.exe"), "onverwacht programma: {}", prog);
        assert_eq!(args[0], "-t", "zonder pty tekent de agent-TUI niet");
        // Standaardpoort hoort niet als -p mee.
        assert!(!args.contains(&"-p".to_string()));
        // Geen key_path -> geen -i, ssh mag zelf kiezen.
        assert!(!args.contains(&"-i".to_string()));
        // Een GEWIJZIGDE hostkey moet geweigerd worden, niet stil geaccepteerd.
        assert!(args.contains(&"StrictHostKeyChecking=accept-new".to_string()));
        assert!(!args.iter().any(|a| a.contains("StrictHostKeyChecking=no")));
        // Doel en payload staan achteraan, in die volgorde.
        assert_eq!(args[args.len() - 2], "arjen@support01");
        // De payload reist als base64 en start in een login shell (zie
        // a_posix_launch_uses_a_login_shell voor het waarom).
        assert!(args[args.len() - 1].starts_with("echo "));
        assert!(args[args.len() - 1].contains("exec sh -l /tmp/"));
    }

    #[test]
    fn wrap_remote_adds_the_port_only_when_it_is_not_the_default() {
        let mut h = test_host();
        h.port = 2222;
        let (_, args) = wrap_remote(&h, "/p", "claude".into(), vec![]).unwrap();
        let i = args.iter().position(|a| a == "-p").expect("-p verwacht");
        assert_eq!(args[i + 1], "2222");
    }

    #[test]
    fn wrap_remote_omits_the_user_when_the_host_has_none() {
        let mut h = test_host();
        h.user = String::new();
        let (_, args) = wrap_remote(&h, "/p", "claude".into(), vec![]).unwrap();
        assert_eq!(args[args.len() - 2], "support01");
    }

    #[test]
    fn wrap_remote_refuses_a_host_without_a_hostname() {
        let mut h = test_host();
        h.hostname = "  ".into();
        assert!(wrap_remote(&h, "/p", "claude".into(), vec![]).is_err());
    }

    #[test]
    fn resolve_key_falls_back_to_ssh_config_and_rejects_a_missing_file() {
        let mut h = test_host();
        assert_eq!(resolve_key(&h).unwrap(), KeySource::SshConfig);
        h.key_path = "   ".into();
        assert_eq!(resolve_key(&h).unwrap(), KeySource::SshConfig);
        // Een pad dat niet bestaat moet nu al falen, niet pas als ssh start --
        // anders is de foutmelding een cryptische ssh-exit in de terminal.
        h.key_path = r"C:\taurus-test\does-not-exist\id_ed25519".into();
        assert!(resolve_key(&h).is_err());

        // En een key die er wel is, komt als pad terug.
        let dir = std::env::temp_dir().join(format!("taurus-test-key-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let key = dir.join("id_ed25519");
        std::fs::write(&key, b"not a real key").unwrap();
        h.key_path = key.to_string_lossy().into_owned();
        assert_eq!(resolve_key(&h).unwrap(), KeySource::Path(key.to_string_lossy().into_owned()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn expand_home_only_touches_a_leading_tilde() {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_default();
        if !home.is_empty() {
            assert_eq!(expand_home("~/.ssh/id_ed25519"), format!("{}/.ssh/id_ed25519", home));
            assert_eq!(expand_home(r"~\.ssh\id_ed25519"), format!(r"{}\.ssh\id_ed25519", home));
        }
        // Geen tilde, of een tilde midden in het pad: onaangeroerd.
        assert_eq!(expand_home("/abs/path"), "/abs/path");
        assert_eq!(expand_home(r"C:\keys\a~b"), r"C:\keys\a~b");
    }

    #[test]
    fn apply_host_leaves_a_local_launch_untouched() {
        let (p, a, cwd) =
            apply_host(None, r"C:\proj", "claude".into(), vec!["-n".into()]).unwrap();
        assert_eq!(p, "claude");
        assert_eq!(a, vec!["-n".to_string()]);
        assert_eq!(cwd, r"C:\proj", "lokale werkmap mag niet verschuiven");
    }

    #[test]
    fn lookup_host_reports_an_unknown_id_instead_of_falling_back_to_local() {
        assert!(lookup_host("").unwrap().is_none(), "leeg id = lokaal");
        // Stil terugvallen op lokaal zou een remote tab ongemerkt op het
        // werkstation starten -- met de agent-rechten van dit werkstation.
        let e = lookup_host("nope-does-not-exist").unwrap_err();
        assert!(e.contains("onbekende host"), "onverwachte fout: {}", e);
    }

    #[test]
    fn remote_launches_never_carry_a_local_path() {
        // Gevonden tegen een echte host: build_command resolveert het programma
        // via het PATH van DIT werkstation, dus er ging
        // C:\Users\<mij>\.local\bin\claude.exe naar een machine met een andere
        // gebruiker en een andere installatie. Remote hoort de kale naam te zijn.
        for (os, expect) in [("windows", "claude.exe"), ("linux", "claude")] {
            let (program, _) = build_command(
                "claude",
                LaunchKind::Create,
                "sid",
                "t",
                "",
                "default",
                "",
                false,
                Some(os),
            );
            assert_eq!(program, expect);
            assert!(!program.contains('\\') && !program.contains('/'), "geen pad: {}", program);
        }
        assert_eq!(remote_agent_program("agy", "linux"), "agy");
        assert_eq!(remote_agent_program("agy", "windows"), "agy.exe");
    }

    #[test]
    fn probe_tcp_sees_an_open_port_and_misses_a_closed_one() {
        use std::net::TcpListener;
        // Poort 0 = het OS kiest een vrije poort; geen vaste poort die op een
        // andere machine (of een parallelle testrun) al bezet kan zijn.
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        assert!(probe_tcp("127.0.0.1", port, 1000), "open poort moet bereikbaar zijn");
        drop(listener);
        // Dicht: de kernel weigert meteen, dus een korte timeout volstaat.
        assert!(!probe_tcp("127.0.0.1", port, 300), "gesloten poort mag niet bereikbaar heten");
    }

    #[test]
    fn probe_tcp_treats_an_unresolvable_name_as_unreachable() {
        // .invalid is gereserveerd (RFC 2606) en resolvet nooit.
        assert!(!probe_tcp("taurus-nonexistent.invalid", 22, 300));
    }

    #[test]
    fn probe_tcp_finds_the_working_address_of_a_multi_address_name() {
        use std::net::TcpListener;
        // "localhost" levert doorgaans zowel ::1 als 127.0.0.1 op. We luisteren
        // op precies EEN daarvan, dus dit slaagt alleen als alle adressen aan
        // bod komen. Serieel proberen met een gedeelde deadline liet een naam
        // met meerdere adressen ten onrechte als onbereikbaar gelden.
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        assert!(
            probe_tcp("localhost", port, 2000),
            "moet het werkende adres vinden, ook als een ander adres eerst komt"
        );
    }

    // Print de ssh-aanroep die Taurus voor een echte host zou doen, zodat je hem
    // los kunt uitvoeren en de keten kunt narekenen zonder de app te starten:
    //   set TAURUS_TEST_HOST=192.168.2.9
    //   set TAURUS_TEST_USER=arjen
    //   set TAURUS_TEST_CWD=C:\pad\op\de\host
    //   cargo test print_the_real_ssh_invocation -- --ignored --nocapture
    #[test]
    #[ignore]
    fn print_the_real_ssh_invocation() {
        let host = Host {
            id: "ursu".into(),
            nickname: "ursu".into(),
            hostname: std::env::var("TAURUS_TEST_HOST").expect("TAURUS_TEST_HOST"),
            machine: String::new(),
            user: std::env::var("TAURUS_TEST_USER").unwrap_or_default(),
            port: 22,
            key_path: std::env::var("TAURUS_TEST_KEY").unwrap_or_default(),
            default_project: String::new(),
            os: std::env::var("TAURUS_TEST_OS").unwrap_or_else(|_| "windows".into()),
            mux: std::env::var("TAURUS_TEST_MUX").unwrap_or_else(|_| "none".into()),
            mux_auto: true,
            agent_version: String::new(),
            via: String::new(),
        };
        // Met TAURUS_TEST_ATTACH=<sessienaam> print hij de AANHAAK-aanroep in
        // plaats van de start-aanroep, zodat je die met de hand tegen een echte
        // host kunt draaien.
        if let Ok(session) = std::env::var("TAURUS_TEST_ATTACH") {
            let payload = build_attach_payload(&host, &session).expect("attach payload");
            let (prog, a) = ssh_interactive(&host, payload).expect("ssh_interactive");
            println!("PROGRAM: {}", prog);
            for (i, x) in a.iter().enumerate() {
                println!("ARG[{}]: {}", i, x);
            }
            return;
        }
        let cwd = std::env::var("TAURUS_TEST_CWD").unwrap_or_else(|_| r"C:\Users\arjen".into());
        // Exact het pad dat create_session loopt: agent-vlaggen eerst, dan de
        // remote wikkel eromheen.
        let (program, args) = build_command(
            "claude",
            LaunchKind::Create,
            "11111111-2222-3333-4444-555555555555",
            "remote test",
            "",
            "default",
            "",
            false,
            Some(host.os.as_str()),
        );
        let (prog, a) = wrap_remote(&host, &cwd, program, args).expect("wrap_remote");
        println!("PROGRAM: {}", prog);
        for (i, x) in a.iter().enumerate() {
            println!("ARG[{}]: {}", i, x);
        }
    }

    // Zet de herdr-chrome uit op een ECHTE Windows-host. Draaien met:
    //   TAURUS_TEST_HOST=... TAURUS_TEST_USER=... TAURUS_TEST_KEY=...
    //   cargo test tune_a_real_host -- --ignored --nocapture
    #[test]
    #[ignore]
    fn tune_a_real_host() {
        let mut host = test_host();
        host.hostname = std::env::var("TAURUS_TEST_HOST").expect("TAURUS_TEST_HOST");
        host.user = std::env::var("TAURUS_TEST_USER").unwrap_or_default();
        host.key_path = std::env::var("TAURUS_TEST_KEY").unwrap_or_default();
        host.os = "windows".into();
        host.mux = "herdr".into();
        host.via = String::new();
        println!("TUNE: {:?}", tune_herdr(host));
    }

    // Meet tegen een echte host uit het eigen netwerk. Draaien met:
    //   set TAURUS_TEST_HOST=192.168.2.9 && cargo test -- --ignored
    // Geen adressen in de code: dit is een publieke repo.
    #[test]
    #[ignore]
    fn probe_tcp_reaches_a_real_host_from_the_environment() {
        let host = std::env::var("TAURUS_TEST_HOST")
            .expect("zet TAURUS_TEST_HOST=<hostname of ip>");
        let port: u16 = std::env::var("TAURUS_TEST_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(22);
        let t0 = std::time::Instant::now();
        let ok = probe_tcp(&host, port, REACH_TIMEOUT_MS);
        println!("{}:{} bereikbaar={} in {:?}", host, port, ok, t0.elapsed());
        assert!(ok, "{}:{} niet bereikbaar binnen {}ms", host, port, REACH_TIMEOUT_MS);
    }

    #[test]
    fn host_json_fills_in_the_default_port_and_optional_fields() {
        // Een met de hand geschreven hosts.json hoeft alleen het minimum te
        // bevatten; alles wat de probe invult heeft een default.
        let h: Host = serde_json::from_str(
            r#"{"id":"h1","nickname":"Home PC","hostname":"192.168.1.42"}"#,
        )
        .expect("minimale host moet parsen");
        assert_eq!(h.port, 22);
        assert_eq!(h.user, "");
        assert_eq!(h.key_path, "");
        assert_eq!(h.os, "");
        assert_eq!(h.mux, "");
        assert_eq!(h.agent_version, "");
    }

    #[test]
    fn host_round_trips_through_json() {
        // save_hosts/get_hosts gebruiken dezelfde struct, dus een opgeslagen
        // host moet ongewijzigd terugkomen -- inclusief een niet-default poort.
        let src = Host {
            id: "h2".into(),
            nickname: "Work".into(),
            hostname: "work.tail.net".into(),
            machine: String::new(),
            user: "arjen".into(),
            port: 2222,
            key_path: r"C:\Users\AST\.ssh\id_ed25519".into(),
            default_project: "/home/arjen/proj".into(),
            os: "linux".into(),
            mux: "tmux".into(),
            mux_auto: true,
            agent_version: String::new(),
            via: String::new(),
        };
        let txt = serde_json::to_string(&src).unwrap();
        let back: Host = serde_json::from_str(&txt).unwrap();
        assert_eq!(back.port, 2222);
        assert_eq!(back.hostname, "work.tail.net");
        assert_eq!(back.mux, "tmux");
        assert_eq!(back.key_path, r"C:\Users\AST\.ssh\id_ed25519");
    }

    // Een testexemplaar naast een draaiende Taurus mag NOOIT dezelfde configmap
    // pakken: dan hervat het je lopende sessies en overschrijft het ze daarna.
    #[test]
    fn config_dir_override_wins_over_appdata() {
        let d = resolve_config_dir(Some(r"C:\Temp\TaurusTest".into()), Some(r"C:\Users\x\AppData\Roaming".into()));
        assert_eq!(d, std::path::PathBuf::from(r"C:\Temp\TaurusTest"));
        // Geen "Taurus" eronder plakken: de opgegeven map IS de configmap.
        assert!(!d.ends_with("Taurus"));
    }

    #[test]
    fn config_dir_falls_back_to_appdata() {
        let d = resolve_config_dir(None, Some(r"C:\Users\x\AppData\Roaming".into()));
        assert_eq!(d, std::path::PathBuf::from(r"C:\Users\x\AppData\Roaming\Taurus"));
    }

    // Een lege of witruimte-variabele is per ongeluk gezet, niet bedoeld als
    // "gebruik de huidige map".
    #[test]
    fn empty_override_is_ignored() {
        let d = resolve_config_dir(Some("   ".into()), Some(r"C:\Users\x\AppData\Roaming".into()));
        assert_eq!(d, std::path::PathBuf::from(r"C:\Users\x\AppData\Roaming\Taurus"));
    }

    #[test]
    fn b64_matches_rfc4648_vectors() {
        assert_eq!(b64(b""), "");
        assert_eq!(b64(b"f"), "Zg==");
        assert_eq!(b64(b"fo"), "Zm8=");
        assert_eq!(b64(b"foo"), "Zm9v");
        assert_eq!(b64(b"foob"), "Zm9vYg==");
        assert_eq!(b64(b"fooba"), "Zm9vYmE=");
        assert_eq!(b64(b"foobar"), "Zm9vYmFy");
        assert_eq!(b64(&[0xff, 0x00, 0xee]), "/wDu");
    }

    #[test]
    fn split_command_plain_and_quoted() {
        assert_eq!(split_command("a b c"), vec!["a", "b", "c"]);
        assert_eq!(
            split_command(r#""C:\Program Files\tool.exe" --flag "an arg""#),
            vec![r"C:\Program Files\tool.exe", "--flag", "an arg"]
        );
        assert!(split_command("   ").is_empty());
        assert!(split_command(r#""""#).is_empty());
        assert!(parse_override(r#""""#).is_err());
        assert_eq!(
            parse_override("prog -x").unwrap(),
            ("prog".to_string(), vec!["-x".to_string()])
        );
    }

    #[test]
    fn norm_title_falls_back_and_trims() {
        assert_eq!(norm_title("  "), "agent");
        assert_eq!(norm_title(" ZGV-debug "), "ZGV-debug");
    }

    #[test]
    fn logo_mime_by_extension() {
        assert_eq!(logo_mime("a.SVG"), "image/svg+xml");
        assert_eq!(logo_mime("a.jpeg"), "image/jpeg");
        assert_eq!(logo_mime("a.webp"), "image/webp");
        assert_eq!(logo_mime("a.png"), "image/png");
        assert_eq!(logo_mime("noext"), "image/png");
    }

    #[test]
    fn claude_session_file_encodes_the_path() {
        let p = claude_session_file(r"C:\Users\AST\claude\Taurus", "uuid-1");
        let s = p.to_string_lossy().into_owned();
        assert!(s.contains("C--Users-AST-claude-Taurus"), "{}", s);
        assert!(s.ends_with("uuid-1.jsonl"), "{}", s);
    }

    #[test]
    fn build_command_claude_create_and_resume() {
        let (_, a) = build_command("claude", LaunchKind::Create, "u1", "t", "do it", "plan", "opus", true, None);
        assert_eq!(a[0..2], ["--session-id".to_string(), "u1".to_string()]);
        assert!(a.windows(2).any(|w| w == ["--permission-mode", "plan"]));
        assert!(a.windows(2).any(|w| w == ["--model", "opus"]));
        assert!(a.contains(&"--append-system-prompt".to_string()));
        assert_eq!(a.last().unwrap(), "do it");

        let (_, r) = build_command("claude", LaunchKind::Resume, "u1", "t", "do it", "default", "", false, None);
        assert_eq!(r[0..2], ["--resume".to_string(), "u1".to_string()]);
        // Bij resume geen taak meesturen en geen --permission-mode "default".
        assert!(!r.contains(&"do it".to_string()));
        assert!(!r.contains(&"--permission-mode".to_string()));
    }

    #[test]
    fn build_command_agy_resume_continues_without_task() {
        let (_, a) = build_command("agy", LaunchKind::Resume, "u1", "t", "task", "auto", "", true, None);
        assert!(a.contains(&"--continue".to_string()));
        assert!(a.contains(&"--dangerously-skip-permissions".to_string()));
        // agy kent geen prompt bij --continue en geen full-paths-equivalent.
        assert!(!a.contains(&"--prompt-interactive".to_string()));
        assert!(!a.contains(&"--append-system-prompt".to_string()));
    }

    #[test]
    fn resolve_in_paths_prefers_exe_and_wraps_shims() {
        let dir = std::env::temp_dir().join(format!("taurus-test-resolve-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let paths = dir.to_string_lossy().into_owned();

        // Alleen een .cmd-shim (npm-installatie): cmd.exe + /c-prefix (#40).
        std::fs::write(dir.join("fakeagent.cmd"), "@echo off").unwrap();
        let (prog, pre) = resolve_in_paths("fakeagent", &paths).unwrap();
        assert_eq!(prog, "cmd.exe");
        assert_eq!(pre[0], "/c");
        assert!(pre[1].ends_with("fakeagent.cmd"));

        // Komt er een echte exe bij, dan wint die (PATHEXT-volgorde).
        std::fs::write(dir.join("fakeagent.exe"), "MZ").unwrap();
        let (prog, pre) = resolve_in_paths("fakeagent", &paths).unwrap();
        assert!(prog.ends_with("fakeagent.exe"));
        assert!(pre.is_empty());

        // Niets gevonden -> None (resolve_program valt dan terug op de kale naam).
        assert!(resolve_in_paths("bestaatniet", &paths).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unique_path_appends_a_counter() {
        let dir = std::env::temp_dir().join(format!("taurus-test-unique-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("x.txt");
        assert_eq!(unique_path(f.clone()), f);
        std::fs::write(&f, "a").unwrap();
        assert_eq!(unique_path(f.clone()), dir.join("x (2).txt"));
        std::fs::write(dir.join("x (2).txt"), "b").unwrap();
        assert_eq!(unique_path(f.clone()), dir.join("x (3).txt"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dest_inside_src_detects_self_copy() {
        let root = std::env::temp_dir().join(format!("taurus-test-nest-{}", std::process::id()));
        let input = root.join("input");
        std::fs::create_dir_all(&input).unwrap();
        // De werkmap zelf droppen: dest = <root>\input\<naam> ligt binnen root.
        assert!(dest_inside_src(&root, &input.join("root")));
        // Een bestand van elders is prima.
        let elsewhere = std::env::temp_dir().join(format!("taurus-test-else-{}", std::process::id()));
        std::fs::create_dir_all(&elsewhere).unwrap();
        assert!(!dest_inside_src(&elsewhere, &input.join("file.txt")));
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&elsewhere);
    }

    #[test]
    fn verify_sha256_rejects_bad_pins_and_mismatches() {
        let f = std::env::temp_dir().join(format!("taurus-test-sha-{}.bin", std::process::id()));
        std::fs::write(&f, b"hello").unwrap();
        // "hello" -> bekende SHA-256.
        let good = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert!(verify_sha256(&f, good).is_ok());
        assert!(verify_sha256(&f, &good.to_uppercase()).is_ok());
        // Ongeldige pin (te kort / geen hex) wordt geweigerd.
        std::fs::write(&f, b"hello").unwrap();
        assert!(verify_sha256(&f, "abc").is_err());
        // Mismatch: fout EN het bestand is verwijderd.
        let wrong = "0000000000000000000000000000000000000000000000000000000000000000";
        assert!(verify_sha256(&f, wrong).is_err());
        assert!(!f.exists());
    }

    #[test]
    fn parse_model_list_keeps_cli_order_and_cleans_up() {
        // Echte `agy models`-uitvoer (ingekort), met CRLF zoals op Windows.
        let out = "Gemini 3.6 Flash (High)\r\nGemini 3.5 Flash (Medium)\r\nGPT-OSS 120B (Medium)\r\n";
        assert_eq!(
            parse_model_list(out),
            vec![
                "Gemini 3.6 Flash (High)",
                "Gemini 3.5 Flash (Medium)",
                "GPT-OSS 120B (Medium)"
            ]
        );
        // Lege regels en witruimte eromheen verdwijnen, volgorde blijft.
        assert_eq!(parse_model_list("\n  A  \n\n\tB\n"), vec!["A", "B"]);
        // Dubbelen vallen weg; de eerste (nieuwste) blijft staan.
        assert_eq!(parse_model_list("A\nB\nA\n"), vec!["A", "B"]);
        assert!(parse_model_list("").is_empty());
        assert!(parse_model_list("   \n\n").is_empty());
    }

    #[test]
    fn parse_model_list_bounds_unexpected_output() {
        // Een hulptekst of stacktrace op stdout mag de datalist niet volspammen.
        let long = "x".repeat(121);
        assert!(parse_model_list(&long).is_empty());
        let many = (0..200)
            .map(|i| format!("model {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let got = parse_model_list(&many);
        assert_eq!(got.len(), 64);
        assert_eq!(got[0], "model 0");
    }

    #[test]
    fn only_agy_has_a_model_list_command() {
        assert_eq!(model_list_subcommand("agy"), Some("models"));
        assert_eq!(model_list_subcommand("claude"), None);
        assert_eq!(model_list_subcommand(""), None);
    }

    // Roept de echte CLI aan, dus alleen zinvol op een machine waar agy op PATH
    // staat -- daarom #[ignore]: `cargo test -- --ignored` draait 'm bewust.
    #[test]
    #[ignore]
    fn list_agent_models_talks_to_the_agy_cli() {
        let models = list_agent_models("agy".to_string()).expect("agy models failed");
        assert!(!models.is_empty());
        // agy kijkt naar zijn stdout: een terminal krijgt labels ("Gemini 3.6
        // Flash (Low)"), een pipe -- dus ook wij -- krijgt slugs
        // ("gemini-3.6-flash-low"). Beide vormen worden door --model
        // geaccepteerd, dus we eisen alleen bruikbare, getrimde entries.
        assert!(
            models.iter().all(|m| !m.is_empty() && m.trim() == m),
            "unexpected entries: {:?}",
            models
        );
        // Een agent zonder list-commando hoort netjes te falen i.p.v. te spawnen.
        assert!(list_agent_models("claude".to_string()).is_err());
    }
}
