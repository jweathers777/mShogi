# Current desktop document contract

The current envelope is **`mshogi.desktop.document.v2`**. All six games save only `record`, `human` and `settings`
in its payload. Development-era desktop archives are not supported. There is
no compatibility reader or converter; see design A-09.

## Envelope and native records

The envelope has three required fields: `format`, `game_kind` and `payload`.
Unknown and duplicate fields reject. The game tag selects the native parser;
the native record carries the profile, edition and schema. `payload` is a raw JSON value,
so its original bytes reach the strict parser and native replay.

| Game | Selected profile | Native record schema |
| --- | --- | --- |
| Chess | `mshogi.chess.match.v1` | 1, M2 untimed match |
| Shogi | `mshogi.shogi.csa36.v1` | 2 |
| Xiangqi | `mshogi.xiangqi.wxf2018.v1` | 2 |
| Tori Shogi | `mshogi.tori.modern.v1` | 4 |
| Chu Shogi | `mshogi.chu.jcsa2019.v1` | 3 |
| Tenjiku Shogi | `mshogi.tenjiku.adams1999-modern.v1` | 3 |
| Tenjiku Shogi | `mshogi.tenjiku.muller-completed.v1` | 3 |

Operational interruptions are session state. Cancellation, engine failure and
work limits add no game command and do not change a saved record. Current native
records have no Stop command or incomplete field. Tori schema-1 records retain
their promised replay path under the current repetition rule: old stop entries
are validated and omitted, while all player commands remain. Development-era
Shogi/Xiangqi schema 1, Tori schemas 2–3, Chu schemas 1–2 and Tenjiku schemas
1–2 are unsupported under design A-09.

Tori schema 4, Chu schema 3 and Tenjiku schema 3 store their profile, edition,
schema version, complete native root and ordered actor/command pairs. Chu roots
include Lion protection. Tenjiku roots hold the whole profile and original-piece
rights; the record does not repeat the profile outside the root. Effects, current
position, offers, obligations and conclusions are reconstructed by native replay.

Shogi schema 2 keeps the root category, SFEN and counter source, followed by
actor/command pairs. Xiangqi schema 2 keeps its complete PositionRoot, claim
policy and all actor/command pairs, including admitted rejected claims. Native
replay restores declaration evidence, warnings, claim results and conclusions.

The document stores each non-Chess native record directly. Engine contexts use
a separate canonical wrapper with `envelope_version: 2`, `game_kind` and `payload`.
That wrapper retains the promised version-1 Tori schema-1 reader. Chess native
history schema 1 bytes and schema 2 adjudication settings are unchanged.

The edition is the exact native `RulesProfile::edition()` value. A different
whole profile rejects; no rule toggle migrates an existing document.

Chess stores an initial native GameRecord string and an ordered list of
accepted player commands, each with its actor. Native replay restores moves,
claims, draw offers and conclusions. A claim with an intended move does not
play that move. Untimed replay supplies deterministic request versions and
zero timestamps. Start, Observe, rejected requests, clocks and controller
identities are not saved commands. Timed, live-Full and witnessed resignation
restoration remain unsupported. Full analysis is a separate read-only operation.

The other games store their native record. Complete moves, drops and promotion
decisions remain recorded inputs; native replay derives effects and conclusions.
Chess needs a match command list because a board GameRecord alone cannot retain
player draw offers and match conclusions. It uses the same document envelope,
file worker, admission receipts, session lifecycle and history controls.

Settings include `play_mode` (`human_engine` or `two_humans`), orientation and
analysis defaults. A missing play mode defaults to `human_engine`. Display
preferences cannot change ownership during a game.

## Session state and history

Engine observations and Xiangqi exact evidence stay in session memory. Each
list retains its newest 256 results. They are not saved. Full certificates
receive bounded native validation when accepted; Open does not restore or
verify old analysis evidence.

Take back cuts the command list. It retains the current session's results as
historical observations. Abandoned continuations, history selection, pending
work, request identities and diagnostics are not saved. Chess history indices
count initial native events and accepted player commands, without Start or
Observe. Browsing can inspect an imported root's history; Take back does not
remove that immutable root.

Open restores the live game with fresh runtime identities, empty result lists
and execution stopped. Return to live and Resume remain separate actions.

## Resource and file guarantees

The encoded document limit is 16 MiB. Chess admits at most 4,096 combined
initial events and player commands. Other native event, byte and replay limits
remain game-specific. Tenjiku retains its 1 MiB/1,024-event/512M native limits
and 160M document reconstruction budget. No history is truncated to fit a file.

Validation runs on a bounded, cancellable worker with a 15-second deadline.
It returns the already replayed game for guarded installation. A stale,
invalid, oversized or cancelled candidate cannot replace the accepted game.
Save uses a same-directory temporary file, flush/sync and atomic replacement.
Failure preserves the prior valid file and dirty state. New and Open replace
only after valid atomic admission. Quit retains the Save/Discard/Cancel guard
for never-saved or dirty documents.

## Verification fixtures

`cargo run --locked -p mshogi-desktop-session --example write_document_fixtures -- desktop/scripts/fixtures`
reconstructs the GUI hand-inspection document through the current writer.
`write_frontend_fixtures` builds current native snapshots for frontend tests.
Neither generator is part of the product application.

Retained protocol regression fixtures use the current native record encoding.
Their canonical-prefix hashes identify those encoded bytes. An encoding update
changes those hashes, but does not change engine traffic, accepted commands,
outcomes or timing observations. Git retains the original captured files.
