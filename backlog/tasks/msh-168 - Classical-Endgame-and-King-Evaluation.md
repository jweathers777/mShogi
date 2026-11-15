---
id: MSH-168
title: Classical Endgame and King Evaluation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-15T18:47:00Z'
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

**Approach:** Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md`. First measure the present engine: the blunder suite (its 47 endgame positions separately) and, if useful, games from endgame starting positions. Use that baseline to choose the work. Candidate sources of gain: Texel tuning of the existing weights (authorized 2026-11-13); recognizers and scale factors for drawn or hard-to-win material; passed-pawn, king-distance and pawn-race terms; king-shelter and pawn-storm terms. Texel tuning needs labeled quiet positions with a recorded source, a license that permits the use, and a held-out check. Positions from self-play games of the engine are acceptable. The MSH-161 games and the blunder suite are exposed development material; do not use them as the tuning or held-out set. No neural-network evaluation and no tablebase in this task. The evaluation is Chess-only code; the Tori evaluation is not in scope.

**Completion:** The accepted evaluation is the default and wins the game test against the engine at the start of the task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A baseline of the engine at the start of the task is recorded before evaluation changes: the blunder suite with its endgame positions reported separately
- [ ] #2 Each evaluation change set has a game-test result with a sequential decision, and each change left out has a recorded reason
- [ ] #3 The accepted evaluation is stronger than the engine at the start of the task in a game test
- [ ] #4 The blunder suite result is not worse than at the start of the task, with the endgame positions reported separately
- [ ] #5 Weight tuning, if used, records its position source, license, method and held-out check, and the tuned weights are confirmed by a game test
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
1. Freeze main8ddfc1a0 before evaluation changes; record baseline bench and1000ms four-worker blunders, including47 endgames. 2. Use the baseline (100/146 avoided,31/47 endgames;50 reference matches) and code inspection to implement one coherent classical evaluation set: material/pawn-count and opposite-bishop draw scaling, endgame king distance to pawns, and coordinated queen-led king danger. Preserve specialized endings, native authority, current search and Tori evaluation. No Texel tuning, training set or new tablebase. 3. Add independent positive/negative material and king-geometry fixtures, color symmetry, and run Chess perft/differential/search plus affected consumer checks. Measure candidate bench/blunders without per-position tuning. 4. Freeze the candidate and run the existing paired sequential game test against8ddfc1a0 at50ms/move,4workers,seed168,H0=0/H1=15,alpha=beta=.05,min30,max1200pairs,75minute wall cap. Record actual compute and failures. At the decision/bound assess strength, overall/endgame blunders and cost; no small-screen rejection and no repeated local fixes without a new evidence-based decision. 5. Independent review and normal main integration only when all criteria pass. Level1 local verification; broader platform/GUI deferred to next Level3 checkpoint.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ticket revision, 2026-11-15 (requested by John): the problem statement now keeps only the MSH-161 difference that separates mShogi from its opponent (endgame give-backs). The criterion on level-endgame scores far from zero was removed, because the opponent had almost the same figure. A fresh baseline comes first, because the MSH-161 games predate MSH-164. Group-by-group game tests were replaced by coherent change sets, as in docs/m5/strength-roadmap.md. The tuning data rules (license, held-out set, no exposed material) are now explicit.

Owner Codex /root, sole builder in primary main checkout /Users/johnweathers/Development/Personal/mShogi. Startup main 8ddfc1a0 is clean and matches origin/main; no active task owner. Backlog1.55.3 matches baseline. Frozen predecessor build started before evaluation changes. Verification Level1: Chess-only classical evaluation, unchanged native rules/shared search/Tori evaluation. Local perft/differential, evaluation regressions, affected consumer tests, baseline and candidate blunders including47 endgames, bench, sequential predecessor games and independent review. Linux/Windows/packaged GUI checks deferred to the next Level3 checkpoint.

Baseline complete before evaluation changes: frozen8ddfc1a00c03, bench406067nodes/0.329s median; blunders100/146 avoided (endgame31/47,middlegame54/83,opening15/16),50 reference matches. This supersedes earlier engine observations for acceptance. Existing code already has passed-pawn runway, king escort, storms and specialized KPK/fortress endings. This round adds missing general draw scaling and king-activity/attack calibration as one coherent set; it does not repeat earlier isolated pawn-race tuning. Classical technique reference: Stockfish11 material.cpp and evaluate.cpp (pawnless material, opposite bishops and low-pawn scaling, king proximity and nonlinear king danger); implementation and heuristic constants fit the existing centipawn evaluator. No weight fitting or new tablebase is used.

Candidate checkpoint before games:104/146 blunders avoided versus100, including32/47 endgames versus31;53 reference matches versus50. Fixed-depth benchmark508702nodes/0.416s versus406067/0.329s: slower fixed-depth work, not a speed gain. No weight tuning used. Independent initial review found no blocking boundary/symmetry defect; added asymmetric-scale and material guard tests, preserved full scale for pawnless queen-vs-rook. Initial test failures: raw arithmetic expectations needed the independently calculated king-distance contribution; one new queen-vs-rook FEN illegally checked the nonmoving king and was corrected by moving that king. No production rule changed. Decision: freeze the coherent set and proceed to the predeclared sequential game test.
<!-- SECTION:NOTES:END -->
