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

The report describes W/D/L, actual nominal/selective depths, visits/time,
practical limitations and representative loss mechanisms using concrete native
positions/actions. Interpretations must remain distinguishable from observed
facts. This establishes an initial practical baseline, not parity, human rating
or a numerical release-strength threshold. Then prioritize conventional Shogi
and Xiangqi; deeper Chess analysis returns during learned-evaluation research.
No further Chess tuning or repeated small benchmark campaign is part of this task.

Official references: [Stockfish19 release](https://github.com/official-stockfish/Stockfish/releases/tag/sf_19)
and [UCI controls](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-Protocol-and-Stockfish-Commands.html).

## Observed results

**mShogi: 0 wins, 0 draws, 8 losses.** Every game ended in native checkmate.
There were no failed, capped or unscored diagnostic games, truncated transcripts,
stderr diagnostics or cleanup failures. All four pair reports passed native
replay, independently repeated by the reviewer. Total active match work was
**494.8894 seconds (8 minutes14.9 seconds)**:489.0490 seconds for this diagnostic
plus the preserved5.8404-second smoke. No further match batch ran.

The table uses ordinary **centipawn-scored turns** for median nominal depth,
keeping late forced-mate telemetry separate. Plies include the four-ply opening.

| Opening | mShogi side | Total plies | Result for mShogi | mShogi median depth | Stockfish median depth |
| --- | --- | ---: | --- | ---: | ---: |
| Open Game | White |90|Loss|6|23|
| Open Game | Black |69|Loss|5|19|
| Queen’s Gambit | White |92|Loss|6|22|
| Queen’s Gambit | Black |37|Loss|5|22|
| English | White |78|Loss|5|22|
| English | Black |43|Loss|5|24|
| French | White |68|Loss|5|21|
| French | Black |77|Loss|6|25|

Across these turns, mShogi reported nominal depth2–7 (median5), selective depth
12–22 (median18), and12,984–116,018 visited nodes (median22,139). Stockfish
reported nominal depth17–36 (median22), selective depth19–56 (median36), and
780,626–1,692,100 nodes (median1,003,846). Depth definitions, selectivity and node
accounting differ between engines; these raw figures are observations, not a
calibrated speed ratio or an explanation of all strength differences. mShogi’s
exchange-estimation/native-query work is not counted as searched node visits.
Its conservative complete-history TT bounds also limit cross-history reuse;
no fresh profiling/ablation study was performed to assign a share of the gap.

mShogi’s final per-turn reported time was702–1005ms on centipawn-scored turns,
with actual bestmove response times703.1–1006.1ms across all turns. Stockfish’s
centipawn-scored turns reported1000–1002ms; some forced-mate replies finished
in about2ms. Both had the same1000ms allowance, with their own early-finish
behavior and small cooperative/scheduling overheads. Stockfish’s nominal245
reports in short forced-mate lines do **not** mean245 board plies were searched.

## Representative loss observations

In the Open Game with mShogi Black, **17...Rfe8 18.Qf6 Qf8 19.Bxf8 Bxf8**
conceded its queen for a bishop under kingside pressure. Its previous root
assessment was−207cp at nominal depth4; at ...Qf8 it reported−640cp at depth5.
The earlier PV already contained Qf6, so merely including a threat in the PV
had not meant valuing its consequences accurately. Native board inspection
confirms the pieces, captures and move order. This is an observed failure to
handle the attack effectively, not proof of the first or an avoidable mistake.

In the Queen’s Gambit with mShogi Black, **15...Qxa1** captured a rook and was
valued at+239cp by its depth6 search. The actual continuation was **16.Qg5 g6
17.Qh6 Bh2+ 18.Nxh2 Qd4 19.Qg7#**. On its next turn mShogi already reported
mate against itself. This illustrates the practical limit of its short search
and attack assessment: a material gain looked favorable immediately before a
mating attack. It does not isolate evaluation, pruning or horizon effects, or
establish that ...Qxa1 was the first losing move. The native FEN before the grab
was `rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15`.

Stockfish also converted the first White game through a passed c-pawn ending
in **...c3, ...c2, ...c1=Q#**. The material/positional phase and final tactical
conversion both mattered; the diagnostic provides no single-technique causal
verdict. Full histories, native inspection commands/output and the descriptive
[telemetry extraction](evidence/msh49-2/observations.json.gz) are retained.

## Baseline and handoff

These eight games demonstrate a substantial practical gap under the recorded
conditions. They establish the requested initial conventional baseline, with
no significance test, confidence interval, Elo estimate or population-strength
claim. The engine implementation remains the independently accepted MSH-49.1
checkpoint; this task changes no rules, search, evaluation, product protocol or UI.

Execution configuration/tool source was frozen at `cc2ec7c`; the participant and
runner build is `0b3b458`, with production-equivalent native acceptance at
`b8cc851` and the same engine executable hash as MSH-49.1. Final evidence-only
integration is identified by Git history and the task completion report.

Conventional **Shogi (MSH-50) and Xiangqi (MSH-54)** are next. Transfer established
techniques with native game-specific safeguards and proportionate correctness,
tactical and profiling checks. Do not repeat small Chess benchmark campaigns.
Deeper Chess strength analysis returns during **learned evaluation (MSH-55)**;
M5 and the other-game deliveries remain unfinished.

Independent final review approved the diagnostic and owner-rescoped MSH-49
checkpoint after reconciling all eight results, artifact hashes, controls,
native inspection and four independent offline native replays. No outstanding
technical or acceptance finding remains.
