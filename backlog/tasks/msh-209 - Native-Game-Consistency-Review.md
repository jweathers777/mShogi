---
id: MSH-209
title: Native Game Consistency Review
status: To Do
assignee: []
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-23T11:09:00Z'
labels:
  - handle:native-consistency-review
  - architecture
  - rules
milestone: m-5
dependencies:
  - MSH-208
documentation:
  - docs/design.md
ordinal: 216000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Native Consistency Review

**Problem.** The six native games were built by different sessions in different design styles, so equivalent mechanisms differ without a product reason. Observed during MSH-204 (Tori Strength Harness) and MSH-208 (Tori Repetition Rule), 2026-11-20: only Tori, Chu and Tenjiku have a separate replay-work count (65,536 / 4,000,000 / 512,000,000), added in their delivery tasks to bound load time of untrusted files, while Shogi and Chess bound history by entry limits (Shogi 321, the CSA move limit) or irreversible moves; entry and byte limits differ per game (321 to 2,048 entries, 512 KiB to 1 MiB) without recorded reasons; native repetition checks compare full keys against all earlier positions in some games; Tori admission encoded the whole history at every move; a no-replay Game::fork exists only for Xiangqi and Tori; the incremental session protocol (MSH-189) serves only Shogi and Tori; engine searches stop repetition scans at the last capture/drop in drop games. John (2026-11-20): design style and implementations must be consistent across the engines, with code reuse wherever it is efficient (Do Not Repeat Yourself).

**Objective.** One consistent design for equivalent native and engine mechanisms across all six games, implemented once in shared code where that is efficient, with per-game differences only where a game rule or a measured need requires them.

**Approach.** Inventory each mechanism across the six games (resource limits and their purpose, repetition detection, record encoding and capacity checks, copies/fork, incremental sessions, undo/prefix, search-position history and repetition scans, search adapters). For each, record the current variants, the reason (rule, measurement or none) and a proposed common design. Present the proposals for approval before changing a recorded native contract; then implement approved changes in bounded follow-up tasks.

**Completion.** The criteria below are met with recorded evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An inventory (docs) covers every listed mechanism for Chess, Shogi, Xiangqi, Tori, Chu and Tenjiku, with the current implementation, its recorded source or reason, and duplicated code
- [ ] #2 Each inconsistency has a proposed common design (shared code where efficient) or a recorded rule/measurement reason to differ
- [ ] #3 Proposals that change a recorded native contract (limits, record formats, adjudication) are approved by John before implementation
- [ ] #4 Approved changes are implemented directly or as bounded follow-up tasks with handles and dependencies; no game rule changes without a decision
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-229 now records a codebase-wide audit/exactness inventory in its task notes. Concrete simplification work is split into MSH-230 (operational Stop history), MSH-231 (redundant record metadata), MSH-232 (native resource admission/copies), MSH-233 (execution identity/validation), with Xiangqi replay costs in existing MSH-227. Use that inventory as current input and avoid duplicate implementation tasks; no rules change was made by MSH-229.
<!-- SECTION:NOTES:END -->
