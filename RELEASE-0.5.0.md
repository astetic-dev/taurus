## 🖥 Remote agents — run an agent on another machine, in the same window

▶ **[Watch the explainer](https://youtu.be/_Q_6gRCFEYc)** (60 seconds)

Taurus 0.5.0 lets a tab run its agent **somewhere else**. Not in a cloud — on your
own machines, over your own SSH.

**The point:** your folders do not move. The agent goes to them. Server logs on a
support box, a build machine that can reach things your laptop cannot, a jump host
that exists so traffic stays put — an agent there works *with the access that
machine already has*.

### What you can do

- **Add a machine once.** One connection test reports what is actually there: which
  OS, whether an agent CLI is installed, whether the box can reach
  `api.anthropic.com` at all, and whether a session survives a dropped connection.
  A machine that fails the test is not saved.
- **Give an agent a machine.** An agent card is a workplace — a machine plus a
  folder — so "runs on" sits with the agent, not with starting it. Local is what you
  get unless the agent says otherwise. Its tab carries a badge for where it runs.
- **Keep working after the connection drops.** With tmux on the host the session
  keeps going; reconnect and you reattach to the same one, mid-task. A Windows host
  with WSL gets that for free — pick **In WSL** and Taurus uses the tmux that is
  already there.
- **Hand files to a remote agent.** The DROPZONE sends them over scp to the agent's
  input folder and pastes the path *on that machine* into the prompt.
- **Move an agent to another machine, folder and all.** Right-click a tab or an
  agent card. Taurus measures the folder first and shows what it is about to copy —
  project files, work folders (`input`, `output`, `log`, `review`) and rebuildable
  ones (`.git`, `node_modules`, `target`) each with their own size. Untick what
  should not travel. Works in both directions, and the source folder always stays
  where it is.

### Also in this release

- **Command override is now a choice, not a field.** Pick **Own command…** in the
  agent list and the field appears; the model, mode and task grey out, because a
  command override drops all of them. Previously they stayed editable and looked
  like they applied, which quietly ran agents on their default model (#93).
- **The agents editor fits more than two agents.** Rows collapse to colour, label
  and path; click one to open it. The task is a proper text area now, and every
  control shares one style (#100).
- **Agents no longer inherit the launcher's Claude session variables.** A Taurus
  started from inside an agent session handed those on, which turned off transcript
  saving — and without a transcript, session restore silently stopped working
  (#101).

### ⬇️ Download (Windows x64)

👉 https://github.com/astetic-dev/taurus/releases/tag/v0.5.0

| | |
|---|---|
| Installer | `Taurus_0.5.0_x64-setup.exe` |
| MSI | `Taurus_0.5.0_x64_en-US.msi` |
| Portable | `taurus.exe` |

Setup for a remote machine is two prompts you paste into an agent — see
[REMOTE-HOSTS.md](REMOTE-HOSTS.md).

Implements #98, #93, #100, #101, #102.
