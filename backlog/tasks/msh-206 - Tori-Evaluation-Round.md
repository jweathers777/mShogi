---
id: MSH-206
title: Tori Evaluation Round
status: Ready
assignee: []
created_date: '2026-11-19T19:43:00Z'
labels:
  - handle:tori-evaluation-round
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-205
references:
  - docs/m5/strength-roadmap.md
ordinal: 213000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Evaluation Round

**Problem.** The Tori evaluation is material plus placement with fixed values.

**Objective.** The evaluation step of the program for Tori.

**Approach.** As MSH-188 and MSH-196: linear terms with incremental static parts, fitted with the shared Texel tool on at least about 500,000 quiet self-play positions with a held-out split. MSH-196's lesson: test per-node terms separately from static terms.

**Completion.** A fitted evaluation that passes the sequential predecessor test, or a reported null result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A corpus of at least about 500,000 quiet positions with a split; held-out loss before and after recorded.
- [ ] #2 The kept evaluation passes the sequential predecessor test (H1 about 10) or the null result is reported and reverted; independent review recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
