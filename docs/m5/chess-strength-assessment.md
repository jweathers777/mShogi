# MSH-139 (Chess Strength Assessment)

Owner revision, 2026-11-09: **authorized discovery, no performance pass bar**.
The [unapproved qualification proposal](chess-strength-qualification.md) is
superseded. Determine current strength, explain weaknesses, report and stop for
John's next objective. No production tuning or cross-game transfer in this task.

## Frozen comparison

Current candidate/runner source is `9a509f077460e07341d2bccf830e4b02fcb0e95e`;
production is unchanged from accepted MSH-138 (Desktop Search Limits). Build
release/locked on the M1 Pro Mac. Retain actual binary/compiler/options hashes.
Both players receive only `go movetime 1000`, one search worker, no pondering,
strength limitation or own opening book. Preserve normal embedded knowledge;
record actual memory, do not silently shrink reference tables to mShogi's 4096
entries. No external tablebase files. Run games sequentially, without concurrent
heavy builds/analysis. Record OS/load and elapsed work. Native ARM64 references.

Use this single frozen [CCRL 40/15 list, November 16, 2024](https://www.computerchess.org/rating_list_all.html):

| Release/family | Single-CPU anchor (published margin) | Official source commit |
| --- | ---: | --- |
| Shallow Blue 1.1.0 | 1452 (20) | a59d0586d30bb961e72bbd3fb9d1929eec91156d |
| Shallow Blue 2.0.0 | 1616 (22) | a04fbd9861770c897eb566d83b0d2e3b17aa9fc0 |
| Oxidation 0.6.0 | 1943 (25) | 46a705b80e7191e7c302838eb8b85053b5994d65 |
| Goldfish 1.13.0 (added before its first game) | 1982 (18) | b7aed1c1b9188c55f4d773fd4fac43de30c83cbb |
| Simbelmyne 1.3.1 | 2460 (22) | ede2b01e408501b4f59a05c2951bba2dec5f6b74 |
| Arasan 15.6 | 2713 (24) | b1a161da023fb7c7d403b6e7ca0ecbd68247600e |
| Arasan 18.3 | 2889 (22) | 49b2ff652f543fcb84eb743f218342fb3f71c82d |
| Texel 1.07 | 3114 (9) | 1874e98e51276386f44d5992ce64e38fb1f17dfd |
| Arasan 21.2 | 3117 (14) | a318500eb78126cff635900b4f60fdfe25aaa3e3 |
| Laser 1.7 | 3256 (6) | 96bfbdba8d064442626e1bb23e02473a46559a2e |

These span discovery, not a tournament schedule or target. Start with Simbelmyne
near the middle of the useful range, subject to valid preflight. Build only needed references from official
repositories. Record compiler compatibility adjustments without gameplay changes.
Preflight excluded Simbelmyne 1.3.1: upstream FixedTime compares the requested
duration against that duration minus overhead, aborting around4096nodes. Its
unscored transport smoke returned in2–14ms; do not modify a rated release to fix
its gameplay. Start instead with the valid lower-middle Oxidation0.6.0 (1943),
with Arasan15.6 (2713) available if results justify moving upward. Both are from
the same pool, not handicapped engines. Sources: [Simbelmyne](https://github.com/sroelants/simbelmyne),
[Shallow Blue](https://github.com/GunshipPenguin/shallow-blue),
[Oxidation](https://github.com/Mathmagician8191/Liberty-Chess),
[Goldfish](https://github.com/bsamseth/Goldfish/tree/v1.13.0),
[Arasan](https://github.com/jdart1/arasan-chess),
[Texel](https://github.com/peterosterlund2/texel),
[Laser](https://github.com/jeffreyan11/laser-chess-engine).

Use the generic CC0 [Stockfish 8moves_v3 book](https://github.com/official-stockfish/books).
Pin archive/commit. Parse complete 16-ply standard-start prefixes; reject parse
errors, wrong lengths, duplicates of the same final board/side/castling/EP state,
or terminal positions before outcomes. Preserve all moves for native replay.
Partition roots into discovery/confirmation halves by frozen SHA256 key. Each
opponent/phase has its own deterministic seeded uniform draws **with replacement**.
Confirmation is fresh relative to discovery by construction; repeated draws within
a stream are retained/reported. This is a generic book, not individually certified
balanced openings. Root generation is offline; the native runner admits every
actual history/action/result.

## Adaptive inference and stopping

The observation unit is an opening **pair**, with reversed colors. Its score is
candidate points divided by two: 0, .25, .5, .75 or 1. Use the established
pentanomial representation ([Fishtest mathematics](https://official-stockfish.github.io/docs/fishtest-wiki/Fishtest-Mathematics.html))
and a proper conjugate Dirichlet(.5,.5,.5,.5,.5) prior on its five probabilities.
Direct posterior draws give score-mean equal-tailed **95% credible intervals**.
Repeat with .2 and 1 per bin for prior sensitivity. These are Bayesian conditional
intervals, not frequentist confidence bounds; optional sampling assumes stationary
independent opening-pair draws and the model/prior. Keep scouting and fresh
confirmation separately reported. Do not treat two games as independent trials.

Keep posteriors separate by exact opponent release and phase; family labels
identify independent coverage, not pooling. Map each posterior draw's score mean p to its conditional local reference index
`anchor + 400 log10(p/(1-p))`. Published anchor margins and time/architecture/style
transfer uncertainty are **separate**; these are not guaranteed simultaneous rating
intervals. Do not pool conflicting families or claim official CCRL/human Elo.
As a conservative supplemental check, report upstream [confseq](https://github.com/gostevehoward/confseq)
predictable-mixture empirical-Bernstein anytime score bounds, alpha=.05/(2*9)
per played opponent/phase (at most nine actual reference releases, including
adaptively added nearby references), independent with-replacement streams. Their broadness is not
a reason to run a disproportionate campaign; Bayesian intervals are primary.

Route from scores and uncertainty: after convincing losses move down, after
convincing wins move up; choose plausible near references for high expected
`p*(1-p)` logistic routing information proxy per observed pair-second and independent-family coverage.
A regularized Elo expected-score fit may guide routing, never replace per-family
inference. Start with small batches, avoid drawing conclusions from one pair,
and spend progressively more on competitive references. Record selection reasons.
Do not repeatedly sample a clearly mismatched engine merely to narrow its tail.

Stop when fresh paired results against **two independent nearby families** give
consistent, decision-useful estimates, prior sensitivity no longer dominates the
conclusion, and tactical/endgame/mistake evidence explains the main limitations.
A roughly 300–400-point credible range can be useful for the current engineering
decision; it is a soft precision guide, not an acceptance gate. Report broader
uncertainty if the marginal work would not change that decision; investigate
family disagreement. No predetermined game count, strength target or total-hour
allocation. If further work becomes genuinely substantial relative to the
information gained, report current evidence and a concrete resource tradeoff.

## Safety, integrity and quality

Reuse the existing native external UCI runner/replay and one pair per report.
Actual native conclusions only; **no evaluation adjudication**. A 512-ply cap,
protocol failure or cleanup failure stays unscored, never a draw or loss. Preserve
all attempts, including interrupted/capped games and partial pairs; analyze
missingness before inference; withhold population-strength interpretation if
material outcome-dependent censoring affects the eligible subset. Do not silently
replace adverse results.

Exclusive launcher lock, durable inflight PID/process-group receipt, exact binary
hash checks, bounded streams/artifacts, per-response/process watchdogs and a
per-pair worst-case timeout protect local resources. Track every launch, failed
attempt, replay and diagnostic in actual elapsed-time accounting. No arbitrary
campaign cutoff, paid compute, runaway daemon or unattended infinite retry.
Stop on infrastructure/identity failure and diagnose. Any subsequent new pair
requires an explicit retained investigation/resolution receipt for the failed
pair; failures are never converted to eligible data or silently replaced. Preserve original reports
compressed; native replay must pass before eligibility.

Use proportionate independent tactical and endgame positions with source/solution
provenance. A puzzle's setup move is not its solution. Independently check legal
alternatives; distinguish search misses, strategic opinions and tablebase facts.
Inspect representative actual mistakes with bounded unrestricted Stockfish 19
analysis, including depth/work telemetry, without tuning the frozen candidate.
Quality scores do not become a rating. No broad packaged GUI/native CI campaign
for unchanged production code. Offline instrumentation tests and independent
method/result review are required. Final findings precede any owner decision
about a target, further Chess development or transfer.

Actual builds, accounting, commands and results: [structured record](evidence/msh139/verification.json).
