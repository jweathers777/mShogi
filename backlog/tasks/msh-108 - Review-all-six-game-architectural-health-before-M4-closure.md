---
id: MSH-108
title: Review all-six-game architectural health before M4 closure
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-27T20:26:00Z'
updated_date: '2026-11-05T22:30:00Z'
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
  - docs/review/architecture-health-108.md
  - docs/review/evidence/msh108/verification.json
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
- [x] #1 Cover Chess, Standard Shogi, Tori Shogi, Chu Shogi, Tenjiku Shogi and Xiangqi after shared delivery MSH-46. Reconcile the post-Xiangqi and post-Chu review findings against the current source, retain acceptable-area evidence, and deliver the architecture diagnostic before MSH-47 may close M4. Do not require every advisory recommendation to be implemented.
- [x] #2 Publish a current architecture map at an exact source revision identifying native position/state ownership, complete-action representation, candidate generation, attack/movement queries, legality, apply/undo, history and adjudication, canonical records/codecs, perft/diagnostics, evaluator/search boundaries, shared game/profile dispatch, transport/session/archive abstractions, capability discovery, desktop presentation boundaries, frontend/native rules separation and dependency structure. Map responsibilities to actual source symbols and real consumers; explicitly distinguish delivered, unavailable and unchanged coverage.
- [x] #3 Assess duplicated logic, game-specific branching in shared code and verification consistency across games using concrete source/examples. Examine both duplicated implementations whose real consumers demonstrate stable shared semantics and premature abstractions that force different game semantics through misleading common models. Apply: "Generalize when multiple real consumers demonstrate the same semantics, not merely similar-looking code."
- [x] #4 Judge whether responsibilities are explicit, coherent and independently testable, without requiring identical file layouts. No game needs generate.rs, attack.rs or another filename merely because Chess has one. Very large modules, unclear responsibility boundaries, repeated game-identity conditionals, awkward dependency direction, duplicate serialization and tests difficult to isolate are investigation indicators, not automatic defects.
- [x] #5 Produce concrete findings tied to source/examples with risk, severity, evidence, affected consumers and rationale; explicitly record areas reviewed and found acceptable. Distinguish rule correctness, search correctness, execution performance, playing strength and application integration; preserve all six games, accepted profiles/contracts and completed evidence.
- [x] #6 For each recommended change, if any, record expected benefit, affected consumers, regression risks and suggested bounded follow-up tickets with focused differential/regression verification expectations. Record decision/disposition and any genuine blocker separately from advisory recommendations. "No material architectural changes recommended" is a valid evidence-supported outcome.
- [x] #7 Keep this task diagnostic: completing the review neither requires nor authorizes a refactor. Perform no substantial restructuring here; only a very small correction explicitly required by an existing acceptance criterion may be in scope. Any material refactor becomes a separate bounded task preserving behavior with focused differential/regression evidence, and any product/rules/contract change requires the established owner decision.
- [x] #8 Record the report, actual commands/outcomes/environment/source revision and proportionate independent assessment of the findings and coverage. Reuse valid existing evidence with precise references; identify checks not run and unavailable evidence honestly. Complete the full review coverage and record unresolved findings without turning every recommendation into a mandatory refactor.
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
Primary analyst/integrator Codex, primary codex/rewrite checkout clean and origin-aligned at1d3920227bc376b5cc1694928597e7b5a1d5a864; unrelated historical-access worktree preserved. Diagnostic only. (1) Reconcile original108eightcriteria, design/A09–A10,106/115/107 findings and R1–R6 accepted migration against current concrete six-game consumers; map native state/actions/effects/history/replay/diagnostics/search and shared dispatch/runtime/document/capacity/frontend/console/dependencies at this exact source. (2) Read-only isolated native and Tenjiku stress/performance research; root reviews application/frontends, prior dispositions, measurements and MSH112. Separate measured cost, hypothetical scalability, functional correctness, strength and maintainability; no substantial refactor or contract changes. (3) Publish docs/review/architecture-health-108.md with current symbols, all earlier finding dispositions, acceptable areas and bounded optional recommendations including risks/consumers/focused verification; no manufactured task tree. (4) Fresh independent read-only assessment of complete map, six-game/Tenjiku coverage, risk classifications/M5 readiness and blockers versus advisory. Focused source/dependency/metadata/hash/link/Backlog checks only; reuse unchanged88/89/46platform/conformance evidence, no broad CI/native/frontend/GUI campaign. (5) Reconcile all8AC/4DoD, update47handoff without activating it, normally integrate/push if supported; leave47/M4/M5/refactoring unstarted and stop after108.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-107 handoff at accepted source a8581f09107648d03e3016786b41c267ace88bca: current five-game map/report docs/review/architecture-health-107.md confirms R1–R6 ownership with real Tori/Chu consumers; no material redesign or demonstrated architectural Tenjiku blocker. Reconcile advisory F01 repeated replay/unused retained undo, F02 cooperative replay/watchdog granularity, F03 synchronous inspection/choice-cost risks and F04 narrow duplication/comment drift against the eventual six-game source. Preserve native semantics, selected profiles, one controller/envelope/frontend/choice lifetime, actual saveability reserves and conformance policy; no automatic refactor or new dependency. Existing MSH-83/87/79/44 own actual Tenjiku profile/actions/effects/native search/capacity/kanji product evidence. Reuse107 exact focused checks and qualified R6/Tori/Chu evidence; Tenjiku readiness is not delivered proof. MSH-108 remains Ready/unassigned/dependency-blocked by46; no activation or plan added.

