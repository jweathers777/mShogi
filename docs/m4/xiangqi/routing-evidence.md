# MSH-95 routing boundary evidence

Baseline: `0a90ec8e8669b5dff70f37b8ceaec48d1702be1b`, primary `codex/rewrite`.
Root@codex is sole product builder/integrator; no temporary worktree was required.
The only initial dirty work was this session's preserved scope-conflict notes.
The owner explicitly corrected the evaluator/search misassignment and retained
existing95/97/105 ownership, dependencies and acceptance criteria. See MSH-95's
plan and notes. [Delivered contract](shared-routing.md) defines the boundary.
Final tested/integrated source and actual CI links/outcomes are recorded in the
MSH-95 completion summary after execution, not guessed before the commit exists.

## Acceptance mapping

| Criterion | Evidence |
| --- | --- |
|1 strict native identity/payload routing |`native_xiangqi_envelope`5tests plus existing `native_envelope`5: real imported native root, supplied counters, known history/warning/fourth-loss, native claims and audits; outer/native duplicate/unknown/missing/unsupported/conflicting identity rejection before normalization, both directions of wrong-game payload; qualified Red/Black side parsing |
|2 complete context and retired work |`xiangqi_routing`5tests mutate purpose, full native history, request/session/generation, settings and every selection field; actual native turn validation and foreign result-family refusal; `xiangqi_internal`5tests include real child echo, same-board/different-history forgery, cancellation/replacement and undo/reopen/selection retirement |
|3 native records and honest unsupported execution |Complete MSH-94 ledger stays authoritative; native dispatch preserves `Submission.result` AND undo, non-turn claims/resignation stay native, board actions enforce native actor. All3well-formed purpose shapes return unscored UnsupportedMode; empty capability list. Concluded/proven/unknown native data preserved without invented execution/outcome |
|4 bounds/supervision/compatibility/review |512-command/1024-event actual encoded composition; exact1MiB/+1 tests; existing supervisor/queues/diagnostics/cleanup unchanged. Real existing Chess/Shogi subprocess suites and new Xiangqi lifecycle tests; desktop unsupported-New/Replace preserves current tokens/context/document/pending work. Separate completed-boundary review with actual independent execution |

No evaluator/search, exact verifier, console feature, protocol, desktop session,
archive or renderer was added. No governing rule or historical fixture changed.
No reference engine comparison is needed for routing; MSH-94/91–93 rule/reference
evidence is reused, never converted into search or execution evidence.

## Actual focused verification and methods

Local environment: macOS27.0.1 arm64, native pinned Rust1.89.0; frontend pinned
Node24.19.0 from the existing Codex runtime. The system Homebrew Node26 binary
failed to load a missing simdjson dylib; no project check ran through it and no
system repair/configuration change was made. The correct pinned runtime was used
for the subsequent actual checks.

- `cargo test --locked -p mshogi-core --test native_xiangqi_envelope`: PASS5.
- `cargo test --locked -p mshogi-engine --test xiangqi_routing -- --nocapture`:
  PASS5;512commands/1024events measured native253656/envelope253832/
  request254001/response254283 bytes; no limits increased.
- `cargo test --locked -p mshogi-console --test xiangqi_internal`: PASS5, actual
  normal `mshogi internal-search` child and separately labeled adversarial helper.
- `cargo test --locked -p mshogi-desktop-session xiangqi_tags_do_not_enable`:
  PASS1, native controller rejection/retirement protection, not GUI evidence.
- Existing focused core native_envelope5, engine shared6+shogi_protocol6,
  console internal5+shared_internal3+shogi_engine3 remain passing in independent
  review. These are actual Chess/Shogi execution compatibility, not a mock-only
  assertion of unchanged behavior.
- Initial development compilation caught required exhaustive enum matches and
  test-only API/type mistakes; a clippy unit-variant-pattern warning was fixed.
  Those attempts are not counted as passed checks. Final full-regression
  outcomes are appended below once commands complete.

## Independent review

Separate reviewer `/root/routing95_review` did not author any production or test
file. [Initial audit](routing-reviews/initial-audit.md) identified concrete native
metadata, target-side, unsupported-fallback, native submission, desktop replacement
and capacity risks before completed review. All are addressed.
[Completed review](routing-reviews/completed-boundary-review.md) independently
read the bounded source/test diff and actually executed **45 focused tests**,
including the existing published-completion cancel/replace test. Verdict **PASS**;
no material unresolved finding. The report binds reviewed production SHA256s,
actual commands, scope and platform limitations. Root owns full regressions/CI.

The isolated test delegate authored only `xiangqi_internal.rs` and adversarial
fixture27. [Process report](routing-reviews/process-verification.md) distinguishes
its verification from independent review, preserves corrected failed command/
compile attempts, and explains exact real-child versus test-helper evidence.
The30ms sleep grants scheduling opportunity, not proof of exact child launch;
shutdown acknowledgment is worker exit after cleanup attempt, not independent
OS process-reaping instrumentation. Existing supervised cleanup regressions are
reused; no such instrumentation enters normal distributables.

## Completion boundary

No MSH-105 work began. Its94/95 dependencies remain unchanged; it becomes
ready by dependency after95 closes. General exact no-win proof/feasibility is
not claimed. MSH-97 remains after84/95, with84 requiring105, and owns evaluator,
search and real engine capabilities. Native handoff84, full game45 and M4 stay
open until their required complete scopes are satisfied. No follow-on task is
silently started by routing or by a successful CI build.

## Completed local regressions

| Actual command | Outcome |
| --- | --- |
| `cargo fmt --all -- --check` |PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` |PASS after correcting the test-only unit-pattern lint |
| `cargo test --workspace --locked` |PASS478,0failed,2existing intentionally ignored deep-perft tests |
| `cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings` |PASS |
| `cargo test --locked -p mshogi-desktop-session --all-features` |PASS93,0failed |
| `cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- desktop/src/__fixtures__` then `git diff --exit-code -- desktop/src/__fixtures__` |PASS; existing frontend/native snapshots unchanged |
| `npm run check` with pinned Node24.19.0 |PASS strict TypeScript |
| `npm test` with pinned Node24.19.0 |PASS21files/161tests |

These are local source/controller/frontend/subprocess categories, not new
three-platform GUI evidence. Normal native and package CI bind the integrated
production revision; their actual final outcomes/URLs are recorded in the task
completion summary. Existing M1/M2/M3/Standard Shogi/Xiangqi91–94 evidence remains
applicable to unchanged behavior. No full exact-verifier or playing-strength
result is claimed by any routing check.
