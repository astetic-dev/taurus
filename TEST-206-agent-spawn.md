# Test #206: agents start on macOS

## macOS (Mac mini)

The Taurus window the Mac session started runs this build. It was started with the bare `PATH` a Finder launch gets (`/usr/bin:/bin:/usr/sbin:/sbin`).

- [ ] Start an agent (Claude) in a folder: the tab shows Claude Code, not "kon claude niet starten".
- [ ] Try "New process" again: note the error here if it still appears.
- [ ] Close Taurus, and start it from Finder (Programs or the `.app` in `src-tauri/target/debug/bundle/macos`): an agent still starts.

## Windows (workstation)

Only `cfg(not(windows))` code and platform-aware tests changed. On Windows, `agent_exe`, `resolve_in_paths`, `ssh_program` and `scp_program` are the same as before.

- [ ] `cargo test --lib` is green.
- [ ] Starting an agent works as before (native exe and npm `.cmd` shim).
