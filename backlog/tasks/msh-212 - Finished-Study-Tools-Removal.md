---
id: MSH-212
title: Finished Study Tools Removal
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T22:29:00Z'
updated_date: '2026-11-20T23:07:00Z'
labels:
  - tools
milestone: m-5
dependencies: []
ordinal: 219000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: remove tools that only served finished studies and whose inputs were in evidence directories removed by MSH-211 (Git History Cleanup): the MSH-161 Chess study runner (tools/experiments/chess.py, the study parts of cli.py and core.py, tests/test_chess.py, status-settings.json, quick_status.py if unused), tools/strength/extract_blunders.py, make_openings.py and request_overhead/summarize.py. Keep tools/experiments core.py and gcp.py, which the strength tools use.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No tool or test refers to a path under docs/*/evidence/
- [x] #2 The remaining strength cloud tools import and their unit tests pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Remove the MSH-161 study runner: tools/experiments/{chess,cli,quick_status,package}.py, status-settings.json and their tests; reduce core.py to encode/decode/digest/file_hash/environment. 2. gcp.batch_config takes the worker command from its caller (anchor_cloud already overrides it). 3. Dockerfile: generic worker image, no study files, entrypoint python3. 4. Remove tools/strength/extract_blunders.py, make_openings.py and request_overhead/ (MSH-189 one-off). 5. Remove docs/experiments study records (parallel-runner, coordinator handoff, pilot run data, review, verification); keep the GCP setup guide without the study status section. 6. Run tools/experiments and tools/strength unit tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Removed: tools/experiments chess.py, cli.py, quick_status.py, package.py, status-settings.json and their tests; core.py reduced to encode/decode/digest/file_hash/environment (new focused tests); gcp.batch_config now takes the worker command from the caller (anchor_cloud passes it), unused submit_job/cancel_job/cost_estimate removed; Dockerfile is a generic worker image (entrypoint python3, no study files); python-chess dropped from worker requirements. tools/strength extract_blunders.py, make_openings.py and request_overhead/ removed (outputs stay in tools/strength/data). docs/experiments study records and pilot run data removed; the GCP setup guide stays without the study status section. Checks (macOS, Python 3.11.0): tools/experiments tests 30 passed (before: 1 failure, 13 errors from study imports); tools/strength tests 153 passed before and after; every tools.strength module imports; cargo fmt clean. Code and tests no longer read docs/*/evidence/; many old docs/m4 and docs/m5 reports still link to removed evidence, left for the docs trim.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed the finished MSH-161 study runner and three one-off strength scripts whose inputs were deleted by MSH-211 (Git History Cleanup), about 5,500 lines. Kept and narrowed the shared cloud helpers (core.py, gcp.py) that the strength tools use; the Batch worker command is now explicit. Experiment and strength tool tests pass. Remaining stale links in old docs reports belong to the planned docs trim.
<!-- SECTION:FINAL_SUMMARY:END -->
