# Engine strength test harness

MSH-163 (Engine Strength Test Harness) supplies the acceptance test for engine
changes in MSH-164 (Shared Search Core) and later tasks. It has three parts and
one command for each part. Run all commands from the repository root with
Python 3.11 or later. No Python packages are necessary.

The harness measures the engine. It is not a rules oracle. The native rules
stay the authority for every game: the game test uses the native
`mshogi-match run` controller, which replays and scores each game.

## Commands

Build the release binaries first:

```sh
cargo build --release --locked -p mshogi-console --bin mshogi -p mshogi-runner --bin mshogi-match
```

Build a frozen predecessor from a Git revision. The binary goes to
`target/strength/engines/<rev12>/mshogi`, with an `.identity.json` file that
records the revision, compiler and SHA-256:

```sh
python3 -m tools.strength.cli build-engine <revision>
```

### 1. Speed benchmark

```sh
python3 -m tools.strength.cli bench --engine ENGINE --out FILE.json
```

- Position set: `tools/strength/data/bench-chess-v1.json`, 18 positions. These are
  the start position, five Chess Programming Wiki perft positions, six MSH-161
  master opening roots and six MSH-161 endgame positions with full histories.
- Each position has a fixed depth. The depths were set once, on 2026-11-13, as the
  first depth whose search took more than 0.6 s with the baseline engine. Do not
  edit the set. A changed set gets a new name.
- The command sends `go depth N` for each position, three times by default.
  It reports total nodes, time to depth per position, and nodes per second.
  `nps` uses the driver's wall time for each search, including UCI I/O.
  `engine_nps` uses the time that the engine reports.
- The command stops with an error if the engine does not reach the fixed depth.
- Total nodes is a search signature. It is the same in each repeat when the search
  is deterministic. A change in search or ordering changes it; that is expected.

### 2. Development blunder suite

```sh
python3 -m tools.strength.cli blunders --engine ENGINE --movetime 1000 --out FILE.json
```

- Suite: `tools/strength/data/chess-blunders-msh161.json`, 146 positions.
- Selection: each mShogi move in the 200 MSH-161 (Competitive Gameplay Diagnosis)
  games where the 500k Stockfish screen gives a root expected score from .30 to .70
  and a played expected score below .15, and the 20M confirmation status is
  `supported_finite`. This is the definition of the
  [independent review](evidence/msh161/cloud-primary/independent-review-fable/review.md)
  (its "146 have 20M support"). `tools/strength/extract_blunders.py` made the file
  from the saved evidence; it needs the Git-ignored local evidence mirror.
- Each position has the initial FEN, the full move history before the error,
  the error move and the 20M reference alternative.
- **These positions are exposed. Use them for development only, not as held-out
  evidence.**
- `avoided` means only that the engine played a move different from the confirmed
  error. A different move can also lose. `matched_reference` compares with one
  reference move. Use the suite to detect change, not to prove good play.

### 3. Game test

```sh
python3 -m tools.strength.cli match --candidate NEW --baseline FROZEN --label NAME --out DIR
```

- Openings: `tools/strength/data/openings-msh161.json`, the 90 balanced MSH-161
  master roots with their full prefixes. They are exposed development material.
  The order is shuffled with a recorded seed (`--seed`, default 163), so each
  part of a run samples all families.
- Each opening is played twice with colors reversed (one pair) in one
  `mshogi-match run` process. Several pairs run in parallel (`--workers`,
  default: performance cores on Apple silicon). Both engines get
  `ClaimPolicy auto`; the controller uses `auto_claim`.
- Time control: default fixed 200 ms per move (`--movetime`). Clock mode is
  `--base SECONDS --increment SECONDS`. See the limitation below.
- Stopping rule: generalized SPRT on pair scores, logistic Elo hypotheses
  H0 = `--elo0` (default 0) and H1 = `--elo1` (default 15), alpha = beta = 0.05.
  The pair-score variance includes two prior pairs at .25 and .75, so a few
  decisive pairs in a drawish match cannot stop the test. No decision is
  possible before `--min-pairs` (default 30) and 2 x workers scored pairs. The run
  also stops at `--max-pairs` attempted pairs (default 600), or when more than
  2% of the pairs fail.
