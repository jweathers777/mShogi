---
id: MSH-201
title: Chu Search Speed
status: In Progress
assignee:
  - codex
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-24T16:15:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies:
  - MSH-213
references:
  - docs/m5/strength-roadmap.md
ordinal: 208000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: make the Chu search faster with conventional techniques. Measure where search time goes (move generation, make and unmake, legality and check tests, attack tests), then remove the largest costs. This includes the unusual pieces (Lion double moves and igui, Lion-like moves of the Horned Falcon and Soaring Eagle, and the Lion-capture rules). Keep a change only on evidence: correctness checks, a speed measurement, and a game test against the frozen predecessor.

Native rules stay the authority. A faster search-side position must agree with them in perft and differential tests. No rule changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Baseline and final speed are recorded on the same machine and positions: nodes per second and time to a fixed depth, with the cost profile that guided the changes
- [ ] #2 Chu perft and differential tests against the native rules pass
- [ ] #3 Each kept change passed the blunder suite and a game test against the frozen predecessor; rejected changes are recorded with their result
- [ ] #4 A short design note in docs/ explains how the unusual pieces are generated and checked in search
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze the accepted engine and a benchmark set covering standard openings, real middle/endgame histories and unusual-piece tactics. Measure fixed-depth search and time spent in generation, make/unmake, threat/rule checks and exchange tests on c3-standard-4. 2. Remove measured hot-path costs without changing rules, evaluation, move ordering or search policy where possible. 3. Run native perft/differential and exact fixed-depth comparisons, tactical probes and a bounded clocked predecessor SPRT (H0=0,H1=15,alpha=beta=.05;10s+0.1s nominal, same frozen openings). Fix run size after the measured speed/pilot within US$10. Retain only supported changes; record rejected attempts. 4. Write the unusual-piece search design note, run required checks/review, integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20, John: rescoped from a discussion task to an ordinary speed task; agents choose the techniques. Blocker: Chu has no strength harness yet (opening set, blunder suite, game test against the predecessor; see MSH-204 (Tori Strength Harness) and docs/m5/strength-roadmap.md). Speed profiling and perft/differential work can start; AC 3 needs the harness.

Harness blocker is now the dependency MSH-213 (Chu Strength Harness).

Started from main182f017 after MSH-213 completion. Frozen predecessor engine source4f5e2ed and harness inputs/results are recorded in MSH-213. No compatibility obligation or game-rule change. Budget US$10; measurements and heavy builds use c3-standard-4 cloud workers.

Benchmark tooling ready before optimization:18 frozen positions (first8 opening catalog entries, first case of each4 tactical families,2 explicit Horned Falcon/Soaring Eagle diagrams, first completed source game prefixes80/160/240/320). Baseline pilot uses depth3,3 fresh-table repeats and a separate coarse timed-wrapper pass; wrapper asserts exact full search-output equality with the untimed pass. Timings exclude native root setup and separately report it. No product search change yet. New chu_bench helper and manifested executable selection reuse the cloud worker. Next build/profile baseline, then choose the measured hot path. Branch codex/msh-201-chu-search-speed.

Baseline helper source4599c86 built successfully on c3-standard-4; SHA25651519869cf9bd88fb9edef60348b57799e18366e4be5866fb00700fd1a275ac7. Build cleanup verified; conservative compute upper bound US$0.143. Worker image pinned sha256:c15692c83e248b70e8c758662674559691ae4f362141dc7b045cd7f3c122f191 (Cloud Build43d16165-e11a-4402-8514-6080e43d2f01). Baseline profile job msh213-msh201-profile active, using the existing Chu worker prefix; GCS runs/msh-201/baseline-profile. No search implementation change yet.

Baseline depth3 profiling (18 fixed positions,3 uninstrumented repeats plus exact-result-checked instrumented pass):exchange34.3%, generation/route admission22.6%, make/threat17.4%, rule/capture threat9.0%, unmake1.8%. Total profiled time0.133s; individual uninstrumented medians0.07–58.25ms. The repeated direct_reaches scan is shared by exchange filtering, move admission and threats. First change checks the target displacement and allowed ray direction before scanning only its intervening squares; no tables, pruning, ordering, evaluation or limits change. Final same-VM comparison is fixed before results at depth5,5 repeats, baseline/candidate/baseline order, with complete search-signature equality required.

First candidate implemented: direct_reaches computes mover-relative displacement once, checks direct movement, rejects nonaligned/unsupported rays, and scans only the target ray. No allocation, precomputed table, ordering, evaluation, selectivity or rules change. Chu checks28 passed/1 ignored, including native perft, >150 sourced fixtures, randomized differential/undo and exact successor comparisons. Search Clippy passes. A fresh movement-code review is active. Next cloud build and same-VM depth5 comparison before game tests.
<!-- SECTION:NOTES:END -->
