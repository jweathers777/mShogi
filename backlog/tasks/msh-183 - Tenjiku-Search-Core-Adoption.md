---
id: MSH-183
title: Tenjiku Search Core Adoption
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T15:15:00Z'
labels:
  - handle:tenjiku-search-core-adoption
milestone: m-5
dependencies:
  - MSH-178
  - MSH-182
priority: medium
ordinal: 190000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tenjiku Search Core Adoption.

**Problem:** Tenjiku Shogi still uses the first full-width search (`mshogi.tenjiku.full-width.v1`): material only, at most depth 2 and 2,000 nodes. The shared alpha-beta core gave Chess and Tori large gains (MSH-164 (Shared Search Core), MSH-165 (Tori Search Core Adoption)). Project decision 2026-11-15: Shogi and Xiangqi move to the core first, then Chu Shogi and Tenjiku Shogi.

**Objective:** Tenjiku searches through the shared core and is much stronger than its present engine.

**Approach:** Follow the path of MSH-165 and "How to add a game" in `docs/m5/shared-search-core.md`: a fast search position for Tenjiku, a classical evaluation, and an adapter that keeps the native rules as the authority for the root, the chosen move and the reported line. If the core lacks something Tenjiku needs, add it to the core through the trait; do not add a search loop for one game. Tenjiku needs a 256-square board and the Fire Demon burn, so make and unmake need a variable-length capture record and the tactical gain must include burned pieces. It also needs the four jumping generals (native rules in `crates/mshogi-core/src/tenjiku/legal.rs`): Great General (rank 3), Vice General (rank 2, also an area move), Rook and Bishop General (rank 1); King and Crown Prince have rank 4. A general jumps over any number of pieces of lower rank, of either side, and only a piece of equal or higher rank stops it; after a jump it cannot capture a royal piece, and what else it can capture differs between the profiles. So move generation, attack detection and exchange estimates must use occupancy by rank, and a general can attack through many pieces. Both rule profiles (Adams and Muller) come from the native game. Reuse what the Chu adoption built for large boards and Lion moves. Saved observations of the present search identity must stay valid; the identity split of saved observations belongs in one shared helper, not in a copy for each game. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Use the time and session mechanism of MSH-179 (Clock and Session for All Games) if it is complete; it is not a dependency. Level 1, with local checks of all engine, console, runner and desktop consumers of Tenjiku.

**Completion:** Tenjiku plays through the shared core, its private search is removed, and the game test shows it stronger than the present engine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tenjiku searches through the shared core with no game-specific branch inside the core
- [ ] #2 The Tenjiku search position agrees with the native rules on perft counts and on a randomized differential test of legal actions, resulting positions and hashes, with constructed positions that exercise general jumps and post-jump captures in both profiles
- [ ] #3 The private Tenjiku search is removed, the engine, console, runner and desktop paths of Tenjiku still work, and saved observations of the earlier search identity stay valid
- [ ] #4 Tenjiku with the shared core is stronger than its present engine in a game test with a sequential decision
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
1. Freeze main 4fe70d24 as predecessor. Inspect both native Tenjiku profiles and the full search/consumer path. Build a compact 256-square position with rank occupancy, complete routes, promotion-refusal rights and reversible direct/burn/suicide effects. Reuse the shared core and the bounded Chu mechanisms; measure generation and state-update cost against native local operations.
2. Replace the private full-width loop. Preserve exact native root conclusions, repetition admission, selected actions and replayable lines. Keep both immutable profiles and legacy saved observations through the shared identity helper. Inspect engine, console, runner, desktop and standalone host consumers before broad checks.
3. Level 1 on macOS arm64: independent profile fixtures, hand/native perft, randomized action/result/right/hash/undo comparisons, general rank/post-jump cases in both profiles, burn/suicide/promotion/compound tactics and cancellation. Run affected consumers, broader search regressions and independent review. Linux, Windows and packaged GUI execution stay at the next significant integration/release checkpoint.
4. Calibrate ordinary, tactical and balanced reduced roots against the frozen predecessor. Freeze a two-profile corpus before games. Record comparable finite work settings, H0=0/H1=15, alpha/beta0.05, minimum30/maximum600 color pairs and actual compute before each profile game gate; initial combined campaign wall bound1800seconds with four total workers. Keep standard pilots separate, preserve every failure and stop on unscored pairs. Compare practical completion, speed/depth, decisions and predecessor strength; reconsider the approach if weak.
5. Record all private approximations and evidence. Integrate the independently reviewed, verified result into main, push and remove the completed branch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex, chat01a1e160-c3a4-7e02-912f-c64d50b95f4d, branchcodex/tenjiku-search-core in the primary checkout released by this chat after MSH-182 (Chu Search Core Adoption). Main4fe70d24 was clean, no In Progress tasks, dependencies178/182 Done; other worktrees remain untouched. Backlog1.55.3 matches baseline. MSH-179 (Clock and Session for All Games) is still Ready and separate. Baseline Tenjiku is material-only, defaultdepth1/1024nodes and maximum2/2000. Native full-width transitions dominate cost; earlier standard maximum was about7.5seconds. Both Adams and Muller profiles must keep complete rules and native histories. Initial work is one primary integration owner with isolated position, consumer and harness assignments; independent review follows.

