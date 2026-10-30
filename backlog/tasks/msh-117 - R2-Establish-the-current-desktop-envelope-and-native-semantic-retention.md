---
id: MSH-117
title: 'R2: Establish the current desktop envelope and native semantic retention'
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T15:26:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-116
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
  - docs/desktop/document-v1.md
  - docs/review/desktop-envelope-117.md
priority: medium
type: task
ordinal: 117000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One explicitly identified supported current desktop envelope carries game/profile/edition/native schema plus authoritative game-native payloads. Remove old desktop v1/v2/v3 acceptance/read/write switches and compatibility readers; old/unknown/conflicting formats reject explicitly and atomically, without converters required solely for development saves.
- [ ] #2 Native replay validates complete Chess MatchRecord commands/policies/timestamps, Shogi actions/declarations and Xiangqi warning/obligation/claimant/offer/conclusion/event histories. Preserve root/provenance, exact same-board/different-history prefixes, settings, controls and retained Take back branch/observation lineage; no FEN/move-list-only reconstruction.
- [ ] #3 Ordinary and exact saved observations preserve exact target history, purpose, original identities, modes/limits, work/proof/witness/Unknown/cancel/failure as historical evidence. Open does not rerun expensive proof work, invent a conclusion or resume pending execution; native unsupported restoration modes still reject.
- [ ] #4 Actual new encoded envelope/lineage sizes and native event/control/combined ordinary-exact observation/replay costs are measured on realistic long records. Admission precedes acceptance with stop/close/identity headroom, exact 256-to-257 and other relevant boundary tests; no truncation or constant-only remedy.
- [ ] #5 Failed/interrupted Save preserves the previous valid file and dirty state; malformed/corrupt/oversized/resource-limited/stale Open preserves live state and unsaved guards. Current-format roundtrips establish fresh stopped runtime with explicit Resume and stale old-result rejection.
- [ ] #6 The only permitted temporary codec bridge maps existing runtime state to the new writer and validated new-format data to existing installation; it reads only the new format, owns no separate live state, and is explicitly deleted by R3. Coordinate observation cutover with R1 as documented; no perpetual old reader.
- [ ] #7 Native current-format/duplicate/replay/capacity/atomic-file tests and actual packaged Save/Open/history/Take back/failure journeys on an identified platform establish this boundary; final integrated three-platform GUI acceptance belongs to R6, not inferred here.
- [ ] #8 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
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
1. Introduce one strict current desktop document envelope with explicit game/profile/edition/native schema identity and raw native payload dispatch; preserve specialized Chess MatchRecord and Shogi/Xiangqi journals. 2. Migrate all writer/parser/admission and real file consumers, delete old format version switches/readers and document the finite runtime-to-codec/install bridge for118. 3. Port semantic regressions/fixtures/verifiers to current format, add strict identity/duplicate/old-format rejection and exact lineage/observation roundtrips; preserve actual byte/replay/count/identity/stop headroom accounting and measure long documents. 4. Verify native/controller/host/frontend and real packaged Save/Open/history/Take back/failure journeys, obtain independent schema/preservation/capacity/deletion review, run applicable CI, integrate/push and finalize only117 when all criteria supported.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup2026-10-30: Codex primary builder/integrator owns117 in primary codex/rewrite checkout atd6250cb71b7c498ddcdf0b82bb1041a4365c2c33. Clean/fetched origin aligned0/0, no In Progress task. Existing unrelated worktrees preserved; no new worktree. InstalledBacklog1.55.0 incompatible; pinned1.55.1 overview/execution guides used. Read request/design/approved migration/target/task/dependency116Done and actual writers/parsers. Current Chess Archive stores schema1/3 format selectors; Shared ArchiveV2 + Xiangqi duplicate envelope writer, version-based compatibility dispatch. Native replay/capacity/file worker retained. Read-only independent archive audit delegated; only root owns production initially. No118activation.

Currentcodecimplementation complete: one document.v1/envelope1 with strictnativeidentity+RawValuepayload; olddesktop1/2/3 readers/writer switches/compatibilitydispatchdeleted. SpecializedChess MatchRecord/nativejournals/exactevidence retained. SelectedChesshistorysaved/restoredstoppedfresh; branchbudgetreservesworstviewprefixindependentselection. Durableobservationsschema1unchanged. Realcurrentnativefixturegenerator replacesGUIoldfileinputwithoutconverter orhistoricalevidencerewrite. Local8document+12takeback+12console+5host+43verifierPASS; persistenceinitial150/4fixturefailuresresolvedinfocusedreruns, finalCIpending. Separatereadonlyreviewer57testsPASS inclFull40001witnessbytebound, no materialfindingafterboundederrors/wording/prefixreservefixes. Measurements/documentation docs/review/desktop-envelope-117.md. NoACchecked; CI+actualpackagejourneysawaitreviewedsourcecommit. No118activation.
<!-- SECTION:NOTES:END -->
