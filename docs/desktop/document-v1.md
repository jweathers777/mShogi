# Desktop document format

The envelope is **`mshogi.desktop.document.v2`**. All six games save
`record`, `human` and `settings` in its payload. See
[design A-09](../design.md#a-09-pre-release-internal-compatibility) for the
pre-release compatibility policy.

## Envelope and game records

The envelope has three required fields: `format`, `game_kind` and `payload`.
Unknown and duplicate fields reject. The game tag selects the record parser. Each game record preserves its
rules identity. The original payload bytes reach the strict parser and replay.

| Game | Selected profile | Game record schema |
| --- | --- | --- |
| Chess | `mshogi.chess.match.v1` | Untimed match commands and an initial Chess game record |
| Shogi | `mshogi.shogi.csa36.v1` | 2 |
| Xiangqi | `mshogi.xiangqi.wxf2018.v1` | 2 |
| Tori Shogi | `mshogi.tori.modern.v1` | 4 |
| Chu Shogi | `mshogi.chu.jcsa2019.v1` | 3 |
| Tenjiku Shogi | `mshogi.tenjiku.adams1999-modern.v1` | 3 |
| Tenjiku Shogi | `mshogi.tenjiku.muller-completed.v1` | 3 |

Operational interruptions are session state. Cancellation, engine failure and
work limits add no game command. Rules effects and outcomes come from replay.

Tori, Chu and Tenjiku store the complete game root, edition, schema version
and ordered actor/command pairs. Chu roots include Lion protection. Tenjiku
roots contain the whole profile and original-piece rights. Effects, current
position, offers, obligations and conclusions are reconstructed by game replay.

Shogi schema 2 keeps the root category, SFEN and counter source, followed by
actor/command pairs. Xiangqi schema 2 keeps its complete PositionRoot, claim
policy and all actor/command pairs, including admitted rejected claims. Game
replay restores declaration evidence, warnings, claim results and conclusions.

The document stores each non-Chess game record directly. Engine contexts use
a separate canonical wrapper with `envelope_version: 2`, `game_kind` and `payload`.
Chess initial history can carry its adjudication settings.

The edition is the exact `RulesProfile::edition()` value. A different
whole profile rejects; no rule toggle migrates an existing document.

Chess stores an initial GameRecord string and an ordered list of
accepted player commands, each with its actor. Game replay restores moves,
claims, draw offers and conclusions. A claim with an intended move does not
play that move. Untimed replay supplies deterministic request versions and
zero timestamps. Start, Observe, rejected requests, clocks and controller
identities are not saved commands. Timed and witnessed resignation
restoration remain unsupported.

Settings have the same shape in every game: `play_mode` (`human_engine` or
`two_humans`), `orientation` (the game side at the bottom) and
`analysis_defaults` (`depth`, `nodes`), which Analyze updates. Display
preferences cannot change ownership during a game.

## Session state and history

Open restores the live game with fresh runtime identities and stopped play.
The history selection, pending work, diagnostics and engine observations belong
to the open session. Take back and Undo cut the command list at a command
boundary. A surviving document can retain an immutable imported root. See the
[session contract](native-application.md#history-and-execution) and
[Take back](take-back.md).

## Resource and file guarantees

Documents and game records have no byte, event or game-length limit
(MSH-268). No history is truncated. Compact game command admission uses the
[root and command bounds](../game-resource-admission.md); it does not
serialize hypothetical endings. A very large or damaged file can use much
memory; validation stays cancellable.

Validation runs on a bounded, cancellable worker with a 15-second deadline.
It returns the already replayed game for guarded installation. A stale,
invalid or cancelled candidate cannot replace the accepted game.
Save uses a same-directory temporary file, flush/sync and atomic replacement.
Failure preserves the prior valid file and dirty state. New and Open replace
only after valid atomic admission. Quit retains the Save/Discard/Cancel guard
for never-saved or dirty documents.

## Verification fixtures

See [desktop fixture regeneration](../../desktop/src/__fixtures__/README.md)
for the current frontend snapshots and ordinary Load documents.
