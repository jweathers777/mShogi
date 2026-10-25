---
id: MSH-28
title: Establish the Tauri 2 desktop application and packaging foundation
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
documentation:
  - docs/desktop/m3-design.md
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop slice needs a reproducible native application using the accepted stack and a bundled engine without exposing shell execution to the webview.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tauri 2, React, strict TypeScript, Vite and SVG application sources are reproducibly locked and documented, with development and packaged launch commands.
- [ ] #2 Normal native packages bundle the mShogi engine and web assets; test drivers and fixture executables are excluded, and frontend shell access is absent.
- [ ] #3 Applicable native CI builds packages and exercises the packaged engine on Windows, macOS and Linux; local running desktop screenshots and package evidence are recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
