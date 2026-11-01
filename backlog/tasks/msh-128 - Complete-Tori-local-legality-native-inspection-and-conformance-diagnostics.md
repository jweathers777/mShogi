---
id: MSH-128
title: Complete Tori local legality native inspection and conformance diagnostics
status: Done
assignee:
  - '@codex'
created_date: '2026-11-01T12:21:00Z'
updated_date: '2026-11-01T17:40:00Z'
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
  - docs/testing/conformance-policy.md
  - docs/m4/tori/native-local-evidence.md
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
- [x] #1 Public complete local legal choices and checked apply use native geometry, resources, compulsory promotion, king safety and exact drop restrictions for both sides; no missing/extra actions or duplicate choices for identical hand copies. Check evasions include every admitted move/capture/drop, and pinned-piece threat geometry follows T05. Royal captures/self-check/wrong-turn/basic illegal actions reject atomically. Applying a local choice does not claim permission after a concluded/history-restricted game.
- [x] #2 Admitted local roots enforce nonmoving Phoenix safety, nonadjacent royals, no stranded last-rank Swallow and at most two own unpromoted Swallows/file, in addition to B structural checks. Standard and custom/imported roots stay distinct from proven reachability. Local status truthfully reports ongoing/check/checkmate or quiet no-action; D owns installing the corresponding native game conclusion.
- [x] #3 Swallow drops reject third own unpromoted/file, last rank and direct Swallow-drop mate; friendly Goose/enemy Swallow do not count. Full defense tests distinguish forbidden immediate mate from legal escape/capture, and use board-legal defenses before repetition sanctions. Other-kind mate drops and permitted last-rank drops remain available. Every originally assigned local admission case and its owner rotation is executed; transition effects and exact reversal from B are rerun through public local admission.
- [x] #4 Native immutable inspection yields complete locally legal move/drop choices for a selected piece/hand owner on a private hypothetical turn, with handed kind/forced-result face and authoritative geometry. Both orientations/owners and board/hand choices are checked, including enemy pieces; inspection never mutates state/history, grants live admission, starts work or duplicates movement in TypeScript. MSH-77 later projects these facts into presentation metadata.
- [x] #5 Independent complete-choice fixtures, deterministic sequences and apply/undo checks establish local conformance, including the two reviewed root-terminal positions, checks/pins/evasions and drop-mate/defense pair. Meaningful deterministic, property, metamorphic and adversarial native tests must establish behavior independently, including drop-mate. External comparisons are optional investigative checks only and never a conformance/CI/completion dependency. Provide deterministic perft/divide diagnostics and expected counts only where independently established; mismatches retain root/action path and reviewed resolution rather than changing expectations to match an engine.
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
1. Root Codex is sole builder/integrator in clean primary codex/rewrite4710a4d; pinnedBacklog1.55.1 and origin alignment verified, Done127 dependency and no active other task owner. Preserve unrelated worktrees, existing games, profile/103fixtures/manifest and completed evidence.
2. Add one public LocalPosition boundary admitting structural Position only after T07 royal/drop-root checks. Reuse private127 geometry/effects/transitions; expose checked complete legal choices/apply/reversal, native attack/check/status observations and explicit capacity errors. Keep structural helpers crate-private and full Game permission/history/records129 unimplemented.
3. Implement exact Swallow file/last-rank and direct-drop-mate rules. Test complete safety-valid defender actions; direct adjacent Swallow check admits no drop evasion, so mate validation has no recursive drop-mate cycle. Use capacity-neutral private rule probes for hypothetical defenses/inspection, preserving actual counters/provenance and honest capacity failures at live admission.
4. Add immutable native board/hand inspection for either owner, complete actions/effects on a private hypothetical turn; deterministic bounded perft/divide with explicit work errors and no invented counts.
5. Extend actual126 adapters to all281 planned128assertions while preserving202127passes and honest pending history/record/consumer work. Recheck40 accepted board transitions through public local admission, all negative/root/terminal/drop-defense cases; add independently hand-enumerated complete action sets, threats/pins/check evasion, rotation, material/reversal, rejection, inspection and capacity tests.
6. Obtain separate independent review, resolve findings, run focused conformance/native and relevant existing-game regressions, format/lint and3platform nativeCI. Record actual source/results/limits, finalize128only after supportedAC/DoD, integrate/push and stop before129/85final/77/42.

Board-rule query clarification: legal_actions/is_legal/status and hypothetical inspection use private counter-neutral copies, consistent with existing native board-query semantics; actual checked apply preserves declared counters and returns explicit Capacity on unrepresentable work. Queries never infer zero choices or a game result from counter exhaustion.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

Owner request2026-11-01 authorizes128only. Clean primary4710a4d aligned withorigin, no InProgress/InReview task owner at startup; existing111/100/98worktrees preserved.127Done sourcece1801d supplies fixed structural foundation. Root/safety/drop local admission differs from129history/terminal-command permission. No externalengine orGUI campaign authorized/required; no productselector/capability/runtime changes.

