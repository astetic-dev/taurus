# Taurus — Agent Launcher

[![Taurus — Agent Launcher (click to watch the explainer)](media/screenshot.png)](https://youtu.be/ofswaJBX39k)

<sub>▶ Click the image to watch the explainer on YouTube.</sub>

A small Tauri desktop app that runs and manages multiple **Claude Code** agents as
**terminal tabs in one window**.

_▶ Watch: [15-second teaser](https://youtu.be/7WDtN5giKSk) · [full explainer](https://youtu.be/ofswaJBX39k)._ Each agent starts in the working folder you pick,
so you never have to wonder whether you're on a local (`C:`) or network (`X:`) path.

Pick a project on the left, see the folder + a **LOCAL / NETWORK** badge, give the
session a title, choose a mode, and hit **Start** — the agent opens as an embedded
terminal tab. Open several, switch between them, and let a tab flash in its own
colour when an agent is done and waiting for you.

> Windows-only for now (uses ConPTY + Windows Terminal-style embedding).

## Why Taurus

Giving Claude the *right* context at the *right* moment is still one of the hardest
parts of working with coding agents day to day.

Elaborate memory systems help a single power user, but they don't scale to an
enterprise: they're hard to curate, easy to pollute, and brittle as the number of
people and projects grows. Honestly, the wheel hasn't been invented yet — nobody
has a clean, proven answer for how context should work at team scale.

Taurus is built around the **[Interpreted Context Methodology (ICM)](https://github.com/RinDig/Interpreted-Context-Methdology)** — *"folder structure as agent architecture."* Instead of orchestration code or sprawling memory, context lives in the **folders** themselves: a `CLAUDE.md`, conventions, and reference material that load in layers the moment an agent starts there. Approaches built around one person's bespoke setup are expensive to onboard a whole team onto; a folder anyone can open is not.

Taurus' contribution is the **entry point**: let everyone start the agent in the process folder they actually need. From there ICM does the rest, so:

- the **user** immediately knows *where they should be* for a given task, and
- **Claude** starts with exactly the scoped context that folder carries — giving
  repeatable results across people and runs.

It isn't the final answer to context-at-scale, but it removes the biggest piece of
day-to-day friction: people land in the right place, and the agent behaves
consistently. That's what Taurus is for.

## Features

- **Embedded terminals with tabs** — each agent runs inside the window (xterm.js +
  ConPTY via `portable-pty`); click tabs to switch.
- **Attention flash** — a tab flashes in its own colour only when its agent has
  finished and is waiting for input (a working agent shows a steady green dot).
- **Live status** — optionally shows Claude's current activity verb on the tab.
- **In-app project editor** — add/edit/remove launch buttons (name, folder with a
  browse dialog, colour, default title/task, mode).
- **Per-project mode** — start in `default`, `plan` or `auto` (`--permission-mode`).
- **Per-project agent + model** — launch `claude` (Claude Code) or `agy` (a
  Gemini-backed agent CLI), optionally pinned to a model, set per project and
  overridable per session.
- **Inline HTML preview** — right-click a tab → *HTML preview*, or click an `.html`
  path in the terminal; it renders beside (or instead of) the terminal.
- **Restart / resume** — right-click a tab to restart the session and resume the
  same conversation (handy after updating an MCP server).
- **Open folder in Explorer**, **search** (Ctrl+Shift+F), **copy/paste**, font zoom
  (Ctrl+= / - / 0), tab shortcuts (Ctrl+Tab, Ctrl+1..9, Ctrl+T/W) — all toggleable.
- **EN / NL** language switch.
- **White-label & skins** — rebrand from a local `branding.json` (name, logo,
  colours), and pick a visual skin (retro Mac/Windows, XP, Aqua, green-CRT
  terminal) in Settings. See [Branding](#branding-white-label).

## Requirements

- [Node.js](https://nodejs.org/) and [Rust](https://rustup.rs/) (MSVC toolchain)
- WebView2 runtime (preinstalled on Windows 11)
- Windows Terminal (`wt`) — optional
- The **Claude Code CLI** (`claude.exe`) available on your `PATH`

## Develop

```powershell
npm install
npm run tauri dev
```

## Build (installer + standalone exe)

```powershell
npm run tauri build
```

Output:
- standalone: `src-tauri/target/release/taurus.exe`
- installers: `src-tauri/target/release/bundle/` (NSIS `*-setup.exe`, MSI)

## Configuration

Projects live in `%APPDATA%\Taurus\projects.json` (per user, created on first run).
Edit them with the in-app **Projects** editor, or by hand. Format:

```json
[
  {
    "id": "my-id",
    "label": "Button name",
    "path": "C:\\path\\to\\folder",
    "title": "Default session title",
    "task": "",
    "accent": "#7c9cff",
    "mode": "default",
    "agent": "claude",
    "model": "",
    "command": ""
  }
]
```

- `agent` (optional) — which agent CLI to launch: `claude` (default, Claude Code)
  or `agy` (a Gemini-backed agent CLI). Selectable per project in the editor and
  overridable per session on the launch form.
- `model` (optional) — model the agent starts with (free text, e.g. `opus`,
  `sonnet`, `gemini-2.5-pro`). Empty means the agent's own default. Passed as
  `--model`.
- `command` (optional) — run a different program instead of the agent for this
  project (verbatim, no agent flags). Takes precedence over `agent`/`model`.
- A fresh install starts with an **empty** list (no baked-in paths). UI settings
  (language, font, toggles) are kept in the WebView2 local storage.

## Branding (white-label)

You can rebrand the launcher — app name, subtitle, logo, colours, skin, window
title — **without forking or editing code**. Copy
[`branding.example.json`](branding.example.json) to `%APPDATA%\Taurus\branding.json`
(next to `projects.json`) and edit it. Without the file the build is plain
Taurus; remove it to get Taurus back. Every field is optional:

```json
{
  "appName": "Acme Agent Launcher",
  "subtitle": "Acme Corp",
  "logo": "C:\\ProgramData\\Acme\\logo.png",
  "windowTitle": "Acme Agent Launcher",
  "skin": "default",
  "theme": { "--accent": "#3b82f6", "--bg": "#0b1220", "--bg-panel": "#11182a" }
}
```

- `theme` overrides the CSS variables in `src/styles.css` `:root` — the themeable
  ones are `--bg`, `--bg-panel`, `--bg-card`, `--bg-card-hover`, `--border`,
  `--text`, `--text-dim`, `--accent` (and `--term-bg` / `--term-fg` / `--term-sel`
  for the terminal).
- `skin` sets the **default theme** (see *Skins* below).
- `font` sets the UI font family (e.g. `"'IBM Plex Mono', monospace"`). IBM Plex
  Mono ships bundled (offline); any installed/CSS font name also works.
- `garble` (`true`/`false`) toggles the hover **letter-scramble** effect on the
  sidebar buttons, project cards and tab titles. It's on by default for a branded
  theme; set `false` to disable.
- `logo` is an absolute path; it's read at startup and inlined as a data URI.
- The in-app branding above is runtime/config-driven. To also change the
  **installer** product name / identifier (baked into the bundle), build with a
  Tauri config overlay: `npm run tauri build -- --config <overlay>.json`.

### Skins

Built-in visual themes that restyle the sidebar, top bar, buttons and terminal.
Pick one live under **⚙ Settings → Theme**, or set a default via `"skin": "…"`
in `branding.json` (an explicit Settings choice wins). Available skins:

| value | look |
|-------|------|
| `default` | the standard dark theme |
| `retro-mac` | classic Mac System 7 / Platinum (grey, beveled, pinstripe) |
| `aqua` | early Mac OS X (pinstripes, glossy lozenge buttons) |
| `retro-win` | Windows 95/98 (teal desktop, raised 3D grey controls) |
| `winxp` | Windows XP Luna (blue bars, glossy buttons, green primary) |
| `terminal` | green-on-black CRT (monochrome, scanlines) |
| `nord` | modern — Nord arctic blue-grey (dark) |
| `dracula` | modern — Dracula (dark, vibrant) |
| `solarized` | modern — Solarized Light (warm, light) |
| `catppuccin` | modern — Catppuccin Mocha (soft pastel, dark) |

Skins are pure CSS (`src/skins.css`, scoped by `html[data-skin="…"]`) — adding
more is just another block. The retro skins adapt techniques from the MIT
projects [98.css](https://jdan.github.io/98.css/) / XP.css / system.css; the
modern skins are palette-only.

> Keep brand assets out of this repo: `branding.json` and `src/*-logo.png` are
> git-ignored. A real brand lives only in the local config, never committed.

### Portable / pre-branded distribution

`branding.json` is read from `%APPDATA%\Taurus\` **or**, as a fallback, from the
folder next to `taurus.exe`. A relative `logo` path resolves against the folder
the `branding.json` was found in. So a self-contained, pre-branded build is just
a folder you can zip and hand out:

```
MyBrand Agent Launcher/
  taurus.exe          (rename freely, e.g. "MyBrand Agent Launcher.exe")
  branding.json       ("logo": "logo.png", relative)
  logo.png
```

Unzip and run — no setup. A `branding.json` in `%APPDATA%\Taurus\` takes
precedence over the one next to the exe. To also rebrand the installer's product
name, build with a Tauri config overlay (`--config`).

## How it works

- Frontend: `src/` — vanilla HTML/CSS/JS (xterm.js vendored in `src/vendor/`).
- Backend: `src-tauri/src/lib.rs` — Rust commands for project config, the native
  folder picker, and the ConPTY sessions (`create/restart/write/resize/close`).
