---
id: MSH-182
title: Chu Search Core Adoption
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T09:38:00Z'
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

Pre-game calibration checkpoint on frozen candidate 3d028f61 and predecessor e6e2d51f: choose candidate depth 64 / 100,000 nodes against predecessor maximum depth 2 / 2,000 nodes. Same-root median request times candidate/predecessor: ordinary 274/351 ms with depth 7/1; compact 75/329 ms with depth 5/1; tactical 58/146 ms with nominal depth 14/2. Terminal-tree nominal depth is not strength evidence. Candidate is faster than the stronger predecessor setting in each group. Frozen eight compact families and 600 setup seeds produce 591 distinct native states; all roots remain live and balanced in material. No result-based selection.

Protocol fixed before play: separate standard seed 0 and compact seed 1 full-pair feasibility pilots, one worker, 180-second pair watchdog and native 500-step cap; retain both regardless of score. If compact resource completion permits, use the same budgets for seeds 1–600, H0 = 0/H1 = 15 Elo, alpha = beta = 0.05, minimum 30/maximum 600 pairs, four workers, 180 seconds per pair and 1,800 seconds campaign wall. Any unscored pair stops the campaign; no exclusions or invented draws. Standard-pilot evidence is separate from the conditional reduced-position strength claim. Native event/replay/report limits and draw-agreement semantics remain unchanged.

Both predeclared feasibility pilots completed with native outcomes. Standard seed 0: candidate won both games, 75 and 94 plies, 115.65 seconds total; request medians candidate/predecessor 299/759 ms and depth 6/1. Compact seed 1: candidate won both games, 3 and 10 plies, 8.77 seconds total. The pilots remain separate and are not added to the sequential result. Compact resource completion permits the unchanged fixed-corpus campaign, which is now running at the predeclared settings. No family or root was removed based on outcomes.

First game checkpoint failed: the campaign stopped after 12 launched pairs and 119.90 seconds. Eleven complete pairs gave 22 candidate wins; pair 12 gave one win and one unscored candidate NodeLimit before depth 1 (100,000 nodes, no applied action). The SPRT has no decision and AC4 remains unmet. No pair was excluded or replaced; standard pilot, pair 1 and failed pair 12 replayed natively. Complete evidence remains in game-test/.

Change direction for the next bounded round: inspect the full first-iteration path in capture-based Chu, where legal moves can leave all royals capturable and check extensions/quiescence may explore those losing moves. Proposed practical improvement is a cheap private proof of an available one-action capture of all enemy royals, preserving complete legal move generation, native outcomes and root/PV admission. Compare the failed root, ordinary roots and all fixed families at the same 100,000-node budget; run native differential/regression checks and the same frozen-predecessor game gate if the improvement is sound. No budget-only workaround or result-based corpus change. Record and review the concrete approach before editing.

Concrete round approved after independent source review: add an interior-only one-action all-enemy-royal capture shortcut to private rule_score, after actual royal/material terminals and before approximate repetition. The proof must use complete-route local admission including Lion law/protection and all victims; separate threats to two royals are insufficient. Score is MATE_SCORE minus current ply minus one. All legal moves, native root/PV validation and capacity refusals stay intact. No game-specific core change. Bound validation to focused both-side/compound/Lion/material/state tests, failed seed-12 root and fixed ordinary/family root checks at 100,000 nodes, then the unchanged predecessor campaign if checks pass. A short PV ending before the proven next capture remains a selective line, not a complete native proof.

Round-two checkpoint failed broadly: the complete 600-root sweep took 140.71 seconds; candidate admitted 581 actions and failed depth 0 at 100,000 nodes on 19 roots, while the predecessor admitted all 600. Failures span families 4, 6 and 8. Seed 12 is fixed but first-iteration cost is not solved. No later pilot or game campaign ran. All reports, failed-root IDs and native replay of seed 46 remain in round2/.

Reconsider the search structure rather than patch individual roots. Independent review recommends a bounded generic check/threat-extension policy experiment: preserve current defaults for other games, disable depth extensions for Chu, and keep all quiet evasions plus the no-stand-pat rule in threatened quiescence. The current core extends even depth-zero and root-threat nodes, which can keep large capture-based quiet trees alive. Baseline is frozen 675a0083. Intended benefit: complete the first iteration across ordinary and failed families at the unchanged 100,000-node budget. First check all 19 failures and representative passed roots, then all 600 first requests, ordinary/tactical comparisons and the unchanged game gate. Retain only evidence of broad improvement; if ineffective, change approach before more local fixes. Diagnostic root sweep is bounded at 600 seconds/four workers; game campaign remains 1,800 seconds. No family removal, budget-only fix, or change to native legal actions. Other-game fixed-depth nodes/moves must stay identical; under the new roadmap rule that needs no game test.

