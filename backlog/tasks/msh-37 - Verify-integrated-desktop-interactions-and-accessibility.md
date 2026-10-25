---
id: MSH-37
title: Verify integrated desktop interactions and accessibility
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T19:00:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-33
  - MSH-34
  - MSH-35
  - MSH-36
  - MSH-61
documentation:
  - docs/desktop/m3-design.md
ordinal: 37000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Individual board tests do not establish the complete application interaction and accessibility experience.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Integrated desktop evidence covers play, promotion, claims, analysis cancellation/replacement, save/reopen, historical view and the synthetic compound fixture.
- [ ] #2 Keyboard navigation, focus recovery, labels/status announcements, readable layout and board orientation are checked through complete user journeys and documented limitations are actionable.
- [ ] #3 An independent interaction/accessibility review is resolved, using separate test instrumentation when needed and checking that normal distributions omit it.
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
Exercise complete packaged macOS journeys with the actual bundled engine: play/promotion/claims, genuine engine failure/retry, analysis cancellation/replacement, canonical Save/Open/history/ReturnLive/Resume, unsaved replacement/quit and capacitypause. Use separately labeled verification fixtures for difficult states and exact native window-size controls; keep instrumentation out of normal artifacts. Independently assess keyboard focus, status semantics, contrast/readability and access at1180x820 and760x650 including synthetic nestedscrolling; fix material findings with focusedtests. Reuse unchanged semantic evidence, then record exactsource/packages/screenshots, independentreview, normalintegration/push andapplicableCI beforeDone.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Final M3 batch owner clarification: integrated journeys must include MSH-61 archive capacity admission, recovery/retry, explicit Return to live and Resume, unsaved-change handling, status announcements/focus recovery and access to controls at supported sizes; preserve existing AC. MSH-61 is required before integration verification.

Owner @codex codex/m3-desktop .claude/worktrees/next-task; startsafterMSH-61/MSH-36Done, all otherdependenciesDone. Actualthree-platformGUI remainsMSH-38, not inferred fromthislocalmacOS interactiontask orpackageCI. M3staysopen; nofutureM4–M6 implementation.
<!-- SECTION:NOTES:END -->
