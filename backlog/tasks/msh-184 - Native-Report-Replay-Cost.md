---
id: MSH-184
title: Native Report Replay Cost
status: Done
assignee:
  - '@codex'
created_date: '2026-11-16T15:38:00Z'
updated_date: '2026-11-16T20:13:00Z'
labels:
  - handle:native-report-replay-cost
milestone: m-5
dependencies:
  - MSH-183
documentation:
  - docs/m5/evidence/msh183/round2/README.md
priority: medium
ordinal: 191000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Native Report Replay Cost.

**Problem:** MSH-183 (Tenjiku Search Core Adoption), round 2 Adams, wrote a complete 9.8 MB native report but exceeded the outer 645-second process bound during post-write processing. Execution, JSON export and summary generation each validate the report. Actual request time was 570.69 seconds; the validation passes were not timed separately. This cost is separate from the candidate node-limit failure.

**Objective:** Export and summarize a completed native experiment with bounded cost while keeping native replay and strict report validation authoritative.

**Approach:** Measure the full path on the retained long report and ordinary smaller reports. Remove repeated work where evidence supports it. Preserve rejection of forged, stale, malformed and inconsistent external reports, reproducible failures and process cleanup. No game-rule or score-admission change is authorized.

**Completion:** Measured reduction in repeated work, preserved native validation and independent review. Select verification from actual risk; no new playing-strength campaign is required.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Measure separate execution, validation, export and summary costs on the retained long Tenjiku report and ordinary reports; record baseline, bounds and a decision checkpoint.
- [x] #2 Reduce repeated native replay in report export and summary generation without weakening external report validation or changing scores, native rules, refusals or cleanup.
- [x] #3 Relevant runner regressions, malformed-report rejection and independent review pass; record measured benefit and actual verification scope.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Baseline is main 900c7ba5. Owner: Codex, primary checkout, codex/native-report-replay-cost. Measure parsing/validation, JSON export and summary separately on the retained round-2 Adams report and ordinary smaller reports. Preserve historical execution time; use a small actual experiment for current execution-stage measurement. Initial profiling bound: 10 minutes wall time per round, with per-process bounds and no rerun of the long strength games.
2. At the baseline checkpoint, choose a minimal immutable validated-report boundary so one successful native validation can serve export and summary. Keep mutable/external Report entry points fully validating; preserve byte output, native outcomes, score admission and cleanup. No native-rule, search or opponent changes.
3. Compare identical reports and outputs after the change. Require measured reduction in repeated replay plus forged/stale/malformed rejection tests and runner CLI regressions. Verification Level 1 if the final change stays in portable in-process report handling: local runner/native evidence checks, workspace consumer compilation and independent review. Process supervision, filesystem contracts and GUI are unchanged; broader platform/packaged checks remain at the next integration/release checkpoint.
4. Record source-specific evidence and independent review, resolve findings, integrate into main, push and clean up the feature branch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Baseline checkpoint on frozen binaries before product edits: ordinary Chu native validation 0.088304 s, JSON export 0.088681 s, summary 0.087191 s; ordinary Shogi stages are about 1 ms each. The repeated native pass dominates the ordinary Chu export/summary path. Tiny actual two-game Chess CLI run took 0.090357 s including final checks; request elapsed and separate report validation remain separately labelled, not subtracted as an exact execution-only measurement. Long retained Adams measurement remains running under a 300 s process bound from the frozen binary. Decision: implement the owned immutable ValidatedReport boundary. One native validation can serve JSON/summary outputs, while mutable Report methods and external parsing keep full checks. Independent design review found no material issue.

Long baseline completed within bound: 10.32 MB, 103 steps, 2 games; raw parse 16.283516 s, explicit native validation 17.034410 s, JSON export 17.066660 s, summary 17.057348 s (67.512 s wall). Historical recorded request time is 570.687558 s, not a new run or an exclusive execution-stage measurement. Repeating validation in export and summary costs more than 34 s after a successful validation. Baseline three-report profiling took 68.153 s. Implemented owned immutable ValidatedReport with no deserialization or mutable access; consuming it returns untrusted editable Report. Existing Report methods still validate. Native JSONL projection replay remains required; its redundant report-validation passes are removed.

Candidate checkpoint: long export plus summary fell from 34.124008 s to 0.033189 s (99.90%); complete measured parse/validation/export/summary stages fell from 67.441934 s to 33.687309 s (50.05%). Full native validation remains about 17 s and strict native parsing about 16 s. Chu post-validation outputs fell from 0.175871 s to 0.000645 s; Shogi from 0.001923 s to 0.000060 s. All four input/output/summary hashes and complete summaries match. Candidate round took 34.142 s within 600 s. Decision: retain the minimal immutable boundary; do not expand into native parser/replay optimization. These are descriptive single-run stage timings, not a strength claim or proof the old 645 s full game campaign would now finish.

Implementation and measurements are ready for final review. Runner release suite: 84 passed, 2 ignored, including the immutable API compile-fail test. Console native experiment suite: 3 passed, covering all seven profiles, corruption rejection through both external constructors, output equality, incomplete outcomes and cancellation. Formatting and diff checks passed; workspace all-target clippy is running. Independent source review found no material defect. Level 1 remains appropriate: native process supervision and file writes are unchanged; standalone desktop host does not consume the runner API.

Final independent review accepted product source 97a718f8cde28703c064f3fcfcedb22b1ac4b241. The reviewer independently checked source/binary/input hashes, all four output and summary equalities, timing arithmetic and successful test/lint logs. No material findings. Corrected a prose timing rounding error: baseline round receipt is 68.154573 s (68.155 s rounded), not 68.153 s. All required Level 1 checks passed; no broader platform or GUI checks are represented as run. Ready for normal main integration; production diff from the tested source is empty.

Reviewed source 97a718f8 and evidence 21d30000 are integrated into main. The completion update is documentation-only; product source remains unchanged.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Native experiment execution now returns an owned immutable ValidatedReport after full native replay. JSON export and summaries reuse that validation. External JSON and edited mutable Reports still receive full checks; JSONL keeps the replay needed for native before/after states. Output formats, scores, rules, refusals, process cleanup and file-write behavior are unchanged.

On the retained 10.32 MB Tenjiku report, export plus summary fell from 34.124 s to 0.033 s; all measured stages fell from 67.442 s to 33.687 s. Three smaller reports support the same mechanism. All four encoded outputs and complete summaries match exactly. Timings are single sequential observations; the historical 645 s failure remains failed, and no long games or strength campaign were rerun. Native parsing and validation remain expensive. Decision: retain this bounded change and stop the round.

Level 1 verification passed on macOS arm64: runner 84 passed/2 ignored; actual console experiment tests 3 passed across seven profiles; all-target consumer check, workspace clippy, formatting and immutable API compile-fail check passed. Independent review verified source, receipt hashes, equality, timing arithmetic and logs with no material findings. Broader platform and packaged GUI checks remain deferred to the next integration/release checkpoint.

Integrated into main at 21d30000; tested product source is 97a718f8cde28703c064f3fcfcedb22b1ac4b241. Evidence and review: docs/m5/evidence/msh184/README.md and verification.json. The primary checkout was used; no extra worktree was created. Next ready task: MSH-173 (Frozen Legacy Opponent).
<!-- SECTION:FINAL_SUMMARY:END -->
