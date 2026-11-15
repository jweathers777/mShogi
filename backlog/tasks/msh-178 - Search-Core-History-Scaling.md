---
id: MSH-178
title: Search Core History Scaling
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-15T23:40:00Z'
labels:
  - handle:search-history-scaling
milestone: m-5
dependencies: []
priority: medium
ordinal: 185000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Search Core History Scaling.

**Problem:** The continuation-history table of the shared core (`crates/mshogi-search/src/alphabeta/mod.rs`) has `HISTORY_SLOTS` squared entries of four bytes. With slots of side, piece type and target square this is about 2 MB for Chess, 3 MB for Tori and about 20 MB for Shogi, but about 430 MB for Chu Shogi and more than 2 GB for Tenjiku Shogi (see `docs/m5/shared-search-core.md`). Chu and Tenjiku cannot use the core until this changes.

**Objective:** The ordering statistics of the core have a bounded memory cost for any board size, without a loss of Chess strength.

**Approach:** Change the core, not a game: for example a separate smaller index for continuation history supplied through the trait, or a fixed-size hashed table. Keep the other history tables proportionate as well. Chess and Tori must keep their strength. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Level 1.

**Completion:** The memory of the history tables has a recorded bound that fits Chu and Tenjiku, and Chess is not weaker.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The memory of all history tables of the core has a recorded bound that does not grow with the square of board squares times piece types, with the expected size for Chess, Tori, Shogi, Xiangqi, Chu and Tenjiku recorded
- [ ] #2 The core still contains no game-specific type or rule, and the Chess and Tori search tests pass
- [ ] #3 A game test of Chess against the engine at the start of the task shows no loss of strength: H1 for a gain, or a recorded result that the change is not weaker
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze main 61f8f0c1 as the predecessor. Keep exact dense history behavior for all four current adapters. Bound continuation storage at 32 MiB and basic history at 256 KiB; use a shared hashed index only above those bounds. This removes the large-game quadratic allocation without game-specific core branches.
2. Verify allocation bounds, overflow safety, clear/decay and collision updates, plus exact dense behavior. Record expected memory for all six games. Run Chess, Tori, Shogi and Xiangqi correctness and shared consumer regressions, development blunder checks and a bounded predecessor game comparison.
3. Practical benefit: Chu and Tenjiku ordering history fits a fixed 32.25 MiB ceiling instead of hundreds of MiB or GiB. Compare actual allocated entries and ordinary-case speed/decisions; preserve current-game behavior. Record the game protocol before play, with sequential stopping and actual compute. Initial comparison cap is 600 pairs at fast time control, four workers; reassess at the checkpoint if evidence shows regression or no decision.
4. Level 1: portable private search storage only. Local macOS arm64 checks and independent review; Linux, Windows and packaged GUI checks deferred to the next integration/release checkpoint. Integrate reviewed source into main, push, and remove the completed branch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex, chat 01a1e160-c3a4-7e02-912f-c64d50b95f4d. Primary checkout was clean main 61f8f0c1, just released by this chat after MSH-181 (Xiangqi Search Core Adoption). New branch: codex/search-history-scaling. Other existing worktrees remain untouched. Backlog 1.55.3 matches the project baseline. No task dependencies. Native rules, saved history and search policy remain unchanged.

Pre-game protocol: frozen predecessor 61f8f0c1; H0 = -10 Elo and H1 = 0 Elo, alpha = beta = 0.05, minimum 30 and maximum 600 color pairs, 50 ms per move, four workers, seed 178, existing 90 development roots, native automatic claims and full reports. Stop at 1,800 seconds wall with an explicit no-decision result if needed. H1 excludes a loss of 10 Elo under the harness model; it does not prove exact equality. Combine this result with exact dense indexing and fixed-depth decision equivalence for the recorded not-weaker acceptance alternative. The independent reviewer accepted the design before implementation. Baseline benchmark: 466,994 nodes; development blunder suite: 113 of 146 avoided, 59 reference matches.
<!-- SECTION:NOTES:END -->
