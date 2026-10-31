---
id: MSH-123
title: Deliver standard Moves notation exact history mapping and last-move cues
status: Done
assignee:
  - '@codex'
created_date: '2026-10-31T09:02:00Z'
updated_date: '2026-10-31T12:35:00Z'
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
- [x] #1 Visible panel is Moves: numbered pairs of actual complete board actions occupy White/Black, Sente/Gote or Red/Black columns with clickable moves, accessible selected-position state, an explicit starting-position target, and correct empty/odd-length/root-second-player cases. Declared imported counters do not fabricate omitted moves; relative known-history numbering/provenance is explicit when appropriate.
- [x] #2 Native-derived display text uses English Chess SAN, JSA-sourced standard Japanese Shogi notation and WXF2018 Article7-compatible Xiangqi notation as documented in post-r4-ux.md. Independently expected source-linked fixtures cover Chess castling/captures/en-passant/disambiguation/promotion/check/mate; Shogi same-square/promoted identity/drop/optional and compulsory promotion/mover-relative ambiguity including the JSA no-直 exception for 竜/馬; Xiangqi both-side files/directions/front-rear and stacked/multiple-file soldiers. Any source gap is resolved with a concrete fixture and documented interpretation; do not silently substitute USI/ICCS/debug labels or invent a standard.
- [x] #3 Every move selects its exact authoritative post-action event prefix, with command/event boundaries retained separately from board ply. Relevant derived warnings/outcomes are attached to the actual move/position without extra move rows. Independently selectable non-move prefixes remain accessible through position/event detail or navigation; repeated identical boards with different warnings/claims remain distinct. No automatic collapse to a later same-ply prefix, no future history in analysis and no frontend legality/replay.
- [x] #4 Native selected-history presentation supplies the most recent completed board-action geometry. Subtle origin and destination cues persist through engine thinking, polling, orientation and read-only historical navigation; destination is visually stronger. Drops show only destination; compound actions preserve ordered native route/complete identity including equal-endpoint routes, and special Chess actions use native geometry. Non-move events retain the preceding genuine cue or none at root and never fabricate movement.
- [x] #5 Completed-action cues are distinct from actionable legal targets, partial selection, opponent inspection and keyboard focus, readable without relying only on color and non-actionable. Replacement/Load/Take back/Return to live update them from the exact current native prefix; stale snapshots/results and cancelled partial routes cannot invent a last move. R5 controller ownership stays singular; specialized Chess/Shogi/Xiangqi and separately labeled synthetic geometry remain appropriate.
- [x] #6 Focused native/frontend regressions use independently expected notation and exact-prefix/geometry cases, including WXF action-plus-warning intra-command boundaries, same-board/different-history, historical analysis targets, paused/reopened and Take back lineage. Verify actual identified packaged all-three-game Moves/click/keyboard/orientation/thinking/history journeys at normal/minimum sizes and a separate synthetic compound/drop fixture where needed; normal distributables exclude instrumentation. Unit checks do not substitute for GUI evidence.
- [x] #7 Independent review of notation source interpretation, exact mapping, cue geometry and interaction distinctions resolves material findings. Record actual revisions, package/environment/platform, commands, failures, reused evidence, applicable CI and normal integration/push. Preserve MSH-119 evidence and native rules/current document/execution/capacity guarantees; do not start later work automatically.
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
1. Add a presentation-only native Moves projection over authoritative Chess record/match events, Shogi commands and Xiangqi exact event snapshots. Native adapters own standard notation, side/counter/provenance, post-action prefix versus command boundary, non-board detail association and selected completed-action/effect geometry. Keep current records, profiles, archive and execution semantics unchanged; intra-command Xiangqi views must remain exact, with unsupported committed execution explicitly unavailable rather than normalized.
2. Implement sourced SAN/JSA/WXF display adapters and independent expected fixtures. Resolve source interpretation through primary sources/concrete positions; record any genuine remaining material convention gap without silently inventing a standard. Avoid expensive repeated game replay on polling: derive/cache the projection at actual native-history changes using admitted positions/actions.
3. Replace NativeHistory contents with one shared two-column Moves panel, explicit root, exact native navigation and unobtrusive independently selectable event details. Feed native selected-prefix geometry through R5 readonly props; draw origin/stronger destination/special-effect/ordered-route cues in specialized renderers, separate from selection, inspection and focus. Do not change sidebar ordering, lifecycle flows or R5 ownership.
4. Add focused native and frontend mapping, notation, geometry, thinking/polling/orientation/history/Takeback/reopen/analysis/stale tests. Extend separately labeled GUI fixtures/driver as needed; actual packaged three-game normal/min pointer/keyboard/history/cue journeys, synthetic compound/drop, normal instrumentation exclusion. Independent notation/source/interaction/visual review, applicable CI, exact source/package evidence, normal integration/push; stop after123.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: root Codex owns MSH-123 in primary codex/rewrite at0b4218deaa8dbe705b882169d9e4cdab33333b0e, clean/fetched aligned0/0; no active InProgress/InReview worker displaced, unrelated worktrees preserved. Pinned Backlog1.55.1 resolves PATH1.55.0 mismatch; overview/execution read. MSH120 Done and seven checked criteria/evidence inspected;124/125/121 remain unstarted. Current native PresentedEvent has only index/board_ply/diagnostic label; NativeHistory renders all events, Board geometry props are readonly unused R5 handoff. Xq immutable event_prefix already differs from committed_prefix; native rejects analysis inside atomic-command prefixes. Source-grounded plan recorded before coding. Primary builder/integrator owns shared projection/consumer cutover; bounded independent notation researcher reads sources and does not edit product files or Backlog.