The bounded diagnostic rejects the proposed check-extension experiment before implementation: all 19 failed roots and every legal root child are unthreatened. Their depth-one work enters quiescence directly. At a diagnostic-only one-million-node bound, all complete depth one in 101,250–493,181 nodes, with selective depth 14–23. The original acceptance/game budget remains 100,000. Root legal counts are 117–132 versus 70–90 distinct successor effects, and passing controls also contain equivalent routes. No extension-policy production change was made. Next measure a coherent equivalent-route reduction in private search before choosing it; keep full native legal/perft behavior and explicit exact successor equivalence.

Diagnostic route-equivalence reduction alone leaves four of the 19 failed roots above 100,000 nodes. The common cause is unfiltered quiescence captures (Chu exchange_at_least always returns true). Start a bounded classical static-exchange round against frozen 675a0083: after full initial action admission, count both victims and promotion; alternate complete admitted recaptures of the last mover at its actual final square, including Lion return routes. Each side can decline. Use threshold minimax with at most 64 candidate projections, 512 generated routes and eight recapture plies per call; unresolved bounds or terminal uncertainty retain the move. Preserve royal/material terminal captures, full legal generation, native admission and the shared core. Deduplicate only equal resulting state plus equal next target. Practical objective is completion across all failed and ordinary roots at unchanged 100,000 nodes. Validate independent exchange fixtures, existing differential/perft tests and 29 diagnostic roots within two minutes, then unchanged 600-root gate, ordinary/tactical timing and predecessor campaign if supported. No extension-policy change or budget increase. Review this coherent round before accepting it.

First bounded exchange checkpoint: existing Chu tests and initial independent exchange fixtures pass. At unchanged 100,000 nodes, all ten passing controls still complete, but only six of 19 previous failures now complete; 13 still fail. The 29-root check took 2.231 seconds. Do not run the 600-root or game gate on this result. Separate conservative royal-threat refusals from exchange work-limit refusals before changing the algorithm. The material-exchange model is a private heuristic, not a full tactical proof; assess whether the blanket threat guard defeats the conventional approach. Retain source and diagnostic evidence, keep native contracts and final game criteria unchanged.

Independent review clarifies that retaining every geometric royal threat is a conservative design option, not a native rule or public correctness requirement. Conventional static exchange estimates model material recaptures and may undervalue future checking sacrifices. If diagnostics identify that blanket guard as the main cost, remove it within this coherent exchange round; retain actual royal captures/missing royals, narrow material cases, complete local legality and Unknown-to-keep work limits. The shared core already bypasses SEE when the current node is threatened. Document this private selectivity limit; do not call a rejected material exchange a proven tactical loss.

Measured guard ablation: 308,070 SEE calls with 54,169 geometric-royal unknowns and only nine route-budget unknowns; no projection/geometry/depth exhaustion. Removing that optional guard reduces calls to 224,266 and unknown royal/material cases to 549, but seven of 19 roots still fail (46, 52, 118, 268, 420, 510, 556). Thus increasing work bounds is not supported. Test the coherent combination of proper exchange filtering and equivalent-successor route suppression on the same 29 roots within two minutes before further production changes. Earlier route suppression alone was insufficient; combining the two measured sources of redundant work is a new hypothesis. If supported, expose a default generate_search trait hook with unchanged behavior for all other games, preserve complete generate/perft for Chu, and use exact board/turn/rights equality for search-only route equivalence. No native action or history contract changes.

Combined diagnostic passes all 29 roots at unchanged 100,000 nodes. The 19 former failures use 7,513–85,024 nodes (median 36,296 versus 133,552 on frozen675 at the diagnostic one-million bound); ten controls also pass. Diagnostic wrapper verifies exact cells/side/Lion rights in addition to hash. Wall2.260 seconds; control search time158ms versus227ms for675 despite wrapper copying full history. Proceed with the reviewed default generate_search hook and exact Chu successor reduction. Require complete successor-set differential tests, fixed-depth node/best-move identity for other games, focused consumers/review, then frozen600-root/timing/game gates. This is a checkpoint to continue, not strength acceptance.

Production combined checkpoint: 26 Chu tests passed with one ignored benchmark; the added active Lion-window successor check also passed. Actual production29-root check completes all at100k; former failures median36,296nodes, controls84ms versus675227ms (single sample, not strength). Independent final source review accepted. All64 fixed-depth searches across Chess/Tori/Shogi/Xiangqi match before and after the default hook byte-for-byte, including nodes,bestmove,score; no other-game game test is needed under the requested roadmap exception. Durable evidence: round3/diagnosis and round3/non-chu-hook. Full workspace regression is running before candidate freeze and full600/game gates.

Final third-round full release workspace run passed: 1,201 tests, zero failures, 11 ignored. The explicit active-window successor test also passed after its test-only tightening. Independent review accepts the source and64 other-game fixed-depth identity. Clippy is running; no timed game work overlaps these checks. Source changes are frozen for final candidate commit.

Third-round Clippy all workspace targets and formatting checks passed. Freeze the independently reviewed source for unchanged600-root, timing and game protocol. No test/build process will overlap measured game work.
<!-- SECTION:NOTES:END -->