- A pair with a process failure, a ply limit or an unresolved native conclusion is
  not scored. It is recorded in `failures`.
- Output: `DIR/result.json` (decision, pentanomial counts, Elo estimate with 95%
  interval, wins/losses/draws, median depth and nodes, compute) and
  `DIR/games.jsonl.gz` (one record per pair: moves, conclusions, per-engine search
  medians, wall and CPU seconds). `--keep-reports` also keeps each full native
  report.
- `decision` is authoritative. `all_completed` adds pairs that finished after
  the decision.
- Compute: `cpu_seconds` is the resource use of each `mshogi-match` process and
  its engines, including cancelled pairs.

## Baseline (engine 65887fb0, 2026-11-13/14)

Engine: `target/strength/engines/65887fb0c653/mshogi`, built by `build-engine`
from 65887fb0 (Rust 1.99.1, release), SHA-256
`167cd1407abcfbf1f0595643e3c069bc1022040b011c0386a709d40932578b98`. The search
and evaluation source is the same as f1b64300. Machine: Apple M1 Pro
(8 performance cores), macOS 27.0, Python 3.11.0. Results apply to this machine
only. Evidence: [baseline-65887fb0](evidence/msh163/baseline-65887fb0/).

| Part | Result |
|---|---|
| Bench (`bench-chess-v1`, 3 repeats) | 5,431,097 nodes (same in each repeat); median 18.31 s; median 296,656 nps (range 294,262–298,949) |
| Blunder suite, 1000 ms, 4 workers | avoided 47 of 146; matched reference 17; median depth 7. Opening 4/16, middlegame 26/83, endgame 17/47 |
| Self-test, 200 ms per move, 8 workers | 600 pairs, no SPRT decision (LLR −1.58; bounds ±2.94); +277 −258 =665; Elo +5.5, 95% interval −3.0 to +14.0; 0 failed pairs |
| Self-test compute | 2,754 s wall; 21,305 CPU seconds (58 s in 7 cancelled pairs) |

The self-test interval includes 0, which is consistent with equal strength.
Both sides had median depth 6 and about 55,000 nodes per move. The run reached
the 600-pair cap: at this draw rate (72% of pairs 1–1), the default [0, 15]
hypotheses need more than 600 pairs when the true difference is near zero.
A change with a large effect, such as the expected speed gain of MSH-164
(Shared Search Core), will stop much earlier. For small changes, increase
`--max-pairs` or use wider hypotheses.

The blunder suite at 1000 ms avoids fewer errors (47) than a quick 100 ms check
(70). The 1000 ms search agrees more often with the engine's own 5 s game choice,
which was the error. This shows that the suite measures the engine's present
decision, not how hard each position is.

The harness revisions were 2cdae107 (bench) and c2595a18 (blunders and match).
The only change between them is the conclusion cross-check for clock mode, which
does not affect these results.

## Known limitation: clock games

With the 65887fb0 engine, a 5 s + 0.05 s Fischer clock gives flag falls in about
three of four games, near ply 60. The engine uses its reserve down to about 50 ms
(`go wtime 50 winc 50` searches 50 ms). The native controller records these as
unresolved, so they are not scored. For this reason the game test uses fixed time
per move by default. MSH-167 (Engine Time Management) owns the fix; the clock
mode is ready for its game tests.

## Verification

`python3 -m unittest tools.strength.tests.test_strength` (26 tests): logistic Elo
values, Wald bounds, a hand-computed LLR, small-sample and draw-dominated stopping
cases, simulated SPRT error rates (true 0 Elo: H1 at most 10%; true +25 Elo: H1
at least 90%), data-file invariants, UCI parsing, every native conclusion shape,
the native-summary cross-check, and one real paired game through
`mshogi-match run` when the release binaries exist. The native controller
accepted all 90 openings as legal during the smoke runs.
