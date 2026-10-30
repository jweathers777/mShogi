> Corrected pre-release policy — MSH-115, 2026-10-30: see the
> [target architecture reassessment](review/architecture-target-115.md) and
> accepted design A-09/D-17. Development-era internal APIs/save formats need
> no compatibility preservation. MSH-106 remains Done and its source/evidence
> remain historical; its compatibility-based recommendations are superseded
> for future work. No refactor, implementation task or new gate is authorized.

> Current checkpoint — MSH-106, 2026-10-30: the [three-game architecture report](review/architecture-health-106.md)
> reviews exact accepted integration `2f17510e4c5b8c3b92a051adfa070b4d5d834b21`.
> It records diagnostic findings and owner-prioritized follow-up proposals,
> without authorizing refactors or changing variant/M5 gates. MSH-107–109
> retain their existing future scope; the historical planning record follows.

# Architectural health review checkpoints

Owner-requested planning change, 2026-10-27. These are future diagnostic tasks,
not refactor tasks or completed architectural assessments. Preserve all six
games, accepted product/rules contracts, existing delivery gates and completed
evidence in [the design baseline](design.md).

## Roadmap and dependency boundaries

An arrow means prerequisite → dependent. All existing prerequisites are retained.

| Review | Trigger and consumers | Required downstream boundary |
| --- | --- | --- |
| MSH-106: after complete Xiangqi | MSH-45 → MSH-106; compare delivered Chess, Standard Shogi and Xiangqi through actual rules, console, engine, records and desktop. MSH-95 routing or MSH-103 review alone is insufficient. | MSH-106 → MSH-108. No new prerequisite for earlier Tori, Chu or Tenjiku implementation. |
| MSH-107: after complete Chu Shogi | MSH-43 → MSH-107; compare actual compound actions, royal semantics and large-board consumers with every then-delivered game. | MSH-107 → MSH-108. No new prerequisite for earlier Tenjiku or other game implementation. |
| MSH-108: all six games before M4 closure | MSH-46, MSH-106 and MSH-107 → MSH-108. MSH-46 already requires the complete game deliveries and final shared acceptance. | MSH-108 → MSH-47, preserving every original M4 closure prerequisite. |
| MSH-109: before substantial M5 optimization | MSH-108 and MSH-48 → MSH-109; reassess the current architecture against actual experiment/pilot evidence and intended optimization consumers. | MSH-109 → each of MSH-49–55 and MSH-104. MSH-56 retains its existing prerequisites and inherits this checkpoint transitively. |

The existing MSH-89 shared-contract acceptance and MSH-47 delivery review do not
explicitly require this complete architecture diagnostic. Existing M5 pilot,
strength and model tasks likewise lack an equivalent architectural checkpoint.
MSH-108 uses those integration reports as evidence, and MSH-109 reassesses the
report against current experiment boundaries; neither repeats unchanged work
without a reason or replaces product acceptance.

MSH-48 retains its original MSH-25 prerequisite. Shared infrastructure, baseline
measurements and budgeted pilots can establish evidence before MSH-109. The
strength/evaluator/opening task gates intentionally require the architecture
checkpoint before substantial optimization or training. They do not add an
ordering dependency between game implementations, nor require MSH-47 closure
before the checkpoint. Future refinement may isolate earlier measurement-only
work, preserving the explicit gate on substantial optimization. No model,
numeric strength target or mandatory restructuring is selected here.

## Review contract

At each checkpoint publish a current architecture map tied to an exact source
revision, real consumers and source symbols. Review every responsibility below;
state explicitly what is delivered, unchanged or unavailable at that boundary:

- Native position/state ownership and complete-action representation.
- Candidate generation, attack/movement queries, legality and apply/undo.
- History/adjudication, canonical records/codecs and perft/diagnostics.
- Evaluator/search boundaries and shared game/profile dispatch.
- Transport/session/archive abstractions and capability discovery.
- Desktop presentation boundaries and frontend/native rules separation.
- Dependency structure, duplicated logic, game-specific branching in shared
  code and verification consistency across games.

