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
    launch_mode: "Modus",
    // Labels zeggen wat de modus DOET, niet hoe hij heet. De teksten volgen wat de
    // CLI er zelf over zegt (#130).
    mode_inherit: "Zoals in je eigen instellingen",
    mode_manual: "Vraagt het per stap",
    mode_accept_edits: "Bewerkt bestanden zelf, vraagt voor de rest",
    mode_plan: "Alleen plannen, voert niets uit",
    mode_auto: "Model beoordeelt elk verzoek",
    mode_dont_ask: "Vraagt nooit; wat niet vooraf mag, gaat niet door",
    mode_bypass: "Geen enkele controle (eenmalig te aanvaarden, beleid kan het blokkeren)",
    mode_default: "Standaard",
    mode_sandbox: "Sandbox (beperkte rechten)",
    launch_agent: "Agent", agent_claude: "Claude Code", agent_agy: "Antigravity",
    launch_model: "Model", model_ph: "standaard",
    model_hint: "Leeg = de standaard van de agent. Een alias volgt vanzelf het nieuwste model; een exact model-ID mag ook.",
    model_fable: "nieuwste Fable", model_opus: "nieuwste Opus", model_sonnet: "nieuwste Sonnet",
    model_haiku: "nieuwste Haiku", model_opusplan: "Opus in plan-modus, daarna Sonnet",
    launch_host: "Draait op", host_local: "Deze computer",
    remote_workdir: "Werkmap op de host", remote_workdir_ph: "bijv. C:\\Users\\arjen\\project of /home/arjen/project",
    remote_hint: "De agent draait op die machine; deze werkmap geldt daar.",
    remote_need_path: "Vul een werkmap op de host in.",
    remote_local_only: "Werkt alleen bij een sessie op deze computer",
    hosts_title: "Machines", host_manage: "Machines beheren…", host_add: "＋ Machine toevoegen…",
    host_nickname: "Naam", host_nickname_ph: "bijv. support01",
    host_hostname: "Hostnaam of IP", host_port: "Poort", host_user: "Gebruikersnaam",
    host_key: "SSH-key (leeg = ssh kiest zelf)", host_default_project: "Standaard werkmap op de host",
    host_add_test: "Toevoegen & testen", host_testing: "Verbinden…", host_retest: "Opnieuw testen",
    host_del: "Verwijderen", close: "Sluiten",
    machine_new: "Nieuwe agent…",
    machine_new_hint: "Open het startformulier met de werkmap van deze machine ingevuld.",
    machine_connect: "Verbinden",
    machine_no_default: "{machine} heeft geen standaard werkmap. Vul die in bij de machine — een sessie elders moet weten waar hij begint.",
    route_preferred: "voorkeur",
    machine_agents: "⇱ agents",
    machine_agents_hint: "Toon welke agents op deze machine draaien.",
    agents_none: "Geen agent aan het werk — niets om mee te verbinden. Start er een met ＋ Nieuwe agent.",
    agents_leftovers: "{n} lege sessie(s) die Taurus liet staan",
    agents_clean: "Opruimen",
    agent_local: "in Taurus daar",
    agent_local_hint: "Deze agent draait in de Taurus op die machine. Zichtbaar, maar er is nog geen kanaal om hem hiervandaan over te nemen.",
    session_attach: "Aanhaken",
    session_stop_hint: "Deze sessie beëindigen op de andere machine",
    session_stop_sure: "zeker weten?",
    persist_ask: "Vragen welke sessies je opent",
    persist_silent: "Stil hervatten wat openstond",
    persist_clean: "Schoon beginnen",
    restore_title: "Vorige sessies openen?",
    restore_lead: "Aangevinkt is wat openstond toen Taurus sloot. Daaronder staat wat er eerder is geweest — die blijft bewaard, ook als je hem nu niet opent.",
    restore_none: "Niets openen", restore_go: "Openen",
    restore_more: "De {read} nieuwste van {total} gesprekken die Claude nog heeft.",
    resume_no_host: "machine bestaat niet meer",
    resume_no_transcript: "geen transcript gevonden",
    ago_now: "zojuist", ago_min: "{n} min geleden", ago_hour: "{n} uur geleden", ago_day: "{n} dagen geleden",
    hosts_known: "Bekende machines",
    found_title: "Iemand vraagt hulp",
    help_from: "{user} op {machine} vraagt hulp bij",
    help_join: "Meedoen",
    help_asked: "Je vraag om hulp bij {title} staat op het netwerk.",
    help_asking: "Je vraagt hulp bij {title} — zichtbaar op het vertrouwde netwerk.",
    help_withdraw: "Intrekken",
    help_answered_toast: "Er is iemand meegekomen in je sessie.",
    ctx_help: "✋ Vraag om hulp bij deze agent",
    found_firewall: "Taurus heeft nog geen eigen firewall-uitzondering. Zonder die regels ziet niemand je, en kan niemand aankloppen.",
    found_firewall_fix: "Firewall-regels aanmaken (vraagt om beheerdersrechten)",
    found_firewall_blocked: "Windows blokkeert taurus.exe met {n} eigen regel(s) - waarschijnlijk van een weggeklikte Defender-vraag. Zo'n blokkade wint van elke uitzondering, dus die moet eerst weg.",
    found_firewall_unblock: "Blokkade weghalen en regels aanmaken (vraagt om beheerdersrechten)",
    found_firewall_busy: "Bezig - dit duurt een paar seconden…",
    host_need_fields: "Naam, hostnaam en gebruikersnaam zijn verplicht.",
    host_none: "Nog geen machines. Voeg er een toe om een agent elders te draaien.",
    host_ok: "Verbinding gelukt", host_reachable: "bereikbaar", host_unreachable: "onbereikbaar",
    host_no_claude: "⚠ Geen agent-CLI gevonden op deze machine — een sessie zal niet starten.",
    host_no_outbound: "⚠ Geen uitgaand HTTPS naar api.anthropic.com — een agent kan hier niet werken.",
    host_no_mux: "ℹ Geen herdr of tmux: een sessie is niet opnieuw aan te haken. Installeer herdr (herdr.dev) op die machine — dat werkt ook op Windows.",
    host_mux: "Sessiepersistentie",
    host_mux_auto: "Automatisch — wat de test vindt",
    host_mux_none: "Geen — sessies niet bewaren",
    host_mux_found: "gevonden: {list} — Taurus kiest {best}",
    host_mux_missing: "Deze machine heeft geen {mux}. Kies Automatisch, of installeer het daar en test opnieuw.",
    host_herdr_tuned: "✓ herdr's eigen sidebar en tabbalk uitgezet op deze machine — die zijn dubbelop in een Taurus-tab. Geldt vanaf de volgende sessie.",
    host_herdr_tune_failed: "ℹ herdr's sidebar kon niet uitgezet worden ({err}). De tab werkt gewoon, maar toont herdr's eigen menu ernaast.",
    attach_open: "Openen",
    attach_menu: "Verder werken…",
    attach_title: "Verder werken",
    attach_lead: "Alleen jouw eigen computer en de machines die je zelf hebt ingericht.",
    attach_local: "Op deze computer", attach_mine: "Op jouw machines",
    attach_no_local: "Geen eerdere sessies op deze computer.",
    agents_none_short: "geen agent",
    attach_refresh: "↻ Opnieuw ophalen",
    attach_loading: "Sessies ophalen…",
    attach_no_hosts: "Nog geen machines. Voeg er een toe met 🖥.",
    attach_not_restartable: "Aangehaakte sessie: Taurus heeft dit commando niet gebouwd en kan het niet herstarten of verplaatsen.",
    dropper_remote_hint: "De agent draait elders: bestanden gaan met scp naar de input-map op die machine.",
    dropper_sending: "Bestand overzetten naar de host…",
    dropper_sent: "Op de host gezet",
    dropper_paste_local_only: "Plakken uit het klembord werkt alleen bij een sessie op deze computer.",
    launch_command: "Commando-override", command_ph: "leeg = start de gekozen agent",
    command_hint: "Draait dit programma zoals het er staat, in plaats van de agent.",
    command_warn: "⚠ Agent-vlaggen gelden niet: model, modus en taak worden niet meegestuurd.",
    cap_agent: "Agent", cap_model: "Model (leeg = standaard)",
    cap_command: "Commando-override — draait dit programma i.p.v. de agent (optioneel)",
    row_expand: "Openklappen", row_collapse: "Dichtklappen",
    agent_command: "Eigen commando…",
    ctx_move: "⇄ Synchroniseer naar andere machine…",
    move_title: "Agent synchroniseren naar een andere machine", move_target: "Doelmachine",
    move_target_path: "Werkmap op de doelmachine", move_start: "Synchroniseren",
    move_surveying: "Werkmap doormeten…", move_core: "Projectbestanden (gaan altijd mee)",
    move_files: "bestanden", move_total: "Wordt overgezet", move_kind_work: "werkmap",
    move_kind_bulk: "opnieuw op te bouwen", move_copying: "Synchroniseren…",
    move_progress: "Synchroniseren — {name} ({n}/{total}, {pct}%)",
    move_done: "Agent gesynchroniseerd", move_need_path: "Vul een werkmap op de doelmachine in.",
    move_no_path: "Deze agent heeft geen werkmap.", move_no_target: "Voeg eerst een machine toe.",
    move_target_newer: "Daar staat al een map, en die is RECENTER bijgewerkt ({when}). Wat je aanvinkt vervangt wat daar staat. Weet je het zeker?",
    move_target_exists_info: "Daar staat al een map (laatst gewijzigd {when}). Wat je aanvinkt vervangt wat daar staat.",
    move_host_to_host_todo: "Van de ene machine naar de andere kan nog niet — haal hem eerst hierheen.",
    move_nothing_selected: "Er is niets aangevinkt om over te zetten.",
    move_keep_source: "De bronmap blijft staan — dit is een kopie.",
    cap_host: "Draait op", cap_workdir_remote: "Werkmap OP DIE MACHINE",
    ph_path_remote: "bijv. C:\\Users\\arjen\\project of /home/arjen/project",
    grp_comfort: "Terminal-comfort", comfort_hint: "(per voorkeur aan/uit)",
    c_copy: "Selectie kopieert automatisch", c_paste: "Rechtermuisklik plakt", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Klikbare links", c_links_new: "(nieuwe sessies)", c_search: "Zoeken in scrollback — Ctrl+Shift+F",
    c_tabs: "Tab-sneltoetsen (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude-status op de tab (✶ Orbiting…)",
    c_groups: "Tabs uit dezelfde map bundelen vanaf", c_groups_unit: "tabs",
    c_recap: "Recap tonen bij hover over een tab",
    grp_tab_members: "{n} sessies", grp_waiting: "wacht", grp_working: "bezig",
    grp_adhoc: "Losse sessies",
    recap_none: "(nog niets van deze agent te zien)", recap_exited: "afgesloten",
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
    grp_sessions: "Sessies", set_persist: "Bij opstarten",
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
    help_groups: "Vanaf dit aantal tabs worden sessies uit dezelfde map onder een tab gebundeld.\nHover (of klik) op zo'n tab om de sessies eronder uit te klappen.\nDe gebundelde tab flitst als een van zijn sessies op je wacht, dus je mist niets.",
    help_recap: "Toont bij hover het laatste wat die agent zei, ook van tabs die niet in beeld staan.\nGelezen uit het terminalvenster van die sessie zelf; er wordt niets naar de agent gestuurd.",
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
    tab_network: "Netwerk",
    grp_reachable: "Bereikbaar op het netwerk",
    ssh_enable: "Laat anderen op dit netwerk een sessie op deze computer starten",
    grp_network: "Netwerk",
    net_hint: "Alleen op een vertrouwd netwerk kan deze computer bereikbaar zijn. Wissel je van netwerk, dan gaat het vanzelf weer dicht.",
    ssh_need_trust: "vertrouw eerst een netwerk hierboven",
    ssh_port: "Poort",
    ssh_hint: "Een sessie draait als jouw Windows-account, met jouw rechten. Elke verbinding vraagt eerst toestemming; alles wordt vastgelegd in een audit-spoor.",
    ssh_on_lbl: "bereikbaar op poort {port}", ssh_off_lbl: "uit",
    ssh_blocked_lbl: "aan, maar dit netwerk is niet vertrouwd — er luistert niets",
    net_trust: "Vertrouw dit netwerk", net_none: "Geen netwerkverbinding gevonden.",
    net_cat_public: "openbaar netwerk", net_cat_private: "privé-netwerk", net_cat_domain: "domein",
    ssh_fp_lbl: "Vingerafdruk van deze computer",
    ssh_failed: "✗ Kan niet gaan luisteren:",
    grp_peers: "Gekoppelde computers",
    peers_hint: "Identiteit is de vingerafdruk van de sleutel, niet de naam — die verzint de client zelf.",
    peers_none: "Er is nog geen computer gekoppeld.",
    peer_blocked: "geblokkeerd", peer_auto: "vraagt nooit",
    peer_block: "Blokkeer", peer_unblock: "Deblokkeer", peer_forget: "Vergeet",
    consent_pair_title: "Nieuwe computer wil verbinden",
    consent_session_title: "Verzoek om een sessie",
    consent_who: "{user} op {address}",
    consent_fp: "Vingerafdruk",
    consent_warn: "Toestaan betekent dat deze computer als jouw account mag werken, met jouw rechten en credentials.",
    consent_remember: "Niet meer vragen voor deze computer",
    consent_full: "Vol beheer: de agent zonder rem",
    consent_full_warn: "Met vol beheer draait de agent op jouw account in zijn eigen modus, zonder dat er iemand meekijkt. Zonder het vinkje start hij in de map waar hij begint en vraagt hij het per stap. Dat is het permissiemodel van de agent, geen grens van het besturingssysteem.",
    consent_block: "Blokkeer", consent_deny: "Weiger", consent_join: "Meekijken", consent_allow: "Toestaan",
    consent_timer: "Weigert zichzelf over {secs} s",
    grp_inbound: "Draait nu op deze computer",
    inbound_none: "Er draait niets namens iemand anders.",
    inbound_joined: "je kijkt mee", inbound_stop: "Stop",
    join_tab: "Meekijken",
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
    launch_mode: "Mode",
    mode_inherit: "Whatever your own settings say",
    mode_manual: "Asks before each step",
    mode_accept_edits: "Edits files itself, asks for the rest",
    mode_plan: "Plans only, executes nothing",
    mode_auto: "A model judges every request",
    mode_dont_ask: "Never asks; anything not pre-approved does not happen",
    mode_bypass: "No checks at all (accept once; policy can block it)",
    mode_default: "Default",
    mode_sandbox: "Sandbox (restricted)",
    launch_agent: "Agent", agent_claude: "Claude Code", agent_agy: "Antigravity",
    launch_model: "Model", model_ph: "default",
    model_hint: "Empty = the agent's default. An alias always follows the newest model; an exact model ID works too.",
    model_fable: "newest Fable", model_opus: "newest Opus", model_sonnet: "newest Sonnet",
    model_haiku: "newest Haiku", model_opusplan: "Opus in plan mode, Sonnet after",
    launch_host: "Runs on", host_local: "This computer",
    remote_workdir: "Working directory on the host", remote_workdir_ph: "e.g. C:\\Users\\arjen\\project or /home/arjen/project",
    remote_hint: "The agent runs on that machine; this working directory applies there.",
    remote_need_path: "Enter a working directory on the host.",
    remote_local_only: "Only works for a session on this computer",
    hosts_title: "Machines", host_manage: "Manage machines…", host_add: "＋ Add machine…",
    host_nickname: "Name", host_nickname_ph: "e.g. support01",
    host_hostname: "Hostname or IP", host_port: "Port", host_user: "Username",
    host_key: "SSH key (empty = let ssh decide)", host_default_project: "Default working directory on the host",
    host_add_test: "Add & test", host_testing: "Connecting…", host_retest: "Test again",
    host_del: "Remove", close: "Close",
    machine_new: "New agent…",
    machine_new_hint: "Opens the launch form with this machine's working directory filled in.",
    machine_connect: "Connect",
    machine_no_default: "{machine} has no default working directory. Set one on the machine — a session elsewhere has to know where it starts.",
    route_preferred: "preferred",
    machine_agents: "⇱ agents",
    machine_agents_hint: "Show which agents are running on this machine.",
    agents_none: "No agent at work — nothing to connect to. Start one with ＋ New agent.",
    agents_leftovers: "{n} empty session(s) Taurus left behind",
    agents_clean: "Clean up",
    agent_local: "in Taurus there",
    agent_local_hint: "This agent runs in the Taurus on that machine. Visible, but there is no channel yet to take it over from here.",
    session_attach: "Attach",
    session_stop_hint: "End this session on the other machine",
    session_stop_sure: "are you sure?",
    persist_ask: "Ask which sessions to open",
    persist_silent: "Silently resume what was open",
    persist_clean: "Start clean",
    restore_title: "Reopen previous sessions?",
    restore_lead: "Ticked is what was open when Taurus closed. Below that is what came before — it stays in the history whether you open it now or not.",
    restore_none: "Open nothing", restore_go: "Open",
    restore_more: "The {read} most recent of {total} conversations Claude still has.",
    resume_no_host: "machine no longer exists",
    resume_no_transcript: "no transcript found",
    ago_now: "just now", ago_min: "{n} min ago", ago_hour: "{n} h ago", ago_day: "{n} days ago",
    hosts_known: "Known machines",
    found_title: "Someone is asking for help",
    help_from: "{user} on {machine} needs help with",
    help_join: "Join",
    help_asked: "Your request for help with {title} is on the network.",
    help_asking: "You are asking for help with {title} — visible on the trusted network.",
    help_withdraw: "Withdraw",
    help_answered_toast: "Someone has joined your session.",
    ctx_help: "✋ Ask for help with this agent",
    found_firewall: "Taurus has no firewall exception of its own yet. Without those rules nobody sees you, and nobody can knock.",
    found_firewall_fix: "Create firewall rules (asks for administrator rights)",
    found_firewall_blocked: "Windows blocks taurus.exe with {n} rule(s) of its own - most likely from a dismissed Defender prompt. A block beats any exception, so that has to go first.",
    found_firewall_unblock: "Remove the block and create the rules (asks for administrator rights)",
    found_firewall_busy: "Working - this takes a few seconds…",
    host_need_fields: "Name, hostname and username are required.",
    host_none: "No machines yet. Add one to run an agent elsewhere.",
    host_ok: "Connection succeeded", host_reachable: "reachable", host_unreachable: "unreachable",
    host_no_claude: "⚠ No agent CLI found on this machine — a session will not start.",
    host_no_outbound: "⚠ No outbound HTTPS to api.anthropic.com — an agent cannot work here.",
    host_no_mux: "ℹ No herdr or tmux: a session cannot be reattached. Install herdr (herdr.dev) on that machine — it works on Windows too.",
    host_mux: "Session persistence",
    host_mux_auto: "Automatic — whatever the test finds",
    host_mux_none: "None — do not keep sessions",
    host_mux_found: "found: {list} — Taurus picks {best}",
    host_mux_missing: "This machine has no {mux}. Pick Automatic, or install it there and test again.",
    host_herdr_tuned: "✓ Turned off herdr's own sidebar and tab bar on this machine — a Taurus tab already has both. Applies from the next session.",
    host_herdr_tune_failed: "ℹ Could not turn off herdr's sidebar ({err}). The tab still works; it just shows herdr's own menu beside the agent.",
    attach_open: "Open",
    attach_menu: "Continue working…",
    attach_title: "Continue working",
    attach_lead: "Only your own computer and the machines you configured yourself.",
    attach_local: "On this computer", attach_mine: "On your machines",
    attach_no_local: "No earlier sessions on this computer.",
    agents_none_short: "no agent",
    attach_refresh: "↻ Refresh",
    attach_loading: "Fetching sessions…",
    attach_no_hosts: "No machines yet. Add one with 🖥.",
    attach_not_restartable: "Attached session: Taurus did not build this command and cannot restart or move it.",
    dropper_remote_hint: "The agent runs elsewhere: files go to that machine's input folder over scp.",
    dropper_sending: "Copying file to the host…",
    dropper_sent: "Placed on the host",
    dropper_paste_local_only: "Pasting from the clipboard only works for a session on this computer.",
    launch_command: "Command override", command_ph: "empty = start the selected agent",
    command_hint: "Runs this program as-is, instead of the agent.",
    command_warn: "⚠ Agent flags do not apply: model, mode and task are not passed.",
    cap_agent: "Agent", cap_model: "Model (empty = default)",
    cap_command: "Command override — runs this program instead of the agent (optional)",
    row_expand: "Expand", row_collapse: "Collapse",
    agent_command: "Own command…",
    ctx_move: "⇄ Synchronize to another machine…",
    move_title: "Synchronize agent to another machine", move_target: "Target machine",
    move_target_path: "Working directory on the target machine", move_start: "Synchronize",
    move_surveying: "Measuring working directory…", move_core: "Project files (always included)",
    move_files: "files", move_total: "Will be transferred", move_kind_work: "work folder",
    move_kind_bulk: "rebuildable", move_copying: "Synchronizing…",
    move_progress: "Synchronizing — {name} ({n}/{total}, {pct}%)",
    move_done: "Agent synchronized", move_need_path: "Enter a working directory on the target machine.",
    move_no_path: "This agent has no working directory.", move_no_target: "Add a machine first.",
    move_target_newer: "A folder is already there, and it was changed MORE RECENTLY ({when}). What you tick replaces what is there. Are you sure?",
    move_target_exists_info: "A folder is already there (last changed {when}). What you tick replaces what is there.",
    move_host_to_host_todo: "Machine to machine is not supported yet — bring it here first.",
    move_nothing_selected: "Nothing is ticked to transfer.",
    move_keep_source: "The source folder stays where it is — this is a copy.",
    cap_host: "Runs on", cap_workdir_remote: "Working directory ON THAT MACHINE",
    ph_path_remote: "e.g. C:\\Users\\arjen\\project or /home/arjen/project",
    grp_comfort: "Terminal comfort", comfort_hint: "(toggle to taste)",
    c_copy: "Selection copies automatically", c_paste: "Right-click pastes", c_ctrl: "Ctrl+Shift+C / Ctrl+Shift+V",
    c_links: "Clickable links", c_links_new: "(new sessions)", c_search: "Search scrollback — Ctrl+Shift+F",
    c_tabs: "Tab shortcuts (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W)", c_status: "Live Claude status on the tab (✶ Orbiting…)",
    c_groups: "Group tabs from the same folder from", c_groups_unit: "tabs",
    c_recap: "Show a recap when hovering a tab",
    grp_tab_members: "{n} sessions", grp_waiting: "waiting", grp_working: "working",
    grp_adhoc: "Ad-hoc sessions",
    recap_none: "(nothing from this agent yet)", recap_exited: "exited",
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
    grp_sessions: "Sessions", set_persist: "On startup",
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
    help_groups: "From this many tabs, sessions from the same folder collapse into one tab.\nHover (or click) such a tab to expand its sessions below it.\nThe grouped tab flashes when one of its sessions is waiting for you, so nothing is missed.",
    help_recap: "On hover, shows the last thing that agent said — including tabs that are not on screen.\nRead from that session's own terminal view; nothing is sent to the agent.",
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
    tab_network: "Network",
    grp_reachable: "Reachable on the network",
    ssh_enable: "Let others on this network start a session on this computer",
    grp_network: "Network",
    net_hint: "This computer can only be reachable on a trusted network. Switch networks and it closes again by itself.",
    ssh_need_trust: "trust a network above first",
    ssh_port: "Port",
    ssh_hint: "A session runs as your Windows account, with your rights. Every connection asks permission first; everything is recorded in an audit trail.",
    ssh_on_lbl: "reachable on port {port}", ssh_off_lbl: "off",
    ssh_blocked_lbl: "on, but this network is not trusted — nothing is listening",
    net_trust: "Trust this network", net_none: "No network connection found.",
    net_cat_public: "public network", net_cat_private: "private network", net_cat_domain: "domain",
    ssh_fp_lbl: "This computer's fingerprint",
    ssh_failed: "✗ Could not start listening:",
    grp_peers: "Paired computers",
    peers_hint: "Identity is the key's fingerprint, not the name — the client makes that up itself.",
    peers_none: "No computer has been paired yet.",
    peer_blocked: "blocked", peer_auto: "never asks",
    peer_block: "Block", peer_unblock: "Unblock", peer_forget: "Forget",
    consent_pair_title: "New computer wants to connect",
    consent_session_title: "Session request",
    consent_who: "{user} at {address}",
    consent_fp: "Fingerprint",
    consent_warn: "Allowing means this computer may work as your account, with your rights and credentials.",
    consent_remember: "Don't ask again for this computer",
    consent_full: "Full control: the agent with no brake",
    consent_full_warn: "Full control runs the agent on your account in its own mode, with nobody watching. Without the tick it starts in the folder it begins in and asks before each step. That is the agent's permission model, not a boundary of the operating system.",
    consent_block: "Block", consent_deny: "Deny", consent_join: "Join", consent_allow: "Allow",
    consent_timer: "Denies itself in {secs} s",
    grp_inbound: "Running on this computer right now",
    inbound_none: "Nothing is running on someone else's behalf.",
    inbound_joined: "you are watching", inbound_stop: "Stop",
    join_tab: "Joined session",
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
// Draait dit exemplaar op een eigen configmap (TAURUS_CONFIG_DIR)? Dan hoort
// dat in de titelbalk: twee identiek ogende vensters waarvan er een je echte
// sessies draait, is vragen om in het verkeerde te typen.
async function markTestInstance() {
  let test = false;
  try { test = await invoke("is_test_instance"); } catch (_) { return; }
  if (!test) return;
  const merk = (s) => (s.includes("⚗") ? s : `${s}  ⚗ TEST`);
  document.title = merk(document.title || "Taurus");
  try {
    const w = window.__TAURI__.window.getCurrentWindow();
    w.setTitle(merk(await w.title()));
  } catch (_) {}
}

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
  // Een testexemplaar (eigen configmap) merken we HIER, want branding zet de
  // venstertitel na de start opnieuw en wist een markering uit Rust.
  markTestInstance();
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

