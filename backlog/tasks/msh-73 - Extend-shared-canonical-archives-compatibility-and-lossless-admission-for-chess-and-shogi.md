---
id: MSH-73
title: >-
  Extend shared canonical archives compatibility and lossless admission for
  chess and shogi
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T20:50:00Z'
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
- [x] #1 A versioned tagged desktop envelope preserves complete native game/match payloads,settings,commands/events,observations and original context/modes/limits for chess and shogi; old chess archivev1 remains strictly readable without semantic downgrade.
- [x] #2 Open/replay/history workers dispatch to authoritative per-game validators,preserve exact prefixes and install fresh runtime execution identities with play stopped; saved observations never resume pending work.
- [x] #3 Every accepted candidate remains completely saveable under documented per-profile capacity/work policies. Existing MSH61 admission,byte/event/control/observation boundaries,shutdown/identity headroom and stable receipts are preserved,not replaced by truncation or raised constants.
- [x] #4 Atomic failed/interrupted Save/SaveAs retains prior valid data and dirty state; corrupt/unsupported/resource-limited/stale Open preserves the current live game and unsaved-change protections.
- [x] #5 Unchanged chess archives including claims,conclusions,advanced observations and unsupported restoration cases plus concrete shogi records pass strict roundtrips. Shogi archive/search limits are evaluated from real record/observation sizes and bounded reconstruction,not assumed universal chess limits.
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
1. Freeze strict legacy chess archive v1 compatibility and add an explicit tagged v2 archive path with authoritative native shogi records, complete desktop control/observation provenance and game-specific settings. Document version dispatch and transient runtime ownership. 2. Reuse the bounded file/admission workers, stable receipts and atomic writer for shared Save/Open and native replay/history. Restore fresh identities with work stopped; preserve exact same-ply events and retired-branch observations. 3. Extend pre-commit shogi admission to encoded byte/event/observation/control and replay-work limits, with stop/close headroom and measured realistic records; preserve unchanged chess MSH-61 guarantees. 4. Add focused roundtrip/adversarial/capacity/race tests and minimal development-only host Save/Open hooks, independently review the completed boundary, run affected regressions and three-platform native/desktop CI, integrate and push codex/rewrite. Stop at MSH-73.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder: Codex in /Users/johnweathers/Development/Personal/mShogi, branch codex/rewrite, clean baseline306f3aa0bacf72d5935bce25cbd9f66fd2fd0e9e; no other worktree/active task owner. Pinned Backlog1.55.1. Read-only preimplementation independent audit identifies strict RawValue dispatch, one bounded worker, exact event identity, stale file/admission retirement, historical observations from undone branches, identity floors, atomic save and real per-profile size evaluation as concrete risks. No native rules/profile changes or MSH-74/75 implementation authorized. The MSH-72 harness remains verification-only; MSH-74 owns product visuals, including full standard9x9 visibility; future12x12/16x16 full-board display without required zoom/pan remains a product requirement.

Implemented frozen chess v1 writer/parser plus explicit v2 tagged shogi/shared reader; native initial/current records and control/observation journal validate exact events, provenance, retired branches and selected prefixes. Stable receipt off-path admission; atomic worker Save/Open, fresh issued-execution floors and stopped reopen. Measured failures at205 and46 maximum-history observations exceeded15s Open: final shogi16k weighted native-event4 replay policy with count-first raw preflight and stop reservation prevents those candidates; no constants raised or data discarded. Focused archive13 and capacity8 pass; workspace387 passed2 pre-existing ignored; session allfeature78; frontend86; independent reviewer executed13+3 Rust cases and3 frontend tests, clippy/diff clean, no remaining material findings. Native macOS packaged development Save/quit/relaunch/Open/history/Returnlive/Resume verified; hashes/screenshots/archive in docs/m4/evidence/msh73. See docs/m4/desktop-archive.md and desktop-archive-validation.md. Source integration/platformCI pending, so task remains In Review; MSH74/75 unstarted,41–47/M4 open.

Final product source integrated/pushed a4278fac13feb5159af7234191dc1d2b06401fa6, after main implementation2ba7ed28781f53f132ed52e678050f8f805379af. Final local workspace388 passed/2 pre-existing ignored, allfeature session79 passed, focusedarchive14 andcapacity8, TS86 tests, host5 tests, fmt/clippy passed. Independent reviewer separately rechecked final resource-classification regression (1 passed), finding automatic-start capacity refusal must be paused rather than engine-failed; full256 slots now reject Play before another engine turn is committed. Actual final-source QA app again opened saved native archive via native picker with exact2-event history and stopped work; normal+QA bundles rebuilt. Final normal host SHA256 b8edc787399bd5a26b3e9c0e675cec3c393c135d34f076dd30495d48469e04eb; final QA host72e5b0c0f11b1153b45ac6fefdc289ed3c50ab329fe2ed8cbf8baa8633d545ab; unchanged bundled engine d36e48dc8d6175ab3c9fc073e4de1e119f6850fd7feef5e6bc4bacc3c8830826. Earlier2ba7 GUI screenshots/journeys reused for unchanged interaction; native cancelled Save-and-close additionally verified dirty session/guard remains open. Native CI https://github.com/jweathers777/mShogi/actions/runs/40396803819 succeeded Windows/macOS/Linux at a4278fa; desktop run40396803594 pending. Superseded2ba7 desktop run40396323828 intentionally cancelled in favor of final-source verification; it is not passing evidence. No worktree created; primary checkout remains codex/rewrite. No MSH74/75 implementation started.

