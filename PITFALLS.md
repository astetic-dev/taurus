# Measured pitfalls

Things that cost real time to find, measured on real machines rather than taken from documentation. Read the section for the area you are about to change. When you measure a new one, add it here with what breaks and what to do instead.

## Build and release

- **Frontend changes can ship stale.** `src/` is embedded into the exe at compile time. A change that only touches `src/*.js` or `index.html` can leave cargo thinking the crate is unchanged. `touch src-tauri/src/lib.rs` before a release build to force the re-embed.
- **A running exe is locked (Windows).** Cargo cannot link over a `taurus.exe` that is running. Build to `target/` and let the owner swap and restart their own copy (see the working agreements in `CLAUDE.md`). The version number does not change per build, so builds are told apart by build time, which `start-taurus-test.ps1` prints.
- **Git Bash rewrites paths in environment variables.** A value like `/srv/project` in an env var becomes a Windows path before cargo or a test sees it. Prefix with `MSYS_NO_PATHCONV=1`.
- **Double backslashes in a Bash heredoc collapse to one** in the agent's Bash tool on Windows, even with a quoted delimiter. That broke JSON (`"C:\\x"` became `"C:\x"`) and JS regexes (`/[\\/]/`). Write such files with an editor tool or a script that uses raw strings, or use forward slashes.
- **Long base64 payloads on a command line** hit "The command line is too long" on Windows. Copy the script to the host and run it by path.

## Security choices that look like mistakes

- **CSP keeps `'unsafe-inline'` in `script-src` on purpose.** The preview renders in `srcdoc` iframes, which inherit the parent CSP, and the preview runs inline scripts by design. `dangerousDisableAssetCspModification: ["script-src","style-src"]` stops Tauri's injected hashes from cancelling `unsafe-inline`. Don't tighten this without redesigning the preview.
- **Speech-to-text downloads require sha256 pins**, because the engine is an executable. Custom registry entries must carry `engineSha256` / `modelSha256`, or the backend refuses them.

## Sessions and the pty

- **pty events carry a generation.** `pty-output` is `(id, gen, base64chunk)` and `pty-exit` is `(id, gen)`. The frontend drops events whose `gen` does not match `session.gen`; that is the fix for the restart race (#71). Keep the shape.
- **Process-tree cleanup (Windows).** Sessions run inside a Job Object with kill-on-close, so the whole tree (agent plus MCP servers) dies with the session or with Taurus, including on a crash (#77). Other platforms need their own mechanism (#199).

## Inbound SSH host (#121)

Measured against OpenSSH for Windows 9.5p2 as the client.

- **russh's default crypto backend does not build on Windows.** `aws-lc-rs` panics with "NASM command not found". Keep `default-features = false, features = ["ring", "flate2", "rsa"]`. A careless `cargo add russh` during an upgrade breaks the build on every machine without NASM.
- **A client without a terminal requests a 0x0 pty** (e.g. `ssh -tt` with stdin on NUL). Passing that on, even clamped to 1x1, makes ConPTY hang forever. Clamp 0 to 80x24.
- **`channel_eof` must not close the pty input.** Dropping the writer closes ConPTY's input pipe, and `cmd.exe` dies at once with 0xC000013A (STATUS_CONTROL_C_EXIT) before anything has run. A real terminal has no stdin EOF, and sshd behaves the same way.
- **Without an exit-status on the sftp channel, scp reports a successful transfer as failed** (exit code 1, "Exit status -1"). Sending it when the sftp stream ends is too late, because the channel is already closing. Send it from the `channel_eof` handler. `russh_sftp::server::run()` spawns internally and returns at once, so there is nothing to await.
- **SFTP paths arrive in three forms**: `C:/x`, `/C:/x` and `C:\x`. `fs::canonicalize` returns a `\\?\` prefix that confuses clients; strip it.
- **Port 8287** is unassigned in the IANA registry (range 8283-8291 is free) and spells TAUR on a keypad. 2222 and 2223 are taken (EtherNet/IP, Rockwell CSP2).
- **Network gate (Windows):** `INetworkListManager` via the `windows` crate (features `Win32_Networking_NetworkListManager`, `Win32_System_Com`, `Win32_System_Variant`). `IEnumNetworks::Next` takes `Some(&mut u32)` as its second argument, not `&mut u32`. It yields a stable GUID per network.
- **The far side is someone's desktop, not a service.** Two things only showed up in real use: the consent popup showed the raw `-EncodedCommand` (kilobytes of base64) for a yes/no decision, and the herdr server opened a visible console window on the host. Anything started for a remote peer must be readable in the popup and invisible on the host's desktop.

## herdr mux (#115)

Measured on a Windows host and under WSL.

- **No attach-or-create in one command.** `herdr workspace create` is not idempotent (the same `--label` twice gives w1 and w2), and workspace ids are not reused (after closing w1 and w2 comes w3). Always resolve by label: `workspace list`, find the label, filter `pane list` on `workspace_id`, then `agent attach <pane>`. Otherwise `workspace create`, `pane run`, `agent attach`. In a fresh session the first pane is always `w1:p1`, so the whole flow can be exit-code checks without JSON parsing in a shell payload.
- **Windows detach trap.** A background process started with `Start-Process` from an sshd session is killed when the SSH connection closes. `Invoke-CimMethod -ClassName Win32_Process -MethodName Create` reparents it outside the session, and then it survives (same PID across two connections).
- **Detection takes about a second** after `pane run`. Attaching without a wait loop fails with "agent target not found". Programs herdr does not know get no agent entry, so fall back to the session TUI.
- **`agent attach` does not exist on Windows yet** (herdr 0.8.0-preview, upstream herdrdev/herdr#2726). There the session TUI attaches, drawing herdr's sidebar and tab bar and capturing the mouse. The host config hides them, and **the `[ui]` section header is required**, otherwise herdr rejects `sidebar_start_collapsed`, `sidebar_collapsed_mode = "hidden"` and `hide_tab_bar_when_single_tab` as unknown keys. On POSIX hosts this does not apply: the tab attaches straight to the agent terminal.
- **herdr's status detection beats our heuristic.** It saw Claude Code's trust prompt as `blocked`, while Taurus' spinner detection saw nothing and reported "done" after 5 s. `herdr agent explain <pane>` shows which rule matched.
- **Local mux was investigated and parked** (Aug 2026, around #168). It is feasible: `herdr_windows_script()` already does the detached start, and the Job Object from #77 stays intact because the agent lives under the herdr server, not under our child process. It still needs a mux setting for local sessions (not a fake host), a non-ssh path for list/stop, and our own attach-or-create.

## Testing the Windows app at runtime

- **Isolated test instance.** Launch the debug exe with `WEBVIEW2_USER_DATA_FOLDER=<fresh dir>` (otherwise it joins the running instance's browser process) and `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`. Move `%APPDATA%\Taurus\sessions.json` aside first and restore it afterwards, otherwise the test instance resumes live sessions. Drive it over CDP (`Runtime.evaluate`).
- **Window screenshots of WebView2** need `PrintWindow` with `PW_RENDERFULLCONTENT`; `CopyFromScreen` renders the webview black. Capture only the Taurus window.
