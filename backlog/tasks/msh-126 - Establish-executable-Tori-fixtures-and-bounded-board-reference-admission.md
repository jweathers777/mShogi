---
id: MSH-126
title: Establish the self-contained executable Tori conformance fixture harness
status: Done
assignee:
  - '@codex'
created_date: '2026-11-01T12:21:00Z'
updated_date: '2026-11-01T14:29:00Z'
labels:
  - m4
  - owner-msh-85
milestone: m-4
dependencies:
  - MSH-81
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
  - docs/m4/tori/comparator-audit.md
  - docs/testing/conformance-policy.md
  - docs/m4/tori/harness.md
type: task
ordinal: 126000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Self-contained executable fixture infrastructure owned by coordinating MSH-85. Selected mshogi.tori.modern.v1 and the103 independently established specifications/provenance remain fixed inputs from DoneMSH-81. The owner2026-11-01 explicitly withdraws the former mandatory Fairy-Stockfish build/admission gate: conformance must work without another engine. Preserve that prior rationale/audit as historical optional investigation, never rule authority. This task implements strict data admission, assertion diagnostics/accounting and reusable native property/metamorphic interfaces;127-129 later supply real native observations. No Tori rules kernel, external engine build, shared runtime or UI is implemented here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A self-contained bounded executable harness loads and validates all103 selected mshogi.tori.modern.v1 fixture IDs, pinned corpus identity/hash, source provenance/independent expectations and complete ownership manifest. Duplicate, missing, unknown, malformed or conflicting definitions reject; expected data is never generated from implementation or external-engine output.
- [x] #2 Usable native adapter/observation interfaces separate geometry, transitions, local legality, history, records and consumer assertions, retain actual native owners, and report executed/passed/pending/failed accurately. Unimplemented assertions remain pending, not passing; bounded failure diagnostics contain fixture ID, input context, expected and observed values. No external game engine or engine-specific runtime/build system is needed for ordinary tests/conformance/CI.
- [x] #3 Focused deliberate observations/mutations prove detection of missing/extra legal actions, wrong destinations, capture effects, promoted/demoted identities, hand ownership/counts, history identity and malformed/conflicting fixture data. Negative CLI/API paths fail usefully; no second Tori rules engine, fabricated perft or conformance pass is introduced.
- [x] #4 Executable reusable property/metamorphic interfaces support authoritative-state apply/undo, rejection immutability, material/face/owner effects, replay equivalence and valid180-degree rotation/side exchange. Meaningful harness-boundary deterministic/generated/negative/metamorphic tests run now; actual native properties requiring127-129 remain explicitly pending and expected data/provenance unchanged.
- [x] #5 Durable projectwide testing policy and current downstream task/doc requirements make independently expected rules/history/regression/reversal/replay/property/metamorphic/adversarial tests and independent review mandatory, with external engines optional/nonauthoritative and outside normal build/test/package/release dependencies. Historical comparator audit/results retained.126 completes through focused tests, independent review, actual integration/push and applicableCI; no future native/product task starts.
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
1. RootCodex integrates from clean primarycodex/rewrite0422504 using pinnedBacklog1.55.1; preserve otherworkers/worktrees. Record owner supersession of mandatory externalcomparator gates in durable policy and task history before implementation.
2. Add a development-only Rust workspace harness with strict duplicate-preserving bounded loading, pinned103-ID corpus/provenance and exact ownership checks. Keep selected profile/fixture/ownership bytes unchanged; no nativeTori engine or rulesimplementation.
3. Implement explicit assertion observations, geometry/transition/local/history/record/consumer separation, missing/extra diagnostics and honest executed/pending/failed accounting, plus reusable snapshot/property/metamorphic interfaces. Deliberately constructed observations test machinery, never count as native conformance.
4. Exercise malformed/mutated data and all requested wrong-result families; run focused harness tests/format/lint and applicable workspace/nativeCI. Obtain fresh independent review, resolve materialfindings.
5. Finalize revised126only after supportedAC/DoD, normalintegration/push and actualCI evidence. Leave127/128/129/85/77/42 and othervariants unstarted.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 withdraws mandatory Fairy-Stockfish build/admission and all engine-oracle gates. Original tasktitle/5AC at0422504 are preserved in Git/history and documented supersession, not completion evidence. No Fairy/cshogi/Pikafish/python-chess build or invocation authorized/needed. Selectedrulesprofile and independently reviewed103expectations/provenance remainauthority. Explicit projectwidepolicy now requires independently expected fixtures/history, deterministic/property/metamorphic/negative/reversal/replay tests and independentreview, with optionalnonauthoritative externalinvestigation only. Dependenciesunchanged126→Done81; futurechain127→126→... remains. ActualownerrootCodex primarycodex/rewrite; startupclean/noactiveotherowner; no taskworktree needed.

