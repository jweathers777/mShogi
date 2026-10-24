---
id: MSH-24
title: 'Harden M1 boundaries: structured rejection reasons and console input bounds'
status: Done
assignee:
  - '@codex'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T20:57:00Z'
labels:
  - rules
  - console
  - hardening
milestone: m-2
dependencies:
  - MSH-14
documentation:
  - docs/review/m1-closure.md
priority: low
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M1 reviews left recorded, non-blocking limits. FenError exposes only a category and message, so a test matches rejection reasons by text and ActionError::IllegalPosition assumes king safety. The console echoes raw input in errors, accepts Unicode whitespace and a leading plus sign, has no line-length or perft-depth bound, and gives a parse error inside a move sequence without its index. These should be fixed before more adapters build on the same boundaries.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Structural FEN and position rejection reasons are available as a structured value, and tests no longer match message text
- [x] #2 Console input handling documents and enforces its grammar (ASCII separators, numeric syntax) and has explicit bounds on line length and diagnostic depth, with clear errors
- [x] #3 A parse failure inside a move sequence reports its index, consistent with other sequence errors, and existing process tests are updated accordingly
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
1. Replace FEN rejection message identity with public structured reasons and preserve category/Display diagnostics; make ActionError diagnostics reflect the actual reason. 2. Bound streaming console lines before allocation, enforce ASCII grammar and unsigned decimal counts/depth with a documented diagnostic cap, and escape echoed input. 3. Add focused boundary/process regressions for recovery, atomicity and sequence indices; update documentation, obtain independent review, integrate and record native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex, branch codex/m2-baseline, existing clean next-task checkout. MSH-14 prerequisite remains c4ed581 with existing three-platform evidence. Work starts while the independent MSH-15 documentation review runs.

Focused verification: cargo test -p mshogi-core --test fen_corpus --locked (9 passed), cargo test -p mshogi-console --test cli --locked (15 passed), cargo test -p mshogi-core --test sequences --locked (3 passed, 1 existing deep opt-in ignored). Structured FenErrorReason covers all reasons; sequence properties now match enums. CLI regressions cover ASCII/numeric syntax, escaped controls, parse indices/atomicity, depth cap and exact/oversized/EOF streaming recovery. Independent review_hardening found the remaining old sequences.rs message coupling; replaced and independently confirmed resolved. No unresolved material finding.

Local verification 2026-10-24: macOS 27.0.1 (26A434), arm64; rustc 1.89.0 (29483883e), cargo 1.89.0. cargo fmt --all -- --check, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked, cargo build --workspace --locked passed on the working batch based on c4ed581. Existing opt-in deep perft/sequence tests remain ignored; no rerun of the M1 deep oracle suite is claimed. Native three-platform CI will be recorded against the integrated source revision before completion.

Integrated normally and pushed to origin/codex/rewrite at 1264e4bf90f2277a11840fefdbafafe8e0d30e64. Native Rust workspace CI https://github.com/jweathers777/mShogi/actions/runs/40131950077 completed successfully on ubuntu-24.04 (job 120985329413), windows-2022 (120985329549) and macos-15 (120985329551): toolchain identity, formatting, Clippy, workspace tests, build and console smoke all passed. This final source revision covers the three batch tasks; unchanged M1 deep/oracle evidence is reused. Closure bookkeeping changes only Backlog records and reuses this exact source evidence. The previously stale rewrite-bootstrap checkout was fast-forwarded without altering master or legacy history.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed structured FenErrorReason identities (including sequence-property enum matching), truthful action rejection diagnostics, bounded streaming input, documented ASCII/decimal syntax and depth cap, escaped tokens and indexed atomic parse failures. Focused FEN, sequence and real-process tests passed; independent review found and verified the resolved legacy Display coupling. Native three-platform integrated CI passed. Adapter limits do not cap library diagnostics. Source 16530f8; integrated evidence 1264e4b.
<!-- SECTION:FINAL_SUMMARY:END -->
