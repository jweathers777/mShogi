# Game-independent strength harness

Use `python3 -m tools.strength.cli COMMAND --help` from the repository root.
The strength program and acceptance thresholds are in the [roadmap](strength-roadmap.md).
Each game command requires `--game chess|shogi|xiangqi|tori|chu|tenjiku`.
Tenjiku also requires `--profile adams|muller`. Keep runs for different profiles separate.
Build the measurement binaries once:

```sh
cargo build --release --locked -p mshogi-runner --example strength --bin mshogi-match -p mshogi-console --bin mshogi
python3 -m tools.strength.cli build-engine REVISION
```

The helper uses the same game records, engine transport and rules as ordinary
experiments. The runner owns legality, clocks, terminal scores and report replay.
Use new output paths under `runs/<task-id>/`. The commands below use Chess for
the examples. The other games use the same commands.

## Openings and tactics

Fix the population, seed, count, depth, node budget and maximum absolute score
before screening. The score is from the side to move in hundredths.
`openings` draws legal continuations from the standard root. It removes duplicate
boards and accepts a live board only when the frozen engine reaches the specified
depth and its score is within the chosen bound. An exhausted attempt budget
writes the admitted rows and returns an error. Use `--source CATALOG` to
screen authored candidates in file order. The catalog has an `openings` array
with unique `id` strings. Each row can carry a canonical `record`. Existing
Chess and Xiangqi FEN/move catalogs, Shogi SFEN/move catalogs, Tori notation
and Chu game record catalogs can also be read.

```sh
python3 -m tools.strength.cli openings --game chess --engine ENGINE --depth 4 --nodes 500000 --balance 100 --seed 1 --plies 8 --count 100 --attempts 1000 --out runs/TASK/openings.json
python3 -m tools.strength.cli suite --game chess --source candidates.json --proof one-ply --out runs/TASK/suite.json
python3 -m tools.strength.cli probe --game chess --suite runs/TASK/suite.json --split held-out --engine ENGINE --depth 6 --nodes 500000 --out runs/TASK/probe.json
```

Tactical input has a `positions` array. Each row has an `id`, a `group` naming
its source game or opening, and the same position fields as an opening.
`one-ply` tries every legal board move and keeps every move that produces a
win for its mover. `supplied` accepts independently established `answers` (or
`expected_moves`) and requires their `proof` or `source`. It verifies each
answer's legality. Supply a complete set for the test criterion. A bounded mate
or checking-only suite measures that stated objective. An exclusion from its
answer set alone cannot establish a blunder.

The builder preserves explicit source partitions, removes duplicate boards,
and keeps each source group in one partition. For unassigned groups it hashes the group. The first SHA-256 byte of the group modulo four assigns zero to
held-out. The builder requires both partitions. Freeze the suite and reserve
held-out play for measurement. Existing suites with `answers` or
`expected_moves` and an explicit `split` can also be probed directly.
For the Tori checking-only catalog, an unlisted choice is checked against the
frozen full-width proofs or a bounded proof. Exhausted proofs are unknown.
The Chu probe reports the capture-return contract separately.
For a development blunder screen use `blunders --suite FILE --depth N --nodes N`.
It records whether each move avoided the known error and matched one reference
alternative. Add `--movetime MS` for a timed screen.

### Tenjiku inputs

Use the catalogs in `tools/strength/data/`. Replace `PROFILE` with `adams`
or `muller` in both the filename and the command. A full opening history binds
the game to its selected rules profile.

- **Openings**. Each `tenjiku-PROFILE-openings-v1.json` has 100 openings.
  The source is four seeded legal actions from the standard position. The
  frozen engine at `2e4b9ad` screened each live position at depth 2 with a
  500,000-node budget and accepted an absolute score of at most 100 pawn
  hundredths. The catalog records the seeds, budgets and individual scores.
  This is a shallow balance estimate for color-swapped tests.
- **Tactics**. Each `tenjiku-PROFILE-tactics-v1.json` has 216 composed
  diagrams. The shared one-ply prover applied every legal board action
  through the game rules and retained every immediate win. The nine
  families exercise Fire Demon burns, ranked general captures over
  intervening pieces and two-leg Lion captures. Variants of one family
  stay together. Three whole families (72 positions) are held out.
  The frozen baseline solved all positions at depth 2. Use this suite
  to detect tactical regressions. It has no headroom at that limit.
  A slower forced win can fail the one-ply criterion.

To regenerate the answers, pass a frozen tactical catalog to `suite`
as the `--source` with `--proof one-ply`. The game recomputes all terminal
winning actions from each record. Compare the position IDs, groups, splits
and complete answer sets. To score the held-out Muller suite, run

```sh
python3 -m tools.strength.cli probe --game tenjiku --profile muller --suite tools/strength/data/tenjiku-muller-tactics-v1.json --split held-out --engine ENGINE --depth 2 --nodes 500000 --out runs/TASK/muller-held-out.json
```

For a clocked Tenjiku comparison, start with 60 s + 1.5 s per move
(`--base 60 --increment 1.5`). MSH-214 used a 768-ply game limit,
a 1,200-second pair limit and the shared late-depth boundary of
40 post-opening plies. The attained late-game target is depth 5 on
`c3-standard-4` at this control. Use the same machine type and pair
concurrency on both sides of a comparison. Some searches exhausted
their time before completing the first iteration. The match driver stops
on an unfinished pair. Keep its report and diagnose the stop before
continuing a strength test. MSH-214 records the calibration, paired
time-odds results and their limits for both profiles.

## Comparisons and time odds

