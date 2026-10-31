# MSH-125 independent boundary review

Reviewer: separate `/root/workflow125_review` session; read-only product/task review, no commits or competing implementation. Baseline `07c0be6f0d616b75feeccd2293e9a6cf007c22d1`; reviewed working implementation on `codex/rewrite` 2026-10-31. Final source review at `871a1fa4b6ae1b463aa4c2abd92bb696d5cfb550` PASS; actual package/CI acceptance remains pending.

## Requirements and method

Read owner attachment `4fd0d7ee-9f2e-4873-bece-764bc2314d1f`, full MSH-125 task, AGENTS, post-R4 browse/lifecycle contract, design/migration references. Inspected native current document, archive/replay/admission, runtime stop/browse origin, lifecycle/close reservation, host picker/receipt handling, shared ProductApp/ApplicationView/GameActions/useGameSession, frontend regressions, and external packaged GUI driver/adapters. This is independent source/contract review plus independently executed focused checks; no actual GUI execution claimed by this reviewer.

## Findings

### F01 — newer Chess historical-analysis stop must supersede an older browse stop (RESOLVED at 871a1fa)

`analysis.rs:237-242` and `255-267` clear `browse_running` on failure/incomplete/cancellation but retain a previous `browse_stop`. `lib.rs:692-717` takes that older stop before retiring analysis and restores its phase/diagnostic. Concrete journey: engine fails, player enters history (saving Failed BrowseStop), starts historical analysis, explicitly cancels it, then Return to live. The older Failed phase and engine diagnostic can replace the newer cancellation while analysis evidence is retired. Shogi/Xiangqi Cancel already clear their saved browse stop. The selected policy says intervening newer stop reasons take precedence. Requested focused all-game original failure -> history -> analysis -> cancel/failure -> Return regression and authoritative newest stop preservation. No stale execution or invented game outcome observed, but the recovery reason was stale. The author corrected cancellation/failed/incomplete analysis by clearing the older browse stop and retaining the newer diagnostic. NonChess failure/incomplete paths also clear the older stop. Independently re-read these exact branches; the correction preserves explicit stopped state and no retired work is revived. The author strengthened the regression to reject the old diagnostic substring and positively require Chess cancellation text/paused state, Shogi/Xiangqi Cancelled phase, and the new failure text. Independently executed final strengthened test: 1 PASS, 0.87 seconds. F01 resolved; no remaining material source finding.

## Verified source boundaries

- First Save retains picker/atomic writer; subsequent Save uses the accepted path. Cancelled picker now returns explicit rejected receipt. Post-picker Save/Open checks exact native source token while holding controller serialization before capture/queueing. Delayed receipt cannot authorize close of newer dirty work; file reservations span picker and completion.
- New/Load remove only intentional replacement guard. Existing current native document stays installed until replay/capacity-admitted fresh installation. Chooser cancellation does not submit. Close pending prevents replacement.
- Close guard exemption is native exact-token and only otherwise-clean stop journaling; canonical dirty state is not falsely marked saved. Cancel releases reservation; newer dirty authority invalidates exemption.
- Direct Resign has an immediate synchronous ref fence plus shared operation ownership, passes rendered native context, preserves native actor/adjudication, and leaves claim/declaration/offer confirmations intact. Actual unresolved Chess outcome stays unscored.
- Browse runtime intent is not persisted or an execution context. Running human return restores action permission; engine return creates newer request/session/process identities. Loaded/Take back/cancel/failure/capacity/concluded paths remain stopped. Existing archive schema/current envelope, limits and authoritative native histories remain unchanged.
- Return plus normal journaled Resume is admitted as one candidate; post-mutation capacity checks retain full saved journals/headroom. Native fixture tests exercise reopen and historical observations without fabricated execution.
- Driver uses Save/Load current controls and exact native state comparison for rejected replacement. After actual accepted Save, filesystem evidence copies are explicitly labeled external copies, never claimed as product Save As. Committed Shogi fixture is copied before ordinary Save can modify its path. Quit asserts actual host/engine absence. Driver reports do not establish actual GUI success until run on identified packages.

## Independently executed checks