// Welke agents hun lijst bij de CLI mogen ophalen. claude hoort hier niet thuis:
// aliassen verouderen niet, en `claude` heeft geen list-commando.
// agy geeft bij piped uitvoer slugs ("gemini-3.6-flash-low") en in een terminal
// labels ("Gemini 3.6 Flash (Low)"); we krijgen dus slugs. Dat mag: geverifieerd
// dat agy een slug zelf naar het label resolvet (`agy --model
// gemini-3.6-flash-low` logt `Propagating selected model override to backend:
// label="Gemini 3.6 Flash (Low)"`). Dat moest expliciet worden nagemeten omdat
// agy een ONBEKEND --model zonder foutmelding slikt en stil op het default-model
// terugvalt -- een verkeerde vorm zou dus nooit zichtbaar falen (#92).
const LIVE_MODEL_AGENTS = new Set(["agy"]);

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

// Modus-opties verschillen per agent.
//
// claude 2.1.232 accepteert er ZES: acceptEdits, auto, bypassPermissions, manual,
// dontAsk, plan. Taurus bood er drie aan, met de oude woordenschat -- "default"
// heet in de CLI inmiddels "manual" (#130). GEMETEN: `default` wordt nog wél
// geaccepteerd, maar als niet-gedocumenteerde alias (`Default` en een onzinwaarde
// worden geweigerd met de lijst van zes). De labels hieronder komen letterlijk uit
// wat de CLI zelf over die modi zegt, niet uit een aanname.
//
// "default" blijft bestaan als opgeslagen waarde en betekent: GEEN vlag meesturen,
// dus de eigen instelling van de agent geldt. Dat is geen legacy-rest maar een
// echte keuze -- wie in zijn settings.json `defaultMode: acceptEdits` heeft staan,
// wil niet dat Taurus daar stilletjes overheen gaat.
//
// agy: geen --permission-mode, wel --sandbox (beperkt) en
// --dangerously-skip-permissions (auto). Waarden komen overeen met de mapping in
// build_command() in de Rust-backend.
const MODE_OPTIONS = {
  claude: [
    { value: "default", key: "mode_inherit" },
    { value: "manual", key: "mode_manual" },
    { value: "acceptEdits", key: "mode_accept_edits" },
    { value: "plan", key: "mode_plan" },
    { value: "auto", key: "mode_auto" },
    { value: "dontAsk", key: "mode_dont_ask" },
    { value: "bypassPermissions", key: "mode_bypass" },
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
  // Tabs bundelen zodra het er te veel worden (#90). Onder de drempel verandert
  // er niets; daarboven vouwen sessies uit dezelfde bron samen.
  tabGroups: true, tabGroupAt: 10, tabRecap: true,
  fullPaths: true,
  // Drie standen sinds #129: ask (default) / silent / clean. Het oude
  // persistSessions-vinkje wordt nog gelezen zodat een bestaande config klopt.
  persistMode: "ask",
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
// Waar draait deze agent? De machine is een eigenschap van de AGENT, niet iets
// dat je per keer kiest -- een kaart is een precieze werkplek, en die ligt op
// een machine net zo goed als in een map.
function agentLocTag(p) {
  const h = p.host_id ? hostById(p.host_id) : null;
  // De MACHINE op de kaart, niet de route: hoe Taurus daar binnenkomt is plumbing.
  if (h) return { text: `(${machineLabel(p.host_id)})`, cls: "remote", title: `${h.user}@${h.hostname}` };
  return { text: driveTag(p.path), cls: locClass(p.path), title: locText(p.path) };
}
function escapeHtml(s) {
  return String(s).replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
}
function modalOpen() { return !!document.querySelector(".modal:not(.hidden)"); }

// Een commando-override draait dat programma zoals het er staat: create_session
// slaat build_command dan volledig over, dus --model, de modus-mapping en de taak
// gaan niet mee. Dat was onzichtbaar -- de velden bleven bewerkbaar en leken te
// gelden, wat een agent stil op zijn default-model liet draaien (#93). Grijs ze
// uit en toon waarom, zodra er een override staat.
function applyOverrideState(command, fields, warnEl) {
  const on = !!(command || "").trim();
  for (const el of fields) if (el) el.disabled = on;
  if (warnEl) warnEl.classList.toggle("hidden", !on);
  return on;
}
// Het startformulier: de override zit achter de agent-keuze "Eigen commando…",
// zodat het veld niet altijd in beeld staat voor iets dat zelden gebruikt wordt.
// Model, modus en taak volgen dan de inhoud van dat veld.
function refreshOverrideState() {
  const chosen = els.agentInput.value === AGENT_COMMAND;
  els.commandField.classList.toggle("hidden", !chosen);
  applyOverrideState(chosen ? els.commandInput.value : "", [els.modelInput, els.modeInput, els.taskInput], els.commandWarn);
}
// Welke agent hoort er in het formulier te staan? Een project met een override
// toont "Eigen commando…", ook al staat er claude/agy in projects.json.
function agentChoiceFor(p) {
  if ((p.command || "").trim()) return AGENT_COMMAND;
  return AGENTS.includes(p.agent) ? p.agent : "claude";
}

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
  // Met gebundelde tabs (#90) staat niet elke sessie los in de balk. Een
  // groepstab vertegenwoordigt zijn leden, dus die vullen we op zijn plek weer
  // in. Zonder dat klopt het aantal niet meer met sessions.size en zou de guard
  // hieronder de hele herordening stil laten vallen.
  const ids = [];
  for (const c of els.tabbar.children) {
    if (c.dataset.tabId) { ids.push(c.dataset.tabId); continue; }
    const key = c.dataset.groupKey;
    if (!key) continue;
    for (const s of sessions.values()) if (tabGroupKey(s) === key) ids.push(s.id);
  }
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
    const loc = agentLocTag(p);
    card.innerHTML =
      `<div class="pc-label">${escapeHtml(p.label)} <span class="pc-drive ${loc.cls}" title="${escapeHtml(loc.title)}">${escapeHtml(loc.text)}</span></div>` +
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
    // Verplaatsen gaat over de AGENT, dus het hoort ook hier te kunnen -- niet
    // alleen op een draaiende tab.
    card.addEventListener("contextmenu", (e) => { e.preventDefault(); openMoveModal(p); });
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
  els.agentInput.value = agentChoiceFor(p);
  els.modelInput.value = p.model || "";
  updateModelDatalist(els.modelSuggestions, els.agentInput.value);
  fillModeSelect(els.modeInput, els.agentInput.value, p.mode || "default");
  els.commandInput.value = p.command || "";
  refreshOverrideState();
  els.status.textContent = "";
  await refreshHostState(); // doet de lokale pad-checks, of slaat ze over bij remote
}
async function loadProjects() { projects = await invoke("get_projects"); renderProjects(); }

/* ============ remote hosts (#98) ============ */
// De machines waarop een tab een agent kan draaien. Leeg = alleen lokaal, en dan
// gedraagt de app zich exact zoals voorheen.
let hosts = [];
// Eén regel per FYSIEKE machine, met de routes eronder (#124). De backend groepeert,
// want daar hoort ook de regel welke route voorkeur heeft.
let machines = [];
async function loadHosts() {
  try { hosts = await invoke("get_hosts"); } catch (_) { hosts = []; }
  await refreshMachines();
  fillHostSelect();
}
async function refreshMachines() {
  try { machines = await invoke("machines"); } catch (_) { machines = []; }
}
function hostById(id) { return hosts.find((h) => h.id === id) || null; }

// De poort waarop Taurus zelf luistert (#121). Een route daarheen vraagt geen
// sleuteluitwisseling en geen sshd aan de andere kant.
const TAURUS_PORT = 8287;

// Bij welke machine hoort deze route? Een agentkaart bewaart een route-id, maar
// overal waar een naam wordt GETOOND hoort de machinenaam te staan: welke route
// het is, is plumbing, en "(Taurus-host)" lekte zo door naar tabs en kaarten.
function machineOf(hostId) {
  return machines.find((m) => m.routes.some((r) => r.id === hostId)) || null;
}
function machineLabel(hostId) {
  const m = machineOf(hostId);
  if (m) return m.label;
  const h = hostById(hostId);
  return h ? (h.nickname || h.hostname) : "";
}
// Hoe je de machine bereikt, kort genoeg voor op één regel. De poort maakt het
// onderscheid; de bijnaam hoeft het niet meer te dragen.
function routeLabel(h) {
  const soort = h.port === TAURUS_PORT ? "Taurus" : h.via === "wsl" ? "WSL" : "sshd";
  return `${soort} :${h.port || 22}`;
}
// Keuzelijsten tonen machines, geen routes: drie regels "ursu" is geen keuze maar
// een raadsel. De waarde blijft een route-id, want dat is wat een kaart bewaart --
// de voorkeursroute, tenzij de kaart al een andere route van diezelfde machine
// gebruikt. Die dan omschrijven levert niets op en zou een werkende kaart raken.
function machineOptions(currentHostId) {
  return machines.map((m) => {
    const eigen = m.routes.find((r) => r.id === currentHostId);
    return { id: eigen ? eigen.id : m.preferred, label: m.label };
  });
}
// De hostlijst is veranderd: de agentkaarten tonen hun machine, en een open
// editor moet de nieuwe keuzes tonen.
function fillHostSelect() {
  renderProjects();
  if (els.editorModal && !els.editorModal.classList.contains("hidden")) renderEditor();
}
/* ---- host-modal: kiezen, toevoegen, testen ---- */
let hostStatus = {}; // id -> {reachable, ms}, gevuld door check_hosts

function openHostModal() {
  els.hfForm.classList.add("hidden");
  els.hostStatusMsg.textContent = "";
  els.hfReport.classList.add("hidden");
  renderHostRows();
  els.hostModal.classList.remove("hidden");
  refreshHostReachability();
  startDiscovery();
}

// Eén plek om te sluiten, want zoeken hoort te stoppen zodra het scherm dicht is:
// discovery is passief en dat is precies wat dat betekent (#125).
function closeHostModal() {
  els.hostModal.classList.add("hidden");
  stopDiscovery();
}
// De dots vullen zich ná de eerste render: check_hosts doet alle hosts naast
// elkaar, dus de modal is meteen zichtbaar i.p.v. te wachten op de traagste.
async function refreshHostReachability() {
  if (!hosts.length) return;
  try {
    const list = await invoke("check_hosts", { hosts: hosts.map((h) => ({ id: h.id, hostname: h.hostname, port: h.port || 22 })) });
    // Alleen overschrijven wat de check daadwerkelijk teruggaf. De hele map
    // wissen liet een net handmatig geteste host weer op grijs staan, en bij een
    // mislukte check verdween ALLE status -- dan lijkt niets meer gemeten.
    for (const s of list) hostStatus[s.id] = s;
  } catch (e) {
    machineFout(e);
  }
  renderHostRows();
}
// Het bolletje van de machine vat zijn routes samen: bereikbaar zodra ÉÉN route
// het doet, want dan is de machine bereikbaar -- dat is wat de regel beweert.
function machineDotClass(m) {
  const sts = m.routes.map((r) => hostStatus[r.id]);
  if (sts.some((s) => s && !s.testing && s.reachable)) return "up";
  if (sts.length && sts.every((s) => s && !s.testing)) return "down";
  return "pending";
}

function renderHostRows() {
  if (!machines.length) {
    els.hostRows.innerHTML = `<div class="host-empty">${escapeHtml(t("host_none"))}</div>`;
    return;
  }
  els.hostRows.innerHTML = "";
  for (const m of machines) {
    const pref = hostById(m.preferred) || m.routes[0];
    const box = document.createElement("div");
    box.className = "machine";
    box.innerHTML = `
      <div class="machine-head">
        <span class="host-dot ${machineDotClass(m)}"></span>
        <div class="host-main">
          <div class="host-name">${escapeHtml(m.label)}</div>
          <div class="host-sub">${escapeHtml((pref.user ? pref.user + "@" : "") + pref.hostname)} · ${escapeHtml(pref.os || "?")}</div>
        </div>
        <button class="machine-sess-toggle" title="${escapeHtml(t("machine_agents_hint"))}">${escapeHtml(t("machine_agents"))}</button>
        <button class="machine-connect" title="${escapeHtml(t("machine_new_hint"))}">${escapeHtml(t("machine_new"))}</button>
      </div>
      <div class="machine-routes"></div>`;
    const routes = box.querySelector(".machine-routes");
    for (const r of m.routes) {
      const st = hostStatus[r.id];
      const cls = !st || st.testing ? "pending" : st.reachable ? "up" : "down";
      const label = !st ? "…" : st.testing ? t("host_testing") : st.reachable ? `${t("host_reachable")} (${st.ms} ms)` : t("host_unreachable");
      const row = document.createElement("div");
      row.className = "route-row";
      // De voorkeur alleen benoemen als er iets te kiezen viel: bij één route is
      // "voorkeur" een woord zonder alternatief.
      row.innerHTML = `
        <span class="host-dot ${cls}" title="${escapeHtml(label)}"></span>
        <span class="route-name">${escapeHtml(routeLabel(r))}</span>
        ${r.id === m.preferred && m.routes.length > 1 ? `<span class="route-pref">${escapeHtml(t("route_preferred"))}</span>` : ""}
        <span class="route-mux">${escapeHtml([r.os, r.mux || "none"].filter(Boolean).join(" · "))}</span>
        <button class="host-test" title="${escapeHtml(t("host_retest"))}">↻</button>
        <button class="host-del" title="${escapeHtml(t("host_del"))}">🗑</button>`;
      row.querySelector(".host-test").addEventListener("click", () => testExistingHost(hosts.findIndex((h) => h.id === r.id)));
      row.querySelector(".host-del").addEventListener("click", async () => {
        hosts = hosts.filter((h) => h.id !== r.id);
        await invoke("save_hosts", { hosts });
        await refreshMachines();
        fillHostSelect();
        renderHostRows();
      });
      routes.appendChild(row);
    }
    box.querySelector(".machine-connect").addEventListener("click", () => newAgentOnMachine(m));
    box.querySelector(".machine-sess-toggle").addEventListener("click", () => toggleMachineAgents(m));
    renderMachineAgents(box, m);
    els.hostRows.appendChild(box);
  }
}

// Een fout uit het machinescherm hoort ZICHTBAAR te zijn. Hij ging naar
// `#hf-status`, en dat element zit in het "machine toevoegen"-formulier dat
// standaard dicht staat -- dus een mislukte join schreef netjes een melding in een
// verborgen doosje en voelde als een knop die niets doet.
function machineFout(e) {
  const msg = String(e && e.message ? e.message : e);
  toast("✗ " + msg, "err");
  if (els.hostStatusMsg) {
    els.hostStatusMsg.textContent = "✗ " + msg;
    els.hostStatusMsg.className = "status-msg err";
  }
}

/* ---- de hand opsteken (#125) ---- */
// Vragen wijst naar ÉÉN agent, en zolang de hand omhoog is ben je zichtbaar op het
// vertrouwde netwerk -- zoals bluetooth in koppelmodus. Laat je hem zakken, dan is
// er niets meer te zien.
//
// Het werk blijft van jou: wie komt helpen leest mee in DEZE terminal en typt erin
// mee. Hij neemt niets over en niets verhuist.
let asking = null;

async function askForHelp(s) {
  try {
    asking = await invoke("help_ask", { session: s.id, title: s.title || "agent", cwd: s.path || "" });
  } catch (e) {
    toast(String(e), "err");
    return;
  }
  toast(t("help_asked").replace("{title}", s.title || "agent"));
  renderAskingBanner();
}

async function withdrawHelp() {
  await invoke("help_withdraw").catch(() => {});
  asking = null;
  renderAskingBanner();
}

// Er is iemand gekomen. De backend heeft de vraag dan al ingenomen -- het token is
// eenmalig -- maar de balk wist dat niet en bleef staan. Dan lijkt het alsof je nog
// steeds vraagt terwijl er niets meer wordt aangekondigd, en een volgende vraag
// voelt als "hij doet het niet".
listen("help-answered", () => {
  asking = null;
  renderAskingBanner();
  toast(t("help_answered_toast"));
});

// En bij het opstarten: de balk hoort te kloppen met wat de backend werkelijk nog
// open heeft staan, niet met wat er toevallig in het venster stond.
async function syncAskingBanner() {
  try { asking = await invoke("help_asking"); } catch (_) { asking = null; }
  renderAskingBanner();
}

// Eén rustige balk onder de tabbalk. Geen popup: je hebt het zelf aangezet, dus het
// hoeft je niet te onderbreken -- het moet alleen niet te vergeten zijn.
function renderAskingBanner() {
  let el = document.querySelector("#asking-banner");
  if (!asking) { if (el) el.remove(); return; }
  if (!el) {
    el = document.createElement("div");
    el.id = "asking-banner";
    el.className = "asking-banner";
    els.tabbar.parentElement.insertAdjacentElement("afterend", el);
  }
  el.innerHTML = `
    <span>✋</span>
    <span class="grow">${escapeHtml(t("help_asking").replace("{title}", asking.title))}</span>
    <button class="btn-ghost">${escapeHtml(t("help_withdraw"))}</button>`;
  el.querySelector("button").onclick = withdrawHelp;
}

/* ---- machines vinden op het vertrouwde netwerk (#125) ---- */
// Zoeken loopt ALLEEN zolang dit scherm openstaat. Geen melding, geen badge, geen
// popup: een aankondiging is omgevingsgeluid. Alleen een verzoek om toegang mag
// onderbreken, anders verdrinkt de popup die wél een antwoord nodig heeft.
let discoTimer = null;
let found = [];
let discoNote = "";
let firewall = null;

async function startDiscovery() {
  if (discoTimer) return;
  found = []; discoNote = "";
  try {
    await invoke("discovery_start");
  } catch (e) {
    discoNote = String(e);
  }
  await pollDiscovery();
  discoTimer = setInterval(pollDiscovery, 1500);
  // Eén keer per opening: kunnen we überhaupt gevonden worden? Dit duurt seconden
  // (het loopt door alle firewallregels heen), dus het mag de lijst niet ophouden.
  invoke("firewall_status", { port: null })
    .then((s) => { firewall = s; renderFound(); })
    .catch(() => {});
}

function stopDiscovery() {
  if (discoTimer) { clearInterval(discoTimer); discoTimer = null; }
  // Ook de lijst leegmaken. Doe je dat niet, dan staat een verzoek dat je zojuist
  // beantwoord hebt er bij het volgende openen nog -- en dat token is op, dus die
  // tweede klik levert alleen een foutmelding op.
  found = [];
  const rows = document.querySelector("#found-rows");
  if (rows) { rows.innerHTML = ""; rows.dataset.sig = ""; }
  const wrap = document.querySelector("#found-wrap");
  if (wrap) wrap.classList.add("hidden");
  invoke("discovery_stop").catch(() => {});
}

async function pollDiscovery() {
  let view;
  try {
    view = await invoke("discovered_machines");
  } catch (e) {
    discoNote = String(e);
    renderFound();
    return;
  }
  // ALLES wat vraagt, ook van een machine die je al kent. Dat filter ("toon geen
  // machine die je al hebt") is een restant van de oude aanwezigheids-discovery en
  // is bij een hulpvraag precies verkeerd: dat ursu in je hosts.json staat is geen
  // reden om zijn opgestoken hand niet te tonen. GEMETEN: de backend gaf het
  // verzoek netjes terug en het scherm bleef leeg.
  found = view.machines;
  discoNote = view.problem || "";
  // Niet meer "je bent niet vindbaar": in de vraagmodus kondig je alleen aan als je
  // zelf iets vraagt, dus zwijgen is de normale toestand. Wat wél de moeite waard
  // is om te melden staat hieronder (firewall), en anders is een lege lijst een
  // waar antwoord: niemand heeft hulp nodig.
  renderFound();
}

// Handtekening van wat er nu staat. Zolang die niet verandert, blijft de DOM staan.
function foundSignature() {
  return found.map((f) => `${f.name}|${f.token}|${f.agentTitle}`).join("~");
}

function renderFound() {
  const wrap = document.querySelector("#found-wrap");
  const rows = document.querySelector("#found-rows");
  const note = document.querySelector("#found-note");
  if (!wrap || !rows || !note) return;
  wrap.classList.toggle("hidden", !found.length);
  renderFoundNote(note);
  // NIET elke ronde opnieuw opbouwen. De poll loopt elke 1,5 s, en wie de knop
  // precies dan indrukt klikt op een element dat tussen muis-neer en muis-op
  // vervangen is -- er gebeurt dan helemaal niets, zonder enige melding. Alleen
  // hertekenen als de verzameling verzoeken echt anders is.
  const sig = foundSignature();
  if (rows.dataset.sig === sig) return;
  rows.dataset.sig = sig;
  rows.innerHTML = "";
  // Een verzoek, geen machine. Er staat wie het vraagt, waarbij, en waar dat werk
  // staat -- zonder die agent zou je uitkomen "op een computer", en dan kan het
  // antwoord een kale prompt zijn.
  for (const f of found) {
    const row = document.createElement("div");
    row.className = "askband";
    row.innerHTML = `
      <span class="askband-hand">✋</span>
      <div class="host-main">
        <div class="host-name">${escapeHtml(t("help_from").replace("{user}", f.user || "?").replace("{machine}", f.name))} <b>${escapeHtml(f.agentTitle || "?")}</b></div>
        <div class="host-sub">${escapeHtml([f.agentCwd, f.fingerprint].filter(Boolean).join(" · "))}</div>
      </div>
      <button class="machine-connect">${escapeHtml(t("help_join"))}</button>`;
    // De hele balk doet mee. Hij ziet eruit als één ding, dus alleen dat kleine
    // knopje laten werken is een val: je klikt ernaast en er gebeurt niets.
    row.addEventListener("click", () => joinHelpRequest(f));
    rows.appendChild(row);
  }
}

// Eén eerlijke regel wanneer er niets te zien is, in plaats van een lege lijst die
// als "er is niemand" leest. Een firewall die ons tegenhoudt is iets anders dan een
// netwerk waar niemand hulp nodig heeft.
function renderFoundNote(note) {
  const lines = [];
  if (discoNote) lines.push(discoNote);
  const fwBlocked = !!(firewall && firewall.checked && firewall.blocked > 0);
  const fwMissing = !!(firewall && firewall.checked && !(firewall.tcp && firewall.udp));
  if (fwBlocked) lines.push(t("found_firewall_blocked").replace("{n}", firewall.blocked));
  if (fwMissing) lines.push(t("found_firewall"));
  note.innerHTML = lines.map((l) => `<div>${escapeHtml(l)}</div>`).join("");
  if (fwBlocked || fwMissing) {
    const b = document.createElement("button");
    b.className = "btn-ghost add";
    b.textContent = t(fwBlocked ? "found_firewall_unblock" : "found_firewall_fix");
    b.addEventListener("click", async () => {
      b.disabled = true;
      b.textContent = t("found_firewall_busy");
      try {
        await invoke("firewall_allow", { port: null });
      } catch (e) {
        machineFout(e);
      }
      try { firewall = await invoke("firewall_status", { port: null }); } catch (_) {}
      b.disabled = false;
      renderFound();
    });
    note.appendChild(b);
  }
  note.classList.toggle("hidden", !lines.length);
}

// Meedoen met een hulpvraag. Geen kaart, geen machine erbij in hosts.json: iemand
// helpen maakt zijn computer nog niet tot een van jouw machines. Je landt in ZIJN
// terminal en typt daarin mee.
async function joinHelpRequest(f) {
  const id = "s" + (++seq);
  const session = spawnTerminal({
    id, uuid: "", path: f.agentCwd || "", title: f.agentTitle || f.name, accent: "#7c9cff",
    mode: "", command: "", agent: "", model: "", hostId: "", projectId: "",
  });
  // Taurus heeft dit commando niet gebouwd en kan het niet hervatten of verplaatsen.
  session.attached = true;
  try {
    await invoke("answer_help_request", {
      id, gen: session.gen, found: f,
      cols: session.term.cols, rows: session.term.rows,
    });
    closeHostModal();
    showView(id);
  } catch (e) {
    sessions.delete(id); session.term.dispose(); session.el.remove();
    renderTabs();
    machineFout(e);
  }
}

/* ---- welke AGENTS er op een machine draaien (#128) ---- */
// DE REGEL: Taurus toont agents. ssh, tmux en herdr maken de weg vrij zodat er een
// agent kan starten; ze zijn leidingwerk en nooit iets wat je kiest. Geen agent
// betekent dat er niets is om mee te verbinden -- geen keuze met een
// waarschuwingslabel, maar geen keuze. Lege sessies bestaan wel en staan daarom
// onderaan als opruimwerk.
//
// Pas op verzoek ophalen: dit zijn twee ssh-rondes per machine. Bij het openen alle
// machines bevragen zou het traagste antwoord de hele lijst laten wachten.
const machineAgents = {}; // machine-key -> { loading, view, error }

async function toggleMachineAgents(m) {
  if (machineAgents[m.key] && !machineAgents[m.key].loading) {
    delete machineAgents[m.key];
    renderHostRows();
    return;
  }
  machineAgents[m.key] = { loading: true, view: null, error: "" };
  renderHostRows();
  const host = hostById(m.preferred) || m.routes[0];
  try {
    const view = await invoke("remote_agents", { hostId: host.id });
    machineAgents[m.key] = { loading: false, view, error: "" };
  } catch (e) {
    machineAgents[m.key] = { loading: false, view: null, error: String(e) };
  }
  renderHostRows();
}

function renderMachineAgents(box, m) {
  const st = machineAgents[m.key];
  if (!st) return;
  const wrap = document.createElement("div");
  wrap.className = "machine-sess";
  const line = (txt, cls) => {
    const d = document.createElement("div");
    d.className = "route-row" + (cls ? " " + cls : "");
    d.textContent = txt;
    return d;
  };
  if (st.loading) { wrap.appendChild(line(t("attach_loading"))); box.appendChild(wrap); return; }
  if (st.error)   { wrap.appendChild(line("✗ " + st.error, "err")); box.appendChild(wrap); return; }

  const v = st.view || { agents: [], empty: [], taurusSeen: false };
  if (!v.agents.length) {
    // Eerlijk en kort: er valt hier niets te verbinden. Dat is een antwoord, geen
    // aanleiding om dan maar leidingwerk aan te bieden.
    wrap.appendChild(line(t("agents_none")));
  }
  for (const a of v.agents) {
    const row = document.createElement("div");
    row.className = "route-row";
    const meta = [a.agent, a.cwd].filter(Boolean).join(" · ");
    row.innerHTML = `
      <span class="host-dot ${a.status ? "up" : "pending"}"></span>
      <span class="route-name">${escapeHtml(a.title || a.session)}</span>
      ${a.status ? `<span class="route-pref">${escapeHtml(a.status)}</span>` : ""}
      <span class="route-mux">${escapeHtml(meta)}</span>
      ${a.attachable
        ? `<button class="host-test sess-open">${escapeHtml(t("session_attach"))}</button>
           <button class="host-del sess-stop" title="${escapeHtml(t("session_stop_hint"))}">✕</button>`
        : `<span class="sess-bare" title="${escapeHtml(t("agent_local_hint"))}">${escapeHtml(t("agent_local"))}</span>`}`;
    if (a.attachable) {
      row.querySelector(".sess-open").addEventListener("click", () => attachFromMachine(m, a));
      wireStopButton(row.querySelector(".sess-stop"), m, a.session);
    }
    wrap.appendChild(row);
  }
  // Opruimwerk onderaan, buiten de keuzes. Ze bestaan echt, dus verbergen zou
  // verwarrender zijn dan ze benoemen -- maar een klik erop opent niets.
  if (v.empty.length) {
    const row = document.createElement("div");
    row.className = "route-row";
    row.innerHTML = `
      <span class="host-dot pending"></span>
      <span class="route-mux">${escapeHtml(t("agents_leftovers").replace("{n}", v.empty.length))}</span>
      <button class="host-del sess-clean">${escapeHtml(t("agents_clean"))}</button>`;
    wireCleanButton(row.querySelector(".sess-clean"), m, v.empty);
    wrap.appendChild(row);
  }
  box.appendChild(wrap);
}

// Twee klikken: een sessie beëindigen is andermans werk afbreken, en dat mag geen
// uitschieter zijn. De knop zegt zelf wat de tweede klik doet.
function armTwice(btn, label, run) {
  btn.addEventListener("click", async () => {
    if (btn.dataset.armed !== "1") {
      btn.dataset.armed = "1";
      btn.dataset.was = btn.textContent;
      btn.textContent = t("session_stop_sure");
      btn.classList.add("armed");
      setTimeout(() => {
        if (!btn.isConnected) return;
        btn.dataset.armed = "";
        btn.textContent = btn.dataset.was || label;
        btn.classList.remove("armed");
      }, 4000);
      return;
    }
    btn.disabled = true;
    try {
      await run();
    } catch (e) {
      btn.disabled = false;
      machineFout(e);
      return;
    }
    renderHostRows();
  });
}

function wireStopButton(btn, m, session) {
  armTwice(btn, "✕", async () => {
    const host = hostById(m.preferred) || m.routes[0];
    await invoke("stop_remote_session", { hostId: host.id, session });
    const st = machineAgents[m.key];
    if (st && st.view) st.view.agents = st.view.agents.filter((x) => x.session !== session);
  });
}

function wireCleanButton(btn, m, names) {
  armTwice(btn, t("agents_clean"), async () => {
    const host = hostById(m.preferred) || m.routes[0];
    for (const n of names) {
      await invoke("stop_remote_session", { hostId: host.id, session: n });
    }
    const st = machineAgents[m.key];
    if (st && st.view) st.view.empty = [];
  });
}

// Aanhaken aan een agent die op die machine draait.
async function attachFromMachine(m, a) {
  const host = hostById(m.preferred) || m.routes[0];
  const id = "s" + (++seq);
  const session = spawnTerminal({
    id, uuid: "", path: a.cwd || "", title: a.title || a.session, accent: "#7c9cff",
    mode: "", command: "", agent: "", model: "", hostId: host.id, projectId: "",
  });
  session.attached = true;
  try {
    await invoke("attach_remote_session", {
      id, gen: session.gen, hostId: host.id, session: a.session,
      cols: session.term.cols, rows: session.term.rows,
    });
    closeHostModal();
    recordSession(session);
    showView(id);
  } catch (e) {
    sessions.delete(id); session.term.dispose(); session.el.remove();
    renderTabs();
    machineFout(e);
  }
}

// "Nieuwe agent…" op een machine: open het gewone startformulier met de map van die
// machine voorgevuld.
//
// Heette eerst "Connect", en dat was geen eerlijke naam. Het startte iets naamloos in
// een map die je niet gekozen had -- vandaar dat je in C:\Users\arjen terechtkwam
// terwijl je bij je werk wilde zijn. Nu kies je wat er start, net als lokaal, en zie
// je vooraf waar het begint.
async function newAgentOnMachine(m) {
  const host = hostById(m.preferred) || m.routes[0];
  if (!host) return;
  const path = host.default_project || "";
  if (!path) {
    els.hostStatusMsg.textContent = t("machine_no_default").replace("{machine}", m.label);
    els.hostStatusMsg.className = "status-msg err";
    return;
  }
  const leaf = path.replace(/[\\/]+$/, "").split(/[\\/]/).pop() || m.label;
  closeHostModal();
  await selectProject(
    {
      id: "", label: `${leaf} (${m.label})`, path, title: leaf, task: "",
      accent: "#7c9cff", mode: "default", command: "", agent: "claude", model: "",
      // De machine hoort bij deze start; het formulier toont de werkmap dan als
      // "op de host" en de bladerknop verdwijnt -- dat pad bestaat hier niet.
      host_id: host.id,
    },
    null
  );
  showView("new");
}

function openHostForm() {
  els.hfNickname.value = ""; els.hfHostname.value = ""; els.hfPort.value = "22";
  els.hfUser.value = ""; els.hfKey.value = ""; els.hfProject.value = ""; els.hfMux.value = "";
  els.hostStatusMsg.textContent = ""; els.hostStatusMsg.className = "status-msg";
  els.hfReport.classList.add("hidden");
  els.hfForm.classList.remove("hidden");
  els.hfNickname.focus();
}

// "Toevoegen & testen": de host wordt pas opgeslagen als de probe lukt, zodat
// hosts.json geen half-werkende entries verzamelt. De probe vult os en mux zelf
// in -- dat zijn eigenschappen van de machine, niet iets om met de hand te typen.
async function addAndTestHost() {
  const nickname = els.hfNickname.value.trim();
  const hostname = els.hfHostname.value.trim();
  const user = els.hfUser.value.trim();
  if (!nickname || !hostname || !user) {
    els.hostStatusMsg.textContent = t("host_need_fields");
    els.hostStatusMsg.className = "status-msg err";
    return;
  }
  const host = {
    id: uniqueHostId(slugify(nickname)),
    nickname, hostname, user,
    port: parseInt(els.hfPort.value, 10) || 22,
    key_path: els.hfKey.value.trim(),
    default_project: els.hfProject.value.trim(),
    // De WSL-route wordt niet meer aangeboden: sinds #115 geeft herdr op Windows
    // zelf persistentie, mét een werkende DROPZONE en zonder /mnt/c. Een
    // bestaande hosts.json met via:"wsl" blijft gewoon werken.
    via: "",
    os: "", mux: "", agent_version: "", mux_auto: !els.hfMux.value,
  };
  const wantMux = els.hfMux.value || "";
  els.hostStatusMsg.textContent = t("host_testing");
  els.hostStatusMsg.className = "status-msg";
  els.hfReport.classList.add("hidden");
  els.hfTest.disabled = true;
  let p;
  try { p = await invoke("probe_host", { host }); }
  catch (e) { p = { reachable: false, error: String(e) }; }
  finally { els.hfTest.disabled = false; }

  if (!p.reachable || !p.authOk) {
    els.hostStatusMsg.textContent = "✗ " + (p.error || t("host_unreachable"));
    els.hostStatusMsg.className = "status-msg err";
    return;
  }
  // Een expliciete keuze die de machine niet heeft, wordt niet opgeslagen: dat
  // zou pas bij de eerste sessie stukgaan, met een foutmelding uit een shell
  // drie lagen diep. "Geen" mag altijd -- dat is een geldige keuze, geen fout.
  const found = p.muxes || [];
  if (wantMux && wantMux !== "none" && !found.includes(wantMux) &&
      !(wantMux === "tmux" && found.includes("psmux"))) {
    els.hostStatusMsg.textContent = "✗ " + t("host_mux_missing").replace("{mux}", wantMux);
    els.hostStatusMsg.className = "status-msg err";
    showProbeReport(p);
    return;
  }
  host.os = p.os;
  // tmux in de keuzelijst dekt ook psmux: dat is dezelfde commandotaal, en welke
  // van de twee er staat is een eigenschap van de machine, geen keuze.
  host.mux = wantMux === "tmux" ? (found.includes("tmux") ? "tmux" : "psmux")
           : (wantMux || p.mux || "none");
  hosts.push(host);
  await invoke("save_hosts", { hosts });
  await refreshMachines();
  const tuned = await tuneHerdrChrome(host);
  fillHostSelect();
  renderHostRows();
  els.hostStatusMsg.textContent = "✓ " + t("host_ok");
  els.hostStatusMsg.className = "status-msg ok";
  showProbeReport(p, tuned);
  els.hfForm.classList.add("hidden");
}

// Op een Windows-host haakt een tab aan herdr's sessie-TUI, want `agent attach`
// bestaat daar nog niet. Die TUI tekent een eigen sidebar en tabbalk over de
// agent heen -- dubbelop in een venster dat dat allebei al heeft, en je kunt er
// niets in selecteren omdat de muis naar herdr gaat. Dit zet dat uit, één keer,
// bij het toevoegen of hertesten van de machine. Mislukt het, dan is dat geen
// reden om de host niet op te slaan: de tab werkt ook mét chrome.
async function tuneHerdrChrome(host) {
  // Ook op Linux/macOS: zodra er geen agent in de pane draait valt de tab terug
  // op herdr's sessie-TUI, en dan is daar dezelfde dubbele chrome te verbergen.
  if (host.mux !== "herdr") return "";
  try {
    return await invoke("tune_herdr", { host });
  } catch (e) {
    return "err:" + e;
  }
}

// Bestaande host opnieuw meten (bv. nadat er tmux is geïnstalleerd).
async function testExistingHost(i) {
  const h = hosts[i];
  els.hostStatusMsg.textContent = `${h.nickname}: ${t("host_testing")}`;
  els.hostStatusMsg.className = "status-msg";
  // Meteen zichtbaar dat er iets gebeurt: de probe doet twee ssh-rondes en dat
  // duurt seconden, waarin de rij er anders onveranderd bij staat.
  hostStatus[h.id] = { id: h.id, reachable: false, ms: 0, testing: true };
  renderHostRows();
  let p;
  try { p = await invoke("probe_host", { host: h }); }
  catch (e) { p = { reachable: false, error: String(e) }; }
  if (p.reachable && p.authOk) {
    // Staat de host op Automatisch, dan volgt hij de nieuwe meting -- dat is de
    // reden dat je hertest na het installeren van herdr. Heb je zelf iets
    // vastgezet, dan blijft dat staan; anders draait deze knop je keuze terug.
    const auto = h.mux_auto !== false;
    hosts[i] = { ...h, os: p.os, mux: auto ? (p.mux || "none") : h.mux };
    await invoke("save_hosts", { hosts });
    await refreshMachines();
    await tuneHerdrChrome(hosts[i]);
    fillHostSelect();
    els.hostStatusMsg.textContent = `${h.nickname}: ✓ ${t("host_ok")}`;
    els.hostStatusMsg.className = "status-msg ok";
  } else {
    els.hostStatusMsg.textContent = `${h.nickname}: ✗ ${p.error || t("host_unreachable")}`;
    els.hostStatusMsg.className = "status-msg err";
  }
  showProbeReport(p);
  // Bereikbaar EN ingelogd: alleen dan is de host echt bruikbaar. Alleen een
  // open poort met geweigerde key is geen groen bolletje waard.
  hostStatus[h.id] = { id: h.id, reachable: !!(p.reachable && p.authOk), ms: 0 };
  renderHostRows();
}

// Wat de probe vond, met de twee dingen die een sessie echt blokkeren bovenaan:
// geen agent-CLI en geen uitgaand HTTPS.
function showProbeReport(p, tuned) {
  const lines = [];
  if (tuned === "ok") lines.push(t("host_herdr_tuned"));
  else if (tuned && tuned.startsWith("err:")) lines.push(t("host_herdr_tune_failed").replace("{err}", tuned.slice(4)));
  if (p.os) lines.push(`OS: ${p.os}`);
  if (p.claude) lines.push(`agent: ${p.claude}`);
  else if (p.authOk) lines.push(t("host_no_claude"));
  if (p.authOk && !p.outbound) lines.push(t("host_no_outbound"));
  if (p.authOk && (!p.mux || p.mux === "none")) lines.push(t("host_no_mux"));
  // Alles tonen wat er staat, niet alleen wat Taurus zou kiezen: dan is de
  // keuzelijst hierboven geen gok maar een keuze.
  else if (p.muxes && p.muxes.length > 1) lines.push(t("host_mux_found").replace("{list}", p.muxes.join(", ")).replace("{best}", p.mux));
  else if (p.mux) lines.push(`persistentie: ${p.mux}`);
  if (!lines.length) return;
  els.hfReport.innerHTML = lines.map((l) => `<div>${escapeHtml(l)}</div>`).join("");
  els.hfReport.classList.remove("hidden");
}

/* ---- aanhaken aan een sessie die al draait op een machine ---- */
// Zonder dit kun je alleen bij een draaiende sessie komen door toevallig dezelfde
// agent met dezelfde map te starten, zodat de sessienaam matcht. Sessies van een
// ander werkstation, of van een kaart die intussen anders heet, waren onzichtbaar.

// ⇱ Verder werken: terug naar werk dat al loopt of dat er geweest is. Nadrukkelijk
// alleen JOUW spullen -- deze computer en de machines die je zelf hebt ingericht.
// Een collega verschijnt hier nooit; die steekt zijn hand op en dat is #125.
//
// Lokaal eerst: dat is negen van de tien keer waar je heen wilt, en het geeft de
// geschiedenis uit #129 de plek die dat issue vraagt ("op elk moment bereikbaar",
// niet alleen bij het opstarten).
function openAttachModal() {
  els.atRows.innerHTML = "";
  document.querySelector("#at-local").innerHTML = "";
  els.atStatus.textContent = ""; els.atStatus.className = "status-msg";
  els.attachModal.classList.remove("hidden");
  loadLocalHistory();
  loadRemoteSessions();
}

// De lokale helft: wat er in de geschiedenis staat en niet al open is.
async function loadLocalHistory() {
  const box = document.querySelector("#at-local");
  if (!box) return;
  let hist = [];
  try { hist = await invoke("session_history"); } catch (_) {}
  const open = new Set([...sessions.values()].map((s) => s.uuid).filter(Boolean));
  const rows = hist.filter((h) => h.uuid && !open.has(h.uuid) && !h.hostId);
  // Plus wat Claude zelf nog weet en Taurus niet (#129): gesprekken die buiten
  // Taurus zijn begonnen, of van voor deze geschiedenis bestond.
  const gezien = new Set([...open, ...rows.map((h) => h.uuid)]);
  for (const f of await scanClaudeSessions()) {
    if (gezien.has(f.uuid)) continue;
    gezien.add(f.uuid);
    rows.push({
      uuid: f.uuid, path: f.cwd, title: f.title,
      agent: "claude", model: "", mode: f.mode || "default",
      hostId: "", projectId: "", lastSeen: f.lastSeen, accent: "#7c9cff",
    });
  }
  rows.sort((a, b) => (b.lastSeen || 0) - (a.lastSeen || 0));
  box.innerHTML = "";
  if (!rows.length) {
    box.innerHTML = `<div class="host-empty">${escapeHtml(t("attach_no_local"))}</div>`;
    return;
  }
  for (const h of rows.slice(0, 12)) {
    const row = document.createElement("div");
    row.className = "host-row";
    row.innerHTML = `
      <span class="host-dot pending"></span>
      <div class="host-main">
        <div class="host-name">${escapeHtml(h.title || h.path)}</div>
        <div class="host-sub">${escapeHtml([h.agent || "claude", h.model, h.path].filter(Boolean).join(" · "))}</div>
      </div>
      <span class="route-mux">${escapeHtml(agoText(h.lastSeen))}</span>
      <button class="host-test">${escapeHtml(t("attach_open"))}</button>`;
    row.querySelector("button").addEventListener("click", async () => {
      const meta = {
        uuid: h.uuid, path: h.path, title: h.title, accent: h.accent,
        mode: h.mode, agent: h.agent, model: h.model,
        host_id: "", project_id: h.projectId || "",
      };
      const why = await resumeBlocker(meta);
      if (why) { els.atStatus.textContent = "✗ " + why; els.atStatus.className = "status-msg err"; return; }
      els.attachModal.classList.add("hidden");
      await restoreSessions([meta]);
    });
    box.appendChild(row);
  }
}

// De remote helft: per eigen machine welke AGENTS daar draaien (#128). Geen agent =
// niets om mee te verbinden, en dat staat er dan ook zo.
async function loadRemoteSessions() {
  els.atRows.innerHTML = "";
  if (!machines.length) {
    els.atRows.innerHTML = `<div class="host-empty">${escapeHtml(t("attach_no_hosts"))}</div>`;
    return;
  }
  els.atRefresh.disabled = true;
  for (const m of machines) {
    const box = document.createElement("div");
    box.className = "machine";
    box.innerHTML = `
      <div class="machine-head">
        <span class="host-dot ${machineDotClass(m)}"></span>
        <div class="host-main"><div class="host-name">${escapeHtml(m.label)}</div></div>
        <span class="route-mux">${escapeHtml(t("attach_loading"))}</span>
      </div>`;
    els.atRows.appendChild(box);
    const host = hostById(m.preferred) || m.routes[0];
    let view = null, err = "";
    try { view = await invoke("remote_agents", { hostId: host.id }); }
    catch (e) { err = String(e); }
    const note = box.querySelector(".route-mux");
    if (err) { note.textContent = "✗ " + err; continue; }
    const agents = (view && view.agents) || [];
    note.textContent = agents.length ? "" : t("agents_none_short");
    for (const a of agents) {
      const row = document.createElement("div");
      row.className = "route-row";
      row.innerHTML = `
        <span class="host-dot ${a.status ? "up" : "pending"}"></span>
        <span class="route-name">${escapeHtml(a.title || a.session)}</span>
        <span class="route-mux">${escapeHtml([a.agent, a.cwd].filter(Boolean).join(" · "))}</span>
        ${a.attachable
          ? `<button class="host-test">${escapeHtml(t("session_attach"))}</button>`
          : `<span class="sess-bare" title="${escapeHtml(t("agent_local_hint"))}">${escapeHtml(t("agent_local"))}</span>`}`;
      if (a.attachable) {
        row.querySelector("button").addEventListener("click", () => {
          els.attachModal.classList.add("hidden");
          attachFromMachine(m, a);
        });
      }
      box.appendChild(row);
    }
  }
  els.atRefresh.disabled = false;
}

/* ---- agent naar een andere machine verplaatsen (#102) ---- */
// Welke agent verplaatsen we, en waar staat hij nu? Een tab en een sidebarkaart
// verwijzen allebei naar hetzelfde project, dus dit werkt vanaf beide.
let movePlan = null;

function fmtBytes(n) {
  if (n < 1024) return `${n} B`;
  const u = ["KB", "MB", "GB", "TB"];
  let v = n / 1024, i = 0;
  while (v >= 1024 && i < u.length - 1) { v /= 1024; i++; }
  return `${v < 10 ? v.toFixed(1) : Math.round(v)} ${u[i]}`;
}

async function openMoveModal(project) {
  if (!project || !project.path) { toast(t("move_no_path"), "err"); return; }
  movePlan = { project, skip: new Set(), survey: null, target: null };
  // Doelen: alles behalve waar hij nu al staat. "Deze computer" hoort erbij als
  // de agent elders draait -- terughalen is net zo goed een richting.
  // Doelen zijn machines, niet routes: "ursu" drie keer in de lijst is geen keuze.
  const hier = machineOf(project.host_id || "");
  const opts = [{ id: "", label: t("host_local") }, ...machineOptions(project.host_id || "")]
    .filter((o) => o.id !== (project.host_id || "") && !(hier && machineOf(o.id) === hier));
  if (!opts.length) { toast(t("move_no_target"), "err"); return; }
  els.mvTarget.innerHTML = opts.map((o) => `<option value="${escapeHtml(o.id)}">${escapeHtml(o.label)}</option>`).join("");
  els.mvPath.value = suggestTargetPath(project, opts[0].id);
  els.mvStatus.textContent = "";
  els.mvWarn.classList.add("hidden");
  els.mvSurvey.innerHTML = `<div class="move-busy">${escapeHtml(t("move_surveying"))}</div>`;
  els.moveModal.classList.remove("hidden");
  await refreshMoveSurvey();
}

// Meet de BRON (waar de agent nu staat) en het DOEL, zodat we kunnen zeggen
// welke kant recenter is bijgewerkt voordat er iets overschreven wordt.
async function refreshMoveSurvey() {
  if (!movePlan) return;
  const { project } = movePlan;
  const targetId = els.mvTarget.value;
  const targetPath = els.mvPath.value.trim();
  els.mvSurvey.innerHTML = `<div class="move-busy">${escapeHtml(t("move_surveying"))}</div>`;

  const surveySrc = project.host_id
    ? invoke("survey_remote_workspace", { hostId: project.host_id, path: project.path })
    : invoke("survey_workspace", { path: project.path });
  const surveyDst = targetPath
    ? (targetId
        ? invoke("survey_remote_workspace", { hostId: targetId, path: targetPath })
        : invoke("survey_workspace", { path: targetPath }))
    : Promise.resolve(null);

  let src, dst;
  try { [src, dst] = await Promise.all([surveySrc, surveyDst]); }
  catch (e) { src = { error: String(e) }; dst = null; }
  movePlan.survey = src;
  movePlan.target = dst;
  renderMoveSurvey();
}

// De bulk-mappen staan standaard UIT: ze zijn meestal het leeuwendeel van de
// omvang en aan de andere kant zo weer opgebouwd.
function renderMoveSurvey() {
  const s = movePlan && movePlan.survey;
  if (!s) return;
  if (s.error) {
    els.mvSurvey.innerHTML = `<div class="move-note err">${escapeHtml(s.error)}</div>`;
    return;
  }
  const rows = [];
  rows.push(`<div class="move-line"><span>${escapeHtml(t("move_core"))}</span><span>${fmtBytes(s.coreBytes)} · ${s.coreFiles} ${escapeHtml(t("move_files"))}</span></div>`);
  const group = (list, kind) => list.map((d) => {
    const off = movePlan.skip.has(d.name.toLowerCase());
    return `<label class="move-line pick"><span><input type="checkbox" data-dir="${escapeHtml(d.name)}"${off ? "" : " checked"} /> ${escapeHtml(d.name)}<span class="move-kind">${escapeHtml(kind)}</span></span>` +
           `<span>${fmtBytes(d.bytes)} · ${d.files} ${escapeHtml(t("move_files"))}</span></label>`;
  }).join("");
  if (s.work.length) rows.push(group(s.work, t("move_kind_work")));
  if (s.bulk.length) rows.push(group(s.bulk, t("move_kind_bulk")));

  let total = s.coreBytes;
  for (const d of [...s.work, ...s.bulk]) if (!movePlan.skip.has(d.name.toLowerCase())) total += d.bytes;
  rows.push(`<div class="move-line total"><span>${escapeHtml(t("move_total"))}</span><span>${fmtBytes(total)}</span></div>`);

  rows.push(`<div class="move-line"><span class="move-kind">${escapeHtml(t("move_keep_source"))}</span><span></span></div>`);
  els.mvSurvey.innerHTML = rows.join("");
  renderTargetWarning();
  els.mvSurvey.querySelectorAll("input[type=checkbox]").forEach((cb) => {
    cb.addEventListener("change", (e) => {
      const key = e.target.dataset.dir.toLowerCase();
      if (e.target.checked) movePlan.skip.delete(key); else movePlan.skip.add(key);
      renderMoveSurvey();
    });
  });
}

// Staat er aan de andere kant al iets, en is dat NIEUWER? Dan is dat het enige
// wat je echt wilt weten voordat je overschrijft. Weigeren is te bot -- soms is
// het doel juist de verouderde kopie die je wilt bijwerken. Dus: de feiten, en
// jij beslist.
function renderTargetWarning() {
  const dst = movePlan && movePlan.target;
  const src = movePlan && movePlan.survey;
  if (!dst || !dst.exists) { els.mvWarn.classList.add("hidden"); return; }

  const dstFiles = dst.coreFiles + [...dst.work, ...dst.bulk].reduce((n, d) => n + d.files, 0);
  if (!dstFiles) { els.mvWarn.classList.add("hidden"); return; }

  const newer = src && dst.newest > src.newest;
  const when = dst.newest ? new Date(dst.newest * 1000).toLocaleString() : "?";
  els.mvWarn.innerHTML = newer
    ? `⚠ ${escapeHtml(t("move_target_newer").replace("{when}", when))}`
    : `ℹ ${escapeHtml(t("move_target_exists_info").replace("{when}", when))}`;
  els.mvWarn.className = newer ? "command-warn" : "hint";
  els.mvWarn.classList.remove("hidden");
}

// Een pad op de andere machine raden uit de mapnaam: beter dan een leeg veld,
// en de gebruiker ziet meteen waar het heen zou gaan.
function suggestTargetPath(project, targetHostId) {
  const leaf = (project.path || "").replace(/[\\/]+$/, "").split(/[\\/]/).pop() || "workspace";
  const h = hostById(targetHostId);
  if (!h) return `C:\\Users\\${(navigator.userAgent, "")}`.replace(/\\+$/, "") || leaf;
  const base = (h.default_project || "").replace(/[\\/]+$/, "");
  if (!base) return effectiveWindows(h) ? `C:\\${leaf}` : `/home/${h.user || "user"}/${leaf}`;
  return effectiveWindows(h) ? `${base}\\${leaf}` : `${base}/${leaf}`;
}
function effectiveWindows(h) { return h && h.os === "windows" && h.via !== "wsl"; }

// Hoe zwaar weegt elk item in de voortgang? (#107) De backend meldt alleen WELK
// item aan de beurt is; de groottes staan hier al uit de survey, dus het
// percentage rekenen we hier uit in plaats van de map nog een keer te tellen.
//
// coreBytes is een gezamenlijk getal voor alle losse projectbestanden, en die
// gaan per stuk over. Ze krijgen daarom een gelijk deel toebedeeld -- niet
// exact, maar wel netjes oplopend, en samen kloppen ze met het totaal dat de
// dialoog boven de knop toont.
function transferWeights(survey, skip) {
  const s = survey || {};
  const map = new Map();
  let total = 0;
  const core = s.core || [];
  const perCore = core.length ? (s.coreBytes || 0) / core.length : 0;
  for (const naam of core) {
    map.set(naam.toLowerCase(), perCore);
    total += perCore;
  }
  for (const d of [...(s.work || []), ...(s.bulk || [])]) {
    if (skip.has(d.name.toLowerCase())) continue;
    map.set(d.name.toLowerCase(), d.bytes || 0);
    total += d.bytes || 0;
  }
  return { map, total };
}

async function runMove() {
  if (!movePlan) return;
  const target = els.mvTarget.value;
  const path = els.mvPath.value.trim();
  const src = movePlan.project;
  if (!path) { els.mvStatus.textContent = t("move_need_path"); els.mvStatus.className = "status-msg err"; return; }
  if (src.host_id && target) {
    els.mvStatus.textContent = t("move_host_to_host_todo");
    els.mvStatus.className = "status-msg err";
    return;
  }

  els.mvGo.disabled = true;
  els.mvStatus.className = "status-msg";
  els.mvStatus.textContent = t("move_copying");

  // Voortgang meelezen (#107). Het percentage loopt op de bytes die de survey
  // al kende; kent hij een naam niet, dan valt hij terug op het aantal items.
  const w = transferWeights(movePlan.survey, movePlan.skip);
  let doneBytes = 0;
  const stopProgress = await listen("transfer-progress", (ev) => {
    const [phase, name, index, total] = ev.payload;
    if (phase === "done") {
      doneBytes += w.map.get(String(name).toLowerCase()) ?? 0;
      return;
    }
    const pct = w.total > 0
      // 99 als plafond: 100% terwijl het laatste item nog loopt leest als klaar.
      ? Math.min(99, Math.floor((doneBytes / w.total) * 100))
      : Math.floor(((index - 1) / Math.max(1, total)) * 100);
    els.mvStatus.textContent = t("move_progress")
      .replace("{name}", name)
      .replace("{n}", index)
      .replace("{total}", total)
      .replace("{pct}", pct);
  });

  try {
    let res;
    if (target) {
      // Deze computer -> host.
      res = await invoke("push_workspace", {
        hostId: target, localPath: src.path, remotePath: path,
        skip: [...movePlan.skip],
      });
    } else {
      // Host -> deze computer. De lijst moet EXPLICIET zijn: scp kan aan de
      // andere kant geen map uitlezen, en een wildcard zou juist ook meenemen
      // wat je hebt uitgevinkt.
      const s = movePlan.survey || { core: [], work: [], bulk: [] };
      const optional = [...s.work, ...s.bulk]
        .map((d) => d.name)
        .filter((n) => !movePlan.skip.has(n.toLowerCase()));
      const items = [...(s.core || []), ...optional];
      if (!items.length) throw new Error(t("move_nothing_selected"));
      res = await invoke("pull_workspace", {
        hostId: src.host_id, remotePath: src.path, localPath: path, items,
      });
    }
    // Pas ná een geslaagde kopie de agent omzetten -- anders wijst hij naar een
    // map die er niet staat.
    const next = projects.map((p) => (p.id === movePlan.project.id ? { ...p, host_id: target, path } : p));
    await invoke("save_projects", { projects: next });
    projects = next;
    if (selected && selected.id === movePlan.project.id) selected = projects.find((p) => p.id === movePlan.project.id);
    renderProjects();
    els.moveModal.classList.add("hidden");
    toast(`${t("move_done")} — ${res}`, "ok");
  } catch (e) {
    els.mvStatus.textContent = "✗ " + e;
    els.mvStatus.className = "status-msg err";
  } finally {
    // Altijd afmelden: blijft deze luisteraar staan, dan schrijft een volgende
    // synchronisatie in de statusregel van een dialoog die al gesloten is.
    stopProgress();
    els.mvGo.disabled = false;
  }
}

function uniqueHostId(base) {
  let id = base || "host", n = 2;
  while (hosts.some((h) => h.id === id)) id = `${base}-${n++}`;
  return id;
}

// Bij een remote host betekent "werkmap" een pad OP DIE MACHINE, niet het lokale
// projectpad -- daarom een eigen veld, voorgevuld met het standaardpad van de host.
// "Map niet bereikbaar" en "Geen CLAUDE.md" zijn checks op DEZE machine. Draait
// de agent elders, dan gaat het om een pad daar en zouden ze altijd vals alarm
// slaan -- dus overslaan.
async function refreshHostState() {
  const remote = !!(selected && selected.host_id);
  if (remote || !selected || !selected.path) {
    els.warn.classList.add("hidden");
    els.claudeWarn.classList.add("hidden");
    return;
  }
  await checkLocalPath(selected.path);
}

// De twee lokale controles op de gekozen werkmap, apart zodat zowel het kiezen
// van een project als het terugschakelen naar "deze computer" ze kan herhalen.
async function checkLocalPath(path) {
  let ok = false;
  try { ok = await invoke("path_exists", { path }); } catch (_) {}
  els.warn.classList.toggle("hidden", ok);
  let hasMd = false;
  try { hasMd = await invoke("has_claude_md", { path }); } catch (_) {}
  els.claudeWarn.classList.toggle("hidden", hasMd);
}

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
// ---- tabs bundelen bij drukte (#90) ----
//
// "Dezelfde bron" is dezelfde map op dezelfde machine. Dat staat al op de
// sessie; er is geen extra veld voor nodig. Twee sidebar-kaarten die naar
// dezelfde map wijzen belanden daarmee wel in dezelfde groep -- dat is de
// bewuste afweging in #90 (een projectId zou scherper zijn).
// Sessies groeperen op de AGENT waaruit ze gestart zijn, niet op de map. Twee
// kaarten die dezelfde map delen maar een ander model of een andere modus
// hebben, horen niet samen te vallen -- en voor de gebruiker is "dit is de
// kaart waarop ik geklikt heb" veel navolgbaarder dan "dit is dezelfde map".
//
// Losse sessies (via map-verkennen gestart) hebben geen kaart en dus een lege
// projectId. Die vallen daarmee vanzelf in een eigen gezamenlijke bak.
function tabGroupKey(s) {
  return s.projectId || "";
}

// De rijen voor de tabbalk: losse sessies, of groepen als het er te veel worden.
// De volgorde volgt de tabvolgorde; een groep staat op de plek van zijn eerste lid.
function tabRows() {
  const alle = [...sessions.values()];
  if (!settings.tabGroups || alle.length <= settings.tabGroupAt) {
    return alle.map((s) => ({ groep: false, leden: [s] }));
  }
  const perBron = new Map();
  for (const s of alle) {
    const k = tabGroupKey(s);
    if (!perBron.has(k)) perBron.set(k, []);
    perBron.get(k).push(s);
  }
  const rijen = [];
  const gezien = new Set();
  for (const s of alle) {
    const k = tabGroupKey(s);
    if (gezien.has(k)) continue;
    gezien.add(k);
    const leden = perBron.get(k);
    rijen.push({ groep: leden.length > 1, leden, key: k });
  }
  return rijen;
}

// De staat van een groep is de dringendste staat van zijn leden. Zonder dit
// verdwijnt het "klaar"-signaal van vier agents achter een dichtgeklapte tab --
// precies het risico dat #90 als niet-onderhandelbaar benoemt.
function groupState(leden) {
  if (leden.some((s) => s.awaiting)) return "awaiting";
  if (leden.some((s) => s.working)) return "working";
  if (leden.every((s) => s.exited)) return "exited";
  return "";
}

function hostBadge(s) {
  const host = s.hostId ? hostById(s.hostId) : null;
  return host ? `<span class="tab-host">${escapeHtml(machineLabel(s.hostId))}</span>` : "";
}

function renderTabs() {
  els.tabbar.innerHTML = "";
  closeTabPanel();
  for (const rij of tabRows()) {
    const tab = document.createElement("div");
    els.tabbar.appendChild(tab);

    if (!rij.groep) {
      const s = rij.leden[0];
      let cls = "tab";
      if (current === s.id) cls += " active";
      if (s.exited) cls += " exited"; else if (s.awaiting) cls += " awaiting"; else if (s.working) cls += " working";
      tab.className = cls;
      tab.dataset.tabId = s.id;
      tab.style.borderTopColor = s.accent || "#7c9cff";
      tab.style.setProperty("--tab-accent", s.accent || "#7c9cff");
      const live = settings.tabStatus && s.status && !s.exited;
      const shown = live ? `✶ ${s.status}…` : s.title;
      // Bij tien agents over vier machines moet je op de tab kunnen zien waar er
      // een draait -- zonder dat een lokale tab er anders uit gaat zien.
      const host = s.hostId ? hostById(s.hostId) : null;
      tab.innerHTML = `<span class="tab-dot"></span>${hostBadge(s)}<span class="tab-title${live ? " live" : ""}">${escapeHtml(shown)}</span><span class="tab-close">✕</span>`;
      tab.title = host ? `${s.title} — ${machineLabel(s.hostId)}` : s.title;
      tab.addEventListener("click", () => { if (suppressNextClick) return; showView(s.id); });
      tab.addEventListener("contextmenu", (e) => { e.preventDefault(); openTabMenu(e.clientX, e.clientY, s.id); });
      tab.querySelector(".tab-close").addEventListener("click", (e) => { e.stopPropagation(); closeSession(s.id); });
      makeReorderable(tab, { axis: "x", itemClass: "tab", endSelector: ".newtab", onDrop: commitTabOrder });
      wireRecapHover(tab, s);
      continue;
    }

    // Gebundelde tab.
    const leden = rij.leden;
    const staat = groupState(leden);
    const eerste = leden[0];
    const bevatHuidige = leden.some((s) => s.id === current);
    tab.className = "tab tabgroup" + (bevatHuidige ? " active" : "") + (staat ? ` ${staat}` : "");
    tab.dataset.groupKey = rij.key;
    tab.style.borderTopColor = eerste.accent || "#7c9cff";
    tab.style.setProperty("--tab-accent", eerste.accent || "#7c9cff");
    const wacht = leden.filter((s) => s.awaiting).length;
    // De naam van de KAART, niet die van het eerste lid: leden kunnen een eigen
    // sessietitel hebben, en dan zou de groep willekeurig de naam van wie
    // toevallig eerst stond lenen. Losse sessies hebben geen kaart.
    const kaart = rij.key ? projects.find((p) => p.id === rij.key) : null;
    const naam = kaart ? (kaart.label || kaart.title || rij.key) : t("grp_adhoc");
    tab.innerHTML =
      `<span class="tab-dot"></span>${hostBadge(eerste)}` +
      `<span class="tab-title">${escapeHtml(naam)}</span>` +
      `<span class="tab-count">${leden.length}</span>` +
      `<span class="tab-caret">▾</span>`;
    tab.title = `${naam} — ${t("grp_tab_members").replace("{n}", leden.length)}` +
      (wacht ? ` · ${wacht} ${t("grp_waiting")}` : "");
    // Hover klapt uit, klik ook -- niets mag alleen via hover bereikbaar zijn.
    tab.addEventListener("mouseenter", () => scheduleTabPanel(tab, rij));
    tab.addEventListener("mouseleave", () => scheduleTabPanelClose());
    tab.addEventListener("click", (e) => {
      if (suppressNextClick) return;
      e.stopPropagation();
      if (tabPanelKey === rij.key) closeTabPanel(); else openTabPanel(tab, rij);
    });
  }
  const plus = document.createElement("div");
  plus.className = "tab newtab" + (current === "new" ? " active" : "");
  plus.textContent = t("newtab");
  plus.addEventListener("click", () => { resetLaunchForm(); showView("new"); });
  els.tabbar.appendChild(plus);
  syncTabPanel();
}

// ---- uitklappaneel van een gebundelde tab ----
//
// Het paneel hangt aan document.body, niet in de tabbalk. Dat is hetzelfde
// patroon als .ctx-menu (openTabMenu) en het scheelt gepuzzel met clipping en
// z-index in de topbar.
let tabPanel = null;
let tabPanelKey = null;
let panelOpenTimer = null;
let panelCloseTimer = null;

const PANEL_OPEN_MS = 160;   // hover moet bedoeld zijn, niet langslopen
const PANEL_CLOSE_MS = 260;  // marge: de muis moet van tab naar paneel reizen
const RECAP_OPEN_MS = 450;
const RECAP_REGELS = 10;

function scheduleTabPanel(anchor, rij) {
  if (panelCloseTimer) { clearTimeout(panelCloseTimer); panelCloseTimer = null; }
  // Ook op tabPanel controleren, niet alleen op de sleutel: is het element weg
  // zonder dat closeTabPanel liep, dan zou een sleutel-alleen-check het paneel
  // voorgoed weigeren te heropenen.
  if (tabPanelKey === rij.key && tabPanel) return;
  if (panelOpenTimer) clearTimeout(panelOpenTimer);
  panelOpenTimer = setTimeout(() => openTabPanel(anchor, rij), PANEL_OPEN_MS);
}

function scheduleTabPanelClose() {
  if (panelOpenTimer) { clearTimeout(panelOpenTimer); panelOpenTimer = null; }
  if (panelCloseTimer) clearTimeout(panelCloseTimer);
  panelCloseTimer = setTimeout(closeTabPanel, PANEL_CLOSE_MS);
}

function closeTabPanel() {
  if (panelOpenTimer) { clearTimeout(panelOpenTimer); panelOpenTimer = null; }
  if (panelCloseTimer) { clearTimeout(panelCloseTimer); panelCloseTimer = null; }
  hideRecap();
  if (tabPanel) { tabPanel.remove(); tabPanel = null; }
  tabPanelKey = null;
}

// renderTabs draait bij elke statuswijziging, ook van een agent op de
// achtergrond. Het paneel daar dichtklappen zou het onder je muis weghalen
// terwijl je aan het kijken bent -- dus opnieuw vullen als de groep nog bestaat.
function syncTabPanel() {
  if (!tabPanelKey) return;
  const rij = tabRows().find((r) => r.groep && r.key === tabPanelKey);
  const anchor = els.tabbar.querySelector(`[data-group-key="${CSS.escape(tabPanelKey)}"]`);
  if (!rij || !anchor) { closeTabPanel(); return; }
  openTabPanel(anchor, rij, true);
}

function openTabPanel(anchor, rij, hervullen = false) {
  if (panelOpenTimer) { clearTimeout(panelOpenTimer); panelOpenTimer = null; }
  if (!hervullen) hideRecap();
  if (tabPanel) tabPanel.remove();
  tabPanelKey = rij.key;

  tabPanel = document.createElement("div");
  tabPanel.className = "tabpanel";
  for (const s of rij.leden) {
    const rw = document.createElement("div");
    let cls = "tabpanel-row";
    if (s.id === current) cls += " active";
    if (s.exited) cls += " exited"; else if (s.awaiting) cls += " awaiting"; else if (s.working) cls += " working";
    rw.className = cls;
    rw.style.setProperty("--tab-accent", s.accent || "#7c9cff");
    const live = settings.tabStatus && s.status && !s.exited;
    rw.innerHTML =
      `<span class="tab-dot"></span>` +
      `<span class="tabpanel-title">${escapeHtml(s.title)}</span>` +
      (live ? `<span class="tabpanel-state live">✶ ${escapeHtml(s.status)}…</span>` : "") +
      `<span class="tab-close">✕</span>`;
    rw.addEventListener("click", (e) => {
      if (e.target.closest(".tab-close")) return;
      closeTabPanel();
      showView(s.id);
    });
    rw.querySelector(".tab-close").addEventListener("click", (e) => { e.stopPropagation(); closeSession(s.id); });
    rw.addEventListener("contextmenu", (e) => { e.preventDefault(); e.stopPropagation(); closeTabPanel(); openTabMenu(e.clientX, e.clientY, s.id); });
    // Bewust GEEN makeReorderable hier: een rij uit een hover-paneel slepen
    // heeft geen betekenis (#90).
    wireRecapHover(rw, s);
    tabPanel.appendChild(rw);
  }
  tabPanel.addEventListener("mouseenter", () => {
    if (panelCloseTimer) { clearTimeout(panelCloseTimer); panelCloseTimer = null; }
  });
  tabPanel.addEventListener("mouseleave", scheduleTabPanelClose);
  document.body.appendChild(tabPanel);

  const r = anchor.getBoundingClientRect();
  const breedte = tabPanel.offsetWidth;
  tabPanel.style.left = `${Math.max(6, Math.min(r.left, window.innerWidth - breedte - 6))}px`;
  tabPanel.style.top = `${r.bottom}px`;
}

// ---- recap: wat is deze agent aan het doen? ----
//
// Uit de xterm-buffer van de sessie zelf. Dat kan omdat pty-output naar
// s.term.write gaat ongeacht welke tab in beeld staat -- dus ook een verborgen
// sessie houdt zijn scherm bij. Claude Code draait in de alternate buffer, dus
// buffer.active IS het zichtbare TUI-scherm.
//
// Niet uit het transcript: dat formaat is intern aan Claude Code en verandert
// tussen versies. En niet via /recap: dat zou tokens kosten en in het gesprek
// belanden.
function recapChrome(regel) {
  const r = regel.trim();
  if (!r) return true;
  // Kadertekens (U+2500..U+257F): de scheidingslijnen om het invoerveld.
  if (/^[─-╿\s]+$/.test(r)) return true;
  if (/^[>❯]/.test(r)) return true;                    // invoerregel: > of ❯
  if (/for shortcuts|to interrupt|manual mode|accept edits/i.test(r)) return true;
  if (/^[⏸▪·]/.test(r)) return true;         // modus-/hintbalk
  // Spinnerglyphs: dezelfde set als lastSpinnerVerb. Het werkwoord staat al op
  // de tab, dus in de recap is die regel ruis.
  if (/^[✶✻✽✳✢✦✧⋆∗]/.test(r)) return true;
  return false;
}

function sessionRecap(s) {
  const term = s && s.term;
  if (!term || !term.buffer) return "";
  try {
    const buf = term.buffer.active;
    const vanaf = Math.max(0, buf.length - Math.max((term.rows || 24) * 2, 48));
    const regels = [];
    for (let i = vanaf; i < buf.length; i++) {
      const ln = buf.getLine(i);
      regels.push(ln ? ln.translateToString(true) : "");
    }
    // Claude Code zet een bolletje voor elk agentbericht; dat is het anker.
    // Zonder anker zou "de laatste niet-lege regels" het invoerveld tonen,
    // want dat staat onderaan het scherm.
    let van = -1;
    for (let i = regels.length - 1; i >= 0; i--) {
      if (regels[i].trim().startsWith("●")) { van = i; break; }
    }
    const schoon = (van >= 0 ? regels.slice(van) : regels)
      .filter((r) => !recapChrome(r))
      .map((r) => r.replace(/\s+$/, ""));
    const gekozen = van >= 0 ? schoon.slice(0, RECAP_REGELS) : schoon.slice(-RECAP_REGELS);
    return gekozen.join("\n").trim();
  } catch (_) {
    return "";
  }
}

let recapTip = null;
let recapTimer = null;

function hideRecap() {
  if (recapTimer) { clearTimeout(recapTimer); recapTimer = null; }
  if (recapTip) { recapTip.remove(); recapTip = null; }
}

function showRecap(anchor, s) {
  hideRecap();
  const tekst = sessionRecap(s) || t("recap_none");
  const host = s.hostId ? hostById(s.hostId) : null;
  const staat = s.exited ? t("recap_exited")
    : s.awaiting ? t("grp_waiting")
    : s.working ? (s.status ? `✶ ${s.status}…` : t("grp_working"))
    : "";
  recapTip = document.createElement("div");
  recapTip.className = "recap-tip";
  // Waarmee is deze agent gestart? Bij een bundel per kaart kunnen leden
  // verschillen in agent, model en modus, en dan moet je hier kunnen zien welke
  // je te pakken hebt zonder eerst te schakelen.
  const meta = s.command
    ? [s.command]
    : [s.agent || "claude", s.model || "", s.mode && s.mode !== "default" ? s.mode : ""].filter(Boolean);

  recapTip.innerHTML =
    `<div class="recap-head">` +
      `<span class="recap-name">${escapeHtml(s.title)}</span>` +
      (host ? `<span class="tab-host">${escapeHtml(machineLabel(s.hostId))}</span>` : "") +
      (staat ? `<span class="recap-state">${escapeHtml(staat)}</span>` : "") +
    `</div>` +
    `<div class="recap-meta">${escapeHtml(meta.join(" · "))}</div>` +
    `<pre class="recap-body">${escapeHtml(tekst)}</pre>`;
  document.body.appendChild(recapTip);

  const r = anchor.getBoundingClientRect();
  const w = recapTip.offsetWidth, h = recapTip.offsetHeight;
  const rand = 8;
  let x, y;

  if (anchor.classList.contains("tabpanel-row") && tabPanel) {
    // Naast het PANEEL, niet naast de rij: anders schuift de recap over de
    // andere rijen heen en kun je niet meer zien waar je hovert. Past hij er
    // rechts niet naast, dan onder het paneel -- nooit erover.
    const p = tabPanel.getBoundingClientRect();
    if (p.right + 6 + w <= window.innerWidth - rand) {
      x = p.right + 6;
      y = r.top;
    } else {
      x = p.left;
      y = p.bottom + 6;
    }
  } else {
    x = r.left;
    y = r.bottom + 6;
  }

  if (x + w > window.innerWidth - rand) x = window.innerWidth - w - rand;
  if (y + h > window.innerHeight - rand) y = window.innerHeight - h - rand;
  recapTip.style.left = `${Math.max(rand, x)}px`;
  recapTip.style.top = `${Math.max(rand, y)}px`;
}

function wireRecapHover(el, s) {
  el.addEventListener("mouseenter", () => {
    if (!settings.tabRecap) return;
    if (recapTimer) clearTimeout(recapTimer);
    recapTimer = setTimeout(() => showRecap(el, s), RECAP_OPEN_MS);
  });
  el.addEventListener("mouseleave", hideRecap);
  el.addEventListener("mousedown", hideRecap);
}

// Buiten klikken of Escape sluit het paneel; hover alleen zou het aan de muis
// vastplakken als de pointer het venster verlaat.
document.addEventListener("mousedown", (e) => {
  if (!tabPanel) return;
  if (e.target.closest(".tabpanel") || e.target.closest(".tabgroup")) return;
  closeTabPanel();
});
window.addEventListener("blur", () => { hideRecap(); closeTabPanel(); });

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
  updateDropperForSession(); // DROPZONE geldt per sessie: remote kan niet
  renderTabs();
}

