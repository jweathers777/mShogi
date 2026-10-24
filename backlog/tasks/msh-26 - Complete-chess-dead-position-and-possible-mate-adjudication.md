---
id: MSH-26
title: Complete chess dead-position and possible-mate adjudication
status: Ready
assignee: []
created_date: '2026-10-24T20:48:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-17
  - MSH-18
documentation:
  - docs/rules/chess-coverage.md
ordinal: 26000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John approved a sound incomplete M2 recognizer on 2026-10-24. Full dead-position detection and side-specific possible-mate adjudication remain chess-completeness obligations; unknown cannot justify a timeout/resignation result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Full dead-position and side-specific possible-mate behavior has primary-rule conformance evidence, including blocked positions outside the M2 material subset.
- [ ] #2 Timeout and resignation unknown cases are resolved only with justified evidence; ledger, profile/version migration and regression fixtures describe the remaining completeness honestly.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
