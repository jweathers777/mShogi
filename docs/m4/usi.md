# Shogi USI and untimed experiments

The USI adapter uses the selected `mshogi.shogi.csa36.v1` profile.
The rules library admits the full supplied game history and complete actions.
The desktop uses the [internal engine protocol](../desktop/internal-execution-v4.md).

Primary protocol references are the [Shogidokoro specification](https://shogidokoro2.stars.ne.jp/usi.html)
and [original USI specification with depth/node extensions](https://hgm.nubati.net/usi.html).
These define transport meanings, not a substitute rules profile.

## Launch and supported subset

From the repository root:

```sh
cargo build --locked --release --bin mshogi --bin mshogi-match
./target/release/mshogi --game shogi usi
```

Bare `mshogi usi` selects shogi. Explicit `--game chess usi` rejects with exit 2
and stderr only. Human help/boards never appear on protocol stdout. `internal-search`
continues to take its game from the structured request and rejects `--game`.

| Command | Actual behavior |
| --- | --- |
| `usi` | `id`, actual supported `option` lines, `usiok` |
| `isready` | Immediate `readyok`, including while searching or holding a result |
| `setoption name Depth value N` | Default 64, range 1–64 |
| `setoption name Nodes value N` | Default 300,000, range 0–5,000,000 |
| `setoption name Hash value N` | Search table in MiB, default 16, range 1–1,024 |
| `setoption name RulesProfile value mshogi.shogi.csa36.v1` | Only this immutable profile is accepted |
| `usinewgame` | Retire work and reset the standard-root game; retain options |
| `position startpos [moves ...]` | Standard root plus the entire supplied known sequence |
| `position sfen <four fields> [moves ...]` | Imported root/counter plus complete supplied suffix |
| `go [depth N] [nodes N]` | A bare request uses one second. An explicit depth or node limit, including a configured option, selects fixed work. |
| `go infinite [depth N] [nodes N]` | Same finite work, then hold its real result until `stop` |
| `stop` | Cooperatively stop current work; publish its legitimate last complete iteration if available |
| `gameover win\|lose\|draw` | Retire work and mark protocol game ended; no adjudication |
| `quit` / EOF | Retire work and terminate, suppressing pending bestmove |
| `go mate ...` | `checkmate notimplemented`; no tsume or Full operation |

Use `go movetime MS` for 1–60,000 ms, or give the moving side's clock with
`btime` (Sente) or `wtime` (Gote). The clock accepts `binc`, `winc`,
`byoyomi` and positive `movestogo` (default 30). Clock and move-time requests
require time-based play. The allocator currently treats byoyomi as an increment and caps spending at
remaining main time. With zero main time it assigns zero thinking time (MSH-275).
Pondering, ponderhit and searchmoves are unsupported.
Invalid go controls produce a bounded stderr diagnostic, no bestmove, and leave
the adapter available for a corrected request. An external controller must not
wait for an unsupported operation. Duplicate/unknown controls reject explicitly;
unknown option/value diagnostics never change the profile. Valid option changes
retire old work. Unknown option commands do not alter current work or settings.

ASCII CR/LF input lines have no byte limit, so a long position command is
accepted (MSH-268). There are 32 pending input events. One persistent worker has one replaceable
pending job and one completion slot. No unbounded progress stream is created.
Each result must match the active request, including its position, time and work limits,
table size and game generation. A new game, position or search retires the old request.

## History, lifetime and honest result mapping

SFEN's declared counter is retained. It cannot establish repetitions/checks
before import. SFEN material classification is derived and then checked by
`GameRoot::import`: all 40 pieces means full diagram, otherwise reduced.
Neither label proves historical reachability. Even the initial diagram imported
as SFEN remains an imported root, distinct from `startpos` complete history.
Every supplied move/drop/promotion is parsed and admitted by the game;
replacement is all-or-none. As a safety rule, a position/go command retires old
work even if the requested replacement is invalid; the previous game survives.

`stop` preserves current request identity and completed iterations. Position/go
replacement, new-game, accepted option change, gameover and quit retire identity
and publication. Old pending/held/completed work cannot become current again.
`go infinite` holds a bounded completed observation rather than claiming continual
search or live node telemetry. It remains independently terminable by stop/quit.

Only actual observations are emitted: completed depth, admitted nodes, elapsed
request time, root-side perspective and completed PV. Small-valued reported scores
(including a searched draw value 0) use `score cp`, pawn 100. Decisive reported scores
are emitted only in `info string mshogi reported_score ...`: they may mean entering
king or continuous-check loss rather than checkmate. Distance counts
command edges and is **not** reported as USI `score mate`. PV contains only the
board-action prefix, stopping before any declaration; `win` is never a PV move.

A completed game board/declaration proposal is re-admitted privately against
the exact current game before `bestmove <USI-action>` or `bestmove win` is published.
The adapter does not apply its suggestion to the protocol position. `bestmove win`
means a formal declaration; the host must admit it, including an admitted false
declaration loss. It is not an advisory eligibility query. No evaluation-based
resignation/draw policy was added. An already checkmated root returns the specified
`bestmove resign`, acknowledging its loss. Other already concluded roots
(draw, non-check no-action, or history outcome) cannot be generically encoded as
resignation/win. They emit a terminal diagnostic and terminate without a
fabricated bestmove; a desktop host checks conclusions before requesting search.

USI has no standard unscored no-result bestmove. Zero/insufficient/stopped work
without a complete iteration, or failure, emits bounded
`info string mshogi execution_failure ...` plus stderr, then exits 1. It never
substitutes resignation, declaration, score or a lexically chosen legal move.
Completed iterations survive later node exhaustion/stop. `info` and `gameover`
are observations/notifications; neither overrides game outcomes or awards points.

## Experiment host

```sh
./target/release/mshogi-match shogi-run CONFIG.json REPORT.json
./target/release/mshogi-match shogi-replay REPORT.json
```

Choose fresh report paths; existing files are never overwritten. CLI exit 0 means
a report was produced/replayed, not that every experiment finished or scored.
Each printed/report entry names game conclusion, experiment stop and optional
score separately. A failure report is useful output, not a chess result.

The explicit `fresh_per_turn` policy restarts a child for each requested engine
turn. This costs handshake/initialization work but bounds identity: USI replies
lack request IDs, and a retired child can never answer a later request. Each new
child receives `usi`, only options it actually advertised with validated types/
ranges/combo values, `isready`, `usinewgame`, root+complete moves, then finite go.
The captured game context, actor, known event prefix, work and process generation
are checked before game admission. A well-formed false `win` is an admitted
false-declaration loss. Malformed reply, illegal action, missing output,
nonzero exit, timeout, late/duplicate reply, resource exhaustion and failed
cleanup are unscored failures. Successful candidate acceptance waits for bounded
retirement/cleanup; a post-reply process failure cannot commit the candidate.
`stop`/`quit` retire the process, and further output is recorded as retired output.
A game conclusion alone produces the appropriate `gameover` notice.

Experiments are untimed: work bounds and wall-clock process/experiment watchdogs
are not chess clocks. The operator-selected positive action budget and overall time limit stop unfinished
experiments without awarding a draw or loss. Overall deadlines also cap reads
and writes; cleanup has a separate total deadline including commands/draining/
kill/reap. CSA length 320 is unchanged. Up to 4 openings may be paired;
configuration metadata can use 1 MiB, excluding opening histories. A stdout
line or control command can use 4096 bytes. The queue holds 32 output events.
A command and its replies can use 1 MiB including event overhead. Position
commands carry the full history. Stderr can use 4096 bytes per child. The
128 MiB report budget covers all games and their evidence.
Resource failures stop admission. Incomplete/oversized/failed retirement capture
sets `traffic_complete=false`; stderr truncation is separately explicit. No accepted game history is dropped.

Reports retain executable/model SHA256, engine options and actual advertisements,
root provenance, complete protocol commands/responses, work, observations,
process generations/PIDs/exits/cleanup, accepted replies and full canonical games.
Full-prefix digests identify history, not merely board position. Offline
replay reconstructs each opening and game command, verifies turn context/limits
against actual command/response/process evidence, and checks exact canonical game,
conclusion, stop and score. Duplicates remain visible to strict raw parsers.
Scores are half-point units ordered Sente/Gote: win[2,0]/[0,2], draw[1,1]. Failed
or unfinished experiments have no competition score. No administrative foul or
engine-failure penalty is silently invented.
