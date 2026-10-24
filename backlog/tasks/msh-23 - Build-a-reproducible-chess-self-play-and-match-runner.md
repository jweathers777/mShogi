---
id: MSH-23
title: Build a reproducible chess self-play and match runner
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T20:45:00Z'
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
- [ ] #1 The runner plays complete self-play and external UCI games through the rules-owned game, adjudication and clocks, recording executable/model identity, parameters, work/time limits, openings, colors, policy, commands, clock/match events, results, failures and canonical replay records.
- [ ] #2 Recorded games and match events replay deterministically. Rerun determinism is claimed only under documented deterministic engine settings and work limits. Timed runs preserve configuration and observations without promising identical reruns.
- [ ] #3 Results are reported per game and opening set without a strength claim; at least one recorded match against an independent UCI engine documents executable identity, commands, compatibility and outcome.
- [ ] #4 Process failures, invalid moves, missing responses and protocol/watchdog timeouts have explicit recorded failure categories, diagnostic evidence and unscored status distinct from chess outcomes and chess-clock expiry; regression process tests exercise each.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
