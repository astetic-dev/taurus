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
}

// Lege defaults: een verse installatie start zonder projecten. De gebruiker
// voegt ze toe via de in-app editor ("Projecten"). Zo bevat een gedeelde build
// geen machine-specifieke paden.
fn default_projects() -> Vec<Project> {
    Vec::new()
}

// Per-gebruiker config: %APPDATA%\<APP_DIR>\projects.json (schrijfbaar, geen
// hardcoded dev-pad, werkt na installatie). White-label: de NEXUS-build gebruikt
// een EIGEN map zodat hij naast Taurus draait zonder projects.json/sessions.json
// te delen -- anders zou de NEXUS-instantie Taurus' sessies hervatten.
#[cfg(feature = "nexus")]
const APP_DIR: &str = "NEXUS Agent Launcher";
#[cfg(not(feature = "nexus"))]
const APP_DIR: &str = "Taurus";

fn config_dir() -> std::path::PathBuf {
    let base = std::env::var("APPDATA")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join(APP_DIR)
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

// Zoek claude.exe via PATH, met fallback naar de kale naam.
fn resolve_claude() -> String {
    if let Ok(paths) = std::env::var("PATH") {
        for p in std::env::split_paths(&paths) {
            let cand = p.join("claude.exe");
            if cand.is_file() {
                return cand.to_string_lossy().into_owned();
            }
        }
    }
    "claude.exe".to_string()
}

// Een actieve terminal-sessie: de PTY-master (voor resize), de writer (stdin)
// en het kindproces (om te kunnen afsluiten).
struct Session {
    writer: Box<dyn Write + Send>,
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

#[derive(Default)]
struct AppState {
    sessions: Mutex<HashMap<String, Session>>,
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
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let (program, args) = if !command.trim().is_empty() {
        // Commando-override (bijv. nep-Claude voor de demo): voer dit programma
        // uit i.p.v. claude, zonder claude-vlaggen.
        let mut toks: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        let prog = toks.remove(0);
        (prog, toks)
    } else {
        let mut a = vec![
            "--session-id".into(),
            session_id,
            "-n".into(),
            norm_title(&title),
        ];
        if !mode.is_empty() && mode != "default" {
            a.push("--permission-mode".into());
            a.push(mode);
        }
        if full_paths {
            a.push("--append-system-prompt".into());
            a.push(FULL_PATH_PROMPT.into());
        }
        if !task.trim().is_empty() {
            a.push(task.trim().into());
        }
        (resolve_claude(), a)
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
        let mut a = vec![
            "--resume".into(),
            session_id,
            "-n".into(),
            norm_title(&title),
        ];
        if !mode.is_empty() && mode != "default" {
            a.push("--permission-mode".into());
            a.push(mode);
        }
        if full_paths {
            a.push("--append-system-prompt".into());
            a.push(FULL_PATH_PROMPT.into());
        }
        (resolve_claude(), a)
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
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

// ===== White-label branding =====
// Een NEXUS-build is dezelfde engine met andere branding. Alles wat per merk
// verschilt (naam, ondertitel, logo, kleur-variabelen) komt hiervandaan; de
// frontend past het toe bij het opstarten via invoke("branding"). De standaard-
// build (geen feature) levert exact de Taurus-waarden, dus die blijft visueel
// ongewijzigd. De NEXUS-variant bouw je met `cargo build --features nexus` (en
// voor de installer met --config tauri.nexus.conf.json).
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Branding {
    app_name: String,
    // None = laat de ondertitel uit de UI/i18n staan (Taurus). Some("...") =
    // overschrijf met deze tekst; Some("") = verberg de ondertitel helemaal.
    brand_sub: Option<String>,
    window_title: String,
    logo: Option<String>,
    theme: HashMap<String, String>,
}

#[tauri::command]
fn branding() -> Branding {
    #[cfg(feature = "nexus")]
    {
        // Officiele NEXUS-huisstijl (bron: regels.md HUISSTIJL-VISUEEL ->
        // bedrijfsbreed/huisstijl.md, Merkpaspoort AIED/4128793). Drie families:
        //   Red:   Main #CC3333, Light #DD4A4A, Dark #AF2F2F  (primair accent)
        //   White: Main #EDF2F4, Light #F7F7F7, Dark #CED5D8  (achtergrond/tekst)
        //   Blue:  Main #24252F, Light #8D99AE               (donker accent/tekst)
        // Regel: GEEN groen, geen Microsoft-blauw, geen cyaan. Statuskleuren die
        // in Taurus groen/oranje zijn, mappen we zoals de NEXUS-dashboards doen:
        // groen -> staalgrijs (Light-Blue), oranje -> Light-Red.
        return Branding {
            // Logo toont al "NEXUS nederland"; titel hoeft dat niet te herhalen.
            // De ondertitel is een tool-tagline (werktitel, niet de officiele
            // NEXUS-payoff -- die staat niet in de lokale huisstijl-bronnen).
            app_name: "Agent Launcher".to_string(),
            brand_sub: Some("Start in de juiste context".to_string()),
            window_title: "NEXUS Agent Launcher".to_string(),
            logo: Some("nexus-logo.png".to_string()),
            theme: HashMap::from([
                ("--bg".to_string(), "#16171d".to_string()),
                ("--bg-panel".to_string(), "#1d1e27".to_string()),
                ("--bg-card".to_string(), "#24252f".to_string()),
                ("--bg-card-hover".to_string(), "#2e2f3b".to_string()),
                ("--border".to_string(), "#353746".to_string()),
                ("--text".to_string(), "#edf2f4".to_string()),
                ("--text-dim".to_string(), "#8d99ae".to_string()),
                ("--accent".to_string(), "#cc3333".to_string()),
                ("--green".to_string(), "#8d99ae".to_string()),
                ("--orange".to_string(), "#dd4a4a".to_string()),
                ("--red".to_string(), "#cc3333".to_string()),
            ]),
        };
    }
    #[cfg(not(feature = "nexus"))]
    {
        return Branding {
            app_name: "Taurus".to_string(),
            brand_sub: None,
            window_title: String::new(), // leeg = laat de <title> uit index.html staan
            logo: Some("taurus-logo.png".to_string()),
            theme: HashMap::new(),
        };
    }
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
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            disable_accelerator_keys(app.handle());
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
            copy_to_clipboard,
            branding
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
