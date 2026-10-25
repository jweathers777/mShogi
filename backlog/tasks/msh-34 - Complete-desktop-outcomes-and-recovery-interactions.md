---
id: MSH-34
title: Complete desktop outcomes and recovery interactions
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-32
documentation:
  - docs/desktop/m3-design.md
ordinal: 34000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Minimal play must grow into clear complete outcome and recovery handling without conflating chess results and engine failures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Claims, concluded chess outcomes, unresolved unscored match outcomes, resignation and engine failures have distinct user-facing states and commands respecting the core profile.
- [ ] #2 Recovery and cancellation interactions preserve authoritative history and permit safe explicit retry/new-game behavior; no evaluation-based result is invented.
- [ ] #3 Focused native and desktop interaction tests cover intended claims, unresolved outcomes, failure/retry and cancellation races; any timed-game addition first records and verifies suspension/resumption and event-order policy.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
