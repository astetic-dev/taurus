## 🐂 Special agents — seven specialists you install into a folder

I decided to add special agents to Taurus. From previous competitions I gathered the
winners and the honourable mentions, studied what made them win, and forged all of it
into brand new ICM engineers: **the diagnostician, the editor, the coach, the researcher,
the operator** and **the cartographer** are all there. And as Jake said we should use his
architect, his repository got its place too.

▶ **The film:** https://youtu.be/18_NHe7I2oA

| role | who fills the seat | what it does |
|---|---|---|
| Architect | Jake — [`RinDig/icm-architect`](https://github.com/RinDig/icm-architect) | turns a way of working into a folder structure an agent can run |
| Operator | [Heimdall](https://github.com/astetic-dev/heimdall) | something arrives, it decides whose it is and records the rule that made the decision |
| Cartographer | [Cassini](https://github.com/astetic-dev/cassini-cartographer) | maps a body of work someone will change — one card per thing that matters |
| Diagnostician | [Mímir](https://github.com/astetic-dev/mimir) | names the one structural reason an agent folder stopped doing what you asked |
| Editor | [Forseti](https://github.com/astetic-dev/forseti) | holds work to its own stated word. Ranked findings, never a rewrite |
| Researcher | [Kvasir](https://github.com/astetic-dev/kvasir) | reports the state of the knowledge, and grades the evidence |
| Coach | [Vör](https://github.com/astetic-dev/vor) | the only one whose subject is a person |

### Launch a specialist from Taurus

![The new agent screen](https://raw.githubusercontent.com/astetic-dev/taurus/v0.7.0/media/release-0.7.0/new-agent.png)

Press **＋**, pick the **trade** rather than the name — you install a Cartographer, and
Cassini is who currently fills that seat.

**Taurus reads the source before it writes anything.** It clones into a throwaway folder,
then reports what it found: the slot, the shape, the commit and date, the size, and
whether the repo brings **its own `CLAUDE.md` — which is left untouched**. Brings it none,
and Taurus writes one that names only files that are really there. Point it at something
that is not an ICM workspace and it says so, lists what it did find instead, and leaves
nothing behind.

**Goes in** is a proposal: edit it, or pick another parent and the rest is recomputed.

### Or insert one into a work process, where it only sees what is there

![Deploying a role inside a work process](https://raw.githubusercontent.com/astetic-dev/taurus/v0.7.0/media/release-0.7.0/new-agent-embedded.png)

A **standing** role gets its own workspace and looks at your ICM as a whole. An
**embedded** role lands inside the process it is about — `_diagnosis\over` inside that
process — and uses only the context that is there. Its card sits indented under that
process in the sidebar.

The folder is named after the **trade**, never after the agent, so replacing whoever fills
that seat later does not strand your history.

### Start a new workspace with Jake's help. Or map a repository with Cassini.

![A role started with a task](https://raw.githubusercontent.com/astetic-dev/taurus/v0.7.0/media/release-0.7.0/start-with-a-task.png)

Right-click any card and choose **have Cassini look at this** — the path lands in the
cartographer's task and you press start yourself, so you still see what is about to
happen. Code or ICM, both are territory it surveys.

![Cassini's map of Jake's icm-architect, open in Taurus](https://raw.githubusercontent.com/astetic-dev/taurus/v0.7.0/media/hero-special-agents.png)

Every task you give a role is written into that role's own workspace as `_assignments.md`,
next to the answer it belongs to — untracked, so it never counts as a change to the repo it
came from. **≡** in the agents header shows all of them, per role.

### Don't like mine? Use your own. And keep it up to date.

**Settings → Agents** holds the seven sources. Swap any of them for **your** repository or
**your** local folder and the slot keeps working; the workspace keeps its name, so the
history of that role stays in one place. A local folder is copied instead of cloned, and
says plainly that there is no version watching.

When the source moves on, the card grows a green **↑**. Updating is `git pull --ff-only`
in that one folder and nothing else — and if you changed something in there yourself, it
refuses and leaves the folder exactly as it is. No `reset`, no `checkout --force`, no
`clean`, ever.

### Also in this release

- **Closing asks what you can lose** — per session, what keeps running on its machine and
  what stops but resumes as a conversation. With no running agents it asks nothing.
- **A sidebar that holds eleven processes**: two sections, each with its own list, scrollbar
  and fold, plus a filter on name or path.
- **Roles are named after the role**, deleting a process takes its embedded cards with it
  (nothing leaves disk), and the ✎ on a card edits *that* card.
- **The window says which build it is**, and the startup self-check reports a broken screen
  instead of failing silently.

### Notes

- Windows-only, as before.
- **Not exercised yet:** deploying a role to a remote host, and *also install as a skill*.
  Both are built and marked as such in the code.
- The seven repositories are public, MIT where noted, and readable without Taurus — the
  launcher is the entry point, not the owner.

Issues in this release: #42, #155, #156, #157, #158, #159, #160, #161, #163, #164, #166, #168.
