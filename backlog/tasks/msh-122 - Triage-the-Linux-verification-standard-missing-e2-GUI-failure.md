---
id: MSH-122
title: Triage the Linux verification-standard missing-e2 GUI failure
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T13:46:00Z'
labels:
  - gui-verification
  - planning
milestone: m-4
dependencies:
  - MSH-106
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-health-106.md
  - docs/review/linux-gui-triage-122.md
priority: high
type: bug
ordinal: 122000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-106 V01/P9 captured an unexplained Linux verification-standard missing-e2 selector failure; MSH-111 pooling diagnosis does not establish its cause and MSH-112 concerns another native runner race. This separate bounded triage is intended before substantial architecture migration to establish a reliable GUI baseline. Creation is planning-only, with no active investigator or execution in this assignment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Inspect original run40994147366 at2f17510e4c5b8c3b92a051adfa070b4d5d834b21, exact package/driver/WebKit/environment, phase screenshot/DOM/host/transport evidence and selector/aggregate code. Retain missing-e2 failure as failed; classify observations without assuming driver reset, timing or product cause.
- [x] #2 Controlled focused reproduction or independently reviewed evidence-grounded disposition distinguishes fixture/selector/observation timing, transport and actual native/UI behavior. Any correction remains bounded to the diagnosed issue; rerun success alone, weakened assertions or raised timeouts are not a diagnosis/fix.
- [x] #3 Mutations/input/lifecycle are never automatically replayed after ambiguous delivery. Preserve meaningful phase assertions, actual native host/engine exit proof before teardown, bounded observation retries and normal-package instrumentation exclusion.
- [x] #4 Record an actual current installed/packaged Linux baseline covering all required existing phases and native file/quit journeys, with exact source/package/runtime/proxy identities and failures. If material failures prevent a reliable baseline, record the concrete blocker and leave acceptance unsupported; no inference of Xiangqi/other-platform GUI success.
- [x] #5 Focused boundary verification, independent source/evidence review and applicable CI support the disposition; MSH-111 Done driver history and MSH-112 runner classification remain distinct, unchanged records. No architecture refactor or variant implementation is bundled into triage.
- [x] #6 Execute this separately before substantial R1-R6 refactoring as the intended regression-baseline assignment. No blanket P9 dependency is added to unrelated tasks; any newly demonstrated verification prerequisite must be recorded concretely. Stop after normal integration/push and evidence handoff.
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
Inspect retained original Linux artifact and full phase traceback/DOM/package identities; reproduce the concrete boundary in isolation and apply only a demonstrated narrow correction. Verify observation/mutation and lifecycle invariants with focused checks, obtain independent source review, then run actual packaged Linux baseline and applicable CI. Independently reconcile evidence and finalize only MSH-122, preserving historical failures and MSH-111/112 ownership.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup 2026-10-30: primary builder Codex owns MSH-122 in primary checkout on codex/rewrite at 311ea53e152c71c5eadd2c428427325b30a42b29. Checkout clean; fetched origin and confirmed 0/0 divergence. Existing unrelated worktrees preserved. Pinned Backlog.md 1.55.1 used instead of installed 1.55.0; overview/execution guides read. MSH-106 dependency Done; no competing active task identified. Original Linux artifact12950842851 remains available, digest sha256:7b1be5e0ba05c047bc870dda6f7ff125064005ae2d61985841f787f474f3b3c0. Investigation does not assume driver, timing or product cause.

Original artifact downloaded and retained selected bytes/full report under docs/review/evidence/msh122/original. Full traceback identifies first e2 precondition before any pointer input; original null conflates absent and disabled, delayed capture shows healthy initial Chess. Controlled App/native snapshot ordering reproduces document-card present with64 squares/e2 disabled before command connection; original exact instantaneous branch remains unknown. Verifier-only correction polls specific present/enabled/visible/hit-testable target within unchanged25s budget, then dispatches pointer once. No product/driver/lifecycle changes. Local Node24.19.0/Python3.14.7:41 GUI boundary tests,9 App tests,TS check,diff check pass. Development-only test setup failures preserved in triage doc. Independent original-artifact/source reviewer found no material defect, pending final diff and actual Linux package evidence.

Verifier/test source integrated and pushed normally at1d84232c425270a6517711dbaa598106fc08aa31. Separate reviewer msh122_review independently inspected original artifacts, source/tests/docs, reran41Python tests PASS and approved source with no material findings; final evidence approval awaits actual Linux baseline. Desktop41063294647 and Rust41063294530 are running. No acceptance items checked pending actual package evidence. Existing prior311ea53 Rust41053672140 failure inspected: unchanged runner.rs:172 MissingResponse vs UnexpectedExit, Broken pipe handshake, matching separately owned MSH-112; no assumed connection to GUI target failure.

Actual Linux Desktop41063294647/job124088161908 at1d84232 PASS: all14installed/packagedGUI phases including standard recovery/retry, native file/quit journeys. Artifact12980863590 digestsha256:d46fb1d99f1ed98b1a5cc6a8712ceb12313ca389607042b46aa03d8f1fcb38e5 retained separately from originalfailedartifact. Root and separate reviewer independently verified229GUI hashes,13normalmanifest hashes,QAdeb digest and14pre-teardown native host/engine absence proofs. Actual normalhost16QAmarkers absent; all six realproxy experiments passed. Host/engine binaries and runtime/driver match original byte-for-byte. No pending pointer samples or observation retries in newGUIrun, so passing baseline is not claimed as originaltiming proof. Reviewer approved source/evidence with no material findings and accepts qualified original-branch uncertainty. Full report/docs at docs/review/linux-gui-triage-122.md; otherplatformCI finalization pending.

Final independent source/evidence/doc consistency review approved all6AC with no material findings. Desktop41063294647 and Rust41063294530 at1d84232c425270a6517711dbaa598106fc08aa31 PASSED all3platformjobs; exact metadata retained. Windows own actualGUIstep passed; macOS package/native checks only, no newMacGUIclaim. MSH122 detailed artifact audit establishes Linux only and does not infer Xiangqi GUI. Backlogdoctor/diffcheck pass; production source unchanged. Source integrated/pushed normally on codex/rewrite at1d84232; final evidence/task-only commit follows with current baselines. No worktree created; unrelated worktrees preserved. MSH116 remainsReady/dependency-ready/unstarted; stop122.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Corrected the external GUI verifier to observe a present, enabled, visible and hit-testable board target within the unchanged 25-second budget before exactly one pointer dispatch. A controlled App/native-snapshot ordering test reproduces the prior null-target readiness failure. The original instantaneous absent-versus-disabled state remains unknown; its failed artifact is preserved. No product or driver change. Reviewed source 1d84232c425270a6517711dbaa598106fc08aa31 is integrated and pushed on codex/rewrite. Focused checks pass: 41 Python GUI tests, 9 App tests and TypeScript. Independent source, artifact and final consistency review found no material findings. Actual Linux baseline: all 14 phases passed, with 14 pre-teardown native host/engine shutdown proofs; 229 GUI and 13 normal-manifest hashes, QA package identity and normal instrumentation exclusion audited. Desktop CI 41063294647 and Rust CI 41063294530 passed on all three platforms. Full provenance, original uncertainty and historical failures: docs/review/linux-gui-triage-122.md. MSH-111 remains Done; MSH-112 remains a separate unresolved runner issue. No Xiangqi GUI inference or architecture migration. MSH-116 is next dependency-ready and remains unstarted.
<!-- SECTION:FINAL_SUMMARY:END -->
