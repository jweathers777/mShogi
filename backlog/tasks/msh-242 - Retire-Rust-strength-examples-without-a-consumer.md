---
id: MSH-242
title: Retire Rust strength examples without a consumer
status: Ready
assignee: []
created_date: '2026-11-25T11:20:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-240
priority: medium
ordinal: 7950
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-240 removed Python tools that were the only consumers of some Rust strength examples and example modes (for example shogi_tsume, chu_judge, and the judge and mining modes of tori_strength and xiangqi_strength). Remove what has no concrete current use, so that later cleanup and renaming have less code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each example in crates/mshogi-runner/examples and crates/mshogi-search/examples that served strength campaigns has a recorded current use or is removed with its support files and tests, without a rules, search or evaluation change
- [ ] #2 Documents that name a removed example or mode are updated and the workspace checks pass
<!-- AC:END -->
