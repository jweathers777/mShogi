---
id: MSH-38
title: Verify installed M3 applications and bundled engines on three platforms
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-37
documentation:
  - docs/desktop/m3-design.md
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Package compilation is insufficient evidence that the installed application and engine work together for the complete desktop milestone.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Actual packaged application installation/launch and bundled-engine play are verified on Windows, macOS and Linux with exact source, package and environment identities.
- [ ] #2 Each platform exercises analysis/cancellation, outcomes/recovery, save/reopen/history and synthetic fixture interactions; failures are resolved or remain explicit blockers.
- [ ] #3 Normal package contents exclude instrumentation; durable screenshots/logs/native CI and reproducible launch/install instructions support all three platform claims.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
