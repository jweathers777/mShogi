---
id: MSH-240
title: Retire finished-campaign strength tools
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T09:51:00Z'
updated_date: '2026-11-25T11:26:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
dependencies: []
priority: high
ordinal: 2500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Some tools/strength scripts are kept unchanged only to re-verify evidence of finished campaigns (for example shogi_cloud.py, kept by shogi_campaign.py for frozen evidence). Remove the tools and tests that only support earlier development versions (AGENTS.md zero backward compatibility).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each tools/strength script that only re-verifies finished-campaign evidence is removed, with its tests, or has a recorded current use.
- [x] #2 Current match, screening and cloud tools still work and their Python tests pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory tools/strength: importers, callers, docs, Rust includes, CI.
2. Remove scripts without a concrete current use, with tests and sole-use data; move shared helpers; update docs.
3. Run Python and Rust checks, smoke test the kept tools, one fresh review, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: run before MSH-209, so the review has less code to read.

Kept tools and their current use (one line each):
- native_match.py: shared predecessor SPRT driver (Shogi, Tori, Chu, Tenjiku; MSH-214, MSH-202).
- stats.py: SPRT and Elo statistics for native_match, cli and xiangqi_games.
- cli.py: build-engine freezes predecessors (strength-roadmap.md); bench, blunders and match are the only Chess screening and predecessor tools (AGENTS.md strength check, MSH-55); native_match imports its identity and write helpers.
- uci.py: UCI client and file hash used by cli.py.
- xiangqi_games.py: only driver for a clocked Xiangqi predecessor SPRT (native_match accepts no Xiangqi clock or opening catalog), needed to test a shared-core change on Xiangqi.
- cloud_build.py: Batch compile of Linux binaries (chu-strength-harness.md).
- chu_cloud.py: latest cost-bounded measurement submission; base for the Tenjiku harness (MSH-214).
- native_signature.py: fixed-depth identity comparison that chu_cloud runs; reuse for speed work (MSH-202).
- anchor_cloud.py: cost-bounded Batch controller for chu_cloud and cloud_build; now also holds require, put_new and cloud_environment.
- texel_data.py: Texel corpus split (MSH-214).

Data: all opening, tactical, proof and blunder suites are kept (roadmap suites, Rust tests and examples, match inputs). Only data/tori-cloud-data-files.json (used only by tori_cloud) is removed.

For MSH-209 (duplication in what remains, not changed here):
- Three pair/SPRT drivers: native_match.py, xiangqi_games.py, cli.py match.
- Two environment() functions (cli, experiments/core); file hashing in uci, texel_data and experiments/core; two write-new-JSON helpers (cli.write_new, anchor_cloud.save).
- chu_cloud.py repeats manifest fetch and receipt code of cloud_build, validates with assert, and is fixed to Chu and prefix msh213; native_signature.py is fixed to Chu and protocol 6 and has no help text.
- core.file_hash and GCSStore.delete/exists have no caller outside tests.
- anchor_cloud.py has a historical name and fixes c3-standard-4 and eight workers.
- native_match.py keeps the Tori length-limit rule; cli.py match keeps --baseline-movetime; xiangqi_games.py keeps external-engine and Hash options of the removed ladder.
- docs/m5/strength-harness.md cites extract_blunders.py, which was already absent before this task (MSH-235).

Review (fresh subagent, diff 52f39b8..39c4c84): no real defect. One finding, fixed: the wrong-region and immutable-upload tests of the moved helpers were deleted with test_anchor_worker; they are now in test_anchor_cloud.

Checks on macOS arm64, no cloud use. Python before -> after: strength 175 (1 skipped) -> 62 (1 skipped without release binaries, 62 pass with them); experiments 30 -> 30; desktop/scripts 177 -> 177; ci_checks self-test 8 -> 8. cargo fmt and clippy clean; cargo test --workspace --locked 1186 passed, 11 ignored, 0 failed. Help of each kept command-line tool prints. Local smoke runs with release binaries passed: cli bench, blunders and match; native_match --game chu calibration; native_signature (64 identical rows); xiangqi_games run, summary and sprt. Not run: any cloud submission (chu_cloud, cloud_build) and a real cli build-engine build.

Final run after merging origin/main (no new commits there): fmt and clippy clean; cargo test 1186 passed, 11 ignored, 0 failed; Python strength 62, experiments 30, desktop/scripts 177, ci_checks 8, all pass; help and imports of kept tools pass; the lazy imports of cloud_build resolve and it builds its source file list offline.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed 27 tools/strength scripts of finished campaigns and earlier development versions (rating anchor, per-game campaign, ladder, judge, assessment, self-play and data-builder scripts, the Shogi wrapper and Shogi subcommands of cli.py), 13 test files and tori-cloud-data-files.json: 7,194 lines deleted, 158 added. Kept anchor_cloud, chu_cloud, cli, cloud_build, native_match, native_signature, stats, texel_data, uci and xiangqi_games; the uses and the duplication list for MSH-209 are in the notes. The cloud controller now holds the three worker helpers and requires an explicit campaign policy. All suite and opening data are kept. Offline and local checks pass; no cloud job was run. Follow-up: MSH-242 (Retire Rust strength examples without a consumer).
<!-- SECTION:FINAL_SUMMARY:END -->
