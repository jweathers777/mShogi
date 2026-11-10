# Small Chess decision-quality benchmark

Version 1, established by MSH-155 (Chess Time Scaling). The [positions](chess-decision-benchmark.json)
are frozen independently of the engine answers. Eight roots give two examples per
primary category and four fresh positions; categories overlap. This is routine
engineering evidence, not a strength rating, comprehensive conformance suite or
population estimate.

| Category | Historical control/weakness | Fresh independent root |
| --- | --- | --- |
| Tactical | 22.Nf5, known coordination weakness | SfkRC: unique mate within five plies |
| Positional | 24.h4, successful-game control | STS Undermine.004: published pawn-break preference |
| Defensive | 23...Rf6, known tactical defense weakness | Réti: exact draw-preserving move |
| Endgame | 34.Rc5, winning-game rook control | Dedrle: exact win-preserving move |

The JSON preserves origin, complete known history, exact root FEN, expectations,
source provenance and whether a position is fresh. Four historical roots and the
mate puzzle have complete move histories. EPD/study roots explicitly have no known
prior history and begin with the stated counters. Never replace historical roots
with bare FENs in performance or decision comparisons.

## Outcome rules

Keep separate measures; do not sum them into one accuracy or rating:

- **Exact mating objective:** the unique move forces mate within five plies under
  all legal defenses. A different move misses this bounded objective; it is not
  necessarily a losing move or incapable of a longer mate.
- **Exact tablebase objective:** retain the root WDL using the complete legal-move
  map. Child-side outcomes are inverted. Preserve counter conditions and distinctions
  for cursed/blessed outcomes if future roots include them. DTZ is not mate length.
- **Published positional preference:** report the supplied STS points for listed
  moves as a soft preference. Unlisted moves remain unresolved, not automatically bad.
- **Historical decisions:** use finite-reference-supported acceptable sets, not
  unique-best matching. Earlier h4/Rc5 evidence was exploratory; bound-aware
  adjudication receipts qualify later statements. A materially favorable margin
  changing does not prove a lost win. New alternatives remain unresolved until
  independently assessed; never add them merely because mShogi chose them.

Full PV legality establishes admission only. Inspect best-defense consequences,
score bounds and endpoint outcomes before diagnosing why a decision failed.
Classical evaluation, search coverage, execution time and match strength are
separate questions. Seeing a reply in a PV does not prove it was adequately valued.

## Routine use

Use the existing `chess_frontier_profile` example with this JSON and a release
build; each root's `time_ms` is the one-second baseline. For a selected longer
comparison, make a scratch copy changing only the requested budget. Preserve cold
TT, source/binary/configuration hashes and full history. The MSH-155 evidence driver
records both mShogi and external comparisons, but optional reference engines/APIs
are not normal build/test/CI dependencies.

The normal baseline is small enough for routine use. Full 1/5/15-second matrices
and selected 30-second investigations belong to coherent checkpoints, not every
edit. Record actual time, returned move, exact native completion status, main/q work,
PV/score and outcome tier. Require meaningful decision non-regression and cost
improvement where appropriate; do not demand a new move on already solved controls.

These four fresh positions cease to be fresh holdouts once this evidence guides
implementation. Preserve them as regressions and select additional independent
holdouts before claiming transfer. Freeze expectations before new answers; append
new versions/provenance rather than silently making the benchmark easier.
