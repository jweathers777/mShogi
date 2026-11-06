---
id: MSH-47
title: Independently verify integrated six-game delivery and close M4
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-06T07:32:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-26
  - MSH-39
  - MSH-41
  - MSH-42
  - MSH-43
  - MSH-44
  - MSH-45
  - MSH-46
  - MSH-108
documentation:
  - docs/design.md
  - docs/m4/shared-integration/README.md
  - docs/review/architecture-health-108.md
  - docs/review/evidence/msh108/verification.json
  - docs/m4/final-delivery.md
  - docs/m4/evidence/msh47/verification.json
ordinal: 47000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M4 was shown as complete from one completed ticket while five promised games were absent. This is the evidence-based closure gate for the entire M4 scope, including preserved chess completeness and required shared integration. A task-count percentage is only a count of currently assigned tasks. P-01/P-03/P-04 and the accepted roadmap determine closure. This scope does not authorize future implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 MSH-26 remains Done with its milestone and evidence preserved. All five complete game deliveries MSH-41 through MSH-45 and shared integration MSH-46 satisfy their full acceptance criteria and definition of done; M3 prerequisite closure MSH-39 is established.
- [x] #2 An independent review traces all six games through sourced complete profiles, authoritative state/actions/history/adjudication, independent conformance verification, reusable library, standalone console, functional native mShogi engine, canonical saved records and actual desktop behavior. Review explicitly distinguishes rule/search correctness, functional play, performance and M5 measured strength.
- [ ] #3 Integrated revision and actual Windows/macOS/Linux packaged application and engine evidence are reproducible. Required integrations, unresolved rules, missing promised games, failed checks and material findings prevent closure; none may be hidden by task aggregation, archiving or a 100% board count.
- [x] #4 The durable M4 closure verdict records evidence, resolved findings and honest limitations, reconciles any refined child tasks with the original promised scope, and identifies M5/M6 follow-ups without transferring unfinished M4 obligations. No M4 closure occurs while any required game or integration remains incomplete.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Primary acceptance integrator Codex, primary codex/rewrite clean/origin-aligned at07f1bac4c366c0a327dd7f31e9528ee9aa444b5b. Preserve unrelated historical-access worktree and MSH133 InReview ownership (no milestone). (1) Independently reconstruct original P01/P03/P04/A01–A07, M4definition,26/39 and alloriginal41–46/47criteria against actual source and accepted conformance/search/console/record/desktop/platform records; preserve native semantics and retiredformat policy. (2) Read-only isolated game/conformance audit and originalscope/openissue challenge; root reconciles all6 actual platform receipts/source/package identities, shared46/88/89/108evidence and supported formal milestone workflow. Explicitly investigate112 remaining ToDo/m4 assignment versus original requiredbehavior and derived completion; no hidden reassignment/archive/waiver or green-retry-as-fix. (3) Publish docs/m4/final-delivery.md with originalscope, sevenprofile/sixgame coverage, exact evidence, all limitations/issueclassifications, child/DAGownership reconciliation and closureverdict; compact new acceptance index references existing detailed records rather than duplicateplatformledger. (4) Fresh independent read-only wholemilestone assessment challenging gaps; proportionate hashes/source/dependency/CLI/links only, no broad native/frontend/GUI/CI campaign absent real issue. (5) If every original47AC/DoD and formalclosure obligation supported, normal evidencecommit/push, Done47 and supported formalM4closure with unchangedtaskhistory; otherwise preserve unfinished47/M4, precise blocker and narrow recommendation. Update roadmap/currentM4docs and identify actual nextM5assignment without starting48/109/M5/M6/refactoring/futurevariants. Stop after47review/closure.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-46 final shared-delivery handoff2026-11-05 (closure pending): original46AC1–6 are reconciled in docs/m4/shared-integration/README.md against the immutable88platform record and89independent contract assessment, not merely child statuses. Exact tested runtime/platformbc92bfe5e3b20e9e999df12cbf54b91682c71b8d; MSH89final integration91115368c6e99f344dee002d9264f4d0b62cc76b changes no executable behavior. All6 required jobs and actual Windows/Linux49/49 packaged phases/localmacOS27 seven-profile normal/QA behavior retain exact source/OS/signature/accessibility qualifications. Original47dependencies26/39/41/42/43/44/45/46/108 and four criteria/DoD remain unchanged;47 stays ToDo/unassigned/unstarted and must wait for independent108architecture diagnostic as well as46Done. M4 is not closed here.112 remains ToDo/unfixed and separately owned functional diagnostics; keep recurrence/no-retry-fix/nonblocking shared-contract rationale explicit. No unfinished M4 work is transferred to M5; no strength/release signing/fullscreenreader claim.

MSH-46 is Done after original6AC/4DoD reconciliation, fresh independent completion approval and approved evidence normal commit/push90e1222d8db27ede3212b794e877a975a068c35d; final delivery record docs/m4/shared-integration/README.md. Exact tested runtime/platform source remainsbc92bfe5e3b20e9e999df12cbf54b91682c71b8d, immutable88record/89review unchanged.108 is now dependency-ready but unstarted;47stillwaits108 and retains whole-M4 independent closure with all original dependencies/criteria.47remainsToDo/unassigned/unstarted; M4 is not closed.112remainsseparatelyunfixedfunctionaldiagnostics with original nonblocking shared-contract disposition, no strength/accessibility/release-signing claim.

