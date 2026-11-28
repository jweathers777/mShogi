# Xiangqi external protocol and bounded matches

This adapter uses `mshogi.xiangqi.wxf2018.v1`, edition
`WXF-2018+XIANGQI-P1-2026-10-27`. The rules library owns the known history,
warnings, claimant-specific counts and canonical records. The desktop uses
the [internal engine protocol](../../desktop/internal-execution-v4.md).

## Selected external dialect

The minimum delivered dialect is **`pikafish_uci0_9`**, pinned to the actually
built immutable-release Pikafish `4c17cee11f888ae1d48a9494f2e2239f019f0a1f`. The executable
`mshogi --game xiangqi uci` implements the compatible time- and work-limited subset.
Bare `mshogi uci` remains Chess; Shogi still uses USI. Neither UCCI nor Fairy's
generic UCI1-10 dialect is advertised or silently converted.

- Files a–i, ranks 0–9, Red at the bottom; complete actions such as `a3a4`.
- Six-field FEN: rank 9 first, `k/a/b/n/r/c/p`, uppercase Red/lowercase Black;
  active `w` means **Red**, `b` Black; literal `- -`, declared no-capture
  counter and positive fullmove number. Strict parsing rejects aliases,
  malformed/extra fields and invalid live-domain roots atomically.
- `position startpos [moves ...]` or `position fen <six fields> [moves ...]`
  supplies the entire known ordinary-action prefix. An imported FEN starts
  known history; supplied counters do not invent preceding actions/checks.
- `uci`, `setoption name Depth|Nodes|Hash|RulesProfile value ...`, `isready`,
  `ucinewgame`, `position`, `go`, `stop`, `quit`.
- A bare `go` uses one second. An explicit depth or node limit, including a
  configured option, selects fixed work. Fixed work defaults to depth 64 and
  300,000 nodes. The allowed range is depth 1–64 and 0–5,000,000 nodes.
  `go infinite` holds a bounded result until `stop`. Hash accepts 1–1,024 MiB
  and defaults to 64 MiB.
- `go movetime MS` accepts 1–60,000 ms. Clock play uses `wtime` for Red and
  `btime` for Black, optional increments `winc` and `binc`, and positive
  `movestogo` (default 30). Clock and move-time requests require time-based
  play. Pondering, searchmoves, mate searches and profile changes are unsupported.
- Actual reported score/perspective, completed depth, visits, elapsed time and
  board PV are reported. Reported scores are soldier-hundredths or
  terminal command-edge distance, not a falsely labeled Chess centipawn or
  mate score. A engine result without a UCI action representation reports an
  explicit execution failure. `0000`/`(none)` are not silently mapped to a draw
  or resignation. The runner accepts an explicit `bestmove resign` proposal
  as a participant resignation; mShogi does not choose heuristic
  resignations or evaluation-based draws.

Pikafish's banner before `id` is accepted during handshake. The runner
retains and bounds it with the other handshake lines.

## Supervision and match authority

`mshogi-match xiangqi-run CONFIG.json REPORT.json` is a separate Xiangqi
runner. `xiangqi-replay REPORT.json` validates without launching any engine.
Configuration is strict schema 1, selected profile/dialect, `untimed`,
`fresh_per_turn`, and **`auto_valid_claims`**. Every engine has explicit
executable/arguments/options/model identity and finite depth/visit limits.
Clocks and different claim policies reject rather than downgrade. UCI has no
optional claim channel; the match runner exercises eligible claims under
this recorded policy. It does not infer claims or conclusions from engine
scores. Sound-fast-only root/current evidence is restored by game replay;
a fast miss is not a winnability proof or an automatic draw.

