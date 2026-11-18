---
id: MSH-196
title: Xiangqi Evaluation Round
status: Ready
assignee: []
created_date: '2026-11-18T19:25:00Z'
labels:
  - handle:xiangqi-evaluation-round
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-195
references:
  - docs/m5/strength-roadmap.md
ordinal: 203000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Evaluation Round

**Problem.** The Xiangqi evaluation has only material, a placement table and a tempo bonus. Mobility, General safety (Advisors, Elephants, open files) and phase-dependent piece values (Cannon, Horse) are absent, and no weight is fitted.

**Objective.** The evaluation step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Xiangqi.

**Approach.** As in MSH-188 (Shogi Evaluation Round): a linear term set with incremental static terms, fitted with the shared Texel tool on at least about 500,000 quiet self-play positions generated in the cloud, with a held-out split.

**Completion.** A fitted evaluation that passes the sequential predecessor test, or a reported null result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A self-play corpus of at least about 500,000 quiet positions is generated with recorded compute and a split by game or opening.
- [ ] #2 The fitted weights lower the held-out loss against the initial weights; both losses are recorded.
- [ ] #3 Correctness tests and the tactical suite pass; the set passes the sequential predecessor test (H1 about 10 Elo), or the null result is reported and reverted.
- [ ] #4 Independent review is recorded and findings are resolved.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
