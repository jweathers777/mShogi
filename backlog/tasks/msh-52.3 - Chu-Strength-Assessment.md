---
id: MSH-52.3
title: Chu Strength Assessment
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-24T21:07:00Z'
updated_date: '2026-11-24T21:13:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-52.2
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 247000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Measure the accepted Chu engine with internal time odds and a much deeper same-family blunder screen. John requested these next steps after the evaluation round; no external opponents or absolute rating claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A frozen internal scaling curve reports paired outcomes, uncertainty, search depths and all unfinished games.
- [ ] #2 A deeper-search screen reports coverage, clear blunders, classes and reusable positions with a held-out split; rules are fixed before results.
- [ ] #3 Any general fix follows the declared fix rule and predecessor test; checks, costs, run identities and cleanup are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze accepted product e2658c7 (binary83d00cff) from MSH-52.2. Reuse native runner and opening catalog; no engine change for measurement. Cloud c3-standard-4 only, at most US$10 and two hours of aggregate campaign wall time before a result/stop checkpoint.
2. Scaling: 2x versus1x clock at bases2.5,5,10 seconds, increment base/100, nominal allocation. Same seeds1..32 at each clock, both colors; two bins of16 pairs per clock, four concurrent pairs per VM. Separate same-clock control at10s+0.1s, seeds1..16, two bins of8 pairs. Fixed maximum, no SPRT early stop (min-pairs above maximum); existing stop-on-unscored per bin,1000 steps/game,600s pair watchdog,1500s/bin. Preserve and report every launched pair and failure; estimates conditional on fully scored pairs, no absolute Elo. One campaign, not selective reruns.
3. Deeper screen: use all control games, including unfinished prefixes. Select up to16 evenly spaced played moves per game by index before seeing scores. Search each selected position and its immediate successor with full native history, fresh table,5 million nodes, depth64; use native terminal results. Record player and judge depths/nodes to test whether the judge is materially deeper. Pilot runtime only may reduce the number of samples uniformly before looking at blunder results; never select on score. At most three30-minute judge campaigns within total cap.
4. Freeze rules: p=1/(1+exp(-cp/600)), native wins/losses1/0, native draws0.5, mate scores1/0. Clear blunder p_before>=0.30 and loss>=0.30; mistake p_before>=0.20 and loss>=0.15. Classes first match: time_trouble(clock<1000ms), repetition(exact native repetition key seen earlier), allowed_short_mate(opponent mate<=9 plies), missed_capture(judge captures but played does not), lion_refutation(opponent best move captures a Lion), compound_refutation(opponent best move captures multiple pieces), promotion_refutation(opponent best move promotes), other_capture(opponent best move captures), other. Classes describe symptoms; a class alone does not prove a mechanism.
5. Fix rule: at most one general mechanism accounting for>=30% of clear blunders warrants a fix round; diagnose native lines first. A fix needs its own predecessor SPRT and tactics; otherwise keep product unchanged and report limits. Retain reusable error positions and full-history references under GCS, split by opening SHA256 modulo4 to avoid cross-color leakage. No run output committed.
6. Finish checks and durable tool documentation, record result/cost/cleanup in task, integrate. Leave the final Chu baseline stop decision to John.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: branch codex/msh-52.3-chu-assessment; frozen scaling/control campaign msh213-msh523-scaling is running at gs://mshogi-experiments-216675633801/runs/msh-52.3/scaling. Added full-history sampled chu_judge tool and shared-rule Python classifier; two Rust and three Python tests pass, focused Clippy passes. Shortest-clock bins stopped after first four concurrent pairs: predecessor-side (1x clock) returned no completed iteration, sometimes zero nodes; keep these unscored. This is a time-control measurement limitation, not a native-rule failure. Next finish campaign, build judge in cloud, measure sampled full-history positions and diagnose classes.
<!-- SECTION:NOTES:END -->
