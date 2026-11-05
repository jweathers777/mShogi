---
id: MSH-108
title: Review all-six-game architectural health before M4 closure
status: Ready
assignee: []
created_date: '2026-10-27T20:26:00Z'
updated_date: '2026-11-05T21:51:00Z'
labels:
  - architecture-review
  - planning
milestone: m-4
dependencies:
  - MSH-46
  - MSH-106
  - MSH-107
documentation:
  - docs/architecture-health-roadmap.md
  - docs/design.md
  - docs/review/architecture-health-107.md
  - docs/m4/shared-integration/README.md
type: task
ordinal: 108000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Before M4 closure, use all six completed real game implementations and final shared integration to diagnose accumulated drift. Existing MSH-89 checks shared contract acceptance and MSH-47 checks complete milestone delivery; neither explicitly supplies this whole architecture diagnostic. Coordinate and reuse valid evidence without duplicating those acceptance verdicts or refactoring here.

Future diagnostic work only. Intent, scope, acceptance and verification expectations are recorded here; the reviewer researches the then-current implementation and records a plan when activating this task. See docs/architecture-health-roadmap.md and docs/design.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cover Chess, Standard Shogi, Tori Shogi, Chu Shogi, Tenjiku Shogi and Xiangqi after shared delivery MSH-46. Reconcile the post-Xiangqi and post-Chu review findings against the current source, retain acceptable-area evidence, and deliver the architecture diagnostic before MSH-47 may close M4. Do not require every advisory recommendation to be implemented.
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-107 handoff at accepted source a8581f09107648d03e3016786b41c267ace88bca: current five-game map/report docs/review/architecture-health-107.md confirms R1–R6 ownership with real Tori/Chu consumers; no material redesign or demonstrated architectural Tenjiku blocker. Reconcile advisory F01 repeated replay/unused retained undo, F02 cooperative replay/watchdog granularity, F03 synchronous inspection/choice-cost risks and F04 narrow duplication/comment drift against the eventual six-game source. Preserve native semantics, selected profiles, one controller/envelope/frontend/choice lifetime, actual saveability reserves and conformance policy; no automatic refactor or new dependency. Existing MSH-83/87/79/44 own actual Tenjiku profile/actions/effects/native search/capacity/kanji product evidence. Reuse107 exact focused checks and qualified R6/Tori/Chu evidence; Tenjiku readiness is not delivered proof. MSH-108 remains Ready/unassigned/dependency-blocked by46; no activation or plan added.

MSH-46 final aggregate acceptance handoff2026-11-05 (closure pending): docs/m4/shared-integration/README.md is the current six-criterion ownership/completion record. Reuse unchanged tested runtime/platformbc92bfe5e3b20e9e999df12cbf54b91682c71b8d, immutable MSH88verification.json and MSH89contract-review-89.md/.json; incoming integration91115368c6e99f344dee002d9264f4d0b62cc76b differs only by four neutral rustdoc descriptions in runtime inputs. All six games/seven profiles, native-specific resource/ending/Close policies and localmacOS27 versus hostedmacOS15/actual WindowsLinux49phase qualifications remain explicit. After46Done,106/107 alreadyDone make108 dependency-ready. Keep108 Ready/unassigned/unstarted with original46/106/107 dependencies and eight criteria; no plan/diagnostic/refactor is performed here. Reconcile106/107 advisory findings against then-current source in your own assignment; no advisory change automatically becomes required.112 remains a separately owned unfixed functional diagnostic race with the89disposition retained.47 remains the later independent M4 closure gate.
<!-- SECTION:NOTES:END -->
