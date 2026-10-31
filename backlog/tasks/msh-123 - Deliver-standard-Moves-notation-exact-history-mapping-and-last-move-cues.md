---
id: MSH-123
title: Deliver standard Moves notation exact history mapping and last-move cues
status: Ready
assignee: []
created_date: '2026-10-31T09:02:00Z'
updated_date: '2026-10-31T09:08:00Z'
labels:
  - desktop
  - owner-approved-ux
milestone: m-4
dependencies:
  - MSH-120
documentation:
  - docs/design.md
  - docs/desktop/post-r4-ux.md
  - docs/architecture-migration-plan.md
priority: medium
type: enhancement
ordinal: 123000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved post-MSH-119 product work (2026-10-31). The current sidebar exposes diagnostic event labels as Game history; players need recognizable moves and a stable cue for the last completed action. Own native-derived display notation, two-player move projection and exact move/event-prefix association plus completed-action rendering. R5 remains the selection/inspection/controller migration, not this product feature. This task consumes its completed controller/renderers and owns any minimum native presentation extension required for actual history and action geometry. Canonical actions/events/archives and selected profiles remain authoritative; this is not a notation-import/export or future-variant rules project. Layout/branding and lifecycle changes belong to the two companion UX tasks; R6 verifies their integration. Future work only: unassigned, no implementation plan or execution authorized by creation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Visible panel is Moves: numbered pairs of actual complete board actions occupy White/Black, Sente/Gote or Red/Black columns with clickable moves, accessible selected-position state, an explicit starting-position target, and correct empty/odd-length/root-second-player cases. Declared imported counters do not fabricate omitted moves; relative known-history numbering/provenance is explicit when appropriate.
- [ ] #2 Native-derived display text uses English Chess SAN, JSA-sourced standard Japanese Shogi notation and WXF2018 Article7-compatible Xiangqi notation as documented in post-r4-ux.md. Independently expected source-linked fixtures cover Chess castling/captures/en-passant/disambiguation/promotion/check/mate; Shogi same-square/promoted identity/drop/optional and compulsory promotion/mover-relative ambiguity including the JSA no-直 exception for 竜/馬; Xiangqi both-side files/directions/front-rear and stacked/multiple-file soldiers. Any source gap is resolved with a concrete fixture and documented interpretation; do not silently substitute USI/ICCS/debug labels or invent a standard.
- [ ] #3 Every move selects its exact authoritative post-action event prefix, with command/event boundaries retained separately from board ply. Relevant derived warnings/outcomes are attached to the actual move/position without extra move rows. Independently selectable non-move prefixes remain accessible through position/event detail or navigation; repeated identical boards with different warnings/claims remain distinct. No automatic collapse to a later same-ply prefix, no future history in analysis and no frontend legality/replay.
- [ ] #4 Native selected-history presentation supplies the most recent completed board-action geometry. Subtle origin and destination cues persist through engine thinking, polling, orientation and read-only historical navigation; destination is visually stronger. Drops show only destination; compound actions preserve ordered native route/complete identity including equal-endpoint routes, and special Chess actions use native geometry. Non-move events retain the preceding genuine cue or none at root and never fabricate movement.
- [ ] #5 Completed-action cues are distinct from actionable legal targets, partial selection, opponent inspection and keyboard focus, readable without relying only on color and non-actionable. Replacement/Load/Take back/Return to live update them from the exact current native prefix; stale snapshots/results and cancelled partial routes cannot invent a last move. R5 controller ownership stays singular; specialized Chess/Shogi/Xiangqi and separately labeled synthetic geometry remain appropriate.
- [ ] #6 Focused native/frontend regressions use independently expected notation and exact-prefix/geometry cases, including WXF action-plus-warning intra-command boundaries, same-board/different-history, historical analysis targets, paused/reopened and Take back lineage. Verify actual identified packaged all-three-game Moves/click/keyboard/orientation/thinking/history journeys at normal/minimum sizes and a separate synthetic compound/drop fixture where needed; normal distributables exclude instrumentation. Unit checks do not substitute for GUI evidence.
- [ ] #7 Independent review of notation source interpretation, exact mapping, cue geometry and interaction distinctions resolves material findings. Record actual revisions, package/environment/platform, commands, failures, reused evidence, applicable CI and normal integration/push. Preserve MSH-119 evidence and native rules/current document/execution/capacity guarantees; do not start later work automatically.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
