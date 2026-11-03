---
id: MSH-78
title: Extend shared contracts for real chu shogi compound and royal consumers
status: Done
assignee:
  - '@Codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-03T10:18:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-86
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/chu/profile.md
  - docs/m4/chu/handoff.md
  - docs/m4/chu/native-v1.md
  - docs/m4/chu/native-verification.json
  - docs/m4/chu/shared-v1.md
  - docs/m4/chu/shared-verification.json
type: task
ordinal: 78000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Before activation refine against the accepted game profile and concrete native handoff; verify codecs,context races,records and shared UI behavior with independently expected real-game fixtures. No dependency on the aggregate game delivery or MSH46 final completion.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [x] #2 Shared mechanisms accommodate 12x12 compound routes,same-endpoint choices,native-derived effects and multiple-royal status without chess self-check assumptions through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [x] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [x] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
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
2026-11-03: sole builder/integrator Codex on clean primary codex/rewrite at7362307; original four AC/DoD and69/72/73/86 dependencies preserved. A: extend explicit native GameKind/profile/schema/side/command/RawValue record dispatch; inspect all exhaustive production consumers, standalone Tauri host and TypeScript early. Add Chu immutable payload codec inside the current DocumentEnvelope, strict native replay and actual retained-context accounting; measure realistic compound/history/observation/retired-branch workloads and establish bounded aggregate admission with encoded Stop/Resign/Close reservations. B: integrate native Chu Game and validated candidate into existing ActiveDocument/DesktopController operation/persistence/installation lifecycle; retain actors, exact history/prefixes, fresh stopped reopen, native Undo and desktop Take back, atomic replacement, dirty Quit and stale execution retirement. No second runtime, document format or persistence owner. C: native-derived 12x12 pieces/royals/threats/obligations/complete choices/effects/Moves geometry and immutable inspection; exercise real routes via existing PresentedChoice/useNativeChoiceController, with a clearly labeled development consumer only. No frontend rules, Shogi hands or normal Chu chooser/Load activation. Execution remains actual supervised UnsupportedMode and unscored until43. D: real Chu consumer tests and fixtures, exact accounting of applicable ten future corpus rows vs43 product obligations, focused four-game boundary regressions, measured capacity/adversarial/lifecycle tests and separate independent reviews. Required CI: final-source three-platform Rust jobs plus applicable frontend/typecheck/build and standalone-host/package checks; no finished Chu GUI campaign. One structured evidence record, normal integration/push and original-criteria finalization only after required evidence; stop78,43/M4 open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH82 refinement2026-11-02: consume actual86 payload under mshogi.chu.jcsa2019.v1, no speculative common rules layer. Original dependencies69/72/73/86 and4AC/DoD unchanged. Current native GameKind/profile/schema/RawValue routing, one execution context, DocumentEnvelope/DesktopController/ActiveDocument, frontend session and native-choice controller remain owners. Add real Chu dispatch/codec/replay/royal and admission metadata, complete ordered same-endpoint/return choices and native effects; no hands/self-check/collapsed base origins. Measure long native records/observations/replay before precommit capacity policy; retain complete saveability/stop-close headroom and fresh execution authority. Real native-generated fixtures and current four-game affected-boundary regressions required; synthetic board alone insufficient. Chu execution remains explicitly unsupported/unscored until43. See docs/m4/chu/handoff.md. To Do until actual86 handoff; no implementation started.

