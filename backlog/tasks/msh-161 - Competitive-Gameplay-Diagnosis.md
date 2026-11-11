---
id: MSH-161
title: Competitive Gameplay Diagnosis
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-10T20:47:00Z'
updated_date: '2026-11-11T18:37:00Z'
labels:
  - handle:competitive-gameplay-diagnosis
milestone: m-5
dependencies:
  - MSH-159
documentation:
  - docs/m5/competitive-gameplay-diagnosis.md
ordinal: 165000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Competitive Gameplay Diagnosis.

## Problem
Isolated corrections have not established which weaknesses cost mShogi competitive games.

## Objective
Diagnose recurring decision weaknesses across approximately 200 complete games and rank evidence-based engineering investigations; no production changes.

## Approach
Freeze a reviewed protocol, qualify one fixed reference if needed, pair diverse master-supported openings and separately study unaided starts at comparable five-second thinking time. Analyze every move from both players using fixed Stockfish, confirm consequential events and review meaningful batches. Coordinate opening outcomes with MSH-160 (Chess Opening Quality Assessment).

## Completion
Deliver independently challenged diagnosis, ranked investigations and updated roadmap, then stop for owner review. The owner authorizes the broad study and necessary analysis without a routine aggregate match cap; preserve individual bounds and monitoring. Full request: docs/m5/evidence/msh161/owner-request.txt.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Freeze reviewed protocol, provenance, fixed opponent/binaries/conditions and study-specific resource authorization.
- [ ] #2 Complete approximately 200 diverse paired/unaided games with pilot and batch reviews; document sample changes, failures and actual resources.
- [ ] #3 Analyze every move of both sides with fixed Stockfish; confirm significant events with controlled alternatives and legal histories, preserving bounds, perspective and uncertainty.
- [ ] #4 Coordinate MSH-160 opening outcomes, prepared-position quality and consistent phase transitions without duplicate study.
- [ ] #5 Report affected-game/move frequency, severity, phases/families/results, opportunities/recoveries/conversion and conflicting successes.
- [ ] #6 Independently review ranked behaviors, potential mechanisms and investigations; update roadmap, integrate evidence and stop before engineering.
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
1. Reconcile opening task, sources, prior evidence and independently reviewed protocol.
2. Fingerprint unchanged engines; pilot games, native replay and full-move analysis.
3. Complete broad coverage in reviewed batches, preserving failures and resource use.
4. Confirm events, independently challenge rankings, update roadmap, integrate and stop before engineering.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder Codex on codex/rewrite at 77101fe; unrelated worktrees preserved. PATH CLI1.55.0 read-only; mutations use baseline1.55.1. Level1 offline helpers/evidence with native replay and independent reviews; no new platform or GUI claims.

Preparation checkpoint: unchanged77101fe candidate/runner built and fingerprinted; Goldfish/Oxidation identities match retained binaries and Stockfish19 preflight passed. Independent qualification/source/analysis review recorded.14 offline analysis tests passed. Unattended four-game qualification batch launched; after_qualification software chain waits on shared OS lock then validates automatic reference smoke, analyzes every move and screens330master-source candidates. No model move supervision or repeated result polling. One app review scheduled for next checkpoint; README handoff records exact state/artifact paths. Main approximately200game coverage, method freeze and diagnosis remain unfinished. Sourcev2 resolves symmetry/center/castling coverage before any candidate results; no production changes.

Preparation integrated on codex/rewrite at e066f58. Main study remains In Progress; no acceptance checks or final rankings claimed. Deferred review11November01:55Europe/Paris continues from README/state files; no need to poll individual moves. Live runtime outputs remain local pending completed-batch preservation.

11November heartbeat: all4qualification games completed normally with native replay; candidate2winsOxidation,1win1drawGoldfish. No stronger-opponent selection yet. Reference chain stopped at overstrict zero-uncertainty smoke gate; independent review approved gate-only correction allowing its specifically flagged final-bound uncertainty, while comparison retains null loss.16 tests passed. Preserved original failed-chain receipts, reused successful smoke and resumed unattended fullmove analysis/source screen. Next single review03:15Europe/Paris. See smoke-recovery.md; no production changes or ranked conclusions.

03:15review: initial640decision/1280query allmove analysis plus330source screens complete, zero failedqueries. Full1618artifacts compressed/hash-preserved. Blind gameplay reviewer found several provisional themes plus strongplay, no priorities;14decision25singleton5M confirmation panel frozen. Four strict-balanced contrasting masterroots selected without competitive outcomes;16additional qualification games followed by allmove analysis launched unattended, since neither reference yet established stronger. v1screen retains51of59 rawthresholdcrossings hidden by v0uncertainty without clearing flags or confirming events; offline checks pass, originalv0preserved. Main200game study/method/opponent still unfrozen. Next single review08:30Europe/Paris. See README, pilot-method-review.md, qualification-gameplay-review.md and extension-chain.json.

