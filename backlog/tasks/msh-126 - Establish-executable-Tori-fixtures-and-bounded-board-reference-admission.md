---
id: MSH-126
title: Establish executable Tori fixtures and bounded board-reference admission
status: Ready
assignee: []
created_date: '2026-11-01T12:21:00Z'
labels:
  - m4
  - owner-msh-85
milestone: m-4
dependencies:
  - MSH-81
references:
  - >-
    backlog/tasks/msh-85 -
    Deliver-the-authoritative-tori-shogi-native-handoff-for-shared-consumers.md
documentation:
  - docs/m4/tori/profile.md
  - docs/m4/tori/fixtures.json
  - docs/m4/tori/fixtures.md
  - docs/m4/tori/native-plan.md
  - docs/m4/tori/native-fixture-ownership.json
  - docs/m4/tori/comparator-audit.md
type: task
ordinal: 126000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-81 is Done at6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc. Selected profile mshogi.tori.modern.v1 and all103 independently expected fixture specifications are fixed inputs; do not reopen historical acquisition or import Standard Shogi defaults. This is a component owned by coordinating MSH-85 under complete game delivery MSH-42, following the repository flat component-ID precedent used for MSH-84/91-94. The reviewed corpus is currently declarative: implementation needs a reproducible loader, assertion/adapter boundaries and useful failure reporting before kernel output can be compared. Own test infrastructure and independently expected inputs, not product legality or native game code. All103 rows must remain accounted for even when later owners supply their native executor. Pinned Fairy-Stockfish is only a scoped candidate; its fourth-repetition and raw Legal drop-mate behavior must never become the profile.

Verification: focused reproducible commands at exact tested source, independent bounded review, normal codex/rewrite integration/push and relevant native/shared regression/CI for actually changed code. Distinguish passed/failed/not-run and reference observations from independently expected facts. No unrelated GUI campaign, playing-strength claim, new exact-proof requirement or future-variant implementation. Future worker records an implementation plan only on activation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A runnable bounded fixture harness strictly loads all103 existing IDs and the reviewed corpus hash, preserves sources/independent expectations, exposes each assertion at its declared geometry/transition/local/history/record/consumer boundary and reports executed versus pending downstream adapters honestly. No unimplemented native case silently passes or is discarded; every ID maps to a named owner in native-fixture-ownership.json.
- [ ] #2 Build and actually run Fairy-Stockfish53c1f2bd14a2d3b685904ccf8ec9dec8401800ff in an isolated environment, recording fetched commit/file hashes, license, toolchain/compiler, build options, binary hash, platform, invocations and outputs. Admit only demonstrated compatible board operations after independent sentinels for both quails/owners, jump versus blocked Eagle, compulsory zone entry/within/exit and zone-drop departure, capture/demotion and two/three Swallow drops.
- [ ] #3 Reference mapping names all9 native faces, both owners,7x7 coordinates, hands, complete moves/drops and forced promotion without losing alternatives. Audit actual exposed operation: raw Legal Swallow-drop-mate and all history/terminal/perpetual-check scoring remain excluded unless a separately independently verified narrow filter is demonstrated; excluded reference scope never removes independent native tests.
- [ ] #4 The harness executes known-good independent sentinel assertions and deliberate wrong-action/missing-action/extra-action/effect-mismatch probes with nonzero failure, fixture ID, input/profile/history, expected/observed distinction and bounded diagnostics. Do not adopt upstream perft103857 or regenerate expected data from the reference or future mShogi kernel.
- [ ] #5 Source-derived expectations remain independent from reference observations; adapters for later B/C/D slices can consume the corpus without redefining it. Geometry units are not full legal positions, history-choice units are not full games, and consumer process/purpose facets are explicitly reserved to MSH-77/later real engine work. Any actual contradiction is isolated and reviewed against the selected profile rather than changing accepted rules.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
