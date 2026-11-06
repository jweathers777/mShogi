---
id: MSH-112
title: Investigate early-exit runner failure classification race
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-29T12:53:00Z'
updated_date: '2026-11-06T09:47:00Z'
labels:
  - native-ci
  - runner
milestone: m-4
dependencies: []
references:
  - https://github.com/jweathers777/mShogi/actions/runs/40874033735
  - >-
    https://github.com/jweathers777/mShogi/actions/runs/41816309261/job/126527815548
documentation:
  - docs/m4/evidence/msh102/completion-2026-10-29/README.md
  - docs/review/runner-exit-112.md
  - docs/review/evidence/msh112/verification.json
priority: medium
type: bug
ordinal: 112000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Intermediate MSH-102 Linux CI40874033735 at6dec5ab failed process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored at crates/mshogi-runner/tests/runner.rs:172. Fixture exit expected UnexpectedExit but observed MissingResponse, engine0 handshake, Broken pipe (os error32), no exit status. Runner source was unchanged across095f27a through finala1f8234; final three-platform CI passed, which does not establish a fix. Preserve MSH-25 historical evidence and its earlier isolated-awaited-engine correction; this observation differs from its idle-other-engine race. Future bounded runner diagnosis only; no desktop/rules change, M2 reopening or gate on unaffected Xiangqi work. Implementation is not started.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Establish a controlled reproduction and identify early write-error, EOF and process-exit observation ordering; retain failed CI evidence and prior MSH-25 rationale.
- [x] #2 Resolve the product or fixture defect under the documented bounded failure contract. Preserve meaningful UnexpectedExit versus MissingResponse distinctions, established exit diagnostics, bounded cleanup and failures as unscored; do not merely accept either category or raise timeouts to mask the mismatch.
- [x] #3 Focused controlled process tests and relevant native regressions pass with independent review and actual platform CI; record remaining nondeterminism honestly.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Primary builder/integrator Codex on codex/rewrite in the primary checkout; starting source92a751d3e1df0ef4c4b9610cc55ec42e5e673770. Preserve unrelated worktrees and task133. (1) Trace spawn, handshake write, stdin failure, stdout EOF, stderr, exit sampling, classification and reaping against original25 evidence; prove the single-sample race with an actual-child controlled visibility gate and failing original-source/mutation controls. (2) Keep hard-write/awaited-EOF classification provisional while natural status is unknown; use the existing immediate10ms settlement and move the existing quarter-cleanup/max100ms grace before intervention. Finalize definitive natural0/7, preserve unknown status honestly, fixed receipt/clock semantics, total cleanup budget and separate forced-termination evidence. No timeout increase, alternate supervisor, public cancellation API or rules/protocol/schema changes. (3) Controlled fast/slow, late natural0/7/EOF, live closed-stdin/forced reap, timeout, clock, partial/malformed streams, startup-abort and20 real supplemental stress trials; focused runner/native regressions and fresh read-only review. (4) Required existing Rust Ubuntu24.04/macOS15/Windows2022 CI at frozen corrected44c86b01428ca5f0de56fdb96cbe7e5b013677ea; one completion-aware monitor. Preserve failed/superseded attempts; no packaged GUI campaign. Existing unrelated Windows view-test synchronization failure is recorded as unassigned MSH-134, with at most one unchanged-source Windows-only retry. (5) Reconcile exact-source receipts and independent final evidence, then evaluate original3AC/4DoD, publish bounded corrective evidence, integrate/push and finish112 only if supported. Append only the owner-authorized47 blocker handoff, preserving47 acceptance/status/evidence and leaving M4 open. Stop112; no47 closure or unrelated implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Recurrence observed during MSH-119 final verification at eb027be7cec0f3206094c477a31f0cd9b9f73f6f: Linux Rust CI41131548408 job124323454077 workspaceTest runner.rs172 expectedUnexpectedExit but observedMissingResponse(engine0 handshake, Brokenpipe32, exitNone, at_ns2229415). Runner/engine/config sources unchanged from82a8ae8; initial291Linux behavior suite passed before separate fixture-diff gate. Focused unchanged test passes once on local macOS (cargo test --locked -p mshogi-runner --test runner process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored -- --exact;1PASS,0.59s). This does not establish a fix or Linux success. Original failure retained in MSH-119 evidence; normal focusedCI retry pending workflow completion. No assertion, timeout, productionrunner or ownership/status change; MSH-112 remains To Do/unstarted.

