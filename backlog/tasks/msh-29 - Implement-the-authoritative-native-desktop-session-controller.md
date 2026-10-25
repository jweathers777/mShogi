---
id: MSH-29
title: Implement the authoritative native desktop session controller
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
documentation:
  - docs/desktop/m3-design.md
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Concurrent webview commands and delayed engine results must not corrupt or reinterpret the live chess match.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A serialized Rust controller owns an untimed M2 ChessMatch, exposes native pieces/legal choices/history/claims/conclusions, and rejects stale or invalid human commands atomically.
- [ ] #2 Acceptance checks exact game context, request identity, session version and process generation; duplicates, superseded work and cancelled responses cannot apply actions.
- [ ] #3 Focused native tests cover new-game version protection, same-board different-history context, generation mismatch, cancellation races, legal moves/claims and unsupported mode rejection; independent review is recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
