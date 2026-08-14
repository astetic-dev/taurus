# Remote hosts — setup guide

Taurus can run an agent on another machine and show it in a tab, over SSH. This
is the setup for one host, start to finish.

You need three things: an SSH key on your workstation, that key authorised on
the target machine, and an entry in `hosts.json`. The two prompts below do all
of it — one for the agent on your workstation, one for an agent on the target
machine.

Everything here is verified against a real pair of Windows machines. The traps
called out in the prompts are ones that actually cost time, not hypotheticals.

---

## Step 1 — on your workstation

Paste this into a Claude Code session on the machine where Taurus runs.

> I want to connect Taurus to a remote host over SSH. Do the workstation half of
> the setup:
>
> 1. Check whether `%USERPROFILE%\.ssh\taurus_ed25519` already exists. If it
>    does, do not overwrite it — just show me the public key.
> 2. If it does not exist, create an ed25519 key pair there with **no
>    passphrase** and the comment `taurus-launcher`.
>    **Trap:** in PowerShell, `ssh-keygen -N '""'` produces a key with a literal
>    `""` passphrase, and it fails later with "Server accepts key" followed by
>    "Permission denied". Generate it in WSL (`ssh-keygen -t ed25519 -f /tmp/k
>    -N "" -q`) or otherwise guarantee the passphrase is truly empty, then
>    verify with `ssh-keygen -y -P "" -f <key>` — that must print the public key
>    without prompting.
> 3. Restrict the private key's ACL so OpenSSH accepts it:
>    `icacls <key> /inheritance:r /grant:r "$env:USERNAME:F"`
> 4. Print the **public** key on one line so I can copy it to the other machine.
>
> Do not touch any existing keys in `~/.ssh`.

## Step 2 — on the target machine

Open Claude Code **on the machine you want to reach** and paste this, replacing
the placeholder with the public key from step 1.

> I want this machine to accept SSH connections from my Taurus launcher. Set up
> the host half:
>
> 1. Is OpenSSH Server installed and running? If not, install and enable it:
>    `Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0`
>    `Start-Service sshd; Set-Service sshd -StartupType Automatic`
>    (needs an elevated shell — tell me if you cannot elevate)
> 2. Authorise this public key:
>    `<PASTE THE PUBLIC KEY FROM STEP 1 HERE>`
>    **Trap:** check first whether my account is in the Administrators group.
>    If it is, sshd **ignores** `%USERPROFILE%\.ssh\authorized_keys` entirely
>    and only reads `C:\ProgramData\ssh\administrators_authorized_keys`, which
>    must be owned by Administrators/SYSTEM with inheritance removed. Putting
>    the key in the wrong file is the most common reason key auth silently
>    fails. If unsure, write it to both.
>    **Trap:** make sure the line has no trailing carriage return and the file
>    has no BOM — piping a key through PowerShell adds both.
> 3. Confirm the firewall allows inbound TCP 22 from the local network.
> 4. Report: this machine's IP address, my username, whether `claude` is on
>    PATH and its version, and whether `herdr`, `tmux` or `psmux` exists.
>    (`herdr` is the one that also gives reattach on Windows — see the herdr
>    section below if none of them are there.)
> 5. Check outbound HTTPS works — this is a kill switch, no outbound means no
>    agent can run here:
>    `curl.exe -sS -o NUL -w "%{http_code}" --max-time 10 https://api.anthropic.com/`
>    Any HTTP status (401/404 included) means the connection works.
> 6. If `claude` is missing, tell me — do not install it without asking.
>
> Report the results; do not change anything else.

## Step 3 — back on your workstation

> The other machine is ready. Finish the Taurus side:
>
> - host: `<IP>`, user: `<USERNAME>`, and it reported OS `<windows|linux>`
> - Verify the connection first:
>   `ssh -i <key> -o BatchMode=yes -o StrictHostKeyChecking=accept-new <user>@<ip> "hostname"`
>   **Trap:** always use `BatchMode=yes` when testing. Without it, ssh waits on
>   a password prompt that never gets answered and appears to hang.
> - Then add the host to `%APPDATA%\Taurus\hosts.json` (create the file as a
>   JSON array if it does not exist), using `mux: "none"` unless the host
>   reported tmux or psmux.
> - Restart Taurus and confirm the host appears under "Runs on".

