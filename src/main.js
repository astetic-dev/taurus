const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

/* ============ i18n ============ */
const I18N = {
  nl: {
    brand_sub: "Agent Launcher", projects: "Projecten",
    foot_projects: "✎ Projecten", foot_settings: "⚙ Instellingen", foot_reload: "⟳ Herlaad",
    empty_pick: "Kies links een project om een agent te starten.",
    browse_folder: "📁 Blader naar een map…",
    no_claude_md: "ℹ Geen CLAUDE.md in deze map — agent start zonder projectinstructies.",
    workdir: "Werkmap", session_title: "Titel van de sessie",
    title_hint: "Wordt de naam in Claude én op het tabblad.",
    task: "Taak", optional: "(optioneel)",
    task_ph: "Laat leeg voor een lege sessie, of typ meteen een opdracht…",
    title_ph: "bijv. ZGV-SAML-debug", start_agent: "▶ Start agent",
    path_warn: "⚠ Map niet bereikbaar", newtab: "＋ Nieuw",
    settings: "Instellingen", lang_label: "Taal / Language", lang_choose: "Kies taal",
    grp_screen: "Scherm", set_font: "Lettergrootte", set_scroll: "Scrollback (regels)", set_cursor: "Cursor knippert",
    grp_html: "HTML-preview", html_split: "Naast de terminal (split)", html_full: "Volledig (verberg terminal)",
    set_fullpaths: "Vraag Claude volledige paden te tonen (klikbaar)",
    launch_mode: "Modus", mode_default: "Standaard", mode_plan: "Plan-modus", mode_auto: "Auto (accepteert acties)",
    mode_sandbox: "Sandbox (beperkte rechten)",
    launch_agent: "Agent", agent_claude: "Claude Code", agent_agy: "Antigravity",
    launch_model: "Model", model_ph: "standaard", model_hint: "Leeg = de standaard van de agent.",
    cap_agent: "Agent", cap_model: "Model (leeg = standaard)",
    grp_comfort: "Terminal-comfort", comfort_hint: "(per voorkeur aan/uit)",
    c_copy: "Selectie kopieert automatisch", c_paste: "Rechtermuisklik plakt", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Klikbare links", c_links_new: "(nieuwe sessies)", c_search: "Zoeken in scrollback — Ctrl+Shift+F",
    c_tabs: "Tab-sneltoetsen (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude-status op de tab (✶ Orbiting…)",
    cancel: "Annuleer", save: "Opslaan",
    manage_projects: "Projecten beheren", add_project: "＋ Project toevoegen",
    cap_button: "▸ Knop in het linkermenu", cap_workdir: "Werkmap (lokaal C: of netwerk X:)",
    cap_tabtitle: "⎯ Tabtitel bovenin (standaard, per sessie aanpasbaar)", cap_task: "Taak — wordt direct meegestuurd (optioneel)",
    ph_label: "bijv. DVZA", ph_path: "C:\\… of X:\\…", ph_title: "bijv. DVZA-cert", ph_task: "laat leeg voor een lege sessie",
    search_ph: "Zoeken…",
    ctx_restart: "↻ Herstart (resume gesprek)", ctx_preview: "👁 HTML-preview", ctx_explorer: "📂 Open map in Verkenner", ctx_close: "✕ Sluiten",
    preview_none: "(geen .html in de werkmap)", preview_refresh: "Vernieuwen", preview_mode: "Split / Volledig", preview_close: "Preview sluiten",
    loc_local: "LOKAAL", loc_net: "NETWERK", loc_unknown: "ONBEKEND",
    ended: "[sessie beëindigd — rechtsklik tab voor herstart, of sluit]",
    restarting: "herstarten — resume", restart_failed: "herstart mislukt",
    grp_sessions: "Sessies", set_persist: "Sessies onthouden en bij opstarten hervatten",
    grp_theme: "Thema", set_skin: "Skin", skin_hint: "Of zet een vaste default in branding.json (zie README).",
    skin_default: "Standaard (donker)", skin_retromac: "Retro Mac", skin_aqua: "macOS Aqua",
    skin_retrowin: "Retro Windows", skin_winxp: "Windows XP", skin_terminal: "Terminal (CRT)",
    restore_failed: "Hervatten mislukt voor:",
    err_need_project: "✗ Minstens één project met naam én pad nodig.",
  },
  en: {
    brand_sub: "Agent Launcher", projects: "Projects",
    foot_projects: "✎ Projects", foot_settings: "⚙ Settings", foot_reload: "⟳ Reload",
    empty_pick: "Pick a project on the left to start an agent.",
    browse_folder: "📁 Browse for a folder…",
    no_claude_md: "ℹ No CLAUDE.md in this folder — the agent starts without project instructions.",
    workdir: "Working folder", session_title: "Session title",
    title_hint: "Becomes the name in Claude and on the tab.",
    task: "Task", optional: "(optional)",
    task_ph: "Leave empty for a blank session, or type a task…",
    title_ph: "e.g. ZGV-SAML-debug", start_agent: "▶ Start agent",
    path_warn: "⚠ Folder not reachable", newtab: "＋ New",
    settings: "Settings", lang_label: "Taal / Language", lang_choose: "Choose language",
    grp_screen: "Screen", set_font: "Font size", set_scroll: "Scrollback (lines)", set_cursor: "Cursor blinks",
    grp_html: "HTML preview", html_split: "Beside the terminal (split)", html_full: "Full (hide terminal)",
    set_fullpaths: "Ask Claude to print full paths (clickable)",
    launch_mode: "Mode", mode_default: "Default", mode_plan: "Plan mode", mode_auto: "Auto (accepts actions)",
    mode_sandbox: "Sandbox (restricted)",
    launch_agent: "Agent", agent_claude: "Claude Code", agent_agy: "Antigravity",
    launch_model: "Model", model_ph: "default", model_hint: "Empty = the agent's default.",
    cap_agent: "Agent", cap_model: "Model (empty = default)",
    grp_comfort: "Terminal comfort", comfort_hint: "(toggle to taste)",
    c_copy: "Selection copies automatically", c_paste: "Right-click pastes", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Clickable links", c_links_new: "(new sessions)", c_search: "Search scrollback — Ctrl+Shift+F",
    c_tabs: "Tab shortcuts (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude status on the tab (✶ Orbiting…)",
    cancel: "Cancel", save: "Save",
    manage_projects: "Manage projects", add_project: "＋ Add project",
    cap_button: "▸ Button in the left menu", cap_workdir: "Working folder (local C: or network X:)",
    cap_tabtitle: "⎯ Tab title (default, editable per session)", cap_task: "Task — sent immediately (optional)",
    ph_label: "e.g. DVZA", ph_path: "C:\\… or X:\\…", ph_title: "e.g. DVZA-cert", ph_task: "leave empty for a blank session",
    search_ph: "Search…",
    ctx_restart: "↻ Restart (resume conversation)", ctx_preview: "👁 HTML preview", ctx_explorer: "📂 Open folder in Explorer", ctx_close: "✕ Close",
    preview_none: "(no .html in the working folder)", preview_refresh: "Refresh", preview_mode: "Split / Full", preview_close: "Close preview",
    loc_local: "LOCAL", loc_net: "NETWORK", loc_unknown: "UNKNOWN",
    ended: "[session ended — right-click tab to restart, or close]",
    restarting: "restarting — resume", restart_failed: "restart failed",
    grp_sessions: "Sessions", set_persist: "Remember sessions and resume on startup",
    grp_theme: "Theme", set_skin: "Skin", skin_hint: "Or set a fixed default in branding.json (see README).",
    skin_default: "Default (dark)", skin_retromac: "Retro Mac", skin_aqua: "macOS Aqua",
    skin_retrowin: "Retro Windows", skin_winxp: "Windows XP", skin_terminal: "Terminal (CRT)",
    restore_failed: "Could not resume:",
    err_need_project: "✗ Need at least one project with a name and a path.",
  },
};
function t(k) { return (I18N[settings.lang] || I18N.nl)[k] ?? k; }
function applyI18n() {
  const d = I18N[settings.lang] || I18N.nl;
  document.querySelectorAll("[data-i18n]").forEach((el) => { const k = el.getAttribute("data-i18n"); if (d[k] != null) el.textContent = d[k]; });
  document.querySelectorAll("[data-i18n-ph]").forEach((el) => { const k = el.getAttribute("data-i18n-ph"); if (d[k] != null) el.placeholder = d[k]; });
  document.documentElement.lang = settings.lang || "nl";
}

