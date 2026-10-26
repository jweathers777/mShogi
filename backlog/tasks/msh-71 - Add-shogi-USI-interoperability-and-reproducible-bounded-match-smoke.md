---
id: MSH-71
title: Add shogi USI interoperability and reproducible bounded match smoke
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-70
  - MSH-68
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/shogi-verification.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 71000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Actual USI processes, declared-rule option admission, bounded opponent/self-play transcripts and offline event replay; opponent/source lineage and limitations recorded.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A separate USI adapter exposes real shogi native search with handshake/readiness,SFEN+complete history,move/drop/promotion,resign/win,stop/gameover/quit and a documented supported subset while UCI chess stays available.
- [ ] #2 Host shogi authority validates returned actions and declarations under the recorded profile; protocol draw notification is distinguished from game outcome,score and replay policy. Process/transport failures are distinct from named competition penalties.
- [ ] #3 Bounded self-play and a recorded match against pinned independent opponent candidate YaneuraOu0f421391ae311df9df57dcfcb2b4a78a1972bdf8 (or independently reviewed justified replacement) preserve binary/build/options/profile,commands,limits,observations and native conclusions.
- [ ] #4 Invalid move,false declaration,missing/late/malformed reply,timeout,cancellation and process cleanup have explicit reports and actual process fixtures. Work-limited rerun claims are conditional; recorded event replay is deterministic.
- [ ] #5 Untimed experiments are supported first. Unsupported clock/event profiles fail explicitly; any timed support must define and verify its game-specific timing policy before claiming it. MSH50 retains strength measurement.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
