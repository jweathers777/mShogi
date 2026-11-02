# Tori native desktop runtime — MSH-131

The current native contract integrates `mshogi.tori.modern.v1` through the sole
`DesktopController`, `ActiveDocument::Tori`, common `RuntimeMetadata`, admission
worker and `persistence::install_document`. [MSH-130's immutable document](desktop-document.md)
remains the persistence authority. There is no second controller, file format,
execution family or frontend owner. This is native integration, **not playable
Tori desktop delivery**. MSH-132 owns complete semantic and frontend presentation;
MSH-42 owns notation, engine and final product rendering and play.

## Admission and authority

`SessionOperation::Tori { actor, command }` retains the native side and complete
`GameCommand`; `ToriPreferences` retains the native orientation. Board actions
must come from the assigned human, and native `Game::submit` checks the turn,
complete resulting face, king safety and history obligations. Resign retains its
native out-of-turn meaning. Stop reasons stay unscored. Agreement and impasse
reject atomically. No Shogi action conversion or frontend rules are involved.

The worker captures an immutable document reference and current selected event,
prepares a fresh candidate with the MSH-130 journal/admission machinery, and only
then commits under the complete current token/epoch. Failed candidates preserve
the previous native game, settings, observations and accepted execution. Diagnostic
revisions may advance. Explicit Cancel retires execution immediately. A delivered
response that cannot be admitted ends unscored with a resource diagnostic; it
cannot remain falsely Thinking or be recorded as an accepted observation.

All request/purpose/record-history/session/process/limits and selected-target
fields must match. Selection, Return to live, semantic Undo, Take back, Cancel
and successful replacement permanently retire prior requests. Reconstruction
gets fresh native authority; reopening also gets fresh desktop document, request,
session and selection floors. Record/board equality cannot revive execution.

Tori runtime state owns one native `Game`, accepted immutable document and optional
read-only prefix. Drafts contain immutable references and metadata, not cloned
native command authority. No worker can mutate the installed game before its
candidate wins the final admission check.

## History, inspection and minimal snapshot

History selection names an exact native **event count**, not board ply. Prefix
records are reconstructed on the bounded worker with their own root, occurrences,
obligations, conclusions and incomplete state, without future history. Inspection
calls that position's `LocalPosition::inspect` for board and hand origins; it
cannot poll, retire work or authorize a move. Actual choices use
`Game::legal_actions`; actual commands use `Game::submit`.

Undo restores one event prefix. Take back restores the preceding playable human
decision. Both retain the complete abandoned branch and observations in the
current document and issue fresh authority. Return to live is explicit. Intentional
stops remain stopped. Browse selection is runtime-only: reopening starts at the live prefix; saved
observation targets retain their exact historical prefixes and provenance.
Tori has no engine-running intent to resume at this stage;
existing Chess/Shogi/Xiangqi browse-only Return-to-live behavior remains intact.

The minimal `SharedSnapshot` carries actual Tori identity, 7×7 geometry, native
Sente/Gote, faces, hands, complete choices, exact context/events, selected status,
document/settings/resource facts and unsupported capabilities. Its empty Moves
projection is explicitly labeled pending MSH-132; technical face/action encodings
are not final notation. Zero play limits mean **no implemented play/search**, not
observed work. No search score, PV or telemetry is fabricated.

## Execution and normal product gate

Normal Analyze, Resume and Retry refuse unsupported Tori execution. Engine turn,
move analysis, Chess Full and Xiangqi exact capabilities remain false. Only the
separately compiled verification interface can submit a real bounded request to
the existing supervised child and retain its actual unscored failure observation.
Cancellation suppresses obsolete responses and cleans up through that same bridge.

Native contract Replace/Open can install a validated Tori document through the
sole installation path. Normal host New Game, `queue_product_open` and host
`game_command` explicitly reject Tori activation until MSH-42 supplies a renderer
and functional engine. The playable-game dropdown is unchanged. Test-only console
dev dependencies enable the existing verification feature for process tests;
normal console/Tauri package dependencies do not enable it.

## Document limits, runtime preparation and close

MSH-130 native/document quotas and ending reservations remain unchanged, including
**262,144 whole-document replay units**. That figure measures complete immutable
validation, not total CPU work for an entire runtime operation. Additional runtime
preparation has a finite source-backed bound: at most four native reconstructions
(each at most 65,536 native units plus 128 root charge), plus at most 512 units for
the extra direct native command admission. Thus additional preparation is bounded
by 263,168 units; it does not increase the accepted document budget. Every path
runs under the existing 15-second worker deadline, checking cancellation between
bounded native calls. A single native replay is bounded but not internally
interruptible; the deadline may be observed after that call returns.

Runtime admission also reserves the remaining decimal width of the retired floor
(up to the maximum supported safe identity), so later Save cannot fail merely
because that runtime-only field widens after terminal/close space is consumed.
Both a document and its reserved-close candidate are prepared before installation.
The controller captures their `Arc` references without cloning/encoding a full
archive. Close retires work and swaps the prepared candidate; repeated close is
idempotent. The shared exact stop-only exemption survives repeated confirmation
without falsely marking the new control saved. Dirty/never-saved games remain
protected. Save floor refresh, complete encoding/validation and atomic file I/O
run on the existing worker. Save receipts clear dirty state only for the exact
current document identity and revision. Open failure/cancellation/staleness cannot
replace the document; successful Open is stopped and all saved observations are
historical evidence.

The actual imported 360-action record contains 104 captures, 103 drops and 16 promotions.
Its native charge is65,341. Because this test imports the complete history as the
initial record, the desktop stores that initial record **and** current record:
313,906 bytes,720 retained events,196,404 document units. Stop/Resign/Close reaches
314,540 candidate bytes,722 retained events,3 controls and196,409units. The saved
file is 314,541 bytes after the runtime-only retired floor widens to11. This differs legitimately
from MSH-130's root-plus360-command journal measurement. No earlier observation or
history is discarded. [Evidence](desktop-runtime-evidence.md) records timings,
actual controller checks and source/CI provenance.
