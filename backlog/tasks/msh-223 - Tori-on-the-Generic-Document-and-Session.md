---
id: MSH-223
title: Tori on the Generic Document and Session
status: To Do
assignee: []
created_date: '2026-11-21T23:34:00Z'
labels:
  - desktop
  - refactor
  - needs-john
milestone: m-6
dependencies: []
priority: low
ordinal: 230000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: serve Tori with the generic document and session of MSH-217 (Shared Game Session), so that its copied document, file worker and session code go away. Blocker (John): the Tori saved payload has no selected_event field (deny_unknown_fields), so the generic document changes the saved bytes of every new Tori file, and old app versions cannot read new files. Measured in MSH-217: the Tori document differs from the generic one in 1,608 of 1,168 normalized lines and the session in 639 of 1,139 lines; the Tori core also needs the stop-aware calls and replay work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John decides the saved-format question (add selected_event to Tori files, or keep the Tori format with a per-game switch)
- [ ] #2 Tori uses ValidatedGameDocument and NativeSession; its old document, worker and session code are removed; line counts recorded
- [ ] #3 Committed Tori documents load and save as decided; desktop-session tests pass; one fresh-agent review
<!-- AC:END -->
