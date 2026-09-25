# Taurus — project instructions

## Contribution workflow — changes go through GitHub issues

**Every change to this project goes through a GitHub issue first.** Before writing code,
file (or reference) a GitHub issue that describes the problem and the intended fix; do the
work on a branch and open a PR that references the issue.

- **Language: English.** All issues, PRs, branch names and commit messages are in English
  (the repo is public on GitHub). This overrides the global Dutch default for this project.
- **Account: `astetic-dev`** (arjenstet@gmail.com). The `gh` CLI is authenticated as this
  account; the remote is `https://github.com/astetic-dev/taurus.git`.
- Create issues with `gh issue create --repo astetic-dev/taurus ...`. When a ready-to-file
  issue body exists as a Markdown file in the repo (e.g. `ISSUE-*.md`), file it with
  `--body-file`.

## Stack
- Tauri v2; frontend = vanilla JS under `src/` (`frontendDist: ../src`); Rust under `src-tauri/`.
- Platforms: Windows is the primary target; the macOS port is tracked in #199. Keep one
  codebase: platform differences go behind `cfg` gates or a small platform module, and
  every change keeps the Windows build working.

## Working agreements

These come from working on this project with its owner (Arjen). They hold on every machine.

- **Finish a theme into one working build.** Issues and PRs are the agent's bookkeeping,
  not the owner's homework: merge to `main`, push, close the PR and the issues that are
  done. Don't deliver a stack of branches with "these are still open, please merge".
  Mention once at the end what was deliberately not built; don't end on an open question.
- **Version numbers only on request.** Feedback goes into builds without touching the
  version in `src-tauri/Cargo.toml` / `src-tauri/tauri.conf.json`. When a release is asked
  for, bump conservatively (patch by default; minor only for a substantial feature set)
  and confirm the exact number before releasing.
- **Shared working tree: check the branch.** More than one agent session can work in the
  same checkout. Run `git branch --show-current` before the first change and again before
  `git add`. If it isn't yours, save your patch (`git diff > file`), restore the tree as you
  found it, and continue in your own worktree
  (`git worktree add ../Taurus-<topic> -b <branch> origin/main`). Remove it afterwards.
- **Never start, close or kill the owner's Taurus window.** The agent session is often
  running *inside* it, and a test instance disappearing mid-test looks like a broken build.
  On Windows a running exe is locked, so build into a fresh `target` directory and ask the
  owner to restart their own window.
- **Commands for the owner go in a `.md` file**, one command per fenced block, with the full
  path mentioned in the reply. Copying commands out of the terminal chat adds spaces and
  CRLFs. Test plans that need a second machine go into `TEST-<issue>-<topic>.md` with
  checkboxes; measure the local half yourself and put it in unit tests.
- **A measured limitation is a fact, not a design requirement.** Taurus works where it can;
  a stricter environment is that user's problem. Report the limitation in one honest line
  and offer the normal path as the normal path. Do build what is correct anyway and costs
  nothing.
- **Screenshots: only the Taurus window**, never the full screen. The owner's desktop can
  show password managers and customer data. Delete verification screenshots afterwards.
- **Measured pitfalls** live in `PITFALLS.md`. Read the section for the area you touch
  before changing it. Add to it when you measure a new one.
