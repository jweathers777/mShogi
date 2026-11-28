# Reproducible experiments (MSH-48)

Current use follows the [selected strength roadmap](strength-roadmap.md)
and AGENTS.md: engineering tests/tactics/profiling and small development matches
support coherent conventional-engine implementation. Routine local matches have
a 15-minute cumulative compute allowance per task; substantial/formal studies and
multi-hour benchmarks require explicit owner approval at major engine/evaluation
milestones. These tools and historical pilots are not instructions to run a
formal study after every technique. The earlier MSH-49 depth-handicapped study
is stopped; preserve its evidence rather than resume it.

The existing `mshogi-match` runner now supports offline experiments with mShogi's
actual supervised internal-search executable. It does not add a second rules
engine, engine protocol, application runtime or Python dependency. The concrete
Game owners admit every complete action. Chess additionally retains the
untimed M2 MatchRecord; Full and timed internal matches are explicitly refused.

## Running and replaying

Build with the pinned stable toolchain, recording source, clean/dirty status,
compiler, target, flags and build command. Use an absolute executable path:

```sh
cargo build --release --locked -p mshogi-console --bin mshogi -p mshogi-runner --bins --examples
target/release/mshogi-match experiment-template /absolute/path/mshogi config.json
target/release/mshogi-match experiment-run config.json report.json
target/release/mshogi-match experiment-replay report.json
target/release/mshogi-match experiment-data report.json data.jsonl
```

Outputs refuse to
overwrite existing files. Encoding/validation finishes before creating output;
write failure removes only that newly created partial file. This is not an atomic
publication guarantee. `experiment_pilot` generates the committed diagnostic root
catalog; it runs no engines. Edit the generated config before executing a study.

The strict version 1 config names candidate/player0 and one to three baselines,
absolute executable paths, caller-declared build provenance, effective depth and
node limits per whole profile, complete openings, source/rights rationale,
lineage groups and Train/Validation/Test splits. It records the question and
hardware conditions in `protocol`. Observed executable SHA256, OS, architecture,
logical parallelism and harness build/compiler/target/flags accompany the report.
CPU model, load, power policy and participant build receipts must be supplied and
verified separately. Declared provenance and hashes are not authentication. The
build dirty fingerprint covers tracked Rust source changes; commit all sources
before reproducible acceptance runs, including newly created files.

Internal protocol4 accepts depth/nodes and complete history, not arbitrary
arguments, model files, options or a search seed. Those config fields are rejected.
Internal model identity is explicitly `executable-embedded`; evaluator/search IDs
where the existing protocol supplies them stay in actual telemetry. Chess's
older telemetry has no separate evaluator ID. Effective executable identity and
build receipt therefore identify its embedded evaluator. No model loading or
learned evaluator is claimed here; that lifecycle belongs to MSH-55.

The seed selects a SHA256 counter/rejection permutation of the finite opening
catalog each cycle. Every opening is visited once per cycle against every
baseline, in two adjacent games with reversed game sides. It is a reproducible
catalog schedule, not independent sampling from a population or a search seed.
Related roots must share an authored group. Automatic immutable-root
lineage grouping also refuses cross-split descendants. This conservative grouping
is data provenance, never a rules, repetition or transposition identity. Callers
remain responsible for detecting related diagrams imported as distinct roots.

## Protocol matrix

| Game / whole profile | Game choices and conclusions | Score interpretation / independent-opponent status |
| --- | --- | --- |
| Chess `mshogi.chess.match.v1` | Moves/claims in an untimed M2 match | Root-side static-evaluation/mate-distance telemetry; actual UCI adapter also supports existing clocked matches. Current own evaluator adds positional terms to material. |
| Shogi `mshogi.shogi.csa36.v1` | Moves/drops/promotions and formal declaration | Game root-side heuristic/command-edge win distance; existing actual USI adapter available |
| Xiangqi `mshogi.xiangqi.wxf2018.v1` | Moves, claims, pending adjudication | Game root-side score; pending stays unscored; existing actual Xiangqi UCI0–9 adapter available |
| Tori `mshogi.tori.modern.v1` | Complete actions and conclusions | Perspective/score retained; compatible independent opponent not established |
| Chu `mshogi.chu.jcsa2019.v1` | Complete ordered Lion routes, effects, promotions and game outcomes | Perspective/score retained; NoBoardActions stays unscored; compatible independent opponent not established |
| Tenjiku `mshogi.tenjiku.adams1999-modern.v1` | Complete routes, Lion Hawk/jumping/Fire Demon effects, exact selected profile/history | Perspective/score retained; NoBoardActions stays unscored; compatible independent opponent not established |
| Tenjiku `mshogi.tenjiku.muller-completed.v1` | Same authoritative API under the distinct accepted whole profile | Never pooled with Adams histories/results; compatible independent opponent not established |

