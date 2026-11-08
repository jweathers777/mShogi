---
id: MSH-49.1
title: Develop the mature conventional Chess engine architecture
status: Done
assignee:
  - '@codex'
created_date: '2026-11-08T13:07:00Z'
updated_date: '2026-11-08T15:26:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-25
  - MSH-109
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/chess-conventional.md
parent_task_id: MSH-49
priority: high
ordinal: 135000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner direction2026-11-08 replaces weak-baseline acceptance chasing with development of a strong practical conventional engine. Existing7818c33 alpha-beta/positional work is a foundation, not a substantially complete modern engine. Deliver a coherent native search/evaluation/time-management architecture in reviewable engineering increments, researched from mature primary implementations. Preserve native Game ownership and accepted execution/application boundaries; do not introduce a second rules engine or speculative universal variant framework. Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The integrated conventional search package includes iterative deepening/PVS, aspiration re-search, history-safe bounded transposition reuse, staged TT/capture/quiet ordering with SEE/history heuristics, and bounded quiescence with native legal check evasions; exact diagnostic search remains available for independently solvable correctness comparisons.
- [x] #2 Guarded selective search is integrated coherently: verified null-move/zugzwang safeguards, late-move reductions with re-search, conservative futility/SEE/late-move pruning and bounded extensions. The researched inclusion or exclusion of more aggressive techniques such as ProbCut/singular extensions is justified by concrete correctness or profiling evidence, without per-technique formal strength campaigns.
- [x] #3 Traditional material/positional evaluation covers tapered piece placement, pawn structure/passed pawns, mobility/activity, king shelter/attack pressure, threats and endgame conversion with inspectable terms; independent arithmetic/position/metamorphic tests and measured incremental-state restoration support it. No trained evaluator is introduced.
- [x] #4 Efficient native search-state access, exact make/unmake and hashing/evaluation maintenance are profiled and verified; repetition, claims/counters, mate-distance scores, TT history/context separation and cancellation restore state exactly. Search-only hypothetical/null branches never become legal game actions, played history or native outcomes.
- [x] #5 Real UCI and supervised product execution provide reliable bounded thinking-time allocation, hard-stop responsiveness and truthful completed/selective-depth/PV/work telemetry through the existing protocol family with explicit compatibility handling. Stale observations, process failures and incomplete work retain existing honest semantics.
- [x] #6 Independent review and appropriate actual native platform/affected-consumer checks approve a substantially complete engine checkpoint. Correctness tests, tactical/endgame regressions, short profiling and cumulative match-time accounting address the changed risks; no strength-release claim is inferred from those checks. The checkpoint is ready for the separate approved unrestricted/equal-time milestone.
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
Primarybuilder/integrator Codex, primary codex/rewrite a4ad03a clean/origin-aligned; no activeowner, originaldependencies48/25/109Done; historical-access worktree untouched. One49.1assignment, no49.2formalstudy. Currentcandidate has exactalpha-beta/captureordering/taperedterms but noTT/PVS/quiescence/selectivity or internalChess timebudget; strict currentPVadmission counts nominaldepth. (1) Extend native search access only within authoritativeChessowner for efficient exact transitions/context/hash and explicit reversible analysis-onlynull probes; independently verify state/history/counters/claims/restoration. Implement inspectable classicalevaluation andmeasured incremental state withoutanotherlegality/adjudication. (2) Root builds bounded conventionalpackage: iterativePVS/aspiration, context-safeTT, stagedSEE/history/killers ordering, legal-evasionquiescence, guardedverifiednull/LMR/futility/SEEpruning/checkextensions. Keep exactdiagnostic search andtests; justify optionalProbCut/singularinclusion/exclusion from concrete risks/profile. (3) Integrate truthful selective-depth/PV semantics andboundedtimeallocation into existingUCI/supervisedexecutionfamily andnativeadmission, with explicit old-observation compatibility andexactlimit identities. Preflightallaffected consumers inclstandalonehost/frontendearly. (4) Focusedindependentoracles, nativestate/hash/evalundo+null/TT/history/claim/zugzwang/pruning/cancel tests, realprocess/currentproductchecks; boundedrepresentative tactical/profilebenchmarks andatmost900saggregate routinematches forentiretask, recordingallattempts; noformal/multihourstrengthstudy orcompetitiveclaim. Isolatedevaluator/nativeaccess/execution work maybedelegated; rootownssearch/integration, independentreviewearlyandfinal. (5) Coherentsourcechecks andrequiredactualRustLinux/macOS15/WindowsCI (onecompletionawaremonitor); desktopchecks appropriate to affectedtelemetry/host, noautomaticbroadpackagedGUIcampaign. Resolvefindings, completeall6AC/4DoD onlywithsupport, normalcommit/pushDone andhandoff49.2withoutstartingitsstudy.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ownership2026-11-08: @codex primarybuilder codex/rewrite at a4ad03a297801922d8c9d3a0fd660e05b0ab2ebe, no competing activeworker. Backlog1.55.1 matches projectbaseline. Readyscope isfull49.1coherentenginearchitecture, notoneoptimization orplanningphase. Ownerstrategyallowsroutinealgorithmchoices; preserveallrules/session/nativeprofiles andoriginalstoppedstudyevidence. Routine localmatchallowance900s cumulativetasktime starts0; ordinarycorrectnesschecks/profiling aredistinct, noformalstudy approvalinferred.