```sh
python3 -m tools.strength.cli match --game chess --candidate NEW --baseline FROZEN --openings runs/TASK/openings.json --base 5 --increment .05 --out runs/TASK/comparison
```

The runner plays each opening twice with the colors reversed. Catalog order
cycles from `--first-seed`. A fixed-work comparison can instead set
`--candidate-nodes`, `--baseline-nodes` and `--baseline-depth`. Record the
chosen limits and hypotheses before play. `--candidate-time-factor 2` doubles
the candidate's base and increment. Use the same executable on both sides for
an internal time-odds curve. `--calibrate` requests one turn per color per game.
The driver retains the match reports, both binary identities, timing and depth,
median accepted depth from `--late-ply` onward, the five pair-score counts, Elo interval and SPRT decision. A cutoff supplies
no game score. An unfinished match pair stops the campaign, except for the
predeclared Tori step-limit exclusion (see MSH-204).

`--max-pairs`, `--max-steps`, `--pair-wall-ms` and `--wall-seconds` bound the
campaign. The driver reserves another 45 seconds per pair for shutdown. The
runner also enforces its report-size budget. `bench` measures fixed-depth wall
time and nodes on a positions file, including cold process and transport cost.
`signature` compares full fixed-depth engine observations for an opening catalog.

## Self-play, tuning and deeper judging

```sh
python3 -m tools.strength.cli selfplay --game chess --engine ENGINE --openings runs/TASK/openings.json --random-plies 2 --candidate-nodes 50000 --max-pairs 10 --out runs/TASK/selfplay
python3 -m tools.strength.cli export --game chess --every 4 --out runs/TASK/corpus runs/TASK/selfplay/*.report.json.gz
python3 -m tools.strength.cli judge --game chess --engine JUDGE --depth 12 --nodes 2000000 --samples 16 --out runs/TASK/quality.json runs/TASK/selfplay/*.report.json.gz
```

Self-play gives both participants the same engine and budget. Export replays
each report before extracting board moves and first-player outcomes. Unfinished
games remain in the corpus with no samples. Both colors and seeded continuations
of the same opening stay together. The splitter removes cross-partition board
duplicates, then keeps one copy within each partition. The manifest records
input/output hashes, the split counts, duplicate counts and game stops. Small
demonstrations can pass `--allow-empty-demonstration`. Fitting requires populated
training, validation and held-out sets.

The positional input is FEN for Chess and Xiangqi, SFEN for Shogi, and a diagram
for Tori, Chu and Tenjiku. Chu retains Lion protection. Tenjiku retains profile,
piece identity, promotion, turn and pending bare-royal status. Chess filters
quiet positions during export. The other existing fitting models apply their
own quiet-position filters. Build the applicable `mshogi-search` example
`GAME_texel`, then use `texel --game GAME fit TRAIN VALIDATION OUTPUT` or
`texel --game GAME check FROZEN HELDOUT OUTPUT`.

The Tenjiku model uses the search evaluator's 44 face values, four placement
terms and tempo. It fixes the Pawn, King and Crown Prince values. It excludes
pending bare-royal replies, terminal positions, immediate royal threats and
boards with a positive or uncertain compound exchange. The fit reports each
profile's loss separately. The corpus splitter keeps each catalog opening in
one partition and ignores interchangeable physical piece IDs when comparing
diagrams. Profile, side, piece placement and promotion rights remain part of
the board identity.

The judge samples evenly spaced board moves, including moves in unfinished
games. It searches each full game history before and after the move with a
fresh engine state. A terminal score takes precedence after the move.
Missing scores remain unjudged. The output keeps the inputs, depths, scores,
actual deeper-search count, clear blunders per 100 judged moves and mean
positive win-probability loss. Its `positions` array contains reusable clear-blunder
records for `blunders --suite QUALITY.json`. Choose materially greater work than the players
and inspect the achieved depths. See the roadmap for the probability rule and
the limitations of same-family judging.

## Cloud work

Build Linux executables with `tools.strength.cloud_build`. Build the worker image
from `tools/experiments/Dockerfile` with Cloud Build and pin its digest. A
measurement manifest freezes each input and executable as a basename mapped to
a GCS URI and SHA-256. Mark binaries with `"executable": true`. Each bin
contains one or more argument lists for this same strength CLI, without a shell.
Use explicit relative executable paths such as `./engine`, `./helper` and
`./runner` in these lists. List each report name explicitly in cloud commands.

```json
{
  "schema": 1,
  "campaign_started_epoch": 1800000000,
  "deadline_epoch": 1800001800,
  "estimated_compute_before_usd": 0,
  "files": {
    "engine": {"uri": "gs://BUCKET/PREFIX/engine", "sha256": "ENGINE_SHA256", "executable": true},
    "helper": {"uri": "gs://BUCKET/PREFIX/strength", "sha256": "HELPER_SHA256", "executable": true}
  },
  "bins": [[["openings", "--game", "tenjiku", "--profile", "adams", "--engine", "./engine", "--helper", "./helper", "--depth", "2", "--nodes", "500000", "--balance", "100", "--count", "1", "--attempts", "5", "--out", "openings.json"]]]
}
```

Replace the example timestamps with the original campaign start and a deadline
1,800 seconds later. Include prior task compute in the estimate. Submit with
`cloud-submit --manifest M --config C --image IMAGE@sha256:DIGEST --job strength-NAME --out DIR`.
Use the project Batch configuration with one machine type, a reviewed hourly
rate, up to eight concurrent workers and zero retries. The controller reserves
the remaining maximum compute against the US$10 cap, waits for completion and
verifies that the job, VMs and disks are gone. It retains the manifests, output
and receipts. Record costs and the GCS prefix in the Backlog task.
