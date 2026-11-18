---
id: MSH-197
title: Xiangqi Strength Anchor
status: To Do
assignee: []
created_date: '2026-11-18T19:25:00Z'
labels:
  - handle:xiangqi-strength-anchor
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-196
references:
  - docs/m5/strength-roadmap.md
ordinal: 204000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Strength Anchor

**Problem.** Predecessor tests show gains, not strength. Xiangqi needs one approximate external reference, as Chess (MSH-185) and Shogi (MSH-191) have.

**Objective.** The anchor step of the per-game classical strength program (docs/m5/strength-roadmap.md).

**Approach.** Use the opponent pool proposed in the harness task after John approves it with time and dollar caps. Run in the cloud with the clocked route. No public servers and no published games. If no credible rating reference exists, report a relative ladder and state the limit.

**Completion.** A recorded estimate or ladder with its uncertainty, conditions and compute. The status stays To Do until John approves the pool and caps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John's approval of the opponent pool, time control and caps is recorded before any game.
- [ ] #2 The estimate or ladder, its uncertainty, conditions, failures and compute are reported; all scored games replay natively; cloud cleanup is verified.
- [ ] #3 Independent review is recorded and findings are resolved.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
