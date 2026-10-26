# Shared desktop session and selection contract (MSH-72)

This is the concrete two-game native boundary, using the existing Rust chess
match/session and selected standard-shogi `mshogi.shogi.csa36.v1` game. It does
not deliver shogi desktop archives (MSH-73) or claim the completed shogi desktop
application (MSH-74). M2/M3 and their chess evidence remain closed. MSH-41–47
remain complete-delivery obligations, and M4 remains open.

## Native API and compatibility

`DesktopController::shared_command(SharedDesktopCommand)` and
`shared_snapshot()` extend the same controller and single `EngineBridge` used
by the shipped chess application. No new rules engine, board storage, universal
framework or protocol is introduced. The host's existing mutex serializes
commands, replacement, result acceptance and selection. The framework-free
controller requires exclusive mutable access for the same operations.

Each command supplies an explicit `GameKind` and exact `SessionToken`:
`document`, `version`, `revision`, `selection`. `Replace` identifies the **current**
game in that outer precondition and supplies an independently tagged native
canonical record and native human side for the replacement. Native record
profile/edition/schema checks and side/game agreement are mandatory. Replacement
is an explicit approval to replace exactly that document; callers must provide
unsaved-change confirmation before issuing it. Runtime identities are fresh;
a restored/undone semantic state never restores an execution identity.

Existing `DesktopCommand`, `Snapshot`, v2 execution, chess archive format and
historical observations are unchanged. Shared chess commands delegate to that
controller, including its asynchronous capacity-admission receipt model;
snapshots expose that lifecycle through `chess.capacity`/`chess.io`. The normal
chess application retains its current API and all journeys. The shared wrapper
adds a typed presentation alongside the full legacy chess snapshot. Its
canonical chess context is the rules-owned history record, not a replacement
for the complete match/archive ledger; terminal match events and conclusions
remain in the legacy snapshot/timeline and archive.

Shogi replacement accepts the authoritative native record, root provenance,
supplied counters and complete history. It starts paused. `Resume`, complete
`Play`, formal `Declare`, `Resign`, semantic event `Undo`, `Inspect`,
`ReturnLive`, bounded `Analyze`, `Cancel` and `Retry` are concrete native
operations. Board actions require a resumed human live turn. Terminal commands
retain native admission: a wrong-turn or ineligible formal declaration is an
accepted false-declaration loss; an eligibility query only supplies advice.
Resignation does not acquire a board-action turn restriction. None of these
changes the accepted shogi rules, mutual-check resolution or fixture expectations.

Legacy chess mutations and Save/Open explicitly reject while shogi is active.
A legacy polling call cannot restart hidden chess work. The close compatibility
view reports an unsaved document; shogi semantic changes, accepted observations
and replacement advance document revision, so old close approvals cannot become
current again. File/admission jobs and engine work retire before game switching.
The separately labeled development surface offers explicit discard/keep-open
handling. Shogi has `save: false`; no `.mshogi` support or persistence guarantee
is claimed for it here. MSH-61 admission/save guarantees for chess remain intact.

## Presentation and complete choices

`SharedSnapshot` explicitly supplies game, immutable profile/edition,
profile-qualified sides, first-mover labels, native human and turn identities,
square geometry and both orientations. Chess is White/Black; shogi is Sente/Gote.
A board does not decide game identity from dimensions or notation. Native shogi
ordering is top-left **9a** for Sente at bottom, **1i** for Gote at bottom.

Semantic pieces carry owner, base kind, promoted flag, readable text and native
accessible label. Shogi uses K/R/B/G/S/N/L/P and + for promotion. Both hands
include counts (including zero), native labels and opaque origin keys. Captured
promoted pieces demote into native hands; the frontend performs no inventory,
nifu, dead-rank, king-safety, promotion or outcome calculations.

