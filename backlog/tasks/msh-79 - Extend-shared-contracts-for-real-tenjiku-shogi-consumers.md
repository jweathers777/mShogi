---
id: MSH-79
title: Extend shared contracts for real tenjiku shogi consumers
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-04T20:34:00Z'
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
  - docs/m4/tenjiku/shared-v1.md
  - docs/m4/tenjiku/shared-verification.json
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
- [x] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [x] #2 Shared mechanisms accommodate 16x16 complete routes,profile-specific ordered derived effects,royal status and native history/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [x] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [x] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
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
A. Extend actual native/profile/schema/record/side/command routing with explicit Tenjiku profile binding and cancellable strict codecs; keep execution UnsupportedMode. Check full wrapper size and inner/outer profile equality. Compile root and standalone host consumers early.
B. Add one immutable Tenjiku payload and ActiveDocument adapter within existing transaction/file/install/lifecycle owners. Retain exact journal/observations/branches/prefixes, fresh stopped reopen and real Stop→Resign→Stop→Close reserves. Measure representative and near-limit whole documents before ratifying bounded aggregate work policy; preserve atomic refusal/dirty/runtime/prior-file state.
C. Extend shared native presentation and existing frontend owner/controller for16×16, both profiles/orientations, complete routes, stationary targets and ordered burn/death/promotion phases. Explicit Newprofile plumbing and verification route only; exclude Tenjiku from normal chooser and product Load, gate engine retry/analysis capabilities.
D. Execute12 applicable consumer rows with real native/controller/frontend cases for both profiles; keep5 MSH44 product rows pending and do not recount621 native assertions. Measure worker/inspection/serialization/render/cancel costs. Obtain independent checkpoint/final review, coherent five-game/frontend/host regression and exact-source required nativeCI(Rust Ubuntu24.04/macOS15/Windows2022) plus applicable Desktop/frontend/host checks. Record one structured verification ledger, integrate/push normally, update44handoff and stopafter79.

CI-only resume: reconcile both completed runs/all attempts and exact3a290 source; retry only each latest unsuccessful macOS15 job once in this session after the elapsed cooldown. Use one completion-aware monitor; no overlapping campaigns or healthy Linux/Windows reruns. If acquisition fails again, stop automatic retries, inspect supported provider retry/scheduling limits and preserve In Review with exact receipts. If both execute/pass, briefly independently reconcile all6platform receipts, finalize evidence/AC/DoD and update44handoff without starting44.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH83 two-profile handoff: preserve selected native profile/edition through New root, record/envelope, execution/observation and inspection/history. Existing shared_new_game game-only profile inference and side(NativeSide) hardcoded profile-qualified presentation keys need bounded extensions for two Tenjiku profiles. Immutable profile-to-document binding plus full native context protects tokens; adding a profile token field is an implementation choice. Both-direction same-board cross-profile rejection and intentional fresh New versus in-place mutation are separate expected consumers. Add real native-derived burn/royal/complete-route projections without frontend rules or second controller/envelope/shell. Measure whole retained record/observation/branch/history/install/ending-Close admission and inspection costs against actual87; no Chu default constants silently inherited. Remains ToDo until actual native87 handoff can be inspected; originalcriteria/dependencies unchanged, no implementation begun.

Actual87source handoff now reviewable in public mshogi_core::tenjiku; native-v1.md documents exact types/capabilities/schema1/profile identities and resource facts, native-verification.json holds actual executed evidence. Consume explicit Profile with Game/GameRequest/GameContext/GameRecord, complete Action/Effects/EffectPhase, LocalPosition inspection, royal/history/prefix/capacity metadata. Native1MiB/1024commands-events/512Mlogical units includes real Stop→Resign→Stop reserve; desktop whole-document/observation/branch/ending-Close admission and15second worker/inspection lock risks remain79 measurements. Ordinary replay drops private undo handles. Public-core consumer2 passes bothprofiles and forgedburn/profile/stale rejection; no shared native.rs/controller/envelope/frontend/engine activation implemented. Preserve originalcriteria/dependencies; Ready refinement awaits87finalCI/completion. No79 implementation plan or workstarted.