MSH-86 native CI recurrence at source136f89260090d21bafea3f67b036b3354729e4be: Linux run41536948188 job125591097628 failed the unchanged runner.rs172 exit case, expectedUnexpectedExit versus MissingResponse/handshake/BrokenPipe(os32), no observed exit status. No runner production/test/timeout/assertion changes in86; retained log and exact provenance in docs/m4/chu/native-verification.json. Existing112 owns this same ordering race; no new duplicate task, status or ownership change, no implementation started. Later green native CI would not establish a fix.

MSH87corrected-source recurrence at9749eec0ffdbfffbcc040b6a56c5bb2c90616a1f: UbuntuRustrun41816309261 job126527815548 workspaceTest runner.rs172 expectedUnexpectedExit forfixtureexit7, observedMissingResponse engine0 handshake Brokenpipe(os32), exitNone, at_ns2246915. Matches existing112sameearlywrite-error/exitobservation race; runner/engine/desktop-session/workflow sources unchanged from87starta7f9205 (gitdiffempty). Nativeconformance/core164 andpublicconsumer2passed onUbuntu before unrelatedrunnerfailure. Rawjoblog/private/tmp/msh87-ubuntu-corrected.log anddurable native-verification.json retainprovenance. No assertion/timeout/productrunner change, no112ownership/status/implementation. RequiredCI will retryfailedUbuntu onunchanged9749source; greenretry wouldnotestablish112fix.

MSH87unchanged-source Ubuntu-only retry succeeded in correctedRustcampaign41816309261 at9749eec, finaljob126537522942 allstepsPASS13m53s; requiredmacOS/Windowssuccess retained. This establishes current87CIacceptance only, not an112fix. OriginalfailedUbuntu126527815548 andexactsignature remainin87native-verification.json; no112assertion/timeout/productchange orimplementation, statusremainsToDo.

MSH47wholeM4 acceptance review2026-11-06 at07f1bac4c366c0a327dd7f31e9528ee9aa444b5b identifies existing112 as unresolved original47AC3 failure-contract blocker, with fresh independent scope/final review support. docs/m4/final-delivery.md F47-01 traces actual production mshogi-match run→run_experiment and original documented UnexpectedExit/exitdiagnostic assertion; repeated historical MissingResponse/Brokenpipe/exitNone persists, runner source unchanged since9749eec. No new controlled reproduction/rootcause/productversusfixture decision or fix claimed. Earlier89/46nonblocking sharedapplicationcontracts and108noarchitecturalredesign verdicts remain valid, not a wholeM4waiver; M2/native/gameacceptances not reopened. Existing112AC1–3 remain the narrow correction: controlled writeerror/EOF/exit diagnosis, contract-preserving product/fixture correction, meaningful categories/unscored outcomes/boundedcleanup and focused independent actualplatformverification, thenresume47. No112implementation plan, status, owner, milestone, dependencies, assertion or timeout changed; remainsToDo/unassigned/m4. No M5reclassification or newtaskhierarchy.

Controlledoriginal-source regression andsingle-sample mutation reproduceMissingResponse vsUnexpectedExit withrealchildexit7; restoredexisting10mssettlement/typedhardwritecause andearlyreceipt capture pass. PortableCI negativecontrolwillreproduceone-sampleorderingonLinux; positive1/2/9/10visibility,clean/unavailablestatus,timeout/cleanup/clock,partialstderr/malformed/startupabort and20realstress cases added. Localrunner55pass0fail/1parentignoredfixtureexercisedaschild; Clippyalltargets andengineBridge2pass. Fresh independentread-onlyfixreview andincrementalowner-tailreview approved,no blockers,diffSHA9196a9e7b0bbe2238481ef8d327f6810cca0459bf0dcd483ae3901775cf520b9. docs/review/runner-exit-112.md andsingleevidenceindex retainnegativefailures/original25rationale/historicalrecurrences; no stagepassesrelabeledfix. Required existingRust3OSCI pending freeze/commit; no broadGUI needed. MSH47 acceptance/status/criteria untouched; finalowner-authorizedappendonlyblockerhandoff followsonly112acceptance. No112AC/DoD checked beforeplatformreconciliation.

