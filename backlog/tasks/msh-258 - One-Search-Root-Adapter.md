---
id: MSH-258
title: One Search Root Adapter
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T19:02:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Capture fresh depth-2 results from the starting position of each game and both Tenjiku profiles. 2. Move root checks, iterative-search dispatch, result mapping and line replay into one generic function. Keep native command, cancellation, declaration and adjudication behavior in game-specific hooks. 3. Share the engine result conversion. Compare fixed-depth observations and production line counts. Run workspace checks, get independent review, and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-222 (2026-11-26), finding in this scope: mshogi-engine/src/game.rs converts each search module's SearchResult with a search! macro (Shogi, Tori, Chu, Tenjiku) and a separate Xiangqi impl; StopReason has From impls for search::SearchStatus and xiangqi::Status. One generic search SearchResult (decision, status) lets these become one conversion. The engine trait EngineGame reads EVALUATOR_ID, SEARCH_ID, WIN_SCORE, MAX_DEPTH, MAX_NODES, MAX_LINE, DEFAULT_LIMITS, terminal_score and new_engine from each search module; Limits::valid is the same in all five and the engine now checks it once (EngineGame::valid_limits).

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-258-search-root from 02cc9a2. Next: capture baseline and consolidate the five root adapters.

Baseline on 02cc9a2: fresh engine, depth 2, 500000 nodes, no clock, standard starts. All six cases completed depth 2. Shogi 125 nodes, score 0, 76→67; Xiangqi explicit claims 787, 0, 1→20 (no pending, Unknown); Tori 358, +8, 45→39 Phoenix; Chu 149, 0, 134→121; Tenjiku Adams Modern 426 and Muller Completed 579, both +10, 164→148 travel. Coordinates are zero-based square indices. Captured decisions, full PV, perspective, depth, score, status, nodes and Xiangqi observations in runs/msh-258/before.txt. Baseline root module physical lines: Shogi 285, Xiangqi 317, Tori 253, Chu 234, Tenjiku 269 (1358 total). Next: implement on codex/msh-258-search-root.

Implementation is on codex/msh-258-search-root. One generic root now drives all five games and one engine conversion handles the typed result. All workspace targets compile. Before and after depth-2 observations are byte-identical in all six cases; SHA-256 0785eb03170ad4834e485e34f067e2270326385558a07bfc254b08296d1d0f25. Next: independent review, repository checks, any fixes, then integration.

Independent review identified two issues: charge Xiangqi natural-count history inspection before the private search receives its time budget, and keep each named game entry point type-specific. Fixed both. The engine still checks each reported claim against a private native game. Next: compare fixed-depth observations again and finish workspace and CI checks.
<!-- SECTION:NOTES:END -->