MSH86 realnative handoff2026-11-02 now documented in docs/m4/chu/native-v1.md; no78implementation starts. Original4AC/4DoD and complete69/72/73/86dependencies unchanged; Ready/unassigned means actualscope actionable, execution waits86Done. Consume mshogi_core::chu Game/completeorderedAction/Effects/royals/history/strictGameRecord1 underexact mshogi.chu.jcsa2019.v1 edition. Nativebounds512KiB/512events/4Mdeterministicwork with measured227move132199byte/3979825unitrecord and dynamicencodedStop+Resignheadroom; outer16MiB/observation/control/deadline/close admission must separatelymeasure actualChu completecontexts,nevercopyconstantsblindly. NativeGameContext bindsroot+rights+fullevents+monotonicruntime; sharedpurpose/session/request/process/limits remain currentowner. Extend existingGameKind/profile/schema/record/NativeSide dispatch,RawValue codec,onecontroller/document/runtime/frontend/choice owner and realnative-generatedcompound/royal/status/inspection/historyfixtures. Coreconsumeractualtwo-victimpayload/replay/provenance/stalechecks exists;1076nativeassertions240rows do not establishshared/product10futureconsumerrows. KeepChuengineexplicitlyunsupported/unscored until43; no ChessFullinheritance. Preserve fourgamecurrentcompatibility. No newframework/secondruntime/normalchooseractivation. FinalnativeCI/86completionrefs follow structured evidence.

MSH86 native acceptance completed at tested production source f44e0b9673bbbe1cc57a8a45522b04948d1c69bb, required three-platform Rust CI41539884098 all steps PASS. Original69/72/73/86 dependencies now satisfied; Ready/unassigned remains the next bounded shared-consumer assignment, not started here. Actual native-core consumer and stable API/record1/limits documented in docs/m4/chu/native-v1.md with structured evidence native-verification.json. Consume complete routes/ordered effects, native royal/repetition metadata and exact prefixes via strict native replay; measure outer document/observation/control/Stop/Close headroom rather than inheriting native constants. Ten future consumer corpus rows remain78/43 owned. No execution/chooser capability until43; original AC/DoD/dependency sets preserved.

MSH78 checkpoints A-C implemented on primary codex/rewrite; sole builder Codex, independent read-only codec/runtime and frontend reviewers. Explicit native Chu routing, immutable current-envelope codec and one-controller/worker lifecycle, real compound/royal metadata/choices, strict duplicates and supervised UnsupportedMode remain gated from normal New/Load. Measured 12M aggregate replay policy (ordinary-grown227 events:11939711 units; imported equivalent refuses); unchanged outer/native limits and actual Stop/Resign/Close reserves. New focused routing, codec capacity (256→257,4094+2 endings, control/bytes), six controller, actual child/context, native-generated frontend fixtures/interaction and QA-host tests pass. Frontend391 checks, host15, normal macOS package/smoke pass. Review corrected reservation byte, historical flip, replay cancellation checkpoints and narrow unsupported recovery controls; existing evidence and original criteria preserved. Single structured evidence docs/m4/chu/shared-verification.json; final source/CI and integrated acceptance still pending. No43implementation or finished Chu GUI claim.

Coherent source checkpoint ready: all277 desktop-session all-feature tests pass (182.24s), all391 frontend tests pass, workspace/QA/standalone-host clippy and strict TypeScript pass, normal macOS package and actual bundled-engine smoke pass. Six real Chu controller tests independently repeated PASS;39 focused frontend tests independently PASS after fixes. No actual Chu packaged GUI/finished-window evidence claimed at this infrastructure boundary. Required final-source Rust+Desktop three-platform CI still pending. Original4AC/4DoD remain unchecked until final integrated acceptance.

Final source reconciliation found and corrected Chu u64 presentation precision above JavaScript safe-integer range. Native decimal root/selected/live counters and provenance flags now remain exact; technical Moves number known-root pairs rather than manufacturing unknown prior rows. Nine controller tests and13 focused frontend/Moves checks pass, includingu64MAX and Gote-first ordering. Native records, archive capacity/rules and all existing4game representations unchanged. Supersededf41f CI41606708598/41606708574 were cancelled (partial successes retained, never fullPASS); corrected exact-source CI required beforeDone.

