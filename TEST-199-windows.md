# Test #199: Windows check after the Mac port (main @ 4ea1269 and later)

Everything below is merged into `main`. It is written on the Mac and has not run on Windows yet. Tick what works, and write what doesn't underneath (with the PR number). Build as usual into a fresh `target` directory and restart your own window.

## 0. Build and unit tests

- [ ] `git pull`, then `cargo test --lib` in `src-tauri`: green.
- [ ] `npx tauri build`: builds without new warnings.

## 1. Basics that must not have changed (#205, #207, #209, #219)

- [ ] Starting an agent works (native `claude.exe` and an npm `.cmd` shim).
- [ ] Config is still in `%APPDATA%\Taurus`: projects and open tabs come back after a restart.
- [ ] Closing a tab ends the agent and its MCP servers (Job Object; Task Manager shows nothing left over).
- [ ] Folder picker (browse), dropzone + file picker, and SSH key picker (opens in `~/.ssh`) respond and don't freeze the window.
- [ ] Model list for a new session loads (or falls back to the built-in list).
- [ ] Machines/host screen: firewall status and "create rules" work as before.

## 2. Paths and texts (#215)

- [ ] Clicking a `C:\...\file.md` or `.html` in the terminal opens the preview.
- [ ] Dropzone +: the picker opens in `<working folder>\input`.
- [ ] The agent list shows `(C:)` / `(X:)` as before.
- [ ] Tab menu "Open folder in Explorer" opens Explorer.

## 3. Keys (#230)

- [ ] Ctrl+T new tab, Ctrl+W close tab, Ctrl+1 jump to tab 1: work as before.

## 4. Speech (#223, #232, #237, #239)

- [ ] Settings → Voice: the voice list shows the Windows voices, and "Test" speaks.
- [ ] Speech-to-text: the status line ends in "· Windows x64". Download fetches the win-x64 engine, and dictation works.
- [ ] Without a microphone (or with it disabled): clicking 🎙 gives one message "Geen microfoon beschikbaar…". It does not come back by itself, and a click on the message removes it.

## 5. Machines (#235)

- [ ] Retest the Mac on the machines screen: it shows **macOS**, not linux.

## 6. Looks (#241, #244, #246, #248, #250, #254, #257)

- [ ] **System** is the default theme. With Windows on light, Taurus is light; switch Windows to dark and Taurus follows, including the terminal, without a restart.
- [ ] Accent: change it in Settings → Personalization → Colors, click back into Taurus, and buttons/links take that colour.
- [ ] Theme list: no Catppuccin, and "Aqua (2001)" instead of "macOS Aqua". A saved Catppuccin choice shows as Dracula.
- [ ] Settings → Theme → **Icons**: System defaults to "Line" (line icons on the buttons). "Standard" brings the old icons back.
- [ ] "See-through sidebar" is **not** visible on Windows (Mac only).
- [ ] The logo at the top left is the bull from the app icon. A click collapses it to a small logo with "Taurus" and "Agent Launcher" next to it; another click expands it, and the choice survives a restart.

## Known, not built on Windows

- Mica behind the sidebar (#242).
