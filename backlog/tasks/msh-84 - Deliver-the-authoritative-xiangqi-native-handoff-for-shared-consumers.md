---
id: MSH-84
title: Deliver the authoritative xiangqi native handoff for shared consumers
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-28T07:51:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
  - MSH-91
  - MSH-92
  - MSH-93
  - MSH-94
  - MSH-95
  - MSH-105
references:
  - >-
    backlog/tasks/msh-45 -
    Deliver-complete-xiangqi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/coverage.md
  - docs/m4/xiangqi/native-consumer-contract.md
  - docs/m4/xiangqi/native-handoff-evidence.md
  - docs/m4/xiangqi/native-handoff-review.md
type: task
ordinal: 84000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-45. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Refine after source/profile decisions into independently executable native slices; require rule corpus,exact reversal,record replay and real native consumer fixtures. The complete game remains open.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 After profile refinement,a native game-owned kernel exposes authoritative state,complete legal actions/derived effects,exact reversal,history/adjudication and canonical record contracts needed by its named shared consumer.
- [ ] #2 Independent rule/transition/history fixtures establish the handoff,including relevant board topology,hands/promotions,compound routes,royal identity and non-chess terminal behavior; no unverified rule convention becomes an interface default.
- [ ] #3 The shared extension consumes stable documented native payloads/capabilities without frontend rules or duplicate ownership; further game-specific engine/console/desktop work remains visible in its complete delivery record.
- [ ] #4 Before Ready,the game owner splits this native scope further where useful and records all child dependencies/verification. No implementation plan is prescribed by this planning task and no dependency on its own aggregate delivery is introduced.
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
1. Audit the completed91–95/105 public native system and existing real shared consumer against the selected profile; obtain separate consumer-readiness assessment and identify any actual native defect before considering production changes.
2. Publish a current native consumer contract covering API entry points, identity layers, fast/exact provenance, uncertainty/operational policy, boundedness/errors and precise later ownership; preserve historical handoff/evidence.
3. Add focused integration journeys through public native/shared APIs for actions/undo/admission, warnings/fourth repetition, claims/outcomes, complete replay/context and fast/exact proof/Unknown/cancel; reuse independently reviewed fixtures rather than inventing rule expectations.
4. Resolve review findings, run focused and normal native format/lint/regressions, reuse unchanged platform evidence where appropriate, independently review final handoff diff.
5. Record supportedAC/DoD and exact tested/integrated revisions, integrate/push normally and stop. Keep96/97/106 and45/M4 unstarted/open as applicable.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-80 refinement2026-10-27: original4AC/DoD retained. Native slices91 fixtures/reference,92 state/actions/reversal/codecs,93 full local legality,94 complete selected history/adjudication/canonical records,95 actual typed-record consumer supply this handoff. None depends on84 or45 final completion, so no aggregate cycle.95 extends the shared record/routing boundary only; engine97 and full session/archive98/99 remain later. All future tickets To Do pending80 ratification/research, not implementation authorization. R-EDITION/R-CYCLE/R-CLASS/R-NOWIN remain named obligations; no unsupported convention or scope reduction.

XIANGQI-P1 two-level3.2A refinement2026-10-27: add required105 boundedexact verifier to complete native handoff.94 owns fastsound/native contract;105 depends94/95 and never84/97, avoiding aggregatecycle. Original4AC/DoD/allprior dependencies preserved; native completeness cannot be claimed from recognizer misses or unknown proofs.

MSH84 readiness check2026-10-28 at f548e8225807484bfd3eae1246c4fec0c2db1488: seven direct dependencies80/91/92/93/94/95/105 Done. Existing80 refinement already split authoritative native scope into91fixture/reference,92state/actions/reversal/codecs,93locallegality,94history/adjudication/records/fastassessment,95typed routing and105bounded exact analysis; all child verification/evidence preserved. No dependency on aggregate45/later consumers and no further split needed for this handoff. Current To Do can become Ready under originalAC4 without changing criteria/dependencies.

Primary builder/integrator @codex in primary checkout codex/rewrite atf548e82; clean sole worktree, refreshed origin matches, no active task owners. PATHBacklog1.55.0 is read-only; mutations use pinned1.55.1 matchingAGENTS. Separate /root/handoff84_review audited concrete native APIs, identity/error/Unknown/capability semantics and ran20existing exact/envelope/nativeboundary tests PASS; no material production defect found. Scope is consumer-readiness documentation and crossing integration tests, not96/97/architecture review. Root is sole writer; separate reviewer will assess final diff.

Initial separate native readiness audit found no material production defect/refactor requirement; existing game-owned public module plus native/shared adapter suffice. Added native-consumer-contract.md (identity/API/provenance/error/bounds/Unknown policy) and native-handoff-evidence.md; refreshed stale current status without deleting historical records. Added6public native/shared crossing journeys in mshogi-engine/tests/xiangqi_handoff.rs; initial focused execution PASS6. Existing91–95/105 independently reviewed fixtures unchanged. No production source or profile change and no later task implementation.

Separate final /root/handoff84_review consumer-readiness/source/diff audit PASS with no material findings and no production refactor needed; independently executed20existing exact/envelope/game-boundary tests and6new integratedjourneys (26PASS), diffcheckPASS. Report native-handoff-review.md records actual method/commands/limits. Root fmt/clippy(allworkspace/alltargets/-Dwarnings) PASS; workspace490PASS/2existingignored;6new journeys PASS. Existing91–95/105 expected fixtures and actual reference/source/platform evidence reused; no production behavior changed. Native CI on committed test/document revision and normal integration/push remain pending.
<!-- SECTION:NOTES:END -->
