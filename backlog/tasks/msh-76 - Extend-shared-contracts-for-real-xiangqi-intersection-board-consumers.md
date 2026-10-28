---
id: MSH-76
title: Extend shared contracts for real xiangqi intersection-board consumers
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-28T17:14:00Z'
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
- [x] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [x] #2 Shared mechanisms accommodate 9x10 intersections,palace/river markers,Red/Black sides and native complete history/action/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [x] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [x] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
- [x] #5 The semantic presentation contract can feed the accepted common dark shell with stable document/live/board/sidebar slots and native intersection/river/palace/piece metadata. Native read-only enemy inspection binds exact selected history; no TypeScript movement rules, required hands/promotion/declaration, scrolling or zooming assumption. Actual product rendering and same-size comparison are verified by MSH-101/102, avoiding a prerequisite cycle.
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

Tested/integrated source7edf6cbddbfe704979a0a1bdfb7e0932ff46ff52 pushed normally to origin/codex/rewrite. Reviewer commands: cargo test --locked -p mshogi-core --test native_xiangqi_envelope -- --nocapture (5/5), cargo test --locked -p mshogi-core --test native_envelope -- --nocapture (5/5), cargo test --locked -p mshogi-engine --test xiangqi_routing -- --nocapture (7/7), cargo test --locked -p mshogi-desktop-session shared::xiangqi::tests -- --nocapture (baseline12/12), cargo test --locked -p mshogi-desktop-session shared::xiangqi::tests::handoff -- --nocapture (new2/2 independently passed twice, final0.28s). Root commands: cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings; cargo test --locked -p mshogi-desktop-session --all-features (127/127); cargo fmt --all -- --check; git diff --check; pinnedbacklog doctor; cargo build --locked -p mshogi-console --bin mshogi; cargo run --locked -p mshogi-desktop-session --features desktop-verification --example xiangqi_session_probe -- target/debug/mshogi (PASS). Runtime environment macOS/Darwinarm64 Rust1.89.0. New native CI https://github.com/jweathers777/mShogi/actions/runs/40729449029 and desktop CI https://github.com/jweathers777/mShogi/actions/runs/40729448673 started on7edf6cb; outcomes not yet claimed. No new worktree; primary remains rewrite.

Native CI40729449029 attempt1 macOS failed only in unchanged mshogi-runner/tests/shogi_runner.rs::missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove (expectedResourceLimit, actualLateReply, line355); focused same test passed locally1/1. Independent reviewer examined source/log: fixture390ms vs375ms drain within cleanup500ms exposes receive-wakeup timing; both code paths reject candidate, but CI failed before later safety assertions. Runner/core/engine production unchanged since priorgreen99. No unrelated fix/weakening in76; original failure preserved. Attempted job rerun while matrix still active was unavailable (job cannot be rerun); remaining platform jobs still running, no overall success claimed.

Integration7edf6cb CI update: Desktop40729448673 SUCCESS on Linux/macOS/Windows (including existing actual packaged Chess/Shogi interaction on Linux/Windows, normal packages and exclusion checks; no Xiangqi GUI claim). Native40729449029 attempt1: Linux and Windows SUCCESS; macOS normal session120/120 including new2 passed before unrelated runner category failure. Isolated failed macOS job rerun accepted as attempt2 after matrix completed; no source changes, currently running. Attempt1 retained at https://github.com/jweathers777/mShogi/actions/runs/40729449029/attempts/1 . Prior99 exact-production all-platform green evidence remains valid independently.

Final integration CI at7edf6cbddbfe704979a0a1bdfb7e0932ff46ff52: Rust40729449029 SUCCESS across ubuntu-24.04/windows-2022/macos-15 after isolated unchanged macOS attempt2; desktop40729448673 SUCCESS across all3. Mac rerun passed workspace (including previously failed unchanged runner assertion), all-feature session checks, fixture verification/build/smoke. Attempt1 remains FAILED in recorded history; no test/source/timeout changes were made to obtain rerun. All5AC and4DoD supported by contract audit, combined journeys, complete127local session regressions, strict envelopes/engine tests, actual supervised probe, separate independent review, final native/package CI and durable docs. MSH-101 now has all3dependencies Done when76 closes; status remains unstarted ToDo. No101/102/106implementation;45/M4open. No task76worktree or runtime process remains; primarycodex/rewrite retained.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Accepted the existing real Xiangqi shared-contract handoff with no production changes. Added two test-only combined journeys and documented native ownership, explicit profile/schema/transport identity, complete history/context, intersection presentation and inspection, archive admission/replay and stable dark-shell slots for MSH-101. Frozen F022-CC establishes warning-prefix expectations; historical inspection/analysis, same-board different-history, cancellation/replacement, Save/Open/fresh identities and stale replies are exercised together. Existing Chess/Shogi contracts and MSH-61 capacity policy remain unchanged.

Verified source/integration: 7edf6cbddbfe704979a0a1bdfb7e0932ff46ff52 on codex/rewrite, normally pushed. macOS arm64 Rust1.89 local127/127 all-feature session tests, fmt/clippy, actual supervised engine/archive probe and separate reviewer strict-envelope/session/routing tests passed. Independent reviewer found no unresolved material findings. Final native CI40729449029 and desktop CI40729448673 passed all three platforms; native macOS passed an unchanged rerun after a preserved unrelated Shogi runner timing-category assertion failure. Exact commands, original failures and review method are in the notes.

MSH-101 is dependency-ready and unstarted; it owns actual Xiangqi frontend admission and intersection rendering in the existing common shell. MSH-102 owns further product journeys. This task supplies no finished Xiangqi GUI or new playing-strength evidence. MSH-45/M4 remain open; MSH-106 remains future complete-delivery review. Stopped after MSH-76.
<!-- SECTION:FINAL_SUMMARY:END -->
