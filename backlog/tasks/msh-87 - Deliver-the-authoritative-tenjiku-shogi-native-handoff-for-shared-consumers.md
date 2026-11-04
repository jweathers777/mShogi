---
id: MSH-87
title: Deliver the authoritative tenjiku shogi native handoff for shared consumers
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-04T11:29:00Z'
labels:
  - m4
  - owner-msh-44
milestone: m-4
dependencies:
  - MSH-83
references:
  - >-
    backlog/tasks/msh-44 -
    Deliver-complete-tenjiku-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/testing/conformance-policy.md
  - docs/m4/tenjiku/handoff.md
  - docs/m4/tenjiku/profile.md
  - docs/m4/tenjiku/fixtures.md
  - docs/m4/tenjiku/native-v1.md
  - docs/m4/tenjiku/native-verification.json
type: task
ordinal: 87000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-44. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Refine after source/profile decisions into independently executable native slices; require rule corpus,exact reversal,record replay and real native consumer fixtures. The complete game remains open.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 After profile refinement,a native game-owned kernel exposes authoritative state,complete legal actions/derived effects,exact reversal,history/adjudication and canonical record contracts needed by its named shared consumer.
- [x] #2 Independent rule/transition/history fixtures establish the handoff,including relevant board topology,hands/promotions,compound routes,royal identity and non-chess terminal behavior; no unverified rule convention becomes an interface default.
- [x] #3 The shared extension consumes stable documented native payloads/capabilities without frontend rules or duplicate ownership; further game-specific engine/console/desktop work remains visible in its complete delivery record.
- [x] #4 Before Ready,the game owner splits this native scope further where useful and records all child dependencies/verification. No implementation plan is prescribed by this planning task and no dependency on its own aggregate delivery is introduced.
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
A. Implement explicit two-profile16x16 state,44faces/30promotions/156setup, complete local actions/effects/inspection and instance-bound reversal; focused independent local-rules review.
B. Implement authoritative Game history/promotion rights/repetition/royal and bare-response endings, compact reversal tokens, strict cancellable schema1replay and measured native admission with actual Stop/Resign reserve.
C. Add thin Tenjiku conformance corpus boundary and real production adapters for all applicable350rows, exact geometry/action/effect sets, concrete histories, native properties/adversarial cases and external-core-consumer/perft checks. Keep native versus annotations versus future-consumer accounting explicit.
D. Review source/adapters/resource measurements independently, check root and standalone-host consumer boundaries, run coherent relevant regressions/format/lint and one exact-source three-platform nativeCI campaign. Integrate/push, finalize only supportedAC1–3/DoD, preserveAC4 and update79/44handoffs. No shared/product activation or new task hierarchy; stop after87.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH83 current handoff4November2026: implement the coherent native kernel directly after ratified83, as one bounded assignment with internal state/transitions, LegalAll/effects, history/records and conformance/acceptance checkpoints. No additional child hierarchy is useful under current owner direction; originalAC4 where-useful refinement is satisfied by this direct bounded scope, without changing its text. Two whole immutable profiles: mshogi.tenjiku.adams1999-modern.v1 and mshogi.tenjiku.muller-completed.v1. Preserve actual FD/FE/HT/general differences, explicit shared A/S completions, original/promoted identities and per-piece refusal rights.350corpus rows:204native-boundary/129annotations/17consumer; all native observations pending87, not loader passes. Add thin native corpus adapters reusing shared comparators, independently exercise full action/effect/royal/history/undo/replay/negative/property cases. Native resource accounting must be measured;107 replay-retention/cancel/inspection risks are measurement obligations, not automatic refactor gates or inherited Chu constants.79 consumes actual handoff;44retains engine/console/notation/kanji/playable delivery. No implementation plan or activation is added by83.

Ready refinement completed by83: independent handoff/contract review confirms one bounded direct native task is appropriate, no children useful now under owner instruction; originalAC4planning obligation supported by docs/m4/tenjiku/handoff.md. RemainingAC1–3/native assertions/DoD staypending; unassigned, no implementation plan or implementation started. Dependency83Done, no dependency changes.

Startup4November2026: clean primary codex/rewritea7f9205 after unrelated historical-access documentation integration; origin configuredgithub:jweathers777/mShogi.git, existingfourotherworktrees preserved. No InProgress/InReview owner; @codex sole integrator. PinnedBacklog1.55.1 verified, global1.55.0 not used for writes. Both ratified profile/corpus data remain fixed. Isolated native local-state/rules and record/diagnostic file ownership may be delegated; root owns Game/integration/conformance; fresh review remains read-only and independent. No87child hierarchy,79/44implementation or historical research campaign.

Current evidence supersedes intermediate progress statuses. Implementation 5786e59 and tested LF-checkout correction 9749eec0ffdbfffbcc040b6a56c5bb2c90616a1f are integrated and pushed to codex/rewrite. The public mshogi_core::tenjiku boundary owns both explicit profiles, 156-piece setup, 44 faces, 30 promotions, complete actions and ordered effects, inspection, Game/history/adjudication, strict schema 1 records, exact reversal and bounded full-Game perft. AC3 is supported by the actual external public-core consumer and stable native contract for MSH-79, within the owner's explicit MSH-87 scope; no shared application implementation is claimed. See docs/m4/tenjiku/native-v1.md and the single structured native-verification.json ledger.

