---
id: MSH-82
title: Reconcile adopted chu shogi and historical rules profile
status: Done
assignee:
  - '@Codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T21:51:00Z'
labels:
  - m4
  - owner-msh-43
milestone: m-4
dependencies:
  - MSH-62
references:
  - >-
    backlog/tasks/msh-43 -
    Deliver-complete-chu-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/m4/research/chu.md
  - docs/testing/conformance-policy.md
  - docs/m4/chu/profile.md
  - docs/m4/chu/decisions.md
  - docs/m4/chu/sources.md
  - docs/m4/chu/fixtures.md
  - docs/m4/chu/handoff.md
  - docs/m4/chu/review.md
type: task
ordinal: 82000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-43. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Primary-source clause review and independent profile/fixture assessment; unavailable texts are acquisition blockers for that game,not for unaffected standard-shogi work.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A bounded source acquisition/reconciliation record resolves JCSA2019 versus2004 supplement,historical evidence,lion restrictions,compound actions,promotion timing,king/prince status and capture-based endings,with primary editions,translation limits and adopted rules versus proposed/software conventions explicit.
- [x] #2 Material owner choices have concrete sourced options/recommendation and recorded decisions before affected behavior; unresolved required rules remain blocking named obligations,not silently omitted.
- [x] #3 A named/versioned complete profile and initial independent fixture expectations cover state,complete actions,history/conclusions and notation/consumer constraints without importing chess semantics.
- [x] #4 The native consumer handoff and necessary shared extension scope are refined into actionable acceptance/dependencies without requiring another game final delivery or closing this game.
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
Bounded independent chu shogi source-research pass only: inspect existing scope/inventory, acquire exact primary editions/facsimile locators where available, separate actual text from catalogue/translation/reconstruction/software, and identify concrete unresolved clauses, independently expected fixture seeds and owner decisions. No rules adoption, product implementation, source-profile changes, dependencies on MSH80, institutional messages or game delivery closure. Research worker msh82_research supplies isolated findings; root@codex is sole durable integrator on primary codex/rewrite. Report unavailable source as a named gap; completion is assessed against original AC, never inferred from this partial pass.

Modern Chu profile acceptance and direct native handoff, sole integrator Codex/root in clean primary codex/rewrite at78f8d6c; MSH62 Done, no competing MSH82 owner. Pinned Backlog1.55.1 resolves installed1.55.0 mismatch before mutations. 1. Reuse existing C1/C2/C3/H1 acquisition and inspect current JCSA2019,2004 supplement and movement/setup diagrams; independently reconcile ordinary movement/promotion, Lion compound restrictions, royalty, repetition and endings. Distinguish modern adopted text, historical differences, translation limits and software conventions; no historical-facsimile prerequisite. 2. Write named/versioned complete candidate/adopted profile with exact clause decisions and substantial independently expected machine-readable fixtures; no native Chu rules implemented. Record any material unresolved gameplay decision with alternatives/recommendation/distinguishing fixtures, blocking only affected ratification. 3. Inspect current native Game/action/history/record, execution, document/runtime/presentation/choice contracts. Refine MSH86→MSH78→MSH43 coherent implementation handoff under unchanged ownership/dependencies; no universal framework or new task hierarchy. 4. Separate read-only source/profile/fixture/native-contract review in parallel with documentation/data work, then resolve/review substantive changes. Required checks: source/provenance/schema/corpus negative/coverage/links/Backlog-DAG/diff; native assertions remain pending until86. No product tests, GUI/platform build campaign required because no production changes. One structured verification record per source checkpoint. 5. CLI check only supported AC/DoD, normal commit/push; if material owner choice remains, leave82 unfinished and report exact decision. Preserve prior evidence, four delivered games, M4/43 open; stop after82. No Chu implementation, Tenjiku or M5.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-10-27 permits independent research in parallel with primaryMSH80 only within this existing task. Provisional delivery order unchanged; no blocker from this game is placed on unaffectedXiangqi. Root@codex owns record/integration; delegated msh82_research researches in isolated /private/tmp output only. No substantive game implementation authorized.

