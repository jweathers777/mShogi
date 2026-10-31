# MSH-121 native independent acceptance

Reviewer: bounded independent child `/root/r6_native_audit`, read-only source/test session. Reviewed current source at `13510da9e24b69a340444f272668ecb59478ddbb` on primary `codex/rewrite`; only pre-existing/root-owned task121 edits were present. No repository/Backlog/production mutations, no merge, no GUI interaction and no platform-success inference. This is independently executed source/semantic verification, not a restatement of previous task completion reports.

Environment: macOS27.0.1 build26A434, arm64, Rust1.99.1(58cf8f9c3). Read owner attachment, AGENTS, task121, design, architecture target115/migration plan/post-R4 UX. Pinned Backlog1.55.1 overview read; no task mutations.

## Architecture/source disposition

PASS for the bounded native architecture and current semantic boundaries; final overall R6 acceptance still requires root's frontend, packaged GUI, platform, CI and final-source evidence. No material native production defect identified by this review.

- `crates/mshogi-desktop-session/src/lib.rs:281,303,337,1100` establishes common `RuntimeMetadata`, private native Chess session, exactly one `ActiveDocument::{Chess,Shogi,Xiangqi}`, and one `DesktopController`. Shogi/Xiangqi variants carry the same metadata type directly; there is no dormant Chess document behind them. Enum-native state specialization is intentional.
- Host `desktop/src-tauri/src/main.rs:398,696,704,708`: one managed `Mutex<DesktopController>`, shared lifecycle handle, one registered game IPC family. `with_controller` checks shutdown before/after locking; handlers reserve native operations before off-thread waiters. No obsolete desktop-only Chess handler is registered.
- `shared.rs:583,614,694,941,1045`: current flat tagged native commands, full `SessionToken` document/version/revision/selection precondition, actual game check and native command admission. Private `ChessMutationCommand` is journal/domain state, not a competing transport.
- `mshogi-engine/src/shared.rs:80,195,281,323`: one context/observation/request/response family; protocol4 is distinct from durable observation schema1. Request purpose, canonical history, limits, request/session/process identities and selection are compared, not board-only equality. `server.rs:16` accepts only the current strict request; native `execute_chess/shogi/xiangqi` retain different meanings and reject unsupported ChessFull on other games.
- `mshogi-engine/src/bridge.rs:81,120,153`: one supervisor, one replaceable pending slot and result, fresh increasing request/generation, cancellation clearing published results, bounded process/pipe cleanup. Separate file/admission workers are bounded operation classes within the same controller, not competing game owners.
- `document.rs:13,23,65`: one current `mshogi.desktop.document.v1` outer envelope, explicit game/profile/edition/schema, RawValue payload, no intermediate map. `mshogi-core/src/native.rs:187,253,264` keeps nested native bytes and rejects identity/schema disagreement. Native payload codecs remain appropriate specializations.
- `persistence.rs:1651` is the one current validated-document installation transaction. It computes fresh runtime identities above current/runtime/saved floors, builds a native-tagged candidate, retires admission/bridge, then swaps once. Reopened observations become historical and work remains stopped; no saved pending request is revived.
- `shared_archive.rs:1286,1364` and Xiangqi admission completion preserve token/epoch checking and caller receipts. Native replay/check-capacity precedes candidate install. `persistence.rs:1092` uses exclusive same-directory temp, chunk cancellation, flush/sync and atomic rename; failed writes remove temp and retain the old save.
- Native close ownership is `lifecycle.rs`; host `main.rs:429` routes native close to it. Native `application_pause_for_close` records an exact post-stop token, preserving canonical dirty state; never-saved `path=None` requires Save even when dirty=false. Save receipt cleanliness is exact-revision only. Approval requires exact current token and no outstanding operation reservation.

## Negative reference / caller audit

Read the actual source and searched production + examples + tests + QA/builders rather than treating old-name absence alone as proof.

| Replaced mechanism | Current result / actual survivor classification |
| --- | --- |
| WireRequest::Legacy, WireResponse::Legacy, legacy fallback, old V2 request/response, chess_adapter | No executable declarations/callers in crates/desktop production, examples, tools or workflows. One ExecutionRequest/ExecutionResponse exported by engine lib; unknown transport explicitly fails. |
| Paired internal submit_shared/poll_shared/submit_chess families | Absent. EngineBridge has singular submit/poll. Private poll_chess is native state dispatch, not an alternate wire. |
| Old Archive/ArchiveV2/desktop1-2-3 readers, decode_chess_v1/decode_chess_compatible | No executable readers/writer selection found. Current document parser rejects old formats. ArchivedObservation is the current durable Chess domain observation, not an old archive reader. |
| Ever-present Chess global owner with optional native overlays | Replaced by ActiveDocument; private native Chess fields remain only in Chess variant. |
| R2 capture/install and R3 temporary frontend forwarder | No current cross-stage bridge found. Direct native envelope encoding plus sole install_document; current game handlers invoke shared_command directly. |
| Legacy desktop_snapshot/command/save/save_as/open handlers | Absent from host registration and builders/callers. Current game_file internal save_as writer remains intentionally authorized, without ordinary Save As UI. |
| Development shared-contract surface as normal product | QA fixture initializer/verification mode/synthetic handlers all cfg desktop-verification. Normal initial_controller constructs normal DesktopController; feature-gated initializers are not normal exports. |
| Examples | write_frontend_fixtures/write_document_fixtures use current commands/current documents. xiangqi_session_probe is explicitly labeled development-only and requires desktop-verification; does not create a competing product controller. |
| Tests | Old transport cases are explicit rejection checks. ShogiApp/XiangqiApp occurrences are test-local aliases of the same ApplicationView import, not surviving application implementations. |
| Packaging / helper binary | Desktop workflow builds normal first, smoke-checks it, then separately labeled desktop-verification. Cargo console helper explicitly separate test fixture; desktop packaging selects mshogi, not mshogi-internal-fixture. Actual binary exclusion still needs root's package smoke evidence. |

