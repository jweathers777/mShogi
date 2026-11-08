# Initial conventional Chess diagnostic (MSH-49.2)

John authorized an initial diagnostic of8–12 games against full-strength
Stockfish19, with paired openings, equal one-second thinking time and at most
30 minutes total match work. This replaces the unapproved32-game formal proposal
preserved at `47f2383`. No significance test, confidence interval or Elo estimate
belongs in this diagnostic.

Use eight games: Open Game (`e4 e5 Nf3 Nc6`), Queen’s Gambit (`d4 d5 c4 e6`),
English (`c4 e5 Nc3 Nf6`) and French (`e4 e6 d4 d5`), each with colors reversed.
These are familiar authored prefixes for qualitative investigation, not held-out
or statistically representative samples. Complete orthodox-start history is
retained and native rules own all actions and conclusions.

Both engines receive only `go movetime 1000`, without external depth or node
limits. Stockfish19 uses its pinned full default neural evaluator, Skill20,
strength limiting off, one search thread, no pondering and16MB Hash. mShogi uses
the accepted conventional/classical engine and its4,096-entry TT. They run
sequentially on the same Apple M1 Pro/32GiB macOS27 host. Memory is explicitly
unequal, and equal time allowance need not mean equal actual CPU use or depth.
There is no cloud/vendor fee, external book or tablebase dependency.

One existing native runner executes the four pairs, each once. The1800-second
aggregate allowance includes the prior5.8404-second transport smoke and all
failed attempts/cleanup. A whole-process-group deadline reserves5 seconds for
cleanup; no new pair starts with under60 seconds left. Healthy games reaching
256 total plies remain unscored. Process/protocol/cleanup failure stops further
work; no game is silently replaced. Existing hashes, native replay and bounded
receipts provide reproducibility without a new measurement system.

The [protocol](evidence/msh49-2/protocol.json),
[build pins](evidence/msh49-2/build.json) and
[canonical verification/accounting record](evidence/msh49-2/verification.json)
preserve the exact decision, resources, effective controls, histories and results.
The [launcher](evidence/msh49-2/study.py) is offline tooling around the existing
runner; production code is unchanged, with prior native platform acceptance at
`b8cc851` reused. The old two-game four-ply transport smoke remains unscored and
separate from the eight diagnostic games.

The report will describe W/D/L, actual nominal/selective depths, visits/time,
practical limitations and representative loss mechanisms using concrete native
positions/actions. Interpretations must remain distinguishable from observed
facts. This establishes an initial practical baseline, not parity, human rating
or a numerical release-strength threshold. Then prioritize conventional Shogi
and Xiangqi; deeper Chess analysis returns during learned-evaluation research.
No further Chess tuning or repeated small benchmark campaign is part of this task.

Official references: [Stockfish19 release](https://github.com/official-stockfish/Stockfish/releases/tag/sf_19)
and [UCI controls](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-Protocol-and-Stockfish-Commands.html).
