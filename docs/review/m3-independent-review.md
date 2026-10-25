# Independent M3 integration assessment

Reviewer: fresh delegated session `/root/m3_final_review`, 2026-10-25. This
reviewer did not author the implementation or its earlier evidence. This is a
bounded source, tests and evidence assessment for MSH-39, independently authored
from the builder's milestone verdict. No source or Backlog records were changed
by this review.

**Source/evidence verdict: PASS. No remaining material finding was identified
within M3's accepted scope.** Final milestone closure still requires successful
final-source native and desktop CI and normal closure integration; those gates
are the integrator's responsibility. At this review's final direct GitHub
observation, Rust CI had passed all three systems and desktop CI was still in
progress. A pending check is not a pass.

## Exact scope and provenance

The reviewed integration checkout was clean at
`475e3b1a2d81db81175891f07da61db7806d6589` on `codex/rewrite`. The independent
source reads and Rust tests used that checkout. Actual GUI-tested integration
`2f0e4cee3e1cc05df99ff51178eb4be810fbcb44` has no differences from it in
`crates`, `desktop`, `.github`, `Cargo.toml` or `Cargo.lock`; the later integration
adds completion records and evidence. The builder's `900932f` has identical
product sources, workspace members and lockfile, but its root `Cargo.toml`
still excludes `legacy/rust-prototype`. Integration preserves another worker's
prototype removal and removes that obsolete exclusion. Thus builder and
integration manifests are not byte-identical. Frontend tests used the builder's
already installed locked dependencies against identical frontend sources.
macOS builder GUI reuse therefore establishes unchanged application behavior,
not identical root-manifest bytes. Actual integrated Windows/Linux GUI execution
and native CI cover the workspace after the unrelated prototype cleanup.

The review read `AGENTS.md`, the accepted design, the complete
[M3 contract](../desktop/m3-design.md), MSH-39 and all acceptance criteria for
MSH-27–38 and MSH-61 using `backlog task view <id> --plain`. All thirteen
dependencies are Done with checked acceptance criteria. Their claims were
cross-checked against implementation/tests and the first, second, completion and
three-platform evidence documents, rather than treating ticket status as proof.
The review preserves M1/M2 closure and the MSH-40 roadmap correction. It does not
repeat deep chess-rule, minimax or strength audits.

## Eleven-area acceptance matrix

