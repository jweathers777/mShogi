---
id: MSH-23
title: Build a reproducible chess self-play and match runner
status: In Review
assignee:
  - '@runner-builder'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-25T00:08:00Z'
labels:
  - match
  - reproducibility
milestone: m-2
dependencies:
  - MSH-17
  - MSH-18
  - MSH-22
documentation:
  - docs/design.md
priority: medium
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Design section 6 requires match experiments to preserve executable and model identity, parameters, resource limits, opening positions, colors, results and failures, and to pair openings with reversed colors where possible. M2 needs reproducible self-play before any strength work in M5.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The runner plays complete self-play and external UCI games through the rules-owned game, adjudication and clocks, recording executable/model identity, parameters, work/time limits, openings, colors, policy, commands, clock/match events, results, failures and canonical replay records.
- [x] #2 Recorded games and match events replay deterministically. Rerun determinism is claimed only under documented deterministic engine settings and work limits. Timed runs preserve configuration and observations without promising identical reruns.
- [x] #3 Results are reported per game and opening set without a strength claim; at least one recorded match against an independent UCI engine documents executable identity, commands, compatibility and outcome.
- [x] #4 Process failures, invalid moves, missing responses and protocol/watchdog timeouts have explicit recorded failure categories, diagnostic evidence and unscored status distinct from chess outcomes and chess-clock expiry; regression process tests exercise each.
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
Add a bounded native UCI match runner over ChessMatch with strict configuration/reports, fresh per-game engines, recorded identities/options/commands/diagnostics and paired openings. Apply explicit auto-claim policy and core clocks, preserving unscored process/watchdog/experiment failures separately. Verify complete scripted games, clock/claim handling, process failure categories, bounds and semantic replay; record actual self-play and independent Stockfish matches. Obtain independent rules/replay and process review, integrate and collect native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Dependency-ready after MSH-22 source e35ce58 passed CI40143724587 on Linux/macOS/Windows. Ownership runner-builder (review_history_boundary) owns new runner crate/library/binary/tests and workspace membership/lock; parent owns Backlog, documentation/evidence artifacts and integration. No core/search/console behavior edits without coordination. Actual official Stockfish19 arm64 universal binary staged outside repo for interoperability; parent records identities and commands. Existing M1 oracle evidence retained.

Initial real release experiments completed and replayed on macOS: paired Stockfish19 forced/standard games, self-play forced/standard games, timed pair, known-history claim and genuine descendant search failure (unexpected exit1, unscored). These are alpha evidence pending final review/fix rerun. Independent process/rules reviewers identified configuration-to-identity/work binding, receipt timestamps before diagnostic status waits, earliest watchdog/clock ordering after delayed wake, projected clock balance at go dispatch, regular-file hashing and visible metadata truncation. Builder is resolving these within runner scope; no chess rule or owner-policy changes.

Final runner implements strict effective config/report checksums and semantic identity/command bindings, fresh paired UCI processes, SHA256 executable/model identities, bounded writes/output/metadata/transcripts/cleanup, core AutoClaim and increment clocks. ProcessStartFailure, UnexpectedExit, InvalidMove, MissingResponse and ResponseTimeout remain unscored and distinct from clock/unresolved chess results and ply caps. Deadline precedence uses earliest fixed boundary (clock ties); response/EOF/clock detection timestamps survive bookkeeping/status waits. Actual macOS release evidence: 13 first-run games plus8 deterministic-setting reruns=21games in8 reports, all replayed. Standard SF19 games ended in mate after48/87plies; self-play standard games ended in intended threefold claims after13plies; forced/timed pairs completed; known-history claim precededgo; real search failure exited1/unscored; outstanding ownUCI depth128/movetime10s search under50ms clock ended with explicitly unscored uncertainty and successfulquit. Eight work-limited reruns had identical commands/conclusions/identities; no identical timed promise. Exactcommands/configs/hashes/provenance in tests/fixtures/matches/README.md and docs/match-runner.md. Final18runner checks passed (4deterministicunits,8portableprocess+2Unixboundary,4recorded-evidence), Clippy/fmt/diff passed; broader workspace tests/build/Clippy also passed before last clock-only fix, unchanged evidence retained. Independent review_hardening cleared process/timing fixes and review search_builder cleared rules/replay plus regenerated21game evidence. No unresolved material findings. SHA2 exactpin/license/provenance audit recorded in development docs; no copied crypto implementation. Native CI remains beforeclosure.
<!-- SECTION:NOTES:END -->
