---
id: MSH-22
title: Add the first UCI chess engine adapter with a machine output contract
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T20:53:00Z'
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
P-03/A-06 and the M2 outcome require UCI as the first engine protocol, chosen by John on 2026-10-24. Keep a distinct machine output contract over the rules-owned game and isolated search. MSH-23 records independent external UCI-engine interoperability.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A distinct console mode implements the chosen protocol subset against documented compatibility evidence, separate from the human shell
- [ ] #2 Protocol stdout carries only protocol output, diagnostics go elsewhere, and process tests check this
- [ ] #3 The adapter uses the rules library and isolated search, maps actions without making client-provided effects authoritative, and documents unsupported protocol features honestly
- [ ] #4 The documented first protocol is UCI; process tests cover handshake/readiness, position history, legal bestmove, stop/quit and supported work/clock limits against the published UCI specification.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner decision 2026-10-24: UCI is the first protocol. External-engine interoperability is required within M2; MSH-23 records a match against an independent UCI engine.
<!-- SECTION:NOTES:END -->
