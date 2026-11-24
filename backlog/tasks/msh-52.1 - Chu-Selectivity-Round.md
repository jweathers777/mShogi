---
id: MSH-52.1
title: Chu Selectivity Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T17:44:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies:
  - MSH-201
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 244000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve Chu search decisions with a bounded round of classical shared-core selectivity techniques. John started this step after MSH-201; preserve native Chu rules and use the accepted MSH-201 engine as the frozen predecessor. Evaluation tuning and the later strength measurement are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Quiescence checks, full SEE, game-scaled margins, move ordering and shared df-pn mate search each have a recorded measured outcome or a concrete reason they do not apply.
- [ ] #2 Kept changes pass native correctness and tactical checks and have supporting clocked predecessor evidence; inconclusive or rejected experiments are reported honestly.
- [ ] #3 Required workspace checks pass, useful design documentation is current, and run identities, settings, costs and cleanup evidence are recorded.
<!-- AC:END -->
