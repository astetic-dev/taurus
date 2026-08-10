## 🗂 A tab bar that survives ten agents — and tells you what each one is doing

Taurus 0.5.1 fixes the two things that got worse the more agents you ran: a tab bar
you could no longer read, and no way to tell which of those tabs needed you.

**The point:** with a dozen sessions open, "which agent is waiting for me" should be
answerable without clicking through tabs one by one.

### Tabs that group themselves

- **Above ten tabs, sessions from the same agent collapse into one.** The threshold
  is a setting; below it nothing changes. The grouped tab carries a count.
- **Hover it — or click it — and it expands** into a panel listing its sessions.
  Clicking one switches to it; each row keeps its own close button.
- **It groups by agent, not by folder.** Two agent cards that happen to share a
  working directory stay apart, because *Porter on Sonnet* and *Porter+ on Fable*
  are not the same thing. Sessions of one card group even when their model or mode
  differs — that difference shows up in the recap instead.
- **Sessions you started by browsing to a folder** have no agent card, so they
  collect in one bucket of their own.
- **A grouped tab still flashes when one of its sessions is waiting for you.** That
  was the condition for shipping this at all: collapsing tabs must never hide a
  background agent that finished.

### A recap when you hover a tab

- **Hovering any tab shows the last thing that agent said** — grouped or not, and
  including tabs that are not on screen.
- **It opens with what the session was started with:** `claude · sonnet · plan`, or
  the command when a session uses an override. With members of one group differing
  in model and mode, that line is how you tell them apart.
- **Nothing is sent to the agent to produce it.** The recap is read from that
  session's own terminal view, which Taurus keeps current for every session whether
  its tab is visible or not. Claude Code's own chrome — the input box, the hint bar,
  the separators — is filtered out, so you get the answer and not the furniture.

### Synchronising a folder to another machine

- **"Copy & move" is now "Synchronize".** The old label promised a move while the
  dialog itself said the source folder stays where it is. It was never a move: you
  tick what should be on the other machine, and it puts it there.
- **The transfer reports progress.** A 1.7 GB working directory used to sit on one
  static line for minutes with no sign of whether it was still going. It now reads
  `Synchronizing — node_modules (3/12, 41%)`, weighted by the sizes the dialog
  already measured before starting.

### Known limitations

- **Progress within a single item stays invisible.** `scp` only prints its own meter
  to a terminal, and Taurus captures that output. One very large item still looks
  static — though you now know it is item 3 of 12.
- **`Ctrl+1..9` still indexes all sessions**, not the tabs you see. With grouping on,
  position on screen and shortcut number no longer line up.
- **Sessions saved by an earlier version come back once as ungrouped.** They predate
  the field that records which agent they belong to; restarting them sorts it out.

### ⬇️ Download (Windows x64)

| | |
|---|---|
| Installer | `Taurus_0.5.1_x64-setup.exe` |
| MSI | `Taurus_0.5.1_x64_en-US.msi` |
| Portable | `taurus.exe` |

Implements #90 and #107.
