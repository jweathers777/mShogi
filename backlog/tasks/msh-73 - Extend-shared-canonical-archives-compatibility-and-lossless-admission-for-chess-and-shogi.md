---
id: MSH-73
title: >-
  Extend shared canonical archives compatibility and lossless admission for
  chess and shogi
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T20:30:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-72
  - MSH-69
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/desktop-archive.md
  - docs/m4/desktop-archive-validation.md
type: task
ordinal: 73000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Reuse M3 canonical corpus; strict old/new format,capability mismatch,concurrent IO/candidate races and each exact resource boundary,including256→257 when that policy is selected.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A versioned tagged desktop envelope preserves complete native game/match payloads,settings,commands/events,observations and original context/modes/limits for chess and shogi; old chess archivev1 remains strictly readable without semantic downgrade.
- [ ] #2 Open/replay/history workers dispatch to authoritative per-game validators,preserve exact prefixes and install fresh runtime execution identities with play stopped; saved observations never resume pending work.
- [ ] #3 Every accepted candidate remains completely saveable under documented per-profile capacity/work policies. Existing MSH61 admission,byte/event/control/observation boundaries,shutdown/identity headroom and stable receipts are preserved,not replaced by truncation or raised constants.
- [ ] #4 Atomic failed/interrupted Save/SaveAs retains prior valid data and dirty state; corrupt/unsupported/resource-limited/stale Open preserves the current live game and unsaved-change protections.
- [ ] #5 Unchanged chess archives including claims,conclusions,advanced observations and unsupported restoration cases plus concrete shogi records pass strict roundtrips. Shogi archive/search limits are evaluated from real record/observation sizes and bounded reconstruction,not assumed universal chess limits.
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
1. Freeze strict legacy chess archive v1 compatibility and add an explicit tagged v2 archive path with authoritative native shogi records, complete desktop control/observation provenance and game-specific settings. Document version dispatch and transient runtime ownership. 2. Reuse the bounded file/admission workers, stable receipts and atomic writer for shared Save/Open and native replay/history. Restore fresh identities with work stopped; preserve exact same-ply events and retired-branch observations. 3. Extend pre-commit shogi admission to encoded byte/event/observation/control and replay-work limits, with stop/close headroom and measured realistic records; preserve unchanged chess MSH-61 guarantees. 4. Add focused roundtrip/adversarial/capacity/race tests and minimal development-only host Save/Open hooks, independently review the completed boundary, run affected regressions and three-platform native/desktop CI, integrate and push codex/rewrite. Stop at MSH-73.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder: Codex in /Users/johnweathers/Development/Personal/mShogi, branch codex/rewrite, clean baseline306f3aa0bacf72d5935bce25cbd9f66fd2fd0e9e; no other worktree/active task owner. Pinned Backlog1.55.1. Read-only preimplementation independent audit identifies strict RawValue dispatch, one bounded worker, exact event identity, stale file/admission retirement, historical observations from undone branches, identity floors, atomic save and real per-profile size evaluation as concrete risks. No native rules/profile changes or MSH-74/75 implementation authorized. The MSH-72 harness remains verification-only; MSH-74 owns product visuals, including full standard9x9 visibility; future12x12/16x16 full-board display without required zoom/pan remains a product requirement.

Implemented frozen chess v1 writer/parser plus explicit v2 tagged shogi/shared reader; native initial/current records and control/observation journal validate exact events, provenance, retired branches and selected prefixes. Stable receipt off-path admission; atomic worker Save/Open, fresh issued-execution floors and stopped reopen. Measured failures at205 and46 maximum-history observations exceeded15s Open: final shogi16k weighted native-event4 replay policy with count-first raw preflight and stop reservation prevents those candidates; no constants raised or data discarded. Focused archive13 and capacity8 pass; workspace387 passed2 pre-existing ignored; session allfeature78; frontend86; independent reviewer executed13+3 Rust cases and3 frontend tests, clippy/diff clean, no remaining material findings. Native macOS packaged development Save/quit/relaunch/Open/history/Returnlive/Resume verified; hashes/screenshots/archive in docs/m4/evidence/msh73. See docs/m4/desktop-archive.md and desktop-archive-validation.md. Source integration/platformCI pending, so task remains In Review; MSH74/75 unstarted,41–47/M4 open.
<!-- SECTION:NOTES:END -->