Final-source desktop CI https://github.com/jweathers777/mShogi/actions/runs/40396803594 succeeded on Windows2022/macOS15/Ubuntu24.04 at a4278fac13feb5159af7234191dc1d2b06401fa6, alongside native run40396803819. Desktop evidence includes locked TS/frontend/host checks, normal packages, marker exclusion, bundled-engine compatibility and separate QA packages; Windows/Linux existing installed/packaged chess/synthetic GUI smoke passed. These are not shogi product GUI acceptance; actual native shogi host Save/Open is macOS development-harness evidence. Exact tested/integrated source is a4278fac13feb5159af7234191dc1d2b06401fa6 (initial implementation2ba7ed2). All five acceptance criteria and four DoD items now supported. First sandboxed GUI launch exited134 without diagnostics; authorized non-sandboxed launch and all actual GUI journeys passed. Final source/API and documented limits independently reviewed with no remaining material finding. AC1 frozen strictchessv1 and taggedv2/nativejournal/settings/observations; AC2 nativeprefix replay/freshstoppedidentities; AC3 precommitcounts/encodedbytes/weightedwork/stop+identityheadroom/stablereceipts; AC4 atomicfailures/staleOpen/unsavedguards/concurrentIO; AC5 unchangedchesscorpus and realshogi root/hands/drop/promotions/repetition/check/declarations/resignation with measuredboundaries. Completion changes only this task record; product source CI is pinned above.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented lossless native shogi desktop Save/Open with explicit archivev2 game/profile/edition routing and strict unchanged chessv1 compatibility. Native initial/current records plus accepted control/observation journal preserve provenance, declarations/resignation and event identity distinct from board ply, retired-branch history, settings and actual target-bound results. Open validates authoritative replay, installs fresh runtime identities, keeps saved observations historical, and stops work until Resume. Bounded off-path stable-receipt admission checks byte/event/control/observation and measured weighted replay limits before commit; stop/close and identity headroom remain reserved. Atomic Save/SaveAs and stale/corrupt/resource-limited Open preserve files, accepted state and unsaved changes.

Verification: final source a4278fac13feb5159af7234191dc1d2b06401fa6 integrated/pushed codex/rewrite; initial implementation2ba7ed28781f53f132ed52e678050f8f805379af. Local388 tests passed,2 pre-existing ignored; allfeature79; focusedarchive14+capacity8; frontend86; host5; fmt/clippy and normal package exclusion/engine smoke passed. Separate reviewer read completed boundaries and independently ran14 archive cases,3 resource regressions,3 frontend guards and clippy/diff; material findings resolved. Native40396803819 and desktop40396803594 passed all three target systems. macOS actual packaged development Save/quit/relaunch/Open/history/Returnlive/Resume and cancelledSaveclose passed; exact package/revision evidence docs/m4/desktop-archive-validation.md and task notes.

Compatibility/limits: normal chess writesv1; shared Open accepts strictv1 and taggedv2 chess/shogi, never guesses future games. Shogi remains selected CSA untimed profile, ordinary bounded analysis only. Existing16MiB/256observations/8192controls unchanged; native321-event ceiling and16k weighted replay policy may stop long-history documents earlier (measured10 full-history observations retained;11th refused;256 short-history observations retained and257th refused). No truncation or raised constants. Earlier205/46-context Open deadline failures motivated final admission policy; retained final-boundary archives reopen under unchanged15s worker deadline.

Normal built app: desktop/src-tauri/target/release/bundle/macos/mShogi.app (chess). Shogi development archive example/launch in docs/m4/desktop-archive.md: separate verification app with MSHOGI_DESKTOP_FIXTURE=shared-contract, Resume→move/engine reply→Analyze→Save As; relaunch→Open→inspect exact event→Returnlive→Resume. Harness is not approved shogi visual design. MSH74 is next and remains unstarted, as does75;41–47/M4 remain open, M2/M3 closed. Full9x9 and future12x12/16x16 without required zoom/pan requirements preserved. No disposable worktrees or running verification app left.
<!-- SECTION:FINAL_SUMMARY:END -->
