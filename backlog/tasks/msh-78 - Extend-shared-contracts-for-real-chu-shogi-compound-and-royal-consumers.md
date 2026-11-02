---
id: MSH-78
title: Extend shared contracts for real chu shogi compound and royal consumers
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T23:08:00Z'
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
- [ ] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [ ] #2 Shared mechanisms accommodate 12x12 compound routes,same-endpoint choices,native-derived effects and multiple-royal status without chess self-check assumptions through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [ ] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [ ] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH82 refinement2026-11-02: consume actual86 payload under mshogi.chu.jcsa2019.v1, no speculative common rules layer. Original dependencies69/72/73/86 and4AC/DoD unchanged. Current native GameKind/profile/schema/RawValue routing, one execution context, DocumentEnvelope/DesktopController/ActiveDocument, frontend session and native-choice controller remain owners. Add real Chu dispatch/codec/replay/royal and admission metadata, complete ordered same-endpoint/return choices and native effects; no hands/self-check/collapsed base origins. Measure long native records/observations/replay before precommit capacity policy; retain complete saveability/stop-close headroom and fresh execution authority. Real native-generated fixtures and current four-game affected-boundary regressions required; synthetic board alone insufficient. Chu execution remains explicitly unsupported/unscored until43. See docs/m4/chu/handoff.md. To Do until actual86 handoff; no implementation started.

MSH86 realnative handoff2026-11-02 now documented in docs/m4/chu/native-v1.md; no78implementation starts. Original4AC/4DoD and complete69/72/73/86dependencies unchanged; Ready/unassigned means actualscope actionable, execution waits86Done. Consume mshogi_core::chu Game/completeorderedAction/Effects/royals/history/strictGameRecord1 underexact mshogi.chu.jcsa2019.v1 edition. Nativebounds512KiB/512events/4Mdeterministicwork with measured227move132199byte/3979825unitrecord and dynamicencodedStop+Resignheadroom; outer16MiB/observation/control/deadline/close admission must separatelymeasure actualChu completecontexts,nevercopyconstantsblindly. NativeGameContext bindsroot+rights+fullevents+monotonicruntime; sharedpurpose/session/request/process/limits remain currentowner. Extend existingGameKind/profile/schema/record/NativeSide dispatch,RawValue codec,onecontroller/document/runtime/frontend/choice owner and realnative-generatedcompound/royal/status/inspection/historyfixtures. Coreconsumeractualtwo-victimpayload/replay/provenance/stalechecks exists;1076nativeassertions240rows do not establishshared/product10futureconsumerrows. KeepChuengineexplicitlyunsupported/unscored until43; no ChessFullinheritance. Preserve fourgamecurrentcompatibility. No newframework/secondruntime/normalchooseractivation. FinalnativeCI/86completionrefs follow structured evidence.

MSH86 native acceptance completed at tested production source f44e0b9673bbbe1cc57a8a45522b04948d1c69bb, required three-platform Rust CI41539884098 all steps PASS. Original69/72/73/86 dependencies now satisfied; Ready/unassigned remains the next bounded shared-consumer assignment, not started here. Actual native-core consumer and stable API/record1/limits documented in docs/m4/chu/native-v1.md with structured evidence native-verification.json. Consume complete routes/ordered effects, native royal/repetition metadata and exact prefixes via strict native replay; measure outer document/observation/control/Stop/Close headroom rather than inheriting native constants. Ten future consumer corpus rows remain78/43 owned. No execution/chooser capability until43; original AC/DoD/dependency sets preserved.
<!-- SECTION:NOTES:END -->
