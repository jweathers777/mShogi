---
id: MSH-127
title: Implement native Tori state complete transitions and exact reversal
status: Ready
assignee: []
created_date: '2026-11-01T12:21:00Z'
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
- [ ] #1 Game-owned Tori Side/Square/Piece/Hand/Position/root primitives explicitly represent7x7 coordinates, all9 native faces, distinct LeftQuail/RightQuail and Sente/Gote, selected immutable profile identity/edition and standard32-piece setup. Structural diagram validation enforces unique/domain squares, royal/base inventory and hand types/counts; reduced custom material, root provenance and immutable declared counter are distinct from accepted plies and proven historical reachability. Final royal/drop root admission remains C.
- [ ] #2 Complete typed move/drop actions and native derived effects cover every movement table, rays/blockers, jumps without intermediate capture, quail orientation, captures/demotion/owner changes, hand resources and forced promotion. Promotion depends on origin OR destination in the farthest2ranks, never on drops or a hidden timer; subsequent zone departure/within movement after a drop promotes. Impossible optional faces, promoted drops, invented victims/routes and basic malformed/resource/ownership requests reject atomically.
- [ ] #3 Meaningful structural geometry/transition/root facets of all assigned fixtures execute independently expected facts for both owners, every face, blockers/leaps, quail capture/drop identity, Eagle/Goose demotion, drop hand deltas and compulsory promotion including TORI-P-DROP-EXIT. The42 accepted transition specifications supply exact before/after assertions where applicable; full legal membership, mate/drop-mate and terminal commands are not claimed here.
- [ ] #4 Apply/undo/redo restores board, hands, face, side, immutable root metadata, accepted ply count and incremental caches exactly across accepted captures, drops and promotions. Wrong/stale/out-of-order undo rejects without mutation; deterministic multi-action sequences check restoration. Counter/resource overflow is explicit and cannot masquerade as zero legal choices or a game conclusion.
- [ ] #5 Intermediate transition/candidate helpers remain private or unambiguously limited and inaccessible as complete game admission. No Tori engine capability, desktop chooser, session, canonical-record handoff or claim of complete legal-action conformance is introduced before its owning slice; native Rust remains independent of UI/Python/console.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
