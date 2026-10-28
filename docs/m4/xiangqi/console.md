# Native Xiangqi diagnostic console (MSH-96)

Run from the primary `codex/rewrite` checkout:

```
cargo run --locked --bin mshogi -- --game xiangqi profile
cargo run --locked --bin mshogi -- --game xiangqi shell
```

This is an independently usable native console for
`mshogi.xiangqi.wxf2018.v1`, not an external UCCI/UCI adapter or a new engine.
The [native consumer contract](native-consumer-contract.md) and all accepted
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
- Native input is complete origin/destination `a0a1`: files a–i from Red's left,
  ranks0–9 from Red's baseline. It has no promotion/drop/declaration fields.
  `show` prints rank9 first, uppercase Red/lowercase Black piece letters and river;
  the board is90 intersections. Diagnostic B means Elephant, N Horse; it does not
  rename the native kinds or select an external dialect.
- `position [explicit|auto_valid_claims] startpos|fen <FEN>|json <native-root-JSON>`
  starts a new known-history segment. Native FEN counters/provenance are preserved;
  neither FEN nor a structural diagram can restore earlier repetition/claim
  history. The native live-root validator rejects inadmissible diagrams.
- `fen`, `show`, `status`, `legal`, `history`, `warnings` inspect native state.
  Status preserves explicit conclusion/pending/fast-assessment data. History prints
  full native root provenance and event JSON, including same-ply warning/claim/
  conclusion events. `legal-local` is not permission to bypass game admission.
- `move <action>...` is one atomic batch: all native commands must admit and the
  actual final canonical envelope must fit before the session is replaced.
  A failed batch changes neither current game nor pending exact work.
- `eligibility red|black` is advisory. `claim`, `resign`, `offer`, `accept`,
  `decline` submit actual native commands with the specified actor. Claims use
  claimant-specific history and can return an admitted rejection audit without
  a board action or penalty. Turn admission remains native and command-specific.
- `undo [n]` reverses **native commands**, not arbitrary individual derived events.
  Claims, offers and resignation can consume a command while preserving board ply.
  Restored records retain undo across their entire known command history. Root
  automatic conclusions have no command handle. Runtime identity never rewinds.
- `perft <0..3>` / `divide <1..3>` are explicitly local legality diagnostics, ignoring
  match outcomes. Complete native actions/counts remain rules-owned.

The compact board follows the [common console presentation](../../console-presentation.md)
selected in MSH-110. It uses one-character cells separated by spaces; the river
and files/ranks stay native. The earlier MSH-96 prefixed-board evidence remains
historical. This presentation change does not alter FEN, action or record encoding.

## Canonical Save/Load and bounds

`record print` emits `record <complete-canonical-envelope-JSON>`; save/load-file
uses the same native wrapper. `record load-json` accepts it inline within the
4096-byte console line bound. Larger valid records must use `load-file` with the
existing1MiB encoded-envelope bound. Legacy `record` and `record load <path>`
remain print/file aliases for this game. FEN-only or foreign-game restoration,
unknown/duplicate/missing/conflicting fields and forged derived history reject
before replacement. Unsupported exact-mode restoration is not downgraded.

`record save <path>` atomically publishes a fully written new file and refuses
existing files/symlinks, reusing the established no-overwrite console helper.
Failed saves preserve the existing file and complete session. Session candidates
measure their actual envelope overhead before acceptance; no constant was raised
and no canonical history/audit is discarded. Native512-command/2048-event/1MiB
limits and reserved closure remain unchanged. Opening does not reset capacity.

The minimal native extension `GameRecord::to_game_with_undo` runs the same strict
bounded authoritative replay while optionally retaining native LIFO handles;
ordinary parser/`to_game` callers do not pay for retained undo history. Consumers
never reconstruct commands from warnings or derived events. `Game::claim_policy`
is a read-only getter for the actual recorded policy. Neither changes canonical
schema, rules, outcomes, default replay behavior or existing game compatibility.

Commands produce common tagged text/JSON on stdout, bounded escaped diagnostics
on stderr (`error: line N:`). Exit status0 means success,1 any shell error,2 invalid
program arguments. Oversized input is drained to LF, then later commands can run.
Native admission, resource refusal, audited claim rejection and execution failure
remain distinct. Internal invariant failures are explicit; no result is invented.

## Explicit exact diagnostics and execution ownership

```
nowin fast
nowin exact
nowin poll
nowin wait
nowin exact 5000 50000 1000 4194304 bfs
nowin cancel
```

Fast queries stay scoped native fast evidence. Exact defaults are the current
MSH-105 measured20kstates/200kedges/250ms/4MiB/4096witnessplies/DFS. Optional
numeric settings are states, edges, milliseconds, memory bytes and optional
`bfs|dfs`; the native verifier enforces its existing hard ceilings. Invalid
ceilings produce verifier Failure, exhaustion/cancellation Unknown, never a
made-up ongoing/winnable result or draw. No ordinary query/replay starts exact work.

One worker and one bounded result slot are owned by the session. `nowin exact`
replaces/retire-cancels earlier work; the host polls native cancellation between
bounded operations. `poll` is nonblocking. `wait` is an explicitly blocking
observation bounded by the submitted native limits, not a live progress stream.
`cancel`, successful position/record replacement, move/command, undo and quit
retire the request, signal cancellation and join the worker. Drop does the same.
No child process or unbounded queue is introduced. Polling an inactive request
is an explicit diagnostic error, not a synthetic cancellation observation.

Actual accepted output includes `nowin-context` with native purpose/goal/session/
generation/target/limits, `nowin-source` with the complete canonical source
record, and `nowin-result` with the actual native verdict/witness/work statistics.
Retain these together: `assessment()` alone would lose exact provenance.
Execution is read-only and full context checked; retired results cannot be
applied to a changed/equal/reopened game. Exact results never enter a move or
claim application handler.

These printed observations are bounded diagnostic evidence; `GameRecord` saves
the full game semantics, not pending exact work or a console analysis archive.
Reopening explicitly stops work. No unsupported promise to restore exact
observations as resumable work is made. If an experimental result genuinely
requires incomplete exact adjudication, it remains unresolved/unscored under
the approved MSH-84 policy. The console cannot commit a read-only proof as an
undocumented native conclusion. This is not general practical WXF completeness.

## Verification

Actual subprocess fixtures in `tests/xiangqi_cli.rs` exercise initial44actions,
perft/divide, atomic refusal, native third-warning/fourth-loss, claimant99/100
counts, admitted audits, agreement/resignation, canonical Save/Open and restored
undo, strict/bounded/UTF8/foreign-record rejection, exact255-state closure,
replayed2-ply natural witness, Unknown, cancellation/replacement/undo retirement
and oversized-line recovery. Existing common console tests now use a genuinely
unknown game spelling instead of rejecting the newly delivered Xiangqi selection.

Initial test expectations were corrected from native history-contract.md, not
from an outcome comparator: additional anchors can warn after root occurrence3;
draw offers follow the offering side's own move. A compile-time missing borrow
was fixed before verification. Independent review, actual full regression/CI,
source revisions and limitations are recorded in MSH-96 and the accompanying
review evidence. No MSH-97 evaluator/search or external protocol work is claimed.