Owner execution refinement11November: delete timedreview automation and replace future unattended launches with notify_exit.py -> codex queue to owning chat on success/failure, using normal local execution escalation for database access and no security changes. Extension is completed42steps:25pilotreference queries,16games,8allmove analyses/summaries; review is due now. Added receipt-based exit notifier, success/failure/queuefailure/duplicate tests and independent review. Main study remains active; no periodic watcher or guessed schedule remains.

Exit-notification implementation verified:4tests pass covering success, commandfailure, signalwithhonestcleanupuncertainty, queuefailure and duplicate refusal. Initial sandbox/localgroup-probe failures preserved; notification cannot be suppressed by cleanup errors. Normal escalated actualvalidation of completedextension queued message01a1c989-56b4-78d2-a9ef-41ce90a11784 to owningchat, CLIexit0 and durable queueaccepted receipt. Timedautomationdeleted, no replacementtimer. TargetedLevel2macOSprocess/CLI evidence; no security settings changed, no Linux/Windows claims. Next action is queued substantive batchreview then study continuation via exitwrapped computations.

Completed independent qualification review selects fixedOxidation practical prepared reference(3opponent wins/4draws/1loss; combinedstart+prepared level, no statisticalsuperiority claim). Mainv1method and90sourcehistories frozen after32deeper checks, no relaxedquota:22families,16sym12opposite15closed30open. Main-opening and freeze reviews pass;7phase tests/200gameplan audit pass. Ten20game batches scheduled(180prepared+20unaided). Batch01 now running through exitnotification wrapper with native replay, allmove500k, v1screen and5Mmatched confirmations/audit. No timedwatcher. Final-opening-balance wake already consumed; avoid duplicate launch. Remaining:all200main games, completedbatch/20Mconsequential adjudication and final independent rankings/roadmap. See main-freeze.json, main-method.md, README and qualification resource/bundles.

Batch01 independent execution/gameplay review completed:20games10W2D8L(prepared8W2D8L,unaided2W),native replay/history/hash/conditions and7578referencequeries verified withzero failures. All2494played decisions have provisional dispositions;174nonflagged audit rows show no stable significant missed-error trigger.78behavior examples including16successes retained without final ranking.73decision20Mpanel now running through reviewed review_checks.py and notify_exit(session67182); two cases discover missing alternatives before matched comparison. Full15792artifact pre20M bundle verified31.3MB. Next queued exit: adjudicate consequential20M/episodes, update categories, then launchbatch02. All200games stillrequired; no production changes, timer or rating claim. Level1studyreview and localmacOSprocess evidence only. See latest README, main/batch-01/audit.json, reviews, cumulative-categories.json and verification.json.

Batch01 completed20M independent checkpoint:73decisions144queries verified,0failures;49finite deteriorations(bothplayers),4subthreshold,14unresolved,4controls,2claim-policy-limited.20M supersedes5M including removed support. Native06g1automatic intendedthreefold draw despiteSFwinning estimates explicitly limitsactual-result attribution; no rulesbug/policychange inferred. All2494played dispositions and earlier/linked uncertainty preserved. Updated provisional categoryledger95examples/10parents, nofinalpriorities. Additional20Mwall2546.393s/CPU2540.266s; total7722referencequeries, no parentdoublecounting.723artifact20Mbundle hashes verified. Bothgameplay reviewers andexecution audit approve continuation. Batch02 now running20games+allmove+5M via notify_exit(session1756), receipt notifications/main-batch-02.json; next exitreview thenbatch03. Full200required, no timer/security/productionchanges. Review evidence and latest README/verification.json authoritative.

Owner cloud-migration direction11November supersedes localcontinuation: NO newlocalbatches/queries/retries, NO batch03 even if oldqueuedmessage requestsit. Currentbatch02 hasno supportedpair-boundarypause; independent source review approves ownerfallback lettingexistingchainfinish(includingalreadyplannedanalysis), thenrecordstoppedstate. Snapshot7completepairs14games plusactivepair07;34complete main games total,20fullyanalyzed,20qualificationseparate. Notyetstopped. Preservedcompletedpair/statearchive and5exactMacbinarycopies(localignoredmanifest); all frozenlivehashes/10schedules unchanged. Interim gameplayreport independently reviewed:31candidate20Msupportedcomparisons/49targetedcandidatequeries across18of20analyzedgames, overlappingepisodeexamples anduncertainty explicit; nofinalpriority/productionchange. Repository-only cloudhandoff recordsfrozeninputs, isolatedbranchownership,200cloudprimaryreplay(allplannedpairs),Macexploratorystratum andvalidationgates. No cloudjobs launched. See migration/execution-policy.json, cloud-study-handoff.md, cloud-inputs.json, stop-safety-review.md and docs/m5/msh161-interim-gameplay-assessment.md. On existingexit:preservefinalbatch02/checkcleanup/exactcounts,updatehandoff,STOPLOCAL; noautomaticresume.
<!-- SECTION:NOTES:END -->
