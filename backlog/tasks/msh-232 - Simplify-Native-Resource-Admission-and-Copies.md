---
id: MSH-232
title: Simplify Native Resource Admission and Copies
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T16:48:00Z'
labels:
  - simplification
  - core
  - performance
milestone: m-6
dependencies: []
priority: high
ordinal: 239000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: Chu and Tenjiku encode complete histories and hypothetical ending/Stop continuations to prove capacity on ordinary commands; Tori retains encoded-length and reservation accounting. Replay-work counters and undo/transaction snapshots add further state and copies. Replace unnecessary accounting and history copying with the simplest measured bounds that keep commands atomic, supported games saveable and parsing/replay cancellable. Xiangqi encoding/copy costs are already MSH-227; native consistency coordination is MSH-209. Do not replace game-rule limits, such as Shogi move-count rules, with application limits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure admission/replay time and allocations or copied/encoded bytes for representative short and long Tori, Chu and Tenjiku histories; identify unused or redundant accounting
- [ ] #2 Remove unnecessary work models, hypothetical-event sizing and whole-history copies in bounded reviewed steps while preserving no partial mutation, no loss of the accepted game and bounded cancellation
- [ ] #3 Rules fixtures and perft remain unchanged; record line counts and pass required Rust/frontend/affected-platform checks after each step
- [ ] #4 Remove unused multi-fact/omitted-resource accounting from native and frontend diagnostics, unless a concrete current producer requires it; retain bounded single-cause and unknown diagnostics
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure short/long native play and control histories and encoded bytes. Remove unused multiple-resource DTO and verify native/frontend contract. 2. Replace hypothetical ending encodes and redundant byte/work accounting with compact-record bounds; remove ordinary command and undo whole-history copies, with atomic rollback. Keep work counters that have actual engine validation consumers. 3. Fresh review, measurements, rules/perft and required Rust/frontend/platform checks for bounded steps; integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-229 fresh review also found dead outgoing resource-diagnostic generality: ResourceFailure::multiple/MAX_RESOURCE_FACTS/omitted_facts and frontend multiple-cause fallback have no production multi-fact producer. Every current refusal is one typed resource fact; multi-fact/omitted cases only exist in tests. Simplify this DTO and its callers alongside resource accounting; preserve the one real cause, unknown fallback, output byte/UTF-8 bounds and useful limit/recovery feedback.

Started from main3111f8e on codex/msh-232-resource-admission. Inspect native admission/copy paths and resource diagnostics; capture representative short/long baseline measurements before changes. Rules, saved compatibility, atomic admission and cancellation remain required. MSH-231 macOS CI exception was task-specific; attempt required platform checks for this task normally.

Baseline at 66c251d (main code3111f8e): reusable resource_admission example, release -j2, serial M-series Mac measurements (11 command/undo samples, 5 replay samples). Short/long play0/16/128 for Tori, Chu and both Tenjiku profiles; draw controls16/384 for Chu/Tenjiku. Full CSV/logs in runs/msh-232. At128: Tori admission44.4us/replay5.64ms/16225 encoded bytes; Chu437.7us/345.79ms/17749 bytes; Adams268.8us/28.70ms/27750 bytes; Muller271.6us/28.94ms/27506 bytes. Chu controls16->384 grow29.8->272.4us admission and4.2->85.1us undo. Baseline source171342 lines. Redundant accounting: Chu serializes accepted history plus two cloned hypothetical resignations per command; Tenjiku repeats hypothetical ending checks and charges serialized bytes as replay work; Tori copies complete history and caches event byte sums. Actual Chu/Tenjiku replay counters have engine validation consumers and need separate treatment. Step1 in progress: singular ResourceFailure fact plus bounded diagnostic; no production multiple-cause producer. Frontend/native checks running; fresh review assigned. Branch codex/msh-232-resource-admission.

Step1 checkpoint: native refusals and frontend now carry one fact plus the existing bounded UTF-8 diagnostic. Fresh review found no remaining issues after fixing fixture indentation. TypeScript and600 frontend tests pass;192 native session tests pass; workspace clippy and fmt pass. Workspace tests still running. Source171455 lines (+113 includes reusable benchmark). Byte columns measure saved record size and one command growth, not allocations or cumulative internal encoding. No rules or saved-format changes. Next: verify generated fixtures and full workspace result, then native admission/copy simplification and separate review.

Step1 complete at530b771: fmt/clippy pass;192 native session and600 frontend tests pass. Linux routine45217663343 and generated-fixtures45217689152 pass. Required debug workspace attempt hits the same four known MSH-199 Chu timing failures (30.42s); no tests loosened. Native checkpoint: compact64KiB root/256-byte command/event-count proof replaces hypothetical endings and byte caches; Tenjiku no longer charges removed serialization work. Actual Chu/Tenjiku native work counters remain because engine validation consumes them. In-place private transactions retain the original local owner plus append-only lengths; Chu undo no longer holds whole histories, and redo keeps stable semantic nodes with fresh runtime authority. Fresh reviews found/fixed stable-node redo, exact local-owner rollback and accidental removal of an unrelated Chu test (restored verbatim). Final reviews have no remaining findings. All200 core library tests and workspace fmt/clippy pass. Final workspace/release/frontend/measurements and affected-platform persistence CI remain. Main3111f8e merged (already current). Branch codex/msh-232-resource-admission.

Final ownership cleanup removes now-unshared Tenjiku Arc snapshots and unused private Chu GameData clone traits; fresh review confirms no hidden sharing consumers. Source171583 lines including the reusable measurement harness and added bound/rollback tests (baseline171342; first step171455). Machine is Apple M1 Pro; all local Rust jobs limited to2 workers. Initial repeat measurements show128-event Tenjiku admission about13-15us versus269-272us and Chu384-control admission15.8us versus272us; final quiet measurements still pending. Initial native run200/200 and frontend600/600 pass; required debug workspace again has the same4 MSH-199 timing failures at30.39s. Release workspace is building; Linux persistence CI on ee68296 passes, Windows running, macOS queued without a runner. Native final-code checks and final platform verification remain. Evidence baseline uploaded to gs://mshogi-experiments-216675633801/runs/msh-232/. No cloud compute used; cost below$0.01 for small storage.
<!-- SECTION:NOTES:END -->
