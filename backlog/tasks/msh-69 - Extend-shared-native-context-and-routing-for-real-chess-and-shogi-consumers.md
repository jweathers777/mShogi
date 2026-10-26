---
id: MSH-69
title: Extend shared native context and routing for real chess and shogi consumers
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T14:00:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-67
  - MSH-62
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/shared-context.md
type: task
ordinal: 69000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Strict serialization/protocol fixtures, actual chess subprocess regression and concrete shogi record roundtrips; purpose/history/generation replacement cases.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A small typed game/profile/version and canonical-record envelope supports actual chess and shogi payloads without universal rules traits,plugin ABI or common board storage.
- [ ] #2 Internal engine/library/console routing preserves complete action versus declaration/claim versus concluded/incomplete/failure results and exact purpose,history,request,session/process-generation and limit acceptance.
- [ ] #3 Existing chess protocol/record behavior has an explicit compatible adapter or documented version rejection; strict payload parsing preserves duplicate-field rejection and profile/schema mismatches cannot dispatch to another game.
- [ ] #4 Roundtrip actual native chess/shogi contexts and reject cross-game/profile,wrong-purpose,stale and unknown envelopes. Unsupported shogi execution capabilities stay unadvertised until its engine consumer lands.
- [ ] #5 This ticket solely owns common dispatch/wire evolution; MSH41 owns shogi rules/search/notation behavior. No dependency on MSH41 or MSH46 final completion is introduced.
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
1. Audit actual native records, command owners and supervised engine consumers; preserve chess history observations as data and complete match admission with ChessMatch.
2. Add a small typed chess/shogi canonical-record envelope with derived immutable profile/edition/schema identities and strict RawValue routing to native parsers. Keep native actions, sides, results and record encodings authoritative; provide tagged command dispatch to borrowed native owners.
3. Preserve v2 chess request/response APIs and saved observations unchanged. Add explicit v3 shared contexts/purposes, game-qualified target provenance and response/capability routing. Reuse the real chess executor, reject unsupported shogi execution unscored, and retain chess-only bounded Full semantics.
4. Evolve the existing single supervisor through private v2/v3 wrappers, keeping one current/pending/result owner, global monotonic request/process identities, exact-context acceptance, cancellation, bounded I/O/diagnostics and cleanup. Measure composed payload/envelope sizes and preserve or justify caps before acceptance.
5. Verify actual native chess/shogi roundtrips, strict outer/nested rejection, identity mismatches, same-board/different-history and same-ply terminal commands, purpose/selection/limits/identity mismatches, cross-game cancellation/replacement and actual chess/shogi-unsupported subprocess routing. Reuse unchanged M2/M3/native tests.
6. Obtain separate final boundary review, resolve material findings, run normal native and desktop CI, integrate/push codex/rewrite, complete supported AC/DoD and archive worktree. Stop after69; do not implement68/70/73 or desktop shogi.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started MSH-69 by @codex on codex/shared-context in managed /Users/johnweathers/.codex/worktrees/shared-context/mShogi, based ea05eb13b44a579611f349728f06fed3881c4e4c. Pinned Backlog CLI1.55.1 matches project baseline. Main checkout is clean codex/rewrite. MSH-62/67 dependencies Done; MSH-67 profile, accepted proof, native APIs and prior evidence are established inputs.

Read-only independent consumer audit confirmed that chess GameRecord observations must remain inert, AdjudicatedGame replay executes accepted claims only, and ChessMatch owns complete terminal/clock command admission. The common dispatcher will borrow native ChessMatch or shogi Game rather than synthesize a chess match from a history record. Existing v2 engine contexts are persisted as desktop historical observations, so the selected compatibility strategy preserves their types/wire and archive semantics while adding a separate explicit v3 shared transport. A single supervisor will own both versions. No source edits yet and no future consumer implementation started.

Implementation complete for boundary review: native CanonicalRecord envelope v1 derives game/profile/edition/schema, preserves RawValue and native parsers; borrowed native command dispatch leaves claim/declaration/terminal admission native. Shared transport v3 uses actual native records and game-qualified sides, exact complete context and selection acceptance, existing chess executor, explicit unscored unsupported shogi execution and empty shogi capabilities. Legacy v2 API/wire and saved observations remain unchanged; both versions share the existing supervised owner and monotonic retirement ledger. No MSH-67 rule/fixture/proof changes or future consumer work.

Local macOS arm64 verification: cargo test --workspace --all-targets --locked passed; final compact-dispatch/capacity additions then passed focused core native_envelope5, engine shared6, console shared_internal3 and cargo clippy --workspace --all-targets --locked -- -D warnings. npm ci, npm run check and npm test passed (13 files/78 tests). Composed sizes: 320-event shogi native79733/envelope79914/request80062/response80328 bytes; 4096 u64-max chess clock observations envelope573826/request573974/response574126. Exact1MiB input admitted; one extra byte rejected. No limit increases. Initial old protocol-invalid test exposed lost legacy diagnostic; fixed strict legacy-shaped unsupported-version handling before execution, rerun passed. Initial lint flagged large native dispatch variants; boxed shogi request/undo, rerun passed.

Independent completed review: separate read-only agent msh69_final_review inspected all source/test boundary changes against ea05eb1, AGENTS/task/design/shared-context contract; independently executed current compiled native_envelope5/5, shared6/6, shared_internal3/3 on macOS; no actionable findings. This is independent agent source review plus direct test-binary execution, not CI or a separate human review. Native/desktop CI and integration remain pending.
<!-- SECTION:NOTES:END -->
