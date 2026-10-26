---
id: MSH-74
title: Deliver shogi desktop play analysis saving and historical inspection
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T21:02:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-73
  - MSH-70
  - MSH-68
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/desktop-session.md
  - docs/m4/shogi-desktop-product.md
type: task
ordinal: 74000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Native/controller and frontend checks plus actual packaged shogi GUI journeys/screenshots; distinguish local game-specific evidence from final three-platform cross-game gate.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The actual desktop can select standard shogi,play either human side against the bundled native engine,and present readable9x9 owner-oriented pieces,hands,turn,history and complete native choices.
- [ ] #2 Drops and optional/mandatory promotions work by click and keyboard with clear cancellation/focus. Declaration eligibility versus committed formal action,outcomes,resignation,recovery and failures have profile-correct meanings.
- [ ] #3 Bounded read-only analysis and live engine work remain separate; cancellation/replacement/profile switching/historical selection preserve exact context and never apply stale or wrong-purpose actions.
- [ ] #4 Complete Save/Open and history inspection preserve hands,profile,commands/events,conclusions and observation provenance. Reopen is stopped; Return to live/Resume and unsaved replacement/quit guards remain explicit.
- [ ] #5 Running native application evidence covers full shogi journeys at supported sizes and shogi capacity/defaults. Unsupported chess Full or timed restoration is rejected clearly,not silently relabeled or downgraded.
- [ ] #6 Product design shows the complete standard 9x9 board without scrolling at supported normal desktop window sizes; standard shogi does not inherit synthetic large-board scrolling or assume zoom/pan is needed.
- [ ] #7 Deliberate shogi-specific piece presentation preserves Sente/Gote orientation, ownership and promoted semantics; pieces-in-hand/drop and promotion choices form integrated player interaction rather than diagnostic button rows.
- [ ] #8 The player interface has coherent player, turn and game-status hierarchy with move/history and analysis presentation that does not crowd the board; polished keyboard and pointer operation is verified in the running native application.
- [ ] #9 Developer/raw diagnostic information is separated from normal play. The MSH-72 DEVELOPMENT CONTRACT harness is not evidence of approved/completed visual design and is not used as the MSH-74 design basis or acceptance target.
- [ ] #10 Explicit New Game selects Chess/Shogi and native human side; normal default orientation follows the human with Flip. Japanese pentagonal pieces use a cross-platform licensed font with provenance; native semantics remain independent of glyphs.
- [ ] #11 Actual normal packaged screenshots cover Sente/Gote, both hands, drop selection, optional promotion, history, active analysis, conclusion and minimum size; independent visual review resolves material findings and owner explicitly approves the polished surface before Done.
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
1. Implement the recorded normal product design in docs/m4/shogi-desktop-product.md; retain MSH72/73 harness as verification-only and preserve M3 chess behavior. 2. Extend normal native host game/session/file APIs over established controller/archive with typed New Game selection, both sides and authoritative status/choice data; focused native/host tests, no TS rules. 3. Build warm grid/pentagonal owner-oriented kanji pieces with bundled licensed Japanese font, compact native hands, contextual complete-action promotion, adaptive full-board fit and keyboard/focus; obtain actual packaged macOS visual evidence during implementation. 4. Integrate normal New Game/analysis/history/recovery/outcomes/SaveOpenResume/unsavedQuit with exact native identity protections, clean player-facing language and optional diagnostics; focused frontend/native/process regressions. 5. Capture complete normal packaged screenshot/journey set at1180x820 and760x650, independently review implementation and visuals, run native/desktop CI and Windows/Linux packaged GUI evidence, integrate/push codex/rewrite. 6. Present launchable macOS build/screenshots for owner visual review and leave task In Review until explicit approval; do not start75 or close M4/41–47.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification during MSH-72, 2026-10-26: MSH-74 must perform actual shogi UI design and implementation. Preserve existing semantic/play/analysis/archive/application criteria; added product criteria define full-board fit, deliberate owner-oriented/promoted shogi pieces, integrated hand/drop/promotion interaction, visual hierarchy and diagnostic separation. MSH-72 harness sizing/layout/Latin fallback/buttons/control strip are verification-only and not product approval. Future large boards must assess full-board fit, informed by owner-reported legacy master full16x16 display on smaller screens; zoom/pan/scroll are not assumed necessary. No MSH-74 implementation started. See docs/m4/desktop-session.md and docs/desktop/m3-design.md.

Owner request2026-10-26 recorded before substantial implementation: product UI plus implementation, not harness polish. Primary integrator Codex in clean primary checkout codex/rewrite at570ef0e2c931e6ecb07749d638294722c3e1198d, one worktree; dependencies73/70/68 Done. Backlog1.55.1. Explicit Chess/Shogi choice with native sides, human-bottom+Flip, warm uniform grid/pentagonal Japanese pieces with licensed bundled font, compact hands, complete choices/promotion, board-first side history/analysis, normal user language/optional diagnostics, full-board760x650+1180x820fit, no required zoom/pan for future12/16. M3 chess/73 saveability and stale-work contracts retained. Screenshot/review staging and owner visual checkpoint are required; this approval authorizes implementation but not final UI closure. Ordinary choices autonomous.
<!-- SECTION:NOTES:END -->
