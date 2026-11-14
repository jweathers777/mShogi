---
id: MSH-165
title: Tori Search Core Adoption
status: Done
assignee:
  - '@claude-fable'
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-14T15:48:00Z'
labels:
  - handle:tori-search-core-adoption
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: medium
ordinal: 169000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Search Core Adoption.

**Problem:** A search core built with Chess only can hold Chess-only assumptions. The project goal is reuse across chess-like games and easy addition of new games.

**Objective:** Prove the shared core boundary with a second, different game early. Tori Shogi is small (7x7) and has drops and its own repetition rule.

**Approach:** Give Tori Shogi a search position for the shared core and remove its private minimax copy. Change the trait where Tori shows a Chess-only assumption; do not add a Tori-only path in the core. Record what Shogi, Xiangqi, Chu Shogi and Tenjiku Shogi will need (board width, drops, lion moves, repetition rules) as notes for their later tasks, without building it now.

**Completion:** Tori plays through the shared core, its old search copy is removed, and the trait changes are recorded.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tori Shogi searches through the shared core with no game-specific branch inside the core
- [x] #2 The Tori search position agrees with the native Tori rules on a differential test of legal actions, including drops
- [x] #3 The private Tori minimax copy is removed and Tori engine and desktop paths still work
- [x] #4 Tori with the shared core is stronger than its predecessor in a game test
- [x] #5 Requirements for the four other games on the trait are recorded for their later tasks
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search position (`crates/mshogi-search/src/tori/position.rs`): 49-bit bitboards, hands, incremental hash and material, pseudo-legal generation of moves and drops, make/unmake with self-check, Swallow file, last-rank and drop-mate rules, compulsory promotion. Repetition on the search path scores zero as an approximation; the native forbidden departures of the root are excluded at the root.
2. Tests: random walks and perft that compare legal actions, resulting state, check status and hash with the native Tori rules.
3. Adapter (`tori.rs`): native root conclusion and native legal actions are the authority; the chosen action must be in the native list; the line is cut where the native game refuses an action or concludes. Remove the minimax copy and its test seam.
4. Engine protocol: new search and evaluator identities with new bounded limits and a validation that fits a selective search. Keep the validation of saved observations of the old identity (material score of the line), because saved Tori documents contain them.
5. Consumers: console session, desktop session limits, experiment runner limits, fixtures and tests.
6. Game test: new engine against a frozen predecessor binary through the internal protocol with the native Tori game as controller.
7. Record trait findings and the needs of Shogi, Xiangqi, Chu Shogi and Tenjiku Shogi in docs/m5/shared-search-core.md. Level 1; independent review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Worker: @claude-fable, branch claude/msh-165-tori-core, worktree ../mShogi-msh165 (the primary checkout holds uncommitted work of MSH-170 (Unused Code Removal)).

Verification Level 1. Record: docs/m5/evidence/msh165/verification.json. Final revision 1eef64a6 (rebased on MSH-170): clippy clean, 1150 workspace tests passed, frontend 570 passed. Independent review (Claude Opus 5.5, fresh context): no blocking defect; findings 1, 3, 4 and 7 corrected, 2 recorded, 5 and 6 accepted as observations; corrections not re-reviewed. Not run: Linux, Windows, packaged desktop journey.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tori Shogi now searches through the shared alpha-beta core. A new bitboard search position (crates/mshogi-search/src/tori/position.rs) has drops, compulsory promotion, the Swallow file, last-rank and drop-mate rules, incremental hash and material, and an approximation of the repetition rule; the native forbidden departures are removed at the root. The adapter (tori.rs) returns the native conclusion at a concluded root, returns only an action that Game::legal_actions contains, and replays the reported line natively. The private full-width minimax is removed. The core did not change. Why: prove the shared core with a second, different game, and make Tori stronger.

Evidence: the search position agrees with the native rules on perft (depth 1 to 3) and on 48 seeded random games with many drops, captures and promotions. Game tests through the internal protocol against the frozen earlier engine 6fef75d1: equal time 40 wins, 0 losses, 0 draws; against the largest earlier setting 26 wins, 0 losses, 0 draws (unscored games are runner size-limit stops). Median depth at the desktop setting is 10 against 2.

Decisions and limits: new identities mshogi.tori.shared-core.v1 and mshogi.tori.material-placement.v1; limits depth 1 to 64, up to 5,000,000 nodes, desktop 300,000 nodes; line at most eight actions. Saved observations of the earlier identities keep their exact checks. A new-identity score is not exactly checked (selective search), but a choice cannot claim a win at its own root. Tori has no time budget and no kept state between moves (see MSH-167). Findings for Shogi, Xiangqi, Chu Shogi and Tenjiku Shogi, including the continuation-history memory change needed before Chu Shogi, are in docs/m5/shared-search-core.md. Integrated on codex/rewrite up to b9142fe5.
<!-- SECTION:FINAL_SUMMARY:END -->
