# MSH-62 planning validation

Status: **planning validation and independent reviews passed**, 2026-10-26.
This is documentation/task planning evidence, not shogi conformance, game
implementation, reference runtime admission, engine strength or a new M3 audit.

Baseline: closed-M3 integration `6f29713795804eed3605114d90fed37aa5f554a0`;
builder starts from M3 completion `2b9849c` on `codex/m4-planning` in the reused
`next-task` worktree. Active planning owner is `@codex`, MSH-62. Backlog CLI
1.55.1 matches the managed instructions. The default/master branch and unrelated
integration prototype cleanup are preserved.

## Checks and evidence boundaries

The following checks passed:

* `backlog doctor` using the recorded CLI, without repair flags: identity and
  dependency diagnostics, never an automatic repair or deletion of task history.
* Structured `backlog task view <id> --json` validation against the pre-change
  task snapshot: original statuses, milestones, titles and all AC/DoD preserved;
  only intended MSH-41–46 dependency/documentation/append-only refinement changes.
* Full dependency DFS: all references resolve, no cycles, early shared work has
  no aggregate final-delivery prerequisite, game deliveries depend on their
  required shared work and MSH-46/47 retain complete final gates.
* Future tickets have scope, observable criteria, dependencies, verification
  expectations and no detailed implementation plan. No future delivery is Done
  or active, and all newly created work appears in the ownership/traceability map.
* Local documentation links, fixture inventory identities, whitespace and a
  source-diff audit establishing no product/runtime/packaging changes.
* Fresh independent planning review of source grounding, primary-rule editions,
  reference-operation audit, owner-choice clarity, fixture limitations and graph.

Closed M3 native/application evidence is reused. No Rust/frontend tests, packaged
GUI journeys, shogi reference build or opponent match are required to establish
this planning-only change. Source-audited candidates and proposed fixture
expectations are explicitly distinct from future executed conformance evidence.

## Actual results

Backlog doctor reported: “No duplicate IDs, self-referential dependencies, or
dependency cycles found.” The structured validation checked **89 tasks and
238 dependency edges**, all 61 original records' protected status/title/milestone
and AC/DoD fields, and exact original dependencies outside the intended
MSH-41–46 refinements. All references resolve; full DFS is acyclic. Required
game/shared/final gates are present. Early MSH-69/72/73 and later MSH-76–79 have
no aggregate final-game/M4 delivery prerequisite. MSH-64–66 do not depend on
profile choice; MSH-67 does. Later source research stays independent.

The normalized original AC/DoD snapshot SHA-256 is
`88609add8479db0d212dc2819c5bb16552dadffbb61e1425602fdb849eb26d3f`.
The pre-change input was captured from the baseline with the supported CLI;
original task text is independently recoverable using `git show 6f297137:<path>`.
The temporary validation inputs/scripts are not product dependencies. To
reproduce the semantic checks, read all task records with
`backlog task view <id> --json`, compare those protected fields against baseline
and DFS each record's complete `dependencies` list; inspect the specific gate
sets documented in [the delivery plan](delivery-plan.md). The independent
reviewer performed its own structured comparison and DFS.

All 27 future tasks are unassigned and Ready/To Do, with unchecked criteria,
verification expectations and no detailed implementation plans. MSH-41–47 are
still To Do. All 55 fixture IDs are unique/contiguous. Local documentation links
and `git diff --check` pass. Scoped source comparison against closed M3 is empty
for `crates`, frontend/host sources, desktop scripts and workflows. The existing
root-manifest prototype-exclusion difference is explicitly documented and is not
a change introduced by this batch.

Fresh independent [architecture/ownership/DAG review](planning-review.md) passed
with no material findings. Fresh independent [rules/reference review](rules-review.md)
also passed after checking critical primary clauses, exact source pins,
reference-operation caveats, concrete hand-derived examples and later-source
boundaries. Its small SH-F041 wording alignment was applied: the proposed
declared `n - 1` length counter is explicit, while concrete fixture construction
and pre-root repetition knowledge remain distinct. No new owner choice arose.

## CI and integration scope

This batch contains only documentation and Backlog records. To honor the request
to reuse M3 evidence rather than repeat its application audits, the planning-only
commit/integration uses the [documented CI skip indicator](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs).
This is not reported as a new passing Rust, frontend or packaged-GUI run. Normal
Git integration/push and all server protections still apply. Relevant unchanged
M3 source/native/platform evidence remains linked from its closure records.

MSH-62 records the actual final integration and completion references. MSH-41–47
remain open; SHOGI-P1/MSH-63 is still an actual owner choice. No implementation
queue, reference build or opponent match was executed in this batch.
