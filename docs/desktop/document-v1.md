# Current desktop document contract

The current envelope is **`mshogi.desktop.document.v1`**, with
`envelope_version: 1`. All six games save only `record`, `human` and `settings`
in its payload. Development-era desktop archives are not supported. There is
no compatibility reader or converter; see design A-09.

## Envelope and native records

Envelope fields are required. Unknown and duplicate fields reject. The game,
profile, edition and native schema must agree. `payload` is a raw JSON value,
so its original bytes reach the strict parser and native replay.

| Game | Selected profile | Native schema |
| --- | --- | --- |
| Chess | `mshogi.chess.match.v1` | 1, M2 untimed match |
| Shogi | `mshogi.shogi.csa36.v1` | 1 |
| Xiangqi | `mshogi.xiangqi.wxf2018.v1` | 1 |
| Tori Shogi | `mshogi.tori.modern.v1` | 3 |
| Chu Shogi | `mshogi.chu.jcsa2019.v1` | 2 |
| Tenjiku Shogi | `mshogi.tenjiku.adams1999-modern.v1` | 2 |
| Tenjiku Shogi | `mshogi.tenjiku.muller-completed.v1` | 2 |

Operational interruptions are session state. Cancellation, engine failure and
work limits add no game command and do not change a saved record. Current native
records have no Stop command or incomplete field. Tori schema-1 records retain
their promised replay path under the current repetition rule: old stop entries
are validated and omitted, while all player commands remain. Development-era
Tori schema 2 and Chu/Tenjiku schema 1 are unsupported under design A-09.

The edition is the exact native `RulesProfile::edition()` value. A different
whole profile rejects; no rule toggle migrates an existing document.

Chess stores an initial native GameRecord string and an ordered list of
accepted player commands, each with its actor. Native replay restores moves,
claims, draw offers and conclusions. A claim with an intended move does not
play that move. Untimed replay supplies deterministic request versions and
zero timestamps. Start, Observe, rejected requests, clocks and controller
identities are not saved commands. Timed, live-Full and witnessed resignation
restoration remain unsupported. Full analysis is a separate read-only operation.

The other games store their native canonical record. Their rules, complete
moves, drops, promotion decisions, effects and conclusions remain native data.
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
