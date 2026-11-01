---
id: MSH-130
title: Deliver current Tori desktop document codec and lossless admission
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-01T21:28:00Z'
updated_date: '2026-11-01T22:09:00Z'
labels:
  - m4
  - owner-msh-77
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-85
references:
  - >-
    backlog/tasks/msh-77 -
    Extend-shared-contracts-for-real-tori-shogi-consumers.md
documentation:
  - docs/m4/tori/shared-plan.md
  - docs/m4/tori/shared-consumer-ownership.json
  - docs/m4/tori/native-acceptance.md
  - docs/testing/conformance-policy.md
  - docs/m4/tori/desktop-document.md
  - docs/m4/tori/desktop-document-evidence.md
type: task
ordinal: 130000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Native Tori records are accepted, but the surviving current desktop envelope cannot yet encode or admit them. Establish the immutable document payload and candidate validation before a live session can accept work: this avoids an intermediate unsaveable document. Actual consumers are crates/mshogi-desktop-session/src/document.rs, shared_archive.rs, persistence.rs bounded worker/atomic writer, capacity.rs, resource.rs and desktop/src/resources.ts. The later runtime slice owns ActiveDocument and sole install_document wiring; this task must not introduce another runtime or normal Tori Open/New Game route. Current envelope mshogi.desktop.document.v1/envelope_version1 remains the one format; native schema1/profile edition and shared transport4 stay explicit.

Component of MSH-77 shared integration under MSH-46, not complete Tori delivery MSH-42. Accepted source994521bec0637d6f8f17103f1106c0025d950753/profile mshogi.tori.modern.v1, native acceptance a3677e1e862fdf3877c784f12f56a99c4bfbf167. Original103 independently expected fixtures and713 native assertions remain fixed; external engines are optional nonauthoritative investigations, never gates. See shared-plan.md and shared-consumer-ownership.json for exact consumer obligations. No rules, search/evaluation, console/notation, product Tori artwork/page, selector entry, other variant or M5 implementation. Future worker records its current implementation plan only after activation.

