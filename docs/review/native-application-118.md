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
establishes that at the committed revision in the final results below.

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

Independently approved source `b568ea7103e283155a851df16d3b382b3842eebc`
is integrated and pushed to `codex/rewrite`. Initial Rust41103862958 failed
its Full-byte stress assertion on obsolete diagnostic spacing (`16 MiB` versus
`16MiB`). The local all-feature result177/1 exposes the same assertion. A stronger
native test now asserts authoritative encoded-byte facts, exact unchanged limit,
measurement/scope/units and actual excess usage. Corrected full stress passes
1/1 in162.70s, including the formerly unexecuted complete Save/Open tail.

Initial Desktop41103863940 passed macOS packaging; Linux/Windows each passed
13/15 actual GUI phases. Chess reopened successfully with the new native receipt,
but the verifier waited for the old exact sentence. Xiangqi played successfully,
but its normal Technical Details application `error` field was misclassified as
a WebDriver error. Actual failed DOM/screenshots establish both verifier defects.
The external observer now requires accepted path/unmodified/settled controls plus
a native Open/work-stopped receipt, and wraps read-only Xiangqi diagnostics
without weakening driver error handling.52 external verifier tests pass.
Original [failed reports and captures](evidence/msh118/initial-ci) are retained.
No production behavior or limits changed in these verification corrections.
Transient GitHub jobs-list502/empty steps were observation failures, not passes.
Corrected verification-only integration `6e9b2e20a0315bbe902189376a6c0241dfd810f1`
keeps production code identical to `b568ea7`. Rust41106568111 and
Desktop41106568074 both pass all three target systems. The corrected complete
GUI campaigns below resolve the remaining acceptance gate. The normal-Xiangqi GUI phase adds
file/history/close coverage to the existing14 phases using real pointer/keyboard
and native dialog input, read-only DOM observation and native record assertions.
It records actual completion versus cancellation at the small native budget;
completion is never called cancellation. Its corrected complete CI GUI execution passes on Linux and Windows as recorded below.

Separately, the normal locally built macOS27.0.1 arm64 `.app` at the committed
source passed [actual native interaction](evidence/msh118/macos/interaction.json).
Its host SHA256 is `3a1bf3ead08fcb2ed90115b5498180d24a08ca25351d035d06cba1574b47f1ba`;
bundled engine `57d09b1a22f96a7a9b89a9c6c2550b233ed8213f591580ad8e66106900b7ac3a`.
Package inventory, instrumentation-marker exclusion, UCI, real engine-turn,
read-only analysis and bounded Full/Unknown queries passed. These engine queries
are supporting evidence, separate from actual GUI interaction.

Native AX/pointer/key input and OS Save/Open panels exercised Chess e2e4 and the
real a7a5 reply; cross-game New Xiangqi Red, a3a4 and the real b7b0 reply;
Save, selected event1 analysis1/128, Save/Open with two historical observations,
restored event1, Return to live, explicit stopped Resume, native window-close
Cancel, then Command-Q Save & continue. Saved canonical files retain complete
native events and control history. Actual hostPID15783 and all package host/engine
processes were absent after Quit. AX, screenshots, package hashes and original
saved documents are [indexed](evidence/msh118/macos/artifact-index.json).
This focused local journey is not a new all-game/all-platform GUI campaign;
small-budget analysis completed, so no local cancellation success is claimed.

R6 retains final integrated all-game/all-platform acceptance. No R4, R5, R6,
variant or M5 task starts automatically. Historical MSH-111/112 findings and
M1/M2/M3/profile evidence remain unchanged.


## Acceptance reconciliation

| MSH-118 criterion | Current evidence |
| --- | --- |
| AC1 active native ownership | `ActiveDocument`/`RuntimeMetadata` source deletion and runtime6; exact one-variant cross-game checks, no dormant Chess payload. |
| AC2 bounded admission and file lifecycle | Worker-native Chess/Shogi/Xiangqi mutations/replay; receipt/rollback/stale/capacity tests; host reservations; deterministic budget and committed-Save tests. |
| AC3 atomic installation/fresh authority | Single `install_document`; all-three-game Save/Open/runtime identity regression, exact stale/history/Take back tests; actual macOS stopped reopen/Resume. |
| AC4 complete current native contract | Runtime actual Chess claims/MatchRecord/observations; unchanged independent rules/search fixtures; native claim/Full/history/preferences/inspection/outcome/Take back suites; actual console subprocess12; frontend243/host9. |
| AC5 deleted old ownership and bridge | Optional overlays/Chess metadata ownership and three-install bridge deleted; only named R4 frontend forwarder remains, with no old-file reader/session. |
| AC6 native close and instrumentation | Native ApplicationLifecycle/host9, unchanged Tao patch, real normal macOS window-close Cancel and Cmd-Q Save & continue, process absence and marker exclusion. |
| AC7 semantic and packaged integration | Cross-game native tests and actual identified-platform macOS Chess/Xiangqi file/history/close journeys, independently audited saved payloads and screenshots. Final R6 platform campaign remains separate. |
| AC8 review/CI/integration | Independent source approval at b568ea7; verifier correction review; actual artifact audit. Corrected native/Desktop CI both pass all three systems at6e9b2e2; final independent source, package and acceptance audits approve all criteria. |
| AC9 typed resources | Resource bounded serialization/category tests, actual zero-observation Shogi/Xiangqi controls; actual combined ordinary/exact256→257; byte/event/replay/identity/reserve checks; stronger full16MiB stress Save/Open passed. R4 owns shared frontend fact consumption. |

