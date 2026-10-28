---
id: MSH-76
title: Extend shared contracts for real xiangqi intersection-board consumers
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-28T16:47:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-84
  - MSH-95
  - MSH-98
  - MSH-99
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/shared-contract-handoff.md
type: task
ordinal: 76000
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
- [ ] #2 Shared mechanisms accommodate 9x10 intersections,palace/river markers,Red/Black sides and native complete history/action/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [ ] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [ ] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
- [ ] #5 The semantic presentation contract can feed the accepted common dark shell with stable document/live/board/sidebar slots and native intersection/river/palace/piece metadata. Native read-only enemy inspection binds exact selected history; no TypeScript movement rules, required hands/promotion/declaration, scrolling or zooming assumption. Actual product rendering and same-size comparison are verified by MSH-101/102, avoiding a prerequisite cycle.
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
1. Audit the completed69/72/73/84/95/98/99 native routing/session/archive boundaries against all five76criteria; trace one real Xiangqi consumer path and record explicit identities, ownership, optional capabilities and remaining product gates.
2. Exercise focused cross-boundary journeys with authoritative native/frozen WXF fixtures: canonical creation, native engine/analysis context, historical warning/claim prefixes and inspection, replacement/cancel/new request, Save/Open/stale replies, wrong game/profile and same-board/different-history. Reuse established rule expectations and unchanged tests; add bounded integration tests only where they provide missing aggregate evidence.
3. Map native semantic data to the accepted shared shell slots and document concrete downstream consumption, resource policy and absent concepts without implementing rendering or frontend legality. Make production corrections only for demonstrated shared-contract gaps; no cosmetic refactor, new rules/engine/archive or101/102work.
4. Run focused native/shared compatibility checks and appropriate frontend/type checks if affected; reuse still-applicable exact-source CI from99 when production is unchanged. Obtain separate independent boundary review, resolve findings, record actual commands/revisions/evidence and complete normal integration/push. Stop76;45/M4open and106future.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-80 refinement2026-10-27: original4AC/DoD and prior dependencies retained. Shared ownership is95 typed native routing,98 authoritative session/semantic intersection presentation/native read-only inspection,99 archive/admission. This aggregate accepts those real boundaries; it does not own another rules kernel, engine or finished Xiangqi UI. Product rendering/journeys are101/102 after76. Native84 never depends on76. Accepted MSH-74 shell, full-board visibility and MSH-61 saveability remain product requirements.

Startup892c1ba9ed438bb56952fdf6843ba50f04be6cbb on clean primarycodex/rewrite; remote refreshed, seven recorded prerequisites Done, no InProgress/InReview owner. Root@codex is sole builder/integrator using primary checkout; no new worktree. Existing98/100/111 app-managed cleanup handoffs remain separate. Read owner attachment and existing76AC/DoD/design; pinnedCLI1.55.1 resolves PATH1.55.0 mismatch before mutation. Native95/98/99 remains authoritative; product rendering101/102 and architecture106 not started.

Aggregate audit found no production gap: existing84/95/97/98/99 contracts satisfy native routing/session/execution/presentation/archive handoff. Added two test-only combined journeys using independently reviewed frozen F022-CC; common-shell native field/slot map and intentional101 product-adapter boundary are documented in docs/m4/xiangqi/shared-contract-handoff.md. No UI/product admission, rules, engine, archive policy or dependency changes. Root local macOS arm64 Rust1.89: cargo fmt --all -- --check and all-target/all-feature desktop-session clippy -D warnings passed; cargo test --locked -p mshogi-desktop-session --all-features passed127/127 in73.77s, including new2 and unchanged Chess/Shogi archive/capacity regressions. Separate reviewer independently passed2/2 new tests and baseline strict native envelopes5XQ+5Chess/Shogi,12sessionXQ and7actual engine-routing tests; final doc review pending. First aggregate test expectation incorrectly set analysis defaults before a later analysis request updated them; corrected test ordering to reflect existing native preferences semantics, no production change. First subprocess probe launch used absent target/release/mshogi and failed (0 observed board actions); correct debug engine built for rerun, no GUI claim.

Completed independent review by separate Codex reviewer /root/msh76_contract_review: read-only full native/shared/frontend/archive boundary audit plus independent test execution (not author self-review). No material gap/finding. Resolved documentation precision: Unknown no-win evidence can coexist with ordinary heuristic evaluation; incomplete/failed exact work is unscored. Added optional reviewer assertions of reopened native game/profile/selection scope/events. Corrected actual subprocess probe passed using freshly built target/debug/mshogi (SHA2560a2dde6ad1f924047ad6ac7803dc604279841766e2b0e3a1af5f857bd04c233d): supervised play/live-history analysis/cancel, Save/fresh-controller Open, historical selection, stopped work, ReturnLive/Resume, fresh identities and acknowledged cleanup. No packaged Xiangqi GUI claim. backlog doctor and git diff --check passed. Unchanged production CI from99 at dd510f3 is reused: Rust40722182393 and Desktop40722182395 all3platforms succeeded; new integration CI will run after push. MSH-101 dependencies97/96 alreadyDone plus76 completion;101 remains unstarted ToDo,102/106unstarted,45/M4open.
<!-- SECTION:NOTES:END -->
