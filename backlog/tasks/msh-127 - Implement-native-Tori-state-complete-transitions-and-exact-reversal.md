---
id: MSH-127
title: Implement native Tori state complete transitions and exact reversal
status: Done
assignee:
  - '@codex'
created_date: '2026-11-01T12:21:00Z'
updated_date: '2026-11-01T16:25:00Z'
labels:
  - m4
  - owner-msh-85
milestone: m-4
dependencies:
  - MSH-126
references:
  - >-
    backlog/tasks/msh-85 -
    Deliver-the-authoritative-tori-shogi-native-handoff-for-shared-consumers.md
documentation:
  - docs/m4/tori/profile.md
  - docs/m4/tori/fixtures.json
  - docs/m4/tori/fixtures.md
  - docs/m4/tori/native-plan.md
  - docs/m4/tori/native-fixture-ownership.json
  - docs/testing/conformance-policy.md
  - docs/m4/tori/native-structural-evidence.md
type: task
ordinal: 127000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-81 is Done at6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc. Selected profile mshogi.tori.modern.v1 and all103 independently expected fixture specifications are fixed inputs; do not reopen historical acquisition or import Standard Shogi defaults. This is a component owned by coordinating MSH-85 under complete game delivery MSH-42, following the repository flat component-ID precedent used for MSH-84/91-94. Own the game-specific foundation in mshogi-core, using current Shogi/Xiangqi separation as evidence rather than copying their rules. Deliver useful structural state/actions/derived effects and reversal; C establishes full local legality and D supplies full game/history/record admission. Structural helpers must not become an alternative public complete-legality boundary.

Verification: focused reproducible commands at exact tested source, independent bounded review, normal codex/rewrite integration/push and relevant native/shared regression/CI for actually changed code. Distinguish passed/failed/not-run and reference observations from independently expected facts. No unrelated GUI campaign, playing-strength claim, new exact-proof requirement or future-variant implementation. Future worker records an implementation plan only on activation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Game-owned Tori Side/Square/Piece/Hand/Position/root primitives explicitly represent7x7 coordinates, all9 native faces, distinct LeftQuail/RightQuail and Sente/Gote, selected immutable profile identity/edition and standard32-piece setup. Structural diagram validation enforces unique/domain squares, royal/base inventory and hand types/counts; reduced custom material, root provenance and immutable declared counter are distinct from accepted plies and proven historical reachability. Final royal/drop root admission remains C.
- [x] #2 Complete typed move/drop actions and native derived effects cover every movement table, rays/blockers, jumps without intermediate capture, quail orientation, captures/demotion/owner changes, hand resources and forced promotion. Promotion depends on origin OR destination in the farthest2ranks, never on drops or a hidden timer; subsequent zone departure/within movement after a drop promotes. Impossible optional faces, promoted drops, invented victims/routes and basic malformed/resource/ownership requests reject atomically.
- [x] #3 Meaningful structural geometry/transition/root facets of all assigned fixtures execute independently expected facts for both owners, every face, blockers/leaps, quail capture/drop identity, Eagle/Goose demotion, drop hand deltas and compulsory promotion including TORI-P-DROP-EXIT. The42 accepted transition specifications supply exact before/after assertions where applicable; full legal membership, mate/drop-mate and terminal commands are not claimed here.
- [x] #4 Apply/undo/redo restores board, hands, face, side, immutable root metadata, accepted ply count and incremental caches exactly across accepted captures, drops and promotions. Wrong/stale/out-of-order undo rejects without mutation; deterministic multi-action sequences check restoration. Counter/resource overflow is explicit and cannot masquerade as zero legal choices or a game conclusion.
- [x] #5 Intermediate transition/candidate helpers remain private or unambiguously limited and inaccessible as complete game admission. No Tori engine capability, desktop chooser, session, canonical-record handoff or claim of complete legal-action conformance is introduced before its owning slice; native Rust remains independent of UI/Python/console.
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
1. Root Codex is sole builder/integrator in clean primary codex/rewrite62825dd; pinned Backlog1.55.1, Done126 prerequisite and no active task owner verified. Preserve unrelated worktrees and all selected profile/103 corpus/ownership bytes.
2. Add game-owned tori Rust primitives, immutable structural root/provenance/counters and validated standard/custom/imported position. Implement native movement geometry and complete move/drop effect resolution; keep structural application/candidates crate-private and explicitly incomplete (royal safety/drop restrictions remain128).
3. Add atomic checked transitions with compulsory origin-or-destination-zone promotion, capture/demotion/handed identities and exact logical snapshot undo/redo with non-rewound token lineage; explicit overflow rejection. No native dispatch, record, engine, console or desktop capability.
4. Connect real core unit-test adapters to mshogi-conformance for all127 primary and applicable supplemental assertions; run independent expected geometry/effect/root fixtures and42 accepted transitions, with honest pending128/129/consumer accounting. Add deterministic capture/drop/promotion sequences, rejection/material/reversal and native rotation properties using shared helpers.
5. Obtain separate independent source/test review, resolve findings, run focused core/conformance plus relevant native regressions, format/lint and three-platform native CI. Record exact source/results and handoff, finalize127only via CLI after supported acceptance, integrate/push and stop before128.

