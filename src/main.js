const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

/* ============ i18n ============ */
const I18N = {
  nl: {
    brand_sub: "Agent Launcher", projects: "Agents",
    foot_projects: "✎ Agents", foot_settings: "⚙ Instellingen", foot_reload: "⟳ Herlaad",
    empty_pick: "Kies of maak links een agent om je werkproces te starten.",
    empty_oneoff: "…of blader hieronder naar een eenmalige agent.",
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
    launch_model: "Model", model_ph: "standaard",
    model_hint: "Leeg = de standaard van de agent. Een alias volgt vanzelf het nieuwste model; een exact model-ID mag ook.",
    model_fable: "nieuwste Fable", model_opus: "nieuwste Opus", model_sonnet: "nieuwste Sonnet",
    model_haiku: "nieuwste Haiku", model_opusplan: "Opus in plan-modus, daarna Sonnet",
    cap_agent: "Agent", cap_model: "Model (leeg = standaard)",
    grp_comfort: "Terminal-comfort", comfort_hint: "(per voorkeur aan/uit)",
    c_copy: "Selectie kopieert automatisch", c_paste: "Rechtermuisklik plakt", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Klikbare links", c_links_new: "(nieuwe sessies)", c_search: "Zoeken in scrollback — Ctrl+Shift+F",
    c_tabs: "Tab-sneltoetsen (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude-status op de tab (✶ Orbiting…)",
    c_mouse: "Agent mag de muis gebruiken (anders selecteert/scrolt de muis lokaal)",
    cancel: "Annuleer", save: "Opslaan",
    manage_projects: "Agents beheren", add_agent: "＋ Agent toevoegen",
    cap_button: "▸ Knop in het linkermenu", cap_workdir: "Werkmap (lokaal C: of netwerk X:)",
    cap_tabtitle: "⎯ Tabtitel bovenin (standaard, per sessie aanpasbaar)", cap_task: "Taak — wordt direct meegestuurd (optioneel)",
    ph_label: "bijv. DVZA", ph_path: "C:\\… of X:\\…", ph_title: "bijv. DVZA-cert", ph_task: "laat leeg voor een lege sessie",
    search_ph: "Zoeken…",
    ctx_restart: "↻ Herstart (resume gesprek)", ctx_preview: "👁 HTML-preview", ctx_explorer: "📂 Open map in Verkenner", ctx_close: "✕ Sluiten",
    preview_none: "(geen .html/.md in de werkmap)", preview_refresh: "Vernieuwen", preview_mode: "Split / Volledig", preview_close: "Preview sluiten", preview_toobig: "Bestand te groot om te previewen.",
    loc_local: "LOKAAL", loc_net: "NETWERK", loc_unknown: "ONBEKEND",
    ended: "[sessie beëindigd — rechtsklik tab voor herstart, of sluit]",
    restarting: "herstarten — resume", restart_failed: "herstart mislukt",
    grp_sessions: "Sessies", set_persist: "Sessies onthouden en bij opstarten hervatten",
    tab_general: "Algemeen", tab_theme: "Thema", tab_html: "HTML-preview", tab_terminal: "Terminal", tab_voice: "Spraak",
    help_default: "Beweeg over een instelling voor uitleg.",
    help_lang: "Taal van de interface: Nederlands of Engels. Wijziging is zichtbaar na Opslaan.",
    help_fontsize: "Lettergrootte van de terminaltekst (punten).\nVoorbeeld: 13 = standaard; 15-16 leest prettiger op een groot scherm.",
    help_scrollback: "Aantal regels terminaluitvoer dat bewaard blijft om terug te scrollen.\nVoorbeeld: 8000. Hoger = meer terug te lezen, maar meer geheugen per sessie.",
    help_cursor: "Laat de tekstcursor in de terminal knipperen in plaats van stil te staan.",
    help_persist: "Onthoudt open sessies en hervat ze bij het opstarten (--resume), zodat je verdergaat waar je gebleven was. Uit = elke start begint schoon.",
    help_skin: "Visueel thema (skin) voor de hele app.\nVoorbeeld: 'Terminal (CRT)' = retro groen; 'Nord'/'Dracula' = donkere kleurschema's; 'Solarized Light' = licht.",
    help_html_split: "Toont de HTML-preview naast de terminal (gesplitst), zodat je code en resultaat tegelijk ziet.",
    help_html_full: "Toont de HTML-preview op het volledige venster en verbergt de terminal zolang de preview open is.",
    help_fullpaths: "Vraagt Claude volledige bestandspaden te printen, zodat ze klikbaar worden.\nVoorbeeld: C:\\project\\index.html i.p.v. alleen index.html.",
    help_copyselect: "Zodra je tekst in de terminal selecteert, gaat die automatisch naar het klembord - geen Ctrl+C nodig.",
    help_pasteright: "Een rechtermuisklik in de terminal plakt de klembordinhoud.",
    help_agentmouse: "Wie krijgt de muis in de terminal?\nUit: slepen selecteert tekst lokaal, rechtsklik plakt.\nAan: de agent (bijv. een TUI-menu) ontvangt klikken en scrollen. Geldt voor nieuwe sessies.",
    help_ctrlshift: "Ctrl+Shift+C kopieert en Ctrl+Shift+V plakt in de terminal. De Shift-variant houdt gewone Ctrl+C vrij om een programma te onderbreken.",
    help_weblinks: "Maakt URL's in de uitvoer klikbaar; ze openen in je standaardbrowser. Geldt voor nieuwe sessies.",
    help_search: "Zoeken in de scrollback met Ctrl+Shift+F. Geldt voor nieuwe sessies.",
    help_tabshortcuts: "Sneltoetsen voor tabs: Ctrl+Tab wisselt, Ctrl+1..9 springt naar een tab, Ctrl+T opent een nieuwe, Ctrl+W sluit de huidige.",
    help_tabstatus: "Toont Claude's huidige bezigheid live op het tabblad.\nVoorbeeld: '✶ Orbiting…' terwijl Claude werkt; een stille groene stip = klaar/wachtend.",
    grp_theme: "Thema", set_skin: "Skin", skin_hint: "Of zet een vaste default in branding.json (zie README).",
    skin_default: "Standaard (donker)", skin_retromac: "Retro Mac", skin_aqua: "macOS Aqua",
    skin_retrowin: "Retro Windows", skin_winxp: "Windows XP", skin_terminal: "Terminal (CRT)",
    skin_nord: "Nord", skin_dracula: "Dracula", skin_solarized: "Solarized Light", skin_catppuccin: "Catppuccin",
    restore_failed: "Hervatten mislukt voor:",
    copy_failed: "✗ Kopiëren naar klembord mislukt:",
    err_need_project: "✗ Minstens één agent met naam én pad nodig.",
    dropper: "DROPZONE", dropper_hint: "Sleep bestand of map hierheen",
    dz_move: "Verplaats", dz_copy: "Kopieer", dz_prompt: "Alleen pad", dz_paste: "Plak object",
    dropper_need_project: "Kies eerst een project of open een sessie",
    dropper_no_session: "Geen actieve terminal om het pad in te plaatsen",
    dropper_save_failed: "✗ Opslaan in input-map mislukt:",
    dropper_paste_failed: "✗ Plakken van object mislukt:",
    reload: "Herlaad", new_project: "Nieuwe agent", add_file: "Bestand toevoegen",
    edit: "Bewerken", delete: "Verwijderen", confirm_delete: "Verwijderen?", yes: "Ja", no: "Nee",
    grp_voice: "Spraak", tts_enable: "Spreek uit wanneer een agent klaar is",
    tts_voice: "Stem", tts_rate: "Spreeksnelheid", tts_test: "Test", rate_slow: "langzaam", rate_fast: "snel",
    voice_natural: "Windows-stemmen", voice_classic: "Klassiek (SAPI)",
    voice_install_hint: "Meer stemmen of talen nodig? Voeg ze toe via Windows-instellingen → Tijd en taal → Spraak (of taal toevoegen met Text-to-speech).",
    tts_ready: "{title} is klaar",
    ctx_speak: "🔊 Selectie uitspreken",
    stt_head: "Spraak naar tekst — F9 inhouden (of klik 🎙)",
    stt_model: "Model", stt_download: "Download",
    stt_downloading: "Bezig met downloaden… (zie stt\\download.log)",
    stt_ready_lbl: "geïnstalleerd", stt_missing_lbl: "niet geïnstalleerd",
    stt_autosend: "Transcript direct versturen (Enter)",
    stt_registry: "Modellenbibliotheek-URL", stt_refresh: "Vernieuw lijst",
    stt_failed: "✗ Transcriptie mislukt:", stt_rec: "● Opname… (laat F9 los = stop)",
    rec_idle: "Klik of F9 = dicteren", rec_listening: "● Luisteren…", rec_transcribing: "Transcriberen…",
  },
  en: {
    brand_sub: "Agent Launcher", projects: "Agents",
    foot_projects: "✎ Agents", foot_settings: "⚙ Settings", foot_reload: "⟳ Reload",
    empty_pick: "Pick or create an agent on the left to start your work process.",
    empty_oneoff: "…or browse for a one-time agent below.",
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
    launch_model: "Model", model_ph: "default",
    model_hint: "Empty = the agent's default. An alias always follows the newest model; an exact model ID works too.",
    model_fable: "newest Fable", model_opus: "newest Opus", model_sonnet: "newest Sonnet",
    model_haiku: "newest Haiku", model_opusplan: "Opus in plan mode, Sonnet after",
    cap_agent: "Agent", cap_model: "Model (empty = default)",
    grp_comfort: "Terminal comfort", comfort_hint: "(toggle to taste)",
    c_copy: "Selection copies automatically", c_paste: "Right-click pastes", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Clickable links", c_links_new: "(new sessions)", c_search: "Search scrollback — Ctrl+Shift+F",
    c_tabs: "Tab shortcuts (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude status on the tab (✶ Orbiting…)",
    c_mouse: "Let the agent use the mouse (otherwise the mouse selects/scrolls locally)",
    cancel: "Cancel", save: "Save",
    manage_projects: "Manage agents", add_agent: "＋ Add agent",
    cap_button: "▸ Button in the left menu", cap_workdir: "Working folder (local C: or network X:)",
    cap_tabtitle: "⎯ Tab title (default, editable per session)", cap_task: "Task — sent immediately (optional)",
    ph_label: "e.g. DVZA", ph_path: "C:\\… or X:\\…", ph_title: "e.g. DVZA-cert", ph_task: "leave empty for a blank session",
    search_ph: "Search…",
    ctx_restart: "↻ Restart (resume conversation)", ctx_preview: "👁 HTML preview", ctx_explorer: "📂 Open folder in Explorer", ctx_close: "✕ Close",
    preview_none: "(no .html/.md in the working folder)", preview_refresh: "Refresh", preview_mode: "Split / Full", preview_close: "Close preview", preview_toobig: "File too large to preview.",
    loc_local: "LOCAL", loc_net: "NETWORK", loc_unknown: "UNKNOWN",
    ended: "[session ended — right-click tab to restart, or close]",
    restarting: "restarting — resume", restart_failed: "restart failed",
    grp_sessions: "Sessions", set_persist: "Remember sessions and resume on startup",
    tab_general: "General", tab_theme: "Theme", tab_html: "HTML preview", tab_terminal: "Terminal", tab_voice: "Voice",
    help_default: "Hover a setting for an explanation.",
    help_lang: "Interface language: Dutch or English. Applies right after you Save.",
    help_fontsize: "Font size of the terminal text (points).\nExample: 13 = default; 15-16 reads better on a large display.",
    help_scrollback: "Number of terminal output lines kept for scrolling back.\nExample: 8000. Higher = more history, but more memory per session.",
    help_cursor: "Make the terminal text cursor blink instead of staying solid.",
    help_persist: "Remembers open sessions and resumes them on startup (--resume) so you continue where you left off. Off = every launch starts clean.",
    help_skin: "Visual theme (skin) for the whole app.\nExample: 'Terminal (CRT)' = retro green; 'Nord'/'Dracula' = dark colour schemes; 'Solarized Light' = light.",
    help_html_split: "Shows the HTML preview beside the terminal (split), so you see code and result at once.",
    help_html_full: "Shows the HTML preview full-window, hiding the terminal while the preview is open.",
    help_fullpaths: "Asks Claude to print full file paths so they become clickable.\nExample: C:\\project\\index.html instead of just index.html.",
    help_copyselect: "As soon as you select text in the terminal it goes to the clipboard automatically - no Ctrl+C needed.",
    help_pasteright: "A right-click in the terminal pastes the clipboard contents.",
    help_agentmouse: "Who gets the mouse in the terminal?\nOff: dragging selects text locally, right-click pastes.\nOn: the agent (e.g. a TUI menu) receives clicks and scrolling. Applies to new sessions.",
    help_ctrlshift: "Ctrl+Shift+C copies and Ctrl+Shift+V pastes in the terminal. The Shift variant keeps plain Ctrl+C free to interrupt a program.",
    help_weblinks: "Makes URLs in the output clickable; they open in your default browser. Applies to new sessions.",
    help_search: "Search the scrollback with Ctrl+Shift+F. Applies to new sessions.",
    help_tabshortcuts: "Tab shortcuts: Ctrl+Tab switches, Ctrl+1..9 jumps to a tab, Ctrl+T opens a new one, Ctrl+W closes the current.",
    help_tabstatus: "Shows Claude's current activity live on the tab.\nExample: '✶ Orbiting…' while Claude works; a steady green dot = done/waiting.",
    grp_theme: "Theme", set_skin: "Skin", skin_hint: "Or set a fixed default in branding.json (see README).",
    skin_default: "Default (dark)", skin_retromac: "Retro Mac", skin_aqua: "macOS Aqua",
    skin_retrowin: "Retro Windows", skin_winxp: "Windows XP", skin_terminal: "Terminal (CRT)",
    skin_nord: "Nord", skin_dracula: "Dracula", skin_solarized: "Solarized Light", skin_catppuccin: "Catppuccin",
    restore_failed: "Could not resume:",
    copy_failed: "✗ Copy to clipboard failed:",
    err_need_project: "✗ Need at least one agent with a name and a path.",
    dropper: "DROPZONE", dropper_hint: "Drag a file or folder here",
    dz_move: "Move", dz_copy: "Copy", dz_prompt: "Path only", dz_paste: "Paste object",
    dropper_need_project: "Pick a project or open a session first",
    dropper_no_session: "No active terminal to insert the path into",
    dropper_save_failed: "✗ Could not save into the input folder:",
    dropper_paste_failed: "✗ Could not paste object:",
    reload: "Reload", new_project: "New agent", add_file: "Add file",
    edit: "Edit", delete: "Delete", confirm_delete: "Delete?", yes: "Yes", no: "No",
    grp_voice: "Voice", tts_enable: "Speak when an agent is ready",
    tts_voice: "Voice", tts_rate: "Speaking rate", tts_test: "Test", rate_slow: "slow", rate_fast: "fast",
    voice_natural: "Windows voices", voice_classic: "Classic (SAPI)",
    voice_install_hint: "Need more voices or languages? Add them via Windows Settings → Time & language → Speech (or add a language with Text-to-speech).",
    tts_ready: "{title} is ready",
    ctx_speak: "🔊 Speak selection",
    stt_head: "Speech to text — hold F9 (or click 🎙)",
    stt_model: "Model", stt_download: "Download",
    stt_downloading: "Downloading… (see stt\\download.log)",
    stt_ready_lbl: "installed", stt_missing_lbl: "not installed",
    stt_autosend: "Send transcript immediately (Enter)",
    stt_registry: "Model library URL", stt_refresh: "Refresh list",
    stt_failed: "✗ Transcription failed:", stt_rec: "● Recording… (release F9 to stop)",
    rec_idle: "Click or F9 to dictate", rec_listening: "● Listening…", rec_transcribing: "Transcribing…",
  },
};
function t(k) { return (I18N[settings.lang] || I18N.nl)[k] ?? k; }
function applyI18n() {
  const d = I18N[settings.lang] || I18N.nl;
  document.querySelectorAll("[data-i18n]").forEach((el) => { const k = el.getAttribute("data-i18n"); if (d[k] != null) el.textContent = d[k]; });
  document.querySelectorAll("[data-i18n-ph]").forEach((el) => { const k = el.getAttribute("data-i18n-ph"); if (d[k] != null) el.placeholder = d[k]; });
  document.querySelectorAll("[data-i18n-title]").forEach((el) => { const k = el.getAttribute("data-i18n-title"); if (d[k] != null) el.title = d[k]; });
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
    // Geen drop-shadow op een eigen logo (dat oogt rommelig op een lichte/eigen
    // achtergrond); de meeste merklogo's hebben hun eigen vorm/marge al.
    if (img) { img.src = b.logoDataUri; img.style.filter = "none"; }
  }
  if (b.theme && typeof b.theme === "object") {
    // Het branding-thema wordt een eigen, selecteerbare skin "brand": de vars
    // hangen onder html[data-skin="brand"] (niet :root), zodat het naast de
    // ingebouwde skins staat en niemand het per ongeluk overschrijft. Het label
    // in de Thema-lijst komt uit de lokale branding (skinName/appName) — er staat
    // dus niets merkspecifieks in de publieke code.
    // Ook ';' weren (net als bij font hieronder): anders injecteert een waarde
    // als "red; background:url(...)" extra CSS-declaraties (#78).
    const decls = Object.entries(b.theme)
      .filter(([k, v]) => /^--[\w-]+$/.test(k) && typeof v === "string" && !/[<>{};]/.test(v))
      .map(([k, v]) => `${k}: ${v};`);
    // Optioneel UI-lettertype hoort bij de merk-skin (overschrijft --ui-font).
    if (typeof b.font === "string" && b.font.trim() && !/[<>{};]/.test(b.font)) {
      decls.push(`--ui-font: ${b.font.trim()};`);
    }
    if (decls.length) {
      brandHasTheme = true;
      let st = document.getElementById("taurus-branding");
      if (!st) { st = document.createElement("style"); st.id = "taurus-branding"; document.head.appendChild(st); }
      st.textContent = `html[data-skin="brand"] { ${decls.join(" ")} }`;
      addBrandSkinOption(b.skinName || b.appName || "Custom");
    }
  }
  if (b.windowTitle) {
    document.title = b.windowTitle;
    try { window.__TAURI__.window.getCurrentWindow().setTitle(b.windowTitle); } catch (_) {}
  }
  // Branding mag het garble-effect uitzetten (garble:false).
  if (b.garble === false) brandGarble = false;
  // Effectieve skin: expliciete keuze in Instellingen wint, anders de
  // branding-default-skin, anders de "brand"-skin (als er een thema is), anders
  // gewoon Taurus.
  brandingSkin = (b.skin || "").trim();
  applySkin(settings.skin || brandingSkin || (brandHasTheme ? "brand" : "default"));
}

