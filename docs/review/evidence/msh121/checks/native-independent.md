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

## Native warning-prefix probe correction review at176c46a

At root/platform-review request, independently reviewed `desktop/scripts/gui-smoke.py::warning_prefix_transition` and actual current native projection `shared_xiangqi.rs:610` plus handoff test. The prior verifier expectation that canonical context must differ between selected events8/9 was incorrect: event8 is an immutable intra-command prefix of the one board command that generates derived warning9, not a separately committed native game. No production change or relaxed native parsing was warranted.

Independent external public-API probe (separate `/private/tmp/msh121-exact-public-probe/src/bin/warning.rs`) actually constructs the native QA `xiangqi-warning` fixture and submits Inspect8 then Inspect9 through exact shared command/receipt admission. It asserts same diagram and full live canonical9, different selection tokens in the same document, selected_events8 then9 with identical first8 events, event8 `live_with_immutable_selection`/selected_canonical=None/no warnings, and event9 `selected_committed`/selected_canonical=Some(context) with active Red PerpetualCheck. **PASS exit0**, log `/private/tmp/msh121-native-warning-public-probe.log`.

Also independently reran `cargo test --locked -p mshogi-desktop-session --features desktop-verification warning_inspection_archive_reopen_and_resume_preserve_one_native_context` — **1PASS,0FAIL,0ignored**,0.27s; log `/private/tmp/msh121-native-warning-prefix-review.log`. This existing semantic test additionally verifies exact event8 persistence across Save/reopen, no invented canonical prefix, stale inspection/result retirement, ReturnLive restoring warning9 and explicit Resume.

Disposition: corrected GUI predicate matches authoritative native semantics and strengthens exact event/history provenance. It must not require unequal canonical full context or infer that8 native events are an independent committed game. No material production finding.

## Pending Take back GUI predicate review at176c46a

Independently source-reviewed corrected `retained_pending_takeback` and pending GUI journey. Chess Match Start is an authoritative match event, while its underlying canonical GameRecord has no board event at that boundary. An external public-API transcript actually submitted e2e4 then TakeBack: initial Match event1/0board plies/0GameRecord events → e2e4 Match event2/1board ply/1GameRecord event → TakeBack Match event1/0board plies/0GameRecord events with canonical context equal to the root and play stopped. A GUI assertion requiring event0 conflates two distinct native representations. This transcript verifies boundaries; it does not claim to reproduce the platform's gated pending child itself.

For Shogi/Xiangqi the private native archive journals preserve abandoned actions through initial record plus ordered complete play/TakeBack controls, not Chess's separate MatchRecord-based `takebacks` evidence array. Source replay (`shared_archive.rs` native Shogi TakeBack/decode and `shared_xiangqi_archive.rs` native TakeBack/decode) applies the retained earlier command then native reconstruction; absence of a Chess-only field is not lost lineage.

Independently extracted actual Linux176 artifacts from `/private/tmp/msh121-linux-176.zip`:
- pending-shogi-complete.mshogi SHA256 `d0bce673d6af3ba5938e8ae0b84dd31e2645f90506ada0013b12b5b68baf5b0c`.
- pending-xiangqi-complete.mshogi SHA256 `8f36bb409ec0e7aefbe05cb434211028596de10cd1c2b8df8e7f7a485860dc64`.

The external public-API probe actually Opened each through authoritative current-envelope replay, checked fresh runtime/document token, root canonical history/0board plies, clean disk state, stopped work and no execution purpose, then Saved again. Follow-up structural comparison confirms native initial/current/settings/initial_settings/observations unchanged and all10 controls retained exactly, including play sequence2 (`7g7f`/`a3a4`) before TakeBack sequence6. Neither native format has a `takebacks` field. **PASS exit0**, `/private/tmp/msh121-native-lineage-public-probe.log` and `/private/tmp/msh121-native-lineage-artifact-review.log`; external source is `/private/tmp/msh121-exact-public-probe/src/bin/lineage.rs`.