- `cargo test --locked -p mshogi-desktop-session workflow_tests -- --nocapture`: 8 PASS, 187 filtered, 1.11 seconds, Rust/macOS local native tests. Covers all three games. This ran before F01 correction.
- Node 24 runtime `npm --prefix desktop test -- src/product/ProductApp.test.tsx src/product/GameActions.test.tsx src/product/useGameSession.test.tsx`: 72 PASS / 3 files (6.73 seconds).
- `python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'`: 91 PASS (0.050 seconds), driver/adapters only, not GUI.
- `git diff --check`: PASS.

## Follow-up checks independently executed

- Final-fix `cargo test --locked -p mshogi-desktop-session workflow_tests -- --nocapture`: 10 PASS, 189 filtered, 2.10 seconds. Includes concluded Chess Full analysis finishing, newest historical analysis cancellation/failure and all prior running/stopped journeys.
- `cargo test --locked -p mshogi-desktop-session browse_return_admits -- --nocapture`: Shogi exact two-control-plus-stop-headroom boundary PASS.
- `cargo test --locked -p mshogi-desktop-session browse_return_reserves -- --nocapture`: Xiangqi equivalent boundary PASS. Both inspect candidate refusal before accepted document mutation; no limit raised.
- NativeSessionPanels/GameActions focused frontend: 41 PASS / 2 files, 1.87 seconds. Concluded Chess exposes Finish analysis only with a genuine non-idle analysis, preserving its existing bounded Full retirement path and native conclusion. Capacity copy now accurately refers to Save only.

## Pending final assessment

Final source and native lifecycle/documentation diff re-read at `871a1fa4b6ae1b463aa4c2abd92bb696d5cfb550`: PASS. Current archive envelope/native core/engine/protocol/runner and R5/shared CSS unchanged. Final `git diff --cached --check` PASS. Actual identified packaged GUI evidence, normal instrumentation exclusion and applicable CI remain pending. Do not mark this provisional report as final acceptance or claim new Windows/Linux/macOS GUI evidence from these unit/source checks.

Source disposition: independently reviewed bounded source changes PASS with F01 resolved. Root reported full native 199/frontend319/host12/lint results are author evidence, not newly executed by this reviewer. This report does not mark task Done before required package journeys and CI.

## Native CI follow-up — F02 (2026-10-31, 871a1fa)

Linux native run `41212862703`, completed job `124571306460` failed in **Verify native desktop snapshot fixtures**, after workspace tests, all-features native 211 tests and clippy passed. Actual log preserved `/private/tmp/msh125-native-linux-attempt1.log`, SHA256 `56b7eb50f4e2127163251fa334ddbc400fd3ef58a452b294206b7e14975b9ab2`. This is not the old runner process timing race.

Generator `examples/write_frontend_fixtures.rs:23-40` unconditionally calls Resume for all named Chess positions, including concluded checkmate. Correct new native admission rejects concluded/idle Resume. Panic `.unwrap()` at line240: `There is no paused play or analysis to finish.` Independent local reproduction with `RUST_BACKTRACE=1 cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- /private/tmp/msh125-review-fixtures-unfixed` exited101; backtrace proves caller line35. Log `/private/tmp/msh125-fixture-generator-unfixed.log` retained.

F02 action: adapt verification-only generator to authoritative concluded/paused semantics, regenerate expected native snapshot and validate complete generator output; retain native guard and outcome. This is a material verification gap, not a reason to weaken Resume. Root informed immediately. Final package/CI acceptance pending correction.

### F02 correction independently verified

Author changed only example fixture setup: Resume iff the native snapshot is unconcluded. Native concluded-idle guard retained. Independently ran `cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- /private/tmp/msh125-review-fixtures-corrected`: PASS (0.88 seconds). Compared all **23** generated JSON files byte-for-byte with author-regenerated repository fixtures: all match, none differ. Checkmate snapshot remains genuinely paused, correct lower identity/control count (no invented Resume). Other changes put `close_requires_save` in actual native serialization order and preserve newer cancellation/incomplete diagnostics. Log `/private/tmp/msh125-fixture-generator-corrected.log` retained.

F02 source/example correction PASS; native CI rerun remains required evidence. Production host/frontend/native runtime unchanged by this verification-only generator/fixture correction, so earlier package source identity must remain distinct from later verification revision.

## Partial actual macOS capture review (871a1fa package; not final rebuilt close correction)

