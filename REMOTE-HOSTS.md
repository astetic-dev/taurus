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
>    PATH and its version, and whether `tmux` or `psmux` exists.
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
| `mux` | `tmux`, `psmux`, `taurus-agent` or `none` — what keeps the session alive |
| `via` | empty, or `wsl` to run the agent inside WSL on a Windows host |

## What `mux` buys you

| value | effect |
|---|---|
| `tmux` / `psmux` | session survives a dropped connection, and reconnecting reattaches to the same agent instead of starting a second one |
| `none` | no reattach. On **Windows** the agent still survives, because Windows' sshd does not kill the process tree when the connection drops. On **Linux** it does not — there, the transcript (`claude --resume`) is the only persistence, so you lose the in-flight turn |

### Running in WSL (`via: wsl`)

Windows has no tmux, so a Windows host normally cannot reattach. But a Windows
host that has WSL usually already has one: pick **In WSL** in the add form and
the agent runs there, with real tmux persistence, without installing anything.
"Add & test" tells you whether that machine's WSL has both tmux and an agent CLI.

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