Verification: focused real native/candidate/controller tests appropriate to this boundary, exact-source commands/outcomes, independent completed-diff review, relevant existing-game regressions and applicable native/desktop CI for actual changed production. Distinguish source/controller/frontend/package/GUI evidence; no fake Tori engine or product-capability claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An immutable game-qualified Tori payload for the current envelope preserves native root/provenance/declared counter, initial/current canonical records, full ordered native commands/events/effects/obligations/conclusions, native side, relevant settings, exact historical observations/targets and retained Undo/Take back lineage. Document controls remain distinct from board moves; pending execution is never serialized. No second document owner or development-era codec compatibility layer.
- [ ] #2 Strict RawValue dispatch reaches the authoritative tori::GameRecord parser/replay with duplicate fields intact. Outer/nested duplicate, unknown, missing, conflicting game/profile/edition/schema, forged intermediate/final facts, unsupported mode, corrupt and oversized payloads reject. Codec tests use all seven TORI-REJECT cases; actual live-document nonmutation is proved by the dependent runtime slice, not claimed by a pure codec.
- [ ] #3 Measure complete actual encodings and repeated native replay for long capture/drop/promotion and forced8/unequal16 repetition histories, resignation, controls, retained branches and real supported observations. Document a bounded Tori whole-document replay budget and actual charge model from measurements; native512KiB/512events/65536units are per native record and not an outer replay budget. Preserve outer16MiB/4096retained-events/8192controls/256observations unless a separately justified measured policy is required; no constant-only fix.
- [ ] #4 Every supported immutable candidate is admitted before commit against actual pretty encoded envelope bytes, aggregate native events including retired lineage, controls, observations, total replay work, runtime IDs and reserved close/stop/concluding headroom. Exact boundary and plus-one tests include256-to257 observations, native caps and controls/bytes/replay; refusal preserves a complete Save path without truncation. Map authoritative Resource facts including Tori scope, never diagnostic-number inference, illegal action, outcome or engine-failure conflation.
- [ ] #5 Actual pure codec/worker and atomic-file tests roundtrip real Tori records, full historical prefixes and unscored UnsupportedMode/failure observations through the current ExecutionObservation family. Interrupted/failed Save preserves the old valid file and cannot acknowledge a new clean revision; malformed/resource-limited/cancelled candidate validation cannot install. Existing Chess/Shogi/Xiangqi current-envelope/observation/atomic-write/admission behavior has focused regression coverage. No invented Tori result/score family or analysis facility.
- [ ] #6 Independent losslessness/admission/strict replay review is resolved; measured capacity policy, commands, source revisions, limitations and applicable CI are recorded. The supplemental mapping assigns codec support to actual consumer cases while leaving their live installation verdict pending the runtime task. Native-controller Tori installation remains explicitly unsupported until that task; normal product Open/New Game stays gated until42supplies actual product support; no accepted unsaveable state or product chooser exposure.
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
1. Sole builder/integrator rootCodex in clean primarycodex/rewrite1e51ea5; pinnedBacklog1.55.1, no other activeowner and Done69/72/73/85 verified. Preserve unrelatedworktrees, nativeprofile/103fixtures/713assertions and acceptedR1-R6.
2. Add immutable Tori current-envelope candidate payload: native initial/current records, exact command/control journal, settings/side, retained rewind lineage and current-family historical observations. Strict raw duplicate-preserving preflight and authoritative native replay validate all semantics; no live runtime/controller/installable Tori capability.
3. Derive bounded aggregate replay charges from actual raw command counts/kinds before expensive native parsing (never trust saved replay_work), charge repeated native parses/reconstruction/observations/prefixes; measure real complete long records and choose documented bounded whole-document budget. Apply actual encoded bytes/events/controls/observations/IDs and Stop/Resign/close reservation before accepting immutable candidate.
4. Reuse existing FileWorker/atomic_write with pure Tori validation/save work and typed resource facts including Tori scope. Keep public Open/ActiveDocument/NewGame unsupported; failed/cancelled/stale jobs cannot yield success/clean acknowledgement.
5. Execute independent native fixture/strict negative/boundary/headroom/long-record/atomic-file/current-game regressions; independent read-only source/security/capacity/persistence review, resolve findings, formatting/lint/applicable native+desktopCI. Record actual measurements/revisions/failures and131handoff.
6. Finalize130only after all6AC/4DoD supported, normalintegration/push; stop before131/132/77acceptance/42.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Authorized130only. Startup clean/aligned1e51ea51b3ba6997b75a7c11a25df0c7f4eeaad5 on primarycodex/rewrite; globalBacklog1.55.0 notusedformutations, pinned1.55.1 used. Separate tori130_review read-only reviewer inspects actual codecs/strict replay/resource/atomicfile, rootsolebuilder. No taskworktree orGUIcampaignneeded. Native parser has bounded but not cancellable per-record replay; aggregate admission must precharge before native deserialization and check worker cancellation between operations; no trust in unvalidated saved replay_work.

Immutable current-envelope Tori codec/admission implemented, not runtime installation. New tori_document module supplies strictduplicatepreserving wholecandidate preflight, authoritative native parser+journal replay, retainedbranches/settings/observations, perrecord+aggregate bounds and consumedStop/Resign/close reservation. Existing16MiB/4096events/8192controls/256observations/native512KiB512events65536units unchanged; measured newaggregatebudget262144 with root128 and allrepeatpasses. Actual360board doc206796B/131424units, oneobs364006B/197021, secondrefuses262618+1040>262144; StopResignClose364641B/197026stillstrictreplaysandactualexistingworkerSave4022msPASS. Retained180prefix287206B/213815, root256obs1316178B/66176. Real4096event,8192control,256obsboundariesPASS. Fullsessionallfeatures230PASScheckpoint plusfinal16fastnewchecks/1longmeasurement; core38PASS713nativeassertions, actualchildUnsupported→codecreopen1PASS; clippy/format/TauriCheckPASS(20oldTaowarnings); Node24tsc+36frontendPASS afterknownbrokenHomebrewNode26failure. Independenttori130_review source/security/capacity/persistence PASS and independently16focused+child+lasttargetedchecks; selectionfloor/cancellation/reservation/strictboundaryfindingsresolved. Initialtestinclude/capabilitynames andnativepaddedRawtestfailures corrected withoutnative/expectationchange; fullactualevidence docs. ExactsourceCI/integrationstillpending, remainInReview. No app/GUI/externalengine ornexttaskimplementation.
<!-- SECTION:NOTES:END -->