Reviewer personally viewed six full application screenshots: `normal-chess-quit-save-discard-cancel.png`, `normal-chess-one-step-return.png`, `normal-chess-loaded-stopped.png`, `normal-chess-resign-unresolved.png`, `normal-shogi-one-step-return.png`, `normal-xiangqi-one-step-return.png`. Shared shell/header/Save/Load and Analysis/GameActions/Moves structure remain coherent; full native board content retained. Save Game dialog concise Save/Discard/Cancel; running return images show live human turn without Resume; loaded Chess exposes explicit Resume; actual Chess resignation shown unresolved/unscored rather than a fabricated loss. Shogi/Xiangqi retain game-native labels and contents. Xiangqi retained malformed-file failure notice is visible and accurately says current game kept; screenshot alone does not prove request freshness, which is covered separately by native checks.

These are reviewed captured pixels, not reviewer-driven CUA interaction. Author's native interaction log is separate. Never-saved initial close correction and fresh final package identity remain pending and must not be inferred from these earlier images.

## Never-saved close and final correction delta review

The actual Linux GUI probe exposed a distinct initial-document case: untouched startup Chess has path=None/dirty=false while visibly UNSAVED. Original close authority checked only dirty, so it could close without the promised first-save safeguard. Reviewer independently confirmed the new native regression failed before helper update (`initial.close_requires_save`).

Corrected native authority and its pre-stop capture now use `document.dirty || document.path.is_none()`. Only a real saved/opened path identical to disk qualifies for the stop-only token exemption. Canonical dirty/revision is not fabricated; initial unchanged document remains dirty=false. Shared Quit Save button uses native `close_requires_save`, preserving enabled first Save and exact token admission instead of a frontend dirty heuristic. Support Diagnostics exposes that same native flag. No archive schema or rules changes.

Independent checks on current correction delta:

- `cargo test --locked -p mshogi-desktop-session untouched_never_saved -- --nocapture`: PASS, 0.04 seconds. Untouched initial native state -> protected close -> Cancel -> real Save path -> clean authority, same canonical context/dirty preserved.
- ProductApp/SupportDialog frontend: **50 PASS / 2 files**, 7.26 seconds, including parameterized Chess/Shogi/Xiangqi first-save guard button and rejected/cancelled Save with no close.
- External GUI driver/adapters: **92 PASS**, 0.060 seconds.
- Native generator to independent temp directory: **23 generated files all byte-identical** to current repository fixtures, PASS (0.83 seconds).
- `git diff --check`: PASS.

Driver correction uses native close authority including pathNone/dirtyfalse, asserts dirty unchanged after cancelled Save, and records only a real native close/Save/Cancel flow. Bounded keyboard Tab reveals the uniquely named Moves control and establishes native WebView focus/visibility before one pointer click; it does not mutate DOM focus/scroll or bypass native admission. Existing focus helper failures remain actual test failures. No material finding in this bounded correction delta.

Disposition: source correction PASS, no unresolved material source finding. Production changed after871a1fa, so identified final normal/QA packages must be rebuilt and the never-saved guard exercised; earlier Mac captures remain historical source871 evidence, not proof of the correction. Native/Desktop CI and actual final package evidence remain pending.

## Final macOS visual/accessibility evidence review at 58a5d7a

Source/package revision: `58a5d7ad95cba1724118a91d8ad4958fb1c287ed`. Environment manifest identifies macOS27.0.1 (26A434), arm64. Reviewer independently checked all **8 recorded file hashes** across final normal and separately labeled verification bundles. Normal host SHA256 `7db2eb4cd6306967ea8ac76a3fa8984e631b9b802c968a3276338dfc2596beb3`; QA host `4cc08e66f7691bff936268069b1e3d8711843790c7e2c7a8c7545da7534dc039`. Shared engine hash `92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7`. Independently reran the complete existing **16-marker normal-host exclusion scan**: PASS. Author package smoke establishes actual bundled UCI/internal play/analysis/bounded Full separately from GUI interaction.

Reviewer personally viewed all **9 final PNGs** ending58a5d7a: normal never-saved Quit guard, native Save picker, cancelled Save retaining guard, successfully saved UNMODIFIED document; QA all-three-game760×650 boards; QA Chess and Xiangqi Native Cocoa Quit guards. Save is visibly enabled for untouched UNSAVED game, initial Cancel has full outward focus outline, picker is actual native sheet, cancellation message truthfully says game remains open, and the guard remains usable. Final saved document uses established UNMODIFIED vocabulary and current path. Minimum Chess/Shogi/Xiangqi captures preserve full boards, game-native geometry/pieces/hands, shared header/actions/sidebar and bottom live status. Dialog controls fit at minimum with complete outward Cancel focus outline and no new clipping. No material visual/accessibility regression observed in this bounded pass.

