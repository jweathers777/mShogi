---
id: MSH-42
title: Deliver complete tori shogi rules and functional product integration
status: In Progress
assignee:
  - '@Codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-02T14:50:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-72
  - MSH-73
  - MSH-77
  - MSH-81
  - MSH-85
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/m4/tori/handoff.md
  - docs/m4/tori/profile.md
  - docs/m4/tori/native-plan.md
  - docs/m4/tori/native-fixture-ownership.json
  - docs/m4/tori/shared-acceptance.md
  - docs/m4/tori/playable-delivery.md
ordinal: 42000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete tori shogi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-51 in M5.

Open design questions (D-07/D-08): Resolve the authoritative historical rule sources and conventions for movement, promotions, drops, repetition and game endings; record disputed interpretations before implementing affected behavior. Select the necessary state/action, notation and adapter contracts against the actual consumers, preserving complete actions and history; no speculative universal representation or protocol is prescribed. This To Do scope is not authorization to begin implementation or a change to the current M3 batch.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A sourced, identified and versioned tori shogi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [ ] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [ ] #3 Independent sourced tori shogi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [ ] #4 The independently usable console can select and inspect tori shogi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [ ] #5 The native mShogi engine can search and play tori shogi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [ ] #6 Canonical saved tori shogi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [ ] #7 The desktop supports actual tori shogi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [ ] #8 The game coverage ledger and independent review show all required tori shogi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Sole primary integrator root in clean aligned codex/rewrite2f87096; pinnedCLI1.55.1. Activate42only, preserveall8AC and keepInProgress after this native checkpoint. Disjoint helpers own search/engine and native notation/console respectively; independent reviewer read-only, no competing edits/commits or worktree/identity proliferation. Root owns desktop admission/codec, integration and evidence.
2. Native tori::Game/record reconstruction is sole search authority (no publicClone); finite full-width iterative baseline, deterministic node/depth/stop limits, complete moves/drops/compulsory results, T06conclusions/history, semanticundo. Typed native Tori result/telemetry extends existing execution4/schema1 dispatch with explicit compatibility, history/target/identity validation and actual bridge cancellation; no new protocol or UI engine architecture.
3. Root extends existing Tori journal/observation validator and native session scheduling/acceptance together: separate engine-turn versus readonlyanalysis, precommit lossless capacity, native re-admission, full stale identity, retained observations/branches, stoppedfreshreopen. Preserve historicalFailureobservations and measure actual new result bytes/replay/256-to257/reservedendings; no cap/deadline increase. ProductNewGame/Load remain gated, no finalartwork/normalToriplayer exposure.
4. Native Rust readable complete-action notation/parse and console selector/session commands use coreGame; sharedMoves formatter consumes same native display. Canonical import/save, standard/customroots, hands/legal/move/drop/undo/history/status/resign/Stop/perft/divide plus bounded actual subprocess selfplay. C-NOTATION independentfield/requirement becomes actualconsumer evidence without changing immutablecorpus/selectedprofile.
5. Early exhaustive-result dispatch preflight across workspace and standaloneTauri, Node24strictfrontendchecks, update obsoleteunsupported expectations onlywhere superseded by actualengine. Focusedsearch/engine/codec/controller/console/process/notation tests during edits, then coherent broad regressions. Mandatoryrepresentative success separate from near-limit precise typedrefusal with stateintegrity. Independent review early/final substantiveboundary, resolvefindings.
6. RequiredexactsourceCI: Rustworkspace all3platformjobs; Desktopworkflow lockedfrontend/nativehost checks and normalpackage/bundledengine consumers all3 (actualGUIcampaigns automatic supportingexisting-gameevidence, not nativeTori visual/productacceptance gate). One completion-aware monitor of requiredgates; cancel supersededruns ifneeded, no redundantsleeps. One structured evidence set pertestedsource references unchanged77/85/130-132 evidence. Normal commit/push, leave42InProgress and8criteria incompletewherefinalproductmissing. Report actual limits/selfplay/reviews/CI and direct finaldesktopartwork/playhandoff; stopthere.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only.

MSH81 currentresearch2026-10-31 supplies docs/m4/tori source/profile-gate ledger and currentR1-R6consumer handoff. Tohoku15-imageprimarywitness nowacquired; textdate1828 is notprovenphysicalcopydate. Candidateprofile remainsunratified; P02promotion/drop timing,P03formalrepetition/check andP04exceptionalendings blockconformance. Preserve nativequailidentity/7x7geometry/history,currentexecution/document/runtime/sharedinteractionowners andsaveability. Source-groundedscopeconstraints only: no detailedfutureplan,dependency/ACchange,Readyactivation or deliveryclosure; complete81 before affected85/77implementation.

Superseding owner-approved modern direction2026-11-01: MSH81 now supplies complete mshogi.tori.modern.v1 in docs/m4/tori/profile.md, concrete independent fixture specs and current R1-R6 handoff. Earlier historical acquisition/P01-P06 blocking notes remain dated evidence but do not gate this selected modern identity. T06 explicitly formalizes third-occurrence departure responsibility and forced cycles; T05/T07 specify rare endings/root/admission. No native conformance already claimed. MSH85 remains To Do and must perform original AC4 bounded native-slice refinement before Ready; MSH77 waits actual85handoff;42 retains complete console/functional-engine/archive/desktop/review obligations. All original dependencies and AC/DoD preserved; no activation or implementation by this note.

