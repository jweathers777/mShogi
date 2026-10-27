---
id: MSH-106
title: Review architectural health after complete Xiangqi delivery
status: Ready
assignee: []
created_date: '2026-10-27T20:26:00Z'
labels:
  - architecture-review
  - planning
milestone: m-4
dependencies:
  - MSH-45
documentation:
  - docs/architecture-health-roadmap.md
  - docs/design.md
type: task
ordinal: 106000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete Xiangqi delivery adds a third real game across rules, records, engine, console and desktop. Diagnose accumulated architectural drift against actual Chess, Standard Shogi and Xiangqi consumers after MSH-45 is complete; board rules, routing95 or review103 alone do not establish this trigger. Do not modify the active Xiangqi implementation queue. This review does not gate unrelated earlier Tori/Chu/Tenjiku work.

Future diagnostic work only. Intent, scope, acceptance and verification expectations are recorded here; the reviewer researches the then-current implementation and records a plan when activating this task. See docs/architecture-health-roadmap.md and docs/design.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Compare all delivered Chess, Standard Shogi and Xiangqi paths, including square/intersection presentation and differing history/adjudication semantics; preserve the completed MSH-45 evidence. Feed the later all-six M4 architectural checkpoint without reopening complete game delivery.
- [ ] #2 Publish a current architecture map at an exact source revision identifying native position/state ownership, complete-action representation, candidate generation, attack/movement queries, legality, apply/undo, history and adjudication, canonical records/codecs, perft/diagnostics, evaluator/search boundaries, shared game/profile dispatch, transport/session/archive abstractions, capability discovery, desktop presentation boundaries, frontend/native rules separation and dependency structure. Map responsibilities to actual source symbols and real consumers; explicitly distinguish delivered, unavailable and unchanged coverage.
- [ ] #3 Assess duplicated logic, game-specific branching in shared code and verification consistency across games using concrete source/examples. Examine both duplicated implementations whose real consumers demonstrate stable shared semantics and premature abstractions that force different game semantics through misleading common models. Apply: "Generalize when multiple real consumers demonstrate the same semantics, not merely similar-looking code."
- [ ] #4 Judge whether responsibilities are explicit, coherent and independently testable, without requiring identical file layouts. No game needs generate.rs, attack.rs or another filename merely because Chess has one. Very large modules, unclear responsibility boundaries, repeated game-identity conditionals, awkward dependency direction, duplicate serialization and tests difficult to isolate are investigation indicators, not automatic defects.
- [ ] #5 Produce concrete findings tied to source/examples with risk, severity, evidence, affected consumers and rationale; explicitly record areas reviewed and found acceptable. Distinguish rule correctness, search correctness, execution performance, playing strength and application integration; preserve all six games, accepted profiles/contracts and completed evidence.
- [ ] #6 For each recommended change, if any, record expected benefit, affected consumers, regression risks and suggested bounded follow-up tickets with focused differential/regression verification expectations. Record decision/disposition and any genuine blocker separately from advisory recommendations. "No material architectural changes recommended" is a valid evidence-supported outcome.
- [ ] #7 Keep this task diagnostic: completing the review neither requires nor authorizes a refactor. Perform no substantial restructuring here; only a very small correction explicitly required by an existing acceptance criterion may be in scope. Any material refactor becomes a separate bounded task preserving behavior with focused differential/regression evidence, and any product/rules/contract change requires the established owner decision.
- [ ] #8 Record the report, actual commands/outcomes/environment/source revision and proportionate independent assessment of the findings and coverage. Reuse valid existing evidence with precise references; identify checks not run and unavailable evidence honestly. Complete the full review coverage and record unresolved findings without turning every recommendation into a mandatory refactor.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
