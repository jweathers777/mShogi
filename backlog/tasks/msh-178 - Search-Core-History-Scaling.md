---
id: MSH-178
title: Search Core History Scaling
status: Done
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T00:21:00Z'
labels:
  - handle:search-history-scaling
milestone: m-5
dependencies: []
priority: medium
ordinal: 185000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Search Core History Scaling.

**Problem:** The continuation-history table of the shared core (`crates/mshogi-search/src/alphabeta/mod.rs`) has `HISTORY_SLOTS` squared entries of four bytes. With slots of side, piece type and target square this is about 2 MB for Chess, 3 MB for Tori and about 20 MB for Shogi, but about 430 MB for Chu Shogi and more than 2 GB for Tenjiku Shogi (see `docs/m5/shared-search-core.md`). Chu and Tenjiku cannot use the core until this changes.

**Objective:** The ordering statistics of the core have a bounded memory cost for any board size, without a loss of Chess strength.

**Approach:** Change the core, not a game: for example a separate smaller index for continuation history supplied through the trait, or a fixed-size hashed table. Keep the other history tables proportionate as well. Chess and Tori must keep their strength. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Level 1.

**Completion:** The memory of the history tables has a recorded bound that fits Chu and Tenjiku, and Chess is not weaker.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The memory of all history tables of the core has a recorded bound that does not grow with the square of board squares times piece types, with the expected size for Chess, Tori, Shogi, Xiangqi, Chu and Tenjiku recorded
- [x] #2 The core still contains no game-specific type or rule, and the Chess and Tori search tests pass
- [x] #3 A game test of Chess against the engine at the start of the task shows no loss of strength: H1 for a gain, or a recorded result that the change is not weaker
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze main 61f8f0c1 as the predecessor. Keep exact dense history behavior for all four current adapters. Bound continuation storage at 32 MiB and basic history at 256 KiB; use a shared hashed index only above those bounds. This removes the large-game quadratic allocation without game-specific core branches.
2. Verify allocation bounds, overflow safety, clear/decay and collision updates, plus exact dense behavior. Record expected memory for all six games. Run Chess, Tori, Shogi and Xiangqi correctness and shared consumer regressions, development blunder checks and a bounded predecessor game comparison.
3. Practical benefit: Chu and Tenjiku ordering history fits a fixed 32.25 MiB ceiling instead of hundreds of MiB or GiB. Compare actual allocated entries and ordinary-case speed/decisions; preserve current-game behavior. Record the game protocol before play, with sequential stopping and actual compute. Initial comparison cap is 600 pairs at fast time control, four workers; reassess at the checkpoint if evidence shows regression or no decision.
4. Level 1: portable private search storage only. Local macOS arm64 checks and independent review; Linux, Windows and packaged GUI checks deferred to the next integration/release checkpoint. Integrate reviewed source into main, push, and remove the completed branch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex, chat 01a1e160-c3a4-7e02-912f-c64d50b95f4d. Primary checkout was clean main 61f8f0c1, just released by this chat after MSH-181 (Xiangqi Search Core Adoption). New branch: codex/search-history-scaling. Other existing worktrees remain untouched. Backlog 1.55.3 matches the project baseline. No task dependencies. Native rules, saved history and search policy remain unchanged.

Pre-game protocol: frozen predecessor 61f8f0c1; H0 = -10 Elo and H1 = 0 Elo, alpha = beta = 0.05, minimum 30 and maximum 600 color pairs, 50 ms per move, four workers, seed 178, existing 90 development roots, native automatic claims and full reports. Stop at 1,800 seconds wall with an explicit no-decision result if needed. H1 excludes a loss of 10 Elo under the harness model; it does not prove exact equality. Combine this result with exact dense indexing and fixed-depth decision equivalence for the recorded not-weaker acceptance alternative. The independent reviewer accepted the design before implementation. Baseline benchmark: 466,994 nodes; development blunder suite: 113 of 146 avoided, 59 reference matches.

Frozen production source 5742d8f4. All 127 search tests passed (one ignored), including five new storage tests and current-game perft/differential checks. Independent source review found no blocking issue. Final source f121672d changes only a test expectation to saturating arithmetic for Clippy; production is unchanged. Final Clippy passed. Initial candidate timing and timed blunder runs overlapped broad compilation and are retained as load-confounded; both engines will be measured again under like load before game acceptance. Fixed-depth nodes and moves match in all 18 roots across three repetitions.