87Done actualnative handoff accepted at9749eec, requiredthree-platformRustCI41816309261passed allsteps; independent finalreviewapproved. Earlier ToDo/pending87 statements are superseded. Current dependencies69/72/73/78/87Done, concreteAPI/schema/actions/effects/history/resource contract and realexternalconsumer now available; Readyunassigned, noimplementationstarted and nofutureimplementationplan. Preserve originalcriteria/dependencies. NewGame must select one immutable profile explicitly; implement existingnative/shared routing/session/controller/envelope/completechoice/inspection paths using native-v1, measurewhole-document+observation/branch/endingClose budgets andworker/locklatency againstrealnativecases. Existing112race remains separatelyunfixed, not a dependency or excuseforweakenedchecks.44retains playableactivation/search/console/notation/kanji. Stop87handoff, do notstart79here.

Startup4November2026: clean primary codex/rewrite68915085be4bdefbf9591b7aa4ddd042c013f0f9, origin aligned via per-command existingauthenticatedHTTPS(remoteunchanged), Backlog1.55.1 matchesbaseline. NootherInProgress/InReviewtask owner. @codex primarybuilder/integrator; read-only native/shared/frontend maps establish one survivingcontroller/envelope/runtime/session/choice owner. Existing fourunrelatedworktrees preserved; usecurrentcheckout. Native87isDone andall79dependenciesReady-satisfied. Twoimmutableprofiles andfrozendata unchanged. Independent research identified profile inference, envelopewrappercapacity, stationarycapturegeometry and normalLoad gating seams; no materialproduct/rules ambiguity ornewtaskhierarchy. Clearly isolated implementation fileownership maybedelegated; root integratesallchanges and keepsfreshreviewread-only.

Checkpoints A/B implemented in the existing owners. Both immutable profiles pass native routing/engine UnsupportedMode and12 real controller/file tests; immutable whole-document tests pass9focused plus3serial release measurements. Measured provisional600M policy was rejected:28retained128-action contexts reopened only until15s deadline. Final160M admits8contexts(~152.43M+0.814Mendingreserve); preliminary5.5s is strictdecode+nativeGame restoration, not fullworker. Actual FileWorker/Open/Save/install/ending/Close release probe and native-derived frontend fixtures are underway. Independent B review passed12controller+1typed-observation tests; fixed completewrapper and nestedobservation capacity typing, cancellable status generation, and concludingStop wording. Five product obligations remain44. Required CI notyet run; task remainsInProgress.

Implementation and independent B/C source review are complete; review fixed typed nested observation capacity, redundant uncancellable status generation, concludedStop wording and Resume-after-Close retirement. Real53nativeDTOs exercisebothprofiles;50TenjikuFrontendTests and full495frontendtests pass. Coherentworkspace994tests passed; subsequent focused22Tenjikucases plus correctedclose1pass, nestedcapacity1pass, nativeenvelope11pass; root/hostClippy and18QAhosttests pass. Actualreleaseworker Open/install~5.55s, Save~5.32s, endingoperations~3.4s, prepaidClose<0.5ms; bothprofile finalreopen~5.6s at~153.25Mwork passes. FixedmacOSQA identity reused foractualAdamscompound/cancel/focus/keyboard/flip, freshMullerNew, normalfivechooser exclusion, repeatedQuitCancel/Resume andMullerstationarycapture. Exactlegacywriter modes reproduce51fixtures byte-identically; new53 deterministic. One structuredledger shared-verification.json records failures/scopes; requiredexactsourceRust/DesktopCI pending. No44/M5started.

