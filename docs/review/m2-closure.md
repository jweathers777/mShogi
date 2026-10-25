# M2 search and match integration review

Verdict: the required M2 behavior is implemented and independently verified. Source `47243cdc9497cde81236cba93f6c042ab9ef20cf` contains the integrated rules/history, evaluator/search, cancellation, UCI and runner path. MSH-25 records the final closure integration and its native CI; later bookkeeping or this review document does not change that behavior. This review reuses the [M1 closure](m1-closure.md) and unchanged deep/oracle evidence rather than auditing the movement kernel again.

## Required deliverables and evidence

| Tasks | Delivered boundary | Integration evidence |
| --- | --- | --- |
| MSH-15 | Approved [software match profile](../rules/chess-match-v1.md) | Optional/current/intended claims, automatic outcomes and precedence, known-history imports, sound recognition/uncertainty, clocks, failure separation, replay and physical-procedure exclusions are explicit. Internal APIs remained implementation choices |
| MSH-16/17 | Rules-owned history, [records](../rules/chess-history-record-v1.md) and [live adjudication](../rules/chess-adjudication.md) | Exact repetition identity with legal EP/rights; imported counters and unknown prior history; apply/undo/replay; claims separate from conclusions; automatic/dead/checkmate precedence. The runner's known-history record exercises a claim before any engine move request (`go`) |
| MSH-18 | [Match commands and clocks](../rules/chess-match-api.md) | Injected untimed/increment clocks, offers and commands, ordered event versions and semantic replay. Recorded timed games replay actual balances; actual outstanding-search expiry remains unresolved/unscored when mating feasibility is unknown |
| MSH-19 | [Transparent evaluator](../evaluation.md) | Independent score/perspective/symmetry fixtures; static material does not adjudicate a game. No history dependency or engine-strength claim |
| MSH-20 | [Full-width search](../search.md) | Synthetic minimax and hand-solvable chess expectations, private game state, terminal-before-leaf behavior, same-position/different-history tests, optional zero-valued claims versus policy-forced claims, complete-iteration work limits |
| MSH-21 | [Native coordination](../search-coordination.md) | One worker and bounded pending/result slots, unique identity, exact record/version/settings, active cancellation, delayed/stale result rejection, ABA and history/mode preservation. Consumer acceptance and application stay serialized |
| MSH-22 | [UCI process](../uci.md) | Atomic history-aware position loading, protocol-only stdout, responsive readiness/stop/quit, work/basic clock controls and stale suppression. Both finite/infinite search failures terminate nonzero without fabricated success |
| MSH-23 | [Native runner](../match-runner.md) | Fresh paired UCI processes, executable/model/settings identities, claims and clocks owned by core, bounded supervision, strict replay and explicit unscored failure categories. Actual complete Stockfish19 and self-play games connect these components |
| MSH-24 | Structured rejection and input bounds | Existing core rejection, streaming/ASCII/size/atomicity fixtures and human-console process checks remain applicable. UCI and runner add their own bounded transport contracts |

All required tasks MSH-15–24 are Done. Existing identities, plans, decisions, review findings and evidence remain in Backlog.md. Component independent reviews resolved material findings before integration; they did not authorize weaker rules or substitute move legality for search correctness.

## Actual end-to-end experiments

The [experiment corpus](../../tests/fixtures/matches/README.md) contains eight reports with **21 games**: thirteen first-run games and eight repeated work-limited games. Its commands, configurations, engine/model fingerprints, UCI observations and canonical core match events are committed. Actual execution occurred on macOS 27.0.1 arm64 with Rust 1.89.0.

- Four first-run Stockfish19 games include a paired standard start, ending in rules-verified mates after 48/87 plies, plus a paired independently derived two-ply mating fixture. Both engines actually search; a toy subprocess is not the interoperability evidence.
- Four first-run self-play games include standard-start intended threefold claims and the forced ending. Known-history importing and an exercised claim are also separately recorded before any `go`.
- A timed increment pair preserves actual command/clock observations. A separate real outstanding `go depth 128 movetime 10000` search runs under a 50 ms chess clock: the controller records delayed flag detection, an explicitly unresolved/unscored outcome, no played move or bestmove response, and successful `quit` of both processes. Active cancellation and stale publication are independently tested through the native coordinator and real UCI process.
- A genuine descendant representation failure exits the mShogi engine with code 1 and remains an unscored experiment failure, separate from the clock case. Portable process fixtures additionally exercise start failure, invalid move, clean missing response, nonzero/idle exit, watchdog timeout, blocked writes, output limits and cleanup.