Full integration checkpoint fcccc37664603688a3fc26dbef21e0b0c7d0e484: Rust41610001673 and Desktop41610001503 all6jobsPASS; actual completed logs confirm280session tests per phase and392frontend tests/15host tests per platform. Final independent diagnostic review found native Counter is ambiguous between board counters and runtime IDs; adapter now returns existing typed Unknown with Chu scope rather than inventing IdentityHeadroom. Both affected tests independentlyPASS; complete10controller cases, workspace clippy and host15PASS. Final change is native-only classification plus focused tests; unchanged frontend/host/archive schemas/four-game/package paths reuse successful fcccc37 Desktop3platform evidence. Final exact-source native3platform CI plus local host/normal-package recheck required; original4AC/DoD remain unchecked pending final results. No43implementation.

Final acceptance2026-11-03: tested/integrated source 250b701efd23dac648ffdb984c83da868fa8840a; final native Rust41615048164 all3jobsPASS, full checkpoint fcccc37 Rust41610001673/Desktop41610001503 all6jobsPASS. Original4AC/4DoD supported; independent source+executed review passed. Final narrow correction explicitly reuses unchanged Desktop evidence; local final-source host/package PASS. Cancelled campaigns are not fullPASS. Completion record is documentation/task bookkeeping only; production unchanged since tested revision. Stop78;43/M4 open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered real Chu native routing and shared application consumption at tested/integrated source 250b701efd23dac648ffdb984c83da868fa8840a on codex/rewrite. Preserved mshogi.chu.jcsa2019.v1/edition/native1/protocol4/current envelope1, strict RawValue replay, full history/provenance/Lion rights/royals/events/ordered actions and captures. One existing controller/installation/file lifecycle and frontend choice/session owner; actual supervised UnsupportedMode remains unscored and normal Chu New/Load gated.

AC1: actual native canonical/command/session dispatch and real child/controller identity/purpose/history/cancel/selection/Take back/reopen/replacement probes. AC2: native12x12 complete compound routes, captures/promotion/King+Prince/advisory threat metadata, exact decimal counters and known-root move rows; no frontend rules/hands/self-check. AC3: immutable lossless current-envelope candidates, measured12M aggregate replay policy with unchanged16MiB/4096events/8192controls/256observations/15s and actual Stop/Resign/Close reserves. Boundary failures preserve accepted state/files; ambiguous native Counter uses truthful existing Unknown fact. AC4: seven actual native-generated DTOs;1routing,9codec,10controller,2actual-child/controller,7frontend plus chooser regressions and required current4game checks. Native86 evidence stays historical;1076 native checks not recounted.

Independent read-only codec/runtime/capacity and native presentation/frontend reviews plus actual focused CLI execution passed. Material reservation-byte, historical-flip, cancellation-checkpoint, unsupported-recovery, native Action typing, counter-precision and diagnostic classification findings resolved. Full integration checkpoint fcccc37664603688a3fc26dbef21e0b0c7d0e484: Rust41610001673/Desktop41610001503 all6jobsPASS,280session tests per native phase and392frontend/15host per platform. Final native-only correction 250b701efd23dac648ffdb984c83da868fa8840a: Rust41615048164 all3jobsPASS,10controller tests/host15/workspace clippy and normal macOS package+actual bundled-engine smokePASS; unchanged Desktop3platform evidence reused explicitly, including applicable packages and current4game Windows/Linux GUI campaigns. No new finished Chu GUI or strength claim. Cancelled earlier/redundant campaigns and failed intermediate probes retained accurately in shared-verification.json.

docs/m4/chu/shared-v1.md, shared-verification.json and handoff.md map all10consumer rows and exact43 product boundary. Remaining MSH43: functional engine/search/console/notation, traditional-kanji finished no-scroll12x12 UI, normal activation and complete-game/platform packaged journeys; preserve shared owners, bounded native contexts and admission. MSH43 dependency-satisfied but stays To Do/unstarted; M4/shared integration/Tenjiku/M5 open. No own worktree. Completion record changes only documentation/task evidence after tested source.
<!-- SECTION:FINAL_SUMMARY:END -->
