---
id: MSH-158
title: Chess Decision Corrections
status: In Review
assignee:
  - '@codex-coordinator'
created_date: '2026-11-10T16:28:00Z'
updated_date: '2026-11-10T17:26:00Z'
labels:
  - handle:chess-decision-corrections
dependencies:
  - MSH-157
ordinal: 163000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Decision Corrections.

## Problem
The previous round did not improve broad decision quality. Three development failures expose unresolved quiet continuation and valuation issues.

## Objective
Correct general conventional search/evaluation causes of poor decisions, with independent outcome evidence.

## Approach
Trace actual interior searches for 0UOyM, 0m5mI and28pJK plus successful controls; test distinct focused hypotheses, retaining only supported general corrections. Reuse the existing160/80 benchmark and predecessor receipts. New active engine computation is capped at2400seconds; no matches or unrelated optimization.

## Completion
Independent review, relevant local correctness, frozen candidate comparison and honest retained/rejected findings; stop with concrete technical obstacles if reasonable experiments yield no supported correction.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Actual interior traces establish meaningful failure mechanisms or precise unresolved obstacles for the three development roots, with successful controls.
- [ ] #2 Focused general corrections are compared under controlled conditions; supported decisions justify any retained change, rejected experiments remain documented.
- [ ] #3 Authoritative native state/history/outcome/restoration/cancellation and relevant correctness regressions pass independent review.
- [ ] #4 Frozen candidate, if supported, receives existing development/heldout5s validation and fast checks; resources stay within2400new active seconds and conclusions distinguish exact outcomes, opinions and uncertainty.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read prior reports/search implementation; freeze production3b52314 predecessor and reuse valid MSH-157 candidate measurements. Keep heldout identities/results out of current experiment design.
2. Add temporary bounded actual-search observers for path admission, windows, TT, reductions/pruning, leaves and returned scores; compare5s/selected15s preserving histories.
3. Test a small number of distinct general search/evaluation hypotheses on three failures and wider development successes; reject unsupported changes.
4. Reserve1200s for full240candidate5s checkpoint,80s fast checks,120s independent changed-alternative analysis,1000s diagnostic/experimental searches; global2400 including failures. Freeze source/procedure before heldout, no retuning afterward.
5. Level1 targeted native/search/consumer correctness and independent review; integrate supported correction, preserve trace patches/results, no unrelated CI/GUI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Coordinator owns existing clean codex/chess-adversarial-assessment worktree, base17529cd / production3b52314. Backlog PATH1.55.0 read-only; mutations use pinned1.55.1. Current owner attachment supersedes prior stopping point and authorizes experimental search/evaluation changes. No new corpus or unrelated cost work.

Actual interior traces at5/15s completed for all3failures; focused endpoint traces added for2endings. Trace cap did not drop events, and depth4 traced/untraced decision/score/nodes/stats/PV equality passed for3roots. Queen5s Qc1 is generated/index26 atPVdepth6, reduced1ply, reducedscore173 fails belowalpha177 with no re-search; rootQc8 also reduced. PawnKh6 is actually searched unreduced and fails a narrow bound; bishop5s Kc3 is cut byKg7/returningBd6, while15s Bc1/Bd2 is searched but still values kingretreat/blockade poorly. Experiment1 noPV-LMR selectsQf6, finitepairedreference−368 vsoldQf4−438 stillfacesQc1: notadequatefix. Experiment2 advancedpassedpush extension selectsKc3 at5s butsame+1score andremainingplanweakness; all8successcontrols same. Experiment3 near-alpha reducedPVverification running; noheldoutaccess. Detailed raw records /tmp/msh158, sourcevariants frozen; final evidence packaging follows retained/rejected decision.

Five isolated trials completed. Only cached null-mate threat context is retained provisionally: actual narrow/full-window re-search lost an extension; same native key/depth/credits reuse preserves the effective horizon and selects reference-supported Qc8 at5s twice, with all8successful control choices preserved. NoPV-LMR and near-alpha verification instead select still-inferior Qf6. Passed-push extension selects Kc3 at tied+1 without demonstrated blockade forecast; pawn-race trial changes none of3failed roots: both rejected. Final source removes temporary observers and evaluator trial. Independent source review approves freeze; targeted native admission/history/claim/cancellation tests pass. Wider local regression and160development/80heldout frozen comparison pending. Level1 portable search heuristic only; no required CI jobs, broader Linux/Windows/GUI evidence deferred to significant integration/release. MSH-153 remains held.

Frozen source e93a750 completed160development5s,80heldout5s and40fast1s: only28pJK changes Qf4 toQc8; all239other primary and40fast choices unchanged. Independent preserved fullhistory reference pair supports Qc8 through Qc1 defense versus Rd2/Qa6 attack; finite opinion, not WDL. All48tablebase outcomes and3immediate-mate objectives preserved. 82Chesssearch tests pass,1manualprofile ignored; consoleclaim test passes. Complete audit:360native attempts and2newreference queries succeed,2894artifact hashes verified. Timedengine1568.80s; conservative registry charge1802.80/2400 including test/unusedreference reservations. No tuning afterfreeze. Report docs/m5/chess-decision-corrections.md and evidence/msh158; independent final outcome review pending. Both endgames unresolved; no broadstrengthclaim and MSH-153 remains held.
<!-- SECTION:NOTES:END -->