// Voeg de uit branding.json afgeleide "brand"-skin als keuze toe aan de
// Thema-dropdown (één keer), met een lokaal label. Geen merknaam in de code.
function addBrandSkinOption(label) {
  const sel = document.getElementById("set-skin");
  if (!sel || sel.querySelector('option[value="brand"]')) return;
  const o = document.createElement("option");
  o.value = "brand";
  o.textContent = label;
  sel.insertBefore(o, sel.options[1] || null);
}

/* ============ skins ============ */
// De gekozen skin hangt als data-skin op <html>; skins.css doet de rest.
// "default" / leeg = geen attribuut (gewoon :root). Het terminal-thema leest de
// --term-* variabelen zodat de skin ook in de xterm-terminal doorwerkt.
let brandingSkin = "";
let brandHasTheme = false;
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
  // Garble-effect: standaard alleen aan voor merk-skins; branding kan het
  // uitzetten (garble:false). GARBLE_SKINS is bewust een set zodat het later
  // triviaal naar andere skins uit te breiden is.
  garbleEnabled = GARBLE_SKINS.has(skin) && brandGarble !== false;
  // Open terminals her-thematiseren (cursor blijft de project-accent).
  for (const s of sessions.values()) {
    try { s.term.options.theme = termThemeFromCss(s.accent); } catch (_) {}
  }
}

/* ============ garble (letter-scramble op hover) ============ */
// Editorial micro-interactie: bij hover scrambelt het label kort met willekeurige
// tekens en lost dan op naar de echte tekst. Generiek, maar gated per skin.
const GARBLE_SKINS = new Set(["brand"]);
const GARBLE_SELECTOR = ".foot-btn, .project-card .pc-label, .tab-title";
const GARBLE_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#%&@*<>/\\";
let garbleEnabled = false;
let brandGarble = null; // null = default; false = branding zette het uit
const garbling = new WeakSet();
function scramble(el) {
  if (garbling.has(el)) return;
  const original = el.textContent;
  if (!original || !original.trim()) return;
  garbling.add(el);
  const len = original.length;
  let frame = 0;
  const id = setInterval(() => {
    // Tabs/kaarten worden vaak opnieuw gerenderd: als het element verdwijnt,
    // stop netjes (we schrijven anders naar een losgekoppelde node).
    if (!el.isConnected) { clearInterval(id); garbling.delete(el); return; }
    const revealed = Math.max(0, frame - 5); // eerst even volledig scrambelen
    let out = "";
    for (let i = 0; i < len; i++) {
      const ch = original[i];
      out += (i < revealed || ch === " ") ? ch : GARBLE_CHARS[(Math.random() * GARBLE_CHARS.length) | 0];
    }
    el.textContent = out;
    frame++;
    if (revealed >= len) { clearInterval(id); el.textContent = original; garbling.delete(el); }
  }, 28);
}
function wireGarble() {
  document.addEventListener("mouseover", (e) => {
    if (!garbleEnabled || !e.target || !e.target.closest) return;
    const el = e.target.closest(GARBLE_SELECTOR);
    if (el) scramble(el);
  });
}

