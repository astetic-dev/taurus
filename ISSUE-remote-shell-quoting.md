## Arguments sent to a remote shell are not quoted, and a remembered consent leaves no trail

Three findings from a code review (`input/review.md`, 19 Aug 2026) that held up
against the code. Filed together because they are all in the remote/consent path
and the fixes are small.

### 1. Command injection in `git_on` and the CLAUDE.md writer

`git_on` (`lib.rs`) quoted an argument only when it contained a space, and then
with double quotes:

```rust
if a.contains(' ') { format!("\"{}\"", a.replace('"', "\\\"")) } else { a.to_string() }
```

On a POSIX host a double-quoted argument still expands `$VAR`, `$(...)` and
backticks — and an argument *without* a space is not quoted at all, so `;`, `|`
and `&&` go straight into the remote shell. On a Windows host `&` does the same
through cmd.exe.

This is reachable, not theoretical: `dest` comes from a text field on the deploy
screen, and `normalize_source` accepts `https://github.com/a/b$(...)` — the host
is github.com, there are three path segments, and there is no space, so no
quoting happened. A card created by an agent supplies the same text.

The same function writes CLAUDE.md on the host:

```rust
format!("printf %s '{}' | base64 -d > '{}/CLAUDE.md'", enc, dest)
```

`dest` sits raw inside single quotes: one apostrophe in the path breaks out. The
Windows branch next to it *does* escape (`dest.replace('\'', "''")`), which is
what gave the oversight away.

`shell_quote_posix()` and `ps_quote()` are in the same file and used correctly
elsewhere.

### 2. A remembered peer leaves no audit trail

`ask_session` returns early when `auto_allow` is set — before the `audit()` call
at the end. So for a peer with "don't ask again" there is no record of what
started or with how much power, while every other route (`exec`, `sftp-start`,
`auth-known`) logs something. On that path the audit trail is the only record
there is, because there was no popup.

### 3. "Always allow, with full power" does not stick

`Peer` stores only `auto_allow: bool`. `Decision::AlwaysFull` satisfies
`remembers()`, so it is saved as a plain `auto_allow` and the "full" half is
lost: `ask_session` then returns `Decision::Allow`, whose power is `Sandboxed`.
The first session gets full power and every later one is silently restricted. It
fails toward the safe side, but the checkbox does not do what it says.

### Also

- `save_sessions` and `ssh_consent_reply` fail silently. The first means your
  tabs are gone after a restart with no hint why; the second means the other side
  keeps waiting while you believe you answered.
- `closeSession` does not remove the `mirrorTabs` entry of a closed mirror tab.
  Harmless (the output handler tolerates a stale entry) but the Map only grows.

### Deliberately not changed

- **The non-PTY exec route** (`sshhost.rs`, `exec_request`) runs `cmd.exe /C` without
  a per-command popup. That is a documented choice: pairing is the boundary there,
  and #121 rejected command filtering on the grounds that a boundary which does
  not hold reads as security. Worth revisiting as a trust-model decision, not as a
  bug fix.
- **`snake_case` on `Host` and `RoleInstall`.** That is the on-disk format of
  `hosts.json` and `roles.json`, documented field by field in `REMOTE-HOSTS.md`.
  Adding `camelCase` would break existing installs and the setup guide. The rule
  is: persisted structs snake, view structs camel.