Coherent conventional architecture implemented: nativecachedSearchCursor/attackmaps/exactpersistenthistorykeys; incrementalinspectableclassicaleval; PVS/aspiration/4096entryTT/stagedSEE+historyordering/legalq/checkextensions/verifiednull/LMR/futility/LMP/SEE pruning; oldexactdiagnostics preserved. Time/selective metadata additivewithlegacyomissionbytes andexacttimedrequestidentity; actualUCI/internal/desktopworkusesnewengine,128depthceiling/20kvisits/1s liveplay, oldstoppedarchivesunchangeduntilexplicitResume. Independentfreshsource reviewapproved; fixesprotectnative terminal/claim children beforepruning andabandononlysyntheticCounterOverflow withrealgame/cancelrestoration; directtimedFullnative solverrefused. Fullworkspace1134pass/11intentionalignored; workspacealltargetsClippy pass; native17frontendfixtures byte-identicalrepeat; frontend568full +15affectedtime-caption tests/type/build pass; standaloneTauri hostcheckpass. Newcanonicalrecord docs/m5/evidence/msh49-1/verification.json +docs/m5/chess-conventional.md preservescommands/profiles/failures/coverage, sourceplatformandfinalacceptancepending. Shortrelease20kvisit/1s diagnosticscompleted4–6nominalplies; node/worktestsnotenoughforcompetitiveclaim. ProbCut/singularexplicitlyexcludedwithprofile/context-riskrationale. Routine matchbudget0/900sused; plannedsingleboundedtimedself-playpairaftercleansourcefreeze, no49.2study/oldstudyrun. RequiredRustUbuntu24/macOS15/WindowsCI atnextcoherentsourcecheckpoint; onecompletionawaremonitor, affectedUI/hostlocalchecks(no newbroadpackagedGUIcampaign). All6AC/4DoDremainuncheckeduntilactualsourceCIandfinalindependentevidencereconciliation.

Sourcecheckpoint normallyintegrated+pushed eb4601d0fd33e790ac9905309fb011400aeafb5c. Cleansourcereleasebuild frozen build.json, engineSHA4250eb9e..., runner4c8fef86.... Oneboundedtimedselfplaypair at150ms/20kvisits/max192plies consumed49.692403374705464s of900s taskallowance:1nativeconclusion+1plycapunscored, bothnativehistoriesstrictreplay, nofallback/stderr/truncation/process/cleanupfailure. No furthermatchbatches or49.2study. RequirednativeCI42489944021 sourceexact attempt1: macOS15job128759926440, Ubuntu24job128759926534, Windows2022job128759926539 allacquiredrunners/running. Singleboundedcompletionawaremonitor35903 (/private/tmp/msh491/monitor.py;ci/{run,jobs}.json;ci-monitor.log), no redundantcampaign/retry. UnrequiredDesktop42489943997cancellationrequested. Freshindependentread-onlydelta/provenanceaudit approvedeb4601d/code/hash/logs; finalplatform/ACacceptancereconciliationpending. Sourcechangesstop; remainingworkisactualCIandfinalevidence. Productioncheckoutcodeclean; docs/m5/evidence/msh49-1 iscanonicalrecord withallfailures/caps/receipts. LeaveInReview/uncheckeduntilsupported; no successorimplementation.