Independently reran `cargo test --locked -p mshogi-desktop-session --features desktop-verification takeback_native_shogi_and_xiangqi_pairs_pending_repeated_and_fresh_contexts`: **1PASS,0FAIL,0ignored**,3.54s; `/private/tmp/msh121-native-pending-takeback-review.log`. Existing native test uses admitted pending context/actual result, rejects retired replies, covers repeated human decisions/completed pairs and current Save/reopen.

Disposition: both corrected external expectations respect native event and archive semantics. Adding actual Load of the saved lineage strengthens the GUI journey. No product change, common archive schema invention or semantic weakening is needed. These artifact/native checks do not retroactively mark failed GUI phases passed; the corrected current packaged campaign must actually execute and pass.

## Final current-source native acceptance — d725f255339b639cbc7b50bbd95bbbf3344dc459

Fresh final source/caller audit completed at `d725f255339b639cbc7b50bbd95bbbf3344dc459`. Relative to13510da, all normal rules/search/engine/console/session lifecycle/current-envelope/admission/Take back/inspection code and frontend production files are byte-unchanged. Native differences are exactly the127-line separately compiled QA exact-evidence fixture/test and5-line host QA missing-engine fixture filename correction adding platform EXE_SUFFIX. The latter is inside the existing desktop-verification-only initial_controller (host main.rs:487); the normal initial_controller and handler ownership remain unchanged. Since176c46a, no native or frontend production source changed. Rechecked obsolete-name/caller search across crates/frontend/host/scripts/builders/workflows; zero matching old request/response families, readers, hooks or retired IPC callers. Log `/private/tmp/msh121-native-negative-final-d725.log` is intentionally empty for the zero-match audit.

Newly executed atd725:
`cargo test --locked -p mshogi-desktop-session --features desktop-verification --lib -- document::tests runtime_tests workflow_tests exact_evidence_fixture_has_native_historical_meanings_without_changing_the_game` — **28PASS,0FAIL,0ignored**,1.33s,185 filtered; `/private/tmp/msh121-native-final-d725.log`. This focused current run rechecks strict RawValue/current-envelope identities, atomic native payload admission, exactly-one-document/fresh runtime/flat native claims and close authority, complete browse/return/stopped precedence, engine stale-result retirement, untouched never-saved protection and the actual5-report native QA exact fixture. No broad repeated rule campaign was invented.

Also repeated normal-feature compile: `cargo check --locked -p mshogi-desktop-session --no-default-features --lib` —PASS; `/private/tmp/msh121-native-normal-final-d725.log`.

Evidence distinction: the294 independently executed checks at13510da remain reused evidence for the byte-unchanged normal native implementation. The28 current checks overlap that corpus; do not report322 distinct tests or imply a new full294 run atd725. Separate delta checks and public probes above were actually run at their recorded intermediate sources. Existing actual subprocess15 and native capacity212 coverage remain valid; the QA fixture is additional current-source evidence rather than a product architecture change.

### Native contribution to final AC1–4 / AC8

| Criterion | Independent native disposition atd725 |
| --- | --- |
| AC1 | PASS: single controller/ActiveDocument/runtime and single current engine/context/document contract traced directly; native semantic specializations preserved. Frontend and full-product acceptance remain other reviewers' evidence. |
| AC2 | PASS native/host/export/example/test/QA/builder caller audit: no retired facades/transports/readers/paired bridges or dormant global Chess owner. Binary normal instrumentation exclusion requires actual package evidence, not this source search. |
| AC3 | PASS native evidence: frozen Chess special/claim/Full fixtures, Shogi LegalAll/declaration/history, WXF warning/claim/exact natural-win/Unknown transcripts, exact prefix and abandoned lineage; scopes and purpose remain native. |
| AC4 | PASS native boundary evidence: full-context identities/stale responses, actual child cancellation/replacement, stopped reopen, exact receipts, atomic write/read admission, capacity256→257/combined ordinary-exact/control/event/replay/byte/headroom and lossless lineage. Actual platform pending/Save/Load/close journeys must be established separately. |
| AC8 | PASS native architecture endpoint: no intermediate capture/install bridge or competing native owner; one validated install_document, private native codecs/journals retained for semantic reasons. No general rules framework or flattened game representation. |