Read author interaction-log records for real native control input, keyboard game selection, picker cancellation/Save, native Cocoa Quit Cancel and Discard. Read separate PID-anchored process evidence: normal saved clean CmdQ host91838 exited without subsequent app AX lookup; QA native Cocoa Quit Discard host92029 exited with no matched packaged host/engine process. Those process executions are author evidence, not reviewer personal execution. Earlier AX-triggered possible app relaunch remains qualified by the original log; final termination conclusions use anchored checks, not that lookup.

Visual/accessibility disposition: **PASS** for reviewed current-source captures and documented actual journeys. Combined source + independent focused checks have no unresolved material review finding. Applicable final Native/Desktop CI remains pending at time of this report; this does not independently mark the task Done or claim a full Windows/Linux GUI campaign from macOS evidence.

## Bounded acceptance/documentation reconciliation

Reviewed `docs/review/workflows-125.md` against all seven original MSH-125 criteria and four DoD items, current task notes and owner request. Source871 versus corrected58 package/revision provenance is explicit; earlier failed probes and native generator failure remain historical. Current native/host/frontend/driver expectations map to Save atomicity, deliberate replacement, native Quit authority, direct honest Resign, runtime browse intent, exact execution retirement and real packaged journeys. All local evidence links resolve. No criterion is checked or completion claimed while final CI is pending. Later MSH-121/R6 and variant/M4/M5 ownership remain intact.

One documentation precision request sent to author: the normal871 macOS bullet combines all-three-game aggregate journeys with first/subsequent Save and picker-cancellation/malformed-Load cases, which could read as every subcase personally executed in each game. The retained native CUA log supports an aggregate matrix: Chess first/subsequent/cancelled Save and cancelled Load; Shogi Save/Load and stopped return; Xiangqi Save/invalid Load; all3 running history Return and direct Resign. Make aggregate/per-game scope explicit, with final platform driver reports attributed separately. This does not require another product campaign or alter acceptance. Also replace stale task description future/unassigned sentence at CLI finalization while preserving old notes.

Acceptance review disposition: evidence/mapping is proportionate and supports bounded criteria subject to final actual CI disposition/integration recording and the wording clarification. No unresolved product defect identified by this final review. Do not convert pending CI or historical failed probes into passed results, nor infer all-platform GUI from Mac interaction alone.

## Corrected-source native CI race disposition

Actual Linux native job124575870483 at58a5d7a failed the unchanged early-exit runner classification (MissingResponse/Brokenpipe32/exitNone instead of UnexpectedExit). This exact signature matches existing MSH-112, separate from the earlier F02 generator defect and the unrelated Shogi retirement race. Independent source/log diagnosis and local focused1PASS/0.66s are recorded in `/private/tmp/msh125-independent-native-ci-disposition.md`. Preserve original failure; normal identical-source failed-job retry is appropriate and its actual outcome remains pending. No source/assertion/timeout fix claimed.

## F03 verification-only Chess Resign observer correction

Actual corrected-source Linux GUI reached direct Chess Resign and failed a verifier assumption that every terminal command changes the canonical board-game record. Reviewed frozen `gui-smoke.py`/`test_gui_smoke.py` delta and new actual native diagnostic fixture `desktop/scripts/fixtures/msh125-chess-resignation-actual.json` (source58a5d7a/run41214429648). Existing Chess MatchRecord owns resignation independently of board moves. The corrected observer requires exactly one appended native match event, unchanged full prior match prefix/root/profile/schema/time control, correct White actor and exact preceding match version, monotonically ordered native time, next accepted reply version, no rejection/active turn, actual expected unresolved resignation conclusion and equal live/selected conclusion. It also checks same document and advancing runtime version, and preserves complete board context and Moves data. Visible result must remain Unresolved — unscored with White resigned/no score, rather than invented Black victory. Shogi/Xiangqi native record terminal-event assertions are unchanged.

