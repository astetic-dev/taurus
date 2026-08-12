## 🐘 Sessions that outlive everything — including your workstation

Taurus 0.5.0 let an agent run on another machine. 0.5.2 makes that session *stay*
there, on Windows too, and gives you a deliberate way back to it.

**The point:** an agent you started on a remote box should still be working when
you come back — and you should be able to find it without guessing.

Special thanks to **David Vogel**, who pointed at [herdr](https://herdr.dev) and
said this is what should be underneath. He was right.

### herdr as a persistence backend

- **Reattach now works on Windows hosts.** `mux` promised four options and
  delivered two: tmux does not exist on Windows, psmux is niche, and
  `taurus-agent` was a placeholder for a runtime nobody built. herdr is that
  runtime — one Apache-2.0 Rust binary, on Windows, Linux and macOS — so a
  dropped connection no longer costs you the turn in flight.
- **"Add & test" prefers it** when the host has it, and reports every multiplexer
  it found rather than only the winner.
- **No WSL detour needed.** That workaround existed because Windows had no tmux;
  it cost you `/mnt/c` performance and a DROPZONE that could not reach ext4.
  It still works if you set `via: "wsl"` by hand, but it is no longer the road to
  persistence.
- **One herdr session per (host, project)**, the same unit tmux got. Every step of
  the launch is separately idempotent, so a session whose agent died repairs
  itself on the next start instead of growing a second agent beside the first.

### ⇱ Connect to a session that is already running

- **Ask a machine what it is running.** Session name, which agent, what state it
  is in, and its working directory — then attach to the one you pick.
- **This is the deliberate way back in.** Before, you could only reattach by
  accident: by launching the same agent card with the same folder so the session
  name happened to match. A session started from another workstation, from a
  plain terminal, or from a card since renamed was simply unreachable.
- **Attaching creates nothing.** No workspace, no agent start — re-running the
  agent inside a session that is mid-turn would overwrite that turn.
- **A session whose agent has exited is still listed and still attachable.** You
  land in the pane and can start something there.
- Such a tab is not restartable, not movable and not remembered for the next
  start: Taurus did not build that command and cannot rebuild it. The session
  keeps running on the host regardless.

### Machines: persistence is a choice now

- **Automatic / herdr / tmux / None**, instead of whatever the probe decided.
  Automatic stays the default and behaves as before.
- **Picking something the machine does not have fails the test** instead of being
  saved — that error would otherwise surface at the first session, from a shell
  three layers down.
- **None is always allowed.** Not wanting a long-lived agent process on a machine
  is a legitimate choice, not a mistake.
- **Re-testing no longer undoes your choice.** A host left on Automatic follows
  each new measurement, which is the point of re-testing after installing
  something; a pinned host keeps its value.
- **herdr's own chrome is taken out of a Windows tab.** Attaching there goes
  through herdr's session TUI, which draws its sidebar and tab row inside a
  window that already has both — and takes the mouse, so that region cannot be
  selected. "Add & test" turns it off on the host, once, only when you have not
  set it yourself, with a backup, and validated afterwards with herdr's own
  config check.

### Notes

- herdr is installed on the host, not by Taurus:
  `curl -fsSL https://herdr.dev/install.sh | sh`, or the PowerShell one-liner on
  Windows. Both verify a SHA-256 and install a single binary without root.
- Windows builds of herdr are preview-only, which is the trade for reattach on a
  Windows host. `mux` is per machine, so anything you are not comfortable with
  can stay on `none`.
- On Windows, herdr's `agent attach` is not implemented yet
  ([herdrdev/herdr#2726](https://github.com/herdrdev/herdr/issues/2726)), so a
  tab there attaches to the session instead. Linux and macOS attach to the agent
  terminal directly.

Closes #115, #117, #118.