Full owner tail clarified definitive naturalexit during boundedcleanup mustfinalizeprimaryfailure. Initiald835c7c immediate10ms-onlycheckpoint/initialreviewapproval insufficient; deliberatelysupersededRust42187775585(cancelledall3,allrunnersacquired) andunrequiredDesktop42187775810(cancelledall3). Singlemonitor2673 endedterminalcancelled; rawreceipts retained. Revised privatependingpipe marker defersunknownhardwrite/EOF through existingcleanupgrace BEFOREstop/quit/kill, definitive0/7finalizesprimary+diagnostics withfixedreceipt, unknownstayshonest andforcedkillmetadata separate. Movesexistingquarterbudget/max100msgrace,no secondgrace,no timeoutraise; protocolwritesboundedbyremainingbudget. Controlledlate0/7/EOF,actualaliveclosedstdin/forcedreap,lateclockprecedence andfinalizationmutationfail provefulltail gap/correction. Revisedrunner57pass0fail2actualchildfixturesignoredonlyinparent;Clippyalltargetspass. Freshindependentread-onlyre-review approves revisedsource,no blockers,incrementaldiff358bce6c855d4cd3ebb7228792b08171de2749e343673ea1f3a3640122f4f789 againstd835c7c. Replacementexactsource3OSRustCIpending; noCIoracceptanceinferredfromsupersededcheckpoint.47status/criteriaandM4untouched.

Corrected source44c86b01428ca5f0de56fdb96cbe7e5b013677ea native run42190693190: Windows127801682025 failed unchanged desktop-session historical-root assertion before runner suite. Independent read-only provenance review traced async Chess Work::View/no admission receipt versus immediate helper completion; desktop-session/engine/workflow sources unchanged baseline92a751d and no runner dependency. Separate unassigned Ready MSH-134 records the test synchronization issue, not started. One Windows-only unchanged-source retry after initial run terminal; preserve failed log/receipt, stop blind retries if recurrence. Linux/macOS workspace Test steps passed, remaining native steps still pending. No acceptance or all-platform success inferred.

Actual44c86b0 native attempt1 Ubuntu127801682198 and macOS15 127801682340 passed every required step; completed rawlogs each independently confirm all11 critical runner cases. Independent final evidence review verifies6 source hashes,17 indexed evidence hashes,3 protected47 hashes and no dirty source diff; AC1/2 supported and checked. AC3/DoD remain pending Windows-only attempt2 job127812835546 and durable final integration. Old monitor15083 ended on terminal attempt1; only monitor53716 active. GitHub attempt2 carry-forward receipt aliases127812836773(Ubuntu)/127812882169(macOS) retain original execution timestamps/runners; no healthy-platform rerun. Failed Windows127801682025 remains preserved, unrelated MSH-134 unstarted. No MSH47 acceptance/status or M4 change.

Final independent read-only acceptance approves44c86b01428ca5f0de56fdb96cbe7e5b013677ea, no source/evidence defect. Rust42190693190 attempt2 success: actual Ubuntu127801682198/macOS15 127801682340 attempt1 and Windows127812835546 attempt2 each all8 required steps success; all3 completed rawlogs independently contain all11 critical cases pass, including Linux original-ordering control and20 stress trials per platform. Reviewer verifies6 source hashes,19 evidence hashes,3 protected47 hashes and no source diff. AC1–3/DoD1–3 supported and checked. DoD4/statusDone wait only normal durable final evidence publication+push; product44 already integrated/pushed. Windows original127801682025 failed unchanged view-test helper before runner; sole Windows retry passed, not an MSH-134 fix. Raw failed/superseded/carry-forward receipts retained. Main report docs/review/runner-exit-112.md and single index docs/review/evidence/msh112/verification.json. No active monitor; no further campaign. MSH47 acceptance/status/reports unchanged and M4 open.
<!-- SECTION:NOTES:END -->
