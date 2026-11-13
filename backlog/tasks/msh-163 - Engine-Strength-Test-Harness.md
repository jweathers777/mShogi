---
id: MSH-163
title: Engine Strength Test Harness
status: Done
assignee:
  - '@claude-opus'
created_date: '2026-11-13T21:28:00Z'
updated_date: '2026-11-13T23:32:00Z'
labels:
  - handle:engine-strength-test-harness
milestone: m-5
dependencies: []
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 167000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Strength Test Harness.

**Problem:** Chess engine changes were judged on small position panels and by explanation. No quick test showed if a change made the engine stronger, so work moved to isolated cases.

**Objective:** Give each later engine change a quick, repeatable acceptance test.

**Approach:** Three parts. (1) A fixed speed benchmark: nodes per second and time to depth on a fixed position set. (2) A development blunder suite from the confirmed level-to-lost positions of MSH-161 (Competitive Gameplay Diagnosis), each with its full history and the reference alternative; these positions are exposed and are for development only. (3) A game test of a candidate against a frozen predecessor at a fast time control, with paired openings and a sequential stopping rule, on the existing experiment tools and cloud runner (see MSH-162 (Parallel Experiment Runner)). Do not build a second framework.

**Completion:** A recorded baseline for the current engine on all three parts, and one command per part that a later task can run.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The speed benchmark reports nodes per second and depth for the current Chess engine on a fixed, recorded position set
- [x] #2 The blunder suite holds the MSH-161 confirmed level-to-lost positions with full histories and reference alternatives, and reports how many the engine avoids at a stated time
- [x] #3 The game test plays candidate against predecessor with paired openings and a sequential stopping rule, and records games, result and actual compute
- [x] #4 A baseline result for the current engine is recorded for all three parts
- [x] #5 A self-test of the game test (current engine against itself) gives a result consistent with equal strength
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
1. Blunder suite data: extract the 146 MSH-161 confirmed level-to-lost mShogi positions (root E .30-.70, played E <.15 at 500k, 20M supported_finite) with initial FEN, full history, blunder and 20M reference alternative into tools/strength/data (tools/strength/extract_blunders.py, saved evidence only).
2. tools/strength package, one command per part over the mshogi UCI product path: bench (fixed position set, go depth, nodes/time/nps), blunders (go movetime T per position, avoided count, reference match), match (native mshogi-match run, one paired opening per process, parallel local workers, pentanomial GSPRT stopping rule, max-pair cap, Elo estimate and interval, wall/CPU compute record). build-engine REV builds a frozen predecessor from a Git revision via git archive into target/strength.
3. Openings: 90 MSH-161 balanced master roots with full prefixes (development material).
4. Unit tests for the statistics, suite loading and result parsing.
5. Baselines on this Mac: bench, blunder suite at 1 s, self-test match at a fast Fischer control; record in docs/m5/strength-harness.md and an evidence JSON.
6. Level 1 verification, independent review, integrate on codex/rewrite.
Decision: the game test runs on the existing native mshogi-match runner with local processes. A GCP job kind is not added now; MSH-162 cloud execution stays available if a later test needs more compute.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @claude-opus (Claude Code session), branch codex/rewrite in primary checkout. Backlog CLI 1.55.3 matches baseline. Also: .gitignore now ignores docs/**/*.log (commit 9c46eb1f, owner request); 419 log files tracked earlier stay tracked until John decides.

Verification level 1 (routine local tooling; no production Rust change). Environment: Apple M1 Pro, macOS 27.0, Python 3.11.0, Rust 1.99.1.
Checks at c2595a18: python3 -m unittest tools.strength.tests.test_strength -> 26 passed (includes one real paired game through mshogi-match run). Native controller accepted all 90 openings during smoke runs. tools/experiments tests: 13 errors/2 failures pre-existing and unrelated (no python chess package; stale quick-status settings; one process test) - no tools/experiments file changed.
Independent review (Claude subagent, fresh context, read-only): 11 findings (SPRT small-sample/degenerate variance, opening order, exception handling, depth parsing, compute accounting, cancellation race, blunder metric meaning, bench depth check, test strength, strict JSON, tar pipe). Resolved in 2cdae107; re-review confirmed all resolved and found one new defect (Win/Draw native conclusions in clock mode), resolved in c2595a18 with tests.
Decision: game test default is fixed 200 ms per move because the engine loses on time in ~77% of 5 s + 0.05 s clock games (recorded on MSH-167). Cloud execution not added; local runner suffices (self-test 46 min wall).
Baseline evidence: docs/m5/evidence/msh163/baseline-65887fb0/; guide docs/m5/strength-harness.md. Frozen predecessor: target/strength/engines/65887fb0c653/mshogi (rebuild with build-engine 65887fb0).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added tools/strength (python3 -m tools.strength.cli): bench (fixed 18-position set bench-chess-v1, nodes, time to depth, nps), blunders (146 MSH-161 confirmed level-to-lost positions with full histories and 20M reference alternatives, avoided count at a stated move time), match (candidate vs frozen predecessor on native mshogi-match run, 90 paired MSH-161 openings in seeded order, generalized SPRT with regularized variance and minimum pairs, games/result/compute record) and build-engine (frozen predecessor from a Git revision). Why: MSH-164 and later engine work need a quick, repeatable acceptance test.
Baseline for engine 65887fb0 on Apple M1 Pro: bench 5,431,097 nodes, median 296,656 nps; blunders at 1000 ms avoided 47/146 (reference 17); self-test at 200 ms/move 600 pairs, no SPRT decision, +277 -258 =665, Elo +5.5 [-3.0, +14.0], consistent with equal strength; 21,305 CPU s.
Limitations: blunder positions and openings are exposed development material; 'avoided' is not proof of a good move; clock mode is blocked in practice by the engine's time management (MSH-167); at 200 ms the default [0,15] SPRT needs more than 600 pairs near zero difference; results are machine-specific. Also .gitignore now ignores docs/**/*.log (9c46eb1f). Integrated on codex/rewrite: 9c46eb1f, 65887fb0, 32feefb7, 2cdae107, c2595a18, 731974f6 and this record.
<!-- SECTION:FINAL_SUMMARY:END -->
