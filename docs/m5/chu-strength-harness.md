# Chu strength harness

Chu uses the shared clocked native experiment runner and structured internal
protocol. No external opponent or text protocol is needed. Timed engine turns retain a
native game in the shared incremental session. An authenticated exact prefix
permits a suffix; takeback, unrelated roots and process restarts require a full
reset. Every suffix command and its derived event are checked before the new
state is installed. Saved-game JSON is unchanged. Native rules admit
every complete action and decide the result. Measurements and costs belong in
the relevant Backlog task and its ignored `runs/` directory.

Build the tools with `tools.strength.cloud_build` for cloud work:

- `mshogi-console:bin:mshogi`
- `mshogi-runner:bin:mshogi-match`
- `mshogi-runner:example:chu_match_config`
- `mshogi-runner:example:chu_strength`

Freeze the accepted predecessor separately. Record each binary's source commit
and SHA-256. Use a digest-pinned worker image built with Cloud Build. The
`tools.strength.chu_cloud` module runs bounded commands through the existing
Batch controller, which records completion and verifies resource deletion.

## Inputs and independent answers

`chu_strength openings OUT COUNT DEPTH` creates standard-root candidates from
eight seeded quiet rear-zone moves. It removes duplicate repetition positions
and screens at fixed depth, with a 500,000-node cap and an absolute score limit
of 100 Pawn hundredths. Only completed searches can admit an opening. Fix the
count, depth and balance rule in the task before screening. Preserve the full
screen as run evidence; the reusable catalog needs the admitted native records,
source and rule. The config writer replays those records before a match.

`chu_strength tactics OUT` constructs elementary Lion and royal positions and
lists every one-turn native winning action. The reference proof uses full
native transitions and terminal outcomes, independently of the search engine's
score, ordering and pruning. It treats a visit cutoff as unknown and does not
invent a result for unresolved no-action positions. Cases cover double capture,
capture-return, the qualifying intermediate capture of a defended Lion and
capture of both opposing royals.

`chu_strength forcing OUT COUNT` separately constructs three-ply material wins.
It enumerates every defender reply with the native proof, rejects one-ply wins
and omits a position if a complete answer set exceeds 200,000 proof visits.
These positions also share construction templates; retain this limitation.

The first suite is an elementary screen, not a deep tactical-strength measure.
Its held-out part uses unseen placements from shared construction templates.
Keep that limitation when reporting scores. `chu_strength probe SUITE OUT MS`
compares the search choice with the complete independently proved answer set.
It separately scores an exact igui route/effect contract in both native and
search positions. Ordinary winning captures cannot satisfy that contract.

## Clocked comparisons and data

Run `python3 -m tools.strength.native_match --game chu` with `--candidate`,
`--baseline`, `--runner`, `--config-writer`, `--openings`, `--base`,
`--increment` and `--out`. The driver records all executable identities and
plays both colors of each opening. Chu selects nominal allocation explicitly;
older reports retain their recorded allocation. `--candidate-time-factor 2`
doubles both candidate base time and increment. An unfinished Chu pair stops
the campaign and remains unscored. Do not convert a move or clock limit to a
draw. A small capped sample can give no statistical decision.

`chu_strength export REPORT_PATHS.json OUT.jsonl EVERY` validates and replays
the source reports, then samples board positions at the requested interval.
Only native concluded games supply outcome labels. Unfinished games remain
listed with no samples. Exported board inputs include side to move and Lion
protection; these are evaluation inputs, not complete saved games.

Split with `python3 -m tools.strength.texel_data split CORPUS MANIFEST --game
chu --by-opening`. Both colors and every prefix of an opening stay together.
The shared splitter removes board duplicates across partitions. A small
export demonstration may use `--allow-empty-demonstration`; its manifest says
that empty partitions are not ready for training. The later evaluation task
must generate a sufficiently large quiet-position corpus before fitting weights.
