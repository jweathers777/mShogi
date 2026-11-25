---
id: MSH-263
title: One Per-Game Table for Native Experiments
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
labels:
  - runner
  - refactor
  - model:medium
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7650
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
crates/mshogi-runner/src/experiments/native.rs repeats one match arm per game in from_root, root, turn, outcome and admit; the Tori, Chu and Tenjiku admit arms are the same shape; Shogi and Xiangqi build actor and command twice each; the Xiangqi score copies the one in xiangqi.rs. experiments/mod.rs has further per-game tables (valid_work, first_side), the test PROFILES list copies the one in mod.rs, and run.rs repeats the player and step decision logic of the report validation in mod.rs. MSH-245 (One Clock Policy for All Games) changes the clock part of mod.rs first.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Per-game experiment behavior comes from one trait or table; arms of the same shape are merged; the run and the validation share one decision function.
- [ ] #2 Experiment tests and recorded-report replay pass; production line counts before and after are recorded.
<!-- AC:END -->