No new native migration/product finding remains from this review. The earlier current scheduling documentation inconsistency has been reconciled in the current migration-plan overlay; dated historical handoffs appropriately remain. No other task or production architecture was changed by the reviewer.

### Known MSH112 recurrence, accurately qualified

Inspected actual176 Linux failure log `/private/tmp/msh121-native176-linux-failure.log:1167–1181`: `process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored` failed at runner.rs172, expected **UnexpectedExit**, observed **MissingResponse** during engine0 handshake, detail **Broken pipe (os error32)**, exit=None, last_output=None. Runner test result9PASS/1FAIL. This matches the independently owned MSH112 early-exit/write/EOF classification race; it is a real failed category assertion, not a GUI failure, false verifier oracle or a newly fixed condition. It remained unscored; that alone does not satisfy the intended distinction.

Confirmed runner production/tests byte-unchanged13510da→d725. MSH112 remains To Do/unassigned/unstarted; no assertion relaxation, timeout increase or fix is claimed. Later Rust workspace CI at `e778135fbc9dc835b88b13ec3d38b9f63c723f4d`, run41234727114, completed success on macOS/Linux/Windows per retained status JSON. That establishes that later execution passed, **not** a fix or deterministic absence of the known race. At final native-review time d725 native/Desktop CI was queued in the retained runs snapshot; root must record its actual eventual result rather than extrapolate frome778. Root owns final acceptance qualification and preservation of the176 failing log.

Final independent native disposition: **PASS for the migrated current native architecture and its unchanged semantics, with known MSH112 explicitly unresolved**. No GUI was personally driven in this native review; macOS root interaction, actual Windows/Linux campaigns, pixel/accessibility review and full R6 completion are separate evidence and are not asserted passed here.

## QA-only external fixture-selector host delta review

Reviewed root's bounded uncommitted host change afterd725: `read_verification_fixture` and `verification_fixture_name`, setup call and focused test. All new reader/selector/setup use is under existing `cfg(feature="desktop-verification")`; normal `initial_controller()` remains unchanged. This selects an existing QA initializer, not a new native request family, game capability or archive path.

The configured `MSHOGI_DESKTOP_FIXTURE_FILE` path is opened read-only, at most129 bytes are read, >128 bytes rejects, invalid UTF8/empty/multiline/non-token content rejects, and only trimmed ASCII alphanumeric/hyphen fixture tokens pass. A configured missing or malformed file propagates through setup error and never falls back to another fixture. Valid but unknown tokens remain rejected by the existing native initializer. The original direct fixture environment variable is used only when FILE is absent. No security approval database, entitlements, user permission state or production archive is modified by the reader.

Independently executed `cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml --features desktop-verification --bin mshogi-desktop external_fixture_selection_is_bounded_and_never_silently_falls_back`: **1PASS,0FAIL,0ignored**,12 filtered; `/private/tmp/msh121-native-selector-review.log`. Existing vendored Tao20 warnings are preserved. Normal package smoke's existing binary marker `MSHOGI_DESKTOP_FIXTURE` also detects the new `_FILE` string, so exclusion assertions are not weakened.

Disposition **PASS for bounded native host delta**. Actual normal binary exclusion still needs current package smoke. This review neither changes nor promises computer-use approval-cache behavior; fixed-identity/rebuild/relaunch behavior must be established through root's actual experiment. No GUI launch or approval/security-state operation was performed by this reviewer. Separate external staging helper review follows when its author supplies the diff.