Search scope: `crates`, `desktop/src`, `desktop/src-tauri/src`, `tools`, `.github/workflows`, plus explicit examples, manifests and host build.rs; excluded vendored Tao implementation and generated fixture data for obsolete-token search. Historical docs/task reports intentionally retain prior names and evidence. No claim to erase historical references.

## Independently executed checks

All run against source13510da; stdout/stderr logs retained under /private/tmp for root to preserve durably.

1. `cargo test --locked -p mshogi-engine --test execution_contract --test shared --test xiangqi_routing --test xiangqi_handoff` — PASS22:3+6+7+6. Log `/private/tmp/msh121-native-engine-independent.log`.
2. `cargo test --locked -p mshogi-core --test special_actions --test game_history --test shogi_legal --test shogi_history --test xiangqi_history --test xiangqi_exact` — PASS45:6+5+7+5+16+6. Log `/private/tmp/msh121-native-rules-independent.log`.
3. `cargo test --locked -p mshogi-console --test internal --test shared_internal --test xiangqi_internal` — PASS15:5+3+7. These actually launch the current native child/supervisor and bounded failure helper, rather than replacing process execution with wrapper equality. Log `/private/tmp/msh121-native-process-independent.log`.
4. `cargo test --locked -p mshogi-desktop-session --features desktop-verification --lib` — running212 checks at report creation; final result appended below. Log `/private/tmp/msh121-native-session-independent.log`.

## Semantic challenges and concrete independently expected facts

- Chess special-action corpus asserts complete castle/en-passant/four-promotion legal sets and exact transition/rejection/undo, beyond agreement between wrappers. Native game-history tests compare same diagram/different occurrence tables. Engine shared test independently expects Qg7 mate from `7k/8/5KQ1/8/8/8/8/8 w - - 0 1`, fifty-move intended claim from counter99 without playing it, zero-node incomplete score=None, bare-kings Full impossible and initial position zero-budget Full Unknown. Unsupported Full-mode live restoration is rejected explicitly.
- Shogi LegalAll independently reviewed fixtures cover both sides, optional/compulsory promotion, drops/nifu/dead ranks/pawn-drop mate defenses, exact apply/undo and material. History fixture assertions distinguish declaration advice/formal declaration, resignation actor semantics, imported history and same-ply terminal prefixes. No cshogi rebuild/differential campaign was repeated; those historical independent expectations are reused as fixed oracles.
- Xiangqi independently reviewed WXF cycle and natural-count corpora assert warning events separate from board plies, warning retirement, claimant-specific checks,100/102 ordering, fourth occurrence and mutual-check equal fault. Exact tests include a separately enumerated palace universe; a natural stalemate witness is existential, not a forced win or repetition penalty. Handoff asserts 255-state closed graph negative evidence, a two-action Red stalemate witness and explicit Unknown/Cancelled/Failure while canonical game remains unchanged.
- Current engine strict tests mutate duplicate/unknown/missing identity/purpose/target/native fields; full history/request/session/process/limits/selection mismatch refuses even when board placement matches. Actual child tests verify cross-game cancellation/replacement, semantic undo/reopen, zero-work unscored completion, timeout and cleanup.
- Current session tests inspected in detail include real native transcripts under the current envelope, duplicate-native-schema rejection after raw dispatch, exact warning prefixes (including an intra-command Xiangqi prefix without a fabricated canonical game), all-three-game fresh stopped reopen, abandoned Take back lineage, engine-first/pending/completed replies, terminal-only undo, and stale inspection/result rejection.
- Capacity tests inspected before execution exercise actual256 retained observations and proposed257 rejection with complete Save/Open; Xiangqi combined ordinary/exact accounting; true zero-observation control-only refusal; weighted replay and exact-witness cost; actual encoded envelope/native-wrapper overhead; identity and close/stop headroom. This is not a constant-increase or truncation solution.
- Workflow tests inspect older stopped state versus newer historical analysis cancellation/failure, engine-browse fresh context, reopened browse never implicitly resumes, Take back/capacity stop precedence, concluded Return preserving unresolved Chess meaning, exact clean-close exemption and untouched never-saved Chess protection.

## Findings / limits