Reviewedimplementationb459250 integrated/pushed as3a290024df82d23f215165707c81de4928ad3a4a (identicalsource tree). InitialCLIpush lackedworkflowOAuthscope; alreadyconnectedGitHubapp madeexactreviewedworkflowonlycommit99447c6[skipci], thenordinarymerge/pushpreservedhistory; noauth/accesschange. RequiredRust+DesktopthreeplatformCI launched; taskInReview pendingactualresults. AdditionalnormalhostClippy and9tests pass;11fixedmacOShelpertests pass. Sourcehandoff44remainsunchanged andunstarted.

CI41856238536 Ubuntu job126665514599 failed known112early-exitclassificationrace at mshogi-runner/tests/runner.rs:172: MissingResponse(Brokenpipe,exitNone) versusrequiredUnexpectedExit. Preservedrawexcerpt docs/m4/tenjiku/evidence/ci-linux-rust-attempt1.txt; noassertion/sourceweakened. GitHub403defersjobrerunwhileotherRustjobsrun; waitthem, retryonlyfailedLinuxjobatsame3a29002. Desktop41856238041continuesactualfivegamepackagedjourneys; healthyjobsnotcancelled. Independent evidenceauditreconciled224sourcehashes/frozeninputs/51oldDTOs/12consumerrows/rawmeasurements/GUIartifacts; correctedhistorical600M, activeCI andprettyJSONlabels.

Finalindependentboundedverdict: AC1–3supported; AC4sourcefixtures/interaction/ownershipsupportedbutrequiredcross-platformclosureincomplete. DoD2review/findingsresolved supported; docs/handoff andintegration recorded; fullacceptance/DonegateNOTsupportedwithoutmacOS15terminalsuccess. FirstMacjobs126665515114/126665513921andsecond126684767741/126686298972cancelledwithnohostedrunner; exactannotationsnotacquired/capacityconstraints. DesktopthirdsameSHAretryacceptedjob126702335253queued; LinuxRust126684768079allstepssuccessinclComplete16:14:15UTCbutAPIterminalpending at16:21–16:24UTC. No localMac27substitution, assertionweakening, runnerconfigchange orhealthyjobcancellation. Sourcefrozen3a290; onlyfinalevidence/taskhandoff edits. Keep79InReview,44ToDo/unstarted.

Latest receipt16:25:23UTC supersedes earlier Linux-terminal-pending notes: LinuxRust retry126684768079 is terminalSUCCESS, alongside WindowsRust andLinux/WindowsDesktop. OnlymacOS15success receiptsmissing. BothsecondMacjobs126684767741/126686298972 CANCELLED withsame nohostedrunner/capacityannotations. ThirdDesktopretryaccepted/queued126702335253. Rustthirdrerun403 becauseworkflowstillin_progress(attempt2,updated16:19:14), despitealljobsnowterminal; deferuntilworkflowfinalization, thenretryMaconly. No requiredplatformconfigurationchange/localMac27substitution. Completionnotapproved; AC4/DoD1unchecked; sourcefrozen3a290, evidencehandoffonly.

CI-only resume4November2026 20:05UTC: clean primarycodex/rewrite aee38a2, origin aligned0/0, Backlog1.55.1 matchesbaseline; @codex resumes own InReview task with unchangedsource3a290024df82d23f215165707c81de4928ad3a4a. Four unrelatedworktrees preserved. LiveRust41856238536 completedfailure attempt2; Desktop41856238041 completedfailure attempt3. RustMac126665515114/126684767741 andDesktopMac126665513921/126686298972/126702335253 allcancelled beforehostedrunneracquisition(runner_id0). Linux/Windows passed; successfulcopiedjobIDs retain originalexecutiontimes. Noimplementation re-review/broadtestcampaign or44work. Onefreshmac-only retry perworkflow is the session bound; acquisition failure again ends automaticretryloop and preservesblocked acceptance.

