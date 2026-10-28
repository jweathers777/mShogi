# Xiangqi external protocol and bounded matches (MSH-100)

This consumer uses `mshogi.xiangqi.wxf2018.v1`, edition
`WXF-2018+XIANGQI-P1-2026-10-27`. Native rules, known history, warnings,
claimant-specific counts and canonical records remain authoritative. It does
not implement desktop UI, archives, new rules, exact verification or strength
measurement. MSH-98 is an independent native desktop-session consumer.

## Selected external dialect

The minimum delivered dialect is **`pikafish_uci0_9`**, pinned to the actually
built Pikafish `9d7833aa113626b3ea63ad83f49fb9b456bd5839`. The executable
`mshogi --game xiangqi uci` implements the compatible bounded untimed subset.
Bare `mshogi uci` remains Chess; Shogi still uses USI. Neither UCCI nor Fairy's
generic UCI1-10 dialect is advertised or silently converted.

- Files a–i, ranks0–9, Red at the bottom; complete actions such as `a3a4`.
- Six-field FEN: rank9 first, `k/a/b/n/r/c/p`, uppercase Red/lowercase Black;
  active `w` means **Red**, `b` Black; literal `- -`, declared no-capture
  counter and positive fullmove number. Native strict parsing rejects aliases,
  malformed/extra fields and invalid live-domain roots atomically.
- `position startpos [moves ...]` or `position fen <six fields> [moves ...]`
  supplies the entire known ordinary-action prefix. An imported FEN starts
  known history; supplied counters do not invent preceding actions/checks.
- `uci`, `setoption name Depth|Nodes|RulesProfile value ...`, `isready`,
  `ucinewgame`, `go [depth D] [nodes N] [infinite]`, `stop`, `quit`.
  Native default depth1/128 total visits and hard depth1..3/visits0..512 remain
  the measured MSH-97 bounds. Duplicate controls, unsupported clocks,
  pondering, searchmoves, mate/exact operations and profile changes reject
  explicitly. Zero work can finish without a decision; it never selects a
  first-legal fallback. `infinite` holds publication until `stop`; computation
  remains bounded by the selected finite native limits.
- Actual native score/perspective, completed depth, visits, elapsed time and
  board PV are reported. Native scores are soldier-hundredths or native
  terminal command-edge distance, not a falsely labeled Chess centipawn or
  mate score. A native result without a UCI action representation reports an
  explicit execution failure. `0000`/`(none)` are not silently mapped to a draw
  or resignation. The runner accepts an explicit `bestmove resign` proposal
  as a native participant resignation; mShogi does not choose heuristic
  resignations or evaluation-based draws.

[The unchanged source audit](comparator-audit.md) distinguishes Pikafish's
UCI0-9 from Fairy generic UCI1-10/UCCI0-9. Its planning-only verdict is
historical; [the MSH-100 admission evidence](protocol/opponent-admission.md)
records the new actual playing-engine build/network and runtime admission.
Pikafish's banner before `id` is accepted only during handshake, retained in
traffic, and bounded with all other handshake lines.

## Supervision and native match authority

`mshogi-match xiangqi-run CONFIG.json REPORT.json` is a separate native Xiangqi
runner. `xiangqi-replay REPORT.json` validates without launching any engine.
Configuration is strict schema1, selected profile/dialect, `untimed`,
`fresh_per_turn`, and **`auto_valid_claims`**. Every engine has explicit
executable/arguments/options/model identity and finite depth/visit limits.
Clocks and different claim policies reject rather than downgrade. UCI has no
optional native-claim channel; the native host exercises eligible claims under
this recorded policy. It does not infer claims or conclusions from engine
scores. Sound-fast-only root/current evidence stays in the native record;
a fast miss is not a winnability proof or an automatic draw.

Each opening supplies standard root or FEN plus complete ordinary actions, or
one raw-string canonical envelope. The latter reaches the strict authoritative
native parser without JSON-map normalization. A concluded native record can
contain terminal/claim/audit commands and needs no external engine. An ongoing
record containing same-ply non-board commands, or analysis-diagram provenance,
cannot be losslessly represented by this dialect and rejects explicitly; use
the existing structured native engine boundary for such contexts. No commands
or observations are silently discarded. Live imported-root counter bounds are
also restricted to pinned Pikafish's admitted no-capture0..119 and
fullmove1..100000; native records outside that subset remain valid natively and
are never normalized. All actual engine positions retain the root and entire
board prefix, not only the latest FEN. Warning and claim events are native
replay-derived and fully retained in the canonical game.

A fresh supervised child per turn provides the untagged UCI reply's process
and request generation. The controller captures native event-prefix version,
actor, complete canonical-prefix SHA-256, exact position/go commands and work.
It performs `uci`, validates advertised option names/values, readiness,
`ucinewgame` plus readiness, position and go. Before committing a proposal it
checks native admission and encoded canonical saveability, then retires the
generation, sends stop/quit, captures EOF, and checks bounded successful child
exit/pipe cleanup. Duplicate/late replies or post-reply crashes invalidate the
candidate. No older process can supply a later turn or a different game.

Native outcomes and experiment stops are distinct. Malformed/illegal replies,
missing response, timeout, process exit, configuration/policy mismatch,
resource exhaustion, incomplete work, unresolved work and actual engine
failure are unscored categories. Native pending adjudication stops unresolved
with its precise carrier. Action/time watchdog cutoffs are unfinished
experiments, never draws or losses. A successful native conclusion alone can
supply Red/Black half-point scores. Native resignation preserves board ply;
ordinary board actions alone validate the mover's turn. No implicit exact graph
runs in position parsing, commands, match control or replay. Explicit root-exact
analysis remains the unchanged native/internal facility from MSH-105/97;
positive existential witness is not forced-win evidence, and required exact
Unknown/cancellation/failure cannot be settled here by heuristics.

## Bounds, records and replay

The runner uses the existing supervised process transport unchanged: bounded
stdin writes/acknowledgements, stdout queue, escaped diagnostic capture,
watchdogs, quit/kill/reap and pipe cleanup. Config <=1MiB, at most4 openings,
paired assignments optional, <=32 accepted engine actions/game, <=600000ms
overall/game. Existing process validation applies; Xiangqi additionally caps
stdout line4096, queue32, transcript1MiB/game and stderr4096/child. Native
record envelope1MiB and report64MiB count actual encoded bytes, including
traffic and canonical envelope overhead. Retention failures pause before the
candidate action commits and preserve the accepted native prefix.

Reports retain configuration, executable/model file hashes, platform,
root/profile/full command/event history, warning/claim/conclusion/no-win
carriers, every accepted proposal's complete context identity, all retained
protocol commands/replies/timestamps/write results and process diagnostics/
exit/cleanup. Reports refuse overwrite at the CLI. Replay reconstructs the
opening and every native accepted command, validates ordered handshake/options/
readiness/position/go/reply/stop/quit evidence and clean process generation,
checks context/actor/limits and compares the final full canonical record and
native conclusion/pending state. Corrupt duplicate/unknown fields do not become
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
production binaries. Focused tests reuse independently established native
MSH-94 claimant/repetition and MSH-105/97 stalemate expectations; test fixtures
are not rules oracles. Source/review/CI revisions and actual commands are
recorded in MSH-100 and the evidence below.
