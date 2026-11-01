---
id: MSH-129
title: Deliver Tori history adjudication canonical replay and native handoff
status: Ready
assignee: []
created_date: '2026-11-01T12:21:00Z'
updated_date: '2026-11-01T13:44:00Z'
labels:
  - m4
  - owner-msh-85
milestone: m-4
dependencies:
  - MSH-128
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
  - docs/testing/conformance-policy.md
type: task
ordinal: 129000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-81 is Done at6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc. Selected profile mshogi.tori.modern.v1 and all103 independently expected fixture specifications are fixed inputs; do not reopen historical acquisition or import Standard Shogi defaults. This is a component owned by coordinating MSH-85 under complete game delivery MSH-42, following the repository flat component-ID precedent used for MSH-84/91-94. Own the authoritative Tori Game, history/context and bounded canonical-record handoff on top of complete local legality. It supplies native core consumers that MSH-77 can later integrate. Native dispatch belongs here; desktop runtime/document installation, engine execution and UI remain later owners. Extending exhaustive game matches may need minimal compile-preserving unsupported branches, never a parallel session or pretend engine.

Verification: focused reproducible commands at exact tested source, independent bounded review, normal codex/rewrite integration/push and relevant native/shared regression/CI for actually changed code. Distinguish passed/failed/not-run and reference observations from independently expected facts. No unrelated GUI campaign, playing-strength claim, new exact-proof requirement or future-variant implementation. Future worker records an implementation plan only on activation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Root installation and accepted board actions derive mate/stalemate before repetition; no initial prior history is inferred. Implement exact T06 board/hands/turn identity, root occurrence1, latest-three indices and forbidden first departures, departure-side responsibility, consumed obligations and forced repetition-forfeit loss. Execute full8-ply forced-check and16-ply unequal cycles, history units and same-board/different-history cases; preserve distinction from CSA/WXF/chess and check-evasion/perpetual-check alternatives.
- [ ] #2 A native command/context boundary admits complete local choices only in a live game and with full expected root/history/context plus fresh never-rewound native runtime lineage. Out-of-turn live resignation is permitted; terminal/unsupported agreement/impasse/illegal and stale commands reject atomically. Non-board commands preserve board ply/occurrences. Administrative stop, work/resource exhaustion and failure stay incomplete/unscored and do not invent native conclusions. Undo/prefix selection/replay retirement cannot revive old native requests.
- [ ] #3 Versioned strict native records preserve explicit game/profile/edition/schema, standard/custom root/provenance and declared counter, ordered complete commands/events/effects, exact prefixes/board-ply mapping, occurrence/departure obligations and conclusions. Authoritative replay recomputes and checks every redundant derived field at every prefix, including imported terminal roots and resignation. Duplicate/unknown/missing/conflicting fields and unsupported identities reject without intermediate-map erasure; malformed/tampered/corrupt/resource-limited records leave an existing game unchanged.
- [ ] #4 Exact full-game apply/undo/redo and record roundtrips cover board/captures/hands/promotions plus occurrence lists, obligations, conclusions, event indices/caches and same-board/different-history context. Historical prefixes exclude future knowledge. Successful reconstruction gives fresh runtime lineage while canonical semantics remain identical; tests separate native lineage from later request/session/process/purpose/limit acceptance owned by MSH-77 and the actual engine consumer.
- [ ] #5 Measured actual native encoded bytes, events/commands and replay work establish documented bounded parsing/replay/diagnostics and admission before mutation, including headroom for supported terminal/stop operations. Long deterministic real records and exact boundary/refusal tests prove ordinary accepted native history can still serialize completely; no truncation, arbitrary constant-only fix or false game outcome. Supply size/work metadata or documented methods for later desktop admission; desktop observations/control quotas and atomic file Save/Load stay MSH-77.
- [ ] #6 A real core consumer probe uses typed Tori game/profile/native-side/action/command dispatch and CanonicalRecord RawValue envelope routing through the authoritative native parser, with root/full-history roundtrips and wrong-game/profile/schema/edition rejection. Provide native complete legal choices, immutable inspection, exact event/history/conclusion/context and separate repetition versus saved-state identities for MSH-77. Minimal exhaustive-match compatibility adapters keep existing games working; Tori execution remains explicitly unsupported/unscored and unadvertised until the real engine, with no new desktop document/session/UI or transport framework.
- [ ] #7 The complete103-ID ownership manifest has execution evidence for every in-scope native assertion and explicit named later-consumer facets; no stub/ignored native cases or changed expectations hide a gap. Reuse A/B/C evidence unchanged, execute D history/record/native-probe suites and appropriate shared dispatch regressions/CI. Record a reproducible native handoff/capability/coverage document and independent review resolving material findings before coordinating MSH-85 can close.
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
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.
<!-- SECTION:NOTES:END -->
