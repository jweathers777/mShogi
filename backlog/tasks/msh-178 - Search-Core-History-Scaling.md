---
id: MSH-178
title: Search Core History Scaling
status: Ready
assignee: []
created_date: '2026-11-15T22:31:00Z'
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
