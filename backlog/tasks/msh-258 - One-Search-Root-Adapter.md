---
id: MSH-258
title: One Search Root Adapter
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T19:10:00Z'
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
- [x] #1 One generic root search in mshogi-search serves the five games; per-game code holds only real differences (Xiangqi pending adjudication, Shogi declarations, conclusion scores, line limits).
- [x] #2 Search results, node counts and best moves at fixed depth are identical before and after (the fixed-depth comparison is recorded, so no game test is needed); workspace checks pass.
- [x] #3 Production line counts before and after are recorded.
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

Reviewed product commit 22e2417 vs baseline 02cc9a2. Physical production lines (including comments and blank lines, excluding cfg(test) imports, declarations and inline test modules): root modules 1342 → 862; all changed production Rust files 2608 → 2066. With test-only sections included the corresponding file totals are 1358 → 884 and 2956 → 2420. On an Apple M1 Pro, cargo run --offline --manifest-path runs/msh-258/probe/Cargo.toml produced byte-identical full observations on both commits. Each two-action PV by square index: Shogi 76→67, 4→13; Xiangqi 1→20, 82→65; Tori 45→39, 3→9 (both Phoenix); Chu 134→121, 2→13; Tenjiku both profiles 164→148, 65→81. All actions unpromoted. Each search chose the first action of its PV; the scores and visits are recorded above. Reproducible probe, lockfile and both outputs: gs://mshogi-experiments-216675633801/runs/msh-258/. No cloud compute or VMs used. Candidate routine CI passed in run 45880607542. Local fmt and Clippy passed. Next: finish local workspace and frontend tests, then integrate and clean codex/msh-258-search-root.

Final checks on 22e2417 passed with python3 tools/checks.py run --base origin/main on Apple M1 Pro and the pinned Node 24.19.0. Rust formatting and workspace all-target Clippy passed. Workspace tests passed (1165 passed, 10 ignored by the repository). TypeScript and all 575 frontend tests passed. Ubuntu routine CI 45880607542 passed release search regressions, formatting and consumer compilation. The change affects root orchestration with no rules, private search position, search-strength or OS-specific code changes. No strength match or release packaging was required. No paid compute used.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
One root search in mshogi-search now handles Shogi, Xiangqi, Tori, Chu and Tenjiku. The engine maps all five results in one place. All six depth-2 runs matched the predecessor in best move, full line, score, nodes and other reported observations. Production root code fell from 1342 to 862 lines. Independent review fixes, workspace checks and routine CI passed. The direct dependent is MSH-243 (Game-Independent Strength Tool).
<!-- SECTION:FINAL_SUMMARY:END -->
