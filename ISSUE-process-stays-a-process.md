## A process with a source lands in the agents, has no "where does it run", and is hard to delete

Three findings from testing #186/#187.

### 1. A deployed process becomes an agent

The sidebar splits the two lists like this:

```js
const agents    = roots.filter((p) => p.role || p.origin);
const processes = roots.filter((p) => !p.role && !p.origin);
```

"Has a role or a source" was a safe proxy for *agent* as long as only roles and
free specialists could have a source. Since #186 a **process** can come from
GitHub too, so it gets an `origin` — and moves straight into the agents list. It
also stops being draggable, because that rule uses the same proxy.

The card needs to say what it is instead of having it guessed: a `kind` field
(`"agent"` / `"process"`). Empty means a card from before the field, and then the
old derivation still applies — so nothing in an existing `projects.json` moves.

### 2. "Where does it run" is missing for a process

A role can be *standing* or *embedded in* a work process you already have; the
choice sets `parent` and the card nests under its host. A process has no such
choice, but it needs one for the same reason: a work process can be **part of a
bigger one** — four phases, and phase 3 starts something of its own. That folder
belongs under its host, not loose in the list.

The chosen host also moves the proposed folder: the proposal lands under the
host's path instead of the default place — and only while you have not typed a
path yourself.

### 3. Deleting is hard to hit

Reported as "it took three or four tries before the delete yes/no question came".
Two causes, both in the affordance:

- `.pc-actions` is hidden until the card is hovered, and the buttons are ~20x18
  px, sitting in the top-right corner next to each other. With an update arrow
  present (which a deployed process now has) there are three of them and the 🗑
  shifts left, so aiming where it used to be hits ✎ and opens the editor.
- The confirmation covers the card (`inset: 0`) with the question and two buttons
  in a centred row with no shrinking. On a narrow sidebar a longer question
  ("Verwijderen? — met 2 ingebed") pushes Ja/Nee out of the card, so the question
  is there but unusable.

Fix: bigger targets, a question that truncates instead of pushing the buttons out,
and **delete in the right-click card menu** — the same confirmation, but a target
you cannot miss.
