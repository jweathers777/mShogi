---
id: MSH-32
title: Deliver minimal untimed human-versus-mShogi desktop play
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-29
  - MSH-30
  - MSH-31
documentation:
  - docs/desktop/m3-design.md
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The first desktop slice must play real chess through the bundled engine while remaining responsive and protecting the live state.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A packaged running application supports new untimed standard chess games as either color, human moves and real bounded mShogi replies, automatic outcomes and explicit current/intended draw claims.
- [ ] #2 Actual final engine observations are displayed; cancellation, new game, retry and failures cannot apply obsolete work or fabricate a chess conclusion; unsupported modes fail clearly.
- [ ] #3 Focused integrated tests, actual desktop play/screenshots, independent review, normal rewrite integration/push and applicable native CI are recorded, along with honest launch instructions and remaining limitations.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