Concrete position design: packed256-cell board preserving original IDs/base/promotion/refusal rights, four-word occupancy by owner/rank/royal/Demon, complete compact moves with up to3landings or stationaryigui, and reversible changed-square records for direct captures/burn/suicide/promotion. Share the small square-set mechanism with Chu; keep core q8/history/TT mechanisms. Both profiles require their own Demon phase and general post-jump admission. Private bare/repetition logic must follow Tenjiku meanings, never Chu material/protected-Lion rules. Root and selected/PV actions remain native. Corpus proposal fixed before games: eight balanced mirrored18–24piece families cover all36originals with identical placements/IDs/rights across profiles; four quiet rear setup moves, bothprofiles all600seeds. Separate ordinary/tactical controls and standardpilot perprofile. Independent profile SPRTs initially share1800seconds (900each), four total workers. All settings require calibration and completion checks before games. No ratings or external engines.

Initial correctness checkpoint: nine adapter tests pass, preserving native zero/partial/stop, no-action, counter refusal, both-royal compound capture and profile-specific dying Demon behavior. Four private-position tests pass:176 independent movement sets (44faces*2sides*2profiles), concrete profile fixture commands, handperft3/9 plus native counts and >300random full-action/state/right/hash/undo comparisons. Additional effects/history tests and cost measurements are still pending. Sixteen Chu fixed-depth searches match nodes,bestmoves,scores byte-for-byte before/after shared SquareSet extraction; the roadmap identical-search exception applies. Corpus checks pass4tests:600prefixes perprofile,570distinctstates perprofile, all36originals and no immediate base-family terminal action; duplicates retained. Python harness36tests and oldconfigwriter compatibility pass. Frontend Tenjiku17tests/typecheck pass. No games or timing calibration yet.

Measured checkpoint: fast generation is 6.0x native on standard roots and 1.9x on active area/burn roots in both profiles. Reconstruction of 913 accepted known events averages 93–97 microseconds. Independent review found and fixed a Muller Free Eagle blocked-ray threat/SEE prefilter omission, with an exact native regression. Consumer checks pass engine11, session28 (5ignored), consoleCLI8, frontend17/typecheck and standalonehost1/check. Console internal tests preserve a standard-board Timeout under the provisional300000-node default and existing15s deadline (2passed/4failed). Do not relax that deadline. Calibrate the coherent source at10000nodes first, single worker, both profiles and both predecessor settings; bounded600seconds initial calibration, all failures retained. No game-strength conclusion yet.

