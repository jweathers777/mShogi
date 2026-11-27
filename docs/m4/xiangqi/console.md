# Xiangqi diagnostic console (MSH-96)

Run from the primary `codex/rewrite` checkout:

```
cargo run --locked --bin mshogi -- --game xiangqi profile
cargo run --locked --bin mshogi -- --game xiangqi shell
```

This is an independently usable console for
`mshogi.xiangqi.wxf2018.v1`, not an external UCCI/UCI adapter or a new engine.
The [consumer contract](rules-consumer-contract.md) and all accepted
rules/history/record/proof semantics remain authoritative. MSH-97 owns evaluator,
search and actual structured engine execution. MSH-100 owns external protocols
and match consumers; MSH-45/M4 remain open.

## Commands and notation

`help` prints the actual commands. Example:

```
position startpos
show
legal
move a3a4 a6a5
history
undo 2
record print
record save /tmp/xiangqi-new.json
record load-file /tmp/xiangqi-new.json
quit
```

- Explicit `--game xiangqi` selects the console/profile. Chess remains the default;
  existing Chess/Shogi aliases and UCI/USI modes are preserved. Xiangqi is refused
  by those external protocol modes rather than routed to another game's engine.
- Input is complete origin/destination `a0a1`: files a–i from Red's left,
  ranks 0–9 from Red's baseline. It has no promotion/drop/declaration fields.
  `show` prints rank 9 first, uppercase Red/lowercase Black piece letters and river;
  the board is 90 intersections. Diagnostic B means Elephant, N Horse; it does not
  rename the kinds or select an external dialect.
- `position [explicit|auto_valid_claims] startpos|fen <FEN>|json <root-JSON>`
  starts a new known-history segment. FEN counters/provenance are preserved;
  neither FEN nor a structural diagram can restore earlier repetition/claim
  history. The live-root validator rejects inadmissible diagrams.
- `fen`, `show`, `status`, `legal`, `history`, `warnings` inspect game state.
  Status preserves explicit conclusion/pending/fast-assessment data. History prints
  full game root provenance and event JSON, including same-ply warning/claim/
  conclusion events. `legal-local` is not permission to bypass game admission.
- `move <action>...` is one atomic batch: all game commands must admit and the
  actual final canonical envelope must fit before the session is replaced.
  A failed batch does not change the current game.
- `eligibility red|black` is advisory. `claim`, `resign`, `offer-draw`,
  `accept-draw`, `decline-draw` submit actual game commands with the specified actor. Claims use
  claimant-specific history and can return an admitted rejection audit without
  a board action or penalty. Turn admission remains and command-specific.
- `undo [n]` reverses **game commands**, not arbitrary individual derived events.
  Claims, offers and resignation can consume a command while preserving board ply.
  Restored records retain undo across their entire known command history. Root
  automatic conclusions have no command handle. Runtime identity never rewinds.
- `perft <0..3>` / `divide <1..3>` are explicitly local legality diagnostics, ignoring
  match outcomes. Complete actions/counts remain rules-owned.

The compact board follows the [common console presentation](../../console-presentation.md)
selected in MSH-110. It uses one-character cells separated by spaces; the river
and files/ranks stay. The earlier MSH-96 prefixed-board evidence remains
historical. This presentation change does not alter FEN, action or record encoding.

## Canonical Save/Load and bounds

`record print` emits `record <complete-canonical-envelope-JSON>`; save/load-file
uses the same wrapper. `record load-json` accepts it inline;
`load-file` reads it from a file. Neither has a byte limit (MSH-268). FEN-only or foreign-game restoration,
unknown/duplicate/missing/conflicting fields and forged derived history reject
before replacement.

`record save <path>` atomically publishes a fully written new file and refuses
existing files/symlinks, reusing the established no-overwrite console helper.
Failed saves preserve the existing file and complete session. No canonical
history/audit is discarded. Game records have no command, event or byte limit
(MSH-268).

The minimal extension `GameRecord::to_game_with_undo` runs the same strict
bounded authoritative replay while optionally retaining LIFO handles;
ordinary parser/`to_game` callers do not pay for retained undo history. Consumers
never reconstruct commands from warnings or derived events. `Game::claim_policy`
is a read-only getter for the actual recorded policy. Neither changes canonical
schema, rules, outcomes, default replay behavior or existing game compatibility.

Commands produce common tagged text/JSON on stdout, bounded escaped diagnostics
on stderr (`error: line N:`). Exit status 0 means success,1 any shell error,2 invalid
program arguments. Input lines have no byte limit.
Game admission, resource refusal, audited claim rejection and execution failure
remain distinct. Internal invariant failures are explicit; no result is invented.

## No-win assessment

```
nowin fast
```

`nowin fast` prints the scoped fast assessment of the current position;
`status` prints the same value. `unknown` is a fast miss, not a possible win or
a draw. There is no exact no-win analysis. This is not general practical WXF
completeness.