First nativeCI42489944021 completed: Ubuntu128759926534 andWindows128759926539 passed; macOS128759926440 passedRusttests/lint butfailedsnapshotfixturestep. Historicalreceipt/sourceeb4601d retainedcanonicalrecord/log. Rootcause: Inspect admission receipt acceptscommand, separatefileworker reconstructshistory; generatorpublishedViewing/liveevent2 beforeComplete/historicalevent1. Narrowexample-onlyfixwaitsexactioid usingexisting20scompletionbound andassertsprefix; noengine/product/rule/expectedfixturechange. Twofixedgeneratorrunsmatchchecked-infixtures; focusedexampletests/fmt/Clippy pass. Independentnarrowreviewandnew3OSnativeCI pending. Selfplayandallunchangedsourceevidence reusedatactualeb4601d provenance; noadditionalmatches.

Narrowfixturefixindependentlyapproved(no findings) andnormallypushed b8cc85167f85eb031830d5f5268c1f94520f8a25. Allengine/product sourcesidenticaltoeb4601d; onlyfixtureexample+evidencechanged. RequirednewnativeCI42492362033 attempt1 running Windows128767302041/Linux128767302175/macOS128767302193; onecompletion-awaremonitor53153/cacheci-fixed. UnrequiredDesktop42492362042 cancellationrequested. Oldnativecampaigncompleted, nooverlappingretry. Elevencompressedloghashes/selfplayhash/budget/sourceequivalence reconciled. AC/DoDremainuncheckeduntilnewsourceplatformcompletionandfinalindependentreconciliation.

Final acceptance: requirednativeCI42492362033 attempt1 exactsource b8cc85167f85eb031830d5f5268c1f94520f8a25 completedSUCCESS, Windows128767302041/Linux128767302175/macOS15128767302193 all13steps/job passed onactualhostedrunners. Singlemonitor53153completed, no retry/overlappingcampaign. Independentfreshread-onlyfinalreconciliation APPROVED all6AC+4DoD; no remainingtechnical/platform/ownerblocker. Canonicalledger preserves oldeb4601d failedmacsnapshot/healthyLinuxWindows receipts, correctedfixturemechanism, allsource/build/log/reporthashes, exact49.692403374705464/900s onepair (native mate+honestunscoredplycap), unchangedproductevidence. No formalstrengthstudy orcompetitiveclaim. Allchecksnow evidenced, allcriteria checked, Done. Finalchangesonlydocs/evidence/backlog, normalintegration/push follows; acceptedsourceb8cc851 andimplementationeb4601d remainexplicit.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered conventional Chess search, classical evaluation and bounded timed execution through the existing native, UCI, supervised engine and desktop architecture. Native rules remain authoritative; search preserves exact history, claims, terminal results and undo. Legacy documents and observations retain explicit compatibility handling.

Production implementation: eb4601d0fd33e790ac9905309fb011400aeafb5c. Corrected fixture generator and exact tested source: b8cc85167f85eb031830d5f5268c1f94520f8a25, integrated into codex/rewrite. Native CI run 42492362033, attempt 1, passed every step on Windows job 128767302041, Linux job 128767302175 and macOS 15 job 128767302193. Local verification passed 1,134 workspace tests (11 intentionally ignored), Clippy, affected frontend tests/type/build and standalone host checks. Independent source, fixture and final evidence review approved all six acceptance criteria and four completion items.

The canonical record is docs/m5/evidence/msh49-1/verification.json. It preserves historical failures, source/build hashes, profiles and the complete 49.6924-second development self-play pair: one native mate and one unscored ply cap, with both histories replayed and no process/cleanup failures. History-conservative TT reuse, bounded selective search/SEE and the static absolute-ply horizon remain documented approximations. These checks establish implementation correctness and integration, not measured playing strength.

MSH-49 remains unfinished. MSH-49.2 owns the later unrestricted, principally equal-time Stockfish comparison, with owner approval required for its material budget and comparison conditions. No formal study ran here. The final documentation/evidence/backlog commit integrates this completion summary normally; its actual revision is recorded in Git history and the completion report.
<!-- SECTION:FINAL_SUMMARY:END -->
