---
id: MSH-185
title: Chess Strength Anchor
status: Ready
assignee: []
created_date: '2026-11-16T20:50:00Z'
labels:
  - handle:chess-strength-anchor
milestone: m-5
dependencies: []
references:
  - https://www.computerchess.org/
  - https://github.com/official-stockfish/books
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chess-strength-assessment.md
  - docs/m5/chess-strength-assessment-report.md
  - docs/m5/strength-harness.md
priority: high
ordinal: 192000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Strength Anchor

## Problem
The last external Chess measurements came before the classical search round. MSH-139 (Chess Strength Assessment) estimated about 1900 on a conditional local CCRL-referenced index at 1 s per move (range 1700-2200). MSH-161 (Competitive Gameplay Diagnosis) scored 52.5% against Oxidation 0.6.0 (CCRL 40/15 1943, November 2024 snapshot) on prepared openings. After that, MSH-164 (Shared Search Core), MSH-166 (Search Selectivity Completion), MSH-167 (Engine Time Management) and MSH-168 (Classical Endgame and King Evaluation) gave large gains, but only against frozen predecessors. Self-play gains usually overstate the real gain. We do not know the present strength.

## Objective
One anchored estimate of present Chess strength against independent rated engines. The result tells John how much of the self-play gain is real and closes the Chess round before Shogi work. It is a measurement, not a qualification: no target is set and the result does not block Shogi work.

## Owner approval (John, 2026-11-16)
This rating study against external engines is approved within these bounds:
- Time control 60 s + 0.6 s Fischer increment for both engines. mShogi uses its own time manager (MSH-167); no fixed move time.
- About three independent opponent engine families, about 300-400 scored games in total including scouts.
- Hard compute cap: 6 hours of wall time for all match work, including scouts, failures and reruns. Stop at the cap and report what is complete.
Any larger sample, longer time control or different opponent class needs a new approval.

## Approach
- Freeze current main with `python3 -m tools.strength.cli build-engine` and record the revision and binary hash.
- Opponents: full-strength engines from one dated CCRL list snapshot. Prefer the CCRL Blitz list because it is nearest to this time control; record its URL and date and do not mix lists or 4-CPU rows. Use engines from different authors/families, official sources, native ARM64 builds, 1 thread, equal hash (for example 64 MB), ponder off, own book off. Do not weaken opponents.
- Selection: expect the engine to be clearly stronger than 1900 now. Run a short scout (about 4 pairs per candidate) against candidates that span roughly 2200-2900, then choose three opponents where mShogi scores about 25-75%. Count scout games in the budget; report them separately from the main sample.
- Openings: the CC0 Stockfish 8moves_v3 book, as in MSH-139, one fresh opening per pair with colors reversed. The pair is the statistical unit.
- At most 4 games in parallel on the M1 Pro Mac. Record OS, load and hardware.
- Reuse the existing native external UCI runner and replay path that MSH-139 used (`docs/m5/chess-strength-assessment.md`, `docs/m5/evidence/msh139/`) and the strength harness clock mode where it fits. A minimal extension is acceptable; do not build a new framework.
- Native rules decide every result. No evaluation adjudication. Process failures, time losses caused by the harness and technical caps stay unscored and are reported.
- Estimate: per-opponent score and conditional index with 95% intervals, and one combined estimate. If per-opponent estimates disagree by more than 150 points, report the disagreement and do not hide it in a pooled number.

## Completion
The report states the estimate with its uncertainty and conditions, compares it with MSH-139 and MSH-161, and updates docs/m5/strength-roadmap.md. An independent review checks the method and report. Verification level: measurement only, no production change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The frozen mShogi revision, binary hash, opponent engines (exact versions, sources, build hashes, options), CCRL snapshot URL and date, openings, time control, hardware and actual compute are recorded.
- [ ] #2 Games run at 60 s + 0.6 s for both engines within the approved 6-hour wall-time cap; every scored result is a native conclusion that passes native replay, and failures or unscored games are reported, not hidden.
- [ ] #3 The report gives per-opponent scores, a conditional CCRL-referenced index with a 95% interval, an agreement check between opponents, and a comparison with MSH-139 and MSH-161; it states that the index is not an official CCRL or human rating.
- [ ] #4 docs/m5/strength-roadmap.md records the result as the closing Chess checkpoint, and an independent review accepts the method and the report.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
