## 🐂 Taurus 0.7.1 — hand a work process over, keep your own hands free

A maintenance release with one new capability: **a work process can now come from a
GitHub address**, so handing one over is a link instead of a folder someone has to
copy. The rest is the kind of polish you only find by using the thing — the
dropzone, the sessions, and a handful of edges that were sharper than they looked.

### A work process from a GitHub address (#186, #188)

**Nieuw proces** grew a *Source* field. Leave it empty and nothing changed: you
point at a folder you already have. Fill in a GitHub address, press **Lezen**, and
Taurus reads the source before it writes anything — name, branch, commit, size.

Where it differs from a special agent: **the ICM check warns, it does not refuse.**
A role lands in a field where the method means something, so an unrecognisable
source is still turned away there. A process is yours to place, so the same finding
becomes a line under the field — *no `CLAUDE.md`, `SKILL.md`, `agent.md` or
`identity.md` in the root* — and you decide. The warning only fires when none of
the four is present; complaining that a workspace has no `SKILL.md` would be noise.

A process fetched this way records where it came from, which means it inherits the
version watching the roles already had: checked at startup, and when the source has
moved ahead you get the question — update yes/no, with **don't ask again** for that
version. A later commit asks again.

And because a work process is often part of a bigger one, a process now has
**"where does it run"**: on its own, or part of a process you already have. Four
phases where phase 3 starts something of its own is a work process inside a work
process, and the card nests accordingly.

### You name the agent (#184)

Deploying the architect used to produce an agent called *Jake*, because that is the
name its source brings. You install an **architect**; what it is called is yours.
The tiles now carry the trade and nothing else, and the screen asks for a name —
prefilled with what the source proposes, yours to type over. The name lands on the
button and at the top of the `CLAUDE.md` Taurus writes.

### The dropzone (#173, #175, #176)

- **Paste object no longer keeps the keyboard focus.** It stayed on the button, so
  the next Enter pressed it again and a second file appeared. The focus now follows
  the path into the prompt — for the paste button, the `+` picker, the drop zones
  and a click on a list entry.
- **Pasted files have distinguishable names**: `pasted-a7f3k9.txt` instead of
  `pasted (3).txt`, `pasted (4).txt`.
- **Paste object works on a remote session.** The clipboard is bytes in this
  workstation's memory, so a file is made first: a copied file goes straight from
  where it is, an image or text through a temp folder that is removed again — also
  when the transfer fails, because clipboard content has no business lingering in
  `%TEMP%`.
- **A folder travels as one archive.** Dropping a folder on a remote session used
  to fail. Rather than copy file by file over SFTP — a round trip each, which
  crawls for a folder full of small files — it is packed into a `.tar.gz`, sent as
  one stream, and unpacked on the host. `.tar.gz` because both a Linux host and a
  Windows host can open it with nothing installed. If unpacking fails there, the
  prompt gets the archive path and the reason, and **Move** keeps your original.
- **Move to a host removes the local original** once the copy is confirmed. If the
  copy succeeded but the delete did not, that is not a failed transfer: the path
  goes into the prompt and the message says the original is still here.

### A new remote session is a new session (#182)

Starting a new session on a remote machine attached to the agent already running in
that folder, so a second one in the same folder was impossible. The session name
was derived from (host, folder) and the start does attach-or-create.

That derivation also carried reattaching after Taurus closes, so it could not
simply be randomised. A new session now gets its own name — including the tab
title, so three sessions in one folder are three readable lines both in Taurus and
in `herdr ls` on the host — while a tab that resumes says which session was its
own. The processes screen borrows the real title from the history, so a row reads
*Review van de PR* and not the folder name.

### Quoting, consent and a copied selection (#177, #179)

- **Arguments sent to a remote shell are quoted.** They were quoted only when they
  contained a space, and then with double quotes, so a `;`, `|`, `$(...)` or `&` in
  a path or URL became a second command on the host. Reachable in practice: the
  destination is a text field.
- **"Don't ask again" now leaves an audit trail.** A remembered peer's session
  recorded nothing about what started, while every other route logs. On that path
  the trail is the only record there is, because there was no popup.
- **"Always allow, with full power" sticks.** Only the "always" half was stored, so
  the first session got full power and every later one silently fell back.
- **A copied selection is trimmed**, not padded: no trailing spaces, no borrowed
  indent.
- `save_sessions` and `ssh_consent_reply` no longer fail silently — losing your
  tabs after a restart, or leaving the other side waiting while you think you
  answered, are not fire-and-forget.
- **Deleting a card is easier to hit**: bigger targets, a confirmation that
  truncates its question instead of pushing Ja/Nee out of the card, and delete in
  the right-click menu. And the **＋** you press decides which screen opens — the
  one above Agents on an agent, the one above Processes on a process.

### Known and deliberate

The SSH host binds every interface rather than only the trusted adapter
([#181](https://github.com/astetic-dev/taurus/issues/181)). Nothing gets in
unattended — an unknown key still meets the pairing popup — but on a public network
the SSH banner reaches anyone who can find the port. Recorded rather than fixed,
with the reason at the socket.
