---
id: MSH-192
title: Shogi King and Clock Round
status: Done
assignee:
  - '@claude'
created_date: '2026-11-18T11:04:00Z'
updated_date: '2026-11-18T16:08:00Z'
labels:
  - handle:shogi-king-and-clock-round
milestone: m-5
dependencies:
  - MSH-191
priority: high
ordinal: 199000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi King and Clock Round

## Problem
MSH-191 (Shogi Strength Anchor) measured mShogi at about 2765 on a floodgate-referenced scale against Gikou, but mShogi lost 0/24 to YaneuraOu + elmo at 1,000k nodes, which has the same floodgate rating as Gikou depth 7 (3/24). The evidence is weak (p about 0.06) but points to two concrete leads: the MSH-188 (Shogi Evaluation Round) fit gave extreme King-placement weights (back-rank corner about 456 below 8h; up to +400 for a King beyond the fourth rank), and in clocked games mShogi spends its base time early (Chess-style 30-move allocation) and then plays near its increment.

## Objective
Remove these two likely weaknesses with measured benefit: stronger play against the frozen predecessor, and a short recheck against both anchor families.

## Approach
Record baseline, intended benefit, comparison, caps and checkpoint before tests. Freeze current main as predecessor.
1. Bounded diagnosis of King placement and clock use in the MSH-191 games (both families).
2. Refit the Shogi evaluation with bounded, regularized King-placement weights on the MSH-188 corpus; report validation and held-out loss.
3. Shogi-appropriate clock allocation for USI clocked play (Shogi games are longer than Chess games).
4. Sequential game test vs the predecessor at the anchor control (60 s + 1 s) through the clocked USI route, MSH-190 openings.
5. Short anchor recheck against Gikou depth 5 and YaneuraOu/elmo 1000k (approved by John 2026-11-18, option 2); caps: at most 96 games, 2 hours, US$5.

## Completion
Each change retained with evidence or rejected with a game-test result; anchor recheck reported as a comparison with MSH-191, not a new rating. Verification Level 1. Independent review.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Baseline, intended benefit, comparison method, caps and checkpoint are recorded before tests; the predecessor is frozen.
- [x] #2 King-placement weights are bounded and refit with validation and held-out loss recorded; clock allocation for Shogi USI play is revised; each change has a recorded outcome.
- [x] #3 Retained changes pass Shogi perft/differential, evaluation identity and consumer tests, and win a sequential game test against the predecessor; with no retained change this is not applicable (John, 2026-11-18).
- [x] #4 A short anchor recheck against both MSH-191 families is run if an engine change is retained; with no retained change it is not run (John, 2026-11-18).
- [x] #5 Results, compute, failures and independent review are recorded and the Shogi docs are current.
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
Owner @claude, branch codex/msh-192-shogi-king-time. Predecessor: main ce2f79bd (target/strength/engines/ce2f79bda5c2/mshogi).
1. Diagnosis (done): MSH-191 games.
2. Clock: Shogi USI allocation uses an expected 50 remaining moves instead of the Chess-style 30.
3. King terms: bound king_rank*/king_file* weights to [-100, 100]; refit on the MSH-188 corpus (same tool, splits); keep only if validation loss is within noise and games do not regress.
4. Game test through the clocked USI route (shogi_anchor tool generalized: per-manifest clock, predecessor opponent, SPRT summary), 15 s + 0.25 s (same 60:1 base/increment ratio as the anchor control), MSH-190 openings, SPRT H0=0 H1=10, max 315 pairs.
5. Anchor recheck vs gikou-d5 and elmo-1000k at 60 s + 1 s (<= 96 games, 2 h, US$5).
6. Review, docs, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-18 diagnosis of MSH-191 games (bounded, descriptive). Clock: mShogi's remaining time at ply 30/60/90 (median): elmo losses 41.2/14.3/3.4 s; Gikou losses 44.3/18.3/3.7 s; Gikou wins 44.6/21.2/9.2 s; median per-move time 1.7-2.1 s; games median 82 (elmo) to 95-113 plies. King: share of positions with mShogi's King advanced >= 3 ranks 1.2-2.6%, corner (back-rank edge) 0.1-3.2%, on castle files 2/3/7/8 within one rank of home 42-58%; opponents similar. The extreme fitted King weights do not show up as King walks; the clock lead is supported.
Baseline/intended benefit/comparison/caps: predecessor ce2f79bd; intended >= 10 Elo at clocked play; SPRT H1=10 as above; cloud caps for the set test 3 h and US$6 conservative (c3-standard-4, 8 VMs, 2 games per VM); anchor recheck caps as in the description. Checkpoint: if no H1 by 315 pairs or H0, ablate (clock only vs King only) once, then decide.

2026-11-18 result (@claude). Set test (candidate de66ac9e vs predecessor ce2f79bd, clocked USI route, 15 s + 0.25 s, MSH-190 openings): 315 pairs, pentanomial [77,6,142,14,76], LLR -0.088, no decision, Elo +3.3 [-23.9, +30.5]; 0 failures; 14.00 VM-h, US$2.82, jobs verified absent. Decision: reject both changes (set showed no gain; individual effects not measured) and revert to predecessor code. Deviations, recorded: (a) the planned single ablation was deliberately not run (a short ablation would have fitted the cap; a null set gave little expected value); (b) the test control was 15 s + 0.25 s instead of AC #3's 60 s + 1 s; (c) the anchor recheck (AC #4) was not run because no engine change remains. AC #3 and #4 are left unchecked; closing the task needs John's decision to amend or waive them. Retained: anchor tool clock option, predecessor opponent, --sprt summary, MSH-192 prefix. Independent review (fresh-context subagent) accepted after wording fixes.

John 2026-11-18: waive the anchor recheck and the 60 s + 1 s criterion for a rejected set; ACs 3-4 amended to state this; task closed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-192 (Shogi King and Clock Round) tested Shogi USI clock allocation over 50 moves and King-placement weights bounded to +-100 (refit: validation 0.21815, held-out 0.21386) as one set against predecessor ce2f79bd: 315 pairs at 15 s + 0.25 s, no decision, about +3 Elo, 0 failures, US$2.82. Both changes rejected and reverted; engine identical to the predecessor. Diagnosis of MSH-191 games: no King walks or corner Kings; mShogi reaches the late middlegame with little time. Deviations recorded (ablation not run; 15 s + 0.25 s control; no anchor recheck); John waived the anchor recheck and the 60 s + 1 s criterion because no change was retained (2026-11-18). Retained tooling: anchor tool clocks, predecessor opponent, --sprt summary. Report docs/m5/shogi-king-clock-round.md; integrated into main (merge 0ea5e719). Independent review accepted.
<!-- SECTION:FINAL_SUMMARY:END -->
