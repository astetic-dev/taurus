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
