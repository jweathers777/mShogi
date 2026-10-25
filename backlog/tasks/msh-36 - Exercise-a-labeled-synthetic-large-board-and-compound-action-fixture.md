---
id: MSH-36
title: Exercise a labeled synthetic large-board and compound-action fixture
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T18:37:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-31
documentation:
  - docs/desktop/m3-design.md
ordinal: 36000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop presentation must encounter large geometry and staged selection before variant implementation without claiming an unsupported real game.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A clearly labeled synthetic development fixture displays a readable 16-by-16 board and multi-stage route selection without claiming another real game.
- [ ] #2 Partial selection and cancellation never commit an action; completed native fixture choices remain distinct from chess sessions and records.
- [ ] #3 Pointer, keyboard, orientation and route ambiguity interactions are verified in the running app; normal distributables exclude test instrumentation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Generalize the existing board presentation and prefix selector using native-supplied geometry and complete opaque choices; adapt chess without another rules engine. Share the existing invented strider16 rules fixture between core regression and a feature-only native synthetic controller with distinct profile/record/version acceptance. Add isolated host/bootstrap and separate frontend build output; normal packages exclude fixtures. Verify same-origin/endpoint routes, ambiguous completion prefixes, cancellation, keyboard/orientation, native rejection and record replay; run actual packaged fixture/chess-promotion checks, independent review, normal integration/push and applicable CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex, codex/m3-desktop .claude/worktrees/next-task; starting after MSH-61 Done at integrateda52c2e8. Required actual shared Board/selection components, not a mock duplicate. Synthetic fixture is invented development infrastructure, never another real game or chess archive. Preserve all existing AC/mapping; no M4–M6 implementation.

Implemented sharedgeometry/orderedprefixselector plus exact native completechoices; sharedM1inventedhelper, separate featurecontroller/profile/ephemeralrecord, gatedhost and separate frontendoutput. Source review capacity_review clean after Escape-on-route-control focus gap fixed. Independent10core/4native/4host/14frontend focusedchecks pass; rootfullfrontend64pass. Normalbuild forcesflagoff/checks moduleinventory even inheritedflagtrue; verificationassets separate. ActualpackagedGUI still pending, so AC not yet checked.
<!-- SECTION:NOTES:END -->
