# Conventional Chess decision-quality round

MSH-156 (Independent Chess Benchmark) and MSH-157 (Conventional Decision
Improvement), 2026-11-10. The round repairs a measured native history/claim cost
cliff but **does not demonstrate broadly better Chess decisions**. At five seconds,
238 of 240 moves are unchanged, including all 80 held-out moves. Bounded development
investigation did not establish a safe general search or evaluation correction.
The predeclared broad decision-improvement gate is not met.

## Scope and independent measurement

The [benchmark protocol](independent-chess-benchmark.md) freezes 240 public-source
positions: 96 puzzles, 60 strategic tests, 36 multi-PV reference positions and 48
exact endgames. There are 160 development and 80 held-out positions, with source
families/known continuations/symmetries/near-board relations separated. Whole STS
themes are split between collections. The older eight roots are regression-only.
141 positions retain complete verified source-game histories; 99 have explicitly
unknown earlier history. Four-field evaluation FENs use declared test counters,
not invented historical facts. The [data package](evidence/msh156/corpus/README.md)
retains provenance, exact reconstruction and limitations; STS source fixtures use
retrieval references because upstream redistribution permission was not established.

Expectations were frozen before native measurement. Three immediate-mate proofs
and 48 complete tablebase maps are exact under their documented contexts. Other
puzzle solutions, STS points and finite evaluations remain published preferences
or opinions. An unlisted move is unresolved, not automatically wrong. Passing a
one-step WDL objective is not proof of endgame conversion. Prefix sampling and
published-problem selection are not a representative random sample of ordinary play.

The predecessor is production `d53f344`, binary `55b38b74…`; candidate production
`3b52314`, binary `8355dddd…`. The [freeze record](evidence/msh157/candidate-freeze.json)
preceded held-out measurement. Five seconds is primary; the fixed 40-position
one-second subset uses development roots. Fifteen-second and matched-depth probes
were development-only. Cold processes, one native thread, 4096 TT entries, no visit
ceiling and the depth 128 safeguard are shared. Early solved-mate completion at128
is not useful depth 128. Held-out and fast comparisons alternate version order by
root index. No tuning follows held-out disclosure.

## Established limitations and competing explanations

The five-second development predecessor preserves all 32 exact tablebase outcomes
and the immediate mate, and chooses 60/63 published puzzle preferences. Strategic
choices contain 17 maximum-point moves, 11 lower listed choices and 12 unlisted
alternatives; 17/24 evaluation choices are within their finite preferred sets.
These are separate authority tiers, not a combined accuracy score or rating.

Independent equal-three-second Stockfish19 analyses support three concrete
continuation weaknesses among the selected development cases:

| Case | Inferior sampled choice and supported alternative | What the probes establish |
| --- | --- | --- |
| Puzzle 0UOyM, defensive pawn ending | Kg6 versus exf3+ followed by …Kh6–g5 | The predecessor still chooses Kg6 at 15s. Forcing each root move into a fresh depth 13 child search still ranks the en-passant branch lower. The selected leaf arithmetic is coherent; the engine misses the useful king-route timing and distant pawn-race consequences. This does not isolate valuation from selective horizon. |
| Puzzle 0m5mI, quiet bishop ending | Bd6 versus Kc3, with a later Bc1–d2 blockade | At15s the root choice becomes Kc3, but its PV does not establish the reference blockade and its score remains near equality. Correct move selection alone does not prove accurate forecasting. |
| Puzzle 28pJK, quiet queen activity | Qf4 versus Qc8 | The stronger defense to Qf4 is quiet Qc1. At15s the engine finds Qc8 and includes …Rd2/…Qa6+ in its continuation, while valuation remains materially different from the finite reference. |

These are supported examples, not an estimate of how often these weaknesses occur.
The [reference findings](evidence/msh157/reference-findings.md) retain candidate
pairs, legal continuations and contrary evidence. Strategic gaps were often modest;
for example the lower-scored Undermine choice retained a large reference advantage.
It was not relabeled a lost win.

