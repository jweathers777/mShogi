# Native shogi USI and untimed experiments (MSH-71)

This implements the selected `mshogi.shogi.csa36.v1` profile's native engine
through a separate USI process. Native game/history, complete legal actions and
MSH-70 search remain authoritative. The human consoles, shared canonical record
commands, compact board, chess UCI and structured v2/v3 entry point keep their
existing contracts. Shogi desktop sessions and archives remain MSH-72 onward;
functional interoperability makes no MSH-50 strength claim.

Primary protocol references are the [Shogidokoro specification](https://shogidokoro2.stars.ne.jp/usi.html)
and [original USI specification with depth/node extensions](https://hgm.nubati.net/usi.html).
These define transport meanings, not a substitute rules profile.

## Launch and supported subset

From the repository root:

```sh
cargo build --locked --release --bin mshogi --bin mshogi-match
./target/release/mshogi --game shogi usi
```

Bare `mshogi usi` selects shogi. Explicit `--game chess usi` rejects with exit2
and stderr only. Human help/boards never appear on protocol stdout. `internal-search`
continues to take its game from the structured request and rejects `--game`.

| Command | Actual behavior |
| --- | --- |
| `usi` | `id`, actual supported `option` lines, `usiok` |
| `isready` | Immediate `readyok`, including while searching or holding a result |
| `setoption name Depth value N` | Default2; supported1–4 |
| `setoption name Nodes value N` | Default5000; supported0–20000 |
| `setoption name RulesProfile value mshogi.shogi.csa36.v1` | Only this immutable profile is accepted |
| `usinewgame` | Retire work and reset the native standard-root game; retain options |
| `position startpos [moves ...]` | Standard root plus the entire supplied known sequence |
| `position sfen <four fields> [moves ...]` | Imported root/counter plus complete supplied suffix |
| `go [depth N] [nodes N]` | Measured MSH-70 defaults for omitted limits; finite native search |
| `go infinite [depth N] [nodes N]` | Same finite work, then hold its real result until `stop` |
| `stop` | Cooperatively stop current work; publish its legitimate last complete iteration if available |
| `gameover win\|lose\|draw` | Retire work and mark protocol game ended; no native adjudication |
| `quit` / EOF | Retire work and terminate, suppressing pending bestmove |
| `go mate ...` | `checkmate notimplemented`; no tsume or Full operation |

Clock controls, movetime, pondering/ponderhit and searchmoves are unsupported.
Invalid go controls produce a bounded stderr diagnostic, no bestmove, and leave
the adapter available for a corrected request. An external controller must not
wait for an unsupported operation. Duplicate/unknown controls reject explicitly;
unknown option/value diagnostics never change the profile. Valid option changes
retire old work. Unknown option commands do not alter current work or settings.

ASCII CR/LF input lines are bounded at4096 bytes with bounded draining/recovery.
There are32 pending input events. One persistent native worker has one replaceable
pending job and one completion slot. No unbounded progress stream is created.
Every accepted search uses full native context, exact work limits, monotonic
request identity and session version. The process itself is the execution generation.

## History, lifetime and honest result mapping

SFEN's declared counter is retained. It cannot establish repetitions/checks
before import. SFEN material classification is derived and then checked by
native `GameRoot::import`: all40 pieces means full diagram, otherwise reduced.
Neither label proves historical reachability. Even the initial diagram imported
as SFEN remains an imported root, distinct from `startpos` complete history.
Every supplied move/drop/promotion is parsed and admitted by the native game;
replacement is all-or-none. As a safety rule, a position/go command retires old
work even if the requested replacement is invalid; the previous game survives.

`stop` preserves current request identity and completed iterations. Position/go
replacement, new-game, accepted option change, gameover and quit retire identity
and publication. Old pending/held/completed work cannot become current again.
`go infinite` holds a bounded completed observation rather than claiming continual
search or live node telemetry. It remains independently terminable by stop/quit.

Only actual observations are emitted: completed depth, admitted nodes, elapsed
request time, root-side perspective and completed PV. Small-valued native scores
(including a searched draw value0) use `score cp`, pawn100. Decisive native scores
are emitted only in `info string mshogi native_score ...`: they may mean entering
king or continuous-check loss rather than checkmate. Distance counts native
command edges and is **not** reported as USI `score mate`. PV contains only the
board-action prefix, stopping before any declaration; `win` is never a PV move.

A completed native board/declaration proposal is re-admitted privately against
the exact current game before `bestmove <USI-action>` or `bestmove win` is published.
The adapter does not apply its suggestion to the protocol position. `bestmove win`
means a formal declaration; the host must admit it, including an admitted false
declaration loss. It is not an advisory eligibility query. No evaluation-based
resignation/draw policy was added. An already checkmated root returns the specified
`bestmove resign`, acknowledging its native loss. Other already concluded roots
(draw, non-check no-action, or history outcome) cannot be generically encoded as
resignation/win. They emit a native-terminal diagnostic and terminate without a
fabricated bestmove; a native host checks conclusions before requesting search.

USI has no standard unscored no-result bestmove. Zero/insufficient/stopped work
without a complete iteration, or native failure, emits bounded
`info string mshogi execution_failure ...` plus stderr, then exits1. It never
substitutes resignation, declaration, score or a lexically chosen legal move.
Completed iterations survive later node exhaustion/stop. `info` and `gameover`
are observations/notifications; neither overrides native outcomes or awards points.

## Native experiment host

```sh
./target/release/mshogi-match shogi-run docs/m4/usi/selfplay.config.json /tmp/msh71-selfplay-new.json
./target/release/mshogi-match shogi-replay /tmp/msh71-selfplay-new.json
./target/release/mshogi-match shogi-run docs/m4/usi/opponent.config.json /tmp/msh71-opponent-new.json
./target/release/mshogi-match shogi-replay /tmp/msh71-opponent-new.json
```

Choose fresh report paths; existing files are never overwritten. CLI exit0 means
a report was produced/replayed, not that every experiment finished or scored.
Each printed/report entry names native conclusion, experiment stop and optional
score separately. A failure report is useful output, not a chess result.

The explicit `fresh_per_turn` policy restarts a child for each requested engine
turn. This costs handshake/initialization work but bounds identity: USI replies
lack request IDs, and a retired child can never answer a later request. Each new
child receives `usi`, only options it actually advertised with validated types/
ranges/combo values, `isready`, `usinewgame`, root+complete moves, then finite go.
The captured game context, actor, known event prefix, work and process generation
are checked before native admission. A well-formed false `win` is an admitted
native false-declaration loss. Malformed reply, illegal action, missing output,
nonzero exit, timeout, late/duplicate reply, resource exhaustion and failed
cleanup are unscored failures. Successful candidate acceptance waits for bounded
retirement/cleanup; a post-reply process failure cannot commit the candidate.
`stop`/`quit` retire the process, and further output is recorded as retired output.
A native conclusion alone produces the appropriate `gameover` notice.

Experiments are untimed: work bounds and wall-clock process/experiment watchdogs
are not chess clocks. The action limit1–32 and overall time limit stop unfinished
experiments without awarding a draw or loss. Overall deadlines also cap reads
and writes; cleanup has a separate total deadline including commands/draining/
kill/reap. Native CSA length320 is unchanged. Up to4 openings may be paired;
configuration input is1MiB, native envelopes1MiB, protocol lines/commands4096,
queued output32, transcript retention at most1MiB/game, stderr4096/child and
report input/output64MiB. These caps account for serialized event overhead.
Resource failures stop admission. Incomplete/oversized/failed retirement capture
sets `traffic_complete=false`; stderr truncation is separately explicit. Normal
retained runs have complete traffic. No accepted native history is dropped.

Reports retain executable/model SHA256, engine options and actual advertisements,
root provenance, complete protocol commands/responses, work, observations,
process generations/PIDs/exits/cleanup, accepted replies and full canonical games.
Native full-prefix digests identify history, not merely board position. Offline
replay reconstructs each opening and native command, verifies turn context/limits
against actual command/response/process evidence, and checks exact canonical game,
conclusion, stop and score. Native duplicates remain visible to strict raw parsers.
Scores are half-point units ordered Sente/Gote: win[2,0]/[0,2], draw[1,1]. Failed
or unfinished experiments have no competition score. No administrative foul or
engine-failure penalty is silently invented.

## Admitted external opponent and actual runs

The exact candidate was admitted, without replacement:

- YaneuraOu `0f421391ae311df9df57dcfcb2b4a78a1972bdf8`, source tree
  `0844b429c4fb89e0c4f9160aa65447dcc17bcd58`.
- Unmodified materiallevel1, native APPLEM1, Apple Clang21, macOS27.0.1 arm64.
- Actual identity `YaneuraOu MaterialLv1 9.80git 64APPLEM1`.
- Binary SHA256 `74eded93711841dbdef41c0d38c8b85de9af77b9925274269c7b42a6d3987f86`.
- GPLv3 external verification process; no opponent implementation or binary is bundled.

Rebuild at a fresh path matching the supplied opponent config:

```sh
git clone https://github.com/yaneurao/YaneuraOu.git /private/tmp/msh71-yaneuraou
git -C /private/tmp/msh71-yaneuraou checkout --detach 0f421391ae311df9df57dcfcb2b4a78a1972bdf8
make -C /private/tmp/msh71-yaneuraou/source -j4 YANEURAOU_EDITION=YANEURAOU_ENGINE_MATERIAL MATERIAL_LEVEL=1 TARGET_CPU=APPLEM1 COMPILER=clang++
```

[Admission](usi/opponent-admission.json) retains compiler/build flags, license hash,
actual complete handshake/options and readiness/search traffic. The initial
sandbox DNS-denied fetch and successful authorized fetch are retained; the
unmodified build succeeded first attempt with an obsolete linker `-s` warning.
[Supplement](usi/opponent-admission-supplement.json) records additional controls,
actual stop/quit and the known valid declaration fixture. An admission-only
`gameover draw` probe is explicitly a notice, not an adjudicated game.

Every configured setting was actually advertised: Threads1, USI_Hash64,
USI_Ponder=false, Stochastic_Ponder=false, MultiPV1, USI_OwnBook=false,
BookFile=no_book, BookMoves0, GenerateAllLegalMoves=true, EnteringKingRule=CSARule27,
MaxMovesToDraw0, ResignValue99999, DrawValueBlack/White0. MaxMovesToDraw0 uses an
upstream100000 sentinel; mShogi still owns length320 and all conclusions. The
material opponent uses no external learned network. Ponder suffixes may be
observed despite pondering disabled; they are syntax-checked predictions, never
accepted board actions. Full legal-action conformance remains MSH-64/66 evidence.

The final [self-play report](usi/selfplay.report.json) and
[opponent report](usi/opponent.report.json) each contain6 actual games:

| Opening, paired engine assignments | Self-play | External opponent |
| --- | --- | --- |
| Standard root,8-command experiment cap | 2 unfinished, unscored | 2 unfinished, unscored |
| Independently expected mating Gold drop | 2 native Sente no-action/checkmate wins | 2 native Sente no-action/checkmate wins |
| Independently expected CSA28 declaration | 2 native Sente entering-king wins | 2 native Sente entering-king wins |

These artificial decisive roots test both engine assignments; they are not fair
strength comparisons. The native [repeat run](usi/selfplay-repeat.report.json)
reproduced all6 canonical games under the recorded deterministic finite settings.
Traffic timestamps/PIDs/elapsed time are observations and differ. No identical
fresh external-engine rerun is promised: source draw jitter/hash/search details
can affect external decisions even with controlled options. Deterministic native
record replay is a separate claim and passed on all retained reports.

[Zero-work report](usi/no-result.report.json) retains2 actual native-USI failures,
with no move/score/conclusion. [Late-reply report](usi/late-fixture.report.json)
retains2 separately labeled injected watchdog/retired-reply failures from the
verification executable; it is excluded from normal packages. Earlier exploratory
success reports remain under `usi/exploratory/`. The
[execution manifest](usi/execution-manifest.json) identifies actual source-file,
engine/runner/fixture binary and report hashes, environment and scope. These
opponent/experiment executions occurred on macOS arm64 only. Three-platform CI
executes product/fixtures and packages; it does not imply three-platform opponent
execution. Exact integrated revisions, CI and independent review are in MSH-71.

## Independent review findings and retained regression evidence

The protocol/fixture audit used primary specifications and accepted native
fixtures before implementation. The separate implementation reviewer rebuilt
and executed real subprocesses, rather than treating an author reread as review.
It found two material boundary defects, both corrected with actual-process tests:

1. An accepted trailing TAB was preserved raw in the result but escaped in the
   transcript. Replay now compares the escaped representation correctly while
   retaining the exact raw reply.
2. A duplicate reply could arrive after the retirement drain but before a clean
   child exit. Acceptance now requires observed retirement EOF within the capture
   budget; incomplete capture is an explicit unscored resource failure even if
   the child subsequently exits0. The [injected report](usi/retirement-fixture.report.json)
   retains2 rejected candidates with their native games unchanged.

The independent [pre-fix reproduction](usi/review-findings/cleanup-before-fix.json)
and [external probe](usi/review-findings/cleanup-probe.py) preserve the failed check.
That old full experiment report is deliberately rejected by final replay; its
underlying accepted native record remains inspectable. It is a regression
counterexample, not current successful experiment evidence. All8 supported
`.report.json` files replay offline (36 games including exploratory and labeled
failures). CI exercises that replay without launching the external opponent.
An early stop test's machine-speed assumption was also corrected: stopping before
a full iteration legitimately yields incomplete work, not an expected move.

Normal package inspection already rejects paths containing fixture/test-driver
markers. The new `mshogi-usi-fixture` is a separate verification executable and
is never selected as a bundled production engine.