---

## The `hosts.json` entry

```json
[
  {
    "id": "ursu",
    "nickname": "ursu",
    "hostname": "192.168.2.9",
    "user": "arjen",
    "port": 22,
    "key_path": "C:\\Users\\you\\.ssh\\taurus_ed25519",
    "default_project": "C:\\Users\\arjen",
    "os": "windows",
    "mux": "none"
  }
]
```

| field | meaning |
|---|---|
| `id` | internal key; sessions reference it, so keep it stable |
| `key_path` | path to the **private** key. Empty = let ssh decide (`~/.ssh/config`, agent) |
| `default_project` | working directory **on the host**, prefilled in the launch form |
| `os` | `windows` or `linux` — decides how the command is quoted for the remote shell |
| `mux` | `herdr`, `tmux`, `psmux` or `none` — what keeps the session alive. `taurus-agent` is still accepted but superseded by `herdr` |
| `via` | empty, or `wsl` to run the agent inside WSL on a Windows host |
| `machine` | which *physical* machine this is. Empty = fall back to `hostname` |

## A machine is a machine, not a route (#124)

An entry above is really a **route**, not a machine: the same computer appears once per
way of reaching it — its sshd on 22, WSL on 2223, the Taurus host on 8287. That forced
the distinction into the *name* ("ursu (Taurus-host)"), and from there it leaked into tab
badges, the machine dropdowns and every agent card pointing at it.

The machine screen shows **one row per machine**, with the routes small underneath, each
with its own signal dot:

```
● ursu                                 ⇱ agents   [Nieuwe agent...]
    ○ sshd :22        windows · herdr
    ● Taurus :8287    voorkeur   windows · none
      ● Ontwikkel     claude · C:\Users\arjen\ontwikkelmap   [Aanhaken]
      ○ 2 lege sessies die Taurus liet staan               [Opruimen]
```

- Routes to the same address collapse automatically, so an existing `hosts.json` needs no
  rewrite. Set `machine` explicitly when one computer answers on two addresses.
- The name shown is the **shortest** nickname among the routes — the long one is exactly
  the disambiguating suffix being removed.
- The **Taurus route wins** when a machine offers one: it needs no key exchange and no
  sshd on the other side.
- Dropdowns and badges name the machine, never the route.

**Nieuwe agent...** opens the normal launch form with that machine's folder filled in. It
used to be called Connect and started something nameless in a folder you had not chosen,
which is how you ended up in C:\Users\arjen instead of at your work.

**⇱ agents** lists the **agents** running on that machine — that is the only noun on this
screen. ssh, tmux and herdr clear the way so an agent can start; they are plumbing, and
they are never something you pick. The route is a detail on the machine row.

Two sources answer, and to the person looking they are the same thing:

- agents Taurus started there over SSH, which herdr knows about;
- agents running in the Taurus **on** that machine, read from the `sessions.json` it
  already rewrites on every change.

The same folder seen from both sides collapses to one row. An agent living inside the
other Taurus is shown but not offered — there is no channel to it yet, and the row says so
rather than failing on click.

**No agent means there is nothing to connect to.** Not a choice with a warning label on
it: no choice. Empty mux sessions still exist, so they sit under the agents as cleanup
with a two-click removal — `herdr session stop` followed by `herdr session delete`, since
stop alone leaves the entry in the list, which was the original complaint. A machine
without Taurus can only answer with what herdr knows, and says so, so the difference stays
visible instead of being papered over.

Note that a machine has **three** kinds of "session" and they are not the same list:

| where you see it | what it is |
|---|---|
| ⇱ on the machine screen | agents on that machine |
| Settings → Netwerk on that machine | inbound sessions a peer opened *on* it (#121) |
| ⇱ in the sidebar | your own work: this computer first, then your machines |

## Coming back to your work (#129)

The ⇱ button in the sidebar is where you return to work, and it is explicitly **yours**:
this computer first, then the machines you configured. A colleague never appears here —
they raise a hand, and that is the section further down.

Startup no longer silently resumes. It asks, with what was open pre-ticked and the rest of
the history below it. An entry that cannot resume right now keeps its row and shows why —
no transcript, machine gone, N days old — instead of vanishing, because vanishing was the
bug: a failed restore used to erase the only record that the session had existed. History
lives in its own `history.json`, is added to and updated, and is never trimmed because a
restart did not work. "Open nothing" and Escape lose nothing.

The setting under Settings → Sessies has three states: **ask** (default), **silently
resume**, and **start clean**. Clean only empties the open-list; the history stays.

## What `mux` buys you

| value | effect |
|---|---|
| `herdr` | same as tmux, **and it works on Windows and macOS too**. The preferred value: "Add & test" picks it whenever the host has it |
| `tmux` / `psmux` | session survives a dropped connection, and reconnecting reattaches to the same agent instead of starting a second one. `tmux` does not exist on Windows |
| `none` | no reattach. On **Windows** the agent still survives, because Windows' sshd does not kill the process tree when the connection drops. On **Linux** it does not — there, the transcript (`claude --resume`) is the only persistence, so you lose the in-flight turn |

### herdr — the one that also works on Windows

[herdr](https://herdr.dev) is an Apache-2.0 terminal runtime for coding agents: a
background server that owns the terminals, with clients attaching and detaching.
That is exactly what a remote tab needs, and unlike tmux it exists on all three
platforms. Install it on the host and nothing else changes:

```
# Linux / macOS
curl -fsSL https://herdr.dev/install.sh | sh
# Windows
powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"
```

Both installers verify a SHA-256 from a release manifest, install a single binary
without root, and register no service. Then re-test the host from the 🖥 button;
`mux` flips to `herdr` by itself.

Taurus gives every (host, project) pair **its own herdr session** — a namespace
with its own server and socket — which is the same unit tmux gets. Inside a fresh
session the first pane is always `w1:p1`, so starting a tab is a short handshake
of existence checks: is the server up, does the pane exist, is the agent already
running. Each step is separately idempotent, so a half-torn-down session (server
alive, agent gone) repairs itself on the next launch instead of putting a second
agent next to the first.

Two things are worth knowing, both measured rather than read:

- **On Windows the session TUI does the attaching.** `herdr agent attach` answers
  *"direct terminal attach is not supported on Windows yet"* (0.8.0-preview), so a
  Windows tab attaches to the session instead — and that draws herdr's own
  sidebar (spaces, agents) and tab row inside your tab. In a window that already
  has tabs and an agent list that is a second copy of both, and the sidebar takes
  the mouse, so you cannot select text in it.

  "Add & test" therefore turns that chrome off on Windows hosts, once, by adding
  to the machine's `%APPDATA%\herdr\config.toml`:

  ```toml
  [ui]
  sidebar_start_collapsed = true
  sidebar_collapsed_mode = "hidden"
  hide_tab_bar_when_single_tab = true
  ```

  The `[ui]` header is not optional: without it herdr rejects the keys as unknown
  and silently ignores them. Taurus keeps a `.taurus.bak`, never touches the
  keys if `sidebar_start_collapsed` is already set (your own choice wins), and
  validates the result with `herdr config check` — if that reports a problem the
  backup goes back.

  **This happens on Linux and macOS too** (`~/.config/herdr/config.toml`), and
  for the same reason. Attaching goes straight to the agent terminal only while
  an agent is actually running in the pane; once it has exited — the normal
  state of a session you come back to, and what ⇱ lists as *"no agent"* — the
  tab falls back to the session view and the chrome is right back. herdr reads
  the config when a session starts, so a session that is already running keeps
  its chrome until it is recreated.
- **Windows builds are preview-only.** herdr refuses `channel set stable` there
  until stable Windows releases exist. That is the trade for reattach on a
  Windows host; `mux` is per host, so anything you are not comfortable with can
  stay on `none`.

If herdr does not recognise the program you launch — `agy`, or a `command`
override that runs something else — there is no agent to attach to, and the tab
falls back to the session view. It works; it just shows a little more chrome.

### Choosing it yourself

The add form has a **Session persistence** dropdown: *Automatic*, `herdr`, `tmux / psmux`, or *None*. Automatic is the default and takes whatever the test finds, preferring herdr. The test reports everything it found, not just the winner, so the choice is informed rather than a guess.

Picking something the machine does not have fails the test instead of being saved — that error would otherwise surface at the first session, from a shell three layers down. *None* is always allowed: it is the honest setting for a machine where you would rather not leave an agent process running.

A host you left on *Automatic* follows the probe every time you re-test it, which is the point of re-testing after installing something. A host where you picked a value keeps it: re-testing must not quietly undo your choice. That is what `mux_auto` in `hosts.json` records; an entry from before this existed counts as automatic.

### Connecting to a session that is already running

⇱ beside ＋ in the AGENTS sidebar opens **Connect to a running session**: pick a machine, see what is actually running on it — session name, agent and its state, working directory — and attach to one.

This is the deliberate way back in. Starting an agent card the normal way only reattaches when the session name happens to match, which it does not for a session started from another workstation, from a plain terminal, or from a card that has since been renamed.

Attaching creates nothing: no workspace, no `pane run`. Re-running the agent inside a session that is mid-turn would overwrite that turn. A session whose agent has exited is still listed and still attachable — you land in the pane and can start something there.

Such a tab is not restartable or movable, and it is not remembered for the next start: Taurus did not build that command and cannot rebuild it. The session keeps running on the host regardless; ⇱ brings you back.

### Running in WSL (`via: wsl`)

Not offered in the form any more — herdr made it unnecessary. A `hosts.json` that says `via: "wsl"` still works, so this remains available as a hand-edit for the case where the work belongs in Linux anyway.

Windows has no tmux, so a Windows host could not reattach — WSL was the way
around it. **Installing herdr on the Windows side is now the simpler answer**: it
keeps the DROPZONE working and the agent sees real Windows paths instead of
`/mnt/c`. Set `"via": "wsl"` by hand on a host whose work belongs in Linux
anyway; "Add & test" still reports whether that machine's WSL has a multiplexer
and an agent CLI.

What it costs:

- the agent lives in Linux and reaches Windows drives through `/mnt/c`, which is
  slow for git and file watching
- the DROPZONE cannot send files to a `via: wsl` host: the scp server runs on
  Windows and cannot write into WSL's ext4 filesystem

### WSL stops on its own — and `vmIdleTimeout` does not fix it

Measured on a real machine: a WSL distro shuts down roughly 20–30 seconds after
the last process in it exits, **even with `vmIdleTimeout=-1`**. That setting
governs the utility VM, not how long a distro instance stays alive; WSL stops a
distro once nothing is running in it.

That matters in two ways:

- an SSH server *inside* WSL is only reachable while WSL happens to be running,
  so a host on that port works intermittently at best
- a session you expected to persist ends when the distro does — unless something
  keeps it alive, which a running tmux session does by itself

The fix is a process that never exits, started at boot. A Windows scheduled task
with trigger *At startup*, set to **run whether the user is logged on or not**,
running as the account you connect with:

```powershell
# sshd is both the service and the keep-alive
wsl -d Ubuntu-24.04 -u root -- /usr/sbin/sshd -D

# or, with [boot] systemd=true in /etc/wsl.conf, just keep the distro up
wsl -d Ubuntu-24.04 -- sleep infinity
```

With that in place the machine is simply a **Linux host** on that port —
`os: linux`, no `via` — and the DROPZONE works to it as well. Without it,
`via: wsl` over the Windows SSH server is the more reliable route, because that
starts WSL on demand.

## Making *this* computer reachable

Everything above is Taurus reaching out. The reverse — a colleague reaching **your**
machine — needs no OpenSSH Server and none of the traps above. Settings → Netwerk →
**"Let colleagues start a session on this computer"** starts an SSH server inside
Taurus itself, on port **8287**.

It is off by default. A Taurus host is an ordinary host to the other side — "Add & test"
probes it, agent tabs work, the DROPZONE transfers files, ⇱ lists what is running.

**What it starts is an agent, not a shell.** The agent runs *in* a shell, so speaking of
it that way is fine, but a bare prompt on someone else's machine is never the product —
that is as true inbound as it already was outbound, where Taurus asks every host for
`ssh -t host "<agent>"` and not for a login. A request carrying a command line runs that
line, because it stood in the popup and was approved as such; a request without one starts
the agent in the folder it begins in.

What it is for:

- **Handing off a run.** Your usage limit is hit mid-task; a colleague finishes it from
  their desk.
- **Working together, or teaching.** Two people on one agent session — see *Join* below.
- **Your own other machines.** A lab box or second workstation that runs Taurus anyway
  is reachable without a dedicated sshd setup.

### First: trust the network

The setting only takes effect on a network you have marked as trusted. Settings →
Netwerk lists the networks this machine is on, each with a **Trust this network** box —
tick your office LAN, leave café Wi-Fi alone. Taurus follows you around: the listener
closes when you land on an unknown network and reopens when you are back, without you
touching the setting. The checkbox keeps showing what you *want*; the line underneath
says whether anything is actually listening, because a checkbox that appears to switch
itself off is worse than one that explains itself.

Networks are remembered by the GUID Windows' Network List Manager assigns them — the
same source as the Private/Public prompt — so renaming an SSID does not lose the
decision. Network changes are noticed by polling every 15 seconds rather than by a COM
event sink: you walk between networks, you do not switch them every second.

Be clear-eyed about what this is: a guard against *accidents*, not a security boundary.
A hostile network can advertise a trusted network's name. Identity remains the key
fingerprint and the consent popups; this only stops you listening somewhere you never
meant to.

### Asking for help (#125)

Two sides of one rule, and mDNS only exists on the second:

| | your own machines | a colleague's machine |
|---|---|---|
| how it is found | you configure it, in `hosts.json` | mDNS, and only while they are asking |
| what you may do | start an agent, unattended; return to one that runs | join the one agent they offer |
| consent | your own computer — pair once | their invitation *is* the consent, per session |
| take it over? | yes, it is yours | **no** — the work stays in their session |

So mDNS is not a directory of machines. It announces **a request, while the request is
open** — the way a Bluetooth device is discoverable while pairing and invisible the rest
of the time. A Taurus that is merely reachable announces nothing.

Right-click a tab of a session running on this computer and pick **Vraag om hulp**. The
announcement carries one agent: who is asking, which machine, which agent, its folder,
the host key fingerprint and a one-time token. A quiet bar under the tab bar shows the
hand is up, with *Intrekken*. Nothing interrupts you — you switched it on yourself.

On every other Taurus with the machine screen open, the request appears at the top under
**Iemand vraagt hulp**. Answering connects with the token as the SSH username; the asking
side recognises it during authentication and lets it in **without a pairing popup**,
because the person who would have to answer that popup is the one who asked. That
acceptance is deliberately narrow: with a help token, reading along with the offered
session is the only thing the connection can do. Anything else is refused and audited,
and the ordinary consent path for ordinary sessions is untouched.

**Joining starts nothing.** You land in the terminal that is already running on the other
machine, and what you type goes into it — two keyboards on one agent, the same shape as
#121's join. Taking the work over onto your own machine is not possible, on purpose: it
stays in the session of the person with the problem, which keeps "who owns half-finished
work" from ever being a question.

An empty list is a real answer here: nobody needs help. That is worth more than the old
list, which said who happened to be switched on.

**It needs its own firewall rule.** Every mDNS allow rule Windows ships is scoped to a
program (`svchost.exe` for its own responder, `msedgewebview2.exe` for Edge); Taurus is
neither, so `taurus.exe` on UDP 5353 needs an exception of its own, next to the TCP 8287
one the listener already needs. The machine screen checks for both — counting only rules
that actually apply to this executable, and counting **block** rules against it, which
beat any allow — and offers to fix all of it in one elevated step.

The announcement binds to the trusted interface only. mDNS libraries will happily
announce every address on every adapter; measured here that meant the Hyper-V and WSL
internal ranges going out to the LAN, telling everyone how this machine is carved up
inside and offering nothing the other side can reach.

There is no port scan and there will not be one, not even as a button: on a segment where
multicast is blocked, that machine stays a hand-made entry. That is a better answer than
shipping something that behaves like a network sweep.

### Who gets in: pairing, then per session

Consent lives in the GUI instead of in `authorized_keys`, in two steps.

1. **An unknown key** produces a pairing popup: claimed username, address, and the key
   fingerprint → **Deny / Allow / Block**. Allowed fingerprints are remembered in
   `%APPDATA%\Taurus\peers.json`, and the pair appears under "Paired computers".
2. **A session request** produces a second popup naming what is being asked →
   **Deny / Allow / Join**. Tick *"Don't ask again for this computer"* on a machine of
   your own, where nobody is sitting there to click Allow.

**Block** is the mute button for a colleague who keeps knocking: further attempts from
that fingerprint are refused at the auth step with no popup at all, though they still
land in the audit trail. Blocking a paired computer also revokes its pairing. It works
per key fingerprint — someone who generates a fresh key gets a fresh pairing popup, so
treat it as a mute, not an access control list.

No answer within 45 seconds counts as **Deny**. A popup nobody sees must never grant
access.

The name in the popup is a claim the client makes up; the **fingerprint** is the only
identity. That is why it is shown, and why Deny is the default outcome.

### Join — two keyboards, one agent

**Join** is Allow plus a tab on your own screen showing that same session. It is not a
screen share: Taurus owns the terminal, so both sides read and write the *same* one.
Whatever the colleague types appears in your tab, and whatever you type lands in their
session. That is the point — for pair-working with an agent, and for guiding someone who
does not know the tool yet.

The tab appears as soon as the session produces output, marked 👥. Two windows rarely
have the same size, so the **smaller one wins**, the way tmux handles it: someone loses
empty space rather than having text run off screen.

Closing the joined tab only stops you watching; the colleague's session continues. It is
not remembered across restarts either — there is no command Taurus could replay to
rebuild it. Ending the session itself is a separate, deliberate action: Settings →
Netwerk → *Running on this computer right now* → **Stop**, which also lists every
inbound session whether you joined it or not.

### What is *not* restricted, and what replaces it

There is no command filter, and that is a decision rather than an omission. The session
exists to run an agent with shell access; "no delete actions" cannot be enforced across
cmd, PowerShell, .NET and the agent's own tools, and a filter that does not hold is
worse than none because it reads like safety. The controls that do hold:

- **consent**, per pairing and per session;
- an **audit trail** under `%APPDATA%\Taurus\audit\` — `events.log` records every
  authentication, decision and non-interactive command, and each session gets its own
  full transcript;
- **visibility**: connected peers are shown, and turning the setting off closes the
  listener and ends inbound sessions;
- for real restriction: the agent's own permission mode (`plan` / ask) and OS ACLs.

An audit transcript is exactly what went over the wire, so a token someone pastes into
a session is in that file. The files stay local, under your own profile.

### Power follows supervision (#126)

Every session getting the same amount of room flattened two situations that are not
alike: one where you joined and are watching every keystroke, and one where you allowed
it and walked away. How much the agent may do on its own is now tied to **supervision**
rather than to trust:

| | what it starts | why |
|---|---|---|
| **Join** | the agent in its own mode | you see every keystroke in a mirrored tab; watching *is* the control |
| **Allow**, unattended | the agent in `dontAsk` mode, in the folder it starts in | nobody is looking, so it must not stall on a prompt and must not wander |

Both start an agent — that part never varies. A checkbox on the session popup — *Vol
beheer: de agent zonder rem* — gives an unattended session the same room a joined one
has, and the warning next to it says what that grants rather than asking "are you sure".
The audit line records which of the two was given, so `session-allow … [vol beheer]` and
`session-allow … [agent, geen shell]` are distinguishable afterwards.

**Be clear about what that unattended mode is.** It is a *structural* difference: nothing
hands out a bare prompt, and `--permission-mode dontAsk` means the agent never prompts and
denies whatever was not pre-approved. That is the right direction to fail in when nobody
is watching — plan mode would execute nothing at all, so the session could not do the work
it was allowed to do, and a prompt nobody answers is a hung session rather than a safe one.
It is **not an OS boundary.** The agent can still run commands and its tools can touch paths outside the
working directory. The containment is exactly as strong as the agent's own permission
model — a real mechanism, but one that belongs to the agent, not to Taurus. A real
boundary would need a separate restricted Windows account, or a Job Object / AppContainer
with filesystem restrictions; both are heavy, and neither is in here.

One more thing that is deliberately unchanged: an **exec request** (`ssh -t host "…"`,
which is how Taurus opens a tab) runs what was asked, unattended too. That is not a gap
in the above but the other half of the same principle — that command line was shown in
the popup and approved as such. Second-guessing it afterwards would be command filtering,
and the paragraph above says why that is not done.

### Things worth knowing

- **Inbound sessions are never elevated.** The embedded server does no Windows logon:
  publickey only, and everything runs as the account Taurus runs as. That is an
  improvement on stock Windows sshd, which gives an Administrators-group account an
  elevated session.
- **The agent uses your credential.** In the usage-limit hand-off, a colleague finishing
  your run keeps burning *your* (exhausted) limit unless they switch account inside the
  session with the agent's own `/login`.
- **The first start triggers the Windows Firewall prompt.** Allow it on Private
  networks; it is effectively a second consent layer.
- **Sessions live in Taurus.** A dropped connection loses nothing — reconnecting lands in
  the same terminal — but closing Taurus ends them, exactly like local sessions (#77). A
  peer who needs launcher-independent persistence sets `mux: herdr` against this host;
  the two compose, with herdr keeping the agent and Taurus keeping the door.
- Port 8287 sits in an unassigned IANA range and is configurable; the connecting side
  already has a `port` field in `hosts.json`.

## Moving an agent to another machine

Right-click a tab or an agent card → **Move to another machine**. An agent is a
workplace — a machine plus a folder — so it cannot move without its folder.

Before anything is sent, Taurus measures the working directory and splits it in
three, because how big it is decides whether you want to wait:

| | |
|---|---|
| project files | always included |
| `input`, `output`, `log`, `logs`, `review` | each with its own size, **on** by default — usually exactly what the agent needs |
| `.git`, `node_modules`, `target`, `dist`, `.venv`, `__pycache__` | each with its own size, **off** by default — usually the bulk, and rebuilt on the other side in a moment |

Untick what should not travel; the total updates as you go. Each item is copied
separately rather than with one `scp -r` on the whole tree, which is what makes
leaving something out possible at all.

**The source folder is not deleted.** "Move" describes the agent; the files are
copied. Removing a source tree after a network transfer is a different promise,
and one wrong path makes it unrecoverable. The agent is repointed only after a
successful copy, so a failure leaves it pointing at a folder that exists.

A non-empty target directory is refused rather than merged into. Bringing an
agent back to this computer, and moving between two remote hosts, are not
supported yet.

## Security notes

- Host keys use `StrictHostKeyChecking=accept-new`: an unknown host is accepted
  once, a **changed** key is refused. That is deliberate — silently accepting a
  changed key would throw away MITM protection in a tool that reaches
  management hosts.
- An unknown host id in a session is an error, never a silent fall back to
  local. Falling back would start a remote tab on your workstation, with your
  workstation's credentials.
- The agent on the target machine needs its own Anthropic credential. That is a
  second secret to manage, separate from the SSH key. Decide per host class what
  an agent there is allowed to do before the first one runs — `auto` on your own
  and QA machines, `plan` or nothing on customer environments.
- On Windows, a session opened by sshd for an account in the Administrators
  group runs elevated. Check what you are handing the agent.