/* ============ agents + modellen ============ */
// Welke agent-CLIs Taurus kan starten. De losse gemini-CLI is end-of-life en
// staat hier bewust niet bij; "agy" is de ondersteunde Gemini-agent.
const AGENTS = ["claude", "agy"];
// Een vastgepinde modellijst veroudert bij elke modelrelease (#92), dus pinnen
// we niets meer: claude krijgt ALIASSEN, die de CLI zelf naar het nieuwste model
// in die lijn vertaalt, en agy's lijst vragen we op bij de CLI (`agy models`).
// Zo verschijnt een nieuw model zonder dat Taurus mee hoeft te updaten.
// `claude --model` accepteert zo'n alias of een exact model-ID; vrije tekst
// blijft toegestaan, dus een pin op een specifieke versie kan nog steeds.
const CLAUDE_ALIASES = [
  { value: "fable", key: "model_fable" },
  { value: "opus", key: "model_opus" },
  { value: "sonnet", key: "model_sonnet" },
  { value: "haiku", key: "model_haiku" },
  { value: "opusplan", key: "model_opusplan" },
];
// Projecten van vóór #92 bewaarden een weergavenaam in plaats van een alias.
// Die blijven we vertalen zodat een opgeslagen agent gewoon start; nieuw kiezen
// gebeurt via de aliassen hierboven. Alleen aanvullen bij een bewaarde naam die
// anders als onbekend model naar de CLI zou gaan — niet uitbreiden met nieuwe.
const CLAUDE_LEGACY_MODELS = {
  "Claude Opus 4.8": "claude-opus-4-8",
  "Claude Sonnet 4.6": "claude-sonnet-4-6",
  "Claude Haiku 4.5": "claude-haiku-4-5",
};
// Terugvallijst voor het model-veld (datalist): gebruikt zolang de CLI zelf geen
// lijst geeft — agy niet geïnstalleerd, of een oudere agy zonder `models`. agy
// selecteert op de volledige label-string (incl. effort-suffix), dus die nemen
// we letterlijk over. Een entry mag een string zijn of {value, key} met een
// i18n-sleutel voor het label naast de waarde.
const MODEL_SUGGESTIONS = {
  claude: CLAUDE_ALIASES,
  agy: [
    "Gemini 3.5 Flash (Medium)",
    "Gemini 3.5 Flash (High)",
    "Gemini 3.5 Flash (Low)",
    "Gemini 3.1 Pro (Low)",
    "Gemini 3.1 Pro (High)",
    "GPT-OSS 120B (Medium)",
  ],
};
// Modellen die de CLI zelf opsomt, per agent gecached: het is een procesaanroep
// en de lijst verandert niet tijdens een sessie. null = geprobeerd en mislukt
// (dan blijft de terugvallijst staan en proberen we het niet elke keer opnieuw;
// ⟳ Herlaad herstart de webview en dus ook deze cache).
const liveModels = new Map();

// Welke agents hun lijst bij de CLI mogen ophalen. claude hoort hier niet thuis
// (aliassen verouderen niet) en agy staat er bewust nog NIET in: `agy models`
// geeft bij piped uitvoer slugs ("gemini-3.6-flash-high") terwijl de vaste lijst
// labels gebruikt ("Gemini 3.6 Flash (High)"), en agy accepteert een onbekend
// --model zonder foutmelding -- de verkeerde vorm zou dus stil op het
// default-model terugvallen in plaats van zichtbaar te falen. Zodra vaststaat
// welke vorm --model honoreert, is "agy" hier toevoegen genoeg (#92).
const LIVE_MODEL_AGENTS = new Set();

