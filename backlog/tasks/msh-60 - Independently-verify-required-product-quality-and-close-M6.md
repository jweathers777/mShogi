---
id: MSH-60
title: Independently verify required product quality and close M6
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-6
dependencies:
  - MSH-56
  - MSH-57
  - MSH-58
documentation:
  - docs/design.md
ordinal: 60000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M6 combines required product polish/distribution with optional research direction. This closure gate covers required product delivery only; MSH-59 is explicitly not a closure dependency. Optional experiments may remain To Do even after the required M6 product scope closes, so task-count percentage cannot be its verdict.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 M5 required delivery is closed through MSH-56, and six-game product polish/analysis/accessibility MSH-57 plus release/distribution MSH-58 satisfy their full criteria with resolved applicable D-10 decisions.
- [ ] #2 Independent review of the integrated product confirms promised six-game library/console/own-engine/desktop behavior, honest strength/coverage claims, and required native installation, analysis, accessibility, persistence/recovery and distribution evidence on all three supported platforms.
- [ ] #3 The durable M6 verdict identifies required scope, exact release/evidence revision, resolved findings and honest remaining limitations. Optional MSH-59 experiments are reported separately and may remain unstarted; neither their completion nor noncompletion substitutes for required product delivery.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