/* ============ white-label branding ============ */
// Optionele branding uit %APPDATA%\Taurus\branding.json (via de Rust-command).
// Lege velden = geen override, dus zonder bestand blijft alles gewoon Taurus.
// Draait NA applyI18n zodat een ingestelde ondertitel niet overschreven wordt.
async function applyBranding() {
  let b;
  try { b = await invoke("branding"); } catch (_) { return; }
  if (!b) return;
  if (b.appName) {
    const el = document.querySelector(".brand-title");
    if (el) el.textContent = b.appName;
  }
  if (b.subtitle) {
    const el = document.querySelector(".brand-sub");
    // data-i18n weghalen zodat een taalwissel de ondertitel niet terugzet.
    if (el) { el.textContent = b.subtitle; el.removeAttribute("data-i18n"); }
  }
  if (b.logoDataUri) {
    const img = document.querySelector(".brand-logo");
    if (img) img.src = b.logoDataUri;
  }
  if (b.theme && typeof b.theme === "object") {
    for (const [k, v] of Object.entries(b.theme)) {
      if (/^--[\w-]+$/.test(k) && typeof v === "string") document.documentElement.style.setProperty(k, v);
    }
  }
  if (b.windowTitle) {
    document.title = b.windowTitle;
    try { window.__TAURI__.window.getCurrentWindow().setTitle(b.windowTitle); } catch (_) {}
  }
  // Skin-default uit branding onthouden en de effectieve skin toepassen:
  // expliciete keuze in Instellingen wint, anders de branding-default, anders default.
  brandingSkin = (b.skin || "").trim();
  applySkin(settings.skin || brandingSkin || "default");
}

/* ============ skins ============ */
// De gekozen skin hangt als data-skin op <html>; skins.css doet de rest.
// "default" / leeg = geen attribuut (gewoon :root). Het terminal-thema leest de
// --term-* variabelen zodat de skin ook in de xterm-terminal doorwerkt.
let brandingSkin = "";
function termThemeFromCss(accent) {
  const cs = getComputedStyle(document.documentElement);
  const v = (n, fb) => (cs.getPropertyValue(n).trim() || fb);
  return {
    background: v("--term-bg", "#14161c"),
    foreground: v("--term-fg", "#e6e8ee"),
    cursor: accent || "#7c9cff",
    selectionBackground: v("--term-sel", "#33405c"),
  };
}
function applySkin(name) {
  const skin = name && name !== "default" ? name : "";
  if (skin) document.documentElement.setAttribute("data-skin", skin);
  else document.documentElement.removeAttribute("data-skin");
  // Open terminals her-thematiseren (cursor blijft de project-accent).
  for (const s of sessions.values()) {
    try { s.term.options.theme = termThemeFromCss(s.accent); } catch (_) {}
  }
}

/* ============ agents + modellen ============ */
// Welke agent-CLIs Taurus kan starten. De losse gemini-CLI is end-of-life en
// staat hier bewust niet bij; "agy" is de ondersteunde Gemini-agent.
const AGENTS = ["claude", "agy"];
// Claude Code's --model accepteert een alias of een exact model-ID, geen vrije
// weergavenaam (anders dan agy). We tonen nette namen als suggestie en vertalen
// die in resolveModelArg() naar het model-ID dat de CLI accepteert; een alias of
// ID die je zelf typt gaat ongewijzigd door. De opgeslagen/getoonde waarde blijft
// de nette naam.
const CLAUDE_MODELS = {
  "Claude Opus 4.8": "claude-opus-4-8",
  "Claude Sonnet 4.6": "claude-sonnet-4-6",
  "Claude Haiku 4.5": "claude-haiku-4-5",
};
// Suggesties voor het model-veld (datalist). Vrije tekst blijft toegestaan en
// leeg = de eigen default van de agent; de lijst is slechts een hint, want
// model-ID's verschuiven en het veld dwingt niets af. agy selecteert op de
// volledige label-string (incl. effort-suffix), dus die nemen we letterlijk over.
const MODEL_SUGGESTIONS = {
  claude: Object.keys(CLAUDE_MODELS),
  agy: [
    "Gemini 3.5 Flash (Medium)",
    "Gemini 3.5 Flash (High)",
    "Gemini 3.5 Flash (Low)",
    "Gemini 3.1 Pro (Low)",
    "Gemini 3.1 Pro (High)",
    "GPT-OSS 120B (Medium)",
  ],
};
// Vertaal de gekozen/ingetypte modelnaam naar de --model-waarde voor de agent.
// Voor claude: nette naam -> model-ID; alles anders (alias of ID) ongewijzigd.
// Voor agy: ongewijzigd (agy verwacht juist de volledige label-string).
function resolveModelArg(agent, model) {
  const m = (model || "").trim();
  if (agent === "claude" && CLAUDE_MODELS[m]) return CLAUDE_MODELS[m];
  return m;
}
function fillModelDatalist(dl, agent) {
  if (!dl) return;
  dl.innerHTML = "";
  for (const m of MODEL_SUGGESTIONS[agent] || MODEL_SUGGESTIONS.claude) {
    const o = document.createElement("option");
    o.value = m;
    dl.appendChild(o);
  }
}

// Modus-opties verschillen per agent. claude: --permission-mode default/plan/auto.
// agy: geen --permission-mode, wel --sandbox (beperkt) en
// --dangerously-skip-permissions (auto). Waarden komen overeen met de mapping in
// build_command() in de Rust-backend.
const MODE_OPTIONS = {
  claude: [
    { value: "default", key: "mode_default" },
    { value: "plan", key: "mode_plan" },
    { value: "auto", key: "mode_auto" },
  ],
  agy: [
    { value: "default", key: "mode_default" },
    { value: "sandbox", key: "mode_sandbox" },
    { value: "auto", key: "mode_auto" },
  ],
};
function modesFor(agent) { return MODE_OPTIONS[agent] || MODE_OPTIONS.claude; }
// Geldige modus voor deze agent? Anders terug naar "default" (bv. claude "plan"
// bestaat niet voor agy).
function clampMode(agent, mode) {
  return modesFor(agent).some((o) => o.value === mode) ? mode : "default";
}
// (Her)vul een <select> met de modus-opties van de agent; bewaar de huidige keuze.
function fillModeSelect(sel, agent, current) {
  if (!sel) return;
  const val = clampMode(agent, current);
  sel.innerHTML = "";
  for (const o of modesFor(agent)) {
    const opt = document.createElement("option");
    opt.value = o.value;
    opt.textContent = t(o.key);
    sel.appendChild(opt);
  }
  sel.value = val;
}

