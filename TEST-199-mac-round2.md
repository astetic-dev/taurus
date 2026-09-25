# Test #199, round 2: the Mac port after #204-#226

**First, restart your Taurus window.** Your window (and this Claude session) runs the build from 18:05. The new build is already on disk at the same place, `src-tauri/target/debug/bundle/macos/Taurus.app`. Close Taurus yourself and start it from Finder: that also tests the Finder launch. Closing Taurus also ends this Claude session. Start it again afterwards (`claude --resume` in `~/dev/taurus`).

Everything below is merged into `main`. Tick what works, and write what doesn't underneath.

## Starting and saving (#206, #208)

- [ ] Started from Finder: a Claude agent starts in a folder (no "claude.exe").
- [ ] No "Openstaande sessies opslaan mislukt" when you start or close a tab.
- [ ] After restarting Taurus, the open tabs come back (sessions.json in `~/Library/Application Support/Taurus`).
- [ ] "New process" no longer gives `os error 2`.

## Dropzone and paste (#210, #212, #214)

- [ ] Drag a file onto the dropzone: "Move" and "Copy" light up where you point, and the file lands in `<working folder>/input`.
- [ ] Copy a file in Finder (⌘C), then "Paste object": the file itself arrives (e.g. `taurus-bundle.log`), not a `pasted-*.png`.
- [ ] Dropzone +: the file picker opens in `<working folder>/input`. There is no folder named `…\input` next to the working folder.

## Preview and texts (#214)

- [ ] In a local Mac session, click a `/Users/…/something.md` or `.html` in the terminal: the preview opens it.
- [ ] Tab menu: "Show folder in Finder" opens Finder.
- [ ] The agent list no longer shows "unknown location" behind local folders.

## Keys and menu (#216)

- [ ] ⌘Q asks for confirmation when agents are running (the same question as the close button).
- [ ] ⌘W closes the current tab, not the window.
- [ ] ⌘T new tab, ⌘1..9 jump to a tab, ⌃Tab switch tabs.
- [ ] ⌘+ / ⌘- / ⌘0 font size.
- [ ] In the terminal: select text, ⌘C, ⌘V pastes it. ⌃C still interrupts.
- [ ] ⌘C / ⌘V in text fields (project editor) work.
- [ ] Settings show ⌘ instead of Ctrl.

## Closing a tab (#218)

- [ ] Start an agent that runs MCP servers, then close the tab. Afterwards `ps -ax | grep -i mcp` (in a separate Terminal) shows nothing of that session.

## Inbound SSH and network (#220)

- [ ] Settings → SSH host: your network appears as `Wi-Fi · 192.168.2.254 (home)` and can be trusted.
- [ ] Trusted: the Windows machine finds the Mac in discovery. (Only if you want to test this now.)

## Speech (#222)

- [ ] Settings → Voice: the voice list shows macOS voices (e.g. Xander — Dutch), and "Test" speaks.
- [ ] Speech-to-text is not offered on the Mac (deliberate, see #222).

## Windows (for the Windows session)

Merged with "Windows-check nodig": #205, #207, #209, #215, #219, #221, #223. On the Windows workstation: `cargo test --lib` is green, and a short round of starting an agent, the folder picker, the dropzone, TTS and the firewall screen.
