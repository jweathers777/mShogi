---
id: MSH-3
title: Establish the minimal Rust workspace and repeatable checks
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rust
  - build
  - ci
milestone: m-1
dependencies:
  - MSH-1
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Make a clean checkout build and run focused verification without a GUI or training environment.

## Design references

P-03; P-05; A-01; M1

## Scope

Create a small reusable Rust library and console binary, record a stable toolchain and dependency policy, define formatting/lint/test commands, and configure appropriate Windows/macOS/Linux CI. Add useful scaffolding only; no empty future subsystems.

## Verification expectations

Run the documented checks on the available host and inspect CI results for each claimed platform. Record unavailable platforms honestly. Confirm the library has no frontend, console-I/O, or Python dependency.

## Out of scope

Tauri/React scaffolding, database setup, Python training environment, generic plugin systems, and engine search.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reusable library and headless console binary build from documented clean-checkout instructions.
- [ ] #2 The stable Rust toolchain, dependency/lockfile policy, format, lint, and test commands are recorded and reproducible.
- [ ] #3 CI defines relevant checks for Windows, macOS, and Linux; actual results are distinguished from configuration alone.
- [ ] #4 Core library dependency boundaries are preserved and no speculative frontend, training, or plugin framework is introduced.
- [ ] #5 README commands reflect operations that actually exist and have been verified on the stated environment.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