A `PresentedChoice` contains a typed `ChoiceOrigin::Board` or `Hand`, an opaque
complete native action, ordered `steps` and optional explicit completion. The
same endpoints need not mean the same route/action. Optional promotion supplies
both Promote/Do not promote choices; compulsory promotion supplies only Promote.
The shared `Board` retains the M3 synthetic route/complete-action handling and
adds hand selection and native coordinate order. `NativeBoard` adapts only DTO
presentation. Click, arrows, Enter/Space, Escape and completion-dialog focus
operate over supplied choices. A selected hand, route prefix or promotion dialog
is never a committed action. Game/document/version/selection/orientation changes
retire partial choices. No Chu/Tenjiku interaction implementation is claimed.

History selects **complete event prefixes**, including terminal commands with
unchanged board ply. The selected native record never includes future moves or
repetition information. `ReturnLive` is explicit and remains paused. Native
capabilities distinguish complete actions, drops, promotion, undo, engine turns,
analysis, chess Full, claims, declarations, resignation, hands, history and Save.
Formal declaration availability is deliberately distinct from advisory eligibility.

## Execution and bounds

Chess continues through its established v2 play, analysis and explicit bounded
Full paths. Shogi uses the existing v3 native engine consumer, ordinary play
limits depth 2 / 5,000 nodes, and explicit analysis depth 1–4 / at most 20,000
nodes (zero nodes is a supported incomplete-work experiment). Chess Full and
claims explicitly reject for shogi. There is no shogi tsume operation here.

There is one execution resource. Starting/replacing analysis retires engine-turn
work and pauses play; Cancel retires publication without a move/outcome; Resume
retires analysis before live work. Every accepted result equals the complete
pending request: native root/history/profile, purpose, request ID, session version,
process generation, work limits and historical target/selection. Request and
process-generation counters increase across chess/shogi/chess. Analysis only
records observations and cannot pass through native move/declaration admission.
Engine-turn choices are separately admitted by the native game. Conclusions,
incomplete work, process failures, and rejected choices remain distinct. Retry
keeps native history and uses new identities. All accepted observations retain
exact contexts and real telemetry; no live progress is invented.

Shogi scores retain root-side hundredths-of-pawn meaning. Native terminal win
distance includes declaration command edges and must not be labeled chess mate.
Full raw native result/context and score interpretation remain available in
technical details. Shared chess analysis stays in its typed compatibility view
with its existing score/perspective and Full meaning.

Existing bridge watchdog, bounded queues, cancellation and child cleanup remain.
Full encoded engine request/response limits stay 1/2 MiB; diagnostics stay 4 KiB.
The raw shared desktop command cap is native envelope cap (1 MiB) plus 4 KiB for
fixed command/token/side overhead; native envelope and payload caps also apply.
Raw `SessionOperation` replacement dispatch preserves duplicate fields through
strict native parsing, avoiding serde's internally tagged enum buffering.
Diagnostics truncate at a valid UTF-8 boundary. No existing limit was raised.
Shogi history/reconstruction uses the native bounded maximum of 320 actions and
321 event parser capacity, with no Full reachability reconstruction. Retained
observations are capped at 256; new analysis/resume/retry is refused before
committing at capacity. Native accepted history/observations are not truncated.
This is in-memory bounded development support, not an archive design.

## Development surface and reproduction

Normal app launch remains `cd desktop && npm run desktop:dev` (Node 24.19.0;
Tauri/native prerequisites in `desktop/README.md`). It remains the chess product.
The development contract surface is compiled only in the separate verification
host/frontend and is labeled **MSH-72 DEVELOPMENT CONTRACT**:

```sh
cd desktop
npm ci
npm run engine:prepare
npm run tauri -- build --features desktop-verification --config src-tauri/tauri.verification.conf.json --bundles app
# macOS: run the executable directly so it receives the selected fixture.
MSHOGI_DESKTOP_FIXTURE=shared-contract 'src-tauri/target/release/bundle/macos/mShogi Verification.app/Contents/MacOS/mshogi-desktop'
```