All 21 games replay from recorded observations without starting an engine. The eight untimed reruns have identical complete command histories, color assignments, engine identities and conclusions under the recorded deterministic settings and depth/node limits. Wall-clock observations and PIDs are excluded from that comparison. Timed configurations preserve their observations and make no identical-rerun promise.

## Fresh independent review and native evidence

Independent reviewer `m2_closure_review` inspected the actual implementations and required task records at `47243cd`, then reported **no material integration findings**. The reviewer independently passed 93 focused checks on Darwin arm64 / Rust 1.89.0:

```sh
cargo test -p mshogi-search -p mshogi-runner --locked --offline
cargo test -p mshogi-console --test uci --locked --offline
cargo test -p mshogi-core --test game_history --test game_record --test adjudication --test matchplay --test completeness --locked --offline
```

Those checks comprise 27 search tests plus one doctest, 18 runner checks, 11 UCI process tests and 36 focused core tests. The reviewer verified history/claims, private search and cancellation, protocol/controller integration, deterministic replay and honest resource/platform limitations. No unchanged M1 deep/perft/oracle run was repeated.

[Native CI run 40145526937](https://github.com/jweathers777/mShogi/actions/runs/40145526937) passed for the exact integrated source `47243cdc9497cde81236cba93f6c042ab9ef20cf`:

| Native host | Job | Actual result |
| --- | --- | --- |
| Ubuntu 24.04 | 121025908283 | Format, Clippy, locked workspace tests/build and console smoke passed |
| macOS 15 | 121025907952 | Same checks passed |
| Windows Server 2022 | 121025908161 | Same checks passed |

These jobs run real native UCI/fixture child processes and offline replay of the actual macOS reports. FIFO/non-UTF-8 boundary checks are Unix-only; Windows runs the eight portable runner process cases, four deterministic timing units and four evidence checks. Stockfish execution itself is macOS evidence; neither offline replay nor a configured matrix claims that the independent engine ran on the other hosts. The MSH-25 task retains the final closure revision/run in addition to this applicable source evidence.

The first closure-document run, [40145892269](https://github.com/jweathers777/mShogi/actions/runs/40145892269) at `a71e513`, passed macOS/Windows but exposed a Linux test-fixture race: both children were configured to exit, so the first observation could legitimately be the awaited EOF or the other child's idle exit. The test now faults only the awaited child and asserts its exact identity; a separate ready-then-idle-exit case preserves the other category. Independent review confirmed this strengthens the checks without changing production behavior. Twenty focused repetitions and the runner suite passed locally; final native verification is recorded in MSH-25. The existing actual-game corpus remains applicable and was not regenerated for this test-only change.

## Profile, limits and next work

The profile remains `mshogi.chess.match.v1` over `mshogi.chess.position.v1`. Core claims remain optional; the initial unattended runner accepts recorded AutoClaim and sends the matching option to mShogi search. Imported FEN starts known repetition history and preserves counters. Evaluation-based resignation/draw adjudication is disabled. Process failures, watchdogs, experiment limits and unresolved mating feasibility cannot invent a chess score. Physical tournament procedures remain outside this software profile.

The named M4 completeness task **MSH-26 is also Done**, implemented early because its dependencies were satisfied. Its [exact finite reachability algorithm](../rules/chess-completeness.md) supplies witnesses or exhaustive proofs, including independently checked blocked positions beyond the M2 subset. It assumes sufficient resources for unlimited completion; bounded or cancelled work remains Unknown. Default M2 behavior and legacy schema1 records remain unchanged. Full mode is explicit and recorded in schema2. This does not promise practical general-position latency or turn a failed bounded recognition into proof of mating possibility.

Cancellation and process timing are cooperative and depend on OS scheduling. Records preserve actual receipt/detection times separately from fixed deadlines; configuration checksums detect inconsistency rather than certify authenticity. UCI is a documented subset, with controller-owned claims and explicit unsupported features. The material/full-width baseline establishes inspectable correctness, not competitive strength. Packaged desktop behavior, the other five target games, learned evaluation and strength targets remain future roadmap work. None is silently presented as M2 delivery or as another dependency-ready task.
