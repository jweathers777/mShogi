---
id: MSH-186
title: Shogi Strength Harness
status: Done
assignee:
  - '@codex'
created_date: '2026-11-16T20:51:00Z'
updated_date: '2026-11-17T02:22:00Z'
labels:
  - handle:shogi-strength-harness
milestone: m-5
dependencies:
  - MSH-185
documentation:
  - docs/m5/strength-harness.md
  - docs/m5/strength-roadmap.md
  - docs/m5/shared-search-core.md
  - docs/m5/experiments.md
  - docs/m5/evidence/msh180/README.md
priority: high
ordinal: 193000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Strength Harness

## Problem
Chess progress came from a measurement harness: a speed benchmark, a development blunder suite, curated openings and a sequential game test against the frozen predecessor (MSH-163 (Engine Strength Test Harness), docs/m5/strength-harness.md). Shogi has only part of this. The Shogi game test (`tools/strength/shogi_match.py` through `native_match.py`, used by MSH-180 (Shogi Search Core Adoption)) uses fixed node limits and seeded random short openings. Shogi has no benchmark positions, no tactical or blunder suite, no curated opening set, no clock-mode game test and no labeled positions for Texel tuning. It also has no approved independent opponent pool, which MSH-50 (Conventional Shogi) needs before any strength verdict.

## Objective
Give Shogi the same development measurements that Chess has, so that MSH-187 (Shogi Strength Round) can accept or reject changes on evidence. Propose an external opponent pool for John's decision.

## Approach
- Extend the shared tools in `tools/strength` for Shogi. Do not build a second framework.
- Benchmark: fixed SFEN positions (opening, middlegame, endgame, positions with pieces in hand) with fixed depths, reporting nodes per second and time to depth.
- Tactical suite: positions with independently established answers. Good sources are published tsume (mate) problems with known solutions and checked positions from real games. Record source and license for each set. Keep exposed development positions separate from any later held-out set.
- Openings: a balanced opening set from established opening lines or professional game prefixes, with source and license recorded, used in pairs with colors reversed.
- Game test: add a clock mode (base + increment) for Shogi, as for Chess, using the clock and session support of MSH-179 (Clock and Session for All Games). Keep the sequential stopping rule of the Chess harness.
- Texel data: a reproducible way to make labeled Shogi positions (for example from self-play games of the current engine at a fast control), so that MSH-187 can tune evaluation weights.
- Record the baseline of current main with all of these tools.
- Opponent pool proposal (no rating games in this task): research USI engines that can play the accepted profile `mshogi.shogi.csa36.v1` (fourth-occurrence repetition, perpetual-check loss, entering-king declaration). Candidates include Fairy-Stockfish and YaneuraOu (also at reduced settings, for example classical evaluation or node limits) and other open engines. For each: license, version, source, profile compatibility, settings, approximate strength and fit with the existing USI adapter (`experiment-external usi`). Present a short recommendation with a bounded baseline protocol and budget for John's approval. External engines stay measurement tools only, never rules oracles or build/CI dependencies.

## Completion
Tools, data, baseline numbers and the opponent proposal are documented in docs/m5 (link from docs/m5/strength-harness.md and docs/m5/strength-roadmap.md). Independent review covers the tools and the data sources. Targeted verification level 2 for native clock execution and POSIX process supervision; no production rule or search change. Verify the actual macOS process path and record deferred Linux/Windows scope without claiming unperformed platform checks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shogi benchmark, tactical suite and opening set exist in tools/strength/data with recorded sources and licenses, and the strength CLI runs each one for Shogi.
- [x] #2 A Shogi clock-mode predecessor game test with the sequential stopping rule runs end to end; native rules decide and replay every scored game.
- [x] #3 A reproducible procedure makes labeled Shogi positions for Texel tuning, with documented size and generation conditions.
- [x] #4 Baseline results of current main for benchmark, tactical suite and depth at a stated time control are recorded with revision and environment.
- [x] #5 A written opponent-pool proposal (engines, versions, licenses, csa36 compatibility, settings, protocol and budget) is presented to John; no external rating games run in this task.
- [x] #6 Independent review accepts the tools and data sources.
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
1. Freeze task-start main d75ecdb8 and inspect the existing shared protocol client, native match runner and data-generation path. Record bounded baseline methods before measurements. 2. Add Shogi bench and tactical commands by extending shared CLI/protocol mechanisms; obtain independently justified tactical answers and sourced, licensed opening lines, with native validation. 3. Extend the existing native predecessor runner for Fischer clocks and fixed opening records, retaining paired colors, native replay, failure records and the existing sequential test. 4. Add a bounded reproducible Shogi labeled-position procedure that uses native records and outcomes. 5. Record current-main benchmark, timed-depth and tactical baselines plus a short clock-game/data-generation demonstration. Use cloud execution if the planned local measurement exceeds one hour; no external rating games. 6. Prepare an independently sourced USI opponent proposal with compatibility limits, protocol and budget for John; independently review tools/data, resolve findings, document, integrate into main and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex chat 01a1e634-7fc9-72c2-b4b4-07022a7df3ec. Primary checkout was clean main d75ecdb8, with no active Backlog owners; this chat completed the prior task here. New branch codex/shogi-strength-harness. Other worktrees remain untouched. Backlog 1.55.3 matches project baseline. Scope: shared Shogi measurement tools, licensed development data, baseline and an external-opponent proposal; no external rating games or production rule/search change. Verification level 1 as scoped; actual environment and evidence will be recorded.