## Independent macOS fixed-identity staging helper review

Reviewed author `/root/r6_frontend_audit`'s new `desktop/scripts/macos-verification.py` and tests; no reviewer source changes or app launches. Fixed normal/QA/synthetic IDs and paths prevent per-scenario identity proliferation. `prepare` is explicit QA/synthetic-only, copies the existing QA build into a temporary staging directory, writes fixed metadata/external-selector environment, ad-hoc signs once and strict-verifies before replacing the destination. It refuses running targets before staging and again before installation; failed signing leaves the prior bundle untouched, and failed install restores its backup. `inspect` reports actual/expected identity, every signed-bundle file/symlink hash and actual codesign display/strict-verification diagnostics. CLI returns failure for invalid strict signature or unexpected bundle ID. No permission-store/security-policy access, app launch or UI automation exists in the helper. The optional real-engine gate is fixed external QA-only instrumentation, not normal bundled code.

Independent review found a bounded safety inconsistency in the first helper version: `select` would rewrite the external selector while its fixed staged app ran, despite new AGENTS/documentation's stopped-only requirement. Author resolved it by checking the exact native executable before any write and adding a regression that preserves the previous selector and all bundle bytes. Path comparison resolves /var versus /private/var aliases. Fixture validation is a small fixed admitted-name list, atomic sibling-temp/fsync/replace retains the previous complete token on failure, and successful selection does not change signed bytes or re-sign.

Independently executed final `python3 -m unittest discover -s desktop/scripts -p test_macos_verification.py -v`: **11PASS**,0failed,0.105s; `/private/tmp/msh121-macos-helper-independent-final.log`. `python3 -m py_compile desktop/scripts/macos-verification.py desktop/scripts/test_macos_verification.py` passed. Earlier9-test independent pass is retained at `/private/tmp/msh121-macos-helper-independent.log`; it preceded the stopped-selection/identity assertions and is not substituted for the final11. Tests mock signing/process inventory and explicitly forbid app-launch subprocesses; these are helper behavior tests, not actual signature or approval-reuse experiments.

Final helper/native-selector disposition: **PASS; finding resolved**. The actual unchanged relaunch, scenario swap, rebuild/re-sign reuse, current package identities and any platform-controlled prompt are root's required observed experiment. Bundle ID reuse alone cannot prove approval persistence after a changed code signature; no such guarantee is made. The reviewer did not access or modify approval/security stores and performed no GUI launch.

## QA native content/frame measurement title delta

Independently reviewed the bounded host diff after801e1a905c682c408fc293a88b131d873c9c960b: verification_window_title now takes two separately measured native dimensions, and update_verification_window_title queries actual inner_size and outer_size with native scale factor conversion. Resize/DPI events refresh the QA title. Golden tests deliberately provide distinct760×650 content versus760×678 frame and1000×700 content versus1000×728 frame, preventing accidental equality of the two claims. All modified title functions/callers remain desktop-verification-only; normal product composition, sizing, minimum constraints and lifecycle are unchanged.

Independent `cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml --features desktop-verification --bin mshogi-desktop`: **13PASS,0FAIL,0ignored**,0.77s; `/private/tmp/msh121-native-host-frame-review.log`. Independent `cargo clippy --locked --manifest-path desktop/src-tauri/Cargo.toml --features desktop-verification --bin mshogi-desktop -- -D warnings`:PASS,1.21s; `/private/tmp/msh121-native-host-frame-clippy.log`. Existing vendored Tao20 warnings remain visible and are not represented as newly resolved.

Disposition **PASS; no material finding**. This reports native content/frame logical observations, not a proof that a CUA screenshot's pixel rectangle equals either dimension. Root must retain actual native title values, CUA image dimensions/scale and platform/window constraints separately in the fresh captures. No reviewer GUI launch or approval-state change was performed.
