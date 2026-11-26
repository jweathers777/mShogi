---
id: MSH-254
title: One Perft for All Games
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T17:00:00Z'
labels:
  - core
  - console
  - refactor
  - model:medium
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each game has its own perft and divide API in mshogi-core: chess/perft.rs (u32 depth, no budget), shogi/perft.rs, xiangqi/perft.rs (u8 depth, maximum 3), tori/perft.rs (visit budget 1,000,000 and report types) and tenjiku/perft.rs (budget 100,000 and stop). Chu has none, so the console (chu_session.rs, about 58 lines) and chu/conformance.rs write their own recursion. The console also copies the private chess perft order_key (session.rs). The console visit limit is 1,000,000 for Tori and 100,000 for the others, and tenjiku_session.rs writes 100_000 instead of the constant. Perft is rules evidence, so the counts must not change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic perft and divide with depth limit, visit budget and stop serves all six games in mshogi-core; per-game perft code keeps only what the game needs, such as its node order.
- [ ] #2 Console perft and divide use it for every game with one budget rule; the console Chu recursion and the copied order_key are removed.
- [ ] #3 All perft expected values and conformance results are unchanged; workspace checks pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add one rules traversal and typed reports. Use thin native adapters for board copies, transitions, restoration and divide order. Keep Tenjiku console on full Game history.
2. Migrate native callers and all six console commands. Put ASCII depth and visit validation in one console function with a 1,000,000-visit default and cap.
3. Run existing rules, conformance and workspace checks, plus focused budget checks. Get a fresh review, integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Checkpoint: claimed MSH-254 on codex/msh-254-one-perft. Next, map native rules traversal and console callers, then consolidate and verify.

Checkpoint on codex/msh-254-one-perft: all native callers now use one exact-depth traversal. The six shells use one optional ASCII visit budget, with a default and cap of 1,000,000. Chess, Standard Shogi and Xiangqi now accept an explicit budget. Chu divide visits include the root, as do the other divides. Xiangqi divide depth zero now returns the common error, consistent with its documented positive depth. Tenjiku retains full Game history and endings in the shell. Next: complete workspace checks and fresh review, then integrate.

Fresh review of 094c31d..4e51a9f confirmed native mode/order, restoration on cancellation and unchanged fixture expectations. Fixed its Chess test binding finding. Preserved an absolute full-Game Tenjiku ceiling using the same 1,000,000-visit constant as the shell and native Tori, and test the ceiling. Next: finish workspace and deep Chess checks, then integrate.
<!-- SECTION:NOTES:END -->
