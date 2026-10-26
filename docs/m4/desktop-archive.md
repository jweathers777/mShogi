# Native desktop archives (MSH-73)

This extends [the desktop session contract](desktop-session.md). Native chess
and shogi records remain authoritative; this is concrete dispatch, not a shared
rules model. The selected shogi profile remains `mshogi.shogi.csa36.v1`, with its
immutable CSA edition and untimed software policy. MSH-74 owns product UI and
MSH-75 final integration; neither is implemented by this archive boundary.

## Version and compatibility

The existing strict chess desktop archive **v1 is frozen**. Its writer remains
v1, with complete match requests/replies, initial game history, policies, mode,
limits, preferences, controls and observations. It is not reinterpreted as a
multi-game archive. The old parser still rejects additional fields; no missing
advanced mode is silently supplied. Existing M3/M61 corpus and Full restoration
restrictions apply unchanged.

New shogi saves use **v2**, with required, explicit `schema_version:2`,
`format:mshogi.desktop.archive.v2`, `game_kind`, `rules_profile`,
`profile_edition`, and a game-specific `payload`. These are distinct from the
native record envelope/schema and engine transport versions. A v2 chess payload
is exactly the original strict v1 archive (accepted by both chess and shared
Open); saving chess continues to produce v1 for compatibility. The old chess
endpoint refuses v2 shogi explicitly. Shared Open dispatches only the named game;
unknown games, wrong profile/edition and unsupported versions never fall through
to another parser. There is no downgrade or guess from board geometry/notation.

All nested records/commands enter their authoritative strict parsers through
original `RawValue` bytes. Header/count probes do not reserialize maps. Duplicate,
unknown, missing, conflicting and malformed fields remain observable and reject.
Outer wrong-game/profile/version messages are distinct from corrupt replay and
`Application capacity:` refusals. Files must be regular files, bounded at read.

## Shogi payload and reconstruction

The payload contains initial and current **native canonical records**, the native
human side, initial/current shogi preferences, ordered accepted desktop controls,
complete observations, and a retired execution high-water mark. Initial/current
records are journal base and validation target; the desktop does not duplicate
pieces, hands or conclusions in a lossy position model.

The native records preserve profile/edition/schema, canonical root/category,
SFEN declared counter and its provenance, known/unknown earlier history,
complete moves/drop/promotion/capture effects, actors and order, board ply,
repetition/check evidence, declaration and resignation events and native
conclusions. Undo reconstructs a native prefix. Controls preserve accepted
semantic commands and observation ordering, including retired branches. Native
replay must equal the current record and final preferences; serialized conclusions
are never trusted independently. Declaration advice is not a committed event.
Unsupported chess Full/claims are rejected as shogi operations.

Historical selection is intentionally document state: Inspect/Return-to-live
controls retain the exact event prefix, including before/after a terminal event
at the same board ply. Partial board/hand selection is transient. Saved analysis
retains exact native history/target/purpose, side, event/board-ply distinction,
request/session/process/selection IDs, work limits and actual result. Observations
on undone branches remain evidence about those branches, never current analysis.
Settings currently retain native orientation and ordinary depth/node defaults;
actual observations keep their original limits even after defaults change.

Open runs through the off-path bounded worker, validates the entire journal and
native targets/PV, and atomically installs only against the approved document,
session version and accepted-content revision. It allocates fresh runtime
identities above current runtime and saved issued-execution floors, clears pending
work/current analysis, marks observations historical, and pauses play. **Resume
is explicit.** Runtime ownership is not saved. The watermark only prevents old
execution identities from becoming current; UI/I/O revision growth cannot poison
Save. Historical selection and Undo cannot revive retired requests.

## Saveability and bounded resources

Every shogi command and engine/analysis completion is first applied to a private
candidate, then admitted off-path using a stable receipt. Complete encoded bytes
and counts are checked before accepted contents change. Invalid native commands,
resource refusal, incomplete search, process failure and chess/shogi outcomes
remain distinct. Capacity refusal retains accepted history/observations and pauses
execution: save the complete current document, then start a new document. Merely
saving/reopening does not reset limits, and retrying the same impossible save is
not offered as a remedy. No history or observation is dropped.