Local verification complete on final test source f121672d: workspace 1,181 passed, zero failed, 10 ignored; final five storage tests, Clippy and formatting passed. Independent final-source review accepts the equivalent test-only correction. Clean paired benchmark after broad work stopped: predecessor 1,076,614 nodes/s, candidate 1,089,688 nodes/s; exact 466,994 nodes and identical moves in all 18 roots across three repetitions. Clean development blunders: predecessor 112/146 avoided, candidate 113/146; both 59 reference matches and median depth 14. These show no aggregate regression, not a strength gain. Approved paired game comparison has started; completion review remains pending its evidence.

Game checkpoint: the original 1,800-second wall bound stopped the comparison after 571 scored color pairs. Candidate 300 wins, 291 losses, 551 draws; score 50.394%. The SPRT reached NO DECISION: LLR 1.3956 within ±2.9444; descriptive estimate +2.74 Elo, interval -11.86 to +17.34. Zero failed pairs; four in-flight pairs cancelled at the wall bound are retained and included in compute accounting. Both engines had median depth 10 and median 81,920 nodes. No extension or statistical non-inferiority claim. At this checkpoint, the proposed engineering decision is retain for the proved memory bound and unchanged current-game ordering arithmetic, supported by 54 identical fixed-depth runs, clean speed/blunder parity and no contrary game signal. The roadmap permits other recorded grounds after no decision. Final independent audit must accept these grounds before closure.

Final independent review accepted with no blocking findings: docs/m5/evidence/msh178/review.md. The reviewer independently verified all 1,142 archive member hashes/sizes/source bytes, all 571 pair scores and engine hashes, complete 575-index accounting, CPU including four cancellations, 54 fixed-depth equivalences, and native replay of the first and last completed pairs. Retain on the recorded engineering not-weaker grounds; the SPRT remains no decision and its interval does not exclude a 10-Elo loss. AC3 uses its recorded not-weaker alternative, not a claim of statistical non-inferiority. All local checks and review support Level 1 completion. Integration into main is next.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
The shared core now caps basic history at 256 KiB and continuation history at 32 MiB: at most 32.25 MiB of payload, plus fixed metadata. Larger slot spaces use bounded indexing; all four current adapters keep exact dense indices and update arithmetic. Chu and Tenjiku expected memory is recorded in docs/m5/shared-search-core.md. Native rules and search policy are unchanged.

All acceptance criteria have evidence in docs/m5/evidence/msh178/README.md and verification.json. Workspace verification passed 1,181 tests, with 10 explicitly ignored; search, history bounds/collisions, Clippy and formatting passed. Independent review accepted with no blocking findings and audited all report hashes, scores, compute accounting and representative native replays.

The original 30-minute Chess comparison completed 571 color pairs: 300 wins, 291 losses, 551 draws, zero failed pairs and four recorded cancellations. The SPRT reached NO DECISION; no statistical non-inferiority or gain is claimed. AC3 uses its recorded not-weaker alternative on independently reviewed engineering grounds: exact unchanged dense behavior, 54 identical fixed-depth searches, clean speed/blunder parity, and no contrary game signal. The demonstrated benefit is the memory bound. No protocol extension was needed for this storage-only change.

Integrated into main by merge de2572c28a665f53d02178090649f16a9b972334. Frozen production: 5742d8f480fb9eca253fd591532601a724ca4829. Final test source: f121672d8421eacaa8a974b314f73c2b86d2cda1. Evidence: 1a9f58ea. Integrated product and test files match the verified source.

Level 1 on macOS arm64. Linux, Windows and packaged GUI execution remain deferred to the next significant integration/release checkpoint. The future hashed ordering quality belongs to MSH-182 (Chu Search Core Adoption) and MSH-183 (Tenjiku Search Core Adoption). Next recommended eligible task: MSH-182 (Chu Search Core Adoption). MSH-179 (Clock and Session for All Games) remains separate.
<!-- SECTION:FINAL_SUMMARY:END -->
