---
id: MSH-180
title: Shogi Search Core Adoption
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-15T23:02:00Z'
labels:
  - handle:shogi-search-core-adoption
milestone: m-5
dependencies: []
priority: high
ordinal: 187000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Search Core Adoption.

**Problem:** Shogi still uses the first full-width search (`mshogi.shogi.full-width.v1`): material only, at most depth 4 and 20,000 nodes, default depth 2 and 5,000 nodes. The shared alpha-beta core gave Chess and Tori large gains (MSH-164 (Shared Search Core), MSH-165 (Tori Search Core Adoption)). Project decision 2026-11-15: Shogi and Xiangqi move to the core first, then Chu Shogi and Tenjiku Shogi.

**Objective:** Shogi searches through the shared core and is much stronger than its present engine.

**Approach:** Follow the path of MSH-165 and "How to add a game" in `docs/m5/shared-search-core.md`: a fast search position for Shogi, a classical evaluation, and an adapter that keeps the native rules as the authority for the root, the chosen move and the reported line. If the core lacks something Shogi needs, add it to the core through the trait; do not add a search loop for one game. Shogi needs drops with the Pawn file, last-rank and drop-mate rules (as Tori has for the Swallow), optional promotion, and a repetition approximation where perpetual check loses for the checking side; the native rules decide entering-king declarations at the root. Keep the USI adapter working. Saved observations of the present search identity must stay valid; the identity split of saved observations belongs in one shared helper, not in a copy for each game. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Use the time and session mechanism of MSH-179 (Clock and Session for All Games) if it is complete; it is not a dependency. Level 1, with local checks of all engine, console, runner and desktop consumers of Shogi.

**Completion:** Shogi plays through the shared core, its private search is removed, and the game test shows it stronger than the present engine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shogi searches through the shared core with no game-specific branch inside the core
- [x] #2 The Shogi search position agrees with the native rules on perft counts and on a randomized differential test of legal actions, resulting positions and hashes
- [x] #3 The private Shogi search is removed, the engine, console, runner and desktop paths of Shogi still work, and saved observations of the earlier search identity stay valid
- [x] #4 Shogi with the shared core is stronger than its present engine in a game test with a sequential decision
- [x] #5 Approximations of the native rules inside the search, and any change to the trait or core, are recorded in docs/m5/shared-search-core.md
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze b5d14b3b as the predecessor. Implement and verify a fast Shogi position against native rules, including drops, promotion and perpetual check.
2. Replace private minimax with the shared core; retain native root declarations and line validation. Share observation identity classification with Tori and check all consumers.
3. Run Level 1 native differential/perft, search decisions, engine/USI/runner/desktop tests and local consumer compilation. Broader platform and packaged GUI checks remain deferred to integration/release.
4. Compare ordinary and tactical roots for depth and elapsed time. Run a bounded paired predecessor game test with a sequential decision under the current roadmap. Initial resource bound: 30 minutes of local match wall time, no external engine or cloud spending. Record settings before the match and stop to reconsider if practical benefit is absent.
5. Obtain independent review, resolve findings, integrate into main and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex in chat 01a1e160-c3a4-7e02-912f-c64d50b95f4d. Checkout: primary repository, branch codex/shogi-search-core. Startup: clean main at b5d14b3b; no active task owners or other active chat in this checkout. Backlog 1.55.3 matches recorded baseline. MSH-179 is not complete; retain finite node-budget interface in this task.

Checkpoint before game test: frozen b5d14b3b release predecessor at target/strength/engines/b5d14b3b45e4/mshogi. Eight ordinary-root calibration: candidate depth64/20,000 nodes median31.15ms/depth7 versus predecessor depth2/5,000 nodes median36.95ms/depth2. Four reduced tactical roots:16.07ms/depth10.5 versus9.73ms/depth2. Candidate150,000 nodes was159.21ms on ordinary roots and is not the equal-time setting. Retain 20,000 nodes for paired test; no claim of identical time on all positions. Protocol fixed before games: H0=0/H1=15, alpha=beta=.05, min30/max600 color pairs, four workers, seeded openings1..600, native outcomes only,180s pair wall/1800s campaign wall. Incomplete pair stops the campaign unscored. Continue to test practical benefit; no external opponent or cloud spending. First calibration failed at old experiment depth bound before engine launch; fixed consumer admission and retained failure. Independent code review found old desktop analysis caps; corrected native bounds and added command-path regression. Full workspace check first failed compilation because desktop has no direct search dependency; reuse engine-exported constants instead.

Retain decision: frozen d2391b21 candidate won64, drew0 and lost0 against b5d14b3b at32 color pairs; H1 accepted (LLR18.3723, upper2.9444),118.0006s wall. All games concluded natively; no failed, cutoff or excluded games. Candidate median request31.08ms/depth7; predecessor80.99ms/depth2 on their respective game positions (descriptive; controlled timing is same-root calibration). Initial independent review found only old desktop analysis caps, now fixed with regression. Final broad verification and evidence closure in progress. The broader workspace run exposed two obsolete exact opening-move assertions; replaced with native action legality plus equality to recorded engine choice, and all3 desktop_shared tests pass. Current source checkpoint66a9d8fb changes only those tests and comments after frozen candidate d2391b21.

Final source bf329f3d. Verification record: docs/m5/evidence/msh180/verification.json. Final coverage1167 passing Rust tests with10 explicitly ignored resource tests, using unchanged broad-suite results plus corrected shared_internal3 and desktop316 reruns. Broad command failures remain recorded. Clippy all-targets with warnings denied, formatting, frontend TypeScript and11 DOM tests on Node24,32 Python harness tests, standalone Tauri host check passed. No platform, GUI packaging or external-rating claim. Final review checks all32 report hashes/native results and independently replays first/last pairs. Practical decision: retain. Next priority-ready task is MSH-181 (Xiangqi Search Core Adoption); MSH-179 (Clock and Session for All Games) and MSH-50 (Conventional Shogi) retain their separate scope.

Independent final review ACCEPT, no remaining findings: docs/m5/evidence/msh180/review.md. Reviewer verified source/scope reuse, exactly13 additional unchanged desktop-verification tests, final union1167/10, all32 compressed report hashes and64 native wins. First and last pairs replayed independently. Integration into main remains the final completion step.
<!-- SECTION:NOTES:END -->