Assess both duplicated implementations where multiple real consumers now
demonstrate stable shared semantics, and premature abstractions forcing games
with different semantics through misleading common models. Apply this rule:

> Generalize when multiple real consumers demonstrate the same semantics, not merely similar-looking code.

Judge explicit, coherent, independently testable responsibilities. Do not require
identical layouts: a game does not need `generate.rs`, `attack.rs` or any other
filename because Chess has one. Very large modules, unclear responsibility
boundaries, repeated game-identity conditionals, awkward dependency direction,
duplicate serialization and hard-to-isolate tests warrant investigation; they
are not automatic defects. Similar code can encode different semantics, and
separate files can still duplicate an established semantic responsibility.

Each report must include a current architecture map; concrete source/example
findings; risk and severity; areas explicitly reviewed and found acceptable;
recommended changes, if any; expected benefit and affected consumers; regression
risks; and suggested bounded follow-up tickets. Record actual commands, outcomes,
environment, source revision, evidence limits and proportionate independent
assessment. Distinguish rule correctness, search correctness, execution
performance, playing strength and application integration.

Completing a review does not imply a refactor must occur. **“No material
architectural changes recommended” is valid.** Record the disposition of actual
risks and any genuine correctness/contract blocker separately from advisory
recommendations. Do not make every improvement recommendation a closure blocker.

Do no substantial restructuring inside a review task. Only a very small
correction explicitly required by an existing acceptance criterion can be in
scope. Any material refactor becomes a separate bounded task, preserves behavior
with focused differential/regression evidence, and names its affected consumers
and regression risks. Product/rules/contract changes follow the existing owner
decision process. Creating review records approves no refactor implementation.

## Isolated planning handoff

Planner: Codex in `codex/architectural-health-roadmap`, temporary worktree
`/private/tmp/mshogi-architecture-roadmap`, based on
`6412f483bd3ccdd1e27499cf05cdc47ff2723467`. The remote and local `codex/rewrite`
tips matched that revision at isolation. The primary checkout had just integrated
the MSH-95 source commit; MSH-95 remained In Review, owned by `@codex`, with
final integration evidence still belonging to the other session.

This branch changes no production source, active implementation task, game
delivery task or completed evidence. Only MSH-47, MSH-49–55 and MSH-104 receive
additional prerequisites; their criteria, ownership, status and evidence remain
intact. Four unassigned Ready review tasks have actionable scope but unsatisfied
dependencies. No review has been performed, and no milestone has been closed.

The temporary worktree is retained solely for the owner/designated integrator
to inspect and integrate this planning commit after the implementation session
finishes. No merge, rebase, fast-forward of `codex/rewrite` or push is authorized
for this session. Recheck current task IDs, ownership, graph and file overlap
before integration; reconcile any later overlapping planning edits without
overwriting active work. Remove this disposable worktree after integration and
preservation of anything needed.

## Planning validation

Final checks on the planning diff based at `6412f48`, macOS arm64,
Python 3.14.7, pinned Backlog.md 1.55.1:

- `backlog task list --json` and `backlog task view <id> --json` captured all
  105 baseline records and all 109 resulting records. A read-only Python
  comparison passed: every prior criterion/check, DoD, status, assignee,
  description, plan, notes, evidence and summary is preserved. Only the nine
  named dependency sets, update timestamps and derived graph/readiness change.
- A full DFS passed with 109 unique task IDs, no missing dependencies and no
  cycles. Reachability checks confirmed all four boundaries and no new review
  prerequisite on any earlier game implementation/handoff/routing task or
  MSH-48. Chess completeness26 stays Done and remains an original47 prerequisite.
