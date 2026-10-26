---
id: MSH-69
title: Extend shared native context and routing for real chess and shogi consumers
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T14:16:00Z'
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
- [x] #1 A small typed game/profile/version and canonical-record envelope supports actual chess and shogi payloads without universal rules traits,plugin ABI or common board storage.
- [x] #2 Internal engine/library/console routing preserves complete action versus declaration/claim versus concluded/incomplete/failure results and exact purpose,history,request,session/process-generation and limit acceptance.
- [x] #3 Existing chess protocol/record behavior has an explicit compatible adapter or documented version rejection; strict payload parsing preserves duplicate-field rejection and profile/schema mismatches cannot dispatch to another game.
- [x] #4 Roundtrip actual native chess/shogi contexts and reject cross-game/profile,wrong-purpose,stale and unknown envelopes. Unsupported shogi execution capabilities stay unadvertised until its engine consumer lands.
- [x] #5 This ticket solely owns common dispatch/wire evolution; MSH41 owns shogi rules/search/notation behavior. No dependency on MSH41 or MSH46 final completion is introduced.
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

Reviewed source commit42777c4 integrated normally as f3cf879acdd5720ee3ce9dd2e0796984ae998bee and pushed to origin/codex/rewrite. Final integrated local macOS27 arm64, Rust1.89 native/Rust1.90 Tauri: cargo test --workspace --locked passed291 tests (including doc tests); desktop-session --all-features passed46; Tauri cargo clippy --locked --all-targets -- -D warnings and cargo test --locked passed2; regenerated frontend fixtures have zero diff. Third-party vendored Tao emitted existing warnings; product checks passed. Frontend check/tests previously passed on identical source (78 tests).

Required exact-source CI pending: Rust workspace https://github.com/jweathers777/mShogi/actions/runs/40335673851 and Desktop packages https://github.com/jweathers777/mShogi/actions/runs/40335676334, both head f3cf879. Prior MSH67 Desktop packages40325840456 has now completed successfully; original accepted MSH67 source/history evidence remains unchanged. No closure claim until current required CI completes.

Required CI complete on exact integrated source f3cf879acdd5720ee3ce9dd2e0796984ae998bee: Rust workspace40335673851 PASS Linux ubuntu24.04, macOS15 and Windows2022; Desktop packages40335676334 PASS all three. Native CI includes format/lint/tests, isolated desktop QA constructors, identical snapshot fixtures, build and console smoke. Desktop CI includes locked frontend/native checks, normal package and bundled-engine smoke on all three, separate verification-package installed GUI interactions on Linux/Windows; this is not a new macOS GUI-interaction claim. Existing M3 macOS GUI/native termination and Tao patch evidence is reused unchanged. Local cargo check --locked --features desktop-verification also passed. Current CI had no failed jobs or reruns.

AC coverage:1 native typed envelope/derived identities and real native payloads;2 native command routing plus v3 purpose/native result distinctions, exact context and common supervision;3 untouched v2 adapter/native encodings/observations with strict raw version/native parsing and explicit rejection;4 actual roundtrips, forged/stale/cross-game contexts and actual entrypoint tests, empty shogi capabilities;5 only common dispatch/transport touched, MSH41–47 remain open. M2/M3 closures, MSH61 persistence guarantee, MSH67 selected profile/mutual-check fixtures/evidence and roadmap correction preserved. No new rule/product conflict found. Source42777c4 is integrated and pushed via f3cf879; final task-only evidence update follows. Disposable shared-context worktree has no remaining builder/reviewer purpose and will be archived after the evidence push.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented and independently reviewed MSH-69 only. CanonicalRecord envelope v1 explicitly carries actual native chess/shogi identity, immutable profile edition and native schema through strict RawValue parsing; native command owners retain admission. Shared transport v3 runs through the existing single supervised owner, retains exact purpose/history/selection/request/session/process/limit acceptance and adapts real chess execution without changing v2 consumers or archives. Shogi execution is explicitly unsupported and unscored until MSH70; capabilities remain empty.

Reviewed source42777c4 integrated/pushed as f3cf879acdd5720ee3ce9dd2e0796984ae998bee. Separate read-only agent source review plus independent focused binary execution found no actionable findings. Final local291 native tests,46 QA-feature desktop-session tests,78 frontend tests and2 Tauri tests passed; format/lint/check and unchanged native snapshot fixtures supported. Native CI40335673851 and Desktop CI40335676334 passed Linux/macOS/Windows on f3cf879. See docs/m4/shared-context.md for compatibility, limits, measured composed sizes and scope.

No native record encoding/rules/proof expectation, archive or desktop feature was changed. Shared byte limits were not raised. Chess Full record restoration remains explicitly unsupported; bounded separate Full analysis is preserved. Next eligible shogi queue: MSH68 console adapters and MSH70 real engine consumer; neither started. MSH41–47 and M4 remain open.
<!-- SECTION:FINAL_SUMMARY:END -->