The exact profile IDs are authoritative in generated config/game records. The
engine result alone determines a score. Experimental action/wall/artifact caps,
cancellation, process failure and incomplete results end a game without a
score. The runner can admit a legal move at depth zero when the engine result
contract permits it. Such a move has no search score and play continues.
Observations precede accepted actions and preserve exact request, purpose,
participant, session, limits, history and profile identities.
All continuations, including compound routes and claims, replay through
rules owners. Replay consistency is not proof of optimal or honest search.

The `experiment-external uci|usi|xiangqi-uci09 CONFIG REPORT` command invokes the
existing actual external adapter; `experiment-external-replay REPORT` checks its
complete nested report and game replay. These preserve existing adapter
options, model fingerprints, work/watchdogs, protocol transcripts and failures.
They do not reinterpret external protocol seeds or manufacture unsupported
variant translations. The shared wrapper adds observed harness provenance.
Portable fixture-engine tests verify transport integration only, not strength or
an independent opponent. Existing adapter configs remain distinct because their
real opponents and protocol semantics differ.

## Evidence, data and bounds

Reports retain the whole scheduled sample, including games never attempted after
a budget stop, complete final histories and actual work observations.
Re-admission checks both replay and proposed effects. Imported observations use
the existing duplicate-aware bounded decoder. Profiles/history are never reduced
to board-only identities.

The operator chooses the positive whole-run wall budget in milliseconds and the
accepted step count per game. Zero steps produces a replayable opening-only
sample. A batch can have up to 16 roots, 4 participants, 16 cycles and 128
games. These bounds control the sample size. Each game can use the full step
budget. Search retains its own work limits.

The 1 MiB configuration metadata budget excludes opening histories. The 128 MiB
report budget limits storage for a batch. The runner allows for the scheduled
final roots and diagnostics, and charges the encoded step and final-root growth
before it commits a transition. When the next transition cannot fit, the report
retains the accepted prefix and marks the game unscored. Exported data has a
separate 128 MiB output budget. Watchdog and cleanup use EngineBridge;
no alternate supervisor is introduced. Cancellation is polled before/while child
work. Cleanup acknowledgement indicates supervisor termination, not an independent
OS-process audit; missing acknowledgement is recorded. Bridge cleanup errors
remain actual execution diagnostics. Wall time is a cooperative soft whole-run
budget; synchronous rules validation, serialization and bounded cleanup can
extend elapsed time. Parsing/replay is byte/count bounded, not hard real time.
Request timings include transport, search and receipt, plus launch when a child
must start. They are not pure search speed. The shared bridge can keep a child
between requests; this report change does not alter its lifetime or cleanup.

MSH-184 (Match report Replay Cost) removes repeated whole-report validation from
export and summary after a successful check. `run` and `run_with_stop` return an
owned immutable `ValidatedReport`. Its JSON export and summary methods reuse
that validation. `ValidatedReport::from_json` still performs strict bounded
parsing and complete rules validation. There is no deserializer or mutable
access for the validated type. `into_report` discards its validated status;
ordinary mutable `Report` methods always validate again. No validation flag is
stored in a file, and no hash substitutes for game admission.

JSONL projection still replays transitions to construct before/after
states. It reuses the validated report for its source digest and does not repeat
whole-report validation. See measurements and review.

JSONL rows bind the exact source-report SHA/config identity, participant/executable,
opening/pair/split/lineage, before/after complete game roots, actual request and
observation, admission/failure and terminal label availability. Compound actions
and history remain typed data, not abbreviated move strings. Applied
commands may be action targets; only genuinely concluded games have terminal
value labels. Failed or capped games keep null value labels. Zero-step games have
no JSONL rows: their attrition exists in the mandatory referenced full report.
Export/replay is sufficient for this data consumer; a Python/PyTorch
binding is deferred until a concrete training consumer requires it. Do not split
color pairs or trajectory prefixes across training and evaluation.

## Comparisons and research boundaries

Summaries separate each whole profile and baseline. They retain scheduled pairs,
complete played pairs, five paired score bins, played W/D/L, preconcluded
root diagnostics and unscored attrition/stop counts. A paired result has score
0,0.5,1,1.5 or 2 for the candidate across both games. Do not treat its two color
games as independent observations. Completed-only results can be biased by
attrition; the harness reports missing outcomes and makes no improvement verdict.
There is no Elo, confidence interval, significance or stopping-rule claim from
these small finite diagnostic pilots. [Stockfish's primary Fishtest mathematics](https://official-stockfish.github.io/docs/fishtest-wiki/Fishtest-Mathematics.html)
provides a research basis for future paired inference; its model is not blindly
applied to censored games or unequal-work comparisons here.

Rules conformance continues to use independently established expectations,
property/negative checks and review, not opponent agreement. Search correctness
uses completed-iteration/no-fallback/restoration tests. Performance measures
actual work/time and overhead. Playing strength requires a separately justified
distribution, opponent pool, fair resource conditions, sample/stopping rule and
budget. An equal-executable depth/node ablation is not an independent opponent
or evidence that one evaluator improved. The pilot measures infrastructure only.

Record the exact binary, model or network file, provenance and license for
each external opponent. The [strength roadmap](strength-roadmap.md) defines
the per-game comparison and training work.
