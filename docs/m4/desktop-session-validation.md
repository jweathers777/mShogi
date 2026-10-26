# MSH-72 verification and review

Scope: the source integration introducing this record, based on
`eb9cc7ef43500c9d32a5bc99ebbc9c6c2978ff60`, on `codex/rewrite`.
MSH-72's durable completion record pins the final revisions and current
three-platform CI URLs. This boundary does not close MSH-73/74/75, MSH-41–47 or M4.

## Execution evidence

Local environment: macOS27.0.1 arm64, Rust1.89.0 workspace, Rust1.90.0 Tauri,
Node24.19.0. Used the bundled Node runtime because the unrelated Homebrew
Node26 installation has a missing simdjson dylib. Lockfiles and selected shogi
profile/rules/fixture expectations remain unchanged.

| Layer / actual command | Result and scope |
| --- | --- |
| `cargo test -p mshogi-desktop-session shared::tests` | 11 passed. Identity/labels/orientation, native moves/drops/capture demotion/promotion, declarations vs advice, terminal same-ply history, exact contexts and wrong identities, history prefixes, semantic undo/replay retirement, file/admission retirement, capacity, strict parsing/UTF-8 diagnostics, close preconditions, incomplete/failure/retry. |
| `cargo test -p mshogi-console --test desktop_shared` | 2 passed through the actual `mshogi internal-search` executable. Chess→shogi→chess on one supervisor with increasing IDs; native move and declaration, analysis read-only, historical prefix. |
| `cargo test --workspace --locked` | 366 tests/doc-tests passed, including existing chess/core/search/protocol/process/console/runner/session/archive/capacity regressions. |
| `cargo test --locked -p mshogi-desktop-session --all-features` | 57 passed, including separate QA constructors and MSH-61 limits. |
| Workspace fmt, all-target clippy; session all-feature clippy | Passed with locked dependencies and `-D warnings`. |
| Tauri fmt, all-target/all-feature clippy; all-feature tests | Passed; 5 host tests. Existing Tao patch/provenance unchanged; dependency warnings are distinct from product lint results. |
| `npm run check`; `npm test` | Strict TS passed; 83 tests in14 files passed. Native-generated board/hand/promotion choices; click/keyboard/focus/cancel/orientation; stale frontend snapshot/replacement protection; all existing chess and synthetic route regressions. |
| `cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- desktop/src/__fixtures__` | Native standard/drop/optional/forced promotion snapshots generated; existing chess fixtures unchanged. CI regenerates and checks committed bytes. |
| `npm run build`; `npm run build:verification` | Passed; normal frontend excludes QA modules/markers. Normal package smoke also rejects native contract endpoint markers in the host. |
| Tauri verification `.app` build | Passed, using the real bundled engine and separate verification identity/configuration. |

Expected rule facts reuse MSH-63–71: initial30 choices; optional5d5c/5d5c+;
compulsory5b5a+; capture demotes +P to hand P; P*5e consumes one pawn;
valid CSA28 declaration and wrong-turn false-declaration loss. `2RBGSNLP`
means two rooks and **one pawn**, so dropping that pawn leaves zero. These are
native adapter tests, not another UI rules implementation or new rules research.

## Actual macOS development GUI

Primary builder used native macOS accessibility/UI automation on the packaged
**mShogi Verification** `.app`, with
`MSHOGI_DESKTOP_FIXTURE=shared-contract`. The actual screen displayed
**MSH-72 DEVELOPMENT CONTRACT**, native Sente/Gote hands and standard shogi
board. Resume enabled choices; selecting7g then7f produced `Sente: 7g7f`
and the actual bundled-engine reply `Gote: 4c4d`. Bounded analysis retained
the two-event game, exposing real v3 `move_analysis` context (event2, board_ply2,
Sente, exact runtime IDs). Root selection showed read-only event0 and retained
the two-event live timeline. Return to live restored the live position paused.
Native Command-Q displayed explicit unsaved development-session discard/keep-open;
explicit discard closed the disposable session through native cleanup.

The first interaction exposed two clarity/layout issues, corrected before final
integration: Resume now reports Idle on a human turn, and standard9x9 uses the
normal board panel while16x16 keeps its scrolling behavior. Native/frontend
checks reran after those fixes. This narrow development journey is not final
shogi GUI acceptance, shogi archive evidence, accessibility certification or
Windows/Linux shogi GUI evidence.

## Independent review

Separate agent `msh72_contract_review` traced the raw Tauri command boundary,
native admission, supervisor/context acceptance, presentation/selection and
chess persistence guards. Actual method: read-only source review and independent
execution, not primary-author self-review. It independently ran all11 final
shared native tests; the2 actual executable tests; and16 NativeBoard/existing
Board/route tests, plus `git diff --check`.

Two material findings were resolved with regressions: shogi changes/observations
and chess replacement must advance close document preconditions; parser
errors must obey the4KiB UTF-8-safe diagnostic bound. Primary strict roundtrip
verification also caught serde internally tagged enum buffering, fixed by
RawValue-first replacement dispatch. Final independent review found no remaining
material boundary issue. Reviewer did not perform GUI interaction/platform CI.

## Platform evidence and remaining ownership

The task completion record records exact final-source Rust and desktop workflow
runs on Windows/macOS/Linux. Package construction, bundled-engine queries and
existing chess GUI smoke establish only their named checks. Prior unchanged
M2/M3 packaged interaction evidence and native macOS termination regression
coverage remain applicable; they are not relabeled shogi application evidence.

MSH-73 next owns complete typed desktop archive semantics/compatibility.
MSH-74 owns the polished shogi application and packaged GUI acceptance;
MSH-75 owns final shogi integration. Normal application launch remains chess;
the development fixture has no shogi Save/Open and clearly advertises that limit.
See [the API/launch contract](desktop-session.md).

## Explicit limits of visual evidence (owner clarification)

The displayed development harness is **not evidence of completed or approved
shogi visual design**. Scrolling/layout/sizing, oversized Latin fallbacks,
diagnostic hand buttons and raw-state/control-strip presentation are not the
MSH-74 design basis or acceptance target. The actual product requirements and
non-prescriptive semantic API are recorded in
[the shared contract](desktop-session.md#visual-design-boundary-owner-clarification-2026-10-26)
and [the M3/M4 design clarification](../desktop/m3-design.md).
MSH-74 owns full-board visibility, deliberate shogi pieces/ownership/promotion,
integrated hands/drop, hierarchy, uncrowded history/analysis, polished keyboard
and pointer use, and separation of diagnostics. This evidence establishes only
MSH-72 semantics and its development interaction path.

Final reviewed local QA package: host SHA-256
`6b6404be4fdcea3ea4d2a5a11f0e281376cfb2fe24b21b264fa7a81fbd29ee0b`,
bundled engine SHA-256
`d36e48dc8d6175ab3c9fc073e4de1e119f6850fd7feef5e6bc4bacc3c8830826`.
The final package rerun confirmed Resume→Idle and the native7g7f/4c4d path;
owner/tone remains orientation-independent in the final frontend tests and
independent review. Package hashes identify this local development build only.
