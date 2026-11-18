---
id: MSH-193
title: Shogi Blunder Screen
status: Done
assignee:
  - '@claude'
created_date: '2026-11-18T16:08:00Z'
updated_date: '2026-11-18T18:17:00Z'
labels:
  - handle:shogi-blunder-screen
milestone: m-5
dependencies:
  - MSH-191
priority: high
ordinal: 200000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Blunder Screen

## Problem
John accepted the classical Shogi engine (MSH-191 index about 2765) as the baseline for later learned-evaluation work (MSH-50 decision, 2026-11-18). Its weaknesses are not documented beyond the forced-mate suite, and it scored 0/24 against YaneuraOu/elmo 1000k (weak evidence). Before freezing, find whether a few mechanisms explain a large share of its clear blunders.

## Objective
A documented, mechanism-level account of the baseline's clear blunders in the 304 MSH-191 games, a reusable Shogi blunder suite, and, if the rule below triggers, one fix round for a general mechanism.

## Approach
1. Judge every position of the 304 MSH-191 games with YaneuraOu v4.79 + elmo WCSC27 (the admitted MSH-191 build) at 3,000,000 nodes, offline in the cloud; the judge is a measurement tool only, never a rules authority or dependency.
2. Rules fixed before results: win probability p = 1/(1+exp(-cp/600)) from the side to move; a clear blunder is an mShogi move with p_before >= 0.30 and p_after <= p_before - 0.30; mate scores map to 0/1.
3. Classify each blunder by mechanism (time trouble < 2 s; allowed short mate; refutation is a drop; refutation is a capture of an undefended or under-defended piece; King-safety collapse; no single refutation / drift); recheck each blunder position with mShogi at a longer time to separate search-limited from evaluation blind spots.
4. Save the blunder positions as a suite with the judge's move as reference (development data, held-out split by game).
5. Fix rule: only if one mechanism explains >= 30% of clear blunders and maps to a general search, evaluation or time change: one fix round with the usual predecessor game test. Otherwise document and stop.

## Completion
Screen results, mechanism shares, suite and decision recorded; caps: judge compute <= US$5, one fix round at most (game test as MSH-192, <= 3 h, US$6). Verification Level 1. Independent review.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Rules (judge, nodes, win-probability conversion, blunder threshold, mechanism classes, fix rule) are recorded before results.
- [x] #2 All 304 MSH-191 games are screened within the compute cap; blunder counts and mechanism shares are reported, with mShogi long-search rechecks.
- [x] #3 A Shogi blunder suite with judge reference moves and a by-game held-out split exists with recorded provenance.
- [x] #4 The fix rule outcome is recorded; any fix is general, passes Shogi correctness and consumer tests and wins a sequential predecessor test, or is rejected with its result.
- [x] #5 Results, compute, limits and independent review are recorded and docs are current.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-18 (@claude). Screen: 31,535 positions judged (1.38 h, US$2.23, cleanup verified); 226 clear blunders in 15,805 mShogi moves (1.43/100), 116 games; elmo games 1 blunder. Mechanisms: time trouble 28.3%, capture refutation 27.0%, drop refutation 23.5%, other 19.0%, short mate 1.8%, king zone 0.4%. 10 s recheck: 86 changed, 63 avoided (28%). Fix rule not triggered; no fix round. Suite tools/strength/data/shogi-blunders-v1.json (226; 164 dev / 62 held-out); baseline at 1 s avoids 73. Deviation recorded: implemented capture class counts any capture, King-safety proxy is 'refutation lands next to the King' (decision unaffected). Independent review accepted after wording/provenance fixes.

Integrated into main by merge 7c1508a0; pushed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-193 (Shogi Blunder Screen) judged all 31,535 positions of the 304 MSH-191 games with YaneuraOu v4.79 + elmo at 3M nodes (cloud, US$2.23). The baseline made 226 clear blunders in 15,805 moves (1.43 per 100), spread over time trouble (28%), capture refutations (27%) and drop refutations (24%); the elmo losses were gradual. A 10 s rerun avoids only 28%. The frozen fix rule (one mechanism >= 30%) was not triggered, so no engine change was made. The 226 positions form shogi-blunders-v1 (by-game held-out split); baseline avoids 73 at 1 s. Report docs/m5/shogi-blunder-screen.md; evidence docs/m5/evidence/msh193; roadmap and MSH-50 updated; independent review accepted. Integrated into main.
<!-- SECTION:FINAL_SUMMARY:END -->
