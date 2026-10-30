# MSH-118/R3 native application ownership

Baseline `7414b795d47e9a6408f1b21f07cb1baacfddf8dc`, primary `codex/rewrite`.
Root owns integration and host changes; isolated workers owned native migration,
resource types and external GUI verification. No worker committed or integrated.
No task worktree was created. Existing unrelated worktrees remain unchanged.

## Delivered boundary

The [current native contract](../desktop/native-application.md) records one
accepted `ActiveDocument::{Chess,Shogi,Xiangqi}` and one `RuntimeMetadata` instance.
Native candidate copies are pending work, never another accepted live session.
Shogi and Xiangqi contain no dormant Chess game. SharedState retains admission
bookkeeping only; common document epoch moved into native runtime metadata.

ChessMutation, ShogiSeed and XiangqiDraft preserve specialized native semantics
while moving commands, result application, replacement replay and validation
onto the bounded document worker. One native admission completion path and one
`install_document` transaction own acceptance. Old Chess-only Open work/results,
three installation paths and the optional native overlays are deleted.

The current Chess snapshot carries actual observations, selected analysis,
telemetry and native MatchRecord instead of empty common projections. Its full
native claim/intended-claim, Full proof/Unknown, history, preferences, unresolved
outcome and Take back behavior remains available. Shogi declaration semantics
and Xiangqi WXF event/claimant/warning/proof distinctions remain specialized.
Core rules, search, engine and the Tao patch are unchanged.

ApplicationLifecycle owns exclusive RAII command/file reservations and exact
document-token close approval. Host technical shutdown observation remains
separate. Current native close endpoints/events serve all games; the old Chess
frontend DTO/IPC is a narrow forwarder with no native state or old-file reader.
MSH-119 deletes that forwarder together with its actual frontend callers.

The current document envelope and all established product limits remain.
Typed resource facts distinguish accepted usage from a rejected candidate,
ordinary/exact components, bytes, native events, controls, replay, identities and
validation budget. Unknown/multiple causes and bounded diagnostics remain
explicit. Save, Save As and complete history/observation retention are preserved.

## Verification and independent findings

Local working-tree checks are [indexed with original log hashes](evidence/msh118/local-index.json).
Default native165/165 passed before the final focused review corrections; the
final corrections have separate runtime6, budget2, byte1, replay1 and
verification-control1 checks. These are not represented as a final pinned full
suite. All-target/all-feature native Clippy passes. Root checks pass:
workspace Clippy, host9, actual console subprocess12, frontend243, TypeScript,
GUI-verifier50 and fixture regeneration. Formatting/diff/Backlog doctor pass.
Bundled Node24.19.0 resolves PATH Node23; pinned Backlog1.55.1 resolves PATH1.55.0.

Fresh read-only reviewer `/root/r3_boundary_audit`, not a builder, inspected
the full source/deletion diff and independently ran native runtime/resource/
lifecycle, byte/control/replay/budget, frontend20 and GUI-verifier37 focused
checks. It did not repeat the expensive Full byte stress locally; mandatory CI
will establish that at the committed revision.

| Finding | Correction and evidence |
| --- | --- |
| Open picker could finish after same-token close request | Post-picker serialized close recheck; host regression proves no file work is queued. |
| Xiangqi byte-only reserve refusal claimed256 observations | Separate native count and encoded-observation-byte facts with actual response reserve; real producer regression, including zero/control-only cases. |
| Deadline and known identity refusals lost resource facts | Worker distinguishes actual deadline from cancellation; native identity/IO paths publish typed facts; deterministic14999/15001ms checks. |
| Legacy public command bypassed close; snapshot could panic across games | Native close admission guard; obsolete public snapshot retired; actual console tests consume current tagged commands/snapshots. |
| Open could lower observation revision | Installation includes live revision while keeping persisted archive watermark independent; all-three-game Save/Open regression. |
| Replay fact hid stop reserve in usage | Separate measured usage and mandatory close unit, tested at the actual boundary. |
| Post-deadline wrapper could report a committed Save as failed | Successful atomic Saved completions retain success; late noncommitted candidates still refuse. Deterministic Chess/native15001ms tests. |

All material source findings are resolved. The reviewer approved the corrected
working source; exact committed-source CI and actual package evidence remain
required below. Early migration runs149/7,151/6 and154/3 exposed private-access
and asynchronous-contract assumptions plus actual replacement/poll issues;
those were corrected. The retained first failing log is diagnostic history,
not an unresolved result. Frontend's initial242/1 used an obsolete harness-close
expectation; updated exact native token coverage and final243 pass supersede it.

## Committed-source CI and packaged acceptance

Pending the reviewed source commit, applicable three-platform native/Desktop CI
and actual installed/packaged lifecycle evidence. MSH-118 remains In Review
with unchecked criteria until reconciliation. The normal-Xiangqi GUI phase adds
file/history/close coverage to the existing14 phases using real pointer/keyboard
and native dialog input, read-only DOM observation and native record assertions.
It records actual completion versus cancellation at the small native budget;
completion is never called cancellation. No actual GUI success is claimed here.

R6 retains final integrated all-game/all-platform acceptance. No R4, R5, R6,
variant or M5 task starts automatically. Historical MSH-111/112 findings and
M1/M2/M3/profile evidence remain unchanged.