Implemented one public LocalPosition admission/legality/inspection API over127 structuralPosition. Native geometry/effects/reversal reused; all T07 local roots, T05 geometric threats/royal safety and T04 Swallow rules enforced. Direct checking Swallow drops examine all safety-valid responses without recursion; any defending drop necessarily leaves the adjacent check. Actual126adapter483executed/483PASS/244pending/0failed, including202existing127 and281new128 fields;83consumerrequirements pending.40accepted board-transition fixtures pass through checked local apply/undo/redo; history/records/conclusions remain129pending. Focused25Toriunit +1externalpublictoken +2ToriAPI/doctestsPASS, noToriignored. Independent tori128_review PASS, no materialfindings; optional publicwrapper stale/clone/branch test recommendation addressed and independently rerun. Meaningful source-derived complete-choice/pin/check/drop/capture/root/rotation/reversal/property tests, boundedperft3/9hand-derivedcounts and actualfailpaths covered. Initial focus run had unusedimportwarning; removed before final lint. No fixture/profile/manifest/selectedrule change, no externalengine orGUI. Fullcore/conformance regressions and nativeCI pending; taskInReview until actualintegration/evidence.

Final local verification:262core/conformance checksPASS across33reported suites,0failed;2existing release-only deepChess opt-ins notrun (deep_independent_divides_match_in_order and deep_fixed_seed_sequences). Focused25Toriunit/1externalpublictoken/2ToriAPItestsPASS; all127structural tests retained. Deterministic legal sequence actually192moves twice with exactreverse/redo at everyprefix. Final all-targetcore/conformanceClippy-Dwarnings and workspaceformatPASS. Independent reviewer finalPASS independentlyreran25+1+2tests and reviewed source/API/fixture logic, no remainingfindings; exacthashes in native-local-fixtures.json. Fullgraph129tasks344edges, deps, oldDone records/otherplans, selectedprofile/corpus/manifest and existinggames/consumers/workflows/dependencies unchanged;127geometry/transitionsemantics byteunchanged, onlyEffectspublicdata+testhelpervisibility adapted. Readyfor reviewedsourceintegration and3platformnativeCI; no later task started.

Reviewed source integrated and normally pushed to codex/rewrite at5c60ac65dc03a7f9afcaa796a2babb252329d361. Native Rust workspace CI41331055051 dispatched for exactsource. Desktop packages workflow also triggered automatically but is outside this native-only task; no Tori GUI/product claim follows. Source and reviewed hashes unchanged; taskInReview pending actual CI. No later task or speculative shared infrastructure started.

Final native Rust workspace CI41331055051 PASS at exact reviewed/integrated/pushed source5c60ac65dc03a7f9afcaa796a2babb252329d361 on Ubuntu24.04/macOS15/Windows2022: format/lint, workspace tests, QA constructors, native snapshot fixtures, build and console smoke. Actual metadata in docs/m4/tori/evidence/native-local-ci.json. No CI source correction required, no native assertion failed. Independent tori128_review finalPASS;25unit+1externalpublictoken+2ToriAPI tests and262core/conformance regressionsPASS (2existing deepChess opt-ins notrun). All original5AC/4DoD supported; final completion docs change no code. Selected profile/103fixtures/manifest, deps and oldDone records preserved. MSH-129 now first dependency-ready, unassigned/unstarted;85final/77/42/futurevariants remainunstarted and M4open. Stop after128.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered one authoritative public tori::LocalPosition boundary over127 structuralPosition for mshogi.tori.modern.v1. Complete native choices/checked apply reuse geometry, capture/demotion/hand resources, compulsory promotion and exact local undo/redo. T07 root admission, pinned geometric royal threats, check evasions and full Swallow file/last-rank/direct-drop-mate rules are enforced without recursive mate evaluation. Local status reports Ongoing/Check/Checkmate/QuietNoAction, not match conclusions. Immutable board/hand inspection uses a private owner turn and actual complete actions/effects, never live permission. Board queries are counter-neutral; commits retain explicit Capacity and atomic failures. Perft/divide are explicit bounded diagnostics with native failure paths and only independently justified counts. Actual126 adapters483executed/483passed/244pending/0failed across83rows (202127+281128);83later-consumer requirements remain pending. All40 accepted board transitions execute through public local admission.25Toriunit/1publictoken/2ToriAPItests and262core/conformance checksPASS;2existing deepChess opt-ins notrun, noToriignored. Independent read-only semantic/source/test reviewPASS, publictoken suggestion addressed. NativeCI41331055051 PASS on3platforms at tested/integrated/pushed5c60ac65dc03a7f9afcaa796a2babb252329d361. Commands/review/coverage: docs/m4/tori/native-local-evidence.md. No profile/expectation/dependency/existing-game/consumer change; no external engine orGUI. Full-game history, repetition, commands, conclusions, canonical replay and dispatch remain129; finalnative85 and consumer77/42 remainopen.129nextReady; none started, M4open.
<!-- SECTION:FINAL_SUMMARY:END -->
