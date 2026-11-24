---
id: MSH-52.3
title: Chu Strength Assessment
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-24T21:07:00Z'
updated_date: '2026-11-24T21:30:00Z'
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

Frozen judge implementation source e36cf8393b87854334049cdafdfc292ca5aa1521; source archive uploaded to build-judge, awaiting scaling controller completion before starting the single controller build. Rust judge tests2/2 and Python assessment/Tori regression tests11/11 pass. Added opening-cluster bootstrap intervals (2000 draws, seed523) for sampled blunder rate and mean loss; no result-based sampling changes. Local workspace checks running. Judge orchestration uses32 shards/four processes per c3-standard-4 VM,16 evenly spaced moves/game,5M nodes before and after.

Before full judge results: rename the planned allowed_short_mate symptom to allowed_short_win. Chu WIN_SCORE denotes a native win, not specifically checkmate (royal capture and other native conclusions also use it); threshold remains nine plies, class order and all probability rules unchanged. This corrects the label, not a game rule or sample. Control bins finished:16 pairs launched,15 fully scored,31 native conclusions; all32 played prefixes remain eligible for the fixed512-move sample.

Scaling1 complete, all resources verified absent; conservative compute boundUS$0.43843. At bases2.5/5/10s (increment base/100),2x-clock scored WDL2/0/0,17/0/1,34/0/4 from1/9/19 pairs; launched8/12/20. Conditional estimates:2.5s unusable,5s+492 Elo (95% lower+215, upper unbounded),10s+372 (lower+226, upper unbounded). First-iteration interruptions caused7/3/1 unscored pairs. Late median depths2x/1x:4/3,5/4,6/5. Control16 pairs:15 scored11W/19L, one1000-step stop;31 native conclusions. Its equal-engine interval includes zero. Source and conditions unchanged.
Engineering extension fixed before new results: these clocks do not reach the harness depth8 target. Add20s+0.2s and40s+0.4s,2x time,16 pairs each, new seeds33..48, two8-pair bins/level, four workers,1000steps,900s pair watchdog,1400s/bin,30-minute campaign. This adds new longer controls, not reruns or score-based replacement of the first campaign. Keep all earlier results. Fixed control sample remains all32 games from the first campaign. At most two judge campaigns now; total actual campaign wall time remains capped at2h andUS$10. Judge cloud build is active frome36cf83.

Validation after origin/main merge (already current): fmt and all-target workspace Clippy pass;1188 workspace tests pass,12 ignored; three Chu judge tests and twelve Python assessment/Tori regression tests pass. Native report smoke test: four sampled move pairs parsed/replayed and judged at1000 nodes; smoke scores excluded from the full screen. Runtime judge source remains e36cf83; later Rust change adds only a terminal-perspective test. Longer-clock campaign will share its eight-VM allocation with the first16 judge shards (four game VMs plus four judge VMs); remaining16 judge shards run after it. Same samples/nodes, no score-based scheduling.
<!-- SECTION:NOTES:END -->