Current decision: use depth64/10000nodes for interactive default after both standard roots, eight ordinary Adams roots, both eight-root reduced sets and Adams tactical controls complete. Standard candidate4/3depth at1.10/0.77seconds. Old default1/1024 fails reduced seed8 with depth0; retain it. Old maximum2/2000 completes both reduced sets and is the proposed game comparator. Reduced median request cost54ms candidate versus1.9seconds predecessor; both nominal depth1 there. These are completion/performance observations, not strength. Fixed quiet prefix at seed6 permits an immediate conclusion; retain it and disclose this corpus limit. Independent source review has no unresolved blocker; runtime console rerun and both profile game gates remain open.

Source checkpointf4e1b7c2: full workspace1218passed/1failed/13ignored. The sole failure is an obsolete runner depth3 rejection; change it to65 for the new64 limit and rerun affected runner checks. All Tenjiku console6/selfplay2 journeys pass unchanged15s deadlines. Clippy requests two equivalent is_multiple_of expressions; apply and rerun. Calibration82pairs/251.71summed stage seconds: candidate10k admits all actions; oldmaximum admits all40fixed roots; olddefault seed8 fails bothprofiles. An exposed native-only tactical proof on existing seed1000002 checks all42replies perprofile: new8f-4b+ forces actual royal capture in3; old8f-9e+ permitsR10d-10c and none of606native replies wins immediately. This is post-calibration decision evidence, not a blind strength score. Game settings and1320s separate pilot cap plus1800s reduced campaign cap are recorded before play in corpus-protocol.md.

Final sourcef3f4ee2c passes full search129tests(4ignored), runner31(2ignored), workspace/all-target clippy with warnings denied, both format checks and the standalone Tenjiku default host test. Reuse the other1218 workspace passes fromf4e1b7c2: the only production delta is equivalent unsigned parity spelling; corrected runner boundary now passes. Independent reviewer approves this source before games. Frozen final candidate and runner/config identities are recorded; separate standard pilots and profile SPRTs now run with the prewritten caps. No strength acceptance or integration yet.

Game checkpoint FAILED at practical completion. Both standard color pairs stop on candidate depth0 NodeLimit after8–9steps at10000nodes; no timeout. Adams reduced first4pair batch also stops:3completepairs plus1partial (7completed wins), candidate incomplete at seed4 swapped step2. No Muller reduced run and no SPRT decision. Preserve all6launchedpairs/fiveexactfailedrequests/native replay in round1/. Actual summedstagewall149.88seconds. Decision: stop strength campaigns and change work-budget/search approach before anotherround. Initialroot calibration was insufficient for ordinary trajectories. Bound nextmeasurement to120seconds on exactfailedroots plusordinaryroots at50k/100k, inspect firstiteration and main-threat extension versus qsearch cost; no fixtureexclusions or fabricated results.

Bounded diagnosis supports a budget correction, not a core change: first iterations for all five refused roots complete at11266–44540nodes (0.16–0.98seconds); qsearch dominates, with selective depth9. The hypothesized repeated-threat extension path is not the measured cause here. Full100k turns on failures plusbothinitialroots complete at leastdepth1 with observed maximum7.71seconds. Keep q8 and core policy unchanged. Confirm frozenunmodified adapter agreement within the120second diagnostic runtime cap, then use explicit100k for a new independent pilot checkpoint. Do not yet change productdefault or claim restored gameplay completion.

Frozen unmodified f3f4ee2c confirms all7diagnostic roots at100k: exact node/depth/best agreement with passive instrumented copy, all choices completed, maximum7.516seconds. Actual measured search/probe runtime92.220seconds (plus0.013s rejected wire command); total preparation-to-receipt elapsed318.646seconds including compilation/analysis/messages. Thus the120second runtime cap was met, but an end-to-end120second interpretation was not; record both, no false wall-cap claim. No production source changed. Round2 runs only both standard pilots at explicit100k, then stops at a checkpoint before reduced tests; protocol in round2/.
<!-- SECTION:NOTES:END -->