- `backlog doctor` passed: no duplicate IDs, self-referential dependencies or
  dependency cycles. All four new tasks are unassigned Ready with blocked
  prerequisites, eight unchecked criteria, unchecked DoD, no implementation
  plan and valid documentation references; MSH-106–108 belong to M4 and109 to M5.
- `git diff --check` passed. Diff inspection found only the 13 named task
  records and three documentation files. No production, active-task, completed
  task, milestone-definition, project configuration or instruction file changed.
- `git diff --name-only 6412f48 codex/rewrite` at implementation tip `6983806`
  showed six implementation-session paths, disjoint from this planning diff.
  Integration is clean by current path comparison; recheck after that session
  finishes because later changes may overlap. No integration was performed.

The temporary comparison snapshots/scripts are convenience files, not repository
dependencies. Reproduce the comparison with the pinned CLI against base commit
`6412f48` and this branch; the task records carry the complete durable graph.
Rust/frontend tests, runtime architecture diagnostics and packaged application
checks were not run for this planning-only change. No game, performance,
strength or platform verification result is inferred from these planning checks.

## Integration validation (2026-10-27)

After MSH-95 completed and pushed, the designated integrator merged planning
commit `75d99a66939465c1577d748e8b5509d70bf6d8e0` into `codex/rewrite` at
`ebc7bd8b15f44b27b0abe71c6d9a75125588db3f`, with implementation parent
`50b114a9a1624ed15700cc88e5a221fb23745457`. This supersedes the isolated
planner's session-only integration restriction above; its historical account
remains intact. The primary checkout stays on `codex/rewrite`.

Reconciliation required no task edits or renumbering. The planning diff and all
later implementation/evidence paths were disjoint. MSH-95 remains Done;
MSH-105 retains its complete latest evidence, To Do status and satisfied
dependencies. No subsequent implementation or diagnostic review was started.

Actual checks on Darwin 27.0.0 arm64, pinned Backlog.md 1.55.1 (the PATH version
is 1.55.0 and was not used for mutations):

- Refreshed `origin` with `git fetch origin`; the clean primary checkout and
  remote integration tip both matched `50b114a`. No In Progress/In Review tasks
  were found by `backlog task list --status 'In Progress,In Review' --plain`.
- Read-only Python/PyYAML comparison of every task against Git parent `50b114a`
  passed: all 105 existing task bodies/evidence and metadata remain identical
  except the nine explicitly requested dependency additions and timestamps.
  Four new records produce 109 unique/resolved IDs. Full dependency DFS passed
  with no missing dependencies, duplicate edges, self-dependencies or cycles.
- Exact gate and reachability assertions passed for MSH-106–109, MSH-47,
  MSH-49–55/MSH-104 and transitive MSH-56. Earlier implementation tasks and
  MSH-48 gain no review prerequisite. MSH-106–108 remain in M4 and MSH-109 in M5;
  each remains unassigned Ready with eight unchecked criteria, four unchecked
  DoD items, no implementation plan and valid documentation references. All
  retain the full diagnostic/output contract and valid no-material-change
  outcome. These assertions can be reproduced from the task frontmatter and
  bodies at the two merge parents and the merged tree; temporary scripts are
  convenience files only.
- Pinned `backlog doctor`, `git diff --check` and `git diff --cached --check`
  passed. The integrated planning scope is exactly 13 task records and three
  documentation files. Production behavior is unchanged, so no new production,
  GUI or platform tests were run; existing MSH-95 evidence remains applicable.
- A separate Codex reviewer independently read every planning change and
  performed read-only Git/Python graph and preservation checks against the
  planning base and final implementation parent. Review passed with no
  actionable findings. This reviewed planning integrity, not runtime
  architectural health.
- Before cleanup, the ordinary planning worktree had no tracked changes,
  untracked files or ignored files. `lsof +D` found no process using it; its
  recorded sole remaining purpose was integration. Cleanup is permitted only
  after the merge is confirmed pushed, using ordinary non-forced Git worktree
  removal and deletion of the integrated disposable local branch.
