---
id: MSH-162
title: Parallel Experiment Runner
status: In Progress
assignee:
  - '@codex-cloud-runner'
created_date: '2026-11-11T18:35:00Z'
updated_date: '2026-11-11T21:31:00Z'
labels:
  - handle:parallel-experiment-runner
dependencies: []
ordinal: 166000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Problem
MSH-161 (Competitive Gameplay Diagnosis) serial execution is too slow.

## Objective
Handle: Parallel Experiment Runner
Deliver reusable local and GCP Batch execution over native match/replay and frozen analysis.

## Approach
Versioned immutable jobs, bounded workers, dependency scheduling and verified artifacts. Preserve frozen study and coordinator ownership.

## Completion
Reviewed isolated branch handoff. Cloud pilot requires project/access/spending approval; coordinator owns integration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Game pair, all-move screening and confirmation jobs reuse native and frozen analysis authorities.
- [ ] #2 Parallel local/GCP jobs support dependencies, integrity, retries, immutable success, status/retrieve/cancel and resources.
- [ ] #3 Targeted macOS/Linux tests and independent source/execution review cover replay, failure and restart.
- [ ] #4 Authorized cloud pilot proves complete pair and analysis, recovery and eight-worker throughput/cost.
- [x] #5 Reviewed operating and integration handoff preserves frozen inputs.
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
1. Wrap frozen pair/query/extraction functions with portable manifests and stable identities.
2. Implement immutable stores, local dependency execution and GCP Batch one task per VM with GCS conditional writes.
3. Provide CLI and fixed Linux packaging.
4. Level 2 macOS/Linux process/storage tests and independent review; actual cloud gate after owner authorization. No desktop campaign.
5. Preserve isolated branch for coordinator integration.

Owner-requested quick status: add a read-only concurrent receipt snapshot with plain-text and JSON output, optional Batch state, honest unknown/stale-state labels and no artifact verification. Add tests for partial/derived progress, retries, failures and no artifact reads. Remove obsolete tools/study_status scripts (no external references found). Update operating instructions, run focused/full relevant tests and independent review. Preserve the running frozen cloud worker and existing slow verification commands.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner codex-cloud-runner; branch codex/parallel-experiment-runner; worktree /Users/johnweathers/.codex/worktrees/parallel-experiment-runner/mShogi; base c01d54bf. Cached CLI 1.55.1 matches baseline; PATH 1.55.0 will not mutate records.

Implementation ready for source checkpoint: versioned immutable local/GCS jobs, native pair/replay and frozen screening/confirmation adapter, Linux physical-core process isolation, Batch one task per VM, status/recovery/retrieve/cancel/resources and event-driven Pub/Sub exit notification. Independent reviews found and resolved dependency substitution, missing derived confirmations, signal cleanup, notification accounting, runtime/store identity and pipeline coverage defects. Targeted Mac/Linux ARM tests and native replay performed; exact final-source record follows. No new study searches/matches, cloud resources, integration or coordinator communication. Pending owner GCP project/region/service-account/bucket/registry and spending authorization, fixed Linux x86 artifacts and real pilot.

Source checkpoint 14bea72cb326d77fe780927f07d2da60e66b6528 passes48tests on macOS and local Docker Linux aarch64 (2.624s/2.774s), including eight concurrent execution fixtures and detached-child termination. Full frozen schedule preparation yields100pairs/200pending initialjobs with no execution. Existing complete Mac pair native replay passes. Independent source handoff approved after all findings resolved; see docs/experiments/review.md and single structured verification.json. Actual fixed Linux x86 build/GCS/Batch/pilot/throughput/cost/live notification remain NOT RUN. No billable resources created or new study computation. Await owner project/access/spending inputs and coordinator validation; no integration. Keep app-managed worktree for this explicit ongoing purpose. AC5 records reviewed operating/integration handoff only; other criteria remain unchecked until execution gates are evidenced.

Owner supplied new project mshogi and requested novice setup Markdown. Read-only confirmed project216675633801 and billing linked; required Compute/Batch/ArtifactRegistry/PubSub setup absent. Added independently reviewed docs/experiments/gcp-novice-setup.md and exact pilot config; recommends Iowa at verified C3standard4 list$0.201608/hour and proposes$30 pilot, not authorized yet. Source51351c48ea1adf9ac96011aec1440e44ceb741a4 passes50tests on Mac/LinuxARM/packagedLinuxx86. Fixed frozen native and Oxidation Linuxx86 release builds succeeded; official Stockfish19 Linux release archive hash verified; actual native replay/UCI handshakes pass without searches. Local image f6353cb4 built and validated; compile/image exit notifications accepted. Single verification.json updated. Remaining gate is project setup, spending approval, image upload/final registry digest and actual cloud pilot. Task stays In Review; no integration or full study.

