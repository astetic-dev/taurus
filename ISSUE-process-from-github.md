## A work process should also be able to come from GitHub

A process (`Nieuw proces`) can only point at a folder that already exists here.
That makes a work process hard to hand over: the folder travels by hand, and
nothing knows where it came from or whether it is still current.

A process should accept a **GitHub address** next to a local path, with the same
reading of the source that a special agent gets — but **without the lock**.

### What it does

- **Source field in the process screen**, optional. Empty is exactly what it does
  today: you point at an existing folder. Fill in a GitHub address and *Lezen*
  reads the source and shows what it found: name, branch, sha, size.
- **The ICM check warns, it does not refuse.** For a role, a source that cannot be
  recognised as an ICM work process is still refused: it lands in a field where
  the method means something. For a process the same finding is a warning —
  "no CLAUDE.md, SKILL.md, agent.md or identity.md in the root" — and you decide.
  `git_probe` and `git_deploy` take a `strict` flag for this; absent means strict,
  so a role deploy keeps its rule and no other caller loses it by accident.
- **The version is recorded and watched.** A process deployed from a source gets
  the same `origin { source, branch, sha }` a role gets, which means it inherits
  the machinery that already exists: `check_sources` at startup, and the update
  question ("update? yes/no") with *don't ask again* per version — a later commit
  asks again, the skipped one does not.
- The chosen name (#184) is passed along, so a generated CLAUDE.md carries it.

### Notes

- The marker set for the *warning* is wider than `icm_shape`: CLAUDE.md, SKILL.md,
  agent.md, identity.md. `icm_shape` decides whether a **role** deploy may proceed
  and was measured over the thirteen repos that concerns; loosening it there would
  weaken a rule that earns its strictness. The warning is a different question, so
  it gets its own list, and it only fires when *none* of the four is present —
  complaining that a workspace has no SKILL.md would be noise.
- A process can also be deployed to a **host**: `git_deploy` already takes a host,
  so the machine dropdown in the process screen works for a source too.
