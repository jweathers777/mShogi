# Agent selection

[AGENTS.md](../AGENTS.md#agent-level) defines capability tiers and reasoning
effort. The mappings here are selection recommendations, not measured claims
that models from different providers have equal capability. Task requirements,
acceptance criteria and checks stay the same when the tool changes.

## Model selection

| Task capability | Claude Code | Codex | Antigravity with John's current access |
| --- | --- | --- | --- |
| `model:small` | `haiku` | `gpt-6.1-terra` | Gemini 4 Hydrogen |
| `model:medium` | `sonnet` | `gpt-6.2-sol` | Gemini 4 Hydrogen |
| `model:large` | `opus` | `gpt-6.2-astra` | Gemini 4 Hydrogen is the only available model; record the capability fallback |

John currently has access only to Gemini 4 Hydrogen in Antigravity, with Low,
Medium, High and Extra-High reasoning. Using it for a large task does not prove
that it meets that capability tier. Start with bounded work and assess the
result against the task's evidence. Extra-High is a reasoning setting, not a
different or stronger model.

## Effort selection

| Task effort | Claude Code `effort` | Codex `model_reasoning_effort` | Antigravity Hydrogen |
| --- | --- | --- | --- |
| `effort:low` | `low` | `low` | Low |
| `effort:medium` | `medium` | `medium` | Medium |
| `effort:high` | `high` | `high` | High |
| `effort:xhigh` | `xhigh` | `xhigh` | Extra-High |
| `effort:max` | `max` | `max` | Extra-High; highest available, not an exact equivalent |

Use these settings where the selected model and interface support them. If a
model or setting is unavailable, choose the nearest supported option, tell
John and record the actual model, effort and limitation in the task notes.
Do not change task labels merely because a subscription or interface limits
the choices. If a worker cannot complete the task at its available capability,
follow the escalation and handoff rule in AGENTS.md.

## Switching tools

All workers follow AGENTS.md and use the same Backlog, Git history and
[check command](development.md#checks-and-git-hooks). `CLAUDE.md` imports
AGENTS.md; Antigravity can read AGENTS.md directly. Keep one process definition,
not separate copies for each tool.

Before handing work over, record completed work, remaining work, check results
and the branch in the task notes, then push. The next worker reads that record
and the current checkout; tool-specific chat history or private plans must not
be required to continue. Check that the new environment can run the repository
CLIs and access the services needed by the task.
