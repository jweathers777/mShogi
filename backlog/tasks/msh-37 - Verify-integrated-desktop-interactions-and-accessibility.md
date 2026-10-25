---
id: MSH-37
title: Verify integrated desktop interactions and accessibility
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T19:41:00Z'
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
- [x] #1 Integrated desktop evidence covers play, promotion, claims, analysis cancellation/replacement, save/reopen, historical view and the synthetic compound fixture.
- [x] #2 Keyboard navigation, focus recovery, labels/status announcements, readable layout and board orientation are checked through complete user journeys and documented limitations are actionable.
- [x] #3 An independent interaction/accessibility review is resolved, using separate test instrumentation when needed and checking that normal distributions omit it.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Exercise complete packaged macOS journeys with the actual bundled engine: play/promotion/claims, genuine engine failure/retry, analysis cancellation/replacement, canonical Save/Open/history/ReturnLive/Resume, unsaved replacement/quit and capacitypause. Use separately labeled verification fixtures for difficult states and exact native window-size controls; keep instrumentation out of normal artifacts. Independently assess keyboard focus, status semantics, contrast/readability and access at1180x820 and760x650 including synthetic nestedscrolling; fix material findings with focusedtests. Reuse unchanged semantic evidence, then record exactsource/packages/screenshots, independentreview, normalintegration/push andapplicableCI beforeDone.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Final M3 batch owner clarification: integrated journeys must include MSH-61 archive capacity admission, recovery/retry, explicit Return to live and Resume, unsaved-change handling, status announcements/focus recovery and access to controls at supported sizes; preserve existing AC. MSH-61 is required before integration verification.

Owner @codex codex/m3-desktop .claude/worktrees/next-task; startsafterMSH-61/MSH-36Done, all otherdependenciesDone. Actualthree-platformGUI remainsMSH-38, not inferred fromthislocalmacOS interactiontask orpackageCI. M3staysopen; nofutureM4–M6 implementation.

Implemented and independently reviewed focus recovery, native-position analysis announcements, neutral rejected-engine evidence, coordinate readability, and minimum-window controls. Actual macOS normal/QA journeys cover play, genuine failure/retry, cancellation/replacement, canonical Save/close/Open, exact history-prefix analysis, Return to live/Resume, unsaved replacement/native Quit/window close, promotion/claims, 256-to-257 capacity and synthetic routes at measured 760x650. GUI findings (WebKit disabled-button blur; sticky/collapsed inspector controls) fixed and retested. Final source 6eed007 integrated as fec83413c06ccb46127566bfcd256627fb677041. Focused evidence: 77 frontend, 46 native session feature, 5 native host feature tests and strict Clippy; independent source/GUI/evidence review has no remaining material findings. See docs/desktop/completion-batch-evidence.md and interactions-*.json/screenshots. Rust CI 40226454182 passed all three systems; package CI 40226454197 remains pending. Do not mark Done until that gate passes. MSH-38 still owns actual Windows/Linux GUI, and M3 remains open.

Final gate passed: Rust CI 40226454182 and desktop package CI 40226454197 both succeeded on Windows, macOS and Linux at fec83413c06ccb46127566bfcd256627fb677041. Actual GUI coverage remains explicitly local macOS, with exact earlier/final package identities preserved and unchanged semantic evidence reused. Final normal package host 29ec5b5c7fcd561e1069f21bdd3c2c0d4a0be88d28f29357a078f61e86561fe5; verification host 89c024ee8b9adeee37aed5eb65fd8a8b27418c2e19f703aa593e26a81eb49bf1; actual bundled engine 45d264831e806ea81b426b4fbbb94f2ecbcba384d23696a5c87537acca555ba5. Independent closure assessment verifies all twelve root screenshots, exact hashes and canonical archives; no remaining material finding. Screen-reader audio/formal conformance not claimed. Next MSH-38 owns actual three-platform installed GUI; M3 stays open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed integrated desktop interaction and accessibility verification. Fixed real GUI findings in keyboard focus recovery, native analysis announcements, rejected-engine wording, coordinate contrast, and minimum-window inspector access. Actual packaged macOS journeys cover play/promotion/claims, genuine engine failure/retry, read-only cancellation/replacement, complete Save/close/Open, history-prefix analysis, Return to live/Resume, unsaved guards, capacity preservation and synthetic routes at supported sizes. Independent review and native/package CI passed; source integrated into codex/rewrite at fec83413. Evidence and exact identities are in docs/desktop/completion-batch-evidence.md. MSH-38 actual Windows/Linux GUI and MSH-39 closure remain required; M3 open.
<!-- SECTION:FINAL_SUMMARY:END -->