/* ============ sessie starten ============ */
// Bouwt de terminal-UI + sessie-object en bedraadt alle events. Doet NIET zelf de
// backend-aanroep (create vs resume verschilt) -- dat doet de aanroeper.
function spawnTerminal({ id, uuid, path, title, accent, mode, command, agent, model, hostId, projectId, mirror }) {
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
    // Leeg = lokaal. Bij een remote sessie geldt `path` op DIE machine.
    hostId: hostId || "",
    // Van welke agentkaart komt deze sessie (#90)? Leeg = losse sessie, via
    // map-verkennen gestart. Bepaalt onder welke tab hij gebundeld wordt.
    projectId: projectId || "",
    // Gevuld = deze tab kijkt mee met een INKOMENDE sessie van een collega
    // (JOIN, #121). Er is geen eigen proces: toetsen en maat gaan naar diens
    // terminal, en sluiten stopt alleen het meekijken.
    mirror: mirror || "",
    gen: ++genSeq,
    exited: false, working: false, awaiting: false, announced: false, status: null, lastSpin: 0, buf: "",
    decoder: new TextDecoder("utf-8"), previewMode: null, lastSel: "",
  };
  sessions.set(id, session);

  term.onData((d) => session.mirror
    ? invoke("ssh_mirror_write", { id: session.mirror, data: d })
    : invoke("write_session", { id, data: d }));
  term.onResize(({ cols, rows }) => session.mirror
    ? invoke("ssh_mirror_resize", { id: session.mirror, cols, rows })
    : invoke("resize_session", { id, cols, rows }));
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
  // Niet bij een remote sessie: de preview leest het bestand met read_file op DIT
  // werkstation, en het pad dat de agent noemt bestaat op de host. Een klikbaar
  // ogende link die niets doet is misleidender dan geen link.
  if (term.registerLinkProvider && !hostId) {
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
  // "Eigen commando…" is een UI-keuze, geen agent: de agent blijft claude als
  // terugval (de backend gebruikt hem toch niet zolang er een override staat).
  const isCmd = els.agentInput.value === AGENT_COMMAND;
  const command = isCmd ? els.commandInput.value.trim() : "";
  const agent = isCmd ? "claude" : (els.agentInput.value || "claude");
  const model = els.modelInput.value.trim();
  // De machine hoort bij de AGENT, niet bij deze keer starten: standaard lokaal,
  // en remote alleen als de agent zo is ingericht in "Agents beheren".
  const hostId = selected.host_id || "";
  if (hostId && !path) {
    els.status.textContent = t("remote_need_path"); els.status.className = "status-msg err";
    return;
  }

  // Van welke kaart komt deze sessie? Bladeren naar een map zet id op "", dus
  // losse sessies krijgen hier vanzelf een lege projectId.
  const projectId = selected.id || "";
  const session = spawnTerminal({ id, uuid, path, title, accent, mode, command, agent, model, hostId, projectId });
  try {
    await invoke("create_session", { id, gen: session.gen, path, title, task, sessionId: uuid, mode, fullPaths: settings.fullPaths, command, agent, model: resolveModelArg(agent, model), hostId, cols: session.term.cols, rows: session.term.rows });
    recordSession(session);
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
    agent: els.agentInput.value === AGENT_COMMAND ? "claude" : (els.agentInput.value || "claude"),
    model: els.modelInput.value.trim(),
    command: els.agentInput.value === AGENT_COMMAND ? els.commandInput.value.trim() : "",
    host_id: selected.host_id || "", // machine hoort bij de agent, niet bij deze start
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
/* ============ sessiegeschiedenis (#129) ============ */
// Drie standen in plaats van een vinkje: vragen (nieuwe default), stil hervatten
// (wat het was) en schoon starten. "Schoon" wist alleen de OPEN-lijst; de
// geschiedenis blijft, want daar zat de fout.
function persistMode() {
  const m = settings.persistMode || (settings.persistSessions === false ? "clean" : "ask");
  return m === "silent" || m === "clean" ? m : "ask";
}

// Elke sessie die Taurus start belandt in de geschiedenis, en blijft daar. Een
// aangehaakte of gespiegelde sessie heeft geen eigen transcript om te hervatten,
// maar het spoor dat hij bestond hoort er wel te zijn.
function recordSession(s) {
  if (!s || !s.uuid) return;
  invoke("history_record", {
    entry: {
      uuid: s.uuid, path: s.path || "", title: s.title || "",
      accent: s.accent || "#7c9cff", mode: s.mode || "default",
      agent: s.agent || "claude", model: s.model || "",
      hostId: s.hostId || "", projectId: s.projectId || "",
      created: 0, lastSeen: 0, wasOpen: true,
    },
  }).catch(() => {});
}

// Wat staat er NU open. Aparte vraag van "wat is er geweest", en daarom een aparte
// aanroep: zo valt een tab die je sluit netjes af zonder uit de lijst te verdwijnen.
function recordOpenSessions() {
  const open = [...sessions.values()].filter((s) => s.uuid).map((s) => s.uuid);
  invoke("history_mark_open", { uuids: open }).catch(() => {});
}

function persistSessionsToDisk() {
  // De geschiedenis loopt HIER buitenom (#129). Wat er open staat is een andere
  // vraag dan wat er geweest is, en die tweede mag niet meegewist worden door de
  // eerste -- dat was precies hoe twee sessies na een herstart verdwenen.
  recordOpenSessions();
  if (persistMode() === "clean") { invoke("save_sessions", { sessions: [] }).catch(() => {}); return; }
  const list = [...sessions.values()]
    // Een aangehaakte sessie heeft geen uuid en geen commando dat Taurus kent;
    // hem hier opslaan zou bij de volgende start een resume proberen die nergens
    // op slaat. De sessie zelf blijft op de host draaien -- je haakt gewoon
    // opnieuw aan via ⇱.
    // Een spiegel-tab (JOIN) hoort er ook niet in: die heeft geen eigen proces
    // en bij de volgende start bestaat de sessie van de collega niet meer.
    .filter((s) => !s.command && !s.attached && !s.mirror)
    .map((s) => ({ id: s.id, uuid: s.uuid, path: s.path, title: s.title, accent: s.accent, mode: s.mode || "default", agent: s.agent || "claude", model: s.model || "", host_id: s.hostId || "", project_id: s.projectId || "" }));
  invoke("save_sessions", { sessions: list }).catch(() => {});
}

// Bij opstarten: probeer elke opgeslagen sessie te hervatten met `--resume`, zonder
// te vragen. Ontbreekt het transcript (Claude heeft het opgeruimd) of is het ouder
// dan 1 dag -> overslaan, niet eens proberen. Een echte spawn-fout -> tab opruimen
// en melden welke (projectnaam) sessie niet lukte.
// Waarom een opgeslagen sessie NU niet te hervatten is. Een reden, geen stilte:
// overslaan mag hem niet uit beeld halen, want dat was precies de fout (#129).
async function resumeBlocker(meta) {
  if (meta.host_id) {
    return hostById(meta.host_id) ? "" : t("resume_no_host");
  }
  // Het transcript van een REMOTE sessie staat op de host, niet hier -- die check
  // zou hem altijd overslaan. Draait er een multiplexer, dan haakt de herstart
  // bovendien gewoon aan de nog levende sessie aan; zo niet, dan vindt
  // claude --resume het transcript daar zelf.
  let st = { exists: false, ageSecs: 0 };
  try { st = await invoke("session_state", { path: meta.path, uuid: meta.uuid }); } catch (_) {}
  if (!st.exists) return t("resume_no_transcript");
  // Ouderdom blokkeert NIET. Dat was de oude auto-hervat-heuristiek, en #129 zegt
  // er zelf het juiste over: leeftijd is geen bewijs dat een sessie waardeloos is.
  // `claude --resume` doet het prima op een gesprek van vorige week, en jij kiest
  // hier zelf. Hoe oud hij is staat rechts in de rij; dat is informatie, geen slot.
  return "";
}

// Hervat precies wat er is aangevinkt. Geen filter meer hierbinnen: wat niet kon,
// is al bij het vragen als reden getoond.
async function restoreSessions(metas) {
  const failures = [];
  for (const meta of metas) {
    const uuid = meta.uuid;
    if (!uuid) continue;
    const id = "s" + (++seq);
    const session = spawnTerminal({
      id, uuid, path: meta.path,
      title: meta.title || "agent", accent: meta.accent || "#7c9cff",
      mode: meta.mode || "default", command: "",
      agent: meta.agent || "claude", model: meta.model || "",
      hostId: meta.host_id || "",
      // Zonder dit zouden alle hervatte sessies na een herstart als losse
      // sessies in een bak belanden in plaats van bij hun eigen agent (#90).
      projectId: meta.project_id || "",
    });
    session.el.classList.add("hidden");
    session.term.write(`\x1b[2m[${t("restarting")} ${uuid.slice(0, 8)}…]\x1b[0m\r\n`);
    try {
      await invoke("restart_session", {
        id, gen: session.gen, path: meta.path, title: session.title, sessionId: uuid,
        mode: session.mode, fullPaths: settings.fullPaths, command: "",
        agent: session.agent, model: resolveModelArg(session.agent, session.model),
        hostId: session.hostId || "",
        cols: session.term.cols, rows: session.term.rows,
      });
    } catch (_) {
      sessions.delete(id); session.term.dispose(); session.el.remove();
      failures.push(`${meta.title || meta.path} (${uuid.slice(0, 8)})`);
    }
  }
  showView("new");            // herstelde tabs in de balk, maar blijf op het startscherm
  persistSessionsToDisk();
  if (failures.length) toast(`${t("restore_failed")} ${failures.join(", ")}`, "err");
}

// Bij het opstarten. Drie standen: vragen (default), stil hervatten zoals het was,
// of schoon beginnen. "Schoon" verliest niets -- de geschiedenis blijft staan.
async function startupRestore() {
  const mode = persistMode();
  if (mode === "clean") return;
  let saved = [];
  try { saved = await invoke("get_sessions"); } catch (_) { return; }

  if (mode === "silent") {
    const ok = [];
    for (const m of saved) {
      if (m.uuid && !(await resumeBlocker(m))) ok.push(m);
    }
    if (ok.length) await restoreSessions(ok);
    return;
  }

  // Vragen. Wat open stond staat voorgevinkt; de rest van de geschiedenis staat
  // eronder, uitgevinkt. Wat niet te hervatten is krijgt een reden in plaats van
  // te verdwijnen.
  let hist = [];
  try { hist = await invoke("session_history"); } catch (_) {}
  const openUuids = new Set(saved.map((m) => m.uuid).filter(Boolean));
  const rows = [];
  for (const m of saved) {
    if (!m.uuid) continue;
    rows.push({ meta: m, open: true, reason: await resumeBlocker(m) });
  }
  const gezien = new Set(openUuids);
  for (const h of hist) {
    if (!h.uuid || gezien.has(h.uuid)) continue;
    gezien.add(h.uuid);
    const meta = {
      uuid: h.uuid, path: h.path, title: h.title, accent: h.accent,
      mode: h.mode, agent: h.agent, model: h.model,
      host_id: h.hostId || "", project_id: h.projectId || "",
    };
    rows.push({ meta, open: false, reason: await resumeBlocker(meta), lastSeen: h.lastSeen });
  }
  // En alles wat Claude zelf nog weet (#129). Zonder dit kent de vraag alleen wat
  // Taurus ooit zelf startte -- gemeten: één regel, terwijl er meerdere sessies
  // liepen. Dan is stil opstarten beter dan een vraag die het antwoord niet heeft.
  for (const f of await scanClaudeSessions()) {
    if (gezien.has(f.uuid)) continue;
    gezien.add(f.uuid);
    const meta = {
      uuid: f.uuid, path: f.cwd, title: f.title, accent: "#7c9cff",
      mode: f.mode || "default", agent: "claude", model: "",
      host_id: "", project_id: "",
    };
    // Geen resumeBlocker: het transcript is zojuist gelezen, dus dat het bestaat
    // weten we al. Scheelt een IPC-ronde per rij bij een volle geschiedenis.
    rows.push({ meta, open: false, reason: "", lastSeen: f.lastSeen });
  }
  if (!rows.length) return;
  // Nieuwste bovenaan binnen elke groep; wat openstond blijft eerst.
  rows.sort((a, b) => (b.open ? 1 : 0) - (a.open ? 1 : 0) || (b.lastSeen || 0) - (a.lastSeen || 0));
  openRestoreDialog(rows);
}

// Wat Claude zelf nog aan hervatbare sessies heeft liggen. Eén ssh-loze scan over
// zijn projectmappen; de backend leest per transcript alleen tot hij genoeg weet.
async function scanClaudeSessions() {
  try {
    const r = await invoke("scan_claude_sessions", { limit: null });
    // Geen stille afkapping: als er meer transcripts zijn dan gelezen, hoort dat
    // in beeld te komen in plaats van te lezen als "meer is er niet".
    lastScan = r;
    return r.sessions || [];
  } catch (_) {
    lastScan = null;
    return [];
  }
}
let lastScan = null;

// De vraag zelf. Een rij die niet te hervatten is blijft staan mét de reden en is
// niet aan te vinken -- verdwijnen was de fout, en een vinkje dat niets doet zou de
// volgende zijn.
function openRestoreDialog(rows) {
  const box = document.querySelector("#restore-rows");
  const modal = document.querySelector("#restore-modal");
  if (!box || !modal) return;
  box.innerHTML = "";
  // Geen stille afkapping: als er meer transcripts liggen dan gelezen zijn, hoort
  // dat er te staan -- anders leest de lijst als "meer is er niet".
  const note = document.querySelector("#restore-note");
  if (note) {
    const meer = lastScan && lastScan.total > lastScan.read;
    note.textContent = meer
      ? t("restore_more").replace("{read}", lastScan.read).replace("{total}", lastScan.total)
      : "";
    note.classList.toggle("hidden", !meer);
  }
  rows.forEach((r, i) => {
    const row = document.createElement("label");
    row.className = "host-row restore-row";
    const meta = [r.meta.agent || "claude", r.meta.model, r.meta.host_id ? machineLabel(r.meta.host_id) : ""]
      .filter(Boolean).join(" · ");
    row.innerHTML = `
      <input type="checkbox" data-i="${i}"${r.open && !r.reason ? " checked" : ""}${r.reason ? " disabled" : ""} />
      <div class="host-main">
        <div class="host-name">${escapeHtml(r.meta.title || r.meta.path || r.meta.uuid.slice(0, 8))}</div>
        <div class="host-sub">${escapeHtml(r.meta.path)}${meta ? " · " + escapeHtml(meta) : ""}</div>
      </div>
      ${r.reason ? `<span class="sess-bare">${escapeHtml(r.reason)}</span>` : ""}
      ${r.lastSeen ? `<span class="route-mux">${escapeHtml(agoText(r.lastSeen))}</span>` : ""}`;
    box.appendChild(row);
  });
  modal.classList.remove("hidden");

  const close = () => modal.classList.add("hidden");
  document.querySelector("#restore-none").onclick = close;
  document.querySelector("#restore-go").onclick = async () => {
    const picked = [...box.querySelectorAll("input:checked")].map((c) => rows[+c.dataset.i].meta);
    close();
    if (picked.length) await restoreSessions(picked);
  };
}

// "12 min geleden". Seconden sinds epoch, net als de backend ze bewaart.
function agoText(secs) {
  if (!secs) return "";
  const d = Math.max(0, Math.floor(Date.now() / 1000) - secs);
  if (d < 90) return t("ago_now");
  if (d < 5400) return t("ago_min").replace("{n}", Math.round(d / 60));
  if (d < 172800) return t("ago_hour").replace("{n}", Math.round(d / 3600));
  return t("ago_day").replace("{n}", Math.round(d / 86400));
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
  // Meekijken stoppen mag de sessie van de collega niet afbreken.
  if (s.mirror) await invoke("ssh_mirror_detach", { id: s.mirror }).catch(() => {});
  else await invoke("close_session", { id });
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
  requestAnimationFrame(() => {
    try { s.fit.fit(); } catch (_) {}
    if (s.mirror) invoke("ssh_mirror_resize", { id: s.mirror, cols: s.term.cols, rows: s.term.rows });
    else invoke("resize_session", { id: s.id, cols: s.term.cols, rows: s.term.rows });
  });
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
// JOIN (#121): dezelfde bytes die naar de collega gaan, ook hier tekenen. De
// tab ontstaat zodra de eerste output komt -- dat is het moment waarop er echt
// iets te zien is, en de sessie-id is dan bekend.
const mirrorTabs = new Map(); // inkomende sessie-id -> lokale tab-id
listen("ssh-mirror-output", (event) => {
  const [sid, data] = event.payload;
  let tabId = mirrorTabs.get(sid);
  if (!tabId || !sessions.has(tabId)) {
    tabId = `join-${sid}`;
    mirrorTabs.set(sid, tabId);
    const inbound = (sshSessions.find((x) => x.id === sid) || {});
    spawnTerminal({
      id: tabId,
      uuid: "",
      path: "",
      title: `👥 ${inbound.label || t("join_tab")}`,
      accent: "",
      mode: "default",
      command: "",
      agent: "",
      model: "",
      hostId: "",
      projectId: "",
      mirror: sid,
    });
    showView(tabId);
  }
  const s = sessions.get(tabId);
  if (s) s.term.write(Uint8Array.from(atob(data), (c) => c.charCodeAt(0)));
});
listen("ssh-mirror-exit", (event) => {
  const sid = event.payload;
  const tabId = mirrorTabs.get(sid);
  mirrorTabs.delete(sid);
  const s = tabId && sessions.get(tabId);
  if (!s) return;
  s.exited = true; s.working = false; s.status = null;
  s.term.write(`\r\n\x1b[2m${t("ended")}\x1b[0m\r\n`);
  renderTabs();
});
listen("ssh-sessions-changed", () => refreshSshSessions());

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
    await invoke("restart_session", { id, gen: s.gen, path: s.path, title: s.title, sessionId: s.uuid, mode: s.mode || "default", fullPaths: settings.fullPaths, command: s.command || "", agent: s.agent || "claude", model: resolveModelArg(s.agent || "claude", s.model || ""), hostId: s.hostId || "", cols: s.term.cols, rows: s.term.rows });
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
  // Preview en Verkenner lezen de werkmap van DIT werkstation. Bij een remote
  // sessie staat die map op de host: Verkenner zou niets of de verkeerde map
  // openen en de preview blijft leeg. Uitgrijzen i.p.v. stil laten mislukken.
  const off = s.hostId ? " disabled" : "";
  const why = s.hostId ? ` title="${escapeHtml(t("remote_local_only"))}"` : "";
  // Aangehaakt aan een bestaande sessie: Taurus heeft dat commando niet gebouwd.
  // Herstarten zou een nieuwe agent starten in plaats van deze te hervatten, en
  // verplaatsen werkt op een agent-kaart die hier niet bestaat.
  const att = s.attached ? " disabled" : "";
  const attWhy = s.attached ? ` title="${escapeHtml(t("attach_not_restartable"))}"` : "";
  m.innerHTML = `
    <div class="ctx-item${att}"${attWhy} data-act="restart">${t("ctx_restart")}</div>
    <div class="ctx-item${off}"${why} data-act="preview">${t("ctx_preview")}</div>
    <div class="ctx-item" data-act="speak">${t("ctx_speak")}</div>
    <div class="ctx-item${off}"${why} data-act="explorer">${t("ctx_explorer")}</div>
    <div class="ctx-item${att}"${attWhy} data-act="move">${t("ctx_move")}</div>
    <div class="ctx-item${off}"${why} data-act="help">${t("ctx_help")}</div>
    <div class="ctx-item" data-act="close">${t("ctx_close")}</div>`;
  m.style.left = x + "px"; m.style.top = y + "px";
  m.querySelector('[data-act="restart"]').addEventListener("click", () => { if (!s.attached) restartSession(id); });
  // Hulp vragen kan alleen voor een sessie die HIER draait: je nodigt iemand uit in
  // je eigen terminal. Bij een remote sessie zit het werk al ergens anders (#125).
  m.querySelector('[data-act="help"]').addEventListener("click", () => {
    closeTabMenu();
    if (!s.hostId) askForHelp(s);
  });
  m.querySelector('[data-act="speak"]').addEventListener("click", () => {
    closeTabMenu();
    const sel = s.term.getSelection();
    if (sel) speak(sel, true); // force: uitspreken is hier expliciet gevraagd
  });
  m.querySelector('[data-act="close"]').addEventListener("click", () => { closeTabMenu(); closeSession(id); });
  // Verplaatsen werkt op de AGENT achter deze tab; de sessie zelf verhuist niet
  // mee, die draait waar hij draait tot je hem opnieuw start.
  m.querySelector('[data-act="move"]').addEventListener("click", () => {
    if (s.attached) return;
    closeTabMenu();
    const p = projects.find((x) => x.path === s.path && (x.host_id || "") === (s.hostId || ""))
      || { id: "", label: s.title, path: s.path, host_id: s.hostId || "" };
    openMoveModal(p);
  });
  if (!s.hostId) {
    m.querySelector('[data-act="preview"]').addEventListener("click", () => openPreview(id));
    m.querySelector('[data-act="explorer"]').addEventListener("click", () => { closeTabMenu(); invoke("open_folder", { path: s.path }).catch(() => {}); });
  }
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
  els.setTabRecap.checked = settings.tabRecap;
  els.setTabGroupAt.value = settings.tabGroups ? settings.tabGroupAt : 0;
  els.setFullPaths.checked = settings.fullPaths;
  els.setPersistMode.value = persistMode();
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
  refreshSshStatus();
  refreshSshPeers();
  refreshSshSessions();
  els.settingsModal.classList.remove("hidden");
}

/* ============ Taurus als SSH-host (#121) ============ */
// De status komt van de Rust-kant, niet uit localStorage: daar staat wat de
// listener DOET, en dat is het enige eerlijke antwoord op "ben ik bereikbaar?".
let sshStatus = { running: false, port: 8287, fingerprint: "" };
// Wat er NU op deze computer draait namens iemand anders. Zichtbaarheid is een
// van de drie echte knoppen; zonder deze lijst weet je niet wat je toestond.
let sshSessions = [];

async function refreshSshSessions() {
  try { sshSessions = await invoke("ssh_inbound_sessions"); } catch (_) { return; }
  if (!els.sshSessions) return;
  if (!sshSessions.length) {
    els.sshSessions.innerHTML = `<div class="hint">${escapeHtml(t("inbound_none"))}</div>`;
    return;
  }
  els.sshSessions.innerHTML = sshSessions
    .map((s) => `<div class="peer-row" data-sid="${escapeHtml(s.id)}">
      <div class="peer-id"><b>${escapeHtml(s.label)}</b>${s.mirrored ? ` <span class="peer-tag">${escapeHtml(t("inbound_joined"))}</span>` : ""}
        <div class="peer-fp"><code>${escapeHtml(s.what)}</code></div></div>
      <div class="peer-actions"><button class="icon-btn" data-act="kill">${escapeHtml(t("inbound_stop"))}</button></div>
    </div>`)
    .join("");
}

async function refreshSshStatus() {
  try { sshStatus = await invoke("ssh_host_status"); } catch (_) { return; }
  // Het vinkje toont de WENS, niet of de deur toevallig open staat: op een
  // onbekend netwerk blijft hij aan staan en legt de regel eronder uit waarom
  // er niets luistert. Het vinkje zichzelf zien uitzetten is verwarrender.
  const nets = sshStatus.networks || [];
  // Het netwerk vertrouwen is de VOORWAARDE, niet een detail ernaast: zolang
  // niets vertrouwd is, valt er niets aan te zetten. Anders zet je een vinkje
  // aan waar niets van gebeurt -- dat leest als een kapotte functie.
  const netTrusted = nets.some((n) => n.trusted);
  if (els.sshOn) { els.sshOn.checked = sshStatus.desired; els.sshOn.disabled = !netTrusted; }
  if (els.sshPort) { els.sshPort.value = sshStatus.port || 8287; els.sshPort.disabled = !netTrusted; }
  const groep = document.querySelector("#ssh-reach-group");
  if (groep) groep.classList.toggle("locked", !netTrusted);
  if (els.sshState) {
    const lbl = sshStatus.running
      ? t("ssh_on_lbl").replace("{port}", sshStatus.port)
      : !netTrusted
        ? t("ssh_need_trust")
        : sshStatus.desired
          ? t("ssh_blocked_lbl")
          : t("ssh_off_lbl");
    // Gewenst maar niet luisterend is een waarschuwing, geen mededeling: je
    // denkt dat je bereikbaar bent terwijl er niets openstaat.
    els.sshState.className = "stt-state" + (sshStatus.running ? " ok" : sshStatus.desired ? " warn" : "");
    els.sshState.innerHTML =
      `<b>${escapeHtml(lbl)}</b>` +
      (sshStatus.fingerprint
        ? `<div class="hint">${escapeHtml(t("ssh_fp_lbl"))}: <code>${escapeHtml(sshStatus.fingerprint)}</code></div>`
        : "");
  }
  if (els.sshNetworks) {
    els.sshNetworks.innerHTML = nets.length
      ? nets.map((n) => {
          // Wat Windows van het netwerk vindt, staat erbij: op een OPENBAAR
          // netwerk is "laat collega's aankloppen" zelden wat je bedoelde.
          const cat = n.category ? t(`net_cat_${n.category}`) : "";
          const waarschuw = n.category === "public";
          return `<label class="peer-row net-row">
            <span class="peer-id"><b>${escapeHtml(n.name || "?")}</b>
              ${cat ? `<span class="peer-tag${waarschuw ? " blocked" : ""}">${escapeHtml(cat)}</span>` : ""}
              <div class="peer-fp">${escapeHtml(t("net_trust"))}</div></span>
            <input type="checkbox" data-net="${escapeHtml(n.id)}"${n.trusted ? " checked" : ""} />
          </label>`;
        }).join("")
      : `<div class="hint">${escapeHtml(t("net_none"))}</div>`;
  }
}

async function refreshSshPeers() {
  if (!els.sshPeers) return;
  let peers = [];
  try { peers = await invoke("ssh_peers"); } catch (_) {}
  if (!peers.length) {
    els.sshPeers.innerHTML = `<div class="hint">${escapeHtml(t("peers_none"))}</div>`;
    return;
  }
  els.sshPeers.innerHTML = peers
    .map((p) => {
      const tags = [];
      if (p.blocked) tags.push(`<span class="peer-tag blocked">${escapeHtml(t("peer_blocked"))}</span>`);
      if (p.auto_allow) tags.push(`<span class="peer-tag">${escapeHtml(t("peer_auto"))}</span>`);
      return `<div class="peer-row" data-fp="${escapeHtml(p.fingerprint)}">
        <div class="peer-id"><b>${escapeHtml(p.label || "?")}</b> <span class="dim">${escapeHtml(p.address || "")}</span>${tags.join("")}
          <div class="peer-fp"><code>${escapeHtml(p.fingerprint)}</code></div></div>
        <div class="peer-actions">
          <button class="icon-btn" data-act="${p.blocked ? "unblock" : "block"}">${escapeHtml(t(p.blocked ? "peer_unblock" : "peer_block"))}</button>
          <button class="icon-btn" data-act="forget">${escapeHtml(t("peer_forget"))}</button>
        </div></div>`;
    })
    .join("");
}

// Toestemmingsvragen komen als event binnen en worden op volgorde afgehandeld:
// twee popups tegelijk zou betekenen dat je per ongeluk de verkeerde beantwoordt.
const consentQueue = [];
let consentActive = null;
let consentTick = null;

function showNextConsent() {
  if (consentActive || !consentQueue.length) return;
  const req = (consentActive = consentQueue.shift());
  const pairing = req.kind === "pair";
  els.consentTitle.textContent = t(pairing ? "consent_pair_title" : "consent_session_title");
  els.consentWho.textContent = t("consent_who")
    .replace("{user}", req.user || "?")
    .replace("{address}", req.address || "?");
  els.consentWhat.textContent = req.what || "";
  els.consentFp.textContent = req.fingerprint || "";
  // Meekijken kan alleen bij een sessie: bij een pairing is er nog geen terminal.
  els.consentJoin.classList.toggle("hidden", pairing);
  els.consentRemember.checked = false;
  // Vol beheer gaat over wat een SESSIE krijgt; bij een pairing is er nog niets
  // om macht aan te geven. Meekijken geeft het sowieso, dus daar is het geen keuze.
  els.consentFullRow.classList.toggle("hidden", pairing);
  els.consentFull.checked = false;
  els.consentFullWarn.classList.add("hidden");
  els.consentModal.classList.remove("hidden");

  // Uit de backend, zodat teller en server niet uit elkaar lopen.
  let left = req.timeout || 45;
  const paint = () => {
    els.consentTimer.textContent = t("consent_timer").replace("{secs}", left);
  };
  paint();
  clearInterval(consentTick);
  consentTick = setInterval(() => {
    left -= 1;
    paint();
    // De Rust-kant weigert zelf bij het verlopen; hier alleen de popup opruimen.
    if (left <= 0) closeConsent();
  }, 1000);
}

function closeConsent() {
  clearInterval(consentTick);
  consentTick = null;
  consentActive = null;
  els.consentModal.classList.add("hidden");
  showNextConsent();
}

function answerConsent(decision) {
  if (!consentActive) return;
  const id = consentActive.id;
  // "Niet meer vragen" is een sterkere vorm van toestaan, geen apart antwoord.
  let d = decision === "allow" && els.consentRemember.checked ? "always" : decision;
  // Vol beheer is een tweede as: hij zegt niet WIE er binnen mag maar WAT die dan
  // krijgt. Meekijken heeft hem niet nodig -- daar is toezicht de controle (#126).
  if ((d === "allow" || d === "always") && els.consentFull.checked) d += "-full";
  invoke("ssh_consent_reply", { id, decision: d }).catch(() => {});
  closeConsent();
  if (d === "block" || d.startsWith("always")) refreshSshPeers();
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
  settings.tabRecap = els.setTabRecap.checked;
  // 0 = uit. Een aparte aanvinkvakje ernaast zou twee besturingselementen voor
  // een keuze zijn; nul is hier de natuurlijke "nooit bundelen".
  const drempel = parseInt(els.setTabGroupAt.value, 10);
  settings.tabGroupAt = Number.isFinite(drempel) && drempel > 0 ? drempel : DEFAULT_SETTINGS.tabGroupAt;
  settings.tabGroups = Number.isFinite(drempel) && drempel > 0;
  settings.fullPaths = els.setFullPaths.checked;
  settings.persistMode = els.setPersistMode.value;
  settings.skin = els.setSkin.value;
  settings.ttsEnabled = els.ttsOn.checked;
  settings.ttsVoice = els.ttsVoiceSel.value;
  settings.ttsRate = Math.min(10, Math.max(-10, parseInt(els.ttsRate.value) || 0));
  settings.sttAutoSend = els.sttAutoSend.checked;
  settings.sttModel = els.sttModelSel.value;
  settings.sttRegistry = els.sttRegistryInput.value.trim();
  // De SSH-host staat NIET in settings.json: de listener is de waarheid, en die
  // leeft aan de Rust-kant. Hier alleen de gewenste stand doorgeven.
  const wantSsh = els.sshOn.checked;
  const wantPort = Math.min(65535, Math.max(1024, parseInt(els.sshPort.value, 10) || 8287));
  if (wantSsh !== sshStatus.desired || (wantSsh && wantPort !== sshStatus.port)) {
    invoke("ssh_host_set", { enabled: wantSsh, port: wantPort })
      .then((s) => { sshStatus = s; })
      .catch((e) => toast(`${t("ssh_failed")} ${e}`));
  }
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
function blankRow() { return { id: "", label: "", path: "", title: "", task: "", accent: "#7c9cff", mode: "default", agent: "claude", model: "", command: "", host_id: "" }; }
function openEditor() {
  editRows = projects.map((p) => ({ ...p }));
  editorOpen = new Set();
  // Eén enkele (lege) agent hoeft niet ingeklapt: dan is er niets te overzien.
  if (editRows.length === 0) { editRows.push(blankRow()); editorOpen.add(0); }
  renderEditor();
  els.editorStatus.textContent = "";
  els.editorModal.classList.remove("hidden");
}
// Zelfde editor, maar meteen met een verse lege regel (de + bij PROJECTS).
function openEditorAdd() {
  openEditor();
  editRows.push(blankRow());
  editorOpen.add(editRows.length - 1); // verse rij open: die ga je meteen invullen
  renderEditor();
}
// Welke rijen staan open. Een Set van INDEXEN houdt stand over een re-render
// (die de hele lijst opnieuw opbouwt), waar een klasse op het element dat niet
// zou doen.
let editorOpen = new Set();
// Pseudo-agent: "voer mijn eigen commando uit". Staat niet in projects.json --
// daar blijft het `command`-veld de waarheid, en de UI leidt de keuze eruit af.
const AGENT_COMMAND = "__command";

function renderEditor() {
  els.editorRows.innerHTML = "";
  editRows.forEach((r, i) => {
    const open = editorOpen.has(i);
    // Een override VERVANGT de agent, dus hij hoort in de agent-keuze thuis in
    // plaats van als extra veld dat altijd zichtbaar is. Het veld verschijnt
    // alleen bij die keuze; de bestaande waarde in projects.json bepaalt of de
    // rij er al mee begint.
    const hasOverride = !!(r.command || "").trim();
    // Remote: dan is de werkmap een pad OP DIE MACHINE, dus geen lokale
    // bladerknop -- die zou een pad opleveren dat daar niet bestaat.
    const isRemoteRow = !!r.host_id;
    const row = document.createElement("div");
    row.className = "erow" + (open ? " open" : "");
    // Ingeklapt toont een rij precies wat hem identificeert: kleur, label, pad.
    // Uitgeklapt komen de negen velden erbij -- die samen zo'n 350px hoog waren,
    // waardoor er maar twee agents tegelijk op het scherm pasten.
    row.innerHTML = `
      <div class="ehead">
        <input class="e-color" type="color" value="${r.accent || "#7c9cff"}" />
        <button class="e-toggle" aria-expanded="${open}" title="${escapeHtml(t(open ? "row_collapse" : "row_expand"))}">${open ? "▾" : "▸"}</button>
        <div class="ehead-main">
          <div class="ehead-label">${escapeHtml(r.label || t("ph_label"))}</div>
          <div class="ehead-path">${escapeHtml(r.path || "—")}</div>
        </div>
        <button class="e-del" title="${escapeHtml(t("host_del"))}">🗑</button>
      </div>
      <div class="e-fields">
        <div class="e-grid">
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_button"))}</span>
          <input class="e-label" type="text" placeholder="${escapeHtml(t("ph_label"))}" value="${escapeHtml(r.label)}" /></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_host"))}</span>
          <select class="e-host">
            <option value="">${escapeHtml(t("host_local"))}</option>
            ${machineOptions(r.host_id || "").map((o) => `<option value="${escapeHtml(o.id)}"${r.host_id === o.id ? " selected" : ""}>${escapeHtml(o.label)}</option>`).join("")}
          </select></div>
        </div>
        <div class="e-field"><span class="e-cap">${escapeHtml(isRemoteRow ? t("cap_workdir_remote") : t("cap_workdir"))}</span>
          <div class="e-pathrow"><input class="e-path" type="text" placeholder="${escapeHtml(isRemoteRow ? t("ph_path_remote") : t("ph_path"))}" value="${escapeHtml(r.path)}" />${isRemoteRow ? "" : `<button class="e-browse">📁</button>`}</div></div>
        <div class="e-grid">
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_tabtitle"))}</span>
          <input class="e-title" type="text" placeholder="${escapeHtml(t("ph_title"))}" value="${escapeHtml(r.title || "")}" /></div>
        </div>
        <div class="e-field e-taskfield"><span class="e-cap">${escapeHtml(t("cap_task"))}</span>
          <textarea class="e-task" rows="4" placeholder="${escapeHtml(t("ph_task"))}">${escapeHtml(r.task || "")}</textarea></div>
        <div class="e-grid e-grid3">
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_agent"))}</span>
          <select class="e-agent">
            <option value="claude"${(!hasOverride && (r.agent || "claude") === "claude") ? " selected" : ""}>${escapeHtml(t("agent_claude"))}</option>
            <option value="agy"${(!hasOverride && r.agent === "agy") ? " selected" : ""}>${escapeHtml(t("agent_agy"))}</option>
            <option value="${AGENT_COMMAND}"${hasOverride ? " selected" : ""}>${escapeHtml(t("agent_command"))}</option>
          </select></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("cap_model"))}</span>
          <input class="e-model" type="text" list="dl-emodel-${i}" autocomplete="off" placeholder="${escapeHtml(t("model_ph"))}" value="${escapeHtml(r.model || "")}" />
          <datalist id="dl-emodel-${i}">${modelSuggestionsFor(r.agent).map((s) => `<option value="${escapeHtml(suggestionValue(s))}"${typeof s !== "string" && s.key ? ` label="${escapeHtml(t(s.key))}"` : ""}></option>`).join("")}</datalist></div>
        <div class="e-field"><span class="e-cap">${escapeHtml(t("launch_mode"))}</span>
          <select class="e-mode">${modesFor(r.agent).map((o) => `<option value="${o.value}"${clampMode(r.agent, r.mode || "default") === o.value ? " selected" : ""}>${escapeHtml(t(o.key))}</option>`).join("")}</select></div>
        </div>
        <div class="e-field e-cmdfield${hasOverride ? "" : " hidden"}"><span class="e-cap">${escapeHtml(t("cap_command"))}</span>
          <input class="e-command" type="text" placeholder="${escapeHtml(t("command_ph"))}" value="${escapeHtml(r.command || "")}" />
          <span class="e-cmdwarn${hasOverride ? "" : " hidden"}">${escapeHtml(t("command_warn"))}</span></div>
      </div>`;
    // Uitklappen: alleen deze rij, de andere blijven zoals ze staan.
    const toggle = () => {
      if (editorOpen.has(i)) editorOpen.delete(i); else editorOpen.add(i);
      renderEditor();
    };
    row.querySelector(".e-toggle").addEventListener("click", toggle);
    row.querySelector(".ehead-main").addEventListener("click", toggle);
    // De kop toont label en pad; die moeten meelopen terwijl je typt, anders
    // klopt de ingeklapte rij niet meer met wat erin staat.
    row.querySelector(".e-color").addEventListener("input", (e) => (editRows[i].accent = e.target.value));
    row.querySelector(".e-label").addEventListener("input", (e) => {
      editRows[i].label = e.target.value;
      row.querySelector(".ehead-label").textContent = e.target.value || t("ph_label");
    });
    row.querySelector(".e-path").addEventListener("input", (e) => {
      editRows[i].path = e.target.value;
      row.querySelector(".ehead-path").textContent = e.target.value || "—";
    });
    row.querySelector(".e-title").addEventListener("input", (e) => (editRows[i].title = e.target.value));
    row.querySelector(".e-task").addEventListener("input", (e) => (editRows[i].task = e.target.value));
    row.querySelector(".e-mode").addEventListener("change", (e) => (editRows[i].mode = e.target.value));
    row.querySelector(".e-model").addEventListener("input", (e) => (editRows[i].model = e.target.value));
    // Agent wisselen herrendert de rij zodat model-suggesties EN modus-opties
    // meeveranderen; de modus wordt geclampt (claude "plan" bestaat niet voor
    // agy) en het model gewist (een model van de vorige agent is niet geldig).
    row.querySelector(".e-agent").addEventListener("change", (e) => {
      if (e.target.value === AGENT_COMMAND) {
        // Agent blijft staan als terugvalwaarde; de backend negeert hem toch
        // zolang er een override is. Een placeholder zodat het veld niet leeg
        // opent en de rij meteen als override herkenbaar is.
        editRows[i].command = editRows[i].command || "cmd.exe";
      } else {
        editRows[i].agent = e.target.value;
        editRows[i].command = "";
        editRows[i].mode = clampMode(e.target.value, editRows[i].mode || "default");
        editRows[i].model = "";
      }
      renderEditor();
    });
    // Van machine wisselen maakt het bestaande pad betekenisloos: een lokaal
    // pad bestaat niet op de host en omgekeerd. Leegmaken is eerlijker dan een
    // pad laten staan dat straks stil naar de verkeerde map wijst.
    row.querySelector(".e-host").addEventListener("change", (e) => {
      if ((editRows[i].host_id || "") === e.target.value) return;
      editRows[i].host_id = e.target.value;
      const h = hostById(e.target.value);
      editRows[i].path = h ? (h.default_project || "") : "";
      renderEditor();
    });
    const browse = row.querySelector(".e-browse");
    if (browse) browse.addEventListener("click", async () => { const dir = await invoke("pick_folder"); if (dir) { editRows[i].path = dir; renderEditor(); } });
    row.querySelector(".e-del").addEventListener("click", () => {
      editRows.splice(i, 1);
      // editorOpen bevat INDEXEN, en die schuiven op bij een splice: zonder dit
      // klapt na het verwijderen een andere rij open dan je open had staan.
      editorOpen = new Set(
        [...editorOpen].filter((n) => n !== i).map((n) => (n > i ? n - 1 : n))
      );
      renderEditor();
    });
    // Override-veld (#93): schakelt model, modus en taak van DEZE rij uit, want
    // die worden bij een override niet meegestuurd. Geen re-render -- dat zou de
    // cursor uit het veld halen bij elke toetsaanslag.
    const overrideFields = [row.querySelector(".e-model"), row.querySelector(".e-mode"), row.querySelector(".e-task")];
    const cmdWarn = row.querySelector(".e-cmdwarn");
    row.querySelector(".e-command").addEventListener("input", (e) => {
      editRows[i].command = e.target.value;
      applyOverrideState(e.target.value, overrideFields, cmdWarn);
    });
    if (hasOverride) applyOverrideState(r.command, overrideFields, cmdWarn);
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
    .map((r) => ({ id: r.id || slugify(r.label), label: r.label.trim(), path: r.path.trim(), title: (r.title || "").trim(), task: (r.task || "").trim(), accent: r.accent || "#7c9cff", mode: r.mode || "default", agent: AGENTS.includes(r.agent) ? r.agent : "claude", model: (r.model || "").trim(), command: (r.command || "").trim(), host_id: r.host_id || "" }));
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
  if (modalOpen()) { if (e.key === "Escape") { els.settingsModal.classList.add("hidden"); els.editorModal.classList.add("hidden"); closeHostModal(); els.moveModal.classList.add("hidden"); // Escape op de opstartvraag = "niets openen", en dat verliest niets: de geschiedenis blijft (#129).
    document.querySelector("#restore-modal").classList.add("hidden"); } return; }
  if (!els.searchbar.classList.contains("hidden") && e.key === "Escape") { e.preventDefault(); closeSearch(); return; }
  // Een uitgeklapte tabgroep is ook iets dat "open" staat; Escape hoort hem te
  // sluiten voordat de toets naar de terminal gaat (#90).
  if (e.key === "Escape" && (tabPanel || recapTip)) { e.preventDefault(); closeTabPanel(); return; }
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

// De DROPZONE zet een bestand in de input-map van de werkmap en plakt dat pad in
// de prompt. Bij een remote sessie staat die werkmap op de host, dus lokaal
// kopieren levert een pad op dat de agent daar niet kan openen -- hij meldt dan
// "file not found" en niemand legt het verband. Daarom gaat het bestand over met
// scp en komt het pad OP DE HOST in de prompt.
function activeSessionIsRemote() {
  const s = sessions.get(current);
  return !!(s && s.hostId);
}
function updateDropperForSession() {
  if (!els.fileDropper) return;
  els.fileDropper.classList.toggle("dropzone-remote", activeSessionIsRemote());
  els.fileDropper.title = activeSessionIsRemote() ? t("dropper_remote_hint") : "";
}

// Eén bestand naar de host, ongeacht welke zone je raakte: "verplaatsen" zou
// betekenen dat we het lokale origineel na een netwerkoverdracht weggooien, en
// dat is een beslissing die je zelf hoort te nemen, niet een sleepgebaar.
async function dropToRemote(src) {
  const s = sessions.get(current);
  if (!s || !s.hostId) return false;
  toast(t("dropper_sending"), "");
  try {
    const dest = await invoke("scp_to_host", { hostId: s.hostId, src, remoteCwd: s.path });
    insertPathIntoTerminal(dest, true);
    addDropperEntry(dest);
    toast(t("dropper_sent"), "ok");
  } catch (err) {
    dbg(`scp FAIL: ${err}`);
    toast(t("dropper_save_failed") + " " + err, "err");
  }
  return true;
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
    if (activeSessionIsRemote()) {
      for (const src of paths) await dropToRemote(src);
      return;
    }
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
    // Klembord-plakken schrijft eerst lokaal (save_clipboard_to_input) en zou
    // dan nog overgezet moeten worden; die tweetrapsvorm bestaat nog niet.
    if (activeSessionIsRemote()) { toast(t("dropper_paste_local_only"), "err"); return; }
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
  if (activeSessionIsRemote()) {
    // Bij remote is de input-map niet lokaal te openen: begin in de home-map.
    let f = null;
    try { f = await invoke("pick_file", { startDir: "~" }); } catch (_) { return; }
    if (f) await dropToRemote(f);
    return;
  }
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
    setTabRecap: document.querySelector("#set-tabrecap"),
    setTabGroupAt: document.querySelector("#set-tabgroupat"),
    setFullPaths: document.querySelector("#set-fullpaths"),
    setPersistMode: document.querySelector("#set-persist-mode"),
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
    // Taurus als SSH-host (#121)
    sshOn: document.querySelector("#set-ssh-on"),
    sshPort: document.querySelector("#set-ssh-port"),
    sshState: document.querySelector("#ssh-state"),
    sshPeers: document.querySelector("#ssh-peers"),
    sshSessions: document.querySelector("#ssh-sessions"),
    sshNetworks: document.querySelector("#ssh-networks"),
    consentModal: document.querySelector("#consent-modal"),
    consentTitle: document.querySelector("#consent-title"),
    consentWho: document.querySelector("#consent-who"),
    consentWhat: document.querySelector("#consent-what"),
    consentFp: document.querySelector("#consent-fingerprint"),
    consentRemember: document.querySelector("#consent-remember"),
    consentFull: document.querySelector("#consent-full"),
    consentFullRow: document.querySelector("#consent-full-row"),
    consentFullWarn: document.querySelector("#consent-full-warn"),
    consentRememberRow: document.querySelector("#consent-remember-row"),
    consentTimer: document.querySelector("#consent-timer"),
    consentAllow: document.querySelector("#consent-allow"),
    consentJoin: document.querySelector("#consent-join"),
    consentDeny: document.querySelector("#consent-deny"),
    consentBlock: document.querySelector("#consent-block"),
    toast: document.querySelector("#toast"),
    modeInput: document.querySelector("#mode-input"),
    agentInput: document.querySelector("#agent-input"),
    modelInput: document.querySelector("#model-input"),
    modelSuggestions: document.querySelector("#model-suggestions"),
    commandInput: document.querySelector("#command-input"),
    commandWarn: document.querySelector("#command-warn"),
    commandField: document.querySelector("#command-field"),
    moveModal: document.querySelector("#move-modal"),
    mvTarget: document.querySelector("#mv-target"),
    mvPath: document.querySelector("#mv-path"),
    mvSurvey: document.querySelector("#mv-survey"),
    mvWarn: document.querySelector("#mv-warn"),
    mvStatus: document.querySelector("#mv-status"),
    mvGo: document.querySelector("#mv-go"),
    hostModal: document.querySelector("#host-modal"),
    hostRows: document.querySelector("#host-rows"),
    hfForm: document.querySelector("#host-form"),
    hfNickname: document.querySelector("#hf-nickname"),
    hfHostname: document.querySelector("#hf-hostname"),
    hfPort: document.querySelector("#hf-port"),
    hfUser: document.querySelector("#hf-user"),
    hfKey: document.querySelector("#hf-key"),
    hfProject: document.querySelector("#hf-project"),
    hfMux: document.querySelector("#hf-mux"),
    attachBtn: document.querySelector("#attach-btn"),
    attachModal: document.querySelector("#attach-modal"),
    atRefresh: document.querySelector("#at-refresh"),
    atRows: document.querySelector("#at-rows"),
    atStatus: document.querySelector("#at-status"),
    atCancel: document.querySelector("#at-cancel"),
    hfTest: document.querySelector("#hf-test"),
    hostStatusMsg: document.querySelector("#hf-status"),
    hfReport: document.querySelector("#hf-report"),
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
  els.commandInput.addEventListener("input", refreshOverrideState);
  // Twee ingangen: naast de agentlijst (waar je bent als je een machine wilt
  // toevoegen) en in de agents-editor (waar je bent als een agent er een nodig
  // heeft). Alleen die tweede plek bleek onvindbaar.
  document.querySelector("#mv-cancel").addEventListener("click", () => els.moveModal.classList.add("hidden"));
  document.querySelector("#mv-go").addEventListener("click", runMove);
  // Van doel wisselen: het voorgestelde pad hoort bij die machine, niet bij de vorige.
  els.mvTarget.addEventListener("change", () => {
    if (!movePlan) return;
    els.mvPath.value = suggestTargetPath(movePlan.project, els.mvTarget.value);
    refreshMoveSurvey();
  });
  // Pad wijzigen betekent een ander doel: opnieuw meten, maar pas als je even
  // stopt met typen -- anders gaat er een ssh-ronde per toetsaanslag heen.
  let mvPathTimer = null;
  els.mvPath.addEventListener("input", () => {
    clearTimeout(mvPathTimer);
    mvPathTimer = setTimeout(refreshMoveSurvey, 600);
  });
  document.querySelector("#hosts-btn").addEventListener("click", openHostModal);
  document.querySelector("#editor-hosts").addEventListener("click", openHostModal);
  document.querySelector("#host-add").addEventListener("click", openHostForm);
  document.querySelector("#hf-cancel").addEventListener("click", () => els.hfForm.classList.add("hidden"));
  document.querySelector("#hf-test").addEventListener("click", addAndTestHost);
  document.querySelector("#host-close").addEventListener("click", closeHostModal);
  els.attachBtn.addEventListener("click", openAttachModal);
  els.atRefresh.addEventListener("click", () => { loadLocalHistory(); loadRemoteSessions(); });
  els.atCancel.addEventListener("click", () => els.attachModal.classList.add("hidden"));
  // Sleutelkiezer via het bestaande pick_file-command: de dialog-plugin is niet
  // vanuit JS aanroepbaar (staat niet in capabilities/default.json).
  document.querySelector("#hf-key-browse").addEventListener("click", async () => {
    try {
      const f = await invoke("pick_file", { startDir: "~/.ssh" });
      if (f) els.hfKey.value = f;
    } catch (_) {}
  });
  // Andere agent op het startformulier -> model-keuze EN modus-opties mee.
  // Het modelveld wissen: een model van de vorige agent is hier niet geldig en
  // zou de datalist-suggesties wegfilteren (leeg = de default van de agent).
  els.agentInput.addEventListener("change", () => {
    refreshOverrideState();
    if (els.agentInput.value === AGENT_COMMAND) return; // geen model/modus bij een eigen commando
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

  // --- Taurus als SSH-host (#121) ---
  // Een verzoek komt binnen terwijl de gebruiker iets anders doet, dus de popup
  // moet zichzelf tonen: hij is de enige plek waar het antwoord vandaan komt.
  listen("ssh-consent", (ev) => { consentQueue.push(ev.payload || {}); showNextConsent(); });
  // De Rust-kant heeft zelf al geweigerd (time-out); popup weghalen.
  listen("ssh-consent-done", (ev) => {
    const id = ev.payload;
    if (consentActive && consentActive.id === id) closeConsent();
    else {
      const i = consentQueue.findIndex((c) => c.id === id);
      if (i >= 0) consentQueue.splice(i, 1);
    }
  });
  listen("ssh-host-changed", () => refreshSshStatus());
  els.consentAllow.addEventListener("click", () => answerConsent("allow"));
  // De waarschuwing verschijnt pas als je het vinkje zet: hij hoort bij die keuze,
  // en permanent zichtbaar zou hij wegkijken worden.
  els.consentFull.addEventListener("change", () => {
    els.consentFullWarn.classList.toggle("hidden", !els.consentFull.checked);
  });
  els.consentJoin.addEventListener("click", () => answerConsent("join"));
  els.consentDeny.addEventListener("click", () => answerConsent("deny"));
  els.consentBlock.addEventListener("click", () => answerConsent("block"));
  els.sshPeers.addEventListener("click", async (e) => {
    const btn = e.target.closest("button[data-act]");
    if (!btn) return;
    const fp = btn.closest(".peer-row")?.dataset.fp;
    if (!fp) return;
    try {
      if (btn.dataset.act === "forget") await invoke("ssh_peer_forget", { fingerprint: fp });
      else await invoke("ssh_peer_set", { fingerprint: fp, blocked: btn.dataset.act === "block" });
    } catch (err) { toast("✗ " + err, "err"); }
    refreshSshPeers();
  });
  els.sshNetworks.addEventListener("change", async (e) => {
    const box = e.target.closest("input[data-net]");
    if (!box) return;
    try { sshStatus = await invoke("ssh_network_trust", { id: box.dataset.net, trusted: box.checked }); }
    catch (err) { toast("✗ " + err, "err"); }
    refreshSshStatus();
  });
  els.sshSessions.addEventListener("click", async (e) => {
    const btn = e.target.closest('button[data-act="kill"]');
    if (!btn) return;
    const sid = btn.closest(".peer-row")?.dataset.sid;
    if (!sid) return;
    try { await invoke("ssh_kill_session", { id: sid }); } catch (err) { toast("✗ " + err, "err"); }
    refreshSshSessions();
  });
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
  // Ook los aanroepen: zonder branding.json komt applyBranding niet aan de
  // titel toe, en dan zou een testexemplaar er ongemarkeerd bij staan.
  markTestInstance();
  renderTabs();
  loadProjects();
  // Hosts eerst: het herstellen moet een opgeslagen host_id kunnen opzoeken.
  loadHosts().then(startupRestore);
  // Stond er nog een hand omhoog toen de app sloot? Dan hoort de balk er te
  // staan -- een vraag die je niet meer ziet trek je ook niet in (#125).
  syncAskingBanner();

  // Toon de app-versie discreet onderin de sidebar.
  invoke("app_version").then((v) => { if (v) els.appVersion.textContent = "v" + v; }).catch(() => {});
});