Negative fixture checks reject wrong actor, stale command version, rejection, active turn, duplicate event, invented board move, wrong conclusion, replaced document and non-advancing runtime version. The current normal Chess path explicitly chooses White, so this bounded expected actor is native test setup, not a product-side inference. No product code, parser, record schema or game rule changed.

Independently ran all external GUI/adapter tests: **94 PASS**, 0.052 seconds; Python byte-compilation and diff check PASS. Correction review PASS, no material finding. This repairs the observer, not the already valid native terminal behavior. Original failed GUI phase must remain failed in history; new actual corrected-verifier execution remains required evidence and is not supplied merely by these tests.

Final corrected-production native41214429663 attempt2 independently verified all3 SUCCESS; original Linux MSH-112 failure remains retained, not claimed fixed. Passing retry actual runner/native200/QA212/fixture/build/console recorded in separate native CI disposition and metadata. Final verification-only8de native41215847540 pending separately; production/native diff58→8de empty.

## Final Linux actual package evidence, verifier checkpoint8de41e4

Independently inspected `/private/tmp/msh125-final-linux` report/audit/manifests/ZIP. Exact source `8de41e46c892613c0a44d0d3991ffda0f88e8b5e`, Desktop run41215847392. Environment Linux7.0.0-1017-azure x86_64/glibc2.39, Python3.12.3, runner image20261026.354.1, GDKx11 and WebKit compositing disabled. Normal package actually installed with apt; QA separately extracted and labeled. Actual W3C pointer/keyboard and native OS picker/close execution, read-only DOM observations, no direct IPC/injected events.

Independently verified ZIP SHA256 `c509e3864e80150054a0324e157dcfca418de4d59d6e9d2b85cfdd8b3a9cf349`, all469 report-referenced file hashes, all13 normal package manifest hashes and QA package hash, all23 retained screenshot hashes; recomputed all6 complete outward focus outlines from actual overflow ancestors and viewport (3games×2sizes): PASS. Normal host `3f82d186435047c06f9f43b9b8a6cca47e91f605398339805b8062063dfd9ef1`, QA host `c8d7eb3cb4c91151dac74b915073b407e21b679c9b96a09fe2740ba5f445a187`, bundled engine `325157b4d14f21cc39b2f1663ef7eff60f3c79d7b4981a9cefb30c36052ff559`.

Personally viewed12 Linux screenshots: all3 cancelled-Save guards, all3 direct resignation results, Chess Take back stopped, Shogi/Xiangqi reopened stopped, and all3 minimum760×650 keyboard-history views. Dialogs concise and usable, Save/Discard/Cancel retained, native Chess unresolved/unscored distinct from Shogi/Xiangqi wins, stopped Resume visible, shared controls/game-native full boards preserved, history focus outline fully visible. No material visual/accessibility finding. These are personal pixel inspections of independently hashed actual captures, not personal GUI input execution.

Report all15 actual normal/QA phases passed, with explicit native New/Load cancellation/malformed rejection, first/subsequent Save, close Save/Discard/Cancel/clean shutdown, complete Take back records/stopped restoration/explicit Resume, all-game exact-prefix history and one-step Return, direct native resignation, promotion/claims/declaration/capacity/analysis journeys. Passed result follows the strict MatchRecord observer correction; earlier failed observer remains historical, not relabeled green.

Final native8de run41215847540 independently checked all3 PASS attempt1; corrected58 native attempt2 PASS preserved separately. Final Windows/Desktop overall completion still pending at this review checkpoint.

## Final Windows packaged interaction/pixel/provenance review

Final verifier checkpoint `8de41e46c892613c0a44d0d3991ffda0f88e8b5e`, Desktop41215847392, Windows2022Server10.0.20348, Python3.12.10, runner image20261026.347.1. Report contains all15 actual normal/QA phases PASS; normal NSIS installed product and separate verification installer remain distinct. Reviewer personally viewed12 relevant Windows captures: all3 cancelled-Save guards, all3 native resignation results, Chess Take back stopped, Shogi/Xiangqi reopened stopped, all3 minimum keyboard-history views. Save Game remains concise and actionable, cancelled Save keeps game open, native unresolved Chess differs honestly from Shogi/Xiangqi scored outcomes, loaded/Take back Resume remains explicit, full native boards and outward focus cues remain visible. No material bounded visual/accessibility finding.

