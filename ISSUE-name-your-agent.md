## The name of an agent should be asked, not assumed

Agents have names now, but you never get to choose one. Deploy the architect and it
is called **Jake** — the name comes from the source repo's `taurus.json`, and the
tile in the new-agent screen shows it as a subtitle, which reads as a decision
rather than a proposal.

You install an *architect*. What that architect is called is yours to decide.

### What changes

- **The tiles at the top show only the role.** The name subtitle is gone: it named
  something you had not chosen yet. "Free" and "process" keep their subtitle,
  because that is an explanation, not a name.
- **The role header shows the role and the question you come in with**, not a name.
- **A name field in the screen.** Empty means "let the source decide", which is
  today's behaviour. Reading the source proposes the name it brings, and you can
  type over it — a proposal, like the destination path already is.
- **The chosen name is used where it matters:** on the card (the button in the
  sidebar) and at the top of the `CLAUDE.md` that Taurus writes. So a deploy named
  Sofie does not produce a workspace whose CLAUDE.md opens with "# Jake". If the
  repo brings its own CLAUDE.md, that file is still left alone.

`git_deploy` gains a `name` parameter for this; empty or absent keeps the old
behaviour of `taurus.json` first, then the repo name. That matters most on a
remote deploy, where the clone lives on the host and cannot be read back — there
the fallback was the slugged repo name (`Icm Architect`).

### Out of scope

The role list in **settings** still shows the name the default source brings, next
to its URL. That is a description of the source, not of an agent you made, so it
stays.