Verification clarification: the original42 accepted specifications comprise40 board transitions plus2 resignation cases owned129. Execute all40 structural transitions and preserve the2 terminal-command cases as pending; no expectation, AC or dependency change.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

Owner request2026-11-01 authorizes127only. Primary checkout clean62825dd, origin aligned;126Done,127Ready/unassigned before activation; no InProgress/InReview other owner. Existing unrelated111/100/98 worktrees preserved. Native structural facts are not full legal admission; selected profile, independently authored fixtures and engine-independent policy remain authoritative. No external-engine/GUI campaign required.

Implemented native tori primitives/root metadata and private structural geometry/apply/undo/redo; no game dispatch/capability/UI. Real126 adapter executes202 assertions assigned127,202PASS/525pending/0failed;83consumer obligations pending. All40 accepted board-transition fixtures pass exact effects/undo/redo;2 remaining accepted specs are resignation129. Initial12-test suite had11PASS/1failure because test counted42 board transitions; corrected independently verified partition without fixture changes. Initial adapter royal-error label differed from independently expected royal_inventory; adapter spelling corrected, native behavior/fixture unchanged. Initial test compile failed on private token-field assertion, removed in favor of behavior checks. Independent tori127_review PASS after strengthening14 negative structural fixtures to assert exact rejection/nonmutation; earlier conditional-only checks were insufficient. Reviewer independently12focusedPASS,56corelibPASS,1compilefailPASS,clippyPASS; source hashes and full evidence recorded in native-structural-evidence.md. All profile/corpus/manifest bytes unchanged, external engines and GUI not invoked. Platform nativeCI/integration pending; task remainsInReview.

Reviewedsource integrated/pushed ce1801d7801f12e447b625739acd6a0c53aa2640; nativeRustCI41324873187 dispatched. Full local core/conformance command247passed/0failed/2existing opt-in deepChess tests ignored (deep_independent_divides_match_in_order, deep_fixed_seed_sequences); no Tori test ignored. Finalreview12Tori+1compilefailPASS, lint/formatPASS; normal cargo tree confirms harness is dev-only. Nativefixtures202PASS/525pending with all127fields executed; report and sourcehashes in evidence/native-structural-fixtures.json. Existing rules/consumers/workflows and all129task344edgedependencies/oldDone records/profilebytes unchanged. First source docs misstated ignored count0; corrected to actual2 in completion evidence. Await actualCI; no later task activated.

Final native Rust workspace CI41324873187 PASS at exact integrated/pushed source ce1801d7801f12e447b625739acd6a0c53aa2640 on Ubuntu24.04/macOS15/Windows2022, including format/lint, workspace tests, QA constructors, native snapshot fixtures, build and console smoke. Exact run/job/step metadata: docs/m4/tori/evidence/native-structural-ci.json. Separate review PASS, all original5AC/4DoD supported. Final completion/document bookkeeping changes no source; no new GUI claim. Current full graph129tasks/344edges and all dependencies/oldDone records/profile/corpus/manifest bytes unchanged. MSH-128 next dependency-ready, unassigned/unstarted; stop after127.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered native mshogi-core::tori structural foundation for mshogi.tori.modern.v1: Sente/Gote, bounded7x7 squares, nine faces including handed quails, owned pieces/hands, standard32-piece and structural diagram/import roots, immutable profile/edition/provenance/declared counters and separate accepted plies. Complete typed move/drop resolution derives all geometry, captures/demotion/hand ownership, forced origin-or-destination-zone promotion and effects. Crate-private atomic apply/undo/redo restores all logical state with position-bound single-use tokens and non-rewound runtime protection; overflow is explicit. Actual MSH-126 adapter202executed/202passed/525pending/0failed across54rows including14primary127rows;83consumer obligations pending. All40accepted board fixtures execute;2resignation specs remain129. Twelve focused tests, one public-boundary compile-fail test,247local core/conformance checks passed;2existing deepChess opt-ins not run, noTori ignored. Native properties include512-action deterministic reverse/redo chains twice, all-face capture/material and full-square rotation checks. Independent read-only source/test review PASS after strengthening14structural negative cases. Native CI41324873187 PASS on all3platforms at tested/integrated/pushed ce1801d7801f12e447b625739acd6a0c53aa2640. Evidence/commands: docs/m4/tori/native-structural-evidence.md. No selected expectations, existing games, engine/desktop capabilities or dependencies changed. Full legality/safety/drop restrictions/status/inspection remain128, history/records/dispatch129, finalnative acceptance85 and consumers77/42 remain open. MSH-128 nextReady; M4open; no later task started.
<!-- SECTION:FINAL_SUMMARY:END -->