Bounded native refinement2026-11-01 (planning only): coordinating85 now owns flat component126fixture/reference admission→127structuralstate/transitions→128locallegality/inspection→129history/records/nativehandoff;126dependsDone81 and no componentdepends85/77/42. All103immutablefixtures explicitly mapped in native-fixture-ownership.json (126dataownerall;127/128/129primary14/69/20), with lateractualprocess/document/notation facets preserved under77/42. Original85AC4 is satisfied only by reviewedplanning, not kernelimplementation;85cannotclose before nativecomponents and coreconsumerprobe. Native GameKind addition may require minimal explicitunsupported exhaustivearms; real engine/session/archive/frontend remainlaterownership. Preserveprofile/oldresearch/alloriginalAC/DoD; nohistoricalacquisition or optionalexactproof gate. See native-plan.md for actual inspectedboundaries, verification and currentdependencysets.

MSH77 final integrated acceptance at tested production52b0390 is PASS all4originalcriteria;130/131/132Done,713nativeassertions/103fixedexpectations and actual13runtimefields/82requirements preserved. Current direct implementation handoff docs/m4/tori/playable-delivery.md: first native history-aware evaluator/search + actual supervised Tori result/real observation codec/controller admission + bounded console/notation/selfplay, then actual product board/artwork/hands/shared UI play/analysis/SaveLoad/recovery and independent eight-criterion acceptance. No extra planning-only assignment or new implementation tickets created. NormalproductTori activation remains gated until actualplayableconsumer; finishedvisuals requireownerapproval. C-NOTATION1field/1requirement remains42. All8AC/DoD/dependencies preserved,42ToDo/unassigned/unstarted; no implementation plan or activation applied.15sreplay operational qualification and46/47/88/89shared-platform ownership retained; MSH51ownsstrength.

Owner authorizes coherent native playable-consumer checkpoint at2f87096; all6directdependenciesDone, noactivecompetingtask, cleanprimaryandoriginverified. Preserveoriginal8AC/DoD;42willremainInProgress after nativecheckpoint. RequiredCI and soleintegrator/disjointownership recorded inplan. No newresearch/profile/limit/design change or normalproductToriactivation.

Native checkpoint implementation is coherent: native Game-owned deterministic Tori material/minimax search, protocol4/schema1 additive typed results, real bounded native desktop scheduling/observation/action admission, complete precommit archive capacity, readable Rust notation/console and actual subprocess smoke. Normal product chooser/New Game/Open remain gated and final artwork/GUI acceptance unstarted. Focused search8, engine8, notation3, console/process/selfplay13, desktop55 plus separately two real-context controller tests and frontend final344 passed; full workspace/feature/standalone host campaign and exact-source CI are still pending, not claimed. Independent read-only reviewer resolved host-poll replay, conservative PV charges, early-terminal visit bound, cancellation/refusal pause and positive-node UI agreement; final admission-ID consumed-response edge fixed with focused test pending. Single evidence index docs/m4/tori/evidence/native-playable-checkpoint.json and actual selfplay transcript; all original8AC/DoD preserved,42 stays In Progress.

Coherent local verification completed: cargo fmt/workspace Clippy PASS; cargo test --locked --workspace PASS; desktop-session all-features259PASS including final consumed-response headroom regression; standalone Tauri all-target/all-feature check+Clippy PASS and final14host tests PASS; strictNode24TypeScript,344frontend tests and normal frontend build/exclusion checks PASS. First host run13/14 exposed obsolete unsupported-engine assertion; corrected without changing normal product gate, raw failed log retained. Complete immutable profile/corpus/ownership hashes unchanged. Independent separate-agent source/test/evidence review PASS, no unresolved material finding; reviewer did not execute tests. Actual release9/62ms and real256obs/64move capacities plus subprocess traces indexed in native-playable-checkpoint.json. Ready for reviewed source commit and exact-source requiredCI; do not interpret these local results as platform GUI evidence.42 remains In Progress.

Final stopped-reopen audit corrected implicit engine Resume after paused human Play: manual native actions preserve paused intent; only explicit Resume starts engine work after Open/Cancel/Return.25focused Tori controller tests PASS including actual Save/Open human-turn move staying stopped across polls and later explicit Resume; both assignments and running-play admission remain correct. Corrected actual subprocess/controller/selfplay7tests PASS, final344frontend tests and workspace Clippy PASS; source fixtures regenerated. Separate reviewer re-reviewed exact correction PASS; earlier search/protocol/codec/notation review/evidence remain valid.035c35c campaigns41463115080/41463115085 intentionally cancelled as superseded (partial state retained in evidence/native-playable-ci-superseded-035c35c.json), not claimed completed. Corrected source will receive required exact-source CI; no scope/criteria/profile/limits/product-activation changes.
<!-- SECTION:NOTES:END -->
