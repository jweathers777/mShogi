# MSH-119 / R4 — one frontend application

R4 replaces the three product page assemblies with one application composition.
This is the frontend migration authorized by design A-09/A-10 and D-17/D-18;
it preserves the accepted dark shell and native game semantics. R5/MSH-120's
selection-controller consolidation and R6/MSH-121's final integrated campaign
remain separate, unstarted work.

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

## Verification in progress

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
pending checks are not acceptance evidence.

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
positions or geometry. Final current-source CI and package results remain pending.
