## Problem

The launch form offers three permission modes for `claude` (`src/main.js:741`):

```js
claude: [ {value:"default"}, {value:"plan"}, {value:"auto"} ]
```

Claude Code 2.1.232 accepts six:

```
--permission-mode <mode>   (choices: "acceptEdits", "auto",
                            "bypassPermissions", "manual", "dontAsk", "plan")
```

So four are unreachable from Taurus, and the vocabulary is stale.

**Nothing is broken today.** `build_command()` omits the flag entirely when the mode is
`default` (`src-tauri/src/lib.rs:1994`), so no invalid value is ever sent. And `default`
is in fact still accepted by the CLI as an **unlisted alias** — verified: `default` is
taken, while `Default`, `DEFAULT` and a nonsense value are rejected with the list of six.
It works, but it is no longer advertised, so it is on borrowed time.

This is about expressiveness, not breakage.

## What the modes actually do

Taken verbatim from the strings in `claude.exe` 2.1.232, not from guessing:

| mode | what it does |
|---|---|
| `manual` | standard behaviour, prompts for dangerous operations (the old `default`) |
| `acceptEdits` | auto-accept file edit operations |
| `plan` | planning mode, **no actual tool execution** |
| `auto` | use a model classifier to approve/deny permission prompts |
| `dontAsk` | **don't prompt for permissions, deny if not pre-approved** |
| `bypassPermissions` | bypass all permission checks (requires `allowDangerouslySkipPermissions`) |

Two of those are worth having beyond completeness:

- **`acceptEdits`** is the everyday shape of working with an agent: let it write code, still
  ask before it runs things. There is no way to express that today.
- **`dontAsk`** matters *specifically* for Taurus, because Taurus runs agents nobody is
  watching — remote sessions, and the unattended case in #126. A permission prompt that
  nobody will answer is a hung session. `dontAsk` never blocks and denies what was not
  pre-approved, which is exactly the right failure direction for unattended work.

`bypassPermissions` should be offered but labelled honestly: it needs the disclaimer
accepted once via `--dangerously-skip-permissions`, and settings or policy can disable it —
in which case it silently does not apply.

## Proposal

Keep `default` as the stored value meaning **"use the agent's own setting"** (no flag).
That is what it already does, it needs no data migration, and it is a genuinely useful
choice — a user whose `settings.json` sets `defaultMode: acceptEdits` should not have
Taurus quietly override it. Relabel it so it says that.

Then add the five explicit modes next to it:

| stored value | label |
|---|---|
| `default` | Zoals in je eigen instellingen |
| `manual` | Vraagt het per stap |
| `acceptEdits` | Bewerkt bestanden zelf, vraagt voor de rest |
| `plan` | Alleen plannen, voert niets uit |
| `auto` | Model beoordeelt elk verzoek |
| `dontAsk` | Vraagt nooit; wat niet vooraf is toegestaan gaat niet door |
| `bypassPermissions` | Geen enkele controle |

The backend passes the value through verbatim already, so it needs no mapping — but it
does need a **whitelist**, so a stale card (for example an agy card with `sandbox` switched
over to claude) can never produce an invalid flag. Unknown falls back to inherit rather
than to an error three layers down in a remote shell.

## Related, and deliberately a separate decision

#126 starts an unattended inbound session with `--permission-mode plan`. Plan mode does
**no tool execution at all**, which makes an unattended session unable to do the work it
was allowed to do. `dontAsk` may be the better fit: it cannot hang on a prompt, and it
denies anything not pre-approved. Not changed here — that is a security-shaped decision
and belongs in its own change.

## Acceptance criteria

- [ ] All six CLI modes are selectable for `claude`, plus "use my own setting".
- [ ] Labels state what the mode does, not what it is called.
- [ ] `bypassPermissions` says it needs the one-time disclaimer and can be disabled by policy.
- [ ] An unknown or agent-mismatched mode never reaches the CLI as a flag.
- [ ] Existing cards and sessions storing `default` keep working unchanged.
- [ ] agy's own modes are untouched.
