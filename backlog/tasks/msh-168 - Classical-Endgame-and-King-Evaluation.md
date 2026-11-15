---
id: MSH-168
title: Classical Endgame and King Evaluation
status: Done
assignee:
  - '@codex'
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-15T21:56:00Z'
labels:
  - handle:classical-endgame-king-evaluation
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 172000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Endgame and King Evaluation.

**Problem:** The Chess evaluation is the pre-MSH-164 arithmetic, with no tuning of its weights and few endgame recognizers. In MSH-161 (Competitive Gameplay Diagnosis), the clearest difference from the opponent was the conversion of won endgames: mShogi gave back advantages about twice as often. Two other figures in the independent review are not evidence of a defect specific to mShogi: the opponent had almost the same share of level-endgame scores far from zero (48% against 44%) and of pawn pushes next to the own king. All of these games were played before MSH-164 (Shared Search Core) made the search much deeper, so the present weaknesses are not known.

**Objective:** A stronger engine through a better evaluation, with endgames and king safety as the expected main sources of gain.

**Approach:** Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md`. Start the next round with Texel tuning of the existing evaluation weights together with the two terms from the revised set: material draw scaling and endgame king activity. Use labeled quiet positions from engine self-play, with a recorded source, a license that permits the use, a recorded tuning method, and a separate held-out check. The MSH-161 games and the blunder suite are exposed development material; do not use them as the tuning or held-out set. Report the development blunder suite with its 47 endgame positions separately. Then run one sequential game test of the tuned evaluation against a frozen predecessor, with H1 = 10 Elo chosen before the test starts. No neural-network evaluation and no tablebase in this task. The evaluation is Chess-only code; the Tori evaluation is not in scope.

**Completion:** The accepted evaluation is the default and wins the game test against the engine at the start of the task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A baseline of the engine at the start of the task is recorded before evaluation changes: the blunder suite with its endgame positions reported separately
- [x] #2 Each evaluation change set has a recorded sequential game-test result, including “no decision” at a cap, and each change left out has a recorded reason
- [x] #3 The accepted evaluation is stronger than the engine at the start of the task in a game test
- [x] #4 The blunder suite result is not worse than at the start of the task, with the endgame positions reported separately
- [x] #5 Weight tuning, if used, records its position source, license, method and held-out check, and the tuned weights are confirmed by a game test
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
Next round: 1. Keep codex/msh-168-classical-evaluation in the existing chat worktree; freeze main f530662d (product source identical to task-start8ddfc1a0) before changes. Reuse recorded baseline100/146, endgame31/47. 2. Generate fresh native-authoritative self-play from seeded random legal openings, no exposed MSH161 positions: initially600 games,4096nodes/move, four workers,512ply/game and60s/game bounds,20minute generation cap. Keep only native concluded games and quiet positions, split by game before fitting, deduplicate board identities across splits. Record source/binary hashes, seed, outcome coverage, license and actual compute. 3. Jointly fit a bounded set of existing evaluation weights plus material draw scaling and endgame king activity using Texel squared sigmoid loss, fixed pawn anchor and exact Rust evaluation path. Preserve native rules, SEE values, phase, specialized ending guards. Fit training only; use a separate validation partition for stopping and inspect held-out once after freezing weights.10minute fitting cap; do not fit exposed blunders. 4. Run independent correctness/geometry/color tests, perft/differential, affected consumer checks, benchmark and blunders; record overall/endgame/opening results. Freeze one tuned candidate. 5. One sequential game test against frozen predecessor: H0=0,H1=10 chosen now, alpha=beta=.05,min30,max1200pairs,50ms/move,four workers,70minute cap. Stop at decision or cap; a cap is no decision. Record compute including discarded earlier rounds; no automatic additional candidates. 6. Independent review and retain only on accepted strength and non-worse blunders. Level1 local checks; Linux/Windows/GUI deferred to next Level3 checkpoint.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ticket revision, 2026-11-15 (requested by John): the problem statement now keeps only the MSH-161 difference that separates mShogi from its opponent (endgame give-backs). The criterion on level-endgame scores far from zero was removed, because the opponent had almost the same figure. A fresh baseline comes first, because the MSH-161 games predate MSH-164. Group-by-group game tests were replaced by coherent change sets, as in docs/m5/strength-roadmap.md. The tuning data rules (license, held-out set, no exposed material) are now explicit.

Owner Codex /root, sole builder in primary main checkout /Users/johnweathers/Development/Personal/mShogi. Startup main 8ddfc1a0 is clean and matches origin/main; no active task owner. Backlog1.55.3 matches baseline. Frozen predecessor build started before evaluation changes. Verification Level1: Chess-only classical evaluation, unchanged native rules/shared search/Tori evaluation. Local perft/differential, evaluation regressions, affected consumer tests, baseline and candidate blunders including47 endgames, bench, sequential predecessor games and independent review. Linux/Windows/packaged GUI checks deferred to the next Level3 checkpoint.

Baseline complete before evaluation changes: frozen8ddfc1a00c03, bench406067nodes/0.329s median; blunders100/146 avoided (endgame31/47,middlegame54/83,opening15/16),50 reference matches. This supersedes earlier engine observations for acceptance. Existing code already has passed-pawn runway, king escort, storms and specialized KPK/fortress endings. This round adds missing general draw scaling and king-activity/attack calibration as one coherent set; it does not repeat earlier isolated pawn-race tuning. Classical technique reference: Stockfish11 material.cpp and evaluate.cpp (pawnless material, opposite bishops and low-pawn scaling, king proximity and nonlinear king danger); implementation and heuristic constants fit the existing centipawn evaluator. No weight fitting or new tablebase is used.

Candidate checkpoint before games:104/146 blunders avoided versus100, including32/47 endgames versus31;53 reference matches versus50. Fixed-depth benchmark508702nodes/0.416s versus406067/0.329s: slower fixed-depth work, not a speed gain. No weight tuning used. Independent initial review found no blocking boundary/symmetry defect; added asymmetric-scale and material guard tests, preserved full scale for pawnless queen-vs-rook. Initial test failures: raw arithmetic expectations needed the independently calculated king-distance contribution; one new queen-vs-rook FEN illegally checked the nonmoving king and was corrected by moving that king. No production rule changed. Decision: freeze the coherent set and proceed to the predeclared sequential game test.

Checkpoint1: combined candidate939498e4 reached H0 at742pairs (+368 -370 =746), estimate-0.5 Elo, zero failed pairs. The better blunders did not establish stronger play. Authoritative results/compute: docs/m5/evidence/msh168/match/result.json. Decision: change direction by removing the added quadratic king-danger term, leaving the coherent material-scaling/king-activity endgame set. Independent reviewer identified that existing danger already scales with attacker count; the extra square may duplicate that reward and applies with a remote queen. This is a testable design hypothesis, not attribution from H0. No single-position tuning. One revised-set test: same frozen task-start predecessor,50ms,4workers,seed168,H0=0/H1=15,alpha=beta=.05,min30,max600pairs,40minute wall cap. Combined match wall will stay below about81minutes if this cap is used. Record all compute, including first failed-strength set. Stop and reconsider after this second checkpoint rather than repeat local removals without new evidence.

Revised endgame-set checkpoint:106/146 avoided versus100 baseline, endgames34/47 versus31, middlegame59/83 versus54, opening13/16 versus15; reference matches50 versus50. Overall and endgame improve, opening slice regresses and is disclosed. Bench506248nodes/0.427s versus406067/0.329s; no speed gain.76 search tests pass,1ignored. Independent reviewer verified first742 paired receipts and first H0 threshold, confirmed exact king-safety restoration and no new code finding. Proceed with the single predeclared revised-set game test.

Checkpoint2: unchanged revised endgame set9c67c98d reached600pairs with NO DECISION, not rejection: +331 -288 =581, estimate+12.5 (95% interval-1.6 to+26.5), LLR1.45; no failed pairs. Improved overall/endgame blunders and a positive game estimate justify extending this exact comparison rather than another local change. Independent reviewer approved conditional continuation with unchanged binaries/hypotheses/seed/all accumulated scores. Decision: extend to1000 total pairs,25minute additional wall cap, preserving600 result and recording prior/new compute separately. Total match wall remains below about100minutes (first40.15min + revised32.49min + extension25min); no several-hour campaign. Stop and reconsider at that cap if still undecided.

Final bounded checkpoint: revised set reached 1000 pairs with no decision (+535 -496 =969, estimate +6.78 Elo, 95% interval -3.96 to +17.52, LLR -0.360; zero failed pairs). This is not rejection, but does not satisfy strength acceptance. Stop this round; restore evaluator and tests exactly to task-start 8ddfc1a00c03. Both trial commits and receipts remain available. Total match wall 5624.520s (93.74min), CPU 21800.876s including cancellation; the continued comparison includes the first600 once. Evidence: docs/m5/evidence/msh168/README.md and verification.json. AC1 established; no accepted candidate, so task remains unfinished. Next action: reconsider the approach and record a new bounded plan; authorized Texel tuning is a possible direction after establishing a licensed quiet-position corpus and held-out set. No additional trial is started by this handoff. Branch correction: work and evidence preserved on codex/msh-168-classical-evaluation in /Users/johnweathers/.codex/worktrees/msh-168-evaluation/mShogi. Primary main restored to published 8ddfc1a0 before normal reviewed integration. AGENTS.md now explicitly requires feature branches and ownership-based temporary worktrees.

Independent final review by /root/evaluation_review passed: all1000 receipts, unchanged600 prefix, color pairing, source/binary identities, every sequential prefix and cumulative compute verified. Final evaluator and test bytes match predecessor Git objects; git diff 8ddfc1a0 -- crates tools Cargo.toml Cargo.lock is empty. Formatting and whitespace checks pass. Integrate only policy and evidence; task remains To Do for a revised approach. Feature history retains candidate939498e4 and revised9c67c98d; preservation checkpoint e9cfc0cf and policy c061718e. Integration target main. No claim of stronger play or completed acceptance.

Next-round ownership: Codex /root on existing codex/msh-168-classical-evaluation, /Users/johnweathers/.codex/worktrees/msh-168-evaluation/mShogi. User expressly directed reuse of this branch and Texel tuning of existing weights jointly with material draw scaling and endgame king activity. Initial generation and fitting bounds are20+10minutes; one match cap70minutes. Prior round remains separate evidence (93.74min match wall), not reused as training data. Self-play generation is newly produced project data with provenance and project BSD terms; no external games or exposed blunder positions enter fitting.

Corpus coverage checkpoint before any fitting or held-out loss inspection: first600 games all reached native outcomes,3979 quiet samples,39.00s wall/147.52s CPU. Expand once to6000 games total within the same20minute generation bound to give32 jointly fitted parameters broader material coverage. No outcome-based game selection. Keep the initial600 and all generation receipts; no further automatic expansion.

Texel candidate checkpoint:6000 fresh games,5999 native finishes,1unfinished excluded; independent replay728034plies and39270 raw quiet samples passed. Split30902train/3809validation/4052heldout positions, no game/family/board overlap.32 bounded grouped classical weights fit with fixedK=.95; validation selected second completed coarse sweep, patience stopped fitting. Heldout inspected once after freeze: MSE .0561861 to .0529987; all material-phase slices lower. Development blunders112/146 vs100, endgames38/47 vs31, middlegame60/83 vs54, opening14/16 vs15. Bench466994nodes/.4109s vs406067/.3290s: slower fixed-depth work.79 search tests pass,1ignored;29 consumers pass;clippy and formatting pass. Two code review defects (intermediate overflow and weighted bareking centrality cancellation) fixed before fitting. No fit adjustment after heldout/blunders. Proceed with the single predeclared H1=10 game test; evidence docs/m5/evidence/msh168/texel.

Single tuned-candidate game test complete: c3363d3d reached H1 at597pairs under predeclaredH0=0/H1=10,alpha=beta=.05,50ms/four workers/seed168. +424 -340 =430,zero failures,three cancelled. LLR2.96753 crossed2.94444; estimate+24.48Elo (95% interval+8.58 to+40.49). Match wall2022.210s/CPU7839.704s including19.460 cancelledCPU. Retain tuned default: overall blunders112/146 vs100 and endgames38/47 vs31. Earlier first set remainsH0 at742; revised untuned set remainsNO DECISION at1000, not rejection. Total all-round match wall7646.730s (127.45min),CPU29640.580s; generation/fitting separate. Final receipt audit pending. Production source and tests unchanged from testedc3363d3d. Durable raw corpus and frozen binaries byte-verified in primary target/strength/texel; manifests and receipts in docs/m5/evidence/msh168/texel.

Final independent review passed: /root/evaluation_review checked all597paired receipts,1194color-mapped outcomes, first eligible H1 crossing at597, frozen hashes and compute. Cancellation count/CPU are aggregate-only harness records; this limit is disclosed. Accepted tuned evaluator is ready for normal main integration. AC2 literal each-set decision wording conflicts with permitted capped no-decision outcomes and the user-directed next round; clarification requested before checkingAC2 or markingDone. Earlier revised set will not be calledH0. No additional match is needed for the accepted candidate.

Accepted implementation and full evidence integrated into main and pushed as a7357a60 (candidate c3363d3d, audited evidence c77d0253). Exact product/test source remains the tested candidate. MSH-168 remains In Review only for the pending user decision on AC2 wording; no code, game test or further tuning is pending. Existing worktree retained for this record clarification and closure. Next dependency-ready queue item is MSH-173 (Frozen Legacy Opponent); not started.

John approved the exact AC2 clarification on 2026-11-15: each set requires a recorded sequential game-test result, including no decision at a cap, with reasons for exclusions. AC3 still requires the accepted evaluation to win its game test. Applied through the Backlog CLI; AC2 and the remaining completion check are satisfied. MSH-168 is Done. No code or historical test result changed.

Completion record38343d5c is integrated and pushed to main; both checkouts are clean. Cleanup was attempted after integration, but the app archive tool refused: The task's primary checkout cannot be removed. The completed worktree is retained only because it is app-protected, not for ongoing implementation. Required raw corpus and frozen binaries were already preserved and byte-verified in the primary repository target/strength directory.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Joint Texel tuning of 32 bounded classical weights, including material draw scaling and endgame king activity, is the accepted default. A fresh 6,000-game self-play corpus has native outcome labels, recorded provenance and permitted use, and separate training, validation and held-out partitions. Independent native replay and split audits passed. Held-out MSE improved from 0.0561861 to 0.0529987.

The single H1=10 game test reached H1 after 597 pairs (+424 -340 =430), estimated +24.48 Elo at 50ms. Blunders avoided improved from 100 to 112/146; endgames from 31 to 38/47. The opening slice declined from 15 to 14/16. Verification: 79 search tests passed, one ignored; 29 consumer tests passed; tuner/data checks, Clippy, formatting and independent reviews passed. Fixed-depth execution is slower (0.329s to 0.411s), from more nodes and added evaluation work. Linux, Windows and GUI execution remain deferred to the next Level 3 checkpoint.

Integrated and pushed to main as a7357a60; tested candidate c3363d3d, audited evidence c77d0253. John approved the AC2 clarification on 2026-11-15. Earlier results remain H0 for the first set and no decision for the revised untuned set; the accepted tuned set passed H1. All acceptance criteria and completion checks are satisfied. Evidence: docs/m5/evidence/msh168/texel. Raw corpus and frozen binaries are preserved in the primary checkout under target/strength/texel. Next eligible task: MSH-173 (Frozen Legacy Opponent), not started.
<!-- SECTION:FINAL_SUMMARY:END -->
