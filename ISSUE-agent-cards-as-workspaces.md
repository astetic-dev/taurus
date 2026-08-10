## Problem

The tab bar is the bottleneck for heavy multi-agent use. Two real-world patterns
both overflow it:

1. **Parallel work on one process** — several agents on the same folder (one
   fixes, one tests, one reviews). Tabs get near-identical names.
2. **Many processes at once** — one or two sessions each for many agents. With
   10+ tabs the bar becomes unreadable and scrolls horizontally.

And once you have that many, a second problem appears on top: **you cannot tell
which agent is doing what.** A tab shows a title and, with `tabStatus` on, a
single activity verb (`✶ Orbiting…`). That is not enough to decide which tab
needs you.

## Behaviour

### Grouping, above a threshold

- Below the threshold (**default 10 tabs**, a setting) nothing changes: every
  session is its own tab.
- Above it, sessions **started from the same source** collapse into one group
  tab. Sources with a single session stay ordinary tabs.
- The group tab shows the shared name, the member count, and an **aggregated
  state**: waiting if any member waits, working if any works, exited if all did.
- **Hover** (or click) the group tab and it expands into a panel below it,
  listing its members. Clicking a member switches to it; each row keeps its own
  close button.

### Recap on hover

- Hovering a tab — grouped member or ordinary tab — shows a **recap**: the last
  thing that agent said, plus its state.
- This works for background tabs, not only the visible one.

## Why this replaces the earlier direction

This issue previously proposed solving the same crowding by making sidebar cards
act as workspaces (click-to-filter the tab bar, per-card ＋, per-card status
badges). That is dropped in favour of the above: the crowding is a tab-bar
problem, and grouping fixes it where it happens, automatically, instead of asking
the user to filter deliberately.

Two consequences worth stating:

- **#42 (sidebar filter) and #43 (collapsible sidebar groups) are no longer
  superseded.** They stand on their own merits again.
- The per-card **status badge** idea from the old plan had independent value
  ("the sidebar doubles as a status dashboard"). It is out of scope here; the
  aggregated state on the group tab covers the same risk of missing a background
  agent's "ready". Worth reviving separately if the sidebar ever needs it.

## What makes this cheap

Three things already exist:

- **The recap can come from the terminal itself.** `s.term.write(u8)` runs on
  every pty event without checking which tab is visible (`src/main.js:1959`), so
  every session keeps its xterm buffer current even while hidden. Claude Code
  runs in the *alternate* buffer — the wheel handler already relies on that
  (`src/main.js:1434`) — so `s.term.buffer.active` **is** the visible TUI screen.
  There is already a precedent for reading it: the `.html` link provider walks
  `getLine` / `isWrapped` / `translateToString` (`src/main.js:1518-1536`).
- **Grouping needs no new state.** Sessions carry `path` and `hostId`; together
  those are "the same source". (See the caveat below.)
- **A floating panel has a precedent.** `.ctx-menu` is positioned absolutely at
  body level by `openTabMenu`, which sidesteps clipping and z-index entirely.

Two alternatives for the recap were rejected: parsing the transcript (the format
is internal to Claude Code and can change on any release) and sending `/recap`
to the session (costs tokens and writes into the conversation).

## Extracting the recap

The bottom of a Claude Code screen is its own chrome — separator, input box,
separator, hint line. So "the last non-empty lines" would show the input box
rather than what the agent said.

Claude Code prefixes each assistant message with `●`. The recap therefore
anchors on the **last `●` line** and takes from there down, stopping at the
chrome. If no `●` is found it falls back to the last non-chrome lines.

This is screen-scraping, and deliberately in the same class as the busy detection
already in use (`lastSpinnerVerb`, the `esc to interrupt` probe). If Claude Code
changes its layout the recap degrades to showing a little chrome — it does not
break anything.

## Risks to design around

- **The attention signal must not disappear.** Today a tab flashes in its own
  colour when *that* agent finishes. Collapse five tabs into one and four of
  those signals vanish unless the group tab aggregates them. Non-negotiable.
- **`Ctrl+1..9` stops matching what you see.** `selectNthTab` indexes
  positionally over all sessions (`src/main.js:2413`). Keeping the shortcut on
  the flat session list is defensible, but it is a deliberate choice, not an
  oversight.
- **Hover alone is fragile.** The pointer has to travel from the tab into a panel
  below it. Needs an open delay, a close grace period, and it must also open on
  click — nothing may be reachable *only* by hover.
- **No dragging inside the panel.** `makeReorderable` starts on mousedown with a
  5 px threshold (`src/main.js:692`); dragging a row out of a hover panel has no
  defined meaning.
- **`path` + `hostId` as the group key merges two cards that point at the same
  folder** with different modes or models. Adding `projectId` to the session and
  to `PersistedSession` is the cleaner key and would survive restarts; doing it
  now avoids a migration later.
