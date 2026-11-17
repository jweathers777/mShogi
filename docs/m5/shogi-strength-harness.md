# Shogi strength harness

MSH-186 (Shogi Strength Harness) extends the shared strength tools. It does not change search, evaluation or native rules. MSH-187 (Shogi Strength Round) can use this baseline to test an engine change.

## Frozen baseline and data

The engine is task-start main `d75ecdb8c5f45252157bd769498bb080b2ad7462`, release Rust 1.99.1 on arm64 macOS (Darwin 27.0.0). Its SHA-256 is `271950285d4cd5a18f9f567c39b8c84005470832c2b4bae4f5246609422a638e`. Measurement tooling was built from clean `e80f00a8abb4542fec77aaf4e12aa70838bcaab4`. Each receipt records executable hashes and environment.

The nine benchmark roots cover five ordinary opening structures and four composed later positions, including pieces in hand. Three fixed-depth repeats each searched 1,594,093 nodes. Median full-request time was 1.7589 seconds, about 906,283 nodes per second. These times include fresh process startup and cold caches; they are not pure search timings.

At a 1000 ms request budget, the five ordinary roots reached depths 14, 16, 11, 11 and 10. The four composed roots reached 7, 64, 16 and 10. The promoted-piece composition reached the depth cap with a mate score; depth 64 is not representative ordinary depth. The timed probe has one repeat, so its mechanically emitted `deterministic_nodes:true` does not establish repeatability. The final CLI emits `null` when there is only one repeat.

All 12 tactical cases matched the expected moves. They are six independently checked short checking-mate geometries and their reversed-color forms, not twelve independent problems. Native exhaustive checking-mate proofs verify the complete answer sets at one or three plies. The suite is exposed development data with a baseline ceiling; it is not a broad blunder suite or strength qualification. Quiet or longer winning moves are outside this metric.

The six opening prefixes cover established structures. They are original CC0 transcriptions, paired with colors reversed; balanced structure coverage does not certify equal evaluation. See [data sources](evidence/msh186/data-sources.md) and [independent data review](evidence/msh186/data-review.md). Native validation accepts all nine benchmark roots, twelve proof cases and six prefixes.

## Commands

Run from the repository root. Python 3.11 and stable Rust suffice; no external engine is a test dependency.

```sh
python3 -m tools.strength.cli build-engine d75ecdb8
cargo build --release --locked -p mshogi-runner --bin mshogi-match --example shogi_strength --example shogi_match_config
python3 -m tools.strength.cli validate-data --game shogi --out target/shogi-data.json
python3 -m tools.strength.cli bench --game shogi --engine target/strength/engines/d75ecdb8c5f4/mshogi --repeat 3 --out target/shogi-bench.json
python3 -m tools.strength.cli bench --game shogi --engine target/strength/engines/d75ecdb8c5f4/mshogi --repeat 1 --movetime 1000 --out target/shogi-depth.json
python3 -m tools.strength.cli tactics --game shogi --engine target/strength/engines/d75ecdb8c5f4/mshogi --movetime 1000 --out target/shogi-tactics.json
```

Use new output paths for each run. Native rules admit every root and returned move. The probe records full observations and checks engine shutdown. Outer timeout stops the process group and retains failure diagnostics.

## Clock-game contract

The existing native paired runner accepts `--base`, `--increment` and `--openings`. It freezes the opening bytes, runs both color orders, replays every report through native rules, and uses the existing pentanomial sequential test in seed order. All launched pairs remain in the record. An incomplete pair stops the campaign without being converted to a draw or excluded by score.

One engine process per player remains alive for each game. The caller derives a hard request allocation from the remaining clock, charges launch, transport and search time, and adds the increment only after an admitted command. Replay checks the complete clock ledger. Depth64 and five million nodes are symmetric safety caps. This uses the existing single-time-limit engine protocol; it does not add engine soft/hard clock management.

Clock exhaustion is an unscored `clock_limit` experiment stop. The native Shogi owner remains untimed. This task does not invent a time-loss game rule. Native repetition, perpetual-check loss, entering-king declaration and the accepted move limit remain authoritative.

The follow-up demonstration uses the same executable for both roles:

```sh
python3 -m tools.strength.shogi_match \
  --candidate target/strength/engines/d75ecdb8c5f4/mshogi \
  --baseline target/strength/engines/d75ecdb8c5f4/mshogi \
  --base 5 --increment .2 --openings tools/strength/data/shogi-openings-v1.json \
  --workers 4 --max-pairs 6 --min-pairs 30 --wall-seconds 900 \
  --pair-wall-ms 180000 --out target/shogi-clock
```

