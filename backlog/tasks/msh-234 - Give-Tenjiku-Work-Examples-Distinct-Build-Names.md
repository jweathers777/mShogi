---
id: MSH-234
title: Give Tenjiku Work Examples Distinct Build Names
status: Done
assignee:
  - '@claude'
created_date: '2026-11-23T12:31:00Z'
updated_date: '2026-11-25T10:56:00Z'
labels:
  - tooling
  - model:small
  - effort:low
dependencies: []
priority: high
type: chore
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Workspace builds warn that mshogi-core and mshogi-search both emit the tenjiku_work example executable and debug symbols. Give the examples distinct names and update their callers so workspace builds cannot overwrite one example with the other.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Workspace builds emit no duplicate-output warning for the Tenjiku work examples
- [x] #2 Both diagnostic examples and their documented callers remain usable
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Renamed mshogi-core example tenjiku_work to tenjiku_rules_work and mshogi-search example tenjiku_work to tenjiku_search_work (git mv). Only the core example header named the command; updated it. No Cargo.toml [[example]] entries existed. Evidence: before the change, cargo build --workspace --all-targets --locked emitted two output filename collision warnings for tenjiku_work; after, zero collision warnings. Both renamed examples ran with small arguments (core: adams 8 play, exit 0; search: 1 1000, exit 0). Checks: cargo fmt --check passed; cargo clippy -D warnings passed; cargo test --workspace --locked: 1186 passed, 0 failed, 11 ignored. No behavior changes.
<!-- SECTION:FINAL_SUMMARY:END -->
