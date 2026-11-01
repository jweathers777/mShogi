---
id: MSH-128
title: Complete Tori local legality native inspection and conformance diagnostics
status: Ready
assignee: []
created_date: '2026-11-01T12:21:00Z'
labels:
  - m4
  - owner-msh-85
milestone: m-4
dependencies:
  - MSH-127
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
ordinal: 128000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-81 is Done at6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc. Selected profile mshogi.tori.modern.v1 and all103 independently expected fixture specifications are fixed inputs; do not reopen historical acquisition or import Standard Shogi defaults. This is a component owned by coordinating MSH-85 under complete game delivery MSH-42, following the repository flat component-ID precedent used for MSH-84/91-94. Own the complete local board-action boundary consuming B and A. Local Position legality and educational inspection are independent of D repetition obligations/concluded game permission. This separation is necessary for the selected forced-cycle and direct Swallow-drop-mate semantics, and for future native read-only opponent inspection.

Verification: focused reproducible commands at exact tested source, independent bounded review, normal codex/rewrite integration/push and relevant native/shared regression/CI for actually changed code. Distinguish passed/failed/not-run and reference observations from independently expected facts. No unrelated GUI campaign, playing-strength claim, new exact-proof requirement or future-variant implementation. Future worker records an implementation plan only on activation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Public complete local legal choices and checked apply use native geometry, resources, compulsory promotion, king safety and exact drop restrictions for both sides; no missing/extra actions or duplicate choices for identical hand copies. Check evasions include every admitted move/capture/drop, and pinned-piece threat geometry follows T05. Royal captures/self-check/wrong-turn/basic illegal actions reject atomically. Applying a local choice does not claim permission after a concluded/history-restricted game.
- [ ] #2 Admitted local roots enforce nonmoving Phoenix safety, nonadjacent royals, no stranded last-rank Swallow and at most two own unpromoted Swallows/file, in addition to B structural checks. Standard and custom/imported roots stay distinct from proven reachability. Local status truthfully reports ongoing/check/checkmate or quiet no-action; D owns installing the corresponding native game conclusion.
- [ ] #3 Swallow drops reject third own unpromoted/file, last rank and direct Swallow-drop mate; friendly Goose/enemy Swallow do not count. Full defense tests distinguish forbidden immediate mate from legal escape/capture, and use board-legal defenses before repetition sanctions. Other-kind mate drops and permitted last-rank drops remain available. Every originally assigned local admission case and its owner rotation is executed; transition effects and exact reversal from B are rerun through public local admission.
- [ ] #4 Native immutable inspection yields complete locally legal move/drop choices for a selected piece/hand owner on a private hypothetical turn, with handed kind/forced-result face and authoritative geometry. Both orientations/owners and board/hand choices are checked, including enemy pieces; inspection never mutates state/history, grants live admission, starts work or duplicates movement in TypeScript. MSH-77 later projects these facts into presentation metadata.
- [ ] #5 Independent complete-choice fixtures, deterministic sequences and apply/undo checks establish local conformance, including the two reviewed root-terminal positions, checks/pins/evasions and drop-mate/defense pair. Run scoped differential comparisons only against A-admitted compatible operations; retain independent coverage for excluded drop-mate cases. Provide deterministic perft/divide diagnostics and expected counts only where independently established; mismatches retain root/action path and reviewed resolution rather than changing expectations to match an engine.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