Bounded independent research2026-10-27 by msh82_research, root soleintegrator: acquired currentJCSA2019general/2004supplement/movement texts; directly inspected NDL1915雑芸叢書第一 pp212–217 witness DOI10.11501/1869566, not original1703facsimile.32sourceGETs hash/time/status recorded in chu-provenance.json. Setup/movement and compound-lion/promotion/royal seeds are preliminary translations, not ratified expectations. Royal self-exposure/no-action, repetition responsibility, material-win/immediate-recapture and edition/diagram review remain named obligations; no complete profile. AC1–4unchecked, no implementation/adoption/reorder/dependency on80. Research pass ended; To Do/unassigned pending these source/interpretation gates.

Partial independent research report/provenance integrated and pushed with MSH-80 planning at f4e564b66ccecca6d76cadea5a354bb1e366a77d on codex/rewrite. No criterion checked, no full profile/fixture or product completion; exact remaining acquisition/interpretation gates stay in this task and its report. No dependency blocks unaffected Xiangqi.

Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

Owner2026-11-02 redirects82 to complete modern playable profile and direct implementation handoff. Existing original4AC/DoD preserved. Historical acquisition is supporting evidence, not a delivery prerequisite where modern convention is sufficient. External comparators optional under accepted conformance policy. Native86/shared78/product43 retain ownership; no product implementation authorized by82.

Owner clarification2026-11-02 supersedes initial unintegrated CHU-P1/P2/P3 proposals: faithful capture-based JCSA, narrow immediate-threat/complete-response mate, enforce responsible side's must-vary before4 through native admission, narrow material with immediate reply and actual agreement draws. Withdraw automatic repetition forfeit, automatic bare-royal draw and quiet immobility loss; no code implemented them. docs/m4/chu/decisions.md preserves distinction from deterministic faithful interpretations. Independent source/diagram readers resolve last-own-opportunity timing, first actual jitto passer/equivalent paths and Prince-rescue/material reply fixtures. No remaining material owner question identified. Original4AC/DoD/dependencies intact.

Completed profile/data/handoff ready for acceptance2026-11-02. Three separate read-only agent reviews PASS: chu_lion_review directly reviewed source diagrams/all29faces/29rotations/18promotioncategories and independently enumerated171complete Ln88/SE41/HF42 actions; chu_endings_review checked all35C06–C08fixtures and source timing; chu_contract_review inspected actual current native/shared/runtime/frontend contracts, hand-counted7/42divide, and final diff/tasks/hash metadata. Material findings resolved; exact methods/anchors in docs/m4/chu/review.md. Corpus250rows=221authored+29rotated,240native+10futureconsumer,1141top-level expectedfields;0native executed/passed.17data-admission tests and audit validator PASS, product CI/GUI not applicable/not run. Final source checkpoint and completion evidence to follow in verification.json; no product or later-task implementation.

Exact tested/integrated source checkpoint df5083fdfdd840d0655b9f3ef725c54ccac641d2 on primary codex/rewrite PASS: validator250rows/0native;17negative/data-admission tests;27/27source local hashes and unchanged originalC1/C2/C3HTML;111relative documentation links;132task IDs resolved with no cycles/self-dependencies; original four-task AC/DoD wording and dependency lists unchanged; reviewed JSON delta status-only; backlog1.55.1doctor and baseline diff--check. All production files unchanged. Exact commands/environment/artifact hashes/review methods/failed probes/CI applicability in docs/m4/chu/verification.json. No product/platform CI or GUI campaign required/run for this profile-only task. MSH86Ready/unassigned;78/43ToDo; no other task closed. Final evidence/task-only integration commit follows; actual hash from Git history. Remote checked at baseline78f8d6c before normal push.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Ratified mshogi.chu.jcsa2019.v1, edition JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02, from current JCSA general/supplement/movement sources and preserved historical evidence. Complete12×12/original identities/29faces/18once-promotions/Lion routes+law/royals/native must-vary history/strict replay/notation contract; owner P1/P2/P3 direction adopted without novel automatic penalties/draws or Chess self-check. No remaining material owner decision.250 independently expected rows precede native implementation; three separate source/fixture/current-contract reviews PASS and material findings resolved. Original4AC/4DoD supported by review.md and single verification.json at tested source df5083fdfdd840d0655b9f3ef725c54ccac641d2. No native Chu execution claimed. Existing86→78→43 scope refined without changing dependencies or acceptance; next bounded assignment86 coherent native kernel/history/records with real conformance execution.43/M4 remain open; four delivered games/M2/M3 untouched. Integrated on codex/rewrite; final task/evidence commit and normal push recorded by Git.
<!-- SECTION:FINAL_SUMMARY:END -->