Frozen corpus and profile hashes are unchanged. Each profile passed 204 unique native rows and 621 expected-field assertions, with zero failures. The 129 annotations, 17 future consumer rows and 25 data-admission tests are excluded from native execution counts. Native 41 focused tests and two external consumer tests pass. Local core/conformance regressions passed 373 tests, with zero failures and two existing ignores; format, workspace all-target Clippy and the local macOS standalone Tauri host check pass. Independent review approved rules, effects, history, strict replay, adapter independence, capacity, consumer contract and final evidence. Root/counter/error-typing and undo ending-authority findings were resolved. Current 18-file and prior 17-file source manifests match Git; all five measurement records match raw outputs.

Native limits are 1 MiB, 1,024 commands/events and 512 million deterministic units, including widest actual Stop → Resign → Stop reserve across eight actor combinations and runtime counter headroom. Both profiles complete 128 native plies with all 156 pieces. Both 1,021-event control histories refuse atomically, preserve the accepted record/context, then complete all three endings at 1,024 events and replay exactly. Adams also completes 1,021 board plies: 557,612 encoded bytes, 299,287,458 units, 3.264-second replay and 40.3 MB peak RSS for the entire measurement process. Ordinary replay drops private undo handles; the explicit 128-handle vector occupies 5,120 inline bytes, excluding Game history/allocator. Logical units exclude some transient capacity-validation CPU. Empirical byte/work edges, worst-case latency and whole desktop document budgets remain unmeasured; no 15-second desktop guarantee is claimed.

Required Rust campaign 41816309261 passed every step on Ubuntu 24.04, macOS 15 and Windows 2022 at exact source 9749eec. The initial 5786e59 Windows hash failure came from CRLF checkout; LF attributes now preserve the reviewed bytes and a negative CRLF case still rejects. Its other jobs were cancelled. The corrected campaign's first Ubuntu attempt hit the existing MSH-112 early-exit runner race in unchanged runner.rs:172. That failure is retained and appended to MSH-112; an Ubuntu-only retry on unchanged source passed. No assertion, timeout or runner behavior changed, and MSH-112 remains unfixed. Automatic Desktop packages campaigns were cancelled as outside native scope. Final independent acceptance APPROVED the successful receipt, all required steps, source manifests, reused measurements and AC1–3. Original AC4 is preserved. No further source campaign is needed for documentation-only closure.

MSH-79 is Ready, dependency-satisfied, unassigned and unstarted. It owns explicit selected-profile New Game, typed shared routing/session/controller, native-derived complete choices/effects/royals/history, whole-document/branch/observation admission, ending/Close reserves, installation/cancel races and frontend interaction. MSH-44 retains search/evaluation, console/notation, actual engine journeys, traditional-kanji assets and final 16×16 playable delivery. MSH-112 remains To Do. No normal activation, 79/44/M5 implementation, new hierarchy or research campaign was started. No task worktree was created; the four unrelated worktrees are preserved. Per-command authenticated HTTPS handled unavailable SSH and transient GitHub push failures without changing the remote or force-pushing. Stop after MSH-87.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered the complete authoritative native Tenjiku kernel for both immutable AdamsModern and MullerCompleted profiles on codex/rewrite: implementation 5786e59, exact tested source 9749eec0ffdbfffbcc040b6a56c5bb2c90616a1f. State, complete actions, ordered Fire Demon/special effects, Game/history/royal/repetition endings, strict schema 1 records, exact undo/redo, inspection and bounded perft now form the public native handoff.

Each profile passed 204 native rows and 621 expected-field assertions, zero failures; 129 annotations and 17 future consumer rows are separately accounted. Native 41 tests and external consumer two pass. Local core/conformance regressions: 373 passed, zero failed, two existing ignores. Format, workspace Clippy and macOS standalone host check pass. Independent review approved source, conformance, capacity, consumer and final evidence. Rust campaign 41816309261 passed all required Ubuntu/macOS/Windows steps at exact tested source; the fixed LF-checkout failure and unchanged-source retry of the existing MSH-112 race are preserved honestly.

Native admission is 1 MiB / 1,024 events / 512 million logical units with actual Stop → Resign → Stop and runtime headroom. Both profiles passed representative play, atomic limit refusal and real ending replay. Adams 1,021-ply replay measured 3.264 seconds for 557,612 bytes; process peak RSS was 40.3 MB. Worst-case byte/work edges and desktop latency are not established.

The API, measurements and exact evidence are in docs/m4/tenjiku/native-v1.md and native-verification.json. MSH-79 is Ready for shared selected-profile/session/frontend/whole-document integration, but unstarted. MSH-44 retains engine/search/console/notation/kanji/final playable delivery. MSH-112 remains To Do and unfixed. All original AC and DoD are supported; AC4 is preserved. No normal game activation or packaged GUI acceptance is claimed. Stopped after MSH-87.
<!-- SECTION:FINAL_SUMMARY:END -->
