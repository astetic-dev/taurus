# Test #199, round 2: the Mac port after #204-#226

**First, restart your Taurus window.** Your window (and this Claude session) runs the build from 18:05. The new build is already on disk at the same place, `src-tauri/target/debug/bundle/macos/Taurus.app`. Close Taurus yourself and start it from Finder: that also tests the Finder launch. Closing Taurus also ends this Claude session. Start it again afterwards (`claude --resume` in `~/dev/taurus`).

Everything below is merged into `main`. Tick what works, and write what doesn't underneath.

## Starting and saving (#206, #208)

- [x] Started from Finder: a Claude agent starts in a folder (no "claude.exe").
- [x] No "Openstaande sessies opslaan mislukt" when you start or close a tab.
- [x] After restarting Taurus, the open tabs come back (sessions.json in `~/Library/Application Support/Taurus`).
- [x] "New process" no longer gives `os error 2`.

## Dropzone and paste (#210, #212, #214)

- [x] Drag a file onto the dropzone: "Move" and "Copy" light up where you point, and the file lands in `<working folder>/input`.
- [x] Copy a file in Finder (⌘C), then "Paste object": the file itself arrives (e.g. `taurus-bundle.log`), not a `pasted-*.png`.
- [-] Dropzone +: the file picker opens in `<working folder>/input`. There is no folder named `…\input` next to the working folder.
It opens I the folder and that is ok..

## Preview and texts (#214)

- [x] In a local Mac session, click a `/Users/…/something.md` or `.html` in the terminal: the preview opens it.
- [x] Tab menu: "Show folder in Finder" opens Finder.
- [x] The agent list no longer shows "unknown location" behind local folders.

## Keys and menu (#216)

- [x] ⌘Q asks for confirmation when agents are running (the same question as the close button).
- [x] ⌘W closes the current tab, not the window.
- [-] ⌘T new tab, ⌘1..9 jump to a tab, ⌃Tab switch tabs.
- [x] ⌘+ / ⌘- / ⌘0 font size.
- [x] In the terminal: select text, ⌘C, ⌘V pastes it. ⌃C still interrupts.
- [x] ⌘C / ⌘V in text fields (project editor) work.
- [x] Settings show ⌘ instead of Ctrl.

## Closing a tab (#218)

- [ ] Start an agent that runs MCP servers, then close the tab. Afterwards `ps -ax | grep -i mcp` (in a separate Terminal) shows nothing of that session.

## Inbound SSH and network (#220)

- [x] Settings → SSH host: your network appears as `Wi-Fi · 192.168.2.254 (home)` and can be trusted.
- [ ] Trusted: the Windows machine finds the Mac in discovery. (Only if you want to test this now.)

## Speech (#222)

- [x] Settings → Voice: the voice list shows macOS voices (e.g. Xander — Dutch), and "Test" speaks.
- [ ] Speech-to-text is not offered on the Mac (deliberate, see #222).

## Windows (for the Windows session)

Merged with "Windows-check nodig": #205, #207, #209, #215, #219, #221, #223. On the Windows workstation: `cargo test --lib` is green, and a short round of starting an agent, the folder picker, the dropzone, TTS and the firewall screen.

~/dev/taurus/src-tauri/target/debug/bundle/macos/Taurus.app

## Round 2b: tab menu (#229) and dictation (#231)

Restart your Taurus window first (the new build is at the same place).

- [ ] Menu bar: File has New Tab ⌘T and Close Tab ⌘W; Window has Show Previous/Next Tab (⌘⇧[ / ⌘⇧]) and Tab 1..9.
- [ ] ⌘T opens the new-agent screen, ⌘1 / ⌘2 jump to a tab without a beep, ⌘⇧] / ⌘⇧[ cycle. ⌃Tab (Control, not ⌘) cycles too. ⌘Tab stays the macOS app switcher.
- [ ] Settings → Voice: speech to text is visible. Click Download: it downloads about 540 MB (engine plus model), and the status turns "ready".
- [ ] Click 🎙 (or hold fn+F9): macOS asks for microphone permission once, with the Taurus text. Allow it.
- [ ] Say a sentence: the text appears in the active terminal.
