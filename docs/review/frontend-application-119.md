# MSH-119 / R4 — one frontend application

R4 replaces the three product page assemblies with one application composition.
This is the frontend migration authorized by design A-09/A-10 and D-17/D-18;
it preserves the accepted dark shell and native game semantics. R5/MSH-120's
selection-controller consolidation and R6/MSH-121's final integrated campaign
remain separate, unstarted work.

Final current-source verdict: **PASS**. All nine R4 criteria have supporting
source, behavioral, independent review and identified packaged-application
evidence. Production is `0487945b628314eed6d1be49d396dce41a362332`;
verification corrections and final CI are at
`eb027be7cec0f3206094c477a31f0cd9b9f73f6f`. The final completion commit changes
only evidence, documentation and task records. Historical failures below remain
visible; the unchanged MSH-112 runner race is not claimed fixed.

## Surviving ownership

`ProductApp` mounts exactly one `useGameSession`. That hook alone subscribes to
`game-close-requested`, polls `game_snapshot`, accepts monotonic native snapshots,
serializes user operations and sends exact rendered tokens. Save receipts are
reconciled with the latest native state: a delayed clean receipt cannot authorize
replacing newer dirty work or a different document. New/Open/Quit share the
ProductApp guard. Native ApplicationLifecycle remains authoritative for host
reservations, file admission, atomic installation and close approval.

Every production game renders `ApplicationView`, which calls `ApplicationShell`
once. It owns document controls/state, heading, orientation control, history
notice, recovery, analysis/history/actions/technical sections and confirmation
placement. `GameFrame` owns upper player, board, lower player and shared status
footer slots. `NativeAnalysis`, `NativeHistory` and `NativeRecovery` own each
common section. Small native capability content provides Chess Full results and
claims, Shogi declarations and Xiangqi terminal commands/warnings; these are not
parallel page layouts. `useApplicationFocus` owns non-board recovery and leaves
existing roving/selection/completion lifetimes inside the specialized boards.

`GameBoard` chooses the existing Chess SVG, Shogi board/kanji/hands, or Xiangqi
intersections/palaces/river/pieces. It passes complete native choices and the
current movement-inspection context. It implements no legality or adjudication.
Their remaining interaction-state copies are explicitly assigned to MSH-120.

## Native-specific functionality matrix

| Common section/lifetime | Chess | Shogi | Xiangqi |
| --- | --- | --- | --- |
| Board/player/footer | Original Chess art, native White/Black, complete promotion choices | Original kanji/rotated Gote art, hands, move/drop/promotion choices | Original intersections, river/palaces, native Red/Black art/actions |
| Game actions | Current/intended claims (intention never played), native resignation, Take back | Advisory eligibility distinct from formal declaration and invalid-declaration loss, resignation, Take back | Claimant-specific count claim, offer/accept/decline, resignation, Take back |
| Analysis | Read-only bounded moves; explicit Full cooperative proof/Unknown/cancel/failure; native target/limits/side and real telemetry | Bounded native moves; native Sente/Gote meaning and telemetry | Ordinary sound-fast-only search distinct from saved exact proof/witness/Unknown evidence; incomplete command prefixes not analyzable |
| History | Exact native events, including non-move claims and native initial row | Native event prefixes and hands | Exact WXF warnings/obligations/claims and partial-prefix scope |
| Recovery | Unresolved unscored outcomes, distinct process failure/incomplete/cancel, retry and Resume | Native conclusions, failure/retry and stopped Resume | Warnings, draw offer, unresolved pending state; no heuristic outcome or fallback |
| Document | Same current native archive and stopped reopen; exact observation provenance/settings/lineages | Same | Same, including exact evidence as historical information |
| Inspection | Native read-only enemy movement | Native board and hand inspection | Native read-only enemy movement |

All use the accepted shared inspection wording, common dark theme, control
positions and Analysis → History → Game actions → Technical Details ordering.
Native play limits remain Chess2/20,000, Shogi2/5,000 and Xiangqi1/128 by default;
analysis ceilings and explicit Full work bounds are unchanged.

## Deleted development paths

Removed `App`, `useNativeSession`, `Document`, parallel `ShogiApp`/`XiangqiApp`,
old `Analysis`/`History`/`Recovery`/`Capacity`, `SharedContractApp`, their obsolete
hook helper and interface-only tests. Meaningful behavior moves to current
application/panel/hook/board tests. Seventeen old Chess fixtures are replaced by
current `shared-chess-*` snapshots produced by the actual Rust example.

