---
id: MSH-22
title: Add the first UCI chess engine adapter with a machine output contract
status: In Review
assignee:
  - '@search-builder'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T23:40:00Z'
labels:
  - protocol
  - console
milestone: m-2
dependencies:
  - MSH-21
documentation:
  - docs/design.md
  - README.md
priority: medium
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-03/A-06 and the M2 outcome require UCI as the first engine protocol, chosen by John on 2026-10-24. Keep a distinct machine output contract over the rules-owned game and isolated search. MSH-23 records independent external UCI-engine interoperability.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A distinct console mode implements the chosen protocol subset against documented compatibility evidence, separate from the human shell
- [x] #2 Protocol stdout carries only protocol output, diagnostics go elsewhere, and process tests check this
- [x] #3 The adapter uses the rules library and isolated search, maps actions without making client-provided effects authoritative, and documents unsupported protocol features honestly
- [x] #4 The documented first protocol is UCI; process tests cover handshake/readiness, position history, legal bestmove, stop/quit and supported work/clock limits against the published UCI specification.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Implement a distinct bounded UCI event loop over rules-owned AdjudicatedGame and SearchCoordinator. Preserve full known position history atomically; support handshake/readiness, stop/quit, deterministic depth/nodes and basic clock/movetime limits. Document unsupported controls and claim mapping; verify live process stdout purity, responsiveness, legal moves and stale suppression against April 2006 UCI specification. Obtain independent review and native CI before closure.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner decision 2026-10-24: UCI is the first protocol. External-engine interoperability is required within M2; MSH-23 records a match against an independent UCI engine.

Ownership: search_builder implements isolated console UCI files in codex/m2-remaining worktree; parent owns Backlog/integration. Original protocol downloaded from https://www.shredderchess.com/download/div/uci.zip (April 2006 engine-interface.txt). MSH-21 completed at source e438f7c with three-platform CI40143002342.

Independent review_hardening found P2: a descendant SearchError could be hidden behind a legal fallback and fabricated zero-node info, so an unattended match would appear successful. Exact maximal-fullmove KR-v-K depth2 reproducer established this. Resolution makes genuine search errors fatal with explicit stderr and nonzero exit, distinct from normal stop/zero-work fallback; review also requires immediate propagation during infinite search rather than holding errors until stop. Focused process regressions cover both paths. Final re-review and native CI remain required.

Final UCI subset implements silent distinct mode; April2006 handshake/readiness, atomically replayed start/FEN histories, private exact-context search, depth/node/movetime/basic increment clock limits, stop/quit, infinite holding and stale suppression. Bounded32-command input queue/4096-byte streaming lines and bounded finalinfo; stdout protocol only. Custom ClaimPolicy explicit/auto aligns future search with runner policy; root claims stay controller responsibility. Both P2 error paths resolved: finite/infinite SearchError exits immediately nonzero, no fabricatedinfo/bestmove. Independent re-review_hardening confirms no unresolved material findings and reran11 UCI process tests. macOS27 arm64 Rust1.89 focused console tests2unit+19human+11UCI=32 passed; Clippy alltargets warningsdenied, formatting and diffchecks passed. Cargo.lock adds only existing local search dependency. README/development/coverage/doc contracts updated. Native integration evidence follows.
<!-- SECTION:NOTES:END -->
