---
id: MSH-183
title: Tenjiku Search Core Adoption
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T16:02:00Z'
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
1. Freeze main4fe70d24 as predecessor. Inspect both native Tenjiku profiles and the full search/consumer path. Use a compact256-square position with rank occupancy, complete routes, promotion-refusal rights and reversible direct/burn/suicide effects; measure local generation and history reconstruction against native operations.
2. Replace the private full-width loop with the shared core. Keep exact native root conclusions/repetition admission, selected actions and replayed lines. Preserve both immutable profiles and legacy observations through the shared identity helper. Reuse the square-set mechanism with Chu and prove unchanged Chu fixed-depth search.
3. Level1 on macOS arm64: independent profile fixtures, perft, randomized full action/state/right/hash/undo comparisons, general jumps, complete burn/promotion/compound tactics, cancellation and all affected consumers. Use independent review. Linux, Windows and packaged GUI remain at the next significant integration/release checkpoint.
4. Retain failed game checkpoints and measure their full path. The final candidate uses q4 and50000nodes after q8/10000 andq8/100000 fail first-iteration completion. Verify all6failed positions plus ordinary/reduced/tactical controls, then fresh source-frozen standard pilots and separate profile SPRTs: H0=0/H1=15, alpha/beta.05,min30/max600pairs,900seconds/profile,fourworkers. Preserve every launched game and stop on unscored pairs; do not change the fixed corpus or score failures.
5. Reconcile private approximations, exact tested source and evidence. Audit native replays and both sequential decisions, track independent report replay cost in MSH-184 (Native Report Replay Cost), then integrate reviewed work into main, push and remove the completed branch.
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

Round2 Adams still fails candidate firstiteration at100k (step11,3.036seconds,depth0); othercolor reaches native conclusion after92steps, but the pair is unaccepted after outer645second timeout. Complete report was written; source shows repeated final report validation including post-write summary validation. No score counted. ExistingMuller pilot remains bounded; no furthergames. Decision: budget-only correction has failed twice, so change direction. Test one coherent private policy change: Tenjiku q horizon8→4, preserving full threatened responses beforehorizon, native rules and the sharedcore. Compare allknownfailures plusordinary/reduced/tacticalcontrols atfixedlimits; retain/reject on completion/cost/decisions before freshgamegates. Do not weaken no-completed-iteration semantics or exclude positions. Repeatedreportvalidation is a separate observedconsumercost, not explanation for the candidate refusal.

Q4 controlled matrix completed54requests with choices and maximum4.587seconds: all6knownfailed firstiterations now221/1031/7876/149/894/1905nodes; bothinitialcontrols91/99. Full50k matrix covers sixfailedroots, sixteenordinary, sixteenreduced and eighttactical controls. Native response validation and final receipt pending. Decision: retain q4 for the next candidate and set interactive default50k (ample measured firstiteration margin, lower cost than q8/100k). Independent hand-solved core horizon/cache regressions now exercise both4 and8. Run focused search/consumer/host checks, then fresh source-frozen pilot and profile game gates; no acceptance yet.

All54q4 responses pass native admission; both existing tactical seed1000002 choices remain8f-4b+, so the recorded native all-reply proof applies unchanged. Measurement used86.068seconds of requests,148.752seconds conservative process charge and420.925seconds elapsed, within300/660bounds. Sourcef822b955 is the q4/50k candidate. Report replay cost is tracked separately as MSH-184 (Native Report Replay Cost), dependent on this evidence; no report validation or score-admission behavior changed here.

Candidatef822b955 passes final q4/50k checks: search129(4ignored), engine11, desktop-session28(5ignored), console16 including standardselfplay, standalonehost1, workspace/all-target clippy with warnings denied and bothformatchecks. Console/session journeys runserial to keep measured15second request deadlines independent of simultaneous testcases; boards/deadlines/outcomeassertions unchanged. Independentreview approves pilots. Frozenf822 engine built17.48seconds; round3pilots run now at50k againstoldmaximum, unchangedf3nativecontroller provenance and prewritten600/645second bounds. Script stops afterboth; no reducedcampaign yet.

Round3 Adams standard pilot PASSED onf822b955/q4/50k: bothgames conclude natively, candidatewins at11/38steps,230.69seconds wall. Candidate/predecessor medianrequest1.656/7.154seconds andcompleteddepth4/1. No incompletechoice or timeout. Mullerstandardpilot underway; no reducedcampaign yet. This is feasibility evidence, not the separate sequential strength gate.

Round3 standard checkpoint PASSED: all4games conclude natively with4candidate wins, no incompletechoices/timeouts. Adams11/38steps230.69s; Muller39/32steps326.88s; total557.57s. Candidate/old medianrequest1.656/7.154s Adams,1.378/6.898s Muller; completeddepth4/1 and5/1. Decision CONTINUE to fresh separate profile reducedSPRTs with recordedunchangedcorpus/H0=0/H1=15/.05/min30max600/900sprofile/4workers/180spair, startingseed1. No priorpilot/failedroundscorescarryforward; stoponanyunscoredpair. Strength acceptance stillpending.

Final Adams reducedSPRT accepted H1 after32completepairs:59wins/0draws/5losses, no failures/unscoredgames,245.52seconds wall. H0=0/H1=15,alpha/beta.05,min30/max600 unchanged; alllaunchedpairs retained. Candidate/old medianrequest0.263/1.789seconds anddepth5/1. This is the fixed exposed reduced-corpus predecessor claim, not a rating. FreshMullerprofilecampaign underway with separateledger; finalreview/replays andintegrationpending.

Both final profile gates PASS: Muller also acceptsH1 after32completepairs,59wins/0draws/5losses,no failures,269.74seconds. Candidate/old medianrequest0.269/1.808seconds anddepth5/1. Adams245.52s plusMuller269.74s gives515.26seconds reducedcampaignwall, within1800cap. All4standardgames and128reducedgames conclude natively; no finalround exclusions/unscoredgames. Bounded native replay and final independent audit now run before main integration.

PAUSED at user request on2026-11-16 before laptop shutdown/disconnection. All games and native replays have finished; worker agents interrupted, no further work authorized until resume. Final sourcef822b955/q4/50000 passes local checks and both SPRTs: each32pairs,59wins/0draws/5losses; all4standardpilotgames also candidatewins. Native replay26selectedpairs passed in23.71seconds; raw audit covers66pairs/132nativegames, independent audit recomputes bothLLRs6.28334 above2.94444. Evidence is underdocs/m5/evidence/msh183/round3. Remaining: finish aggregate compute/README curation, reviewer final replay-receipt acceptance, reconcile verification/summary, check supported acceptance criteria, integrate into main, push and remove ownfeaturebranch. Main/originmain lastconfirmed4fe70d24; featurecodex/tenjiku-search-core owns primarycheckout. MSH-184 (Native Report Replay Cost) created as separate Ready follow-up depending on183. No merge or push performed for183. Resume from this local checkpoint; do not rerun completed games or broad checks without a new reason.
<!-- SECTION:NOTES:END -->