MSH-46 final aggregate acceptance handoff2026-11-05 (closure pending): docs/m4/shared-integration/README.md is the current six-criterion ownership/completion record. Reuse unchanged tested runtime/platformbc92bfe5e3b20e9e999df12cbf54b91682c71b8d, immutable MSH88verification.json and MSH89contract-review-89.md/.json; incoming integration91115368c6e99f344dee002d9264f4d0b62cc76b differs only by four neutral rustdoc descriptions in runtime inputs. All six games/seven profiles, native-specific resource/ending/Close policies and localmacOS27 versus hostedmacOS15/actual WindowsLinux49phase qualifications remain explicit. After46Done,106/107 alreadyDone make108 dependency-ready. Keep108 Ready/unassigned/unstarted with original46/106/107 dependencies and eight criteria; no plan/diagnostic/refactor is performed here. Reconcile106/107 advisory findings against then-current source in your own assignment; no advisory change automatically becomes required.112 remains a separately owned unfixed functional diagnostic race with the89disposition retained.47 remains the later independent M4 closure gate.

MSH-46 is now Done, all original6AC/4DoD supported after fresh independent completion review and accepted evidence normal push90e1222d8db27ede3212b794e877a975a068c35d. Final current delivery record docs/m4/shared-integration/README.md; runtime/platform source remainsbc92bfe5e3b20e9e999df12cbf54b91682c71b8d. With106/107Done all three existing108prerequisites46/106/107 are satisfied.108 is the next assignment, remains Ready/unassigned/unstarted with no implementation plan added; original8AC/4DoD/dependencies intact.47/M4 remain open;112 remains unfixed as qualified in89. Do not treat advisory architecture recommendations as mandatory refactors.

Current-source six-game diagnostic report and single new diagnostic verification index published in working tree; read-only native and Tenjiku researchers report no material blocker. Original8criteria/4DoD unchanged and unchecked pending fresh final assessment. Offline metadata bothworkspaces, production sourceequivalence (4rustdoccomments only since acceptedbc92), unchanged priorfivegame native/searchsource since107,202immutableevidencehashes, fourcurrentnativeTenjikuhashes vs79measuredsource, originaltaskdependency/criteria and ownership, localfilelinks, Backlog1.55.1doctor anddiff checks pass. New broad product/native/frontend/GUI/CI/performance/strength work not run.47handoff appended without changing status/owner/dependencies/criteria or starting it.

Fresh independent read-only msh108_final_assessment approved the complete current-source report/index at1d3920227bc376b5cc1694928597e7b5a1d5a864, reviewed reportSHA2560cd2ebb3c850c4e82042ac51bbb7fe0d1f54450bbc9b2f5604890918486397f1. All8originalAC and DoD1–3 supported, no material factual/coverage gap or demonstrated architecture blocker. Reviewer independently inspected native/shared/frontend/source/capacity/measurements/priorfindings; checked sourceequivalence, previousfivegameunchangedsource,7acceptedrecordhashes/4nativeTenjikuhashes,original108/47criteria/DoD,22reportfilelinks,diff; no product tests/builds/GUI/CI/benchmarks. Root offline metadata bothworkspaces plus11focusedstaticchecks pass, including202immutableevidencehashes,30exactsourceanchors/29report+roadmaplinks,retireddeclarationnegativeinventory and Backlog1.55.1doctor. Diagnostic verdict: coherent accepted6gameowners; no material architecture change recommended beforeM4/pilot. Four advisory P1–P4 scopes are report recommendations only, no newtasks/dependencies. Existing M5pilot48/diagnostic109 gates unchanged, no strengthclaim.112 separatelyunfixedfunctionaldiagnostics disposition retained. All8AC/DoD1–3 nowchecked; DoD4 pending actual ordinary evidencecommit/push.47stillunstarted; no source/rules/schema/limit/reserve/minwindowbehavior change.
<!-- SECTION:NOTES:END -->
