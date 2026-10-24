---
id: MSH-22
title: Add the first chess engine protocol adapter with a machine output contract
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - protocol
  - console
milestone: m-2
dependencies:
  - MSH-21
documentation:
  - docs/design.md
  - README.md
priority: medium
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-03 and A-06 require standard engine adapters, and the M2 outcome includes the first one. The M1 shell is explicitly a human diagnostic session, and A-05 requires human and machine modes to have distinct output contracts, with diagnostics never corrupting protocol stdout. UCI is the likely first chess adapter; the choice is recorded in the task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A distinct console mode implements the chosen protocol subset against documented compatibility evidence, separate from the human shell
- [ ] #2 Protocol stdout carries only protocol output, diagnostics go elsewhere, and process tests check this
- [ ] #3 The adapter uses the rules library and isolated search, maps actions without making client-provided effects authoritative, and documents unsupported protocol features honestly
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