The [bounded coverage investigation](evidence/msh157/coverage-findings.md) uses
42 native probes, 247.829 seconds, including orthogonal LMR/quiet/SEE/delta controls,
six completed-depth comparisons and two full-history child probes. Disabling LMR
or quiet omissions sometimes selected published strategic alternatives but lost
completed depth and produced other unresolved choices. Both default and no-LMR
searches select the same Recapturing move at depth 7: this demonstrates a horizon
confound, not a unique explanation of the deeper choice or proof that LMR is safe
at all greater depths. SEE/delta removal did not repair the difficult endings.

The child-root probes reset TT, windows, ordering and extension context. In
particular the en-passant child is checked; a new ply0 root does not receive the
same interior check extension. They are supporting nominal-horizon evidence, not
exact replicas of an interior search call. No particular incorrectly omitted
continuation or safe general valuation correction was localized sufficiently to
justify changing production. Independent [method review](evidence/msh157/round-method-review.md)
approved stopping this bounded investigation, with those limitations.

## Corrections selected and measured cost

The native game now caches whether a repetition exists in the current reversible
segment, restoring the previous value on undo. Complete public history and the
archival occurrence map remain intact. A pawn move, capture or castling-right loss
makes older identities unreachable; the cache prevents those obsolete occurrences
from forcing expensive claim catalogs and disabling checked previews.

Claim catalogs also reuse one authoritative checked projection per intended move
across threefold/fifty-move reasons. They retain policy ordering, counter semantics
and the unchanged public single-claim validator. No second move generator,
wholesale representation rewrite, unsafe code, evaluator bonus or pruning change
was introduced.

Ninety-nine fixed-depth predecessor/candidate comparisons preserve decisions,
scores, visits and completed depths. Three alternated one-second pairs on each
selected cost root show:

| Workload | Median predecessor visits | Median candidate visits | Ratio | Decision |
| --- | ---: | ---: | ---: | --- |
| Real obsolete-repetition history | 114,106 | 440,005 | 3.86× | Rxc5 unchanged |
| Live-repetition history | 96,893 | 392,284 | 4.05× | Ne2 unchanged |
| Ordinary-history control | 243,034 | 251,958 | 1.04× | Nf5 unchanged |

These are local own-visit measurements, not general speed or strength estimates.
The small ordinary-control difference deserves caution. The changes were measured
together; the experiment does not separately attribute every gain. Tiny fixed-depth
samples support semantic equivalence and the affected cost cliff, not an overall
workload speedup. [Raw cost evidence](evidence/msh157/cost/summary.json) preserves
all timings and the initial diagnostic validation failure plus reviewed retry.

## Frozen decision comparison

| Collection / time | Same move | Changed move | Exact outcomes | Supported broad benefit |
| --- | ---: | ---: | --- | --- |
| Development 160 / 5s | 158 | 2 strategic alternatives| 32 WDL and 1 mate preserved|Not established |
| Held-out 80 / 5s | 80 | 0 | 16 WDL and 2 mates preserved|None observed |
| Fixed development 40 / 1s | 39 | 1 finite-reference alternative| 10 WDL preserved|Not established |

No primary exact regression is observed. Unchanged decisions are not automatically
correct: unresolved alternatives and known weaknesses remain unresolved/weak. Both
changed strategic choices move from unlisted alternatives to maximum-point source
preferences. This is not sufficient to call them improved; paired continuation
analysis is recorded separately below. The one-second change from Rh1 to Re1 in
`lichess-eval-c3ddf47932005fb2` enters the finite preferred set but is unadjudicated
and remains unresolved. It is secondary, not evidence of independent transfer.

Paired three-second, cold-process Stockfish19 analysis supports a local preference
for Bb1 over Qb4 in Open Files.037 (+117 versus 0 root-side centipawns). The Qb4+
continuation trades queens and takes kingside pawns while the opposing e-pawn and
active rook compensate; Bb1–e4 retains bishop pressure and supports recovering e6.
This is finite continuation evidence, not a proved lost win. In Outposts.023,
Ng5 versus Rh8 receives only a small finite preference (+48 versus +17), with plausible
continuations on both sides; classify its practical benefit as unresolved. Thus
one local improvement is supported and the other primary change remains uncertain;
no supported primary regression was identified. The raw tier grader continues to
label both changes unresolved because it records source authority, with this
separate adjudication retained in [final reference findings](evidence/msh157/final-reference-findings.md).