| Required area and tickets | Independently assessed source/test boundary | Actual/reused evidence and assessment |
| --- | --- | --- |
| 1. Session and interaction contract — MSH-27 | `m3-design.md` maps all eleven areas, specifies native authority, exact history/purpose/identity, optional claims, unsupported restoration and separately bounded Full work. It explicitly gates future timed play and retains all six product games. | First-batch independent contract review and subsequent batch clarifications remain applicable. Contract and current implementation agree; PASS. |
| 2. Tauri application and packaging — MSH-28 | Locked Tauri/React/TypeScript/Vite sources, sibling engine resolution, host commands, `capabilities/default.json`, explicit frontend build script and verification feature gates were inspected. Frontend has core permissions and no shell invocation path. Normal builds force verification off and inspect emitted module/marker inventories; QA has separate output/configuration. | Normal package smoke/inventory reports plus actual Linux apt/Windows NSIS/macOS app evidence establish packaged execution. Native package CI is separately identified from GUI interaction; PASS. |
| 3. Authoritative native controller — MSH-29 | `DesktopSession` owns `ChessMatch`; `DesktopController` and Tauri's controller mutex serialize commands, acceptance and close approval. Match commands use a private candidate, committing only accepted native results. Exact expected version rejects stale commands. Complete pending `EngineRequest` equality binds canonical history, request, purpose, limits and process generation. Frontend monotonic snapshot acceptance prevents old responses repainting newer state. | Independently rerun session tests cover atomic rejection, same FEN/different history, new-game ABA, generation mismatch, duplicate application and cancelled completions. Existing native-shaped UI fixtures remain checked; PASS. |
| 4. Structured engine supervision — MSH-30 | Protocol v2 has distinct move, claim, finished, incomplete, reachability and failure results. `EngineBridge` has one worker, one pending/result slot, bounded request/response/stderr, off-path pipe workers, cancellation publication filtering, watchdog and bounded cleanup. Child context equality precedes result delivery. Windows internal launch uses `CREATE_NO_WINDOW`. Cleanup uncertainty remains explicit and unscored. | Independently rerun actual subprocess/internal and protocol tests cover missing/malformed/oversized/failing/stalled children, replacement, shutdown and exact echo. Final Windows native captures establish the console-focus regression is resolved in the recorded environment. UCI remains available; PASS. |
| 5. Board/promotion/orientation/keyboard — MSH-31 | Shared `Board.tsx` renders native pieces/complete choices with roving SVG focus, coordinate labels, arrow/Enter/Space/Escape handling, visible focus, promotion completion dialog and cancellation. Orientation is independent of human color. Native versions retire partial choices. No frontend legality engine was found. | All frontend tests rerun. Actual normal human-Black macOS screenshot and Windows/Linux promotion cancellation/underpromotion evidence were inspected or cross-checked. Existing MSH-37 default/minimum-size keyboard evidence is reused; PASS. |
| 6. Human/engine play and bounded analysis — MSH-32/33 | Native engine-turn acceptance applies move/claim through the match API. Ordinary analysis has an explicit purpose and separate `accept_analysis_unchecked` path with no move or claim application. One execution owner retires replaced/turn work. Target selection/history/version/generation remain exact. Full reconstruction/reachability runs in a supervised child with deterministic position/transition caps; unknown/exhaustion/cancellation cannot adjudicate the live game. Full/timed live restoration rejects explicitly before expensive replay. | Fresh actual-child tests verify both human colors, read-only suggestions, explicit Resume, Full exhaustion and cancelled replacement. Original advanced Full GUI evidence is reused with its original provenance. Final platform normal archives retain the exact historical prefix; PASS. |
| 7. Outcomes/recovery/cancellation — MSH-32/34 | Native claims do not play intended moves; already concluded, unresolved resignation, cancelled/incomplete work and process failure remain distinct. Failure and recovery preserve records; retry creates fresh request/generation. UI presentation explains unknown mating feasibility without treating it as possible mate or assigning a score. Native close approval is serialized against mutation and protected by exact document revision. | Fresh native tests cover intended/current claims, engine claims, unresolved and proven-impossible resignation, recovery and races. Actual QA sidecar removal/retry, intended-claim and guarded close/Quit evidence supports the UI boundary. macOS Tao callback provenance/native regression is preserved; PASS. |
| 8. Canonical save/open/history and capacity — MSH-35/61 | `persistence.rs` saves complete canonical MatchRecord/GameRecord, controls, settings and exact-context observations. Same-directory exclusive temporary write/flush/sync precedes replacement. Open/validation/replay and historical prefix reconstruction are bounded worker jobs. Exact live preconditions guard candidate commit; Open installs fresh runtime identities and stops work. Saved observations are historical. `capacity.rs` admits complete immutable candidates before acceptance, reserving stop/close/identity headroom. No truncation or save-reset policy was found. | Fresh tests cover byte/event/control boundaries, analysis/engine 256→257, actual large witness byte capacity, failed/interrupted/stale saves/opens, concurrent Save As bookkeeping, exact history/claim prefixes and unchanged-session refusal. Retained actual capacity archives contain every 256 accepted observations on all three platforms. Save/Open and separate Return live/Resume are actual normal-package journeys; PASS. |
| 9. Synthetic 16×16/compound routes — MSH-36 | Verification-only `SyntheticApp` uses the actual shared SVG board/prefix selection. Native controller accepts exact complete choices against a monotonic version and derives effects using the shared M1 fixture source. `i9→j10→i9` and `i9→h9→i9` retain different complete identities/effects. Prefix completion/continuation is explicit; partial/cancelled choices never issue a complete action. Synthetic record profile is distinct from chess and chess file/engine operations are gated. | Fresh native synthetic and frontend route tests passed. Running-app pointer/keyboard/orientation/minimum-size evidence remains explicitly labeled synthetic, with both final Windows/Linux same-endpoint routes recorded; PASS. |
| 10. Integrated interaction/accessibility — MSH-37 | Focus recovery, modal traps, status/alert semantics, history/live labeling and capacity's once-per-new-pause focus were inspected. Frontend tests cover busy focus, promotion, analysis cancellation, unsaved guards and unchanged-poll focus retention. Scrolling preserves readable large-board/control dimensions. | Reused actual measured-size macOS journeys and independent interaction closure review document resolved overlay, collapsed inspector and focus findings. Final Windows/macOS capacity screenshots visibly expose the complete explanation while retaining save controls. Screen-reader audio/certification is not claimed; PASS within the accepted evidence scope. |
| 11. Actual three-platform packaging and final closure — MSH-38/39 | Workflow and external GUI adapters were reviewed separately from product code. W3C pointer/key input and PID-bound native dialogs/menu/close operate actual apps; scripts read DOM without application IPC or injected rule results. Final transport-closure handling is restricted to the dispatched specific native guard and still requires process absence. Failure cleanup cannot pass a phase. | Six final Linux phases and six final Windows phases all pass with native Quit guard dispatch and host/engine absence before teardown. macOS uses actual normal/QA apps with explicit source reuse and focused current regressions. MSH-38 evidence supports all AC; MSH-39 source/evidence review passes, with final-source CI still an explicit closure gate. |

