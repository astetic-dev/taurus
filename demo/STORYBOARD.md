# Taurus intro video — storyboard (clean demo)

Goal: show Taurus with **Porter** and **ASTRID** as separate tabs. Zero sensitive
data — the demo agents are **fake** (mock-claude); they never touch real data or MCP.

## Before
1. Run `demo\demo-on.ps1` (activates demo projects, backs up your real config).
2. In Taurus click **Reload** (bottom-left) → you now see only Porter + ASTRID.

## Record (~60-90s)
1. **Launcher** — click **Porter**: point out the path + **LOCAL** badge, title, **Mode**.
2. **Start agent** — a fake Claude session opens (fresh welcome screen, no real data).
3. **Second tab** — click **ASTRID**, start it (opens in **Plan** mode).
4. **Switch tabs** — show live switching between Porter and ASTRID.
5. **Flash** — a working tab shows a steady green dot; when it finishes and waits for
   you, the tab **flashes in its colour** (only if you're on another tab). Click it → stops.
6. **HTML preview** — click the `.html` path in the terminal (or right-click tab →
   HTML preview). The demo dashboard appears on the right. Toggle split/full.
7. **Wrap-up** — right-click a tab: Restart / Open folder; open Settings (EN/NL, font).

## After
- Run `demo\demo-off.ps1` to restore your real projects.

## Tips
- Record the ~880x660 window with Xbox Game Bar (Win+G) or OBS.
- Type any prompt in a fake session — it replies with fictional data only.