Independently verified artifact ZIP SHA256 `d29f926d16e7cfc304befc829a6b253a6f9376d7331536d907d6eeef1e960bec`, all422 report file hashes (native Windows separators normalized only to locate original ZIP entries), both normal package manifest hashes, verification installer hash, all23 retained PNG hashes and all6 complete outline recomputations (3games×2sizes). Normal host `df3ba9ebbefd9651a3c566d98126d245d138ddbf01577ffdfcd5217e3cc2c7bc`, engine `3f90e562dce8aee762b8bd8a17212c4e9632c9b26ed5312e1f999a521f992592`. Actual driver execution is CI/host evidence; reviewer personally inspected pixels/provenance, not input execution.

## Final acceptance disposition

Read retained final metadata: Native41215847540 and Desktop41215847392 both **attempt1 ALL THREE PLATFORMS SUCCESS** at exact8de41e46c892613c0a44d0d3991ffda0f88e8b5e. Linux/Windows each provide all15 real packaged GUI phases separately from native and package-build checks; macOS provides identified local CUA interaction plus native termination/process evidence and CI package checks. Earlier58 Linux MSH-112 failure/unchanged-source successful retry and all original probe failures remain accurately retained, not fixed or relabeled.

All seven original MSH-125 criteria now have supporting bounded source/test/package evidence:

1. Save/Load controls and first/subsequent/cancelled/failed/interrupted atomic writer behavior, full archive/saveability retained.
2. Deliberate New/Load only replace after authoritative validation/admission; cancellation/rejection/stale source preserves current document.
3. Never-saved/changed Quit guard, exact clean-stop exemption, Save/Discard/Cancel, current receipt/reservations/native shutdown preserved.
4. Direct single native Resign under exact rendered context/actor with honest native conclusion; other confirmations retained.
5. Runtime-only browse origin, newer stops and exact historical retirement, one-step running Return/fresh engine identity, explicit loaded/Take back/failure recovery.
6. Proportionate independent native/frontend/host/driver regressions across all3, authoritative native replay/history and no frontend rules.
7. Actual identified all3-game packages/journeys, separate QA/normal instrumentation exclusion, independent source/pixel/provenance review and applicable final CI supported.

The aggregate Mac evidence wording was clarified as requested. No remaining material review finding or acceptance evidence gap. Recommend completing MSH-125 after root records final task/docs and actual normal integration/push (DoD4); reviewer does not fabricate a future integration hash. Existing MSH-112 remains independently owned/unresolved, MSH-121 is the next integrated architecture/product acceptance, and no M4/variant/M5 completion is implied. No new campaign or product change is requested.

## Evidence text-format follow-up after completion4733f6a

Root reported that the completion commit proceeded after a nonzero diff-check caused by imported CRLF/trailing whitespace in evidence text. This was a real finalization/check-handling failure, not a production defect; the failing check must remain acknowledged. Reviewed the bounded corrective treatment without modifying product or task files.

Independently opened `raw-text-artifacts.tar.gz` without extracting to the filesystem. Archive SHA256 matches provenance `b55992f54f684ffa9449649e24ca23c0a2fd529e7cd2420887935dab428d393b`. Exactly16 regular members match all16 provenance paths. Every retained original matches its declared SHA256/byte count **and** the actual byte-exact file committed at `4733f6a2b6f1e05be4b6fb9a98b4f93081606ab4`. Every normalized view matches its declared SHA256/byte count and independently recomputed LF/trailing-whitespace-only transformation. All9 JSON files also compare identically using duplicate-preserving ordered object pairs; no field/value/order information changed.

Independently ran `git diff --check` and `git diff --check 8de41e4`: PASS. `git diff --exit-code 8de41e4 -- crates desktop/src desktop/src-tauri Cargo.toml Cargo.lock desktop/scripts`: empty/PASS. Production, verifier source, PNG/package/ZIP identities and original auditor hashes remain unchanged. Original audited hashes refer to the byte-exact archive members, with normalized text views explicitly mapped; no original evidence silently discarded or falsely relabeled.

Correction disposition: **PASS**, proportional evidence-format follow-up; no product regression suite required for whitespace-only views with retained originals. Finalization should retain the failure acknowledgment and provenance explanation, record this independent check and actual follow-up integration/push. No remaining material acceptance issue identified.
