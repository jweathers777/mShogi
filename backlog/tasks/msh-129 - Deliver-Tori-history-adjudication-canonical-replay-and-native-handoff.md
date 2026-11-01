---
id: MSH-129
title: Deliver Tori history adjudication canonical replay and native handoff
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-01T12:21:00Z'
updated_date: '2026-11-01T19:50:00Z'
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
  - docs/m4/tori/native-game-evidence.md
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Root Codex is sole builder/integrator in clean primary codex/rewrite e1f7ed4; pinned Backlog1.55.1, Done128 prerequisite, origin alignment and no competing active owner verified. Preserve unrelated worktrees, selected profile, all103 expectations/ownership and completed evidence.
2. Add native Game over LocalPosition, exact board/hands/turn repetition keys and latest-three departure obligations. Install board-terminal outcomes before repetition; context-checked board/resignation/unsupported/control commands separate concluded from incomplete unscored administration. Use full semantic context and independent monotonic native runtime authority.
3. Implement exact full-game undo/redo and immutable prefix views with retired authority. Add strict versioned native root/game records, duplicate-preserving parsing before normalization and authoritative replay comparison of all redundant prefixes/effects/obligations/conclusions. Measure real native encoded sizes/work and enforce bounded precommit admission plus terminal/stop headroom.
4. Extend native.rs typed game/profile/schema/side/command/CanonicalRecord routing with real Tori payloads and a core consumer probe. Adapt only necessary exhaustive consumers to explicit unsupported/unscored Tori branches; no engine/session/document/frontend implementation or capability advertisement.
5. Execute every129 native harness assertion with actual results while retaining127/128 passes and genuinely later77/42 facets pending. Add independent full forced8/unequal16 sequences, unit/adversarial/metamorphic/reversal/context/record/capacity tests and existing-game routing regressions.
6. Obtain independent completed-boundary review, resolve findings, run focused/native/shared checks plus applicable three-platform native CI. Record exact measured source/evidence, integrate/push normally, finalize129 only with all7AC/4DoD supported; stop before85/77/42/other variants/M5.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

Owner request2026-11-01 authorizes129only. Startup clean primarycodex/rewrite e1f7ed407f360ef7b4267225c6921e73ddd1417d aligned with origin; no other InProgress/InReview task owner. Local128 API is complete and native repetition/record/dispatch remains129. RootCodex builds; separate read-only fixture-contract research underway, completed-diff independent review required. No external engine orGUI campaign and no later task activation.

Native Game/T06 obligations/context-lineage, exact reversal/prefix, strict native schema1 RawValue replay and typed dispatch implemented. Actual immutable-corpus adapter713executed/713passed/14pending/0failed;83later-consumer requirements staypending. Complete forced8/unequal16 and40board+2resignation fullGame reversal pass. Initial adapter mate projection used winner rather than expected loser; corrected equivalent native projection, no fixture/rule change. Real360board record156573B/65341replayunits includes104captures/103drops/16promotions; nextboard refused beforemutation, stop+resign stillencode/replay. Real510event path refusesordinary; reserved stop511/resign512 fit165930B. Native512KiB/512event/65536unit bounds and4096B/nonboard reserve are independently checked at exactpolicy/parser/admission boundaries. Core/engine/desktop regressions and separate readonlyreview running; nativeCI notyetclaimed. No later task activation.

Independent tori129_review finalPASS on frozen nativeboundary; no unresolvedmaterial findings. Actualmethod separate readonlysemantic/API/profile/fixture/diff review plus independently13nativegame/713assertions,3Tori+10existing envelope,2engine routing and1actualsubprocess checks. Eventlimit/longredo/prefix test challenges addressed; minor forgeableview panic risk resolved by private native state/getter. Reviewedgame.rs SHAa3987b9276971f424f191ff28d9f4ee979bdedae604afb546c159e170d0c92c6. Finalfocus38ToriPASS; all-targetworkspaceclippy-Dwarnings/formatPASS; existing41engine/201desktop-session/19selectedsubprocesschecksPASS. Initial desktoprustdocE0463artifactfailure retained; focusedserialdoctest rerunPASS0tests. NativeGameprefix records preserve complete root/historywithoutfuture; real360sequence covers104captures/103drops/16promotions andfullundo+redo everyprefix. Fullgraph129/344doctor/integrityPASS withdeps/oldDone/tasks/profile/fixtures/localrules/existingrules/search/runner/frontend/workflows unchanged. NativeCI stillpending actualsourcecommit; remainInReview.

Final core/conformance rerun278PASS/0failed across34reported suites;2existingdeepChessoptinsnotrun. New38Tori+3coreconsumer+2engineunsupported+1actualsubprocess checksPASS, noignoredTori. No code changes after independentreviewhash; current docs/evidence finalized for sourcecommit and real3platformnativeCI. TaskInReview untilactualCI/integration evidence.

Reviewednative source integrated/pushed e1cc89d454db5e0ce98da34bf4cda7960d395413, nativeCI41343664138 andautomaticDesktop41343664148started. Postworkspaceinspection foundseparatelybuiltTauri exhaustive defaultsavefilename match needed explicitToriunsupportedarm. AddedreturnErr beforepicker/savequeue, noUIcapability; independentfollowupPASS and localTauri all-targetcheckPASS with20existingTao warnings. Core/engine/testhashesunchanged. Additive normalcommit/push retainsfirstsource and CIhistory; finalnativeCIrequired beforeDone.
<!-- SECTION:NOTES:END -->