Owner approved US$30 initial pilot on2026-11-11 for project mshogi/us-central1, conditional on setup completion: one opening pair first, then eight concurrent workers after review, all within the same allowance. Full200game study and sixteen workers remain unauthorized. Read-only prerequisite check after approval still finds Compute Engine, Batch, Artifact Registry and Pub/Sub APIs disabled; setup gate remains. No cloud resources created or pilot launched. Do not ask for spending approval again within this scope.

Owner reports all12setup steps complete. Read-only preflight verifies required APIs, C3 CPU quota32 with zero usage, regional IP quota8, correct worker roles and conditional claim deletion, private registry, intended subnet and active dedicated PubSub subscription. Continue within previously approved US$30: upload validated image, freeze real registry digest and one-pair manifest, run one pair with all-move/confirmation pipeline and event-driven exit notification; review before eight-worker stage. No full study or integration.

First approved pilot msh162-pilot-one-v1 failed before tasks/VMs with runDuration0s; initial-terminal listener queued completion successfully. Audit shows Batch service-agent IAM granted0.445s before job creation: propagation is the leading hypothesis, not proven. Real PubSub event exposed numeric project-name alias mismatch; corrected controller matching with UID and full region/job validation plus authoritative re-fetch. All54 local tests pass. Preserve frozen worker51351 image and manifest for bounded retry, with separate controller revision. Evidence in docs/experiments/pilot; full study and integration remain excluded.

Controller34892627 passed54tests and independent28-test GCP review. Retried unchanged manifest/image as msh162-pilot-one-v2 within approved US$30; actual state SCHEDULED with one pending C3standard4 task. Event-driven listener PID54494 queues owning chat on exit; receipts/logs /private/tmp/msh162-pilot-one-v2-listener.*. No periodic polling. See verification.json and pilot/ for exact submission, identity and evidence. Resume on completion: retrieve/verify pair, replay, all-move and derived confirmations; review before eight workers. No primary integration. Keep this worktree active for listener and pilot.

Actual one-pair pilot SUCCEEDED:121jobs verified from1889retrieved objects,2native-replayed games,434screen queries and119deeper confirmations. Two independent reviews passed; see pilot/one-pair-scientific-review.json and one-pair-runtime-review.json. Actual PubSub event queued chat successfully. VM/disk deletion confirmed; estimated VM compute$0.14649, billed cost unavailable. Submitted reviewed recovery probe msh162-pilot-recovery-v1 (same image/C3,900s,no retries): inject adapter failure, preserve failure, run genuine confirmation retry, prove third call skips computation. Listener72758 queues chat; files /private/tmp/msh162-recovery-listener.*. Await event then inspect validation/recovery-v1.json. Eight distinct next pairs prepared/reviewed in pilot/eight-pair-manifest.json but not submitted. Remain within totalUS$30; no full study or integration.

Recovery SUCCEEDED with independent actual-cloud verification: one failed attempt preserved, genuine5M retry complete, success bytes unchanged and third call skipped execution; all15objects retrieved. VM/disk/claim cleanup confirmed (pilot/recovery-review.json). Application failure only, not abrupt VM-loss evidence. Submitted authorized eight-worker pilot msh162-pilot-eight-v1, experiment msh162-cloud-pilot-eight-v1, next eight distinct frozen pairs main-01-01..08; state SCHEDULED,taskCount8,parallelism8. Same totalUS$30. Event listener PID73734 queues owning chat on completion; /private/tmp/msh162-eight-listener.*. Resume on event: parallel pinned-generation retrieval, verify all static/derived jobs, independent replay/scientific/runtime review, overlap throughput/cost and cleanup; then coordinator handoff. No full study or primary integration.

Owner-requested quick-status delivered at cecb9e92: concurrent plan/possible-failure reads, completion-marker counts, optional manifest-bound Batch state, plain text/JSON, explicit unverified/stale-claim/unknown-total labels. No artifact reads or computation. Live check16.67s vs earlier fullreceipt scan45.69s (different evolving snapshots). All60 Mac tests and6focused Linux tests passed; independent6-test review approved. Removed four obsolete tools/study_status files; no callers found. Updated operating/novice guides. Frozen worker image and active eight-worker listener unchanged. Cloud pilot still pending completion; no integration or full study.
<!-- SECTION:NOTES:END -->
