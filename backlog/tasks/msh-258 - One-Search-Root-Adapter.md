---
id: MSH-258
title: One Search Root Adapter
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T11:46:00Z'
labels:
  - search
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-222
priority: medium
ordinal: 7400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
crates/mshogi-search has one root adapter per game: chu.rs, tenjiku.rs, tori.rs, shogi.rs and xiangqi.rs (268 to 451 lines). Each defines Limits, Decision, SearchResult, SearchStatus, SearchError, search, new_engine, search_with_stop, search_with_state (root checks, mapping of the core outcome, PV replay through a native copy) and terminal_score. Chu and Tenjiku differ in about 60 of 300 normalized lines. This is the search-side partner of MSH-222 (Shared Engine Adapters), which runs first; use the same per-game trait shape where it fits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic root search in mshogi-search serves the five games; per-game code holds only real differences (Xiangqi pending adjudication, Shogi declarations, conclusion scores, line limits).
- [ ] #2 Search results, node counts and best moves at fixed depth are identical before and after (the fixed-depth comparison is recorded, so no game test is needed); workspace checks pass.
- [ ] #3 Production line counts before and after are recorded.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-222 (2026-11-26), finding in this scope: mshogi-engine/src/game.rs converts each search module's SearchResult with a search! macro (Shogi, Tori, Chu, Tenjiku) and a separate Xiangqi impl; StopReason has From impls for search::SearchStatus and xiangqi::Status. One generic search SearchResult (decision, status) lets these become one conversion. The engine trait EngineGame reads EVALUATOR_ID, SEARCH_ID, WIN_SCORE, MAX_DEPTH, MAX_NODES, MAX_LINE, DEFAULT_LIMITS, terminal_score and new_engine from each search module; Limits::valid is the same in all five and the engine now checks it once (EngineGame::valid_limits).

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Status is To Do while an unfinished dependency blocks work; otherwise Ready. No implementation has started.
<!-- SECTION:NOTES:END -->
