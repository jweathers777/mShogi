# Reproducible native experiments (MSH-48)

Current use follows the [owner-selected strength roadmap](strength-roadmap.md)
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
native Game owners admit every complete action. Chess additionally retains the
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

The strict version1 config names candidate/player0 and one to three baselines,
absolute executable paths, caller-declared build provenance, effective depth and
node limits per whole profile, complete native openings, source/rights rationale,
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
baseline, in two adjacent games with reversed native sides. It is a reproducible
catalog schedule, not independent sampling from a population or a search seed.
Related roots must share an authored group. Automatic immutable-native-root
lineage grouping also refuses cross-split descendants. This conservative grouping
is data provenance, never a rules, repetition or transposition identity. Callers
remain responsible for detecting related diagrams imported as distinct roots.

## Native protocol matrix

| Game / whole profile | Native choices and conclusions | Score interpretation / independent-opponent status |
| --- | --- | --- |
| Chess `mshogi.chess.match.v1` | Native moves/claims in an untimed M2 match | Root-side static-evaluation/mate-distance telemetry; actual UCI adapter also supports existing clocked matches. Current own evaluator adds positional terms to material. |
| Shogi `mshogi.shogi.csa36.v1` | Moves/drops/promotions and formal declaration | Native root-side heuristic/command-edge win distance; existing actual USI adapter available |
| Xiangqi `mshogi.xiangqi.wxf2018.v1` | Moves, claims, native pending adjudication | Native root-side score; pending stays unscored; existing actual Xiangqi UCI0–9 adapter available |
| Tori `mshogi.tori.modern.v1` | Complete native actions and conclusions | Native perspective/score retained; compatible independent opponent not established |
| Chu `mshogi.chu.jcsa2019.v1` | Complete ordered Lion routes, effects, promotions and native outcomes | Native perspective/score retained; NoBoardActions stays unscored; compatible independent opponent not established |
| Tenjiku `mshogi.tenjiku.adams1999-modern.v1` | Complete routes, Lion Hawk/jumping/Fire Demon effects, exact selected profile/history | Native perspective/score retained; NoBoardActions stays unscored; compatible independent opponent not established |
| Tenjiku `mshogi.tenjiku.muller-completed.v1` | Same authoritative API under the distinct accepted whole profile | Never pooled with Adams histories/results; compatible independent opponent not established |

The exact profile IDs are authoritative in generated config/native records. The
native result alone determines a score. Experimental action/wall/artifact caps,
cancellation, process failure and incomplete first iterations are unscored, never
draws or losses. A completed prior iteration is usable only where its existing
native result contract permits it. No fallback move or score is fabricated.
Observations precede accepted actions and preserve exact request, purpose,
participant, session, process-generation, limits, history and profile identities.
All native continuations, including compound routes and claims, replay through
native owners. Replay consistency is not proof of optimal or honest search.

The `experiment-external uci|usi|xiangqi-uci09 CONFIG REPORT` command invokes the
existing actual external adapter; `experiment-external-replay REPORT` checks its
complete nested report and native replay. These preserve existing adapter
options, model fingerprints, work/watchdogs, protocol transcripts and failures.
They do not reinterpret external protocol seeds or manufacture unsupported
variant translations. The shared wrapper adds observed harness provenance.
Portable fixture-engine tests verify transport integration only, not strength or
an independent opponent. Existing adapter configs remain distinct because their
real opponents and protocol semantics differ.

## Evidence, data and bounds

Reports retain the whole scheduled sample, including games never attempted after
a budget stop, complete final native histories and actual work observations.
Native re-admission checks both replay and proposed effects. A candidate that
would exceed the existing request/history representation is refused atomically,
with the prior state and actual observation retained. Imported observations use
the existing duplicate-aware bounded decoder. Profiles/history are never reduced
to board-only identities.

Experiment bounds: config 1 MiB, report/data 32 MiB, at most 16 roots, 4 participants,
16 cycles, 128 games, 512 commands/game and 4096 scheduled requests. Existing native
search limits, 1 MiB request/2 MiB response and native record budgets still apply.
Before search, the harness reserves bounded transition evidence capacity. Artifact
exhaustion ends remaining work honestly. Watchdog and cleanup use EngineBridge;
no alternate supervisor is introduced. Cancellation is polled before/while child
work. Cleanup acknowledgement indicates supervisor termination, not an independent
OS-process audit; missing acknowledgement is recorded. Bridge cleanup errors
remain actual execution diagnostics. Wall time is a cooperative soft whole-run
budget; synchronous native validation, serialization and bounded cleanup can
extend elapsed time. Parsing/replay is byte/count bounded, not hard real time.
This conservative offline implementation replays histories repeatedly and starts
fresh children per search. Its timing measures launch/transport/search/receipt,
not pure search speed. MSH-109 reviews costs before substantial optimization.

JSONL rows bind the exact source-report SHA/config identity, participant/executable,
opening/pair/split/lineage, before/after complete native roots, actual request and
observation, admission/failure and terminal label availability. Compound actions
and history remain typed native data, not abbreviated move strings. Applied
commands may be action targets; only genuinely concluded games have terminal
value labels. Failed or capped games keep null value labels. Zero-step games have
no JSONL rows: their attrition exists in the mandatory referenced full report.
Export/replay is sufficient for this native data consumer; a Python/PyTorch
binding is deferred until a concrete training consumer requires it. Do not split
color pairs or trajectory prefixes across training and evaluation.