/* ============ state ============ */
let projects = [];
let selected = null;
let current = "new";
let seq = 0;
const sessions = new Map();
const els = {};

const DEFAULT_SETTINGS = {
  lang: "nl",
  fontSize: 13, scrollback: 8000, cursorBlink: true,
  htmlView: "split",
  copyOnSelect: true, pasteOnRightClick: true, ctrlShiftCV: true,
  webLinks: true, search: true, tabShortcuts: true, tabStatus: true,
  fullPaths: true,
  persistSessions: true,
  skin: "", // "" = volg branding-default / anders "default"
};
let settings = { ...DEFAULT_SETTINGS };

function loadSettings() {
  try { const raw = localStorage.getItem("taurus.settings"); if (raw) settings = { ...DEFAULT_SETTINGS, ...JSON.parse(raw) }; } catch (_) {}
}
function saveSettings() { localStorage.setItem("taurus.settings", JSON.stringify(settings)); }

/* ============ helpers ============ */
// Klembord-schrijven loopt via een native Rust-command, niet via de WebView2
// browser-API navigator.clipboard.writeText. Een WebView2/Edge-update kan die
// write in de webview blokkeren (kopieren-bij-selectie en Ctrl+Shift+C deden
// niets meer), terwijl het OS-klembord prima werkt -- zie issue #17. De fout
// niet langer stil wegslikken: loggen naar de console.
function copyToClipboard(text) {
  if (!text) return;
  invoke("copy_to_clipboard", { text }).catch((e) => console.error("clipboard copy failed:", e));
}
function isNetwork(p) { return /^x:/i.test(p) || p.startsWith("\\\\"); }
function locClass(p) { return isNetwork(p) ? "net" : "local"; }
function locText(p) {
  if (p.startsWith("\\\\")) return t("loc_net") + " (UNC)";
  const d = (p.match(/^([a-z]):/i) || [])[1];
  if (!d) return t("loc_unknown");
  return (isNetwork(p) ? t("loc_net") : t("loc_local")) + ` (${d.toUpperCase()}:)`;
}
function escapeHtml(s) {
  return String(s).replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
}
function modalOpen() { return !!document.querySelector(".modal:not(.hidden)"); }

/* ============ projecten ============ */
function renderProjects() {
  els.list.innerHTML = "";
  for (const p of projects) {
    const card = document.createElement("div");
    card.className = "project-card";
    card.style.borderLeftColor = p.accent || "#7c9cff";
    card.innerHTML = `<div class="pc-label">${escapeHtml(p.label)}</div><div class="pc-loc ${locClass(p.path)}">${locText(p.path)}</div>`;
    card.addEventListener("click", () => { selectProject(p, card); showView("new"); });
    els.list.appendChild(card);
  }
}
async function selectProject(p, card) {
  selected = p;
  document.querySelectorAll(".project-card").forEach((c) => c.classList.remove("selected"));
  if (card) card.classList.add("selected");
  els.formEmpty.classList.add("hidden");
  els.form.classList.remove("hidden");
  els.formTitle.textContent = p.label;
  els.formTitle.style.color = p.accent || "var(--text)";
  els.pathValue.textContent = p.path;
  els.locBadge.textContent = locText(p.path);
  els.locBadge.className = "loc-badge " + locClass(p.path);
  els.titleInput.value = p.title || p.label;
  els.taskInput.value = p.task || "";
  els.agentInput.value = AGENTS.includes(p.agent) ? p.agent : "claude";
  els.modelInput.value = p.model || "";
  fillModelDatalist(els.modelSuggestions, els.agentInput.value);
  fillModeSelect(els.modeInput, els.agentInput.value, p.mode || "default");
  els.status.textContent = "";
  const ok = await invoke("path_exists", { path: p.path });
  els.warn.classList.toggle("hidden", ok);
  let hasMd = false;
  try { hasMd = await invoke("has_claude_md", { path: p.path }); } catch (_) {}
  els.claudeWarn.classList.toggle("hidden", hasMd);
}
async function loadProjects() { projects = await invoke("get_projects"); renderProjects(); }

// Open de OS-mapkiezer en start een ad-hoc launch in de gekozen map: vul het
// startformulier voor die map (niet opgeslagen als project). Optimaal heeft de
// map een CLAUDE.md; zo niet, dan toont selectProject een niet-blokkerende hint.
async function browseFolder() {
  let dir = null;
  try { dir = await invoke("pick_folder"); } catch (_) { return; }
  if (!dir) return; // geannuleerd
  const name = dir.replace(/[\\/]+$/, "").split(/[\\/]/).pop() || dir;
  await selectProject({ id: "", label: name, path: dir, title: name, task: "", accent: "#7c9cff", mode: "default", command: "", agent: "claude", model: "" }, null);
  showView("new");
}

/* ============ tabbalk ============ */
function renderTabs() {
  els.tabbar.innerHTML = "";
  for (const s of sessions.values()) {
    const tab = document.createElement("div");
    let cls = "tab";
    if (current === s.id) cls += " active";
    if (s.exited) cls += " exited"; else if (s.awaiting) cls += " awaiting"; else if (s.working) cls += " working";
    tab.className = cls;
    tab.style.borderTopColor = s.accent || "#7c9cff";
    tab.style.setProperty("--tab-accent", s.accent || "#7c9cff");
    const live = settings.tabStatus && s.status && !s.exited;
    const shown = live ? `✶ ${s.status}…` : s.title;
    tab.innerHTML = `<span class="tab-dot"></span><span class="tab-title${live ? " live" : ""}">${escapeHtml(shown)}</span><span class="tab-close">✕</span>`;
    tab.title = s.title;
    tab.addEventListener("click", () => showView(s.id));
    tab.addEventListener("contextmenu", (e) => { e.preventDefault(); openTabMenu(e.clientX, e.clientY, s.id); });
    tab.querySelector(".tab-close").addEventListener("click", (e) => { e.stopPropagation(); closeSession(s.id); });
    els.tabbar.appendChild(tab);
  }
  const plus = document.createElement("div");
  plus.className = "tab newtab" + (current === "new" ? " active" : "");
  plus.textContent = t("newtab");
  plus.addEventListener("click", () => { resetLaunchForm(); showView("new"); });
  els.tabbar.appendChild(plus);
}

// Zet de launch-view terug naar de lege startstaat: geen geselecteerd project,
// het ingevulde formulier verborgen en het lege scherm met de browse-knop zichtbaar.
// Wordt alleen bij ＋ Nieuw / Ctrl+T aangeroepen -- selectProject/browseFolder die
// het formulier juist tonen, blijven zo ongemoeid.
function resetLaunchForm() {
  selected = null;
  document.querySelectorAll(".project-card").forEach((c) => c.classList.remove("selected"));
  els.form.classList.add("hidden");
  els.formEmpty.classList.remove("hidden");
}