The full held-out collection has no move changes, so the requirement for supported
favorable changes across more than one domain including held-out material fails
regardless of the two strategic opinions. No aggregate accuracy or rating is
computed. The sampled exact objectives are ceiling-limited and test root WDL/mate
preservation rather than complete conversion.

An additional historical exposure audit accidentally overlapped 19 held-out roots.
The audit was stopped, its incomplete attempt retained, and both versions repeated
on those timestamp-selected roots without the audit. Original primary results
remain evidence; confirmations are not a favorable substitution or extra
independent sample. All 38 confirmation attempts retained their original moves, and both versions
agreed on all 19 repeated roots. This supports decision consistency under the
quiet repeat; it does not erase timing uncertainty in the original work/depth
measurements. See [confirmation](evidence/msh157/final-review/quiet-confirmation.json).

The historical exposure audit conservatively identifies one prior-exposed
**development family of two roots**: the king-draw board in MSH-139 (Chess Strength
Assessment) is horizontally related, with different side-to-move and counters.
The frozen corpus remains unchanged and both roots remain development/regression
material. Excluding that whole family leaves 238 roots (158 development/80 held-out),
with 156 unchanged development choices and the same two unresolved strategic changes.
No held-out source/board matches were found in the audited scope. The audit covers
2,079 source/evidence files expanded to 3,013 payloads, including all M5 engine
evidence; 2,896 platform GUI/snapshot entries were excluded explicitly. Unrecognized
encodings and unknown latent families remain limitations; this is not a claim of
universal historical novelty. See [freshness review](evidence/msh157/final-review/freshness-review.md).

The first audit was stopped after 155.19 CPU seconds. A later broad scan hit its
600 MiB decompression cap after 73.919 CPU seconds; a scoped retry also hit the cap
before the final narrower scan completed. Failed attempts remain documented. Exact
CPU/wall for the latter two offline passes was not captured, so known audit CPU
229.109 seconds is a lower bound, not a complete total. These costs are separate
from the fully ledgered active engine-search allowance.

## Verification, limits and next decision

Level 1 applies to portable native history/query changes. Core 232 tests, Chess
search 79 tests (one ignored profile) and console claim admission 1 test passed, along
with the release build, independent source review and source-hash checks. The [independent final evidence review](evidence/msh157/final-review/independent-final-review.md)
and [independent closure review](evidence/msh157/final-review/native-independent-closure.md)
approve diagnostic closure without a broad strength claim. No
Linux/Windows or GUI campaign was run; broader compatibility remains for a
risk-required integration/release checkpoint. Data validators and diagnostic
supervisor/grading fixtures are separate from native rules correctness.

All 686 engine attempts across native searches, references and cost profiles
used **2,705.534 active engine seconds (45.09 minutes)**, including the first failed
cost-harness validation. The effective ledger charge with unused external
reservations is 2,847.377 seconds, below the frozen 3,300-second ceiling. All search
UTC intervals are nonoverlapping. The final independent audit verifies 96 source
hashes and 5,343 original artifact hashes; audit scripts and original/archived
fingerprints are preserved in the [evidence package](evidence/msh157/README.md).
Compilation, correctness tests, data retrieval and offline proof/audit work are
separate; the incomplete offline audit CPU accounting is disclosed above.

No opponent games, tournament, learned evaluation or Shogi/Xiangqi transfer were
performed. The exact cost gains do not establish substantially better Chess or a
new rating. Retain these cost repairs on their demonstrated merits, but reconsider the
continuation-quality diagnosis before another engineering series. Faster execution
of substantially the same selective search did not produce broad decision transfer.
The next useful question is where important quiet continuations first diverge in
coverage or value, using comparable search context and independently evaluated
intermediate states. A PV match, global pruning toggle, or nominal child depth is
insufficient to answer it. This is a revised direction, not an automatically
authorized queue of more narrow fixes. MSH-153 (Classical Compensation
Calibration) remains held: current evidence does not isolate a general compensation
term. A competitive campaign requires meaningful decision progress and separate
approval; a throughput gain alone is insufficient justification.