The current native-tagged Chess payload is `ChessPresentation` capability data,
not an embedded legacy application Snapshot. Direct `chess_play`, `chess_claim`
and `chess_preferences` operations replace nested legacy command transport.
Old public Rust Snapshot/DesktopCommand become private implementation helpers;
old frontend Snapshot/DesktopCommand/LegalChoice transport is removed.

Removed host `desktop_snapshot`, `desktop_command`, `desktop_save`,
`desktop_save_as`, `desktop_open`, `desktop_request_close`,
`desktop_cancel_close`, `desktop_finish_close`, old `desktop-close-requested`
event and QA `native_contract_snapshot/command/file` aliases. Actual console
subprocess tests, example/fixture consumers and synthetic close harness use the
current contract. QA metadata `desktop_verification_mode` and separate synthetic
fixture endpoints remain verification-only; normal packages exclude them.

## Structured resources and safety

Only native capacity state and `ResourceFailure` facts determine a capacity
notice. No diagnostic substring or numeric-limit classifier remains. One known
fact identifies controls, ordinary/exact/combined observations, encoded bytes,
native events, replay work, identity/headroom or validation budgets. Unknown,
multiple or omitted facts use truthful general saveability language. Accepted
counts and candidate counts are not conflated. Xiangqi combined observation
facts retain ordinary/exact accounting. Independent file errors remain visible
alongside capacity; technical diagnostics stay in Technical Details. No limit,
archive format, resource admission, stop/close reserve or native rules changed.

## Initial local verification (historical checkpoint)

The first current-source local checks pass: strict TypeScript;223 frontend tests;
168 native session tests;8 current runtime boundary tests;12 actual Chess
subprocess tests;9 all-feature host tests; native/console/host Clippy and Rust
formatting. The20 existing vendored Tao warnings are unchanged. GUI verifier
migration passes55 external-driver boundary tests. Native rules/search/engine
campaigns from R1–R3 remain applicable; this task does not claim a new rules or
playing-strength campaign.

Initial frontend migration failures exposed coordinate-label adaptation, obsolete
capacity-text tests, direct preferences shape and old promotion label detection.
They were corrected before the passing run. The host removal does not change
native close/termination/Tao behavior. Exact source, independent review, actual
packaged GUI, platform CI and final acceptance are recorded below after execution;
pending checks at that checkpoint were not acceptance evidence. The final disposition is recorded below.

## Independent source and visual review

A separate read-only agent reviewed baseline82a8ae8 through2918217 and all six
same-size actual macOS QA captures. It found two material migration omissions:
Chess's supplied in-check indication and explicit Finish analysis for a paused
concluded Chess game. Both are restored using existing native facts and common
resume; the game remains concluded. New semantic regressions cover both. The
reviewer independently reran40 affected application/panel tests after correction,
in addition to107 focused frontend and8 native runtime checks before correction;
all pass. Current full frontend225 and strict TypeScript pass. Two stale living
host/quit documents were corrected; historical R3 evidence is untouched.

The reviewer confirms one actual composition/session owner, removal of public
legacy DTO/IPC/callers, native-specific content, typed resource messages, exact
source-token/file/close safety and R5 scope. All six1180×820/760×650 captures
retain the same shell/theme/control and section positions, complete boards,
Shogi hands and Xiangqi intersection/river/palace art. No material visual finding.
Original JPEG capture bytes, package identities and actual native interactions
are retained in [the macOS evidence](evidence/msh119/macos/interaction.json).
Those captures are at2918217; the two bounded fixes do not affect their initial
positions or geometry. Final current-source CI and package results are recorded below.

## Initial platform findings and bounded verifier correction

Initial Desktop41128859968 at2918217 passes macOS package checks and all15
actual Windows GUI phases, each with observed native shutdown. Linux passes11
QA phases, including promotion, claims, failure, declarations, capacity and
Save/Open. Its three normal-game journeys fail their newly added Take back
click because the control lies below the clipped, scrollable sidebar viewport;
the subsequent Shogi reopen also fails because its prior normal save never ran.
The [original failures](evidence/msh119/initial-linux/failed-campaign.json) and
[Windows audit](evidence/msh119/initial-windows-audit.json) are retained.

The verifier now reaches Take back by real bounded Tab navigation, observing
unique/enabled/focused/full-ancestor visibility and hit testing before one click.
It does not mutate DOM scroll/focus, invoke IPC, repeat the action or accept a
failure as success. Missing/disabled/unreachable controls and delivery failures
remain failures.62 external-driver tests pass; the independent reviewer approves
the correction and separately runs49 driver checks. This changes verification
only; production remains0487945. The corrected actual campaign is recorded below.