The existing numerical archive limits remain unchanged:16MiB encoded archive,
256 observations,8192 controls. Shogi uses its **native321-event parser ceiling**,
not chess's4096-event bound; the selected game policy normally concludes after
320 board actions (terminal commands remain separate events). Encoded native
canonical context and engine request/result must also fit their existing1MiB/2MiB
transport bounds. Limits apply to actual envelope plus payload bytes.

A potentially active journal reserves one control and1024 encoded bytes for
native stop/close, and budgets maximum-width future identity fields. A private
journal-derived reservation survives the runtime pause on Open; preferences
cannot accidentally release it. Close consumes the reserved slot. Historical
observation flags are budgeted at their longer encoding before admission.
The archive's conservative **16,000 journal replay-action work budget** charges
each native event at4 work units (including initial/current records and
observation histories); Undo/Inspect reserve the full native321-event maximum
at that weight, observations add8, and other controls cost1. One replay action is reserved for stop/close whenever active. This bounds repeated reconstruction;
it is an additional, game-specific admission policy, not an increased constant.

Parsing has a separate bounded cost: a raw first stage rejects excessive counts
before native parsing, then probes native event counts and conservative journal work before
authoritative validation. At most258 native records (initial/current plus256
contexts), each bounded by the native321-event/1MiB parser, remain possible within
the16,000-work total. Cancellation
checks separate records, observations and controls. Each individual native record
validation is atomic; it does not check cancellation at every native event.
Control replay has its own stop/deadline checkpoints. This is a bounded worker,
not a hard real-time latency promise. The existing worker deadline can reject an
Open without touching live state. Diagnostics are UTF-8-safe and bounded4KiB.

## Failure and concurrent I/O

The existing same-directory temporary-file write/flush/sync and platform-safe
replacement path is reused. An interrupted/failed Save preserves the previous
valid file and never marks the current revision saved. A concurrent Save captures
an accepted immutable snapshot; later accepted contents remain dirty. Save As may
update the same document's path without overwriting newer settings/content.
Completion from a replaced document cannot claim its path or saved revision.

Corrupt, unsupported, over-budget, cancelled or stale Open leaves the current
accepted native document intact. A successful Open retires pending admissions and
old process work. Native replacement tokens and frontend document epochs prevent
ABA/stale snapshots across games. Host operations retain bounded command/file
reservations; dialogs and disk/native replay never hold the controller lock.
Close/quit retains native unsaved-change approval and child cleanup. Cancelling
Save in Save-and-close or Save-then-Open cannot discard dirty work.

## Development reproduction

Normal desktop remains the chess product. For archive verification only:

```sh
cd desktop
npm ci
npm run engine:prepare
npm run tauri -- build --features desktop-verification --config src-tauri/tauri.verification.conf.json --bundles app
MSHOGI_DESKTOP_FIXTURE=shared-contract 'src-tauri/target/release/bundle/macos/mShogi Verification.app/Contents/MacOS/mshogi-desktop'
```

The clearly labeled **MSH-73 DEVELOPMENT CONTRACT** provides minimal native Save,
Save As and Open pickers, preferences, event inspection, explicit Resume and
unsaved-change guards. Example: Resume→7g7f→actual engine reply→Analyze→Save As;
quit, relaunch fixture→Open saved file→inspect Root→Return to live→Resume. Saved
observations appear only in technical history. There is no archive of resumable
pending work. Other systems build their native Tauri bundle; applicable CI records
its actual tests separately from local macOS harness interaction.

This harness is not evidence of approved/completed shogi visual design. MSH-74
must deliver full9x9 visibility, deliberate owner-oriented pieces/promotions,
integrated hands/drop, coherent hierarchy and polished pointer/keyboard use with
diagnostics separated. Future12x12/16x16 products must display full boards without
requiring zoom/pan; no common renderer/layout imposes the harness's scrolling.
