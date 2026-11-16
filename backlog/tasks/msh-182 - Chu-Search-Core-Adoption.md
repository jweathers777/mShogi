---
id: MSH-182
title: Chu Search Core Adoption
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T08:38:00Z'
labels:
  - handle:chu-search-core-adoption
milestone: m-5
dependencies:
  - MSH-178
  - MSH-180
  - MSH-181
priority: medium
ordinal: 189000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chu Search Core Adoption.

**Problem:** Chu Shogi still uses the first full-width search (`mshogi.chu.full-width.v1`): material only, at most depth 2 and 2,000 nodes. MSH-176 (Legacy Runner Adapter) compares it with the original C++ engine. The shared alpha-beta core gave Chess and Tori large gains (MSH-164 (Shared Search Core), MSH-165 (Tori Search Core Adoption)). Project decision 2026-11-15: Shogi and Xiangqi move to the core first, then Chu Shogi and Tenjiku Shogi.

**Objective:** Chu searches through the shared core and is much stronger than its present engine.

**Approach:** Follow the path of MSH-165 and "How to add a game" in `docs/m5/shared-search-core.md`: a fast search position for Chu, a classical evaluation, and an adapter that keeps the native rules as the authority for the root, the chosen move and the reported line. If the core lacks something Chu needs, add it to the core through the trait; do not add a search loop for one game. Chu needs a 144-square board representation chosen by measurement, Lion moves with two steps (double capture, igui, return to the source) and their capture rules, a real pass that the null move must respect, optional promotion with its history condition, and its repetition rule. The history tables need the bound of the core history task first. Saved observations of the present search identity must stay valid; the identity split of saved observations belongs in one shared helper, not in a copy for each game. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Use the time and session mechanism of MSH-179 (Clock and Session for All Games) if it is complete; it is not a dependency. Level 1, with local checks of all engine, console, runner and desktop consumers of Chu.

**Completion:** Chu plays through the shared core, its private search is removed, and the game test shows it stronger than the present engine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Chu searches through the shared core with no game-specific branch inside the core
- [ ] #2 The Chu search position agrees with the native rules on perft counts and on a randomized differential test of legal actions, resulting positions and hashes
- [ ] #3 The private Chu search is removed, the engine, console, runner and desktop paths of Chu still work, and saved observations of the earlier search identity stay valid
- [ ] #4 Chu with the shared core is stronger than its present engine in a game test with a sequential decision
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
1. Freeze main e6e2d51f as predecessor. Inspect the complete native Chu path and measure its current limits. Build a fast 144-square search position with compact complete moves, reversible state, Lion protection and classical evaluation; compare an array/piece-list design with the native path before accepting it.
2. Replace the private full-width loop with the shared core. Keep native root conclusions, must-vary action admission, chosen actions and reported lines authoritative. Preserve no-board-action uncertainty, complete compound routes and saved legacy observations through the shared identity helper. Inspect every engine, console, runner and desktop consumer.
3. Level 1 on macOS arm64: independent fixtures/perft, randomized native action/result/hash/undo comparisons, special Lion/promotion/pass/history cases, local shared consumer regressions, frontend and standalone host checks, and independent review. Linux, Windows and packaged GUI checks remain deferred to the next significant integration/release checkpoint.
4. Calibrate time/depth and tactical decisions against the frozen predecessor. Record a paired sequential game protocol before play, with H1 near the expected gain, at least 600 maximum pairs, fast comparable budgets and actual compute. Initial local campaign wall bound 30 minutes. At the checkpoint retain only practical improvement; change direction if absent. The new identical-search exception does not apply to this adoption.
5. Record approximations and evidence, obtain independent review, integrate into main, push and remove the completed branch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex, chat 01a1e160-c3a4-7e02-912f-c64d50b95f4d; branch codex/chu-search-core in the free primary checkout. Dependencies MSH-178 (Search Core History Scaling), MSH-180 (Shogi Search Core Adoption) and MSH-181 (Xiangqi Search Core Adoption) are Done. Backlog 1.55.3 matches baseline. Other worktrees remain untouched. The requested identical-fixed-depth-search game-test exception was committed and pushed to main as e6e2d51f before this task. Current Chu baseline is material-only depth 1/512 nodes, maximum 2/2000; it repeatedly uses full native Game transitions. MSH-179 (Clock and Session for All Games) remains separate and Ready.

Implementation uses a 144-square compact array with three-word occupancy sets, compact complete routes and reversible state; representation cost and move generation will be measured against native local positions. Native root legal actions filter the fast root, and every selected action/reported line is replayed through native Game. The old private minimax-only toy seam is removed; the shared core already has independently solved non-Chess game tests. A native CHU-NOACTION fixture now directly protects the unscored root contract.

Before corpus selection, comparison scope is fixed: ordinary standard-start roots for timing/depth, a separate standard-start paired feasibility pilot, and balanced reduced-position families (12–24 pieces, Lions/sliders/promotion/Prince, native legal setup prefixes) for a bounded sequential comparison if full starts exceed native capacity. Retain all feasibility results regardless of score; choose on resource grounds only. Freeze all families before candidate games. Reduced-position results will be reported as conditional, not standard-start qualification. No unscored pair may be silently excluded from the sequential test.

First fast-position checkpoint: native local fixture coverage, perft and more than 1,500 randomized position comparisons pass. Independent source review found no confirmed blocker in the adapter, consumer identity split or local movement implementation. Focused integration exposed a deeper-search preference for a later royal win over the fixture immediate native material award; private C08 narrow exhaustion knowledge is being added with its bare-side reply exception, preserving native expectations. A native-counter refusal test needed a larger finite node budget to complete a selective first iteration; at 20,000 nodes it still reports the native capacity error and preserves the source. Legacy saved compound observations are now constructed independently from the known native route and historical score.

Coherent source checkpoint: C08 private narrow material scoring now preserves the independent immediate-win consumer fixture and all reply exceptions. Focused release Chu tests: 18 passed, one representation benchmark ignored; console 14 passed, engine protocol/routing seven passed, identity one passed. Desktop 27 passed with one new-test assertion failure: an operation refusal can retire a presentation revision while semantic state stays fixed. The corrected test asserts unchanged document/version/selection and native record and passes. The legacy compound observation is constructed independently from the native route and 999999 historical score, then saved/reopened successfully. Frontend ChuExecution ten tests passed on Node 24.19. Independent C08 review and focused native material test passed; documented private reply/history and native-mate precedence limits remain explicit.
<!-- SECTION:NOTES:END -->