function showView(target) {
  current = target;
  const showTerm = target !== "new";
  els.launchView.classList.toggle("hidden", showTerm);
  els.terminals.classList.toggle("hidden", !showTerm);
  if (!showTerm) closeSearch();
  for (const s of sessions.values()) s.el.classList.toggle("hidden", s.id !== target);
  if (showTerm) {
    const s = sessions.get(target);
    if (s) { s.awaiting = false; refitTerm(s); s.term.focus(); }
  }
  renderTabs();
}

/* ============ sessie starten ============ */
// Bouwt de terminal-UI + sessie-object en bedraadt alle events. Doet NIET zelf de
// backend-aanroep (create vs resume verschilt) -- dat doet de aanroeper.
function spawnTerminal({ id, uuid, path, title, accent, mode, command, agent, model }) {
  const el = document.createElement("div");
  el.className = "term-container";
  el.innerHTML = `
    <div class="term-pane"></div>
    <div class="preview-pane">
      <div class="preview-bar">
        <select class="preview-file"></select>
        <button class="preview-refresh">↻</button>
        <button class="preview-mode">⇆</button>
        <button class="preview-close">✕</button>
      </div>
      <iframe class="preview-frame" sandbox="allow-scripts" src="about:blank"></iframe>
    </div>`;
  els.terminals.appendChild(el);
  const termPane = el.querySelector(".term-pane");

  const term = new window.Terminal({
    fontFamily: '"Cascadia Code", Consolas, "Courier New", monospace',
    fontSize: settings.fontSize, cursorBlink: settings.cursorBlink, scrollback: settings.scrollback,
    theme: termThemeFromCss(accent),
  });
  const fit = new window.FitAddon.FitAddon();
  term.loadAddon(fit);
  if (settings.webLinks && window.WebLinksAddon) term.loadAddon(new window.WebLinksAddon.WebLinksAddon());
  let search = null;
  if (settings.search && window.SearchAddon) { search = new window.SearchAddon.SearchAddon(); term.loadAddon(search); }
  term.open(termPane);
  fit.fit();

  const session = {
    id, uuid, path, title, accent, mode, command, agent: agent || "claude", model: model || "", term, fit, search, el,
    exited: false, working: false, awaiting: false, status: null, lastSpin: 0, buf: "",
    decoder: new TextDecoder("utf-8"), previewMode: null,
  };
  sessions.set(id, session);

  term.onData((d) => invoke("write_session", { id, data: d }));
  term.onResize(({ cols, rows }) => invoke("resize_session", { id, cols, rows }));
  term.onSelectionChange(() => { if (settings.copyOnSelect) { const sel = term.getSelection(); if (sel) copyToClipboard(sel); } });
  termPane.addEventListener("contextmenu", async (e) => {
    if (!settings.pasteOnRightClick) return;
    e.preventDefault();
    try { const txt = await navigator.clipboard.readText(); if (txt) invoke("write_session", { id, data: txt }); } catch (_) {}
  });

  el.querySelector(".preview-file").addEventListener("change", (e) => renderPreview(session, e.target.value));
  el.querySelector(".preview-refresh").addEventListener("click", () => loadHtmlList(session));
  el.querySelector(".preview-mode").addEventListener("click", () => { session.previewMode = session.previewMode === "split" ? "full" : "split"; applyLayout(session); refitTerm(session); });
  el.querySelector(".preview-close").addEventListener("click", () => closePreview(session));

  // Maak .html-paden in de terminal klikbaar -> opent ze in de preview.
  // Een lang absoluut pad wrapt vaak over meerdere terminalregels (smal
  // split-paneel). Daarom reconstrueren we eerst de volledige LOGISCHE regel
  // (eerste rij + alle isWrapped-vervolgrijen) en matchen we daarop; anders zou
  // de bevraagde rij alleen een staartfragment zien -> verkeerd (relatief) pad.
  if (term.registerLinkProvider) {
    term.registerLinkProvider({
      provideLinks(y, cb) {
        const buf = term.buffer.active;
        const cols = term.cols;
        const qi = y - 1; // 0-based index van de bevraagde rij
        if (!buf.getLine(qi)) { cb(undefined); return; }
        // Loop omhoog naar de eerste rij van de logische regel.
        let first = qi;
        while (first > 0) {
          const ln = buf.getLine(first);
          if (ln && ln.isWrapped) first--; else break;
        }
        // Verzamel de rijen van de logische regel; elke rij exact `cols` breed
        // zodat tekenoffsets terug te rekenen zijn naar (x, y)-celposities.
        const rows = [];
        for (let r = first; ; r++) {
          const ln = buf.getLine(r);
          if (!ln) break;
          rows.push(ln.translateToString(false).padEnd(cols, " ").slice(0, cols));
          const next = buf.getLine(r + 1);
          if (!next || !next.isWrapped) break;
        }
        const full = rows.join("");
        const w0 = (qi - first) * cols; // offset-venster van de bevraagde rij
        const w1 = w0 + cols;

        // Drive-letter paths may use either separator (C:\dir\f.html or C:/dir/f.html);
        // accept both so forward-slash absolute paths stay clickable too.
        const re = /([A-Za-z]:[\\/][^\s"'<>|]+?\.html?|[\w.\-\\/]+\.html?)/gi;
        const links = [];
        let m;
        while ((m = re.exec(full)) !== null) {
          const o0 = m.index;
          const o1 = m.index + m[1].length;
          const a = Math.max(o0, w0);
          const b = Math.min(o1, w1);
          if (a >= b) continue; // dit deel van de match valt niet op deze rij
          const txt = m[1];
          links.push({
            range: { start: { x: (a - w0) + 1, y }, end: { x: (b - 1 - w0) + 1, y } },
            text: txt,
            activate: () => openPreviewFile(session, txt), // altijd het VOLLEDIGE pad
          });
        }
        cb(links.length ? links : undefined);
      },
    });
  }
  return session;
}

async function startSession() {
  if (!selected) return;
  const title = els.titleInput.value.trim() || selected.label;
  const task = els.taskInput.value;
  const path = selected.path;
  const accent = selected.accent || "#7c9cff";
  const id = "s" + (++seq);
  const uuid = crypto.randomUUID();
  const mode = els.modeInput.value || "default";
  const command = selected.command || "";
  const agent = els.agentInput.value || "claude";
  const model = els.modelInput.value.trim();

  const session = spawnTerminal({ id, uuid, path, title, accent, mode, command, agent, model });
  try {
    await invoke("create_session", { id, path, title, task, sessionId: uuid, mode, fullPaths: settings.fullPaths, command, agent, model: resolveModelArg(agent, model), cols: session.term.cols, rows: session.term.rows });
    showView(id);
    persistSessionsToDisk();
  } catch (e) {
    sessions.delete(id); session.term.dispose(); session.el.remove();
    els.status.textContent = "✗ " + e; els.status.className = "status-msg err";
    renderTabs();
  }
}

/* ============ persistente sessies ============ */
// Schrijf de huidige (herstartbare) sessies naar schijf. Command-override-sessies
// (demo nep-Claude) hebben geen --resume-transcript en slaan we niet op.
function persistSessionsToDisk() {
  if (!settings.persistSessions) { invoke("save_sessions", { sessions: [] }).catch(() => {}); return; }
  const list = [...sessions.values()]
    .filter((s) => !s.command)
    .map((s) => ({ id: s.id, uuid: s.uuid, path: s.path, title: s.title, accent: s.accent, mode: s.mode || "default", agent: s.agent || "claude", model: s.model || "" }));
  invoke("save_sessions", { sessions: list }).catch(() => {});
}

// Bij opstarten: probeer elke opgeslagen sessie te hervatten met `--resume`, zonder
// te vragen. Ontbreekt het transcript (Claude heeft het opgeruimd) of is het ouder
// dan 1 dag -> overslaan, niet eens proberen. Een echte spawn-fout -> tab opruimen
// en melden welke (projectnaam) sessie niet lukte.
async function restoreSessions() {
  if (!settings.persistSessions) return;
  let saved = [];
  try { saved = await invoke("get_sessions"); } catch (_) { return; }
  if (!saved.length) return;

  const ONE_DAY = 86400;
  const failures = [];
  for (const meta of saved) {
    const uuid = meta.uuid;
    if (!uuid) continue;
    let st = { exists: false, ageSecs: 0 };
    try { st = await invoke("session_state", { path: meta.path, uuid }); } catch (_) {}
    if (!st.exists || st.ageSecs > ONE_DAY) continue; // stil overslaan

    const id = "s" + (++seq);
    const session = spawnTerminal({
      id, uuid, path: meta.path,
      title: meta.title || "agent", accent: meta.accent || "#7c9cff",
      mode: meta.mode || "default", command: "",
      agent: meta.agent || "claude", model: meta.model || "",
    });
    session.el.classList.add("hidden");
    session.term.write(`\x1b[2m[${t("restarting")} ${uuid.slice(0, 8)}…]\x1b[0m\r\n`);
    try {
      await invoke("restart_session", {
        id, path: meta.path, title: session.title, sessionId: uuid,
        mode: session.mode, fullPaths: settings.fullPaths, command: "",
        agent: session.agent, model: resolveModelArg(session.agent, session.model),
        cols: session.term.cols, rows: session.term.rows,
      });
    } catch (_) {
      sessions.delete(id); session.term.dispose(); session.el.remove();
      failures.push(`${meta.title || meta.path} (${uuid.slice(0, 8)})`);
    }
  }
  showView("new");            // herstelde tabs in de balk, maar blijf op het startscherm
  persistSessionsToDisk();    // herschrijf zonder overgeslagen/verlopen sessies
  if (failures.length) toast(`${t("restore_failed")} ${failures.join(", ")}`, "err");
}

let toastTimer = null;
function toast(msg, kind) {
  const el = els.toast;
  if (!el) return;
  el.textContent = msg;
  el.className = "toast" + (kind ? " " + kind : "");
  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.add("hidden"), 9000);
}

async function closeSession(id) {
  const s = sessions.get(id);
  if (!s) return;
  await invoke("close_session", { id });
  s.term.dispose(); s.el.remove(); sessions.delete(id);
  persistSessionsToDisk();
  if (current === id) showView([...sessions.keys()].pop() || "new"); else renderTabs();
}

/* ============ HTML-preview ============ */
function applyLayout(s) {
  s.el.classList.toggle("split", s.previewMode === "split");
  s.el.classList.toggle("full", s.previewMode === "full");
}
function refitTerm(s) {
  if (s.previewMode === "full") return;
  requestAnimationFrame(() => { try { s.fit.fit(); } catch (_) {} invoke("resize_session", { id: s.id, cols: s.term.cols, rows: s.term.rows }); });
}
async function openPreview(id) {
  closeTabMenu();
  const s = sessions.get(id);
  if (!s) return;
  if (current !== id) showView(id);
  s.previewMode = settings.htmlView || "split";
  applyLayout(s);
  await loadHtmlList(s);
  refitTerm(s);
}
function closePreview(s) { s.previewMode = null; applyLayout(s); refitTerm(s); }
async function loadHtmlList(s) {
  const sel = s.el.querySelector(".preview-file");
  const frame = s.el.querySelector(".preview-frame");
  let files = [];
  try { files = await invoke("list_html", { dir: s.path }); } catch (_) {}
  sel.innerHTML = "";
  if (!files.length) {
    const o = document.createElement("option"); o.value = ""; o.textContent = t("preview_none"); sel.appendChild(o);
    frame.srcdoc = `<body style="font-family:sans-serif;color:#888;padding:24px">${escapeHtml(t("preview_none"))}</body>`;
    return;
  }
  for (const f of files) { const o = document.createElement("option"); o.value = f.path; o.textContent = f.name; sel.appendChild(o); }
  sel.value = files[0].path;
  await renderPreview(s, files[0].path);
}
// Forwarded to the sandboxed preview: external-link clicks (and programmatic
// postMessage from the page) are relayed to the parent, which opens them in the
// default browser. Keeps the iframe sandbox tight (no allow-same-origin needed).
const PREVIEW_BRIDGE = `<script>
  document.addEventListener('click', function (ev) {
    var a = ev.target && ev.target.closest && ev.target.closest('a[href]');
    if (!a) return;
    var href = a.href || a.getAttribute('href');
    if (/^https?:\\/\\//i.test(href)) {
      ev.preventDefault();
      // Be authoritative: stop the page's own click handlers (e.g. a dashboard
      // openExternal() that would otherwise show a 'copy/paste' fallback).
      ev.stopImmediatePropagation();
      parent.postMessage({ type: 'taurus-open-external', url: href }, '*');
    }
  }, true);
<\/script>`;
async function renderPreview(s, path) {
  if (!path) return;
  const frame = s.el.querySelector(".preview-frame");
  try { frame.srcdoc = PREVIEW_BRIDGE + await invoke("read_file", { path }); }
  catch (e) { frame.srcdoc = `<body style="font-family:sans-serif;color:#c66;padding:24px">${escapeHtml(String(e))}</body>`; }
}
// Open de preview op een specifiek bestand (klik op een pad in de terminal).
async function openPreviewFile(s, rawPath) {
  let p = String(rawPath).trim().replace(/[)\].,;:'"]+$/, "");
  if (!/^([A-Za-z]:[\\/]|\\\\)/.test(p)) {
    p = s.path.replace(/[\\/]+$/, "") + "\\" + p.replace(/^[.][\\/]/, "").replace(/\//g, "\\");
  }
  if (current !== s.id) showView(s.id);
  s.previewMode = settings.htmlView || "split";
  applyLayout(s);
  await loadHtmlList(s);
  const sel = s.el.querySelector(".preview-file");
  const hit = [...sel.options].find((o) => o.value.toLowerCase() === p.toLowerCase());
  if (hit) sel.value = hit.value;
  await renderPreview(s, p);
  refitTerm(s);
}

/* ============ status uit output ============ */
function stripAnsi(s) {
  return s
    .replace(/\x1b\][^\x07\x1b]*(\x07|\x1b\\)/g, "")
    .replace(/\x1b[@-Z\\-_]/g, "")
    .replace(/\x1b\[[0-9;?]*[ -\/]*[@-~]/g, "")
    .replace(/\x1b[()][AB0]/g, "")
    .replace(/[\x00-\x08\x0b-\x1f]/g, " ");
}
function lastSpinnerVerb(buf) {
  const re = /[✶✻✽✳✢✦✧⋆∗*·]\s*([A-Za-z]{3,})(?:…|\.\.\.)/g;
  let m, last = null;
  while ((m = re.exec(buf)) !== null) last = m[1];
  return last;
}

/* ============ PTY-events ============ */
listen("pty-output", (event) => {
  const [sid, bytes] = event.payload;
  const s = sessions.get(sid);
  if (!s) return;
  const u8 = new Uint8Array(bytes);
  s.term.write(u8);
  let render = false;
  if (!s.exited) {
    s.buf = (s.buf + stripAnsi(s.decoder.decode(u8, { stream: true }))).slice(-600);
    const verb = lastSpinnerVerb(s.buf);
    if (verb) {
      s.lastSpin = Date.now();
      // Claude is (weer) bezig -> geen flash; markeer werkend.
      if (!s.working || s.awaiting) { s.working = true; s.awaiting = false; render = true; }
      if (settings.tabStatus && s.status !== verb) { s.status = verb; render = true; }
    }
  }
  if (render) renderTabs();
});
listen("pty-exit", (event) => {
  const s = sessions.get(event.payload);
  if (s) { s.exited = true; s.working = false; s.awaiting = false; s.status = null; s.term.write(`\r\n\x1b[2m${t("ended")}\x1b[0m\r\n`); renderTabs(); }
});
setInterval(() => {
  let changed = false; const now = Date.now();
  for (const s of sessions.values()) {
    // Spinner ~1,5s niet gezien -> beurt klaar. Wacht op input = flash (mits niet actief).
    if (s.working && now - s.lastSpin > 1500) {
      s.working = false; s.status = null;
      if (s.id !== current && !s.exited) s.awaiting = true;
      changed = true;
    }
  }
  if (changed) renderTabs();
}, 1000);

/* ============ herstart + rechtsklik-menu ============ */
async function restartSession(id) {
  const s = sessions.get(id);
  if (!s) return;
  closeTabMenu();
  s.term.reset();
  s.term.write(`\x1b[2m[${t("restarting")} ${s.uuid.slice(0, 8)}…]\x1b[0m\r\n`);
  s.exited = false; s.working = false; s.awaiting = false; s.status = null; s.buf = ""; s.decoder = new TextDecoder("utf-8");
  if (current !== id) showView(id); else renderTabs();
  try {
    await invoke("restart_session", { id, path: s.path, title: s.title, sessionId: s.uuid, mode: s.mode || "default", fullPaths: settings.fullPaths, command: s.command || "", agent: s.agent || "claude", model: resolveModelArg(s.agent || "claude", s.model || ""), cols: s.term.cols, rows: s.term.rows });
  } catch (e) { s.term.write(`\r\n\x1b[31m[${t("restart_failed")}: ${e}]\x1b[0m\r\n`); }
}

let tabMenuEl = null;
function closeTabMenu() { if (tabMenuEl) { tabMenuEl.remove(); tabMenuEl = null; } }
function openTabMenu(x, y, id) {
  closeTabMenu();
  const s = sessions.get(id);
  if (!s) return;
  const m = document.createElement("div");
  m.className = "ctx-menu";
  m.innerHTML = `
    <div class="ctx-item" data-act="restart">${t("ctx_restart")}</div>
    <div class="ctx-item" data-act="preview">${t("ctx_preview")}</div>
    <div class="ctx-item" data-act="explorer">${t("ctx_explorer")}</div>
    <div class="ctx-item" data-act="close">${t("ctx_close")}</div>`;
  m.style.left = x + "px"; m.style.top = y + "px";
  m.querySelector('[data-act="restart"]').addEventListener("click", () => restartSession(id));
  m.querySelector('[data-act="preview"]').addEventListener("click", () => openPreview(id));
  m.querySelector('[data-act="explorer"]').addEventListener("click", () => { closeTabMenu(); invoke("open_folder", { path: s.path }).catch(() => {}); });
  m.querySelector('[data-act="close"]').addEventListener("click", () => { closeTabMenu(); closeSession(id); });
  document.body.appendChild(m);
  const r = m.getBoundingClientRect();
  if (r.right > window.innerWidth) m.style.left = (window.innerWidth - r.width - 6) + "px";
  if (r.bottom > window.innerHeight) m.style.top = (window.innerHeight - r.height - 6) + "px";
  tabMenuEl = m;
}
document.addEventListener("click", closeTabMenu);

/* ============ lettergrootte ============ */
function applyFontToTerms() {
  for (const s of sessions.values()) { s.term.options.fontSize = settings.fontSize; try { s.fit.fit(); } catch (_) {} }
  resizeActive();
}
function changeFont(delta) { settings.fontSize = Math.min(28, Math.max(8, settings.fontSize + delta)); saveSettings(); applyFontToTerms(); }
function resizeActive() { if (current === "new") return; const s = sessions.get(current); if (s) refitTerm(s); }

/* ============ zoeken ============ */
function openSearch() {
  if (current === "new") return;
  const s = sessions.get(current);
  if (!s || !s.search) return;
  els.searchbar.classList.remove("hidden"); els.searchInput.focus(); els.searchInput.select();
}
function closeSearch() {
  els.searchbar.classList.add("hidden");
  const s = sessions.get(current);
  if (s && s.search) { try { s.search.clearDecorations(); } catch (_) {} }
}
function doSearch(dir) {
  const s = sessions.get(current);
  if (!s || !s.search) return;
  const q = els.searchInput.value; if (!q) return;
  if (dir < 0) s.search.findPrevious(q); else s.search.findNext(q);
}

/* ============ instellingen ============ */
function openSettings() {
  els.setLang.value = settings.lang;
  els.setFont.value = settings.fontSize;
  els.setScroll.value = settings.scrollback;
  els.setCursor.checked = settings.cursorBlink;
  els.htmlSplit.checked = settings.htmlView === "split";
  els.htmlFull.checked = settings.htmlView === "full";
  els.setCopy.checked = settings.copyOnSelect;
  els.setPaste.checked = settings.pasteOnRightClick;
  els.setCtrl.checked = settings.ctrlShiftCV;
  els.setLinks.checked = settings.webLinks;
  els.setSearch.checked = settings.search;
  els.setTabs.checked = settings.tabShortcuts;
  els.setTabStatus.checked = settings.tabStatus;
  els.setFullPaths.checked = settings.fullPaths;
  els.setPersist.checked = settings.persistSessions;
  // Toon de effectieve skin: expliciete keuze, anders de branding-default.
  els.setSkin.value = settings.skin || brandingSkin || "default";
  els.settingsModal.classList.remove("hidden");
}
function saveSettingsFromForm() {
  const langChanged = settings.lang !== els.setLang.value;
  settings.lang = els.setLang.value;
  settings.fontSize = Math.min(28, Math.max(8, parseInt(els.setFont.value) || 13));
  settings.scrollback = Math.min(100000, Math.max(500, parseInt(els.setScroll.value) || 8000));
  settings.cursorBlink = els.setCursor.checked;
  settings.htmlView = els.htmlFull.checked ? "full" : "split";
  settings.copyOnSelect = els.setCopy.checked;
  settings.pasteOnRightClick = els.setPaste.checked;
  settings.ctrlShiftCV = els.setCtrl.checked;
  settings.webLinks = els.setLinks.checked;
  settings.search = els.setSearch.checked;
  settings.tabShortcuts = els.setTabs.checked;
  settings.tabStatus = els.setTabStatus.checked;
  settings.fullPaths = els.setFullPaths.checked;
  settings.persistSessions = els.setPersist.checked;
  settings.skin = els.setSkin.value;
  applySkin(settings.skin);
  saveSettings();
  persistSessionsToDisk();
  for (const s of sessions.values()) {
    s.term.options.fontSize = settings.fontSize;
    s.term.options.cursorBlink = settings.cursorBlink;
    s.term.options.scrollback = settings.scrollback;
    try { s.fit.fit(); } catch (_) {}
  }
  resizeActive();
  if (langChanged) {
    applyI18n(); renderProjects(); renderTabs();
    if (selected) { els.locBadge.textContent = locText(selected.path); els.locBadge.className = "loc-badge " + locClass(selected.path); fillModeSelect(els.modeInput, els.agentInput.value, els.modeInput.value); }
  }
  els.settingsModal.classList.add("hidden");
}

/* ============ project-editor ============ */
let editRows = [];
function slugify(s) { return (s || "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "project"; }
function blankRow() { return { id: "", label: "", path: "", title: "", task: "", accent: "#7c9cff", mode: "default", agent: "claude", model: "", command: "" }; }
function openEditor() {
  editRows = projects.map((p) => ({ ...p }));
  if (editRows.length === 0) editRows.push(blankRow());
  renderEditor();
  els.editorStatus.textContent = "";
  els.editorModal.classList.remove("hidden");
}
function renderEditor() {
  els.editorRows.innerHTML = "";
  editRows.forEach((r, i) => {
    const row = document.createElement("div");
    row.className = "erow";
    row.innerHTML = `
      <input class="e-color" type="color" value="${r.accent || "#7c9cff"}" />
      <div class="e-fields">
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_button"))}</span>
          <input class="e-label" type="text" placeholder="${escapeHtml(t("ph_label"))}" value="${escapeHtml(r.label)}" /></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_workdir"))}</span>
          <div class="e-pathrow"><input class="e-path" type="text" placeholder="${escapeHtml(t("ph_path"))}" value="${escapeHtml(r.path)}" /><button class="e-browse">📁</button></div></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_tabtitle"))}</span>
          <input class="e-title" type="text" placeholder="${escapeHtml(t("ph_title"))}" value="${escapeHtml(r.title || "")}" /></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_task"))}</span>
          <input class="e-task" type="text" placeholder="${escapeHtml(t("ph_task"))}" value="${escapeHtml(r.task || "")}" /></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_agent"))}</span>
          <select class="e-agent">
            <option value="claude"${(r.agent || "claude") === "claude" ? " selected" : ""}>${escapeHtml(t("agent_claude"))}</option>
            <option value="agy"${r.agent === "agy" ? " selected" : ""}>${escapeHtml(t("agent_agy"))}</option>
          </select></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_model"))}</span>
          <input class="e-model" type="text" list="dl-emodel-${i}" autocomplete="off" placeholder="${escapeHtml(t("model_ph"))}" value="${escapeHtml(r.model || "")}" />
          <datalist id="dl-emodel-${i}">${(MODEL_SUGGESTIONS[r.agent] || MODEL_SUGGESTIONS.claude).map((m) => `<option value="${escapeHtml(m)}"></option>`).join("")}</datalist></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("launch_mode"))}</span>
          <select class="e-mode">${modesFor(r.agent).map((o) => `<option value="${o.value}"${clampMode(r.agent, r.mode || "default") === o.value ? " selected" : ""}>${escapeHtml(t(o.key))}</option>`).join("")}</select></div>
      </div>
      <button class="e-del">🗑</button>`;
    row.querySelector(".e-color").addEventListener("input", (e) => (editRows[i].accent = e.target.value));
    row.querySelector(".e-label").addEventListener("input", (e) => (editRows[i].label = e.target.value));
    row.querySelector(".e-path").addEventListener("input", (e) => (editRows[i].path = e.target.value));
    row.querySelector(".e-title").addEventListener("input", (e) => (editRows[i].title = e.target.value));
    row.querySelector(".e-task").addEventListener("input", (e) => (editRows[i].task = e.target.value));
    row.querySelector(".e-mode").addEventListener("change", (e) => (editRows[i].mode = e.target.value));
    row.querySelector(".e-model").addEventListener("input", (e) => (editRows[i].model = e.target.value));
    // Agent wisselen herrendert de rij zodat model-suggesties EN modus-opties
    // meeveranderen; de modus wordt geclampt (claude "plan" bestaat niet voor
    // agy) en het model gewist (een model van de vorige agent is niet geldig).
    row.querySelector(".e-agent").addEventListener("change", (e) => {
      editRows[i].agent = e.target.value;
      editRows[i].mode = clampMode(e.target.value, editRows[i].mode || "default");
      editRows[i].model = "";
      renderEditor();
    });
    row.querySelector(".e-browse").addEventListener("click", async () => { const dir = await invoke("pick_folder"); if (dir) { editRows[i].path = dir; renderEditor(); } });
    row.querySelector(".e-del").addEventListener("click", () => { editRows.splice(i, 1); renderEditor(); });
    els.editorRows.appendChild(row);
  });
}
async function saveEditor() {
  const cleaned = editRows
    .filter((r) => r.label.trim() && r.path.trim())
    .map((r) => ({ id: r.id || slugify(r.label), label: r.label.trim(), path: r.path.trim(), title: (r.title || "").trim(), task: (r.task || "").trim(), accent: r.accent || "#7c9cff", mode: r.mode || "default", agent: AGENTS.includes(r.agent) ? r.agent : "claude", model: (r.model || "").trim(), command: r.command || "" }));
  if (cleaned.length === 0) { els.editorStatus.textContent = t("err_need_project"); els.editorStatus.className = "status-msg err"; return; }
  try { await invoke("save_projects", { projects: cleaned }); projects = cleaned; renderProjects(); els.editorModal.classList.add("hidden"); }
  catch (e) { els.editorStatus.textContent = "✗ " + e; els.editorStatus.className = "status-msg err"; }
}

/* ============ sneltoetsen ============ */
function tabIds() { return [...sessions.keys()]; }
function cycleTab(dir) {
  const ids = tabIds();
  if (ids.length === 0) { showView("new"); return; }
  const order = [...ids, "new"];
  let idx = order.indexOf(current);
  idx = (idx + dir + order.length) % order.length;
  showView(order[idx]);
}
function selectNthTab(n) { const ids = tabIds(); if (n >= 1 && n <= ids.length) showView(ids[n - 1]); }

document.addEventListener("keydown", (e) => {
  // Blokkeer het herladen van de webview: F5 / Ctrl+F5 / Ctrl+R / Ctrl+Shift+R.
  // Een reload wist de sessies-Map (alle tabs verdwijnen uit beeld) en laat de
  // Claude-processen als onbereikbare zombies in de Rust-backend achter -- er is
  // geen reattach. Daarom helemaal voorkomen i.p.v. proberen te herstellen.
  if (e.key === "F5" || ((e.ctrlKey || e.metaKey) && (e.key === "r" || e.key === "R"))) {
    e.preventDefault();
    return;
  }
  if (modalOpen()) { if (e.key === "Escape") { els.settingsModal.classList.add("hidden"); els.editorModal.classList.add("hidden"); } return; }
  if (!els.searchbar.classList.contains("hidden") && e.key === "Escape") { e.preventDefault(); closeSearch(); return; }
  const ctrl = e.ctrlKey && !e.altKey;
  if (ctrl && (e.key === "=" || e.key === "+")) { e.preventDefault(); changeFont(1); return; }
  if (ctrl && e.key === "-") { e.preventDefault(); changeFont(-1); return; }
  if (ctrl && e.key === "0") { e.preventDefault(); settings.fontSize = DEFAULT_SETTINGS.fontSize; saveSettings(); applyFontToTerms(); return; }
  if (settings.ctrlShiftCV && ctrl && e.shiftKey && (e.key === "C" || e.key === "c")) {
    const s = sessions.get(current);
    if (s) { const sel = s.term.getSelection(); if (sel) { copyToClipboard(sel); e.preventDefault(); } }
    return;
  }
  if (settings.ctrlShiftCV && ctrl && e.shiftKey && (e.key === "V" || e.key === "v")) {
    e.preventDefault();
    const s = sessions.get(current);
    if (s) navigator.clipboard.readText().then((txt) => txt && invoke("write_session", { id: s.id, data: txt })).catch(() => {});
    return;
  }
  if (settings.search && ctrl && e.shiftKey && (e.key === "F" || e.key === "f")) { e.preventDefault(); openSearch(); return; }
  if (settings.tabShortcuts) {
    if (ctrl && e.key === "Tab") { e.preventDefault(); cycleTab(e.shiftKey ? -1 : 1); return; }
    if (ctrl && (e.key === "t" || e.key === "T")) { e.preventDefault(); resetLaunchForm(); showView("new"); return; }
    if (ctrl && (e.key === "w" || e.key === "W")) { e.preventDefault(); if (current !== "new") closeSession(current); return; }
    if (ctrl && /^[1-9]$/.test(e.key)) { e.preventDefault(); selectNthTab(parseInt(e.key)); return; }
  }
}, true);

/* ============ init ============ */
window.addEventListener("DOMContentLoaded", () => {
  Object.assign(els, {
    list: document.querySelector("#project-list"),
    tabbar: document.querySelector("#tabbar"),
    launchView: document.querySelector("#launch-view"),
    terminals: document.querySelector("#terminals"),
    formEmpty: document.querySelector("#form-empty"),
    form: document.querySelector("#launch-form"),
    formTitle: document.querySelector("#form-title"),
    pathValue: document.querySelector("#path-value"),
    locBadge: document.querySelector("#loc-badge"),
    warn: document.querySelector("#path-warn"),
    claudeWarn: document.querySelector("#claude-warn"),
    titleInput: document.querySelector("#title-input"),
    taskInput: document.querySelector("#task-input"),
    status: document.querySelector("#status-msg"),
    searchbar: document.querySelector("#searchbar"),
    searchInput: document.querySelector("#search-input"),
    settingsModal: document.querySelector("#settings-modal"),
    setLang: document.querySelector("#set-lang"),
    setFont: document.querySelector("#set-fontsize"),
    setScroll: document.querySelector("#set-scrollback"),
    setCursor: document.querySelector("#set-cursorblink"),
    htmlSplit: document.querySelector("#set-html-split"),
    htmlFull: document.querySelector("#set-html-full"),
    setCopy: document.querySelector("#set-copyselect"),
    setPaste: document.querySelector("#set-pasteright"),
    setCtrl: document.querySelector("#set-ctrlshift"),
    setLinks: document.querySelector("#set-weblinks"),
    setSearch: document.querySelector("#set-search"),
    setTabs: document.querySelector("#set-tabshortcuts"),
    setTabStatus: document.querySelector("#set-tabstatus"),
    setFullPaths: document.querySelector("#set-fullpaths"),
    setPersist: document.querySelector("#set-persist"),
    setSkin: document.querySelector("#set-skin"),
    toast: document.querySelector("#toast"),
    modeInput: document.querySelector("#mode-input"),
    agentInput: document.querySelector("#agent-input"),
    modelInput: document.querySelector("#model-input"),
    modelSuggestions: document.querySelector("#model-suggestions"),
    editorModal: document.querySelector("#editor-modal"),
    editorRows: document.querySelector("#editor-rows"),
    editorStatus: document.querySelector("#editor-status"),
    appVersion: document.querySelector("#app-version"),
  });

  document.querySelector("#launch-btn").addEventListener("click", startSession);
  // Andere agent op het startformulier -> model-keuze EN modus-opties mee.
  // Het modelveld wissen: een model van de vorige agent is hier niet geldig en
  // zou de datalist-suggesties wegfilteren (leeg = de default van de agent).
  els.agentInput.addEventListener("change", () => {
    els.modelInput.value = "";
    fillModelDatalist(els.modelSuggestions, els.agentInput.value);
    fillModeSelect(els.modeInput, els.agentInput.value, els.modeInput.value);
  });
  document.querySelector("#browse-btn").addEventListener("click", browseFolder);
  document.querySelector("#reload-btn").addEventListener("click", loadProjects);
  document.querySelector("#settings-btn").addEventListener("click", openSettings);
  document.querySelector("#edit-btn").addEventListener("click", openEditor);
  document.querySelector("#settings-cancel").addEventListener("click", () => els.settingsModal.classList.add("hidden"));
  document.querySelector("#settings-save").addEventListener("click", saveSettingsFromForm);
  document.querySelector("#editor-cancel").addEventListener("click", () => els.editorModal.classList.add("hidden"));
  document.querySelector("#editor-save").addEventListener("click", saveEditor);
  document.querySelector("#editor-add").addEventListener("click", () => { editRows.push(blankRow()); renderEditor(); });
  document.querySelector("#search-next").addEventListener("click", () => doSearch(1));
  document.querySelector("#search-prev").addEventListener("click", () => doSearch(-1));
  document.querySelector("#search-close").addEventListener("click", closeSearch);
  els.searchInput.addEventListener("keydown", (e) => { if (e.key === "Enter") { e.preventDefault(); doSearch(e.shiftKey ? -1 : 1); } });
  window.addEventListener("resize", resizeActive);

  // Open external links forwarded from the sandboxed HTML preview (see renderPreview).
  // The preview iframe has an opaque origin, so Tauri IPC is refused inside it; we open
  // the link here in the parent window, which has a valid app origin and working IPC.
  window.addEventListener("message", (e) => {
    const d = e.data;
    if (d && d.type === "taurus-open-external" && typeof d.url === "string"
        && /^https?:\/\//i.test(d.url)) {
      window.__TAURI__.opener.openUrl(d.url).catch(() => {});
    }
  });

  loadSettings();
  applyI18n();
  // Expliciete skin-keuze meteen toepassen (geen flits); applyBranding() vult
  // daarna eventueel de branding-default in als er geen keuze is gemaakt.
  if (settings.skin) applySkin(settings.skin);
  applyBranding();
  renderTabs();
  loadProjects();
  restoreSessions();

  // Toon de app-versie discreet onderin de sidebar.
  invoke("app_version").then((v) => { if (v) els.appVersion.textContent = "v" + v; }).catch(() => {});
});