No material native source finding from this boundary review. A documentation-only consistency item: the top “Current scheduling refinement” of `docs/architecture-migration-plan.md` still calls123–125 unstarted and121 prerequisites unsatisfied; reconcile current handoff while retaining clearly dated historical planning. `document.rs` line2 is likewise an outdated migration-status comment (states bridges end in118); actual current code has the sole install path. These are clarity issues, not evidence of surviving bridges.

No GUI was driven and no Windows/Linux claim is made by this reviewer. No competitive-strength conclusion. No MSH112 runner fix claimed: the selected independent process checks are outside its independently owned runner race; success here does not resolve it. Synthetic compound-action tests establish only labeled development geometry/interaction, not real Chu/Tenjiku rules. Xiangqi exact graph analysis remains native explicit facility/history evidence, not a newly advertised ordinary execution operation or routine-render work. Historical negative exact reports are observations; replay does not convert them to live conclusions or secretly rerun expensive graph analysis.

Root must still reconcile final CI/current revision, normal instrumentation exclusion and all required packaged platform journeys. If source changes after13510da, evaluate the delta before applying this verdict.

## Final executed result

The current desktop-session QA-feature library suite completed successfully: **212 passed,0 failed,0 ignored**,177.40s, exit0. The longest test `real_bounded_witness_observations_reach_byte_capacity_and_remain_saveable` completed and passed; its over60-second progress line was not a failure or timeout. Combined independently executed checks in this review: **294 passed** (45 fixed native-rule/history/exact expectations +22 current engine routing/strict-context/handoff +15 actual subprocess/supervisor +212 desktop native/current-document/runtime/admission/lineage/workflow). These are test counts, not platform GUI phases or external reference-engine execution.

Final native source/semantic disposition at13510da: **PASS**, no unresolved material native production finding. Root's required complete platform/product acceptance remains separate.

## Bounded QA fixture delta review — xiangqi-exact-evidence

Independently reviewed root's uncommitted change limited to `shared_xiangqi_verification.rs` (127 additions at review time). This is a hard-state initializer under the existing `#[cfg(feature="desktop-verification")] mod verification` in shared_xiangqi.rs:12, not new product execution or a new ordinary analysis capability.

Geometry checked independently against the fixed root and existing native conformance fixture:
- Black General f9→e9 is within its palace; the original root has exactly this one legal Black action (fixed exact test at xiangqi_exact.rs:47).
- Red Rook a8→e8 gives a file check to e9, with no obstruction.
- Black General e9×e8 captures the sole Rook within its palace; surviving Generals d0/e8 are on different files. Only Generals therefore satisfy the established sound-fast neither-can-win condition. This cooperative sacrifice is deliberately different from the independently frozen positive witness f9e9,a8f8, which leaves Black no legal move after2 actions. Existence of that natural stalemate continuation is not forced-win evidence.

The fixture runs actual bounded native ExactAnalysis: historical event0 yields Red natural-win witness length2, live bare-Generals yields NeitherCanWin, zero-state historical work yields Unknown/States, cancellation yields Unknown/Cancelled, unsupported state limit yields Failure/UnsupportedLimits. Native Inspect/ReturnLive is admitted before each attachment; retention validates exact selected provenance, replays/checks complete capacity, journals the observation, and asserts no native game-record change. The initial wrong-selection failure was correct rejection, not justification to relax admission. The final ReturnLive leaves the existing native concluded game intact.

Independently executed:
- `cargo test --locked -p mshogi-desktop-session --features desktop-verification exact_evidence_fixture_has_native_historical_meanings_without_changing_the_game` —1PASS,0FAIL,0ignored,0.05s. `/private/tmp/msh121-native-exact-fixture-review.log`. An initial attempt with `--exact` and an unqualified test name selected0 tests; it was corrected and is not counted as verification.
- `cargo check --locked -p mshogi-desktop-session --no-default-features --lib` —PASS; `/private/tmp/msh121-native-exact-normal-check.log`. Source cfg review proves the initializer/test is omitted from normal compilation; actual package exclusion remains root's package smoke responsibility.
- Independent external public-API probe (source only in `/private/tmp/msh121-exact-public-probe`) constructs this QA fixture, does actual current-envelope Save then Open, checks unchanged canonical context, fresh runtime token, stopped work, clean disk receipt, and byte/semantic-equal5 exact reports with targets `[0,4,0,0,0]`. PASS exit0; `/private/tmp/msh121-native-exact-public-probe.log`. It does not bypass native archive admission.

The external probe's first run incorrectly expected3 live native events because it conflated3 board plies with event count. Native correctly supplies4 events (the separate concluded-outcome event is authoritative). That reviewer-probe failure is preserved at `/private/tmp/msh121-native-exact-public-probe-initial-failed.log`; the corrected assertion explicitly checks4 native events and uses that actual exact-prefix identity. No product/native expectation was changed to satisfy the probe.

Delta disposition: **PASS; no material finding**. No archive schema, limits, strict parsing, native rule or ordinary engine behavior was changed. Real normal-package binary exclusion, GUI report presentation and three-platform acceptance remain separately required.