LiveCI-onlyretry: supported specific-job reruns accepted for RustMac126684767741→126831432252(attempt3) andDesktopMac126702335253→126831460036(attempt4). Bothactuallyacquiredrunnersandareexecutingchecks, unlikeprior5cancellations. SourceSHAremains3a290; successfulLinux/Windowsreceiptscopiedwithoriginalexecutiontimes, notrerun. All224sourcehashesmatchtestedrevision/currentcheckout. Onecombinedcompletion-awaremonitor; no source/config/testchanges, no new broadcampaign or44work. Initialfullhistory/annotationsandfreshretrymetadataareconsolidatedinexistingstructuredledger.

FinalCI-onlyacceptance4November2026: specific macOS job retries only, after cooldown, bothacquiredhostedrunners. Rust126831432252 runner1000002608 attempt3:20:06:26–20:28:42UTC SUCCESS; Desktop126831460036 runner1000002609 attempt4:20:06:29–20:15:43UTC SUCCESS. Both actualmacOS15.8.2/24H232 arm64 image20261028.0489.1; checkoutlogsverify3a290. All6originalexecutionreceipts terminalsuccess atsameSHA: RustLinux126684768079/Windows126665515550; DesktopLinux126665513098/Windows126665513623; freshMac2. BothrunscompletedSUCCESS. Healthyjobsnotrerun; latestcopiedstatusjobIDsretainoriginalexecutiontimes. One60scompletion-awaremonitor exited0; no furthercampaign. Freshindependentreview/root/msh79_final_ci_reconciliation PASS verifiedsource/receipts/requiredsteps/history; populatedexplicitreusedDesktopLinux/Windowsrequired-step lists. OriginalAC/DoDtext, source/config/tests, profile/rules/budgets/architecture unchanged. ClosedAC4/DoD1; allcriteriaDone. HistoricalMac5capacitycancellations+MSH112failure retained, no localMac27substitution. Update44handoffonly, stop79.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed and integrated on codex/rewrite at exact tested source3a290024df82d23f215165707c81de4928ad3a4a (reviewed implementationb459250, identical source tree). Both explicit immutable Tenjiku profiles use the existing native dispatch, desktop envelope, controller/workers/install, frontend session/shell and native-choice controller. Complete routes, stationary capture targets, native capture/burn/mover-death/promotion phases, rights, royals, history, branches, observations, atomic files, cancellation and fresh stopped authority are preserved. Execution remains generic unscored UnsupportedMode and ordinary Tenjiku product activation remains off. Original160M aggregate admission,15s deadline and actual ending/Close reserves are unchanged.

Existing evidence remains accepted:53 real native DTOs,12 applicable consumer rows for both profiles,50 Tenjiku/495 total frontend tests, workspace/focused checks and measured worker/mutex/QA observations. Native87 conformance is not recounted. CI-only resumption obtained the missing actual macOS15.8.2 arm64 executions: Rust126831432252(run41856238536 attempt3), Desktop126831460036(run41856238041 attempt4), both terminalSUCCESS. Reconciled all6 exact-source platform receipts; successful original Linux/Windows executions reused without rerun. Both workflow runs now completedSUCCESS. Five earlier zero-runner macOS cancellations and knownMSH112 firstLinuxfailure are preserved, with successful same-source retry. No source/check/runner configuration or acceptance text changed; all224sourcehashes match.

Fresh independent final evidence reconciliation PASS, no blocking findings; originalimplementationreview reused. All4AC and4DoD supported. Single structured record docs/m4/tenjiku/shared-verification.json, evidence revisionMSH79-CI-FINAL-2026-11-04, includes exactjob/step/source/runner/attempt receipts and historical outcomes. MSH44 now has all6dependenciesDone, but remainsToDo/unassigned/unstarted with all8originalcriteria and5productconsumerrows unchanged: functionalengine/search/completegames, console/finalnotation, licensedassets/finalboard andnormal/minimumthreeplatformTenjigameplay. No44/M5 or unrelated implementation started. Stopafter79.
<!-- SECTION:FINAL_SUMMARY:END -->
