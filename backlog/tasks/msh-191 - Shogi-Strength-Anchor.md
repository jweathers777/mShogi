---
id: MSH-191
title: Shogi Strength Anchor
status: Ready
assignee: []
created_date: '2026-11-17T17:34:00Z'
updated_date: '2026-11-18T05:48:00Z'
labels:
  - handle:shogi-strength-anchor
milestone: m-5
dependencies:
  - MSH-188
references:
  - http://wdoor.c.u-tokyo.ac.jp/shogi/floodgate.html
documentation:
  - docs/m5/shogi-opponent-proposal.md
  - docs/m5/chess-strength-anchor.md
  - docs/m5/evidence/msh185/README.md
  - docs/m5/strength-roadmap.md
  - docs/experiments/parallel-runner.md
priority: high
ordinal: 198000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Strength Anchor

## Problem
After MSH-189 (Shogi Request Overhead), MSH-190 (Shogi Selectivity Round) and MSH-188 (Shogi Evaluation Round), Shogi gains are measured only against frozen predecessors. Self-play gains overstate real gains. The MSH-186 (Shogi Strength Harness) opponent proposal (docs/m5/shogi-opponent-proposal.md) gives a small mismatch check against handicapped YaneuraOu and Gikou settings; it is explicitly not a rating study, and Chess CCRL anchors are not a Shogi scale. Shogi has no CCRL-like list. The closest public reference is floodgate (http://wdoor.c.u-tokyo.ac.jp/shogi/floodgate.html), a computer-Shogi server that publishes ratings for the engines that play there.

## Objective
One approximate estimate of mShogi's Shogi strength on a floodgate-referenced scale, made like MSH-185 (Chess Strength Anchor). It is a measurement for John's information and for MSH-50 (Conventional Shogi); it sets no target. This task supersedes the "not an Elo anchor study" protocol of the MSH-186 proposal; reuse its admission checks, option manifests, rules settings and USI prerequisites.

## Owner approval (John, 2026-11-17)
Approved with this ticket, within the same caps as MSH-185:
- Run in the cloud on the existing Google Cloud Batch path, 8 workers, one game per VM, one machine type for all games.
- Hard caps: 6 hours of wall time and US$15 estimated compute for all match work, including bracketing, failures, reruns and smoke tests. Stop at the first cap reached and report what is complete. If the run is on course to exceed about 4.5 hours or US$10, find the cause and record it.
- About 300 scored games in total including bracketing; about three opponent families in the main phase.
- Do not connect mShogi to floodgate or any public server, and do not publish games. That needs a separate approval.
Any larger sample, longer control or different opponent class needs a new approval.

## Approach
- Freeze current main with `python3 -m tools.strength.cli build-engine`; record revision and hash.
- Reference engines: full-strength, fixed versions that have a published floodgate rating under a recorded player name, built from recorded sources (with evaluation files whose distribution and use provenance is recorded, as the MSH-186 proposal requires). Map each engine version to its floodgate player entry and rating, and record the page, date, number of floodgate games and rating uncertainty. Use engines from different authors; several settings of one engine are one family. Handicapped settings (depth limits, material-only evaluation) have no floodgate rating and are not anchors. Prefer engines whose floodgate rating is stable over many games.
- Expect mShogi to be much weaker than modern NNUE engines; include older or weaker rated engines so that the bracket can reach about 25-75% scores. If no rated engine near mShogi's strength can be admitted, stop, report the gap and propose options to John; do not invent anchors.
- Time control: 60 s + 1 s Fischer increment for both engines (measure game length first; Shogi games are longer than Chess games). Rules: native `mshogi.shogi.csa36.v1`, opponents configured for CSA 27-point entering king and a 320-move draw. One thread, equal hash (for example 256 MiB), no pondering, no own book. Native rules decide every result; clock exhaustion and process failures stay unscored and are reported.
- Openings: the MSH-190 opening set (at least 100 distinct openings), one fresh opening per pair with colors reversed.
- Method as MSH-185: record the prior, model and stopping rule before play; sequential bracketing (4 pairs per round, refit, next opponent nearest the estimate) until within about +/-100, then a main phase rotating about three families within about +/-150; one Bradley-Terry fit of all scored games with opponent ratings fixed at their floodgate values; per-family estimates with an agreement check (more than 150 points apart: report, do not hide).
- First extend and verify the existing USI route for full external games as the MSH-186 proposal requires (persistent per-game processes, clocks, declarations, 320-ply completion, cleanup). Do not create a second game authority.

## Completion
Report in docs/m5 with the estimate, its interval and its limits: floodgate ratings come from different hardware, time controls and opponent pools, so the number is a conditional floodgate-referenced index, not an official floodgate rating. Update docs/m5/strength-roadmap.md and MSH-50. Independent review of method and report.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The frozen mShogi revision and hash, each reference engine (version, source, build hash, evaluation-file provenance, options), its floodgate player entry, rating, game count and snapshot date, the openings, control, cloud machine type, CPU and actual compute are recorded.
- [ ] #2 The USI route for full external games is extended and verified before rated games; every scored result is a native conclusion that passes native replay, and failures, clock exhaustion and unscored games are reported.
- [ ] #3 Games run in the cloud within the approved 6-hour and US$15 caps; the prior, model and stopping rule are recorded before play, and the report shows each bracketing round and every opponent change.
- [ ] #4 The report gives one combined floodgate-referenced estimate from a joint fit of all scored games with a 95% interval, per-family estimates with an agreement check, and states the limits of the scale; if no suitable anchors exist near mShogi's strength, the gap and options are reported instead.
- [ ] #5 docs/m5/strength-roadmap.md and MSH-50 record the result, and an independent review accepts the method and report.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
From MSH-188 (Shogi Evaluation Round), 2026-11-18: the tuned Shogi evaluation has extreme King-placement weights (back-rank corner/Anaguma square about 456 below 8h; up to +400 for a King beyond the fourth rank) fitted on self-play outcomes. The predecessor game test cannot detect blind spots shared by both engines; check King handling and Anaguma games against external opponents here.
<!-- SECTION:NOTES:END -->