Owner decision 2026-10-31: after primary English/Chinese WXF Article7.5 research found literal two/three-stack coverage but no four/five or ambiguous multiple-file convention, John explicitly approved the documented ordinal+file display extension. Number soldiers from the mover's front within the native file, use mover-relative file number for stacks3–5 and otherwise ambiguous two-soldier stacks on multiple files. Example Red c5/c6,g5/g6: c6d6=17=6; g6f6=13=4. Document this as an owner-approved source-consistent display extension, not explicit WXF text; native rules/profile/actions unchanged. Research note /private/tmp/msh123-notation-research.md provides sources and concrete independent fixture strategy. No other product decision outstanding.

Source checkpoint: native SAN/JSA/WXF projection and exact command/action-prefix geometry implemented; 16 independent notation/projection cases plus three actual native session journeys pass. Full frontend264/23 and strictTS pass at current working source. Independent native/source review PASS and frontend106/eight-file review PASS after correcting Shogi completed/capture-corner similarity with actual double underline. Clippy redundant-index finding corrected and passed. Full native desktop-session feature suite running; packaged all3 normal/min GUI and CI remain required, task unfinished. docs/review/moves-123.md preserves source research and candidate-fixture/test-assumption failures; owner-approved rare WXF extension documented in post-r4-ux.md. No124/125/R6 work started.

Sourcefc7a6ae integrated/pushed; fullfront264/23, nativeclippy, source/notation106frontend independent checks and27actualmacOS screenshot visualreview PASS. Actualnormal/QAall3 normal/min, keyboardhistory, dropfailure/retry andnativecompoundroutes verified. InitialLinuxCI41199352841 revealednewdriver Flip/admission readinessrace (disabledroot), elevenfixturephasesPASS; normal3failed andmissingreopenarchive secondary. Verification-onlyfollowup waits observed changed/restoredorientation, exactprefix, enabledcontrols andcapacitycompletion before nextsingle realinput;78driverchecksPASS. Productsource unchanged. Finalcorrectedplatform GUI/CI remains pending, so noAC/DoDchecked andnotDone. Historicalfailures retained in review/evidence.

Final evidence at production fc7a6ae9e7499c40999c5a0396bc31e87d4e07e4 and verification-only integration 65975d8ea4f232883ceeb559be9b3cdb677c6e47: Desktop41200359930 and Rust41200359910 all3platformsPASS. ActualLinux15/15 andWindows15/15GUIphasesPASS; independenthash/manifest/nativeexit audit381Linux/342WindowsGUI filesPASS. Independentprimarysource/native review and106frontend checksPASS;42actualpackagecaptures personallyreviewed(30macOS+12Linux/Windows)PASS. Realall3normal/min Moves/pointer/keyboard/history/flip/root/live/Takeback, QAin-flightrealengine delay, nativeP*5edrop failure/retry anddistinctsyntheticreturnroutes verified;normalexcludesinstrumentation. Original198/199testassumption and firstplatformdriver failures preserved, correctedtest/driver and finalcampaigns supportcompletion. No productdiff fc→659; docs/review/moves-123.md and final-platform-audit.json mapall7AC. Nonmaterialminimumsidebarfocusloweredgeclip recordedforReady/unstarted124; nativecue/labels/focusoperationclear.124/125/R6remainunstarted;M4open. Noownedworktreecreated;ownedQAprocessesexited.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered a native-derived, two-player Moves panel: English Chess SAN, JSA Japanese Shogi notation and WXF Xiangqi notation with the explicitly approved ordinal+file extension. Moves select exact post-action event prefixes; separate Position events retain warnings, claims and terminal commands. Native selected-history geometry supplies origin/stronger destination, castling effects, destination-only drops and ordered compound routes. R5 retains the single interaction controller. Canonical rules, archives, execution identities and capacity guarantees are unchanged.

Verification: 264 frontend tests and strict TypeScript passed; focused native/controller tests, formatting and Clippy passed. Production source is fc7a6ae9e7499c40999c5a0396bc31e87d4e07e4. The verifier-only follow-up is 65975d8ea4f232883ceeb559be9b3cdb677c6e47; Rust 41200359910 and Desktop 41200359930 passed all three platforms there. Linux and Windows each passed 15 actual packaged GUI phases, with independent hash and native-cleanup audits. Independent notation/source/frontend reviews and 42 personally inspected package screenshots passed. Original failed test assumptions and verifier races remain recorded.

Evidence and acceptance mapping: docs/review/moves-123.md. Integration is codex/rewrite; final bookkeeping changes only task/design/evidence. No material MSH-123 finding remains. The minor minimum-sidebar focus-outline edge clipping is recorded for existing, unstarted MSH-124. MSH-124 and MSH-125 are next Ready work; R6 and M4 remain open. Stop after MSH-123.
<!-- SECTION:FINAL_SUMMARY:END -->