## Comparisons and research boundaries

Summaries separate each whole profile and baseline. They retain scheduled pairs,
complete played pairs, five paired score bins, played W/D/L, preconcluded
root diagnostics and unscored attrition/stop counts. A paired result has score
0,0.5,1,1.5 or2 for the candidate across both games. Do not treat its two color
games as independent observations. Completed-only results can be biased by
attrition; the harness reports missing outcomes and makes no improvement verdict.
There is no Elo, confidence interval, significance or stopping-rule claim from
these small finite diagnostic pilots. [Stockfish's primary Fishtest mathematics](https://official-stockfish.github.io/docs/fishtest-wiki/Fishtest-Mathematics.html)
provides a research basis for future paired inference; its model is not blindly
applied to censored games or unequal-work comparisons here.

Rules conformance continues to use independently established native expectations,
property/negative checks and review, not opponent agreement. Search correctness
uses completed-iteration/no-fallback/restoration tests. Performance measures
actual work/time and overhead. Playing strength requires a separately justified
distribution, opponent pool, fair resource conditions, sample/stopping rule and
budget. An equal-executable depth/node ablation is not an independent opponent
or evidence that one evaluator improved. The pilot measures infrastructure only.

MSH-49 through MSH-54 own game-specific strength targets/protocols/pools and any
necessary real-opponent dialect extensions. Stockfish-compatible Chess UCI,
[YaneuraOu's actual USI family](https://github.com/yaneurao/yaneuraou), and
[Pikafish's actual Xiangqi UCI family](https://github.com/official-pikafish/Pikafish)
are research candidates, not bundled or acquired dependencies. Verify each exact
binary, network/model/data provenance, license and redistribution rights before
an affected experiment. Tori/Chu/both Tenjiku opponent availability is unresolved;
no external engine is a build/conformance prerequisite. No approved large compute
or training budget, numerical strength target, acquired training corpus or model
is established by MSH-48. Those are explicit blockers for affected later
experiments, not fabricated successful comparisons. MSH-55 owns learned evaluator
lifecycle; MSH-104 owns later research exploration. No substantial training ran.

See [the structured MSH-48 verification record](evidence/msh48/verification.json)
for exact tested source, measured pilot conditions/outcomes, platform receipts and
independent review. It is the single acceptance evidence ledger.

## First bounded pilot at 9a164eb

The release build from clean source used pinned rustc 1.99.1 on Apple M1 Pro/aarch64,
macOS 27.0.1, 10 logical CPUs, no custom flags and an unpinned shared host.
The finite catalog contains seven standard roots and seven authored live tactical
predecessors. It uses one seeded cycle and three instances of the same executable
with different work settings, not three independent opponents.

The 56-game main pilot compared depth 1 / 512 visits against depth 1 / 128 and
depth 2 / 512, with two accepted commands/game, a 120 s cooperative whole-run budget,
15 s request watchdog and 250 ms cleanup interval. Preliminary debug standard-turn
measurements at depth 1 / 512 ranged 13.59–7373.18 ms, supporting these conservative
work caps before the release run. The release pilot recorded 20 native conclusions,
28 unscored action caps and 8 unscored Tenjiku first-iteration exhaustions. All 84
actual observations were retained. Its 76 accepted commands generated 76 action
targets; only 20 rows have terminal value labels. No capped game was called a draw.

| Profile | Median observed turn ms | Maximum observed turn ms |
| --- | ---: | ---: |
| Chess | 7.57 | 67.51 |
| Shogi | 7.56 | 185.28 |
| Xiangqi | 31.64 | 310.61 |
| Tori | 7.56 | 19.63 |
| Chu | 16.61 | 88.35 |
| Tenjiku Adams | 268.18 | 1880.13 |
| Tenjiku Muller | 286.95 | 1881.99 |

These mix standard/tactical roots, colors and effective work, and include process
launch/transport/search/receipt. They are neither pure search benchmarks nor
cross-game strength comparisons. Exact individual limits, work and timing are
in the replayable reports; descriptive aggregates are in
[pilot-metrics.json](evidence/msh48/pilot-metrics.json).

Because 128/512 visits did not finish the selected Tenjiku tactical iteration, an
explicit eight-game supplement used the existing 2,000 visit native ceiling with
one-command cap. Candidate and peer requested depth 1; the third baseline requested
depth 2. All eight games reached genuine native royal-capture conclusions in both
profiles and reversed assignments. Six searches completed depth 1 at 619 visits;
two exhausted 2,000 visits during the later iteration and retained the earlier
completed depth 1 choice exactly as permitted by the existing result contract.
The initial incomplete report remains preserved. Neither native defaults nor
production limits changed.

A second full 56-game run reproduced pairings, requests, observations, accepted
histories, outcomes, stop reasons and cleanup diagnostics exactly after excluding
measured elapsed time. Independent review checked all 84 exported rows against the
full reports, source/config/executable hashes, lineage/split identities and final
roots. Compressed source reports are small durable evidence; the 2.6 MB JSONL can
be regenerated with `experiment-data` rather than committing a generated training
set. This establishes usable bounded infrastructure and exposes performance and
sample limitations before any substantial training or strength study.