Each opening supplies standard root or FEN plus complete ordinary actions, or
one raw-string canonical envelope. The latter reaches the strict authoritative
record parser without JSON-map normalization. A concluded game record can
contain terminal/claim/audit commands and needs no external engine. An ongoing
record containing same-ply non-board commands, or analysis-diagram provenance,
cannot be losslessly represented by this dialect and rejects explicitly; use
the existing structured mShogi engine boundary for such contexts. No commands
or observations are silently discarded. Live imported-root counter bounds are
also restricted to pinned Pikafish's admitted no-capture 0..119 and
fullmove 1..100000; game records outside that subset remain valid under the game rules and
are never normalized. All actual engine positions retain the root and entire
board prefix, not only the latest FEN. Warning and claim events are
replay-derived and fully retained in the canonical game.

A fresh supervised child per turn provides the untagged UCI reply's process
and request generation. The controller captures game event-prefix version,
actor, complete canonical-prefix SHA-256, exact position/go commands and work.
It performs `uci`, validates advertised option names/values, readiness,
`ucinewgame` plus readiness, position and go. Before committing a proposal it
checks game admission and encoded canonical saveability, then retires the
generation, sends stop/quit, captures EOF, and checks bounded successful child
exit/pipe cleanup. Duplicate/late replies or post-reply crashes invalidate the
candidate. No older process can supply a later turn or a different game.

Game outcomes and experiment stops are distinct. Malformed/illegal replies,
missing response, timeout, process exit, configuration/policy mismatch,
resource exhaustion, incomplete work, unresolved work and actual engine
failure are unscored categories. Pending adjudication stops unresolved
with its precise carrier. Action/time watchdog cutoffs are unfinished
experiments, never draws or losses. A successful game conclusion alone can
supply Red/Black half-point scores. Resignation preserves board ply;
ordinary board actions alone validate the mover's turn. The no-win assessment
is the sound fast recognition only; a positive existential witness is not
forced-win evidence, and fast Unknown cannot be settled here by heuristics.

## Bounds, records and replay

The runner uses the existing supervised process transport unchanged: bounded
stdin writes/acknowledgements, stdout queue, escaped diagnostic capture,
watchdogs, quit/kill/reap and pipe cleanup. Configuration metadata can use 1 MiB. Opening histories are excluded.
The operator can pair up to four openings, choose a positive action budget
and set the overall game watchdog up to six hours (21,600,000 ms). Stdout
lines can use 4096 bytes, the output queue holds 32 events, and a command
and its replies can use 1 MiB. Stderr can use 4096 bytes per child. A position
command carries the full history. The 128 MiB report budget counts all encoded traffic and
game records. Retention failures pause before the
candidate action commits and preserve the accepted prefix.

Reports retain configuration, executable/model file hashes, platform,
root/profile/full command/event history, warning/claim/conclusion/no-win
carriers, every accepted proposal's complete context identity, all retained
protocol commands/replies/timestamps/write results and process diagnostics/
exit/cleanup. Reports refuse overwrite at the CLI. Replay reconstructs the
opening and every accepted command, validates ordered handshake/options/
readiness/position/go/reply/stop/quit evidence and clean process generation,
checks context/actor/limits and compares the final full canonical record and
game conclusion/pending state. Corrupt duplicate/unknown fields do not become
normalized accepted records.

**Recorded games/events replay deterministically.** Rerun equality is narrower:
only documented deterministic engine settings and finite work permit a move/
semantic comparison; OS times, PIDs and diagnostics remain observations.
No identical rerun is promised for timed measurements. This bounded runner
rejects timed engine work entirely; watchdog and elapsed times remain recorded.
These short functional matches do not establish competitive playing strength.

## Launch and verification

```sh
cargo build --release --locked --bin mshogi --bin mshogi-match
./target/release/mshogi --game xiangqi uci
./target/release/mshogi-match xiangqi-run CONFIG.json REPORT.json
./target/release/mshogi-match xiangqi-replay REPORT.json
cargo test --locked -p mshogi-console --test xiangqi_uci
cargo test --locked -p mshogi-runner --test xiangqi_runner --test xiangqi_recorded
```

The deliberately named `mshogi-xiangqi-fixture` is a separate Cargo test child,
not a normal desktop/console distributable. It produces hostile/late/incomplete
replies for actual process tests. Normal packages still select only their
production binaries.
