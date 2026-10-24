---
id: MSH-25
title: Verify integrated M2 search, UCI matches and close the milestone
status: Ready
assignee: []
created_date: '2026-10-24T20:48:00Z'
labels: []
milestone: m-2
dependencies:
  - MSH-15
  - MSH-16
  - MSH-17
  - MSH-18
  - MSH-19
  - MSH-20
  - MSH-21
  - MSH-22
  - MSH-23
  - MSH-24
documentation:
  - docs/design.md
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M2 component checks must be connected by evidence from the actual game/search/cancellation/protocol/runner path before declaring the baseline integrated. Reuse M1 evidence for unchanged boundaries; this is a focused M2 closure, not another M1 audit.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Actual rules-owned game, history-sensitive search/adjudication, cancellation/stale-result rejection, UCI process and runner integration are exercised together with a recorded independent-engine match.
- [ ] #2 Canonical games plus match commands, claim policy and clock/failure events replay deterministically; documented work-limited reruns and timed-run observations meet their separate promises.
- [ ] #3 Independent review resolves material integration findings and maps required M2 deliverables to evidence and honest limitations, including the named M4 chess-completeness obligation.
- [ ] #4 The integrated revision has appropriate native Linux/macOS/Windows build/test/process evidence; platform omissions remain explicit and block unsupported closure claims.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