Implementation checkpoint: timed USI is not supported, so timed probes and game clocks use the existing shared native protocol. The shared experiment controller needs optional Fischer ledger/allocation fields; native rule and search code stay unchanged. Baseline freeze d75ecdb8. Planned local measurement: nine fixed-depth roots (three repeats), twelve development tactics at1000ms and a small clock-mode self-comparison of unchanged search against frozen main, at5s+0.05s, four workers, maximum32pairs/900s. The objective is to prove useful measurement paths and record a baseline, not claim a strength gain. Retain failures and all launched pairs; use existing SPRT, no forced significance verdict. Export native-result labels from these same games, with no added data-generation campaign. Expected total local measurement well below one hour; stop/reassess if bounds or technical failures prevent that. No external rating games or cloud spending is authorized by this task.

Coherent implementation checkpoint: shared native experiment Fischer clocks and replay ledgers, fixed legal opening catalog, native probe/proof/export helper, shared strength CLI Shogi bench/tactics/validation/export, and game-aware Texel partitioning. Clock code independently accepted after healthy multi-turn and watchdog tests. Native data validation passed after independently refuted unprotected-piece answer corrections. Bench depths frozen from recorded d75ecdb8 calibration. Current checks:102Python tests pass; runner library plus two examples pass (counts in target/msh186/native-tests.log). Outer timeout process-group test needed unsandboxed process inspection; actual test passed. Next: commit source checkpoint, rebuild native instrumentation, then run final baselines and32pair bounded same-frozen-engine clock self-comparison; no strength-improvement claim.

Final frozen-engine probes at clean tooling e80f00a8 passed: native data9/12/6; fixed-depth three repeats1594093nodes each, median1.758934875s; short tactics12/12 across6geometries; ordinary opening timed depths14,16,11,11,10. Synthetic mate root depth64 is not ordinary depth; single timed repeat proves no determinism. Runner clippy all-targets warnings denied and workspace all-targets check pass. Bounded clock self-comparison started in target/msh186/clock-selftest, session21870, same frozen engine both sides,5+.05,4workers,max32pairs/900s; no external games or gain claim.

Clock checkpoint: first batch stopped after4pairs/45.383s as required:4native conclusions,3incomplete:Stopped at27-29ms request budgets before depth1,1artifact_limit at16MiB shared report cap. All8reports replay and cleanup is empty; only1completepair enters SPRT, no decision. Preserve run unchanged. Decision: increase bounded report capacity for full320ply histories and rerun one6pair catalog demonstration at5+.2s,4workers,max900s (same frozen engine). This is an execution check, not a significance campaign; no search/rules changes. Reviewer checks actual ledger and cap correction.

Final measurement: follow-up at5+.2 completed all6pairs/12native conclusions in236.400s,577.919summed pair wall-seconds; no cleanup/unscored failures. Same binary nominal5/0/7, no strength inference or SPRTdecision. All1973ledgers independently checked; one34.6MB report exceeds old32MiB cap. Native export497positions,488unique afterdedup; all6opening families hashtrain, explicit demonstration training_readyfalse with emptyvalidation/heldout. Strict split refuses beforewriting. Review corrected verification to targetedLevel2 for processsupervision: actualmacOS POSIX childcleanup/clock checks; no Windows orchestration supportclaim, Linuxruntime deferred. No cloud/Docker work for this task. Durable record docs/m5/evidence/msh186/verification.md; rawgeneratedcorpus ignored, native compressed inputs committed.

Independent tools/export/partition/proposal review accepted at c1b8b479; clock/data reviewers accepted final source and receipts. All six acceptance criteria have evidence in docs/m5/evidence/msh186/verification.md. Corpus is explicitly generation-only, not fit-ready; MSH-187 (Shogi Strength Round) needs independent families before tuning. Exact native source tests and unchanged-source reuse recorded. Ready for normal integration into main; no further study authorized.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented shared Shogi benchmark/tactics/data validation, native clock-mode paired games with replay/SPRT, native-result label export and game-aware Texel partitioning. Added licensed development roots/openings with independent proof/source review and a bounded external-opponent proposal; no external games ran. Frozen engine d75ecdb8 baseline:12/12 short mate cases over6geometries; ordinary opening depth10–16 at1000ms; fixed-depth1594093nodes,1.7589s median. Initial clock attempt failed transparently on short request budgets/artifact capacity; finite128MiB evidence cap and5+.2 follow-up completed12native games, allreplayed/cleanupclean, no strength or sequential decision. Export497samples,488unique; emptyvalidation/heldout explicitly nottrainingready. Tests:102Python;34native+5example,13focused cap regressions; clippy/workspace check passed. Independent tools/data/clock/proposal reviews accepted. TargetedLevel2 macOS POSIX supervision evidence; Linuxruntime deferred, Windowsorchestration unclaimed. Verification and receipts:docs/m5/evidence/msh186/verification.md; user report:docs/m5/shogi-strength-harness.md. Integrated into main at dfd93a32, containing tested code c1b8b479 and initial source b5ec9861/e80f00a8; only completion metadata follows. Next eligible MSH-187 (Shogi Strength Round), not started; tuning needs more independent families, external study needs separate approval.
<!-- SECTION:FINAL_SUMMARY:END -->