// Vraag de agent-CLI om zijn modellen. Geeft true als er een nieuwe lijst is,
// zodat de aanroeper de datalist opnieuw kan vullen.
async function refreshLiveModels(agent) {
  if (!LIVE_MODEL_AGENTS.has(agent) || liveModels.has(agent)) return false;
  try {
    const list = await invoke("list_agent_models", { agent });
    if (Array.isArray(list) && list.length) {
      liveModels.set(agent, list);
      return true;
    }
  } catch (_) { /* niet geïnstalleerd/timeout: terugvallijst blijft staan */ }
  liveModels.set(agent, null);
  return false;
}
function modelSuggestionsFor(agent) {
  const live = liveModels.get(agent);
  if (live && live.length) return live;
  return MODEL_SUGGESTIONS[agent] || MODEL_SUGGESTIONS.claude;
}
function suggestionValue(s) { return typeof s === "string" ? s : s.value; }
// Vertaal de gekozen/ingetypte modelnaam naar de --model-waarde voor de agent.
// Voor claude: oude weergavenaam -> model-ID; alles anders (alias of ID) gaat
// ongewijzigd door. Voor agy: ongewijzigd (agy wil de volledige label-string).
function resolveModelArg(agent, model) {
  const m = (model || "").trim();
  if (agent === "claude" && CLAUDE_LEGACY_MODELS[m]) return CLAUDE_LEGACY_MODELS[m];
  return m;
}
function fillModelDatalist(dl, agent) {
  if (!dl) return;
  dl.innerHTML = "";
  for (const s of modelSuggestionsFor(agent)) {
    const o = document.createElement("option");
    o.value = suggestionValue(s);
    if (typeof s !== "string" && s.key) o.label = t(s.key);
    dl.appendChild(o);
  }
}
// Vul direct met wat we hebben en nogmaals zodra de CLI-lijst binnen is, zodat
// het veld nooit wacht op een procesaanroep.
function updateModelDatalist(dl, agent) {
  fillModelDatalist(dl, agent);
  refreshLiveModels(agent).then((fresh) => { if (fresh) fillModelDatalist(dl, agent); });
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
// Generatieteller per (her)start: pty-events dragen de gen mee, zodat een
// verlaat event van een gekild proces (herstart hergebruikt het sessie-id)
// nooit de nieuwe incarnatie raakt (#71).
let genSeq = 0;
const sessions = new Map();
const els = {};

const DEFAULT_SETTINGS = {
  // Standaardtaal volgt het OS: alleen een Nederlandstalig systeem start in het
  // Nederlands; al het andere in het Engels. Een opgeslagen keuze wint altijd (#46).
  lang: (navigator.language || "en").toLowerCase().startsWith("nl") ? "nl" : "en",
  fontSize: 13, scrollback: 8000, cursorBlink: true,
  htmlView: "split",
  copyOnSelect: true, pasteOnRightClick: true, ctrlShiftCV: true,
  webLinks: true, search: true, tabShortcuts: true, tabStatus: true,
  fullPaths: true,
  persistSessions: true,
  agentMouse: false, // false = muis blijft lokaal (slepen selecteert); true = agent krijgt de muis
  skin: "", // "" = volg branding-default / anders "default"
  ttsEnabled: false, ttsVoice: "", ttsRate: 0, // Windows-native TTS
  sttAutoSend: false, // transcript meteen met Enter versturen
  sttModel: "", // gekozen model uit de bibliotheek (leeg = eerste)
  sttRegistry: "", // optionele modellenbibliotheek-URL (JSON); leeg = ingebouwde lijst
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
// Diagnoselog naar %APPDATA%\Taurus\clipboard.log (alleen metadata, geen inhoud).
// Helpt intermitterende kopieer/plak-problemen debuggen zonder devtools.
function dbg(line) {
  try { invoke("debug_log", { line: `[${new Date().toISOString()}] ${line}` }); } catch (_) {}
}
function copyToClipboard(text) {
  if (!text) { dbg("copy skipped (empty selection)"); return; }
  dbg(`copy attempt len=${text.length}`);
  invoke("copy_to_clipboard", { text })
    .then(() => dbg(`copy ok len=${text.length}`))
    .catch((e) => {
      dbg(`copy FAIL: ${e}`);
      console.error("clipboard copy failed:", e);
      toast(t("copy_failed") + " " + e, "err");
    });
}
function isNetwork(p) { return /^x:/i.test(p) || p.startsWith("\\\\"); }
function locClass(p) { return isNetwork(p) ? "net" : "local"; }
// Compacte drive-aanduiding achter de agentnaam: "(C:)" / "(X:)" / "(UNC)".
// Vervangt de bredere LOCAL/NETWORK-regel zodat er meer agents in de lijst passen.
function driveTag(p) {
  if (p.startsWith("\\\\")) return "(UNC)";
  const d = (p.match(/^([a-z]):/i) || [])[1];
  return d ? `(${d.toUpperCase()}:)` : "";
}
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

/* ============ herordenen (slepen) ============ */
// Slepen met de MUIS (pointer-events), niet HTML5-DnD: de file-dropper gebruikt
// Tauri's eigen OS-drag-events (tauri://drag-*), en element-DnD is onder WebView2
// onbetrouwbaar. Deze aanpak werkt zelfstandig en botst daar niet mee. Tijdens het
// slepen verplaatsen we het element live tussen zijn buren; bij loslaten lezen we
// de nieuwe DOM-volgorde terug. CSS user-select:none voorkomt dat klikken-en-
// vasthouden tekst selecteert -- zie #55.
let suppressNextClick = false; // onderdruk de click die na een sleep-loslaten volgt

function makeReorderable(el, opts) {
  el.addEventListener("mousedown", (e) => {
    // Alleen linkermuisknop; niet starten op knoppen of de tab-sluitknop.
    if (e.button !== 0 || e.target.closest("button, .tab-close")) return;
    const container = el.parentElement;
    if (!container) return;
    const sx = e.clientX, sy = e.clientY;
    let started = false;
    const move = (me) => {
      if (!started) {
        if (Math.abs(me.clientX - sx) < 5 && Math.abs(me.clientY - sy) < 5) return;
        started = true;
        el.classList.add("dragging");
        document.body.classList.add("reordering");
      }
      me.preventDefault();
      const end = opts.endSelector ? container.querySelector(opts.endSelector) : null;
      const sibs = [...container.children].filter((c) => c !== el && c !== end && c.classList.contains(opts.itemClass));
      let placed = false;
      for (const sib of sibs) {
        const r = sib.getBoundingClientRect();
        const before = opts.axis === "x" ? me.clientX < r.left + r.width / 2 : me.clientY < r.top + r.height / 2;
        if (before) { container.insertBefore(el, sib); placed = true; break; }
      }
      if (!placed) { if (end) container.insertBefore(el, end); else container.appendChild(el); }
    };
    const up = () => {
      document.removeEventListener("mousemove", move);
      document.removeEventListener("mouseup", up);
      document.body.classList.remove("reordering");
      if (started) {
        el.classList.remove("dragging");
        // De click die na mouseup zou volgen (project kiezen / tab wisselen) even
        // negeren; setTimeout(0) is de vangnet als er geen click meer komt.
        suppressNextClick = true;
        setTimeout(() => { suppressNextClick = false; }, 0);
        opts.onDrop();
      }
    };
    document.addEventListener("mousemove", move);
    document.addEventListener("mouseup", up);
  });
}

// Lees de nieuwe kaartvolgorde uit de DOM (data-idx = originele render-index),
// herschik het projects-array, persist en herteken.
function commitProjectOrder() {
  const snap = projects;
  const order = [...els.list.children].map((c) => Number(c.dataset.idx));
  const next = order.map((i) => snap[i]).filter(Boolean);
  if (next.length === snap.length) {
    projects = next;
    invoke("save_projects", { projects }).catch((e) => toast("✗ " + e, "err"));
  }
  renderProjects();
}

// Idem voor de tabs: herbouw de sessions-Map in de nieuwe volgorde (Map bewaart de
// invoegvolgorde en renderTabs itereert sessions.values()), persist die volgorde.
function commitTabOrder() {
  const ids = [...els.tabbar.children].map((c) => c.dataset.tabId).filter(Boolean);
  const pairs = ids.map((id) => [id, sessions.get(id)]).filter(([, s]) => s);
  if (pairs.length === sessions.size) {
    sessions.clear();
    for (const [id, s] of pairs) sessions.set(id, s);
    persistSessionsToDisk();
  }
  renderTabs();
}

/* ============ projecten ============ */
function renderProjects() {
  els.list.innerHTML = "";
  projects.forEach((p, index) => {
    const card = document.createElement("div");
    card.className = "project-card";
    card.dataset.idx = String(index);
    card.style.borderLeftColor = p.accent || "#7c9cff";
    card.innerHTML =
      `<div class="pc-label">${escapeHtml(p.label)} <span class="pc-drive ${locClass(p.path)}">${driveTag(p.path)}</span></div>` +
      `<div class="pc-actions">` +
        `<button class="pc-edit" title="${escapeHtml(t("edit"))}">✎</button>` +
        `<button class="pc-del" title="${escapeHtml(t("delete"))}">🗑</button>` +
      `</div>` +
      `<div class="pc-confirm">` +
        `<span>${escapeHtml(t("confirm_delete"))}</span>` +
        `<button class="pc-yes">${escapeHtml(t("yes"))}</button>` +
        `<button class="pc-no">${escapeHtml(t("no"))}</button>` +
      `</div>`;
    card.addEventListener("click", () => { if (suppressNextClick) return; selectProject(p, card); showView("new"); });
    // e.stopPropagation() zodat de kaart-klik (project kiezen) niet meevuurt.
    card.querySelector(".pc-edit").addEventListener("click", (e) => { e.stopPropagation(); openEditor(); });
    card.querySelector(".pc-del").addEventListener("click", (e) => { e.stopPropagation(); card.classList.add("confirming"); });
    card.querySelector(".pc-confirm").addEventListener("click", (e) => e.stopPropagation());
    card.querySelector(".pc-no").addEventListener("click", (e) => { e.stopPropagation(); card.classList.remove("confirming"); });
    card.querySelector(".pc-yes").addEventListener("click", (e) => { e.stopPropagation(); deleteProject(p); });
    makeReorderable(card, { axis: "y", itemClass: "project-card", endSelector: null, onDrop: commitProjectOrder });
    els.list.appendChild(card);
  });
}

// Verwijder een project (na bevestiging in de kaart). Persist, herteken; als het
// geselecteerde project verdwijnt, terug naar het lege startformulier.
async function deleteProject(p) {
  const next = projects.filter((x) => x !== p);
  try {
    await invoke("save_projects", { projects: next });
    projects = next;
    if (selected === p) { selected = null; resetLaunchForm(); }
    renderProjects();
  } catch (e) { toast("✗ " + e, "err"); }
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
  updateModelDatalist(els.modelSuggestions, els.agentInput.value);
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
    tab.dataset.tabId = s.id;
    tab.style.borderTopColor = s.accent || "#7c9cff";
    tab.style.setProperty("--tab-accent", s.accent || "#7c9cff");
    const live = settings.tabStatus && s.status && !s.exited;
    const shown = live ? `✶ ${s.status}…` : s.title;
    tab.innerHTML = `<span class="tab-dot"></span><span class="tab-title${live ? " live" : ""}">${escapeHtml(shown)}</span><span class="tab-close">✕</span>`;
    tab.title = s.title;
    tab.addEventListener("click", () => { if (suppressNextClick) return; showView(s.id); });
    tab.addEventListener("contextmenu", (e) => { e.preventDefault(); openTabMenu(e.clientX, e.clientY, s.id); });
    tab.querySelector(".tab-close").addEventListener("click", (e) => { e.stopPropagation(); closeSession(s.id); });
    makeReorderable(tab, { axis: "x", itemClass: "tab", endSelector: ".newtab", onDrop: commitTabOrder });
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
        <button class="preview-raw" title="Raw / rendered (markdown)">&lt;/&gt;</button>
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

  // Muis-overname (mouse-tracking) van de agent onderscheppen voor de KNOPPEN.
  // Een TUI zoals Claude Code zet via DECSET (ESC[?1000h..1006h) de muis-rapportage
  // aan, zodat de muis naar de agent gaat i.p.v. naar lokale tekstselectie -- dan
  // moet je Shift+slepen om te selecteren en gaat ook een rechtsklik naar de agent.
  // Standaard (agentMouse uit) onderscheppen we die sequenties: slepen selecteert
  // lokaal en rechtsklik plakt enkel. We ONTHOUDEN wel wat de TUI vroeg (appMouseOn
  // + SGR-encoding), zodat we het WIEL alsnog kunnen doorsturen -- zie de wiel-
  // handler hieronder. Andere DECSET-modi (bracketed paste 2004, alt-screen 1049,
  // cursor 25, ...) laten we ongemoeid. Aanzetten geeft de agent de muis helemaal terug.
  let appMouseOn = false;  // TUI vroeg muis-events (1000/1002/1003)
  let appMouseSgr = false; // TUI wil SGR-encoding (1006)
  const MOUSE_MODES = new Set([1000, 1001, 1002, 1003, 1005, 1006, 1015, 1016]);
  const firstParam = (params) => { const p = params && params.length ? params[0] : 0; return Array.isArray(p) ? (p[0] || 0) : (p || 0); };
  const mouseMode = (on) => (params) => {
    // Volg alle params (bv. ESC[?1002;1006h) voor de vlaggen; de swallow-beslissing
    // blijft op de eerste param, net als voorheen (geen regressie op gebundelde modi).
    const n = params && params.length ? params.length : 0;
    for (let i = 0; i < n; i++) {
      const raw = params[i];
      const m = Array.isArray(raw) ? (raw[0] || 0) : (raw || 0);
      if (m === 1000 || m === 1002 || m === 1003) appMouseOn = on;
      else if (m === 1006) appMouseSgr = on;
    }
    return !settings.agentMouse && MOUSE_MODES.has(firstParam(params));
  };
  if (term.parser && term.parser.registerCsiHandler) {
    term.parser.registerCsiHandler({ prefix: "?", final: "h" }, mouseMode(true));  // DECSET (aan)
    term.parser.registerCsiHandler({ prefix: "?", final: "l" }, mouseMode(false)); // DECRST (uit)
  }

  // Muiswiel (#35). De knoppen blijven lokaal, maar het WIEL sturen we naar de TUI
  // door als die full-screen draait (alt-screen) en muis-tracking wilde: dan scrollt
  // Claude z'n eigen transcript -- xterm heeft in het alt-screen immers geen eigen
  // scrollback. In de normale buffer scrollt het wiel gewoon onze lokale scrollback.
  // Zonder deze doorstuur bleef het wiel steken op xterm's wiel->pijltjes-vertaling,
  // die de agent als history-navigatie ("commando-log") las. Met agentMouse aan
  // handelt xterm het wiel zelf af (de agent heeft dan de hele muis).
  const cellUnderWheel = (e) => {
    try {
      const host = term.element || termPane;
      const r = host.getBoundingClientRect();
      const col = Math.min(term.cols, Math.max(1, Math.floor((e.clientX - r.left) / (r.width / term.cols)) + 1));
      const row = Math.min(term.rows, Math.max(1, Math.floor((e.clientY - r.top) / (r.height / term.rows)) + 1));
      return { col, row };
    } catch (_) { return { col: 1, row: 1 }; }
  };
  if (term.attachCustomWheelEventHandler) {
    term.attachCustomWheelEventHandler((e) => {
      if (settings.agentMouse) return true; // agent heeft de muis: xterm/TUI handelt het wiel af
      if (term.buffer.active.type !== "alternate") return true; // normale buffer: xterm scrollt de scrollback
      if (appMouseOn) { // full-screen TUI met muis: stuur een wiel-rapport zodat die z'n transcript scrollt
        const btn = e.deltaY < 0 ? 64 : 65; // 64 = omhoog, 65 = omlaag
        const { col, row } = cellUnderWheel(e);
        const seq = appMouseSgr
          ? `\x1b[<${btn};${col};${row}M`
          : `\x1b[M${String.fromCharCode(32 + btn)}${String.fromCharCode(32 + col)}${String.fromCharCode(32 + row)}`;
        invoke("write_session", { id, data: seq });
      }
      return false; // niet lokaal afhandelen / geen pijltjes naar de TUI
    });
  }

  const session = {
    id, uuid, path, title, accent, mode, command, agent: agent || "claude", model: model || "", term, fit, search, el,
    gen: ++genSeq,
    exited: false, working: false, awaiting: false, announced: false, status: null, lastSpin: 0, buf: "",
    decoder: new TextDecoder("utf-8"), previewMode: null, lastSel: "",
  };
  sessions.set(id, session);

  term.onData((d) => invoke("write_session", { id, data: d }));
  term.onResize(({ cols, rows }) => invoke("resize_session", { id, cols, rows }));
  // Kopieren bij selectie via xterm's onSelectionChange (vuurt betrouwbaar; een
  // DOM mouseup op het paneel komt niet door xterm's eigen muis-afhandeling).
  // We leggen de laatste niet-lege selectie vast en kopieren met een korte
  // debounce -- niet bij elke tussenstap (spam/contentie), en als een herteken
  // (streaming/prompt) de selectie net wist, kopieren we de vastgelegde tekst.
  let selTimer = null;
  term.onSelectionChange(() => {
    const sel = term.getSelection();
    if (sel) session.lastSel = sel;
    if (!settings.copyOnSelect) return;
    if (selTimer) clearTimeout(selTimer);
    selTimer = setTimeout(() => {
      const s = term.getSelection() || session.lastSel;
      if (s) copyToClipboard(s);
    }, 120);
  });
  termPane.addEventListener("mousedown", (e) => { if (e.button === 0) session.lastSel = ""; });
  // Voorkom dat de RECHTERknop als muis-event naar de TUI gaat (mouseMode=any):
  // anders plakt Claude Code op die rechtsklik OOK zelf het klembord -> de tekst
  // verschijnt dubbel. We laten de linkerknop ongemoeid (klikken en
  // Shift-selecteren in de TUI blijven werken). De capture-fase op het paneel
  // draait voor xterm's eigen muis-handlers, zodat xterm de knop niet doorstuurt.
  const swallowRightBtn = (e) => { if (settings.pasteOnRightClick && e.button === 2) e.stopPropagation(); };
  termPane.addEventListener("mousedown", swallowRightBtn, true);
  termPane.addEventListener("mouseup", swallowRightBtn, true);
  // Rechtermuis-plak met debounce: een enkele rechtsklik mag niet twee keer
  // plakken (dubbele event/echo). Negeer een tweede plak binnen 250 ms.
  let lastPasteAt = 0;
  termPane.addEventListener("contextmenu", async (e) => {
    if (!settings.pasteOnRightClick) return;
    e.preventDefault();
    const now = Date.now();
    if (now - lastPasteAt < 250) { dbg("paste IGNORED (debounce)"); return; }
    lastPasteAt = now;
    dbg("paste rightclick");
    try {
      const txt = await navigator.clipboard.readText();
      if (txt) { invoke("write_session", { id, data: txt }); dbg(`paste wrote len=${txt.length}`); }
    } catch (err) { dbg(`paste FAIL: ${err}`); }
  });

  el.querySelector(".preview-file").addEventListener("change", (e) => renderPreview(session, e.target.value));
  // Raw/rendered wisselen (alleen zinvol voor .md); hertekent het huidige bestand.
  el.querySelector(".preview-raw").addEventListener("click", () => { session.previewRaw = !session.previewRaw; if (session.previewPath) renderPreview(session, session.previewPath); });
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
        const re = /([A-Za-z]:[\\/][^\s"'<>|]+?\.(?:html?|md)|[\w.\-\\/]+\.(?:html?|md))/gi;
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
    await invoke("create_session", { id, gen: session.gen, path, title, task, sessionId: uuid, mode, fullPaths: settings.fullPaths, command, agent, model: resolveModelArg(agent, model), cols: session.term.cols, rows: session.term.rows });
    showView(id);
    persistSessionsToDisk();
  } catch (e) {
    sessions.delete(id); session.term.dispose(); session.el.remove();
    els.status.textContent = "✗ " + e; els.status.className = "status-msg err";
    renderTabs();
  }
}

// "＋ Agent toevoegen" op het startformulier: bewaar de nu ingevulde agent in het
// linkermenu (save_projects) EN start 'm meteen. Zo bouwt een nieuwe gebruiker het
// menu op i.p.v. steeds een eenmalige (niet-bewaarde) agent te openen -- zie #54.
// Bestaat er al een agent met hetzelfde id (slug van het label), dan wordt die
// bijgewerkt i.p.v. verdubbeld.
async function addAgentFromForm() {
  if (!selected || !selected.path) return;
  const label = (selected.label || els.titleInput.value.trim() || "agent").trim();
  const entry = {
    id: slugify(label),
    label,
    path: selected.path,
    title: els.titleInput.value.trim() || label,
    task: els.taskInput.value.trim(),
    accent: selected.accent || "#7c9cff",
    mode: els.modeInput.value || "default",
    agent: els.agentInput.value || "claude",
    model: els.modelInput.value.trim(),
    command: selected.command || "",
  };
  const next = projects.filter((p) => p.id !== entry.id);
  next.push(entry);
  try {
    await invoke("save_projects", { projects: next });
    projects = next;
    selected = entry;
    renderProjects();
  } catch (e) {
    els.status.textContent = "✗ " + e; els.status.className = "status-msg err";
    return;
  }
  await startSession(); // toevoegen + starten
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
        id, gen: session.gen, path: meta.path, title: session.title, sessionId: uuid,
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
    // Het RUWE href-attribuut bekijken, niet a.href: die lost '#'/relatief op naar
    // de app-origin (http://...localhost/#) en matchte dan als "externe" link ->
    // localhost opende in de browser. Alleen echte http(s)-links gaan extern; een
    // anker (#...) blijft in het document; al het andere (relatief/mailto) doet niets
    // binnen de sandbox.
    var href = a.getAttribute('href') || '';
    if (/^(https?:\\/\\/|mailto:)/i.test(href)) {
      ev.preventDefault();
      // Be authoritative: stop the page's own click handlers (e.g. a dashboard
      // openExternal() that would otherwise show a 'copy/paste' fallback).
      ev.stopImmediatePropagation();
      parent.postMessage({ type: 'taurus-open-external', url: href }, '*');
    } else if (href.charAt(0) === '#') {
      // Anker: expliciet scrollen -- fragment-navigatie is binnen een sandboxed
      // srcdoc-iframe niet overal betrouwbaar.
      var el = document.getElementById(href.slice(1));
      if (el) { ev.preventDefault(); el.scrollIntoView({ behavior: 'smooth' }); }
      else { ev.preventDefault(); }
    } else {
      ev.preventDefault(); // relatief e.d.: niet laten navigeren binnen de sandbox
    }
  }, true);
<\/script>`;

// Thema-CSS voor de markdown-render (de srcdoc-iframe erft de app-CSS niet).
const MD_STYLE = `<style>
  :root { color-scheme: dark; }
  body { font-family: "Segoe UI", system-ui, sans-serif; color: #e6e8ee; background: #14161c; padding: 20px 26px; line-height: 1.6; max-width: 820px; }
  a { color: #7c9cff; }
  code { background: #21252f; padding: 1px 5px; border-radius: 4px; font-family: "Cascadia Code", Consolas, monospace; font-size: .92em; }
  pre.md-code { background: #1b1e26; border: 1px solid #2e3340; border-radius: 8px; padding: 12px 14px; overflow: auto; }
  pre.md-code code { background: none; padding: 0; }
  h1, h2, h3 { line-height: 1.25; } h1 { border-bottom: 1px solid #2e3340; padding-bottom: .3em; }
  blockquote { border-left: 3px solid #2e3340; margin: 0 0 12px; padding: 2px 14px; color: #9aa1b1; }
  img { max-width: 100%; }
  table { border-collapse: collapse; margin: 0 0 14px; }
  th, td { border: 1px solid #2e3340; padding: 6px 10px; text-align: left; }
  th { background: #1b1e26; }
  del { opacity: .65; }
  li.md-task { list-style: none; margin-left: -18px; }
  li.md-task input { accent-color: #7c9cff; vertical-align: middle; }
</style>`;

// Veilige, compacte markdown-renderer. Kernprincipe: escape EERST alles (zo kan
// ruwe HTML/<script> in de bron niets injecteren -- gelijk aan 'html:false');
// pas daarna markdown-regels toe. Links/afbeeldingen krijgen alleen http(s)/mailto/
// relatieve URL's; javascript:/data: worden geweigerd. Zie #61.
function mdSafeUrl(u) {
  const s = String(u).trim();
  if (/^(https?:|mailto:)/i.test(s) || !/^[a-z0-9.+-]+:/i.test(s)) return s; // http(s)/mailto of geen schema (relatief/anker)
  return null; // javascript:, data:, vbscript:, elk ander schema -> blokkeren
}
function mdInline(s) {
  s = s.replace(/`([^`]+)`/g, (_, c) => `<code>${c}</code>`);
  // Geblokkeerde URL -> GEEN <a>/<img> maar platte tekst. Een geneutraliseerd
  // href="#" loste in de iframe op naar de app-origin en opende zo alsnog
  // "localhost" in de externe browser -- dus helemaal geen link renderen.
  s = s.replace(/!\[([^\]]*)\]\(([^)\s]+)\)/g, (_, alt, url) => { const u = mdSafeUrl(url); return u ? `<img alt="${alt}" src="${u}" />` : alt; });
  s = s.replace(/\[([^\]]+)\]\(([^)\s]+)\)/g, (_, txt, url) => { const u = mdSafeUrl(url); return u ? `<a href="${u}">${txt}</a>` : txt; });
  s = s.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  s = s.replace(/__([^_]+)__/g, "<strong>$1</strong>");
  s = s.replace(/\*([^*]+)\*/g, "<em>$1</em>");
  s = s.replace(/~~([^~]+)~~/g, "<del>$1</del>");
  return s;
}
function mdToHtml(src) {
  const lines = escapeHtml(String(src).replace(/\r\n/g, "\n")).split("\n");
  const special = /^(#{1,6}\s|```|\s*[-*+]\s|\s*\d+\.\s|&gt;\s?|(?:---|\*\*\*|___)\s*$)/;
  let html = "", i = 0, inUl = false, inOl = false;
  const closeLists = () => { if (inUl) { html += "</ul>"; inUl = false; } if (inOl) { html += "</ol>"; inOl = false; } };
  while (i < lines.length) {
    const line = lines[i];
    const fence = line.match(/^```/);
    if (fence) {
      closeLists(); i++;
      let code = "";
      while (i < lines.length && !/^```/.test(lines[i])) { code += lines[i] + "\n"; i++; }
      i++;
      html += `<pre class="md-code"><code>${code}</code></pre>`;
      continue;
    }
    // GFM-tabel: kopregel met | gevolgd door een scheidingsregel (|---|:--:|---:|).
    if (line.includes("|") && i + 1 < lines.length && lines[i + 1].includes("-") && /^\s*\|?[\s:|-]+\|?\s*$/.test(lines[i + 1]) && lines[i + 1].includes("|")) {
      closeLists();
      const splitRow = (l) => l.trim().replace(/^\|/, "").replace(/\|$/, "").split("|").map((c) => c.trim());
      const aligns = splitRow(lines[i + 1]).map((c) => (/^:-+:$/.test(c) ? "center" : /^-+:$/.test(c) ? "right" : ""));
      const cell = (tag, c, k) => `<${tag}${aligns[k] ? ` style="text-align:${aligns[k]}"` : ""}>${mdInline(c)}</${tag}>`;
      html += `<table><tr>${splitRow(line).map((c, k) => cell("th", c, k)).join("")}</tr>`;
      i += 2;
      while (i < lines.length && lines[i].includes("|") && !/^\s*$/.test(lines[i])) {
        html += `<tr>${splitRow(lines[i]).map((c, k) => cell("td", c, k)).join("")}</tr>`;
        i++;
      }
      html += "</table>";
      continue;
    }
    const h = line.match(/^(#{1,6})\s+(.*)$/);
    if (h) {
      closeLists(); const n = h[1].length;
      // GitHub-achtige id zodat ankers ([tekst](#kop)) werken; entities eruit.
      const id = h[2].toLowerCase().replace(/&[a-z#0-9]+;/g, "").replace(/[^a-z0-9\s-]/g, "").trim().replace(/\s+/g, "-");
      html += `<h${n} id="${id}">${mdInline(h[2])}</h${n}>`; i++; continue;
    }
    if (/^(?:---|\*\*\*|___)\s*$/.test(line)) { closeLists(); html += "<hr />"; i++; continue; }
    if (/^&gt;\s?/.test(line)) { closeLists(); html += `<blockquote>${mdInline(line.replace(/^&gt;\s?/, ""))}</blockquote>`; i++; continue; }
    const ul = line.match(/^\s*[-*+]\s+(.*)$/);
    if (ul) {
      if (inOl) closeLists(); if (!inUl) { html += "<ul>"; inUl = true; }
      // GFM-taaklijst: - [x] / - [ ] -> uitgeschakelde checkbox.
      const task = ul[1].match(/^\[([ xX])\]\s+(.*)$/);
      html += task
        ? `<li class="md-task"><input type="checkbox" disabled${task[1] === " " ? "" : " checked"} /> ${mdInline(task[2])}</li>`
        : `<li>${mdInline(ul[1])}</li>`;
      i++; continue;
    }
    const ol = line.match(/^\s*\d+\.\s+(.*)$/);
    if (ol) { if (inUl) closeLists(); if (!inOl) { html += "<ol>"; inOl = true; } html += `<li>${mdInline(ol[1])}</li>`; i++; continue; }
    if (/^\s*$/.test(line)) { closeLists(); i++; continue; }
    closeLists();
    let para = line; i++;
    // Stop de samenvoeging ook bij een mogelijke tabelregel (|) zodat een tabel
    // direct na een alinea niet als tekst wordt opgeslokt.
    while (i < lines.length && !/^\s*$/.test(lines[i]) && !special.test(lines[i]) && !lines[i].includes("|")) { para += " " + lines[i]; i++; }
    html += `<p>${mdInline(para)}</p>`;
  }
  closeLists();
  return html;
}

// Toon een .html-preview (ruw) of een .md-bestand (gerenderd of raw). .md wordt
// altijd escape-eerst gerenderd; ruwe HTML in de .md draait dus nooit als script.
async function renderPreview(s, path) {
  if (!path) return;
  s.previewPath = path;
  const frame = s.el.querySelector(".preview-frame");
  const isMd = /\.md$/i.test(path);
  try {
    const raw = await invoke("read_file", { path });
    if (isMd && !s.previewRaw) {
      frame.srcdoc = PREVIEW_BRIDGE + MD_STYLE + `<body>${mdToHtml(raw)}</body>`;
    } else if (isMd) {
      frame.srcdoc = MD_STYLE + `<body><pre class="md-code"><code>${escapeHtml(raw)}</code></pre></body>`;
    } else {
      frame.srcdoc = PREVIEW_BRIDGE + raw;
    }
  } catch (e) {
    // De 2 MB-grens wordt in Rust bewaakt (vóór lezen/IPC, #72); vertaal die
    // fout naar de nette i18n-melding.
    if (/file too large/i.test(String(e))) {
      frame.srcdoc = MD_STYLE + `<body>${escapeHtml(t("preview_toobig"))}</body>`;
    } else {
      frame.srcdoc = `<body style="font-family:sans-serif;color:#c66;padding:24px">${escapeHtml(String(e))}</body>`;
    }
  }
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
  const [sid, gen, data] = event.payload;
  const s = sessions.get(sid);
  if (!s || s.gen !== gen) return; // verlaat event van een vorige generatie
  // data is base64 (één string i.p.v. duizenden JSON-getallen per chunk, #73).
  const u8 = Uint8Array.from(atob(data), (c) => c.charCodeAt(0));
  s.term.write(u8);
  let render = false;
  if (!s.exited) {
    // Grotere buffer (4000) zodat de status-/spinnerregel niet uit beeld schuift bij
    // een output-piek -- anders leek de agent "klaar" terwijl 'ie doorwerkte.
    s.buf = (s.buf + stripAnsi(s.decoder.decode(u8, { stream: true }))).slice(-4000);
    const verb = lastSpinnerVerb(s.buf);
    // "Bezig"-signaal: een spinner-werkwoord OF Claude's "(esc to interrupt)"-hint,
    // die de hele beurt getoond wordt (ook tijdens tools/streamen) en dus veel
    // betrouwbaarder is dan "geen spinner gezien".
    const busy = !!verb || /\besc to interrupt\b/i.test(s.buf);
    if (busy) {
      s.lastSpin = Date.now();
      s.announced = false; // (weer) bezig -> een latere idle mag opnieuw melden
      if (!s.working || s.awaiting) { s.working = true; s.awaiting = false; render = true; }
      if (settings.tabStatus && verb && s.status !== verb) { s.status = verb; render = true; }
    }
  }
  if (render) renderTabs();
});
listen("pty-exit", (event) => {
  const [sid, gen] = event.payload;
  const s = sessions.get(sid);
  if (!s || s.gen !== gen) return; // exit van een gekilde vorige generatie: negeren
  s.exited = true; s.working = false; s.awaiting = false; s.status = null; s.term.write(`\r\n\x1b[2m${t("ended")}\x1b[0m\r\n`); renderTabs();
});
// "Klaar" = langere tijd GEEN bezig-signaal, mét bevestigingsvenster tegen valse
// meldingen: na READY_IDLE_MS valt "werkend" weg, en pas na nog eens READY_CONFIRM_MS
// stabiel idle flashen/spreken we. Keert de spinner in dat venster terug, dan wist de
// pty-handler s.working/announced en gebeurt er niets (geen valse "klaar"). Per beurt
// hooguit één melding (s.announced), en alleen voor een tab die je niet bekijkt.
const READY_IDLE_MS = 3000;
const READY_CONFIRM_MS = 2000;
setInterval(() => {
  let changed = false; const now = Date.now();
  for (const s of sessions.values()) {
    if (s.exited || !s.lastSpin) continue; // nooit gewerkt -> niets te melden
    const idle = now - s.lastSpin;
    if (s.working && idle > READY_IDLE_MS) { s.working = false; s.status = null; changed = true; }
    if (!s.working && !s.announced && idle > READY_IDLE_MS + READY_CONFIRM_MS) {
      s.announced = true; // afgehandeld (ook als je ernaar keek: geen latere melding)
      if (s.id !== current) {
        s.awaiting = true;
        speak(t("tts_ready").replace("{title}", s.title || "agent"));
        changed = true;
      }
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
  // Nieuwe generatie: verlate pty-events van het zojuist gekilde proces
  // (zelfde id!) worden vanaf nu genegeerd (#71).
  s.gen = ++genSeq;
  s.exited = false; s.working = false; s.awaiting = false; s.announced = false; s.status = null; s.buf = ""; s.decoder = new TextDecoder("utf-8");
  if (current !== id) showView(id); else renderTabs();
  try {
    await invoke("restart_session", { id, gen: s.gen, path: s.path, title: s.title, sessionId: s.uuid, mode: s.mode || "default", fullPaths: settings.fullPaths, command: s.command || "", agent: s.agent || "claude", model: resolveModelArg(s.agent || "claude", s.model || ""), cols: s.term.cols, rows: s.term.rows });
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
    <div class="ctx-item" data-act="speak">${t("ctx_speak")}</div>
    <div class="ctx-item" data-act="explorer">${t("ctx_explorer")}</div>
    <div class="ctx-item" data-act="close">${t("ctx_close")}</div>`;
  m.style.left = x + "px"; m.style.top = y + "px";
  m.querySelector('[data-act="restart"]').addEventListener("click", () => restartSession(id));
  m.querySelector('[data-act="preview"]').addEventListener("click", () => openPreview(id));
  m.querySelector('[data-act="speak"]').addEventListener("click", () => {
    closeTabMenu();
    const sel = s.term.getSelection();
    if (sel) speak(sel, true); // force: uitspreken is hier expliciet gevraagd
  });
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
/* ============ instellingen: tabs + hover-uitleg ============ */
function switchSettingsTab(name) {
  els.settingsModal.querySelectorAll(".stab").forEach((b) => b.classList.toggle("active", b.dataset.tab === name));
  els.settingsModal.querySelectorAll(".spane").forEach((p) => p.classList.toggle("active", p.dataset.pane === name));
  hideHelpTip();
}
// Vast uitleg-paneel onderin de modal: toont de uitgebreide uitleg van de instelling
// waar je overheen beweegt. Tekst komt uit de i18n-tabel (help_*), dus de taal volgt.
// Niets gehoverd -> terug naar de standaard-hint. Een vast paneel i.p.v. een zwevende
// tooltip zodat het ook in een smal venster (min. 720px) altijd past en leesbaar is.
function showHelpTip(row) {
  if (!els.helpTip) return;
  els.helpTip.textContent = t(row.getAttribute("data-help-key")) || t("help_default");
}
function hideHelpTip() { if (els.helpTip) els.helpTip.textContent = t("help_default"); }

function openSettings() {
  switchSettingsTab("general");
  els.setLang.value = settings.lang;
  els.setFont.value = settings.fontSize;
  els.setScroll.value = settings.scrollback;
  els.setCursor.checked = settings.cursorBlink;
  els.htmlSplit.checked = settings.htmlView === "split";
  els.htmlFull.checked = settings.htmlView === "full";
  els.setCopy.checked = settings.copyOnSelect;
  els.setPaste.checked = settings.pasteOnRightClick;
  els.setAgentMouse.checked = settings.agentMouse;
  els.setCtrl.checked = settings.ctrlShiftCV;
  els.setLinks.checked = settings.webLinks;
  els.setSearch.checked = settings.search;
  els.setTabs.checked = settings.tabShortcuts;
  els.setTabStatus.checked = settings.tabStatus;
  els.setFullPaths.checked = settings.fullPaths;
  els.setPersist.checked = settings.persistSessions;
  // Toon de effectieve skin: expliciete keuze, anders branding-default-skin,
  // anders de "brand"-skin (als er een branding-thema is), anders default.
  els.setSkin.value = settings.skin || brandingSkin || (brandHasTheme ? "brand" : "default");
  // Spraak: stemmen één keer ophalen, STT-modellenlijst + status verversen.
  els.ttsOn.checked = settings.ttsEnabled;
  els.ttsRate.value = settings.ttsRate | 0;
  if (!els.ttsVoiceSel.options.length) {
    invoke("list_tts_voices").then((vs) => {
      // Elke stem is "engine|naam"; toon de nette naam, groepeer natuurlijk boven
      // klassiek. De volledige getagde waarde blijft de option-value (voor speak_text).
      // v = "engine|taal|naam". Toon "Naam — Taal" (taal leesbaar via Intl.DisplayNames).
      const langName = (code) => {
        if (!code) return "";
        try { return new Intl.DisplayNames([settings.lang || "en"], { type: "language" }).of(code) || code; }
        catch (_) { return code; }
      };
      const opt = (v) => {
        const p = v.split("|");
        const name = p.slice(2).join("|") || p.slice(1).join("|") || v; // 3-veld, val terug op oud 2-veld
        const ln = p.length >= 3 ? langName(p[1]) : "";
        return `<option value="${escapeHtml(v)}">${escapeHtml(ln ? `${name} — ${ln}` : name)}</option>`;
      };
      const nat = vs.filter((v) => v.startsWith("winrt|"));
      const cls = vs.filter((v) => !v.startsWith("winrt|"));
      let html = `<option value=""></option>`;
      if (nat.length) html += `<optgroup label="${escapeHtml(t("voice_natural"))}">${nat.map(opt).join("")}</optgroup>`;
      if (cls.length) html += `<optgroup label="${escapeHtml(t("voice_classic"))}">${cls.map(opt).join("")}</optgroup>`;
      els.ttsVoiceSel.innerHTML = html;
      els.ttsVoiceSel.value = settings.ttsVoice || "";
    }).catch(() => {});
  } else {
    els.ttsVoiceSel.value = settings.ttsVoice || "";
  }
  els.sttAutoSend.checked = settings.sttAutoSend;
  els.sttRegistryInput.value = settings.sttRegistry || "";
  fillSttModelSelect();
  refreshSttStatus();
  els.settingsModal.classList.remove("hidden");
}

function fillSttModelSelect() {
  els.sttModelSel.innerHTML = sttModels
    .map((m) => `<option value="${escapeHtml(m.name)}">${escapeHtml(m.name)}${m.size ? ` (${escapeHtml(m.size)})` : ""}</option>`)
    .join("");
  els.sttModelSel.value = currentSttModel()?.name || "";
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
  settings.agentMouse = els.setAgentMouse.checked;
  settings.ctrlShiftCV = els.setCtrl.checked;
  settings.webLinks = els.setLinks.checked;
  settings.search = els.setSearch.checked;
  settings.tabShortcuts = els.setTabs.checked;
  settings.tabStatus = els.setTabStatus.checked;
  settings.fullPaths = els.setFullPaths.checked;
  settings.persistSessions = els.setPersist.checked;
  settings.skin = els.setSkin.value;
  settings.ttsEnabled = els.ttsOn.checked;
  settings.ttsVoice = els.ttsVoiceSel.value;
  settings.ttsRate = Math.min(10, Math.max(-10, parseInt(els.ttsRate.value) || 0));
  settings.sttAutoSend = els.sttAutoSend.checked;
  settings.sttModel = els.sttModelSel.value;
  settings.sttRegistry = els.sttRegistryInput.value.trim();
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
  hideHelpTip();
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
// Zelfde editor, maar meteen met een verse lege regel (de + bij PROJECTS).
function openEditorAdd() {
  openEditor();
  editRows.push(blankRow());
  renderEditor();
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
          <datalist id="dl-emodel-${i}">${modelSuggestionsFor(r.agent).map((s) => `<option value="${escapeHtml(suggestionValue(s))}"${typeof s !== "string" && s.key ? ` label="${escapeHtml(t(s.key))}"` : ""}></option>`).join("")}</datalist></div>
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
  // Vraag de CLI-lijst op voor de agents in deze editor; levert dat een nieuwe
  // lijst op, dan één keer opnieuw renderen zodat de datalists kloppen. Dit lust
  // niet: refreshLiveModels geeft na de eerste poging altijd false.
  for (const agent of new Set(editRows.map((r) => r.agent || "claude"))) {
    refreshLiveModels(agent).then((fresh) => { if (fresh) renderEditor(); });
  }
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
    // Val terug op de vastgelegde selectie als getSelection() leeg is (een
    // herteken kan 'm net gewist hebben).
    if (s) { const sel = s.term.getSelection() || s.lastSel; if (sel) { copyToClipboard(sel); e.preventDefault(); } }
    return;
  }
  if (settings.ctrlShiftCV && ctrl && e.shiftKey && (e.key === "V" || e.key === "v")) {
    e.preventDefault();
    const s = sessions.get(current);
    if (s) navigator.clipboard.readText().then((txt) => { if (txt) { invoke("write_session", { id: s.id, data: txt }); dbg(`paste kbd len=${txt.length}`); } }).catch((err) => dbg(`paste kbd FAIL: ${err}`));
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

/* ============ file-dropper (sidebar) ============ */
// Sleep een bestand of map het venster in -> Verplaats/Kopieer naar <werkmap>\input,
// of Alleen-pad (post enkel het bestaande pad). Objecten (geplakte tekst/afbeelding)
// lopen via de Plak-knop. Native drop blijft aan (dragDropEnabled = default), dus we
// luisteren naar de Tauri drag-drop-events, niet naar HTML5 DOM-drop -- die vuren bij
// ingeschakelde native drop niet. De drop-positie (physical px) hittesten we tegen de
// drie zones om de modus te bepalen; buiten de dropper negeren we de drop.

// Doel-werkmap: de actieve sessie, anders het geselecteerde project, anders geen.
function dropperCwd() {
  const s = sessions.get(current);
  if (s && s.path) return s.path;
  if (selected && selected.path) return selected.path;
  return null;
}

// Schrijf een absoluut pad in de actieve terminal (met quotes bij spaties, gevolgd
// door een spatie zodat je meteen door kunt typen). `silent` = geen melding als er
// geen actieve terminal is (voor auto-invoegen bij een drop op het startscherm).
function insertPathIntoTerminal(absPath, silent) {
  const s = sessions.get(current);
  if (!s) { if (!silent) toast(t("dropper_no_session"), "err"); return; }
  const arg = /\s/.test(absPath) ? `"${absPath}"` : absPath;
  invoke("write_session", { id: s.id, data: arg + " " });
}

// Voeg een net geplaatst bestand toe aan het overzicht -- ALLEEN wat jij deze sessie
// dropt/plakt, niet de hele input-map. Op een gedeelde X:-map staan daar ook de
// bestanden van collega's; die zou een dir-listing tonen en je focus wegnemen.
// Nieuwste bovenaan; klik plaatst het volledige pad in de prompt. textContent i.p.v.
// innerHTML zodat een rare bestandsnaam nooit als HTML wordt uitgevoerd.
function addDropperEntry(absPath) {
  const list = els.dropperList;
  if (!list) return;
  const name = absPath.split(/[\\/]/).pop() || absPath;
  const row = document.createElement("div");
  row.className = "dropper-item";
  row.textContent = name;
  row.title = absPath;
  row.addEventListener("click", () => insertPathIntoTerminal(absPath));
  list.prepend(row);
}

function wireFileDropper() {
  const panel = els.fileDropper;
  if (!panel) return;
  const zoneEls = [...panel.querySelectorAll(".dz")];

  // Physical -> CSS px voor het hittesten (Tauri geeft physical; getBoundingClientRect
  // is CSS). Buiten de dropper -> null (drop negeren). In de dropper maar niet op een
  // zone -> "prompt" (veilige default, ook bij een snelle drop voordat je mikt).
  function modeAt(pos) {
    if (!pos) return null;
    const dpr = window.devicePixelRatio || 1;
    const x = pos.x / dpr, y = pos.y / dpr;
    const pr = panel.getBoundingClientRect();
    if (x < pr.left || x > pr.right || y < pr.top || y > pr.bottom) return null;
    for (const el of zoneEls) {
      const r = el.getBoundingClientRect();
      if (x >= r.left && x <= r.right && y >= r.top && y <= r.bottom) return el.dataset.mode;
    }
    return "prompt";
  }
  function highlight(mode) {
    panel.classList.toggle("dropzone-active", mode != null);
    zoneEls.forEach((el) => el.classList.toggle("hot", el.dataset.mode === mode));
  }
  function cool() {
    panel.classList.remove("dropzone-active");
    zoneEls.forEach((el) => el.classList.remove("hot"));
  }

  listen("tauri://drag-enter", (e) => highlight(modeAt(e.payload && e.payload.position)));
  listen("tauri://drag-over", (e) => highlight(modeAt(e.payload && e.payload.position)));
  listen("tauri://drag-leave", cool);
  listen("tauri://drag-drop", async (e) => {
    cool();
    const mode = modeAt(e.payload && e.payload.position);
    if (!mode) return; // buiten de dropper: negeren
    const paths = (e.payload && e.payload.paths) || [];
    if (!paths.length) return;
    const cwd = dropperCwd();
    if (!cwd) { toast(t("dropper_need_project"), "err"); return; }
    for (const src of paths) {
      try {
        // Alleen pad: geen bestandsactie, enkel het bestaande pad in de prompt.
        // Verplaats/Kopieer: naar input\, dan het nieuwe pad in de prompt en in de lijst.
        if (mode === "prompt") {
          insertPathIntoTerminal(src, true);
        } else {
          const dest = await invoke("save_dropped_path", { src, cwd, mode });
          insertPathIntoTerminal(dest, true);
          addDropperEntry(dest);
        }
      } catch (err) {
        dbg(`drop ${mode} FAIL: ${err}`);
        toast(t("dropper_save_failed") + " " + err, "err");
      }
    }
  });

  els.dropperPaste.addEventListener("click", async () => {
    const cwd = dropperCwd();
    if (!cwd) { toast(t("dropper_need_project"), "err"); return; }
    try {
      const paths = await invoke("save_clipboard_to_input", { cwd });
      for (const p of paths) { insertPathIntoTerminal(p, true); addDropperEntry(p); }
    } catch (err) {
      dbg(`paste-object FAIL: ${err}`);
      toast(t("dropper_paste_failed") + " " + err, "err");
    }
  });
}

// De + op de DROPZONE: kies een bestand elders op de computer (de dialoog start in
// de input-map). Staat het gekozen bestand al IN input -> pad meteen in de prompt.
// Komt het van elders -> vraag Verplaats/Kopieer/Alleen-pad (er is geen drop-positie
// om op te hittesten, dus een kleine keuze-rij bovenin de dropper).
async function addFileViaPicker() {
  const cwd = dropperCwd();
  if (!cwd) { toast(t("dropper_need_project"), "err"); return; }
  const inputDir = cwd.replace(/\//g, "\\").replace(/\\+$/, "") + "\\input";
  let file = null;
  try { file = await invoke("pick_file", { startDir: inputDir }); } catch (_) { return; }
  if (!file) return; // geannuleerd
  const nf = file.replace(/\//g, "\\");
  const parent = nf.slice(0, nf.lastIndexOf("\\"));
  if (parent.toLowerCase() === inputDir.toLowerCase()) {
    // Al in de input-map: gewoon het pad in de prompt (en in het overzicht).
    insertPathIntoTerminal(file, true);
    addDropperEntry(file);
  } else {
    showFileChooser(file, cwd); // van elders: vraag wat er moet gebeuren
  }
}

// Kleine keuze-rij bovenin de dropper voor een van elders gekozen bestand.
function showFileChooser(file, cwd) {
  const list = els.dropperList;
  if (!list) return;
  const row = document.createElement("div");
  row.className = "dropper-choose";
  const nm = document.createElement("div");
  nm.className = "dc-name";
  nm.textContent = file.split(/[\\/]/).pop() || file;
  nm.title = file;
  const btns = document.createElement("div");
  btns.className = "dc-btns";
  const run = async (mode) => {
    row.remove();
    try {
      const dest = mode === "prompt" ? file : await invoke("save_dropped_path", { src: file, cwd, mode });
      insertPathIntoTerminal(dest, true);
      if (mode !== "prompt") addDropperEntry(dest);
    } catch (err) { dbg(`pick ${mode} FAIL: ${err}`); toast(t("dropper_save_failed") + " " + err, "err"); }
  };
  const mk = (label, cls, fn) => { const b = document.createElement("button"); b.textContent = label; if (cls) b.className = cls; b.addEventListener("click", fn); return b; };
  btns.append(
    mk(t("dz_move"), "", () => run("move")),
    mk(t("dz_copy"), "", () => run("copy")),
    mk(t("dz_prompt"), "", () => run("prompt")),
    mk("✕", "dc-x", () => row.remove()),
  );
  row.append(nm, btns);
  list.prepend(row);
}

/* ============ spraak: TTS + STT ============ */
// TTS via Windows-native stemmen (SAPI); STT via lokale sherpa-onnx sidecar
// (Parakeet v3). Modellenbibliotheek: ingebouwde lijst, optioneel vervangen
// door een registry-JSON zodat nieuwe modellen zonder app-update verschijnen.
// De sha256-checksums zijn verplicht: de backend weigert een download zonder
// geldige pin (#69), want de engine is een exe die uitgevoerd wordt. De pins
// hieronder zijn geverifieerd tegen de digests van de GitHub-release-assets.
const DEFAULT_STT_MODELS = [
  {
    name: "Parakeet TDT 0.6b v3 (int8, EN+EU talen)",
    engineUrl: "https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.13.3/sherpa-onnx-v1.13.3-win-x64-shared-MT-Release.tar.bz2",
    engineSha256: "0043cd9cdd755d35627299e6a02839e95a262508ae9593af6c5c72ffd674b650",
    modelUrl: "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v3-int8.tar.bz2",
    modelSha256: "5793d0fd397c5778d2cf2126994d58e9d56b1be7c04d13c7a15bb1b4eafb16bf",
    size: "~486 MB",
  },
];
let sttModels = [...DEFAULT_STT_MODELS];
let sttRecording = false;
let sttBusy = false;
let sttWantRecording = false; // gewenste staat (F9 ingedrukt = true); reconciler stemt af

function speak(text, force) {
  if (!force && !settings.ttsEnabled) return;
  invoke("speak_text", { text, voice: settings.ttsVoice || "", rate: settings.ttsRate | 0 }).catch(() => {});
}

async function loadSttModels() {
  if (!settings.sttRegistry) { sttModels = [...DEFAULT_STT_MODELS]; return; }
  try {
    const r = await fetch(settings.sttRegistry, { cache: "no-store" });
    const j = await r.json();
    if (Array.isArray(j.models) && j.models.length) sttModels = j.models;
  } catch (_) { /* offline: hou de huidige lijst */ }
}

function currentSttModel() {
  return sttModels.find((m) => m.name === settings.sttModel) || sttModels[0];
}

// Record-widget boven de DROPZONE: live equalizer-niveau + status.
let sttLevelTimer = null;
function setRecordLevel(v) {
  // *2 tilt een stille microfoon wat op; geklemd op 1.
  if (els.recordWidget) els.recordWidget.style.setProperty("--lvl", String(Math.min(1, v * 2)));
}
function startLevelPoll() {
  stopLevelPoll();
  sttLevelTimer = setInterval(async () => {
    try { setRecordLevel(await invoke("stt_level")); } catch (_) {}
  }, 90);
}
function stopLevelPoll() {
  if (sttLevelTimer) { clearInterval(sttLevelTimer); sttLevelTimer = null; }
  setRecordLevel(0);
}
function setRecordState(s) { // "idle" | "listening" | "transcribing"
  if (els.recordWidget) {
    els.recordWidget.classList.toggle("listening", s === "listening");
    els.recordWidget.classList.toggle("transcribing", s === "transcribing");
  }
  if (els.recordStatus) {
    els.recordStatus.textContent = t(s === "listening" ? "rec_listening" : s === "transcribing" ? "rec_transcribing" : "rec_idle");
  }
}

// Opname sturen via een gewenste-staat + reconciler. F9 INHOUDEN zet de wens
// (down = opnemen, up = stoppen); de 🎙-knop en de opnameknop boven de DROPZONE
// schakelen de wens om (klik = aan/uit). De reconciler roept stt_toggle alleen aan
// als werkelijke != gewenste staat, en draait na afloop opnieuw als de wens intussen
// veranderde -- zo laat een korte F9-tik geen opname hangen. Widget-status:
// listening tijdens opname, transcribing tijdens de (paar seconden durende) stop.
function sttPttDown() { sttWantRecording = true; sttReconcile(); }
function sttPttUp() { sttWantRecording = false; sttReconcile(); }
function sttToggle() { sttWantRecording = !sttRecording; sttReconcile(); }

async function sttReconcile() {
  if (sttBusy || sttWantRecording === sttRecording) return;
  sttBusy = true;
  const starting = !sttRecording;
  if (starting) { setRecordState("listening"); startLevelPoll(); }
  else { setRecordState("transcribing"); stopLevelPoll(); }
  try {
    const r = await invoke("stt_toggle");
    sttRecording = !!r.recording;
    if (!r.recording && r.text) {
      const s = sessions.get(current);
      if (s) {
        invoke("write_session", { id: s.id, data: r.text + (settings.sttAutoSend ? "\r" : "") });
      } else {
        copyToClipboard(r.text);
        toast(r.text.slice(0, 120), "");
      }
    }
  } catch (e) {
    sttRecording = false;
    toast(t("stt_failed") + " " + e, "err");
  } finally {
    sttBusy = false;
    if (!sttRecording) { stopLevelPoll(); setRecordState("idle"); }
    if (sttWantRecording !== sttRecording) sttReconcile(); // wens veranderde tijdens de invoke
  }
}

// Instellingen → Spraak: statusregel + downloadknop, gepolld zolang de modal
// open staat of er een download loopt.
async function refreshSttStatus() {
  let st = { engine: false, model: false, downloading: false };
  try { st = await invoke("stt_status"); } catch (_) {}
  const ok = st.engine && st.model;
  els.sttState.textContent = st.downloading ? t("stt_downloading") : (ok ? t("stt_ready_lbl") : t("stt_missing_lbl"));
  els.sttState.className = "stt-state " + (ok ? "ok" : "miss");
  els.sttDownload.disabled = !!st.downloading || ok;
  if (st.downloading && !els.settingsModal.classList.contains("hidden")) {
    setTimeout(refreshSttStatus, 3000);
  }
}

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
    setAgentMouse: document.querySelector("#set-agentmouse"),
    setCtrl: document.querySelector("#set-ctrlshift"),
    setLinks: document.querySelector("#set-weblinks"),
    setSearch: document.querySelector("#set-search"),
    setTabs: document.querySelector("#set-tabshortcuts"),
    setTabStatus: document.querySelector("#set-tabstatus"),
    setFullPaths: document.querySelector("#set-fullpaths"),
    setPersist: document.querySelector("#set-persist"),
    setSkin: document.querySelector("#set-skin"),
    recordWidget: document.querySelector("#record-widget"),
    recordBtn: document.querySelector("#record-btn"),
    recordStatus: document.querySelector("#record-status"),
    ttsOn: document.querySelector("#set-tts-on"),
    ttsVoiceSel: document.querySelector("#set-tts-voice"),
    ttsRate: document.querySelector("#set-tts-rate"),
    sttModelSel: document.querySelector("#set-stt-model"),
    sttDownload: document.querySelector("#set-stt-download"),
    sttState: document.querySelector("#stt-state"),
    sttAutoSend: document.querySelector("#set-stt-autosend"),
    sttRegistryInput: document.querySelector("#set-stt-registry"),
    toast: document.querySelector("#toast"),
    modeInput: document.querySelector("#mode-input"),
    agentInput: document.querySelector("#agent-input"),
    modelInput: document.querySelector("#model-input"),
    modelSuggestions: document.querySelector("#model-suggestions"),
    helpTip: document.querySelector("#help-tip"),
    editorModal: document.querySelector("#editor-modal"),
    editorRows: document.querySelector("#editor-rows"),
    editorStatus: document.querySelector("#editor-status"),
    appVersion: document.querySelector("#app-version"),
    fileDropper: document.querySelector("#file-dropper"),
    dropperList: document.querySelector("#dropper-list"),
    dropperPaste: document.querySelector("#dropper-paste"),
  });

  document.querySelector("#launch-btn").addEventListener("click", startSession);
  document.querySelector("#add-agent-btn").addEventListener("click", addAgentFromForm);
  // Andere agent op het startformulier -> model-keuze EN modus-opties mee.
  // Het modelveld wissen: een model van de vorige agent is hier niet geldig en
  // zou de datalist-suggesties wegfilteren (leeg = de default van de agent).
  els.agentInput.addEventListener("change", () => {
    els.modelInput.value = "";
    updateModelDatalist(els.modelSuggestions, els.agentInput.value);
    fillModeSelect(els.modeInput, els.agentInput.value, els.modeInput.value);
  });
  document.querySelector("#browse-btn").addEventListener("click", browseFolder);
  document.querySelector("#reload-btn").addEventListener("click", loadProjects);
  document.querySelector("#settings-btn").addEventListener("click", openSettings);
  document.querySelector("#add-project-btn").addEventListener("click", openEditorAdd);
  document.querySelector("#add-file-btn").addEventListener("click", addFileViaPicker);
  document.querySelector("#settings-cancel").addEventListener("click", () => { hideHelpTip(); els.settingsModal.classList.add("hidden"); });
  // Spraak: mic-knop + F9-event uit de backend + instellingen-acties.
  els.recordBtn.addEventListener("click", sttToggle); // grote opnameknop boven de DROPZONE
  listen("stt-ptt-down", sttPttDown); // F9 ingedrukt -> opnemen
  listen("stt-ptt-up", sttPttUp);     // F9 losgelaten -> stoppen + transcriberen
  document.querySelector("#set-tts-test").addEventListener("click", () => {
    // Test met de NU gekozen (nog niet opgeslagen) stem/snelheid.
    invoke("speak_text", { text: t("tts_ready").replace("{title}", "Taurus"), voice: els.ttsVoiceSel.value, rate: parseInt(els.ttsRate.value) || 0 }).catch(() => {});
  });
  document.querySelector("#set-stt-download").addEventListener("click", async () => {
    const m = sttModels.find((x) => x.name === els.sttModelSel.value) || sttModels[0];
    if (!m) return;
    try { await invoke("stt_download", { engineUrl: m.engineUrl, engineSha256: m.engineSha256 || "", modelUrl: m.modelUrl, modelSha256: m.modelSha256 || "" }); } catch (e) { toast("✗ " + e, "err"); }
    refreshSttStatus();
  });
  document.querySelector("#set-stt-refresh").addEventListener("click", async () => {
    settings.sttRegistry = els.sttRegistryInput.value.trim();
    await loadSttModels();
    fillSttModelSelect();
  });
  document.querySelector("#settings-save").addEventListener("click", saveSettingsFromForm);
  // Tabs in het instellingen-menu + hover-uitleg per instelling.
  els.settingsModal.querySelectorAll(".stab").forEach((b) => b.addEventListener("click", () => switchSettingsTab(b.dataset.tab)));
  els.settingsModal.addEventListener("mouseover", (e) => { const row = e.target.closest("[data-help-key]"); if (row) showHelpTip(row); else hideHelpTip(); });
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
        && /^(https?:\/\/|mailto:)/i.test(d.url)) {
      window.__TAURI__.opener.openUrl(d.url).catch(() => {});
    }
  });

  loadSettings();
  applyI18n();
  wireGarble();
  wireFileDropper();
  loadSttModels();
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
