---
id: MSH-191
title: Shogi Strength Anchor
status: Done
assignee:
  - '@claude'
created_date: '2026-11-17T17:34:00Z'
updated_date: '2026-11-18T09:51:00Z'
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
- [x] #1 The frozen mShogi revision and hash, each reference engine (version, source, build hash, evaluation-file provenance, options), its floodgate player entry, rating, game count and snapshot date, the openings, control, cloud machine type, CPU and actual compute are recorded.
- [x] #2 The USI route for full external games is extended and verified before rated games; every scored result is a native conclusion that passes native replay, and failures, clock exhaustion and unscored games are reported.
- [x] #3 Games run in the cloud within the approved 6-hour and US$15 caps; the prior, model and stopping rule are recorded before play, and the report shows each bracketing round and every opponent change.
- [x] #4 The report gives one combined floodgate-referenced estimate from a joint fit of all scored games with a 95% interval, per-family estimates with an agreement check, and states the limits of the scale; if no suitable anchors exist near mShogi's strength, the gap and options are reported instead.
- [x] #5 docs/m5/strength-roadmap.md and MSH-50 record the result, and an independent review accepts the method and report.
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
Owner @claude (Claude Code), branch codex/msh-191-shogi-anchor.
1. USI route: add Fischer/byoyomi clocks to mShogi USI; add a persistent clocked full-game mode to the native runner (shogi-match / shogi-match-replay) with native admission, unscored clock exhaustion and replay. Done in commits on this branch.
2. Anchors: floodgate rating snapshot 2026-11-18 (players-floodgate-20261118.html, archived). Reproducible open-source rated entries near mShogi: Gikou v2.0.2 DepthLimit 5/7 (gikou_depth5 2790, gikou_depth7 3130) and YaneuraOu v4.79 KPPT + elmo WCSC27 at 1,000k nodes (elmo_WCSC27_479_1000k 3131). komadokun_depth* ladder has no known source; Bonanza N100 builds and YSS are not open source; NNUE 1000k entries (3400+) are far above.
3. Build opponents for Linux x86-64 from pinned sources (gcc 13.3.0 image digest), data from official release assets with hashes; admission checks with scripted native fixtures and short games.
4. Cloud worker/collector for clocked games (one game per VM, 8 workers, c3-standard-4), method frozen before play (prior, logistic model with fixed floodgate ratings, pair-cluster interval, bracketing and main-phase rules).
5. Bracket, main phase, joint fit, family estimates and agreement check; report docs/m5, roadmap, MSH-50; independent review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
From MSH-188 (Shogi Evaluation Round), 2026-11-18: the tuned Shogi evaluation has extreme King-placement weights (back-rank corner/Anaguma square about 456 below 8h; up to +400 for a King beyond the fourth rank) fitted on self-play outcomes. The predecessor game test cannot detect blind spots shared by both engines; check King handling and Anaguma games against external opponents here.

2026-11-18 local calibration scout (development only, not rated games; mShogi native on M1, Gikou v2.0.2 Linux build emulated in Docker, 60 s + 1 s, two openings x two colors): Gikou DepthLimit 1: mShogi 4-0; DepthLimit 3: 3-1; DepthLimit 5: 3-1; DepthLimit 7: 1-3. Rough level between gikou_depth5 (2790) and gikou_depth7 (3130). Reports in target/msh191/scout.

2026-11-18 campaign progress (@claude). Bracket b01-b08 (64 games, no failures): estimate 2794.63, half-width 88.68 at 0.66 h, US$1.00. Main m01 (24 pairs): gikou-d5 cumulative 40-40, elmo-1000k 0-24 (all checkmates, 62-119 plies, no clock exhaustion; mShogi plays near its 1 s increment after spending its base). Joint 2771.87, half-width 71.43; family spread 395 > 150 (reported, not hidden). Decision before m02: stop scheduling elmo-1000k (0% gives no further information; its games stay in the fit); m02 alternates gikou-d5 (2790) and gikou-d7 (3130, same rating as elmo-1000k) to test anchor consistency at equal rating. Total 0.96 h, US$1.47.

m02: gikou-d5 cumulative 52-52, gikou-d7 3-21 (12.5%; model expects 11.8% at 2777) - Gikou family internally consistent; elmo-1000k (same 3131 rating) 0-24. Joint 2776.75, hw 58.61, 160 games, 1.22 h, US$1.84. Decision before m03: gikou-d7 leaves 25-75% -> replaced by nearer Gikou settings; m03-m05 alternate gikou-d5 and gikou-d3 to about 300 games.

2026-11-18 final (@claude). Campaign complete at the ~300-game rule: 304 scored games, 152 complete pairs, 0 failures/unscored; 13 batches (8 bracket, 5 main), 1.99 campaign hours, 15.10 conservative VM-h, US$3.04; all jobs verified absent; c3-standard-4 Intel Sapphire Rapids, us-central1-a/b. Joint 2765.24 (approx 95% 2723.04-2807.44); Gikou family 2772.35; elmo-1000k 0/24 (prior-dominated 2398; spread 374 flagged, weakly supported p=0.063). Prior sensitivity 2766.0/2765.2/2765.1. Independent review (fresh-context subagent): accepted after fixes - deviations labelled (elmo dropped after m01; gikou-d7 used outside window), family-check wording tempered, anchor SE and +-25 sensitivity added, watchdog stop now TimeLimit (code fix after the study; no study game affected). Checks: workspace clippy clean; 862 consumer tests (14 ignored); runner tests pass after the fix; Python tools 138 OK.

Integrated into main by merge 0ba20a87 (branch codex/msh-191-shogi-anchor); pushed to origin/main.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-191 (Shogi Strength Anchor) gives one conditional floodgate-referenced estimate of mShogi: 2765 (approx. 95% 2723-2807) at 60 s + 1 s, from 304 scored games (152 pairs, 141 W / 0 D / 163 L) against Gikou v2.0.2 DepthLimit 3/5/7 (floodgate 2637/2790/3130) and YaneuraOu v4.79 + elmo WCSC27 at 1,000k nodes (3131), all built from pinned sources with official data files. Prerequisites added: Fischer/byoyomi clocks in mShogi USI and a persistent clocked full-game USI route in the native runner (shogi-match / shogi-match-replay) with native admission, unscored clock exhaustion and ledger replay. Every result is a native conclusion that passed native replay; no failures. Cloud: 13 batches, 1.99 h, US$3.04 conservative, within the 6 h / US$15 caps; no public servers. Family check flagged (elmo 0/24, spread 374) but weakly supported; elmo was dropped after m01, a labelled deviation moving the estimate toward Gikou (m01-through fit 2771.9). Anchor rating error (>= ~22 per Gikou entry) is about as large as the statistical half-width. Report: docs/m5/shogi-strength-anchor.md; evidence docs/m5/evidence/msh191; roadmap and MSH-50 updated; independent review accepted. Integrated into main.
<!-- SECTION:FINAL_SUMMARY:END -->