Final-source macOS normal/QA packages build and normal package smoke passes at
0487945. [Actual concluded analysis recovery](evidence/msh119/macos/reviewed-correction.json)
shows Checkmate throughout analysis and after Finish analysis retires the work;
no Resume play/reopened game is offered. Native Quit exits0 after the shared
guard. Initial sandboxed direct fixture launch aborted134; the ordinary approved
native launch succeeds. The capture/manifest separates both revisions and QA
from normal distributables.

## Generated-fixture CI correction

Initial Rust41128859801 fails only the generated-snapshot diff on all three
systems: newly exposed `chess_match.events[].request.at_ns` contains actual
wall-clock command timestamps in four Chess fixtures. Production behavior suites
pass before that gate. The generator now clones its output snapshot, rebuilds
only untimed fixture MatchRecords through native replay with event-index
nanoseconds, and requires identical submitted commands/replies, canonical game
history, conclusion, active state and remaining clocks. Timed records reject
normalization. Production clocks, MatchRecords, archives and history are untouched.
Two example regressions and Clippy/formatting pass. Two complete generations
produce [32 byte-identical fixture files](evidence/msh119/fixture-repeatability.json).
The independent reviewer approves and separately runs both regressions. This is
verification-only; the final generated-fixture CI gate is recorded below.

## Acceptance map

| Criterion | Evidence boundary |
| --- | --- |
| AC1 one owner/deletion | ProductApp/useGameSession, direct current host command family, native runtime8 and actual subprocess12; completed independent source audit. |
| AC2 one application UI | One ApplicationView/ApplicationShell/GameFrame, common analysis/history/recovery/actions/technical slots; six actual same-size macOS comparison captures and independent visual review. |
| AC3 native capabilities | ApplicationChess/Shogi/Xiangqi and board/panel tests; actual packaged Shogi keyboard/analysis/history and Xiangqi play; QA promotion/claim/declaration/outcome journeys in the identified platform campaigns. Full GUI evidence is the identified local macOS bounded Unknown journey; proof/cancel/failure cases have frontend/native tests and bundled-engine smoke, not claimed Linux/Windows Full GUI execution. |
| AC4 complete Chess | Current/intended claim dispatch; Full positive/negative/Unknown/cancel/failure/provenance/limits; native check; unresolved outcome, exact history, settings/telemetry/Resume/retry/Take back tests. Actual normal Full Unknown and final-source concluded Finish analysis capture. |
| AC5 authority/lifecycle | Exact rendered-token/dirty Save/replacement/StrictMode/close tests; native controller/host/subprocess tests; actual cross-game Save/Open/stopped restoration/Resume and native window-close/Cancel/Cmd-Q Save-and-Quit. |
| AC6 R4/R5 ownership | One common non-board focus hook; real WebKit disabled/removal/dialog regressions; specialized selection/inspection/route/board focus stays with the existing boards and MSH-120. |
| AC7 semantic/package/visual |225 frontend tests replace obsolete interface tests. Actual normal macOS journeys and exact normal/min QA captures with package hashes; independent visual review; final complete corrected Linux/Windows campaign gate recorded below. |
| AC8 integration/review/CI | Independent full-boundary review and resolved findings; focused native/host/console checks; production0487945, verificationb3c2b57 and deterministic generator integrationeb027be. Final native/Desktop CI gate recorded below; superseded runs are not called passed. |
| AC9 typed capacity | Native capacity/admission tests plus common resource/panel regressions for all games: control/ordinary/exact/combined/byte/native-event/replay/identity/unknown/multiple/candidate counts, file errors, Save availability and technical-only raw diagnostics. No text classifier or limit increase. |

The unchanged native profiles/rules/search/engine/current archive and R1–R3
correctness evidence are reused. R4 does not claim competitive strength, new
rules conformance or final R6 closure. MSH-111/MSH-112 and their historical failed
checks retain their own ownership. Later R5/R6 and variant/M5 work have not begun.

## Existing MSH-112 recurrence

Finaleb Linux Rust attempt1 fails the unchanged runner early-exit classification
test (`runner.rs:172`): nonzero scripted exit was observed first as Broken pipe /
MissingResponse without an exit status, rather than UnexpectedExit. The
[failed evidence](evidence/msh119/runner-112-recurrence.json) is retained. The
independent reviewer verifies no runner/core/engine/manifest/lock diff from the
baseline and the same test/runner10 passed on initial291 Linux. Both failure
categories remain unscored without a Chess conclusion. A local unchanged focused
test passes1/1; that neither establishes Linux success nor fixes the race.

