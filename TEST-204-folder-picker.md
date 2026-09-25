# Test #204: folder picker no longer hangs the app

## macOS (Mac mini)

Build under test: branch `fix/dialog-main-thread-hang`. The Taurus window the Mac session started is already this build.

- [x] Open a folder: the macOS panel appears.
- [x] Go to the home folder (`arjen`) and click "New Folder": the panel asks for a name, and the app keeps responding (no spinning wheel). *Mac: confirmed, the wheel is gone.*
- [ ] Choose the new folder: Taurus fills in the start form for that folder.
- [ ] Browse button in the project editor: choosing a folder fills in the path.

Found during this test, not caused by #204 (next topic in #199, starting agents):

- "New process": `No such file or directory (os error 2)`.
- Starting an agent: `kon claude niet starten: Unable to spawn claude.exe because it doesn't exist on the filesystem and was not found in PATH`.

- [ ] Dropzone +: the file picker opens, and choosing a file works.
- [ ] Cancel in both pickers: nothing happens, and the app keeps responding.

## Windows (workstation)

These commands now run on the thread pool instead of the UI thread: `pick_folder`, `pick_file`, `list_agent_models`, `stt_toggle`, `firewall_status`, `firewall_allow`, `remote_agents`, `stop_remote_session`, `list_tts_voices`.

- [ ] Browse for a folder, and choose a file through the dropzone +: works as before.
- [ ] SSH key picker (machines screen): opens in `~/.ssh`.
- [ ] Model list for a new session: loads (or falls back to the built-in list).
- [ ] Speech-to-text: start and stop a recording, and the text arrives.
- [ ] Firewall status on the host screen, and "create rules": works as before.
