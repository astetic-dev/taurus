## A new session on a remote machine lands in the one that is already running

Starting a **new** session on a remote workspace does not start anything: it
attaches to the agent that is already running in that folder. Starting a second
new session in the same folder on the remote machine is therefore impossible.

Picking up a running session is what the processes screen is for, with its own
resume button. The new-session button should always give a new agent.

### Why it happens

`mux_session_name(host_id, path)` is deliberately deterministic — same (host,
folder) gives the same name — and the payload that starts the session does
*attach-or-create*: `tmux new-session -A -s <name>`, and the herdr script checks
for the session first and only creates it when it is missing. So the second start
in the same folder resolves to the same name and lands in the first agent.

That determinism is load-bearing for something else, though: after closing
Taurus, `restart_session` recomputes the same name and thereby reattaches to the
agent that is still running on the host. Simply randomising the name would fix
the reported bug and break that one, which is the same bug in the other
direction.

### The fix

Make the caller say what it wants instead of deriving it from (host, folder):

- **New session** → a fresh name: the derived name plus a short suffix, so it is
  still readable and greppable on the host but never collides with a running one.
- **Resume / restart of a tab** → the name that tab already had.

That means the name has to be remembered, so:

- `create_session` returns the mux session name it used; the tab stores it
  (`muxName`, which already exists for attached sessions).
- `PersistedSession` gains `mux_name`, so it survives a Taurus restart, and
  `restart_session` takes it as a parameter.
- An empty name (a `sessions.json` from before this field) falls back to the old
  derived name, so an agent still running from an earlier version is found again.
- A host without a multiplexer reports no name at all — there is nothing to find
  later, and a stored name would make the exit dialog try to stop a session that
  never existed there.

Stopping already prefers the stored name (`stop_remote_session` with
`s.muxName`), with `stop_remote_session_at` as the fallback for tabs that have
none.

### Note for the processes screen

With several sessions per folder, that screen shows one row per mux session —
correct — but the row title is the folder leaf, so three sessions in
`C:\Users\arjen` read as three identical rows. Distinguishing them (a short
suffix, or the agent status) is a separate refinement, not part of this fix.
