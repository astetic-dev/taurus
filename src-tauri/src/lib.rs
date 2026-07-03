// Taurus - Agent Launcher
// Draait Claude Code-agents als ingebedde terminals (ConPTY) MET tabs in het
// eigen venster. Elke sessie = een pseudo-terminal die `claude -n <titel>` draait
// in de juiste map (lokaal C: of netwerk X:), zodat je nooit hoeft te twijfelen.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use tauri::{AppHandle, Emitter, State};

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
}

// Lege defaults: een verse installatie start zonder projecten. De gebruiker
// voegt ze toe via de in-app editor ("Projecten"). Zo bevat een gedeelde build
// geen machine-specifieke paden.
fn default_projects() -> Vec<Project> {
    Vec::new()
}

// Per-gebruiker config: %APPDATA%\Taurus\projects.json (schrijfbaar, geen
// hardcoded dev-pad, werkt na installatie).
fn config_dir() -> std::path::PathBuf {
    let base = std::env::var("APPDATA")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join("Taurus")
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

#[tauri::command]
fn get_projects() -> Vec<Project> {
    if let Ok(txt) = std::fs::read_to_string(ensure_config()) {
        if let Ok(list) = serde_json::from_str::<Vec<Project>>(&txt) {
            return list;
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
    if let Ok(txt) = std::fs::read_to_string(sessions_path()) {
        if let Ok(list) = serde_json::from_str::<Vec<PersistedSession>>(&txt) {
            return list;
        }
    }
    Vec::new()
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

// Zoek het exe van de agent via PATH, met fallback naar de kale naam.
fn resolve_program(agent: &str) -> String {
    let exe = agent_exe(agent);
    if let Ok(paths) = std::env::var("PATH") {
        for p in std::env::split_paths(&paths) {
            let cand = p.join(exe);
            if cand.is_file() {
                return cand.to_string_lossy().into_owned();
            }
        }
    }
    exe.to_string()
}

// Een actieve terminal-sessie: de PTY-master (voor resize), de writer (stdin)
// en het kindproces (om te kunnen afsluiten).
struct Session {
    writer: Box<dyn Write + Send>,
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

struct AppState {
    sessions: Mutex<HashMap<String, Session>>,
    // STT-opname: commando-kanaal naar de audio-thread + zichtbare status.
    stt: SttState,
}

// Start een claude-proces in een ConPTY en registreer de sessie onder `id`.
fn start_pty(
    app: &AppHandle,
    sessions: &Mutex<HashMap<String, Session>>,
    id: String,
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
    // claude zijn config/credentials vindt.
    for (k, v) in std::env::vars() {
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
                    let chunk = buf[..n].to_vec();
                    if app2.emit("pty-output", (id2.clone(), chunk)).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        let _ = app2.emit("pty-exit", id2.clone());
    });

    sessions.lock().unwrap().insert(
        id,
        Session {
            writer,
            master: pair.master,
            child,
        },
    );
    Ok(())
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

// Bouw (programma, argumenten) voor de gekozen agent. De `command`-escape-hatch
// wordt door de aanroeper afgehandeld; hier gaat het puur om claude/agy. De twee
// CLIs verschillen sterk in vlaggen, dus we bouwen ze apart op i.p.v. Claude's
// vlaggen voor beide aan te nemen.
fn build_command(
    agent: &str,
    kind: LaunchKind,
    session_id: &str,
    title: &str,
    task: &str,
    mode: &str,
    model: &str,
    full_paths: bool,
) -> (String, Vec<String>) {
    let program = resolve_program(agent);
    let mut a: Vec<String> = Vec::new();
    match agent {
        // agy (Antigravity/Gemini-agent): kent geen --session-id / -n /
        // --permission-mode / --append-system-prompt. Modus: auto -> alle tools
        // auto-goedkeuren; sandbox -> beperkte terminalrechten. full_paths heeft
        // geen equivalent en wordt overgeslagen. Het model is de volledige
        // agy-label-string (bijv. "Gemini 3.5 Flash (Medium)").
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

// Start een nieuwe agent-sessie. session_id is een vooraf bepaalde UUID, zodat we
// later kunnen herstarten met `claude --resume <uuid>`.
#[tauri::command]
fn create_session(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    path: String,
    title: String,
    task: String,
    session_id: String,
    mode: String,
    full_paths: bool,
    command: String,
    agent: String,
    model: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let (program, args) = if !command.trim().is_empty() {
        // Commando-override (bijv. nep-Claude voor de demo): voer dit programma
        // uit i.p.v. de agent, zonder agent-vlaggen.
        let mut toks: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        let prog = toks.remove(0);
        (prog, toks)
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
        )
    };
    start_pty(&app, &state.sessions, id, program, &path, args, cols, rows)
}

// Herstart een sessie: stop het huidige claude-proces en hervat hetzelfde gesprek
// met `claude --resume <uuid>` (bijv. na een MCP-server-update).
#[tauri::command]
fn restart_session(
    app: AppHandle,
    state: State<AppState>,
    id: String,
    path: String,
    title: String,
    session_id: String,
    mode: String,
    full_paths: bool,
    command: String,
    agent: String,
    model: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    {
        if let Some(mut s) = state.sessions.lock().unwrap().remove(&id) {
            let _ = s.child.kill();
        }
    }
    let (program, args) = if !command.trim().is_empty() {
        let mut toks: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        let prog = toks.remove(0);
        (prog, toks)
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
        )
    };
    start_pty(&app, &state.sessions, id, program, &path, args, cols, rows)
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
            .map(|e| e.eq_ignore_ascii_case("html") || e.eq_ignore_ascii_case("htm"))
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

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
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

#[tauri::command]
fn list_tts_voices() -> Vec<String> {
    let script = "Add-Type -AssemblyName System.Speech; \
        (New-Object System.Speech.Synthesis.SpeechSynthesizer).GetInstalledVoices() \
        | ForEach-Object { $_.VoiceInfo.Name }";
    match ps_encoded(script).output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    }
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
    let sel = {
        let v = voice.replace('\'', "''");
        if v.trim().is_empty() { String::new() } else { format!("try {{ $s.SelectVoice('{}') }} catch {{}}; ", v) }
    };
    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         {}$s.Rate = {}; $s.Speak('{}')",
        sel,
        rate.clamp(-10, 10),
        t
    );
    ps_encoded(&script).spawn().map(|_| ()).map_err(|e| e.to_string())
}

// --- STT: opname op een eigen audio-thread (cpal Streams zijn !Send) ---
enum RecCmd {
    Start,
    // Antwoord: pad van de geschreven WAV, of een fout.
    Stop(std::sync::mpsc::Sender<Result<std::path::PathBuf, String>>),
}

struct SttState {
    tx: std::sync::mpsc::Sender<RecCmd>,
    recording: std::sync::atomic::AtomicBool,
}

fn stt_dir() -> std::path::PathBuf {
    config_dir().join("stt")
}

fn stt_wav_path() -> std::path::PathBuf {
    std::env::temp_dir().join("taurus-stt.wav")
}

fn start_capture() -> Result<(cpal::Stream, std::sync::Arc<Mutex<Vec<f32>>>, u32, u16), String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let device = cpal::default_host()
        .default_input_device()
        .ok_or("no microphone found")?;
    let cfg = device.default_input_config().map_err(|e| e.to_string())?;
    let rate = cfg.sample_rate().0;
    let channels = cfg.channels();
    let buf = std::sync::Arc::new(Mutex::new(Vec::<f32>::new()));
    let b2 = buf.clone();
    let err_fn = |_e| {};
    let stream = match cfg.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &cfg.into(),
            move |data: &[f32], _| b2.lock().unwrap().extend_from_slice(data),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &cfg.into(),
            move |data: &[i16], _| {
                b2.lock().unwrap().extend(data.iter().map(|s| *s as f32 / 32768.0))
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &cfg.into(),
            move |data: &[u16], _| {
                b2.lock().unwrap().extend(data.iter().map(|s| (*s as f32 - 32768.0) / 32768.0))
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

fn audio_thread(rx: std::sync::mpsc::Receiver<RecCmd>) {
    let mut current: Option<(cpal::Stream, std::sync::Arc<Mutex<Vec<f32>>>, u32, u16)> = None;
    for cmd in rx {
        match cmd {
            RecCmd::Start => {
                if current.is_none() {
                    current = start_capture().ok();
                }
            }
            RecCmd::Stop(reply) => {
                let res = match current.take() {
                    Some((stream, buf, rate, ch)) => {
                        drop(stream); // stopt de opname
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
fn stt_download(engine_url: String, model_url: String) -> Result<(), String> {
    for u in [&engine_url, &model_url] {
        if !u.starts_with("https://") {
            return Err(format!("https URLs only: {}", u));
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
            for url in [engine_url, model_url] {
                let name = url.rsplit('/').next().unwrap_or("archive.tar.bz2");
                let file = d.join(name);
                if !file.is_file() {
                    dl_log(&d, &format!("downloading {}", url));
                    ps_fetch(&url, &file)?;
                }
                dl_log(&d, &format!("extracting {}", name));
                extract_tar_bz2(&file, &d)?;
            }
            Ok(())
        })();
        match res {
            Ok(()) => dl_log(&d, "done"),
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
        state.stt.tx.send(RecCmd::Start).map_err(|e| e.to_string())?;
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
    transcribe(&wav).map(|text| SttToggle { recording: false, text: Some(text) })
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Audio-thread voor STT: cpal-streams zijn !Send, dus één eigen thread
    // bezit de stream; commands praten er via een kanaal mee.
    let (stt_tx, stt_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || audio_thread(stt_rx));

    tauri::Builder::default()
        .manage(AppState {
            sessions: Mutex::new(HashMap::new()),
            stt: SttState {
                tx: stt_tx,
                recording: std::sync::atomic::AtomicBool::new(false),
            },
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
            {
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                // Mislukte registratie (F9 elders in gebruik) is geen ramp:
                // de mic-knop in de topbar blijft werken.
                let _ = app.global_shortcut().register("F9");
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
            pick_folder,
            pick_file,
            path_exists,
            app_version,
            has_claude_md,
            save_sessions,
            get_sessions,
            session_state,
            create_session,
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
            debug_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
