> **Superseded, unapproved historical proposal.** The owner replaced this proposal
> with [MSH-139 (Chess Strength Assessment)](chess-strength-assessment.md) on
> 2026-11-09. Its targets, sample sizes, budget and transfer gate below are not
> current requirements or authorizations. No qualification games ran.

# MSH-139 (Chess Strength Qualification): approval proposal

**Proposed, not approved or executed.** Owner consolidation, 2026-11-09.
MSH-138 (Desktop Search Limits) supplies time-primary desktop execution;
MSH-137 (Chess Search Refinement) supplies engineering evidence. Neither alone
qualifies practical playing strength. Cross-game transfer remains blocked on
this qualification. Learned evaluation stays later.

## Recommended standard

Aim for a **local reference-index point estimate of at least 3100 and a one-sided
95% lower bound of at least 3000**, supported by two independent nearby engine
families. Each family's point estimate must also be at least 3000; disagreement
exceeding 150 index points requires investigation and an inconclusive verdict,
rather than pooling away a collapse. No unresolved basic tactical/endgame defect
identified by the complementary checks may be dismissed solely because the pooled
match score passes. Approval covers this interpretation as well as the budget.

This is an ambitious classical-engine bar, not Stockfish parity or a convenient
weak-baseline improvement. One fully retrievable **historical** [CCRL40/15 pool,
November 16, 2024](https://www.computerchess.org/rating_list_all.html) has these
single-CPU reference entries (do not mix in 2026 snippets or 4CPU rows):

| Full release | Published reference rating |
| --- | ---: |
| Arasan 18.3 | 2889±22 |
| Crafty 25.2 | 2953±9 |
| Texel 1.07 | 3114±9 |
| Arasan 21.2 | 3117±14 |
| Arasan 22.3 | 3188±19 |
| Laser 1.7 | 3256±6 |

The target approaches Texel/Arasan 21.2, above Crafty and below stronger classical
Arasan/Laser releases. It is a **conditional local index against rated references**,
not an official CCRL, FIDE or USCF rating. CCRL's [conditions](https://www.computerchess.org/about.html)
include calibrated Intel hardware, much longer thinking time and tablebases.
One-second ARM64 matches cannot establish the same strength scale independently
of time/architecture/style effects. Publish actual scores and family estimates
alongside the conditional index; statistical intervals do not remove that caveat.

## Efficient opponent selection and confirmation

Start with full-strength **Arasan 18.3**. Scout in batches of two opening pairs
(four games), at most 24 games total. Use a broad draw-aware Bayesian rating model
with uncertain published anchors. Select the next available reference for expected
uncertainty reduction per unit of expected match time, informed by current scores
and independent-family coverage—not a numerical midpoint. Move down as well as
up; do not waste a fixed tournament on clear mismatches. If scouts show the engine
clearly unready for the target, stop and direct conventional engineering instead
of spending the confirmation budget. Scout inference is model-dependent and
cannot itself qualify the engine.

Once nearby opponents emerge, freeze the candidate, two independent reference
families, options, opening sample and analysis rule. Near the proposed target,
prefer **Texel 1.07 and Arasan 21.2**, subject to source/build preflight. Play
**64 fresh opening pairs / 128 games**, 32 pairs per family. Organize four blocks of 16
new distinct roots, one opponent per block. Use pair-sized native reports, as the
existing launcher does, to respect byte caps; do not assume a whole block fits
one artifact. Use one
final qualification analysis; do not repeatedly inspect a 95% interval and stop
when it happens to pass.

Preserve the five pair-score bins and opening lineage. Use a draw-aware rating
fit with **opening-pair block** uncertainty and published-anchor uncertainty.
A predeclared pair-block bootstrap is an approximate conditional interval, not
guaranteed coverage for CCRL transfer; degenerate outcomes require a finite
conservative pentanomial/profile bound or an inconclusive result, never zero
uncertainty. Freeze that calculation before confirmation. [Fishtest mathematics](https://official-stockfish.github.io/docs/fishtest-wiki/Fishtest-Mathematics.html)
explains paired modeling; [Bayeselo](https://www.remi-coulom.fr/Bayesian-Elo/)
provides the draw-aware rating basis. Around balanced scores this sample offers
roughly 80–120 index-point uncertainty, not a promised ±50 estimate. Report family
heterogeneity rather than hiding it in one number.

Use the established generic [Stockfish `8moves_v3` book](https://github.com/official-stockfish/books/blob/master/README.md),
whose repository [license is CC0](https://raw.githubusercontent.com/official-stockfish/books/master/LICENSE).
Pin its commit/archive hashes and sample without replacement with a frozen seed;
retain all 16 plies of native history. Scout and confirmation roots are disjoint.
Predetermine illegal/terminal/duplicate-root exclusions; do not filter openings by
candidate performance. This is a generic book, not a claim that every root has
independently certified equal evaluation. Random legal walks from the stopped
older study are not the proposed qualification population.

## Conditions, diagnostics and cap

Both players receive only **`go movetime 1000`**, without external depth/node or
strength-limit controls. Use one search thread each, pondering off and no own book;
retain normal embedded evaluation/endgame knowledge. Run one game at a time on
the same plugged-in M1 Pro/32GiB Mac, with actual OS/compiler/architecture/load,
wall time and reported search work recorded. Use native ARM64 reference builds,
not silently emulated binaries. Request 64MiB Hash where supported for references;
mShogi retains its 4096-entry TT. Record actual unequal memory and absence of
external tablebase files; do not claim CCRL conditions were reproduced.

Complement games with a frozen theme-stratified sample of 96 [CC0 Lichess puzzles](https://database.lichess.org/)
and 24 independently certified endgame positions (KPK, KQK, KRK and selected rook
endings). Retain original solutions/IDs, apply the puzzle's first setup move,
natively replay every line and verify acceptable alternatives. Use independent
[Syzygy WDL/DTZ evidence](https://github.com/lichess-org/lila-tablebase#http-api)
with explicit fifty-move semantics; DTZ is not mate distance. Report tactical and
positional mistake categories, and inspect at most 20 significant mistakes with
full-strength Stockfish 19 at up to 5 seconds each. These are complementary quality
evidence, not a rating derived from puzzle scores. The suite does not require
solving every puzzle: verified elementary defects need correction, while difficult
misses remain diagnostic findings.

**Requested total cap: six hours aggregate active local work**, including
preflights, smoke/failures, games, tactical/endgame searches, bounded reference
analysis and cleanup; no paid compute. Maximum 24 scout+128 confirmation games,
with no additional longer-time match sample. This approval covers one frozen
candidate campaign; later source changes do not restart its cap or authorize
reusing confirmation roots for acceptance. At 100–120 searched plies/game,
search time alone is roughly4.2–5.1 hours; longer games/overhead can exhaust the
cap. Stop without extending it. Incomplete confirmation is inconclusive. A
failure or 512-ply cap remains unscored and preserved; do not replace adverse
results or infer a game outcome from an engine failure/evaluation. Qualification
requires complete eligible confirmation pairs and diagnosed failures.

Reuse the existing native runner, UCI external adapter, paired reports and replay
from MSH-49.2 (Stockfish Baseline), with small offline orchestration/statistics.
No new engine protocol, supervision framework or competing evidence system.
Reference source pins below establish feasibility, **not completed M1 builds**;
actual executables/options must pass bounded preflight before games.

| Reference | Official source pin |
| --- | --- |
| Arasan 18.3 | [`v18.3.0`](https://github.com/jdart1/arasan-chess/tree/v18.3.0), `49b2ff652f543fcb84eb743f218342fb3f71c82d` |
| Arasan 21.2 | [`v21.2.0`](https://github.com/jdart1/arasan-chess/tree/v21.2.0), `a318500eb78126cff635900b4f60fdfe25aaa3e3` |
| Texel 1.07 | [`1.07`](https://github.com/peterosterlund2/texel/tree/1.07), `1874e98e51276386f44d5992ce64e38fb1f17dfd` |

Arasan's [Mac/Clang instructions](https://github.com/jdart1/arasan-chess/blob/v18.3.0/doc/INSTALL)
and portable source support the initial choice; Texel's older build needs preflight.
Resolve exact alternative same-pool releases if necessary before sample freezing,
without weakening them or changing the approved bar. No reference was built or
run during proposal preparation. The approval decision is target+protocol+budget;
research and ordinary engineering remain autonomous.