This exactly matches existing To Do MSH-112, where recurrence evidence was
appended without starting implementation or changing ownership/dependencies.
No assertion, timeout or production runner changes are included. A normal failed
job retry was accepted after overall workflow completion; its result is recorded
separately below. Historical M2 evidence and MSH-112's distinct diagnosis remain intact.

## Final packaged application evidence

Desktop workflow [41131548392](https://github.com/jweathers777/mShogi/actions/runs/41131548392)
passes all three systems at verification revision
`eb027be7cec0f3206094c477a31f0cd9b9f73f6f`. Production remains
`0487945b628314eed6d1be49d396dce41a362332`: later changes affect only the
external verifier, fixture writer/output and evidence. The final independently
downloaded [Linux audit](evidence/msh119/linux/audit.json) and
[Windows audit](evidence/msh119/windows/audit.json) each establish all15 actual
installed-normal/separate-QA GUI phases, package checksums and observed native
exit before driver teardown. Real keyboard navigation exposes Take back in the
clipped sidebar (Chess7Tabs, Shogi6, Xiangqi6), followed by one click and explicit
Resume. No action replay, DOM mutation or IPC shortcut substitutes for interaction.

The six Linux normal-package comparison captures, retained with their original
bytes and hash mapping in the audit, establish1180×820 and760×650 logical content
sizes for all three games. The separate reviewer inspected these alongside the
six macOS QA comparison captures and found no unexplained control movement,
board clipping or material visual regression. Windows records the same six size
checks. Normal distributable instrumentation exclusion passes actual package
smoke on each system. Linux host bytes were independently rescanned; Windows's
installed host is not retained in its artifact, so its exclusion evidence is the
successful actual installed-host CI scan plus reviewed scanner and package hashes.

macOS CI establishes construction and bundled-engine/package smoke, not CI GUI
interaction. Actual macOS native app interaction is local and separately labeled:
2918217 normal play/reply, Take back/Resume, cross-game New/dirty guards,
Save/Open/stopped restoration, analysis/history, bounded Full Unknown and native
window-close/Cancel/Save-and-Quit;0487945 normal package smoke and separate QA
concluded-game Finish analysis/native Quit. Exact packages and captures are in
[interaction.json](evidence/msh119/macos/interaction.json) and
[reviewed-correction.json](evidence/msh119/macos/reviewed-correction.json).

Local checks and their commands/revisions/log hashes are indexed in
[local-checks.json](evidence/msh119/local-checks.json):225 frontend,168 native
session,8 current runtime,12 actual subprocess,9 host,2 fixture-writer and62
external GUI-driver checks, strict TypeScript, Clippy and formatting pass.
R1–R3 evidence remains applicable for unchanged native rules/execution/archive
semantics. Superseded CI41129904514,41130968717 and41130968723 were canceled,
not passed;41129904451 had already completed when cancellation was attempted.
Original failures above remain historical evidence.

## Final native CI and disposition

Rust workflow [41131548408](https://github.com/jweathers777/mShogi/actions/runs/41131548408)
passes all three systems at `eb027be7cec0f3206094c477a31f0cd9b9f73f6f`.
Windows and macOS pass on attempt1; Linux passes on attempt2 after the preserved
MSH-112 failure. Format, Clippy, workspace tests, isolated QA constructors,
generated fixture diff, build and console smoke pass. No runner change or
assertion weakening accompanies that retry. This closes the R4 verification
gate, not MSH-112's independent diagnosis.
The [independent final audit](evidence/msh119/native-ci-audit.json) records exact
job IDs, log hashes and passing lines: each system passes168 default and180
all-feature native-session tests, including all generated-fixture gates.

The independent review method is a separate read-only agent's bounded source
review, independently executed focused checks, screenshot-level comparison and
downloaded CI/package/checksum/native-exit audit. It is not author self-review.
Both material Chess findings and the verification/provenance findings are
resolved. All9AC and4DoD items are supported by the matrix and evidence above.
Native game rules, execution semantics, current archive format, resource limits,
M2/M3 and R1–R3 evidence remain unchanged.

MSH-120/R5 is the next dependency-ready migration stage and remains unstarted;
it replaces the explicitly retained specialized board interaction controller
copies. MSH-121/R6 retains final integrated acceptance. MSH-111/MSH-112,20
existing Tao warnings and later variant/strength/shared-integration work retain
their own scope. M4 remains open. No temporary R4 worktree was created; unrelated
existing worktrees were preserved.

Launch the reviewed normal macOS app (production0487945):

```sh
open /Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app
```