The independent reviewer reran52 external tests after the corrections and
confirmed the original failure indexes. Final acceptance review supports all
nine criteria and the applicable completion checklist. No material finding remains.

## Final verdict and platform provenance

**MSH-118/R3 accepted.** Production source is
`b568ea7103e283155a851df16d3b382b3842eebc`; tested verification integration is
`6e9b2e20a0315bbe902189376a6c0241dfd810f1`, with no production changes between
them. The corrected macOS CI host and engine binaries are byte-identical to the
initial CI package. The final completion commit changes documentation/evidence
and task bookkeeping only.

[Rust41106568111](https://github.com/jweathers777/mShogi/actions/runs/41106568111)
and [Desktop41106568074](https://github.com/jweathers777/mShogi/actions/runs/41106568074)
pass Windows2022, macOS15 and Ubuntu24.04. Native166 default and178 all-feature
checks pass on each system, including the real Full byte-capacity Save/Open
stress and golden fixture comparison. Workspace formatting/Clippy/build/console
checks pass. Desktop frontend243/243 and host6 default tests pass on each;
local host9 includes verification-feature coverage. TypeScript, feature check,
normal package smoke/marker exclusion and separate verification packaging pass.
External GUI verifier52/52 passes on Linux (also independently rerun locally).
Exact [native](evidence/msh118/native-ci-final.json) and
[desktop](evidence/msh118/desktop-ci-final.json) job metadata and selected original
result lines are retained. CI annotations concern the upstream upload action's
Node20 deprecation and runner capacity; neither is a failed product check.

| Actual application evidence | Identity and established behavior |
| --- | --- |
| Local normal macOS27.0.1 arm64 | Sourceb568ea7, host3a1bf3ea…/engine57d09b1a…; actual AX/OS-dialog Chess/Xiangqi file/history/Resume/window-close/Save-and-Quit, separately indexed above. |
| Linux normal installed + separate QA packages | Source6e9b2e2, artifact13001911957; normal host3cc9e3b0…/engine1e22e132…; [exact manifests/report/captures](evidence/msh118/linux/artifact-index.json).15/15 actual GUI phases pass. |
| Windows normal installed + separate QA installers | Source6e9b2e2; [exact artifact/installer/report identities](evidence/msh118/windows/artifact-index.json), normal host0fcd149f…/engine417ba5ff…;15/15 actual GUI phases pass. |
| macOS15 CI | [Normal package identity/smoke](evidence/msh118/macos-ci/artifact-index.json); build/engine evidence, separate from the local actual GUI journey. |

The fresh read-only reviewer audited all33 retained Linux originals, all294 GUI
report files and13 normal manifest entries, plus all8 retained Windows originals,
all255 GUI report files and2 installer manifest entries. Both verification-package
digests and normal-host instrumentation exclusions match. All15 native exit proofs
per platform show host absent and empty immediate/final process matches before
driver teardown. Recorded Linux bounded-proxy provenance and signed matching
Windows WebView2/driver131.0.2903.86 remain explicit; no new driver fix is claimed.

Both new Xiangqi phases actually dispatch cancellation and observe `Search stopped`.
They prove native play, bounded analysis after cancellation, fresh document/session/
selection authority on stopped reopen, saved observations historical, exact event1
analysis while live history stays2, Return to live/Resume, corrupt Open preserving
the complete accepted document, native window-close Cancel and Quit. Native rules
fixtures remain the semantic oracle; these are real consumer/lifecycle journeys.

The [independent review record](evidence/msh118/independent-review.json) identifies
the actual separate reviewer method and resolved findings. Original failed probes
remain historical evidence. No new rules interpretation, product limit increase,
frontend redesign, variant delivery, M5 result or MSH-111/112 resolution is implied.
R4 deletes the finite frontend forwarder and consumes native resource facts;
R5 owns board interaction consolidation and R6 final integrated three-game/
three-system acceptance after those stages. Stop at MSH-118; MSH-119 is next.