## Checks actually performed by this reviewer

All commands below are reviewer executions, not inferred from earlier summaries.
Rust test outputs were directed to the builder's existing `target` cache; the
manifest/source checkout remained exact integration `475e3b1`.

* `CARGO_TARGET_DIR=<builder>/target cargo test --locked -p
  mshogi-desktop-session --features desktop-verification`: **46 passed**,
  including capacity byte/event/control boundaries, cancellation/admission races,
  canonical replay, historical provenance and native synthetic acceptance.
* `CARGO_TARGET_DIR=<builder>/target cargo test --locked -p mshogi-engine -p
  mshogi-console --test desktop_play --test desktop_analysis --test
  desktop_persistence --test internal --test protocol`: **22 passed** (4 play,
  2 analysis, 3 persistence, 5 real internal-process and 8 protocol tests).
* Bundled Node 24 executed `node_modules/typescript/bin/tsc --noEmit` and
  `node_modules/vitest/vitest.mjs run` in the builder desktop directory:
  **strict checking passed; 78 tests in 13 files passed**.
* Recomputed every retained-file SHA-256 listed in the final platform
  inventories: Linux **48/48**, Windows **49/49**, macOS final normal **3/3**,
  capacity-focus **3/3**, current **7/7** and status **3/3** matched. This checks
  retained files; it does not claim a fresh download/hash of omitted installers
  or expired remote artifacts.
* Parsed the actual retained normal and capacity archives. Both final
  Windows/Linux normal archives contain three observations and a two-move live
  record; their historical analysis request contains exactly the one-move
  `e2e4` prefix, Black to move, excluding future `a7a5`. Each capacity archive
  retains 256 observations and 256 controls; macOS's focused saved archive does
  likewise.
* Inspected ordered raw reports for all twelve Windows/Linux phases. Each
  records native Quit, the resolved **Discard & continue** or **Close fixture**
  action, and absence of host/engine before driver teardown. Recognized Linux
  transport closure is explicitly not exit proof. Cleanup isolation is separate.
* Viewed representative raw final images: Linux historical read-only analysis,
  Windows visible capacity pause, macOS normal human-Black play and macOS
  capacity focus/save state. These agree with native/DOM reports and preserve
  the visible environment and verification labels.
* Inspected source diffs establishing unchanged product code between GUI-tested
  `2f0e4cee`, builder `900932f` and reviewed `475e3b1`, and unchanged product code
  from focused capacity revision `4e1e9d4` to the final GUI source. A subsequent
  explicit manifest comparison corrected the initial overstatement of exact
  builder/integration equality: both builder revisions retain the obsolete
  `legacy/rust-prototype` workspace exclusion described above. Integrated
  `2f0e4cee` through `475e3b1` and closure integration
  `1e138926d4e35acdbd9912f6aff2fc7dc64aadf5` have identical product code and build
  manifests. The latter diff contains only documentation and the MSH-39 record;
  its README preserves the unrelated prototype deletion/history link.
