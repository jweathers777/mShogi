---
id: MSH-189
title: Shogi Request Overhead
status: Ready
assignee: []
created_date: '2026-11-17T16:41:00Z'
labels:
  - handle:shogi-request-overhead
milestone: m-5
dependencies:
  - MSH-187
documentation:
  - docs/m5/evidence/msh187/prefix-diagnosis/README.md
  - docs/m5/shogi-strength-round.md
  - docs/m5/shared-search-core.md
priority: high
ordinal: 196000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Request Overhead

## Problem
Each timed Shogi search request still receives and validates the complete native game ledger and reconstructs the game (strict decode plus `to_game`). MSH-187 (Shogi Strength Round) measured about 25 + 25 ms for this at 311 history events on the M1 Pro, about 10 + 10 ms at 159 events (docs/m5/evidence/msh187/prefix-diagnosis/README.md). The cost grows with game length and is spent inside the move budget. At fast controls late-game budgets were 115-200 ms, so this overhead takes a large part of each move. It will be larger for long Shogi games and for Chu and Tenjiku. Also, a timed search can return no move: MSH-187's failed campaigns stopped on requests that completed no iteration ("incomplete:Stopped", depth 0). A conventional engine always finishes at least depth 1 (or has a legal move ready) before it obeys a time stop. Desktop play with low clock time has the same risk.

## Objective
Per-move request cost that does not grow with game length, and a timed search that always returns a legal move.

## Approach
- The engine session keeps the authoritative native game for an active timed session and receives only the new actions since its last request. It applies them through the native rules (each new action is still validated by the native authority). A request that does not extend the kept game (takeback, new game, imported root, mismatch) falls back to full validation and resets the session. Native rules stay the single authority for moves, histories, declarations and results.
- Timed search: complete depth 1 (or, at worst, choose a native legal move by the root ordering) before a time stop can end the search. Fixed-work requests keep their present semantics.
- Use one shared mechanism that other games can adopt. Shogi is the first consumer; do not build a Shogi-only path.
- Measure before and after: request overhead at about 50, 150 and 300 history events, and a timed request near the minimum budget.

## Completion
Verification Level 2 (process and session lifetime, protocol). Independent review. No evaluation or search-policy change in this task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measured per-request overhead (excluding search) at about 300 history events is at most 10% of the overhead at task start, and grows at most slightly from 50 to 300 events; measurements and environment are recorded.
- [ ] #2 Every new action in a kept session is validated by the native rules; takeback, new game, imported root and mismatched history reset the session, with tests for each case.
- [ ] #3 A timed request always returns a native legal move, including at the minimum budget and in check; a test reproduces the MSH-187 depth-0 failure history and shows a returned move.
- [ ] #4 Fixed-work requests and saved-game formats are unchanged; affected desktop, USI and runner consumer tests pass.
- [ ] #5 The mechanism is shared and documented in docs/m5/shared-search-core.md for adoption by the other games; independent review accepts it.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