Implemented engine-independent Rust development crate mshogi-conformance, embedded immutable103-ID corpus/hash/provenance/ownership, recursive duplicate-preserving boundedJSON admission, actualowner-qualified observation projections and sixboundaries. DefaultactualCLI:103fixtures,0executed/pass,727pending,0failed,4documentaryannotations,83pendingconsumerrequirements; no nativeconformance claim.18focusedtestsPASS, packageclippy-DwarningsPASS, workspacefmtPASS. Deliberatewrongactions/destinations/capture/promotion/demotion/quail/hand/history/malformed cases detected; 2000generatedsnapshotmutations,49square transforms,35authoredrotationpairs andactualclosurebadundo/mutatingreject tested. Independentconformance126_review PASS afterownerexchange/supplementalfacets/terminalclassification corrections; actualcommandsandhistoricalinitialfailures in docs/m4/tori/harness-evidence.md. Selectedprofile/corpus/manifestunchanged; full129task344edgegraph/doctor/links/deps/futureplans/completedtaskscheckPASS. No externalenginebuilt/run; normalnativeCIpendingactualsourcecommit. Current126ACexplicitlyrevisedunderownerdecision;128AC5externalgateoptional;otherfuturetasksnotes/policycurrentwithoutactivation orplanrewrite.

Firstreviewedsourceintegrated/pushed134f81ba67ef81918734ed1ed1acb5de7532fb99. NativeCI41314482706 Windowsjob124871524762failed13of18harnesstests with corpusSHA mismatch after format/lintpassed. Scoped.gitattributes LF rules forthe2auditedJSONfiles correct checkoutidentity without changingbytes/hash/comparison/tests. ActualWindows-styleGitexportmatchesoriginal141871BcorpusSHA3f9e... and85731BownershipSHA044a...; independentreviewerrepeatedfreshprobePASS. ActualWindowsCIrerunstillrequired; no passingplatformclaimfromlocalprobe. Existingfailurepreservedinharness-evidence.md. TwoactualconstructedCLInegativeprobes exit1 withusefuldiagnostics recorded. Noexternalenginebuilt/run; taskremainsInReview pendingfinalnativeCI.

FirstCI134f81b completed:LinuxworkspacePASS;Windowscorpusbytehashfailure correctedviaLFattrs;Macpassednewharness butlaterunchangedrunner shogi_runner.rs355 missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove failed LateReplyvsResourceLimit. This is distinctfromMSH112earlyexit test; no sharedcause orrunnerfixclaim. Exactoutcomesin evidence/harness-first-ci.json. Finalsource d5f98a2af9e38ae690e2fd5c4940a4d26d12407b nativeCI41315037374running, LinuxworkspaceTeststepalreadyPASS; otherplatforms notyetclaimed. No furthercodechanges orunrelatedtaskstarted.

Final integrated/pushed source d5f98a2af9e38ae690e2fd5c4940a4d26d12407b: native CI41315037374 PASS Ubuntu24.04/macOS15/Windows2022 including workspace tests, format/lint, QA constructors, snapshot consistency, build and console smoke. Exact job metadata in docs/m4/tori/evidence/harness-final-ci.json. Earlier Windows hash failure and unchanged macOS runner failure preserved; passing rerun does not claim a runner fix. Separate independent review PASS; local18 focused checks and metadata integrity/doctor PASS. Final task/document bookkeeping only; no new GUI evidence or external engine invocation. All five revised AC/four DoD supported. MSH-127 is next Ready, no downstream task activated.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered self-contained Rust mshogi-conformance CLI/library and binding six-game engine-independent testing policy. Strict bounded duplicate-preserving data admission pins all103 independently expected Tori fixtures and owner manifest unchanged; usable field/owner-qualified native adapters, useful negatives and real closure-based property/metamorphic interfaces avoid a second rules engine. Actual default report:103 data fixtures admitted,0 native assertions executed/passed,727 pending;4 documentary annotations and83 later-consumer requirements remain pending.18 focused tests, lint/format, separate independent review and final three-platform native workspace CI41315037374 PASS on tested/integrated source d5f98a2af9e38ae690e2fd5c4940a4d26d12407b. Earlier failed CI preserved; LF checkout fix maintains audited bytes. Mandatory comparator gate withdrawn by owner; external engines optional/nonauthoritative, none built/run. Historical research/reviews preserved, downstream requirements reconciled without dependency/profile/expectation changes. Full evidence and commands: docs/m4/tori/harness-evidence.md; invocation/interface: docs/m4/tori/harness.md; project policy: docs/testing/conformance-policy.md. Integrated/pushed codex/rewrite. Native conformance awaits127-129;85 final acceptance/77/42/other variants remain open and unstarted;127 next dependency-ready. M4 remains open.
<!-- SECTION:FINAL_SUMMARY:END -->
