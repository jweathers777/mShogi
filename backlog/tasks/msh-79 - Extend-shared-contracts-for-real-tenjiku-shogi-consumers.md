---
id: MSH-79
title: Extend shared contracts for real tenjiku shogi consumers
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-04T11:27:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-87
  - MSH-78
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/tenjiku/handoff.md
  - docs/m4/tenjiku/profile.md
  - docs/m4/tenjiku/native-v1.md
  - docs/m4/tenjiku/native-verification.json
type: task
ordinal: 79000
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
- [ ] #2 Shared mechanisms accommodate 16x16 complete routes,profile-specific ordered derived effects,royal status and native history/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
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
MSH83 two-profile handoff: preserve selected native profile/edition through New root, record/envelope, execution/observation and inspection/history. Existing shared_new_game game-only profile inference and side(NativeSide) hardcoded profile-qualified presentation keys need bounded extensions for two Tenjiku profiles. Immutable profile-to-document binding plus full native context protects tokens; adding a profile token field is an implementation choice. Both-direction same-board cross-profile rejection and intentional fresh New versus in-place mutation are separate expected consumers. Add real native-derived burn/royal/complete-route projections without frontend rules or second controller/envelope/shell. Measure whole retained record/observation/branch/history/install/ending-Close admission and inspection costs against actual87; no Chu default constants silently inherited. Remains ToDo until actual native87 handoff can be inspected; originalcriteria/dependencies unchanged, no implementation begun.

Actual87source handoff now reviewable in public mshogi_core::tenjiku; native-v1.md documents exact types/capabilities/schema1/profile identities and resource facts, native-verification.json holds actual executed evidence. Consume explicit Profile with Game/GameRequest/GameContext/GameRecord, complete Action/Effects/EffectPhase, LocalPosition inspection, royal/history/prefix/capacity metadata. Native1MiB/1024commands-events/512Mlogical units includes real Stop→Resign→Stop reserve; desktop whole-document/observation/branch/ending-Close admission and15second worker/inspection lock risks remain79 measurements. Ordinary replay drops private undo handles. Public-core consumer2 passes bothprofiles and forgedburn/profile/stale rejection; no shared native.rs/controller/envelope/frontend/engine activation implemented. Preserve originalcriteria/dependencies; Ready refinement awaits87finalCI/completion. No79 implementation plan or workstarted.

87Done actualnative handoff accepted at9749eec, requiredthree-platformRustCI41816309261passed allsteps; independent finalreviewapproved. Earlier ToDo/pending87 statements are superseded. Current dependencies69/72/73/78/87Done, concreteAPI/schema/actions/effects/history/resource contract and realexternalconsumer now available; Readyunassigned, noimplementationstarted and nofutureimplementationplan. Preserve originalcriteria/dependencies. NewGame must select one immutable profile explicitly; implement existingnative/shared routing/session/controller/envelope/completechoice/inspection paths using native-v1, measurewhole-document+observation/branch/endingClose budgets andworker/locklatency againstrealnativecases. Existing112race remains separatelyunfixed, not a dependency or excuseforweakenedchecks.44retains playableactivation/search/console/notation/kanji. Stop87handoff, do notstart79here.
<!-- SECTION:NOTES:END -->
