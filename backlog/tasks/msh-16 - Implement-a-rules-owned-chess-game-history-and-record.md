---
id: MSH-16
title: Implement a rules-owned chess game history and record
status: Done
assignee:
  - '@codex'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T21:32:00Z'
labels:
  - rules
  - history
  - state
milestone: m-2
dependencies:
  - MSH-15
documentation:
  - docs/review/m1-action-boundary.md
  - docs/design.md
  - docs/rules/chess-match-v1.md
  - docs/rules/chess-history-record-v1.md
priority: high
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chess Position is deliberately local: it holds no history, and its Eq is saved-state identity only. The only undo stack today is the console session's adapter-level Vec<Undo>. Repetition, claims and automatic results need history owned by the rules library (A-01, A-03); the M1 action-boundary review records this as required change 6. A canonical game record must also carry the rules-profile ID, initial state and complete actions without effects or private undo layout (A-03, A-06).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A rules-library game type owns the current position plus the history the match profile requires, including repetition identity distinct from saved-state equality and from any cache key
- [x] #2 Applying and undoing through the game restores position, history and repetition tables exactly; sequence-property checks in the style of MSH-12 cover it
- [x] #3 A canonical game record (profile ID, initial state, ordered complete actions, no effects, no private undo layout) round-trips and is replayable deterministically
- [x] #4 Console and future adapters use the rules-owned history rather than keeping their own rule-relevant state
- [x] #5 Records preserve known-history origin (imported FEN never infers earlier repetitions), supplied counters and profile/policy identity; match commands and clock observations can be replayed without reconstructing private undo state.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a rules-owned chess Game with exact repetition identities/counts, explicit standard/imported history origins and claim policy; implement atomic apply and owned LIFO undo with complete history restoration. 2. Add a versioned chess-only canonical record of initial state, origin, policy, complete actions and typed command/clock observations. Rebuild history and replay observations at their action context without private undo serialization; semantic command/clock adjudication remains MSH-17/18. 3. Migrate the diagnostic console onto Game and expose history diagnostics plus record round-trip use. 4. Verify hand-derived identity/history fixtures, fixed-seed apply/undo and record/rejection properties, real-process adapter integration; obtain independent review, resolve findings, integrate and record native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex; branch codex/msh-16-history, existing next-task checkout from clean origin/codex/rewrite 504992b. Backlog.md 1.55.1 matches baseline; no active worker was present. MSH-15 is Done. MSH-16 implements the history/record foundation; it does not silently claim MSH-17/18 outcome or clock adjudication. Command/clock observation replay retains their context and values; later tasks validate and apply their semantics.

Implementation ready for independent review: Game owns exact repetition identity/count map, history origin/policy and LIFO snapshots; console uses Game with history and atomic move-only record export/load. Canonical strict JSON supports ordered moves and typed command/clock observations with checked ply context and separate total event sequence; intended actions remain command data. History replay preserves observations without validating their later MSH-17/18 semantics. Focused core history/record tests pass (5 + 5), covering 933 seeded plies, independent MSH-15 identities/counts and M1 special transition after-states; console process tests pass (18). Decoder regression caught Serde unit-command extra-field acceptance; empty-struct wire variants now reject extras. Pinned production Serde/JSON dependencies use already-locked versions; local manifests/license texts reviewed as MIT OR Apache-2.0; transitive versions unchanged.

Independent review_history_boundary reviewed the full implementation diff against 504992b after its earlier scope review. No material findings. Reviewer independently reran core history/record tests (10 passed) and probed real console rejection of duplicate event kind, action and nullable intended fields. All five acceptance criteria are supported at the documented history/observation boundary. Observations carry independent total sequence plus action context; intended claims are never inserted as played actions. Semantic command/clock validation stays explicitly MSH-17/18. Full match replay remains MSH-23/25.

Verification on macOS 27.0.1 (26A434), arm64, rustc 1.89.0 (29483883e), cargo 1.89.0: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test -p mshogi-core --test game_history --test game_record --locked; cargo test -p mshogi-console --test cli --locked; cargo test --workspace --locked; cargo build --workspace --locked; git diff --check; Backlog.md 1.55.1 doctor all passed. Focused coverage is 10 core tests (933 fixed-seed plies plus independent profile/special fixtures) and 18 console tests including fresh-process record reload/undo and atomic rejection. Existing two opt-in deep test functions remain ignored; no new deep/oracle rerun is claimed. Source review covered the working diff based on integrated 504992b. Actual source revision and native CI outcomes will be appended after normal integration.

Integrated by fast-forward and normal push to origin/codex/rewrite at source revision 4c7d58e4592d77cc0c6a4329d6673290663f6d36. Native CI https://github.com/jweathers777/mShogi/actions/runs/40134522513 passed on macos-15 (job 120993101708), windows-2022 (120993101885) and ubuntu-24.04 (120993101902). All jobs passed pinned toolchain identity, format, lint, workspace tests, build and console smoke. The final task closure changes only Backlog bookkeeping and reuses this exact tested source. Both task and integration checkouts were clean after source integration. Unchanged M1 deep/oracle evidence remains applicable; this task adds focused history/record/process evidence rather than repeating that audit.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed rules-owned Game with explicit known/imported origin and claim policy, exact repetition identity/counts, atomic action application and fully restoring owned undo. Added strict versioned canonical chess JSON records with deterministic history reconstruction, total event order and typed command/clock observation contexts; no private undo/effects are serialized. Console now uses Game and supports history, record export and atomic move-only reload/undo, refusing observation-bearing match records explicitly. All five ACs verified by independent fixtures, 933 fixed-seed plies, 10 core tests and 18 console process tests; workspace checks and native Linux/macOS/Windows CI passed. Independent review found no material issues. Source/integrated revision 4c7d58e. Observation transport does not adjudicate claims, automatic results, commands or clocks: MSH-17/18 retain those obligations, and MSH-23/25 retain full match replay. Next eligible task is MSH-17.
<!-- SECTION:FINAL_SUMMARY:END -->