MSH-108 current six-game architecture diagnostic handoff2026-11-05 (finalreview/integration pending): docs/review/architecture-health-108.md maps all6 native/search/application/console boundaries and reconciles106/115/107 against exactsource1d3920227bc376b5cc1694928597e7b5a1d5a864. No material architectural blocker demonstrated; four bounded advisory findings retain concrete consumers, benefit/risk/verification scope. Chu ordinary replay full-undo retention remains; Tenjiku avoids it and has native cooperative APIs plus actual scoped measurements, while supervisor echoedcontext decode still has a synchronous stop boundary. One accepted controller/document/execution/current-envelope/frontend/shell/choice owner remains, game-owned complete semantics unchanged. Two immutableTenjiku profiles and actual native/document budgets/endingClose reserves preserved; timings retain originalsource/synthetic/local qualifications. New diagnostic index docs/review/evidence/msh108/verification.json reuses unchanged88/89/46 evidence without broadtests/CI/GUI.112 remains separatelyownedunfixedfunctionalrunnerdiagnostics, no new redesign gate inferred. After108Done all existing47prerequisites26/39/41–46/108 will be satisfied;47 remains ToDo/unassigned/unstarted and still requires its own original4AC/DoD/independent wholeM4 closure. No downstreamplan/activation or M4/M5 closure here.

MSH108 is now Done with all original8AC/4DoD, fresh independent final assessment and accepted report/evidence normally integrated/pushed at12fc1cab114dd3f2570973f75e7edc1d06a9a9d5 oncodex/rewrite. Current source-qualified diagnostic docs/review/architecture-health-108.md and index evidence/msh108/verification.json establish coherent six-game owners/no material architectural blocker; four advisory investigations are not requiredrefactors or new dependencies. Reconciles106/115/107 and actualboth-profileTenjiku native/search/document/presentation costs/reserves; acceptedrules/formats/limits/minwindowbehavior unchanged.112 remains separatelyunfixedfunctionalrunnerdiagnostics with honest nonblocking architectural disposition. All existing47prerequisites26/39/41–46/108 are nowDone;47 is dependency-satisfied, remains ToDo/unassigned/unstarted with original4criteria/DoD/dependencies intact. Its independent wholeM4 acceptance stillrequired; no47review/M4closure/M5/refactor started here.

MSH47 current acceptance review at07f1bac4c366c0a327dd7f31e9528ee9aa444b5b: docs/m4/final-delivery.md and evidence/msh47/verification.json reconstruct original P01/P03/P04/A01–A07/M4 and allfour47criteria. Fresh read-only game/conformance researcher supports all6games/bothTenjiku profiles using exact fixture/consumer/source qualifications. Separate originalscope/openissue reviewer finds unresolved112 meaningful UnexpectedExit/exitdiagnostic contract relevant to production mshogi-match run; original47AC3 failedcheck gate prevents wholeM4closure. Not a new rules defect, fabricatedscore, broken ordinarydesktopplay or architecture redesign requirement;89/46sharedapp and108architecture narrower verdicts remain accepted. M4active75/77 (47+112 unfinished); CLI archive onlyretirement, not completion. No reassignment/archival/waiver/closure. Live GitHubconnector confirmsRust42028972679/Desktop42028972586 all6 originaljobs PASS/sourcebc92bfe/attempt1/steps;202masterevidencehashes,rawWindowsLinux49/49reports/13normalhostenginepairs,producer/auditreceipts and4localMacbundleequivalence checks pass. Initial audit assumed packageinventories in rawGUI.files; corrected actualseparateproducer/audit schema, not a productfailure/retry. Acceptedruntime differs fromcurrent only4rustdoccomments. Original47scope/dependencies andallcompletedrecords/owners preserved,unrelated133review/worktree untouched. No product/conformance/frontend/GUI/CI/strength campaign or112reproduction/fix. Fresh finalwholemilestone reportreview pending; no47AC/DoD checked yet. Immediate narrow correction ifconfirmed isexisting112 thenresume47;48 remains actualnextM5pilot afteracceptance, unstarted.

Fresh final independent msh47_final_acceptance approves the closure-withheld report, NOT M4acceptance. Current-source07f1bac, reportcandidateSHA24787ffa595a47beeb79868118bad42001d9f5364f90649009cc89564fc74146; laterlocators clarified actualcrates/mshogi-runner paths only. Independently confirmed original47AC3 failuregate, realheadlessproductionconsumer, exit7fixture/strictcodedexit assertions, unchanged112failurecontract/history, accepted sixgame/shared/platformscope and noadditionalmaterialgap; sourceequivalence,13acceptedrefs/202evidencehashes,sixoriginalCIreceipts,rawWinLinux49/49/13normalpairs,nineDoneprerequisites/originalACtext anddiffchecks passed. No controlled112reproduction or broadtest/build/GUI/CIcampaign. CheckAC1/2 andAC4as explicitdurableblockedverdict, notpositiveclosure;AC3unchecked. DoD1/2unchecked (requiredacceptance/materialfinding unresolved);DoD3currentdocs/existing112handoff;DoD4pending normal evidenceintegration.47remainsInReview,M4active; alloriginalnine dependencies/milestonemembership/ownership preserved.112remainsToDo/unassigned; append-onlyexistingtaskhandoff,no implementationauthorized. Acceptedcomponents and narrower89/46/108verdicts stayDone. Immediatecorrectiveassignment112thenresume47;48onlynextM5afteracceptance,nonebegun.
<!-- SECTION:NOTES:END -->
