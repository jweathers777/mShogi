---
id: MSH-164
title: Shared Search Core
status: In Review
assignee:
  - '@claude-fable'
created_date: '2026-11-13T21:28:00Z'
updated_date: '2026-11-14T10:18:00Z'
labels:
  - handle:shared-search-core
milestone: m-5
dependencies:
  - MSH-163
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 168000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shared Search Core.

**Problem:** The Chess search reaches a median depth of 8-9 in five seconds. It runs on the native rules cursor (array board, a legal-move list and position copies at each node, 0.11-0.27 million nodes per second) and uses a table of at most 16,384 history-keyed entries that is rebuilt for each move. Each of the other five games has its own copy of a plain minimax. There is no shared search.

**Objective:** One game-independent classical search core, first used by Chess, that is much faster and deeper than the present Chess search.

**Approach:** A search core generic over a small position trait (make, unmake, generate, hash, evaluate, in check, terminal). The core owns iterative deepening, principal-variation alpha-beta, quiescence, move ordering, a large compact board-keyed transposition table kept for the full game, and repetition and move-count detection on the search path. Chess supplies a search-internal bitboard position with incremental hash and evaluation and staged pseudo-legal move generation. The native rules stay the authority for the real game; perft and differential tests prove agreement. Carry over the present evaluation and the present selectivity that still helps. Internal APIs have no compatibility duty. The old Chess search stays available as the frozen predecessor until the new one is accepted, then it is removed.

**Completion:** Chess plays through the shared core in the engine and desktop paths, passes the correctness checks and wins the game test against the predecessor.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The Chess search position agrees with the native rules on perft counts for standard test positions and on a randomized differential test of legal moves and resulting positions
- [ ] #2 The transposition table is keyed on board state, is configurable in megabytes and is kept between moves of one game
- [x] #3 The search core contains no Chess-specific type or rule; Chess reaches it only through the position trait
- [x] #4 The speed benchmark shows at least 1 million nodes per second and a clearly larger depth at five seconds than the recorded baseline
- [x] #5 The game test against the frozen predecessor shows the new engine is stronger under the harness stopping rule
- [x] #6 The blunder suite result is better than the recorded baseline
- [x] #7 Real games still take every result, claim and history from the native rules, and the desktop and runner paths use the new search
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Shared core in crates/mshogi-search/src/alphabeta: a SearchPosition trait (hash, in check, staged pseudo-legal generation, make/unmake with legality result, null move, rule-draw score, evaluation, exchange test, move encoding for the table) and a generic searcher: iterative deepening with aspiration, principal-variation alpha-beta, quiescence, table cutoffs, null move, late-move reductions, check extension, shallow futility, killer/history/continuation ordering, soft and hard time limits, node and stop limits. No Chess type in the core.
2. Transposition table in alphabeta/table.rs: 16-byte entries in buckets of four, keyed on the position hash only, size set in megabytes, generation aging, owned by the caller so it lasts for the full game.
3. Chess search position in chess_bitboard: bitboards with magic slider attacks (safe Rust), make/unmake in place, incremental Zobrist hash and incremental material/placement/phase, staged pseudo-legal generation, static exchange evaluation, repetition and fifty-move detection on the path with the earlier game keys.
4. Port the present evaluation to the bitboard position with the same arithmetic (chess_eval), including the KPK bitbase and specialized endings. Differential test: new evaluation equals the old one on many positions.
5. Correctness tests against the native rules: perft counts on standard positions, randomized differential test of legal moves and resulting positions, hash and incremental-state consistency after make/unmake.
6. chess_engine adapter: builds the search position from an AdjudicatedGame (replays the history for repetition keys), takes root outcome and claims from the native rules, checks the chosen move against native legal actions, returns the existing SearchResult type. SearchCoordinator worker owns the table between requests; UCI gets a Hash option (megabytes) and clears the table on ucinewgame.
7. Acceptance with the MSH-163 harness: bench, blunder suite at 1000 ms, game test against frozen predecessor 65887fb0 at 200 ms per move.
8. After acceptance remove the old chess_conventional search, its evaluation file, tests and profile examples. Independent review by a different model. Update docs/m5 and the roadmap.
Verification level 1 (platform-independent search code, local checks and independent review). The three-platform check is deferred to the next integration milestone.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @claude-fable (Claude Code session, model Fable 5.2), branch codex/rewrite in the primary checkout, started 2026-11-14 at b5758a28. Backlog CLI 1.55.3 matches the baseline.

Verification level 1 (platform-independent search code; local checks and independent review). Three-platform compatibility and a packaged desktop run are deferred to the next integration milestone and are not claimed. Environment: Apple M1 Pro, macOS 27.0, Rust 1.99.1, Python 3.11. One structured record: docs/m5/evidence/msh164/verification.json. Design: docs/m5/shared-search-core.md.
Commits on codex/rewrite: a01eb633 (new core, Chess bitboard position, evaluation port, consumers switched; earlier search still in the tree; test proves evaluation equality on more than 8000 positions), ec6dcda8 (earlier search, evaluation, tests and three profile examples removed), 6fef75d1 (review corrections).
Results at 6fef75d1: cargo test --workspace 1173 passed, 0 failed, 11 ignored; clippy clean; bench median 1,380,211 nps (baseline 296,656), 857,606 nodes for the fixed depths (baseline 5,431,097); depth at 5 s 14-16 middlegame and 20-24 endgame; blunder suite at 1000 ms avoided 85/146 (baseline 47); game test against frozen 65887fb0 at 200 ms per move: SPRT H1 at 30 pairs, +55 -0 =5.
Decisions: (1) The tree scores a repeated position, a halfmove counter of 100 and a claimable fifty-move draw as draws also when the consumer cannot claim (UCI), because the opponent or controller can claim; two UCI tests were changed to this. The root still always gives a move to a consumer that cannot claim. (2) Tests that asserted internals of the earlier search (exact node counts, fullmove-counter overflow below the root) were updated or replaced; overflow at the root is still reported as a search failure. (3) Slider magic numbers are fixed constants, so a debug build starts at once. (4) A partial iteration improves the result only after one complete iteration, so the incomplete-result contract is unchanged.
Independent review: Claude Opus 5.5 subagent, read-only, 8 findings. Findings 2, 3, 4, 6, 7 corrected in 6fef75d1. Finding 5 is the existing incomplete-result contract. Finding 8 (large Hash allocation outside the time budget) is a recorded limit. Finding 1: the desktop and internal execution path starts one engine process for each request, so the table is kept between moves on the UCI path only; follow-up MSH-169 (Engine Session Persistence). No re-review of the corrections.
Acceptance criterion 2 is not checked: its last clause is true on the UCI path and not on the desktop path. Owner decision requested. Definition of Done items 1 and 4 wait for that decision.
Other follow-up noticed, not done: crates/mshogi-search/src/chess_candidate.rs is an earlier search generation with no consumer. New frozen predecessor for later tasks: target/strength/engines/6fef75d1b672/mshogi (build-engine 6fef75d1).
<!-- SECTION:NOTES:END -->