Use Resume before board play. The test surface exposes the shared board/hands,
orientation, bounded analysis/cancel, event inspection, Return to live, undo,
declaration/resignation, exact technical observations and unsaved-discard close.
It is deliberately not the final shogi application or Save/Open implementation.
Other platforms use their Tauri bundle format and the same environment selection.
Native endpoints accept raw bounded command JSON; they are absent from normal
builds. Frontend build guards and package inventory/host marker checks enforce
instrumentation exclusion.

Verification and independent-review results belong to
[the MSH-72 evidence record](desktop-session-validation.md). Earlier M2/M3,
MSH-63–71 research, fixtures and platform evidence are reused only for unchanged
behavior. MSH-73 is next; MSH-74/75 and MSH-41–47 stay open.

## Visual-design boundary (owner clarification, 2026-10-26)

**MSH-72 DEVELOPMENT CONTRACT is a development-only verification harness.**
Its visual design, layout, sizing, Latin-letter piece rendering, hand buttons,
control strip and raw technical state are not completed or approved shogi product
design and are not the design basis/acceptance target for MSH-74. No further
harness polish is required to close MSH-72's semantic/verification scope.

The DTO's `kind`, `promoted`, `owner`, geometry, native orientations and hand
resources are authoritative semantics; `text`/labels are readable fallbacks for
verification and accessibility, not required piece artwork. Clients can render
appropriate game-specific pieces using those fields. Coordinates and ordered
orientations prescribe no pixels, viewport sizing, scrolling or zoom policy.
Typed hand origins/counts prescribe no button-row layout. Analysis/observations
carry provenance without requiring raw diagnostics in the normal interface.
`NativeBoard` is one concrete renderer/verification consumer, not a mandate for
identical chess/shogi/large-variant product layouts.

MSH-74 owns the actual shogi UI design and implementation: full9x9 visible
without scrolling at supported normal window sizes; deliberate shogi-specific
Sente/Gote-oriented pieces with promoted-state support; integrated hand/drop and
clear promotion interaction; coherent player/turn/game status; history and
analysis subordinate to a readable board; polished keyboard/pointer use; and
engineering/raw state separated from ordinary play. Its ticket preserves all
existing criteria and adds these product expectations. MSH-72 implementation
and tests do not establish those criteria.

Future12x12/16x16 product design must not inherit the synthetic scrolling default.
The owner cites legacy `master`'s longstanding full16x16 Tenjiku display on
smaller screens. Assess full-board fit/readability first; zoom/pan or other
large-board techniques are alternatives to justify where useful, not assumed
requirements. This clarification neither reopens M3 nor implements MSH-74.

## MSH-73 archive extension

The historical MSH-72 evidence and its then-missing Save/Open limitation above
remain a record of that revision. MSH-73 now adds native shared Save/Open via the
versioned [archive contract](desktop-archive.md), including strict chess v1
compatibility, shogi v2 lossless replay, pre-commit capacity and fresh stopped
reopen. The separately built surface is now labeled **MSH-73 DEVELOPMENT
CONTRACT**; minimal file/unsaved-change controls do not turn it into product UI.
MSH-74 remains next and owns all product visual acceptance. The owner requirement
for future12x12/16x16 is full-board display without requiring zoom or pan; prior
capability demonstrations must not be read as requiring either technique.

## MSH-74 normal product surface

The owner-approved implementation direction is recorded in
[the shogi product design](shogi-desktop-product.md). It introduces actual normal
Chess/Shogi New Game selection and a distinct shogi board/player/hand layout,
keeping the MSH-72/73 harness verification-only. Full9x9 visibility is required
at1180×820 and the760×650 content minimum. Future12x12/16x16 entire-board fit must
adapt square/piece/type sizing without required zoom/pan. An owner visual review
checkpoint follows the complete polished surface/screenshots; MSH-74 remains
In Review until explicit approval, and MSH-75 stays unstarted.