For a later authorized engine comparison, replace the candidate with the new build and freeze the comparison conditions before play. Six pairs only demonstrate execution; they cannot satisfy the 30-pair minimum sequential decision.

## Labeled positions

Export only native-replayed reports with successful process cleanup. Sample every fourth native event from the opening onward; labels are final sente outcomes, never search scores. The split groups equal initial boards, keeps color pairs together, removes all boards shared across partitions and deduplicates within a partition. Shogi identity includes board, side and hands, excluding the move counter.

```sh
python3 -m tools.strength.cli texel-export --game shogi --reports target/shogi-clock --out target/shogi-texel
python3 -m tools.strength.texel_data split target/shogi-texel target/shogi-texel/split-manifest.json --game shogi
```

A small demonstration may lack a validation or held-out partition. In that case strict splitting fails before writing outputs. `--allow-empty-demonstration` permits an explicitly incomplete demonstration and records `training_ready:false`; it does not authorize tuning on empty partitions. Raw generated games and training sets stay outside Git. Manifests retain hashes and generation conditions.

## External opponents

The [opponent proposal](shogi-opponent-proposal.md) recommends source-pinned YaneuraOu and Gikou configurations, with admission checks and a bounded future protocol. John must approve an external study before it starts. No external rating games or cloud jobs ran for MSH-186 (Shogi Strength Harness). Current external USI transport still needs a separate extension for full clock games. External engines remain measurement tools, never rules oracles or build dependencies.

## Measurement checkpoint and verification

The first 5 s + 0.05 s same-engine run stopped after its first four pairs in 45.383 seconds. Four games reached native conclusions, three returned `incomplete:Stopped` before completing depth one at 27–29 ms request budgets, and one reached the artifact cap. Only one complete pair entered the sequential statistic; it made no decision. All eight games passed native replay with empty cleanup diagnostics. The independent reviewer recomputed all 1,028 actual clock receipts and request allocations exactly.

The artifact stop exposed a harness capacity limit: full native history in each observation makes reports grow with the square of game length. The old 32 MiB ceiling, including conservative extra accounting and an 8 MiB transition reserve, stopped an ordinary pair at 183 and 149 steps. Measured history growth projects roughly 85–100 MiB under that accounting for two complete 320-ply histories; this is not a universal size guarantee. A bounded 128 MiB ceiling keeps the reserve and the native game/request limits. A six-pair catalog check at 5 s + 0.2 s follows this correction; the earlier failure is preserved, not replaced by a scored result.

Verification is targeted Level 2 because clock execution and process-group cleanup affect process supervision. Rules, search and evaluation are unchanged. Python process orchestration uses POSIX process groups; this task verifies it on macOS and does not claim Windows support. On macOS, 102 Python strength tests passed; the runner library passed 34 tests (two ignored), the writer example passed 2, and native probe/proof tests passed 3. Runner all-target Clippy with warnings denied and workspace all-target checking passed. Independent reviews cover [tools and proposal](evidence/msh186/tools-review.md), [clock implementation and ledgers](evidence/msh186/clock-review.md), and [data](evidence/msh186/data-review.md). Linux/Windows runtime and packaged desktop checks were not run; broader compatibility remains for a risk-required integration checkpoint.

The follow-up completed all six pairs (12 native conclusions) in 236.400 seconds, with 577.919 summed pair wall-seconds. All records passed native replay; no cleanup errors or unscored games occurred. Nominal candidate results were 5 wins, 0 draws and 7 losses, with median completed depth 9 for both players. Both roles used the same binary. This is a harness demonstration, not evidence of a strength difference. The sequential test made no decision, as expected below its 30-pair minimum. Conditions differ from attempt 1, so the two runs are not pooled.

See the [structured verification record](evidence/msh186/verification.md) for source, commands, failures, review and durable receipts.

The generated demonstration contains 12 concluded games and 497 sampled positions. Deduplication removes 9 repeats, leaving 488 training positions. Validation and held-out partitions are empty because all six frozen opening families hash to training. Strict splitting refused this input without writing partitions; the explicit demonstration mode records `training_ready:false`. No evaluator was fitted. MSH-187 (Shogi Strength Round) must add independent opening families and enough data before tuning; it must not relabel these exposed development positions as held-out evidence.
