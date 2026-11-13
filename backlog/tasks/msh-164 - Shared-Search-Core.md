---
id: MSH-164
title: Shared Search Core
status: Ready
assignee: []
created_date: '2026-11-13T21:28:00Z'
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
- [ ] #1 The Chess search position agrees with the native rules on perft counts for standard test positions and on a randomized differential test of legal moves and resulting positions
- [ ] #2 The transposition table is keyed on board state, is configurable in megabytes and is kept between moves of one game
- [ ] #3 The search core contains no Chess-specific type or rule; Chess reaches it only through the position trait
- [ ] #4 The speed benchmark shows at least 1 million nodes per second and a clearly larger depth at five seconds than the recorded baseline
- [ ] #5 The game test against the frozen predecessor shows the new engine is stronger under the harness stopping rule
- [ ] #6 The blunder suite result is better than the recorded baseline
- [ ] #7 Real games still take every result, claim and history from the native rules, and the desktop and runner paths use the new search
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
