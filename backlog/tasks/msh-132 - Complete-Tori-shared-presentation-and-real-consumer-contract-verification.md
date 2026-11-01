---
id: MSH-132
title: Complete Tori shared presentation and real-consumer contract verification
status: Ready
assignee: []
created_date: '2026-11-01T21:28:00Z'
updated_date: '2026-11-01T21:32:00Z'
labels:
  - m4
  - owner-msh-77
milestone: m-4
dependencies:
  - MSH-131
references:
  - >-
    backlog/tasks/msh-77 -
    Extend-shared-contracts-for-real-tori-shogi-consumers.md
documentation:
  - docs/m4/tori/shared-plan.md
  - docs/m4/tori/shared-consumer-ownership.json
  - docs/m4/tori/native-acceptance.md
  - docs/testing/conformance-policy.md
type: task
ordinal: 132000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete semantic presentation and exercise the integrated Tori session/document through the existing frontend owner and common board interaction contract. Actual consumers: shared.rs presentation structs, shared_inspection.rs, moves.rs projection; desktop/src/product/native.ts/useGameSession.ts/ProductApp.tsx/ApplicationView.tsx/GameFrame.tsx, NativeBoard.tsx, boardPresentation.ts and interaction/useNativeChoiceController.ts. Runtime predecessor supplies a minimally valid snapshot; this task completes its semantic coverage and shared frontend verification. Final Tori-specific notation, board artwork and human-engine product journeys stay MSH-42; any fixture surface must be clearly verification-only, excluded from normal packages.

Component of MSH-77 shared integration under MSH-46, not complete Tori delivery MSH-42. Accepted source994521bec0637d6f8f17103f1106c0025d950753/profile mshogi.tori.modern.v1, native acceptance a3677e1e862fdf3877c784f12f56a99c4bfbf167. Original103 independently expected fixtures and713 native assertions remain fixed; external engines are optional nonauthoritative investigations, never gates. See shared-plan.md and shared-consumer-ownership.json for exact consumer obligations. No rules, search/evaluation, console/notation, product Tori artwork/page, selector entry, other variant or M5 implementation. Future worker records its current implementation plan only after activation.

Verification: focused real native/candidate/controller tests appropriate to this boundary, exact-source commands/outcomes, independent completed-diff review, relevant existing-game regressions and applicable native/desktop CI for actual changed production. Distinguish source/controller/frontend/package/GUI evidence; no fake Tori engine or product-capability claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Native metadata provides7x7 square coordinates/orientations, Sente/Gote profile-qualified identities, all9faces with forced promotion state, LeftQuail/RightQuail distinction before/after capture/drop and native hand counts/origins. Complete choices/effects derive from actual Game permission, inspection from immutable native local queries. No optional promotion/CSA declaration, frontend legality or imposed shared native board model.
- [ ] #2 Extend strict frontend Tori snapshot/side/settings/operation and resource scope types only as required by actual native payloads. One useGameSession, ApplicationView/shared shell and useNativeChoiceController remain owners. Contract tests use real native-generated payloads, click/keyboard/drop/forced completion/cancellation, distinct non-actionable opponent board/hand inspection and delayed replies after selection/game changes. Native full token guards and duplicate submission protection remain intact; no second Tori application page.
- [ ] #3 Exact selected prefix/context/provenance, board-ply versus same-ply native event, current/selected conclusion and incomplete administrative status are presented honestly. Native event/move projection supplies actual board entries and origin/destination/drop cues; Stop/resignation do not become move rows. Native action identifiers may be labeled technical in this bounded contract, not passed off as final game notation; C-NOTATION remains MSH-42. No engine score/analysis progress is shown without a real observation.
- [ ] #4 Every MSH-77 supplemental consumer row has final integration evidence from the prior slices and appropriate semantic/frontend checks;713 original native assertions and all103 immutable expectations remain supported, not rewritten. Explicitly retain the one pending assertion and one consumer requirement of C-NOTATION under MSH-42. No pending native/consumer requirement may be silently passed; material within-scope findings block coordinating77closure.
- [ ] #5 Focused frontend/native snapshot, resource presentation and current existing-game shared owner/board/inspection/history/Moves/Take back/Save/Load/lifecycle regressions pass, with source-tagged native and frontend evidence and independent review. Use separately labeled verification instrumentation only when needed, reuse stable QA identity; no broad three-platform GUI or product acceptance claim from contract tests. Normal distributables exclude fixtures.
- [ ] #6 Tori execution capabilities remain unsupported/unadvertised and playable-game selector unchanged. Durable handoff reconciles all original77criteria, actual limitations and required42engine/console/notation/product/archive/complete-game evidence plus46/88/89all-game acceptance. Coordinating77 performs final integrated review/integration after all slices; this component alone cannot close77,42orM4. Normal product Open/New Game remains explicitly gated until42supplies the actual product renderer/playable consumer; contract/verification access uses this same native controller, never a second implementation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
