---
id: MSH-181
title: Xiangqi Search Core Adoption
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-15T23:14:00Z'
labels:
  - handle:xiangqi-search-core-adoption
milestone: m-5
dependencies: []
priority: high
ordinal: 188000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Search Core Adoption.

**Problem:** Xiangqi still uses the first full-width search (`mshogi.xiangqi.full-width.v1`): at most depth 3 and 512 nodes, default depth 1 and 128 nodes. The shared alpha-beta core gave Chess and Tori large gains (MSH-164 (Shared Search Core), MSH-165 (Tori Search Core Adoption)). Project decision 2026-11-15: Shogi and Xiangqi move to the core first, then Chu Shogi and Tenjiku Shogi.

**Objective:** Xiangqi searches through the shared core and is much stronger than its present engine.

**Approach:** Follow the path of MSH-165 and "How to add a game" in `docs/m5/shared-search-core.md`: a fast search position for Xiangqi, a classical evaluation, and an adapter that keeps the native rules as the authority for the root, the chosen move and the reported line. If the core lacks something Xiangqi needs, add it to the core through the trait; do not add a search loop for one game. Xiangqi needs blocked Horse and Elephant moves, the Cannon screen and the facing-Generals rule, and an approximation of the perpetual-check and chase rules in the search; draw claims of the selected profile come from the native rules at the root. Keep the UCI 0-9 adapter working. Saved observations of the present search identity must stay valid; the identity split of saved observations belongs in one shared helper, not in a copy for each game. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Use the time and session mechanism of MSH-179 (Clock and Session for All Games) if it is complete; it is not a dependency. Level 1, with local checks of all engine, console, runner and desktop consumers of Xiangqi.

**Completion:** Xiangqi plays through the shared core, its private search is removed, and the game test shows it stronger than the present engine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Xiangqi searches through the shared core with no game-specific branch inside the core
- [ ] #2 The Xiangqi search position agrees with the native rules on perft counts and on a randomized differential test of legal actions, resulting positions and hashes
- [ ] #3 The private Xiangqi search is removed, the engine, console, runner and desktop paths of Xiangqi still work, and saved observations of the earlier search identity stay valid
- [ ] #4 Xiangqi with the shared core is stronger than its present engine in a game test with a sequential decision
- [ ] #5 Approximations of the native rules inside the search, and any change to the trait or core, are recorded in docs/m5/shared-search-core.md
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
1. Freeze main at637c0a74 as predecessor. Build a fast Xiangqi position and verify native perft, randomized actions/results/hashes, blocked pieces, Cannon screens and facing Generals.
2. Replace private minimax with the shared core. Preserve native root conclusions, pending adjudication, exact-proof APIs, count claims and reported-line admission. Reuse the shared observation identity helper; update all engine, UCI0-9, runner and desktop consumers.
3. Level1: focused correctness and tactical decisions, then relevant workspace regression checks, frontend and standalone desktop host compilation. Linux/Windows/packaged GUI checks remain deferred to integration/release.
4. Calibrate same-root time/depth against predecessor. Fix a paired sequential game-test protocol before play, H0=0/H1=15 alpha=beta.05 min30/max600 pairs with native outcomes. Initial local game-test resource bound30minutes wall; no cloud or external opponents. At checkpoint retain only measured practical progress; change direction if absent.
5. Independent review, evidence, main integration, normal push and branch cleanup.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex, chat01a1e160-c3a4-7e02-912f-c64d50b95f4d; primary checkout branch codex/xiangqi-search-core. Clean main637c0a74, no active worker in checkout. Backlog1.55.3 matches baseline. MSH-179 (Clock and Session for All Games) remains Ready; retain finite node interface. Current design, execution/finalization instructions and shared-core guide read earlier in this same conversation; MSH-180 (Shogi Search Core Adoption) supplies accepted adapter/helper/harness patterns.

Pre-game checkpoint: six fast-position tests and13 search integration tests pass, including independent board fixtures/perft, over2,000 differential positions, both-color perpetual check, stalemate, bare General and losing-side native count claim. Current consumer checks pass after correcting obsolete old maximum-node fixture. Calibration on8 identical ordinary roots: candidate6,000 nodes/depth64 median36.17ms/depth5.5 versus frozen637c0a74 default1/128 median45.77ms/depth1. Candidate12k and20k cost52.46ms and57.53ms, so choose6k. Four tactical roots at6k cost9.13ms versus7.40ms; their high nominal depth is terminal-tree repetition, not strength evidence. Protocol fixed before full games: H0=0/H1=15 alpha=beta.05, min30/max600 color pairs, four workers, seeded openings1..600, native AutoValidClaims and native outcomes only,180s per pair and1800s campaign. Stop on incomplete pair with no exclusion; no external opponent or cloud spending. Retain/change decision follows game evidence, not speed alone.
<!-- SECTION:NOTES:END -->
