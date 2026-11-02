---
id: MSH-86
title: Deliver the authoritative chu shogi native handoff for shared consumers
status: In Review
assignee:
  - '@Codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T22:45:00Z'
labels:
  - m4
  - owner-msh-43
milestone: m-4
dependencies:
  - MSH-82
references:
  - >-
    backlog/tasks/msh-43 -
    Deliver-complete-chu-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/testing/conformance-policy.md
  - docs/m4/chu/profile.md
  - docs/m4/chu/fixtures.md
  - docs/m4/chu/handoff.md
  - docs/m4/chu/native-v1.md
  - docs/m4/chu/native-verification.json
type: task
ordinal: 86000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-43. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

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
2026-11-02 current assignment: sole root integrator Codex on clean primary codex/rewrite at0e649b2,82Done. Use pinned Backlog1.55.1, preserve original4AC/4DoD and existing ownership. A: native Chu module state/base+promotion/complete ordered actions/derived discarded captures/strict roots/exact transition reversal. B: complete native generation/Lion law/read-only inspection/promotion with focused source-derived fixtures. Isolate movement builder from root-owned Game/records; agree actual small native API, no shared dispatch activation. C: root integrates full-history Game, pre-fourth last-opportunity obligations, finite mate/nonrecursive reply checks, narrow exhaustion/agreement/resignation/Stop, monotonic context, exact prefixes/undo and strict replay with precommit native capacity/reserved headroom. D: actual Chu corpus adapter in existing conformance infrastructure, every applicable240native row executed with field/annotation/future accounting; LegalAll88/41/42 and7/42divide, state/atomicity/rotation/sequence/adversarial/native consumer checks. Independent read-only review of completed movement/Lion and Game/record/adapter boundary; resolve and re-review material changes. Check core/conformance and affected production consumers/separate Tauri workspace early; focused tests first then coherent relevant workspace regressions/fmt/clippy. Required CI: Rust workspace ubuntu24.04/macos15/windows2022 at exact reviewed source; no broad packaged GUI campaign for native-only86. One structured evidence record and exact source refs; refine78handoff without implementing78/43, no native execution advertising. Normal integration/push/CLI finalization only with all evidence; otherwise retainInProgress exact blocker. Stop86;43/M4open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH82 modern-profile refinement2026-11-02: mshogi.chu.jcsa2019.v1 and docs/m4/chu supply complete sourced clauses,250 independently expected rows and direct native/shared handoff. Dependency82 unchanged; Ready is actionable scope, not permission to start before82Done. Original4AC/DoD preserved/unmet. AC4 refinement: no child split is useful now; one coherent native state→complete legality/Lion→Game/history/canonical-record batch with internal independent checkpoints avoids redispatch fragmentation. No extra dependencies. Required native work:12×12/original+once-promoted identities, ordered compound routes/derived discarded captures, capture-based royalty/finite mate, native pre-fourth last-opportunity admission, narrow exhaustion/immediate response/actual draw agreement, strict authoritative replay and exact reversal. Reuse generic conformance infrastructure with a real Chu adapter/corpus; current data validation is0native executed. Require complete LegalAll, all native fixture assertions,7/42 divide, deterministic sequences, meaningful properties/negative mutations/replay/undo, independent code review and applicable native CI. External engines/historical facsimile not gates; product GUI43/shared78 remain separate. No implementation plan prescribed or work started.

Implemented coherent nativeA–D checkpoint onprimarycodex/rewrite. Newgame-ownedChu state/actions/localrules/Game/history/records, no sharedGameKind/engine/console/UIactivation. Actualimmutablecorpusadapter:1076executablechecksPASS,240/240native rowsPASS,0failed/pendingnative;37annotations(including2LionexplanationstringswithseparateactualtypedLionLawchecks),10consumerrowspending78/43. Complete88/41/42sets,7/42divide,583actualchoiceowner-rotation/reversalchecks. Independentfullsource/profile/adapterreview and executed35nativefocusedtestsPASS; reviewfindingscorrected inclLIFOlogicalnodes,read-onlybudget,finite-replymetadata/work,forgedstandardprovenance,counterlessspeculation,dynamicactualencodedStop+Resignheadroom,longestactorpayload,andnativecache/rotatedanchorassertions. EarlyworkspaceandstandaloneTaurihostcheckPASS; workspaceClippyPASSafterordinarylocalstylefixes; coherentworkspacebroadtestsrunonce,finalexactsource/native3platformCIpending. Realcoreconsumer2testsPASS. Measurements:227moves132199bytes3979825unitsbeforetypedworkrefusal;510controlsthenStop+Resign512;3obligationclosingprobe1277bytescomputedfromactualevents. OriginalAC/DoDuncheckeduntilfinalevidence. One structurednative-verification.json recordsactualmethods/limits/corrections;82profile/corpus/researchunchanged.

Finalindependentreview chu_native_review PASS actual35Chu libtests+2externalcoreconsumertests,0failed,withsourcehashes recorded. No remainingmaterialsourcefinding; fullnativecorpus1076/240PASS,37documentaryannotations10futureconsumerrows. Reviewis separateagent fullactualsource+profile/datainspection/realtestexecution,notauthorreread. Coherentbroadworkspaceandnative3platformCI are completion gates pending, neverinferred. Native-v1 describesstableAPI/canonicalrecord1/realcapacitypolicy and unchanged78/43ownership.

RequirednativeCI source136f892 campaign41536948188 Windowsjob125591097581failed2frameworkcorpusadmissiontests beforecorenativerulesexecution: newChu auditedJSONnotpinnedLF, unlikeTori. Preservefailure;supersededcampaigncancelled/replaced, notclaimedpassed. ConcreteGitprobe reproduced unpinnedCRLF590654bytes SHA040da84d... versusreviewedLF563575bytes SHA545f23e7... .gitattributes nowpinsfixtures/profile/sourcesJSONLF; expecteddata/hashesandallcorenativeproductionunchanged. Frameworkhashdiagnosticnowreportsactual/expected/bytecountandCRLFnegativecase;5focusedframeworktests+independentreview/filterchecksPASS. Replacementnative3platformCIrequiredoncorrectedsource. Localbroadworkspacebehavior79binaries/855PASS3ignored thenRustdocE0463 afteroverlappingbuilds; originalcommandFAILretained. Focusedserialdesktopdocandallworkspace-doc rerunsPASSwithoutproductionchange. One evidenceJSONkeepspreciseprovenance ratherthanrelabellingthesechecks.

SupersededCI41536948188 Linuxjob125591097628 also failed an unchangedrunner test: process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored expectedUnexpectedExit butobservedMissingResponse/BrokenPipe beforeexitstatus. Neither runnerproductionnoritsassertionschangedby86; preservethisconcretefailureseparately, do notweakenorclaimresolved. Correctedsourcecampaignwillrununchangedregressionsagain; nativeacceptance remainsblockeduntilrequiredchecksactuallypass. Macoldjobcancelled ratherthanclaimedcomplete.

Final native API audit found that removing Deserialize alone left HistorySnapshot state/protection publicly replaceable, allowing invalid external state to reach local_position expect. Narrow correction makes trusted fields private with immutable getters; serialized record/snapshot payloads and rules unchanged. Independent read-only review PASS; reviewer independently ran two E0616 compile-fail checks and two external consumer tests. Root reran 35 Chu library tests, three core compile-fail doctests, two consumer tests, fmt and focused Clippy: PASS. Source19166e3 native campaign41538750357 cancelled as superseded by this real source correction; all-platform format/lint passed but no full campaign success claimed. Final corrected source requires its own three-platform native CI.
<!-- SECTION:NOTES:END -->
