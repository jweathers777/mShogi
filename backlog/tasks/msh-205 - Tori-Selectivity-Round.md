---
id: MSH-205
title: Tori Selectivity Round
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T03:23:00Z'
labels:
  - handle:tori-selectivity-round
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-204
  - MSH-200
references:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 212000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Selectivity Round

**Problem.** Tori uses the shared core with default hooks; speed, selectivity and drop handling have not been tuned as for Shogi (MSH-190) and Xiangqi (MSH-195, MSH-203).

**Objective.** The selectivity step of the program for Tori, including search speed where the harness shows it pays.

**Approach.** Coherent set of shared-core techniques adapted to Tori (quiescence checks, SEE with drops, scaled margins, mate search, ordering) plus clear speed fixes from profiling; informed by MSH-200 (Tori Move Generation Deep Dive) where John has decided.

**Completion.** Each technique has a recorded outcome; kept changes pass correctness checks and the sequential predecessor test (H1 about 15).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each adopted or rejected technique and speed change has a recorded outcome.
- [ ] #2 Perft, native differential and history tests pass; tactical suite recorded.
- [ ] #3 The set passes the sequential predecessor test or the null result is reported and reverted; independent review recorded.
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
Baseline: main f8ee432 (MSH-200 engine), frozen as predecessor. Its binaries and worker image msh200-40edac60 are in the cloud.
Set (ported from the Shogi round MSH-190; Tori has the same drop-game structure):
1. MARGIN_UNIT 200: the smallest capture moves a Swallow (100) from board to hand.
2. Full SEE with board and hand values and compulsory promotion (Falcon->Eagle, Swallow->Goose), replaces the one-step exchange estimate.
3. Complete checking-move generator (direct checks of each face, discovered checks by ray pieces, drops), tested against the native rules.
4. Quiescence check stage (8) and root df-pn mate search.
5. Quiet ordering: direct checks first, drops near the enemy Phoenix.
6. In check, only drops that block a single ray check are searched.
Benefit: fewer tactical misses (Swallow and drop mates), better pruning scale.
Comparison: sequential pentanomial SPRT H0 0, H1 15, alpha=beta=0.05, max 600 pairs, at 5 s + 0.1 s on c3-standard-4 (tori_cloud.py --match); tactical suite at 20 ms.
Caps: US$10. Checkpoint: if the set is weak, test parts alone only with a specific reason.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20 (John): MSH-205 depends on MSH-200 (Tori Move Generation Deep Dive): move generation, especially drops, limits search speed, and selectivity results measured on the current generator could need repeating after an MSH-200 change.

Checkpoint 2026-11-21 (handoff): set committed 643f645 on branch claude/msh-205-tori-selectivity; Tori tests pass (17), new tests: check generator and in-check search generator against native (48 games), Falcon->Eagle promotion check, SEE values, df-pn drop mate and Swallow rule; mutation of discovered and promotion check paths fails the tests. Predecessor = MSH-200 build 40edac60 (gs://mshogi-experiments-216675633801/build). Candidate build store gs://mshogi-experiments-216675633801/build-643f6451. Next: derived worker image, tori_cloud --match and --suite, collect.

Checkpoint 2026-11-21: candidate build 643f645 (store gs://mshogi-experiments-216675633801/build-643f6451). Worker image worker@sha256:b3e2d72d (tag msh205-643f6451; base = msh200 Tori worker, frozen data digest af2d54e8 verified; candidate in /opt/engines, MSH-200 40edac60 in /opt/engines/baseline). Campaign msh205-match-1 submitted (c3-standard-4, 8 VMs). tori_cloud now accepts any msh<number> campaign name (70ea533). Next: collect match, then suite campaign msh205-suite-1 with its own store prefix.
<!-- SECTION:NOTES:END -->