* Read the integrator's closure draft and the README, desktop README, design and
  desktop-contract status updates. Their capabilities, evidence reuse, current
  native-CI gate and future M4 scope boundary agree with this assessment; no
  product scope or requirement was weakened. The draft explicitly leaves final
  milestone closure dependent on completed checks.

The initial frontend attempt used the machine's default Homebrew Node and failed
to launch because `libsimdjson.33.dylib` was missing. It did not execute checks.
The documented bundled runtime resolved this local tool problem, producing the
successful checks above. The first ordinary `gh` calls could not reach GitHub
under sandbox networking; permitted read-only network access then returned the
actual run states. Neither failed attempt is counted as passed verification.

## Reused evidence and final integration gate

The [first batch](../desktop/first-batch-evidence.md),
[second batch](../desktop/second-batch-evidence.md),
[completion batch](../desktop/completion-batch-evidence.md) and
[packaged GUI assessment](../desktop/packaged-gui-verification.md) retain original
revisions, prior independent reviews, screenshots and commands. The
[MSH-38 independent review](../desktop/evidence/m3-platforms/independent-review.json)
is supporting evidence, not this reviewer's own GUI execution. This reviewer
did not operate a new Windows/Linux/macOS GUI session or rebuild installers.

Actual final GUI evidence is at `2f0e4cee`, supported by successful
[desktop 40234257669](https://github.com/jweathers777/mShogi/actions/runs/40234257669)
and [Rust 40234257686](https://github.com/jweathers777/mShogi/actions/runs/40234257686)
on all three systems. The reviewer directly confirmed current final-source
[Rust 40235926930](https://github.com/jweathers777/mShogi/actions/runs/40235926930)
completed successfully on Windows, Ubuntu and macOS at exact `475e3b1`.
[Desktop 40235927049](https://github.com/jweathers777/mShogi/actions/runs/40235927049)
was still in progress on all three systems at the last direct observation.
Closure must record its completed native-platform results and the final
integration reference; unchanged source alone does not turn a pending required
check into a pass.

## Material findings and limits

No material source, integration, acceptance-evidence or verifier finding remains
from this assessment. MSH-27–38 and MSH-61 have evidence for their full accepted
criteria. MSH-39 AC1 and the substantive dependency/evidence part of AC2 are
supported. AC3 and terminal task/milestone bookkeeping depend on the successful
final CI/integration gate described above.

The evidence establishes Ubuntu 24.04.5 on X11/Xvfb/Openbox with the recorded
software-compositing override, Windows Server 2022's interactive Default desktop
with its matched WebView2 driver, and actual macOS 27.0.1 arm64 applications.
macOS 15 CI package construction is distinct from the macOS GUI environment.
Linux normal was actually apt-installed; its separately labeled QA package was
extracted. Windows normal and QA were actually NSIS-installed.

Earlier failed GUI attempts remain failed. Final Windows Quit is actual native
mouse-menu selection; earlier Ctrl-Q automation has an unestablished cause and
this review does not claim shortcut success. The Cocoa `terminate:` verification
probe/native regression and narrow Tao patch provenance remain valid evidence,
without claiming an actual Dock click. Screen-reader audio, formal accessibility
certification, release signing/notarization, every OS/compositor, hard realtime
filesystem cleanup and descendant-process containment are not established.

These are recorded M3 limits, not silent scope reductions: untimed M2 live play,
no timed/Full live restoration, read-only bounded Full cooperative analysis,
explicit resource ceilings, no branching or crash recovery, and no claim of
complete default chess adjudication or competitive strength. Full chess/product
and six-game obligations remain in the design and future milestone tickets.
MSH-40 is preserved; MSH-41–60 remain future To Do. Work stops at M3.
