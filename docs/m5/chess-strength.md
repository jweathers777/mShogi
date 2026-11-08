# MSH-49: Chess baseline and release-strength decision

**Unfinished: owner target selection is pending.** This record contains bounded
pre-target calibration and a necessary comparison-harness correction. It is not
a Chess release-strength verdict, a search improvement or M5 acceptance.
The original four MSH-49 criteria remain unchanged and unchecked.

## Frozen baseline and actual independent opponent

The baseline is production source
`05eeba326341fb65efa0e60f2641795c5276e863`, built with pinned Rust 1.99.1 using:

```sh
cargo build --release --locked -p mshogi-console --bin mshogi -p mshogi-runner --bin mshogi-match
```

Native production source was clean at build; task ownership notes were the only
pending edits. Frozen executables were copied to `/private/tmp/msh49/baseline`
before any admission correction. mShogi SHA256 is
`a3c0cda3e4da2b00b931e0100000387fa9d34835d0e6565587a02f7750dd1567`.
The material/full-width search and authoritative Chess rules are unchanged.

The optional opponent is the [official Stockfish 19 release](https://github.com/official-stockfish/Stockfish/releases/tag/sf_19),
reacquired into temporary storage. Archive and executable matched the exact
historical M2 hashes before execution. Bundled notices/source/Copying.txt remain
there; no opponent binary, network, source or runtime dependency is vendored or
shipped. Actual `license` output identifies GPLv3. Its embedded network identity
and observed executable hash are retained in every actual report.

The shared existing UCI adapter executes both processes; the native AutoClaim
controller owns claims/adjudication. Complete MatchRecord histories, identities,
protocol traffic, failures and cleanup are preserved in the versioned external
wrapper. The [verification ledger](evidence/msh49/verification.json) contains
build/acquisition provenance and the six compressed replayable source reports.
These are new local macOS observations, not claims of external-opponent execution
on Windows/Linux.

## Bounded calibration, before target or candidate selection

Six project-authored prefixes were chosen before running any candidate:
standard start; `e4 e5 Nf3 Nc6`; `d4 d5 c4 e6`; `c4 e5 Nc3 Nf6`;
`Nf3 d5 g3 Nf6`; and `e4 e6 d4 d5`. Exact legal UCI strings and complete native
starting histories are in the report configs. These are a small fixed diagnostic
catalog, not a sampled population, training corpus or chosen strength standard.
Each was played in both color assignments.

Both engines received depth 2 / 10,000 nodes, untimed, AutoClaim, with a 128-total-ply
cap. Stockfish used Threads 1, Hash 16, MultiPV 1, Skill 20, UCI_LimitStrength false
and Ponder false. Fresh processes start each game. The batch scheduler stopped
before scheduling another pair after a nominal 120 s budget; all six pairs finished
within that budget. The old external API has no hard whole-experiment deadline.

| Opening family | mShogi wins | Draws | Stockfish wins |
| --- | ---: | ---: | ---: |
| Standard |0|0|2|
| Open |0|0|2|
| Queen |0|0|2|
| Flank |0|0|2|
| Fianchetto |0|1|1|
| French |0|1|1|
| Total |0|2|10|

All 12 games reached genuine native conclusions: ten mates and two threefold
claims. Score was 1/12 points, 8.33%. All 353 mShogi requests completed depth 2,
41–2257 visits each; no fallback, stderr, truncation, process failure or cleanup
failure was observed. Independent read-only inspection rechecked identities,
hashes, colors/limits, native final records and replayed all six source reports.

This establishes a weak baseline under these precise conditions. It does not
estimate human Elo, population strength, a candidate improvement or an approved
release threshold. [Stockfish's own documentation](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-Protocol-and-Stockfish-Commands.html)
explains that its reduced strength settings are probabilistic and its Elo
calibration is tied to particular conditions; shallow fixed-node tests do not
inherit a human rating. These pilots used Skill 20 with limiting disabled.

## Reproduced claim-tail admission correction

An actual current child returned a legitimate depth-one continuation from the
accepted native fixture `7k/8/8/8/8/8/8/KR6 w - - 98 1` under AutoClaim:
White `a1a2`, then Black's intended fifty-move claim. Telemetry had depth 1,
16 visits, score 0, Complete and two commands. Board depth counts the first move;
the terminal claim is not another played ply.

The MSH-48 harness incorrectly limited the entire PV command count to depth,
refusing this valid move before native admission. The controlled actual-child
regression failed on that exact signature. The narrow correction permits at most
`completed_depth` board moves plus one terminal claim. Existing full native PV
re-admission still requires the claim last, correct actor and native legality.
It does not allow an extra board move or play an intended claim move.

The same actual-child test then passed both color assignments: one actual board
move followed by an actual claim, native FiftyMove draw, unchanged source/config,
exact history/observation/profile and replay. A negative test rejects an extra
ordinary board ply; nine focused admission regressions and the complete runner
suite passed. Independent source/evidence review approved the correction.
Rules, search/evaluation, protocol identities and other games were not changed.
Required native Ubuntu/macOS 15/Windows verification for the frozen correction
checkpoint remains separately recorded in the ledger.

## Concrete first-release proposal — not yet approved

MSH-49 AC2/AC4 and design D-09 explicitly require an owner-selected release bar
and uncertainty criteria. The initial direction question was followed by this
reviewable proposal after calibration; neither question has been answered.

Proposed first measured conventional-engine release, with **no human-Elo claim**:

1. Candidate versus frozen engine: 64 held-out opening pairs, score at least 70%,
   and one-sided exact paired win/loss test `p <= 0.05`. Both mShogi participants
   receive 10,000 visits and maximum depth 8. Different completed depths under an
   equal visit ceiling are intentional; this is not an equal-wall-time claim.
2. Candidate and frozen engine versus the same pinned Stockfish 19 reference:
   each plays both colors at 32 held-out roots. Stockfish is explicitly capped
   at depth 2 / 10,000 nodes; mShogi uses depth 8/nodes10,000. Candidate must score at
   least 25%, have positive mean improvement over the frozen engine on matched
   roots, and pass a one-sided exact test of positive versus negative matched-root
   score differences at `p <= 0.05`. The handicap is part of the benchmark, not
   an estimate of unrestricted Stockfish strength.
3. Freeze the sampler, independent held-out draws, exact roots/seeds, eligibility
   rules, parameter/settings identities and fixed sample sizes before candidate
   evaluation. Development roots remain separate. Color games form one unit,
   not two independent observations. For the relative test, count pair scores
   above/below equality; ties are excluded by the prespecified sign test. For
   external improvement, the four games at one root form one matched unit.
   The [primary paired-test research](https://official-stockfish.github.io/docs/fishtest-wiki/Fishtest-Mathematics.html)
   motivates paired accounting; this proposal is not Fishtest SPRT or Elo.
4. Require eligible games to conclude natively with completed-search observations.
   Keep failures/caps/incomplete/fallback evidence; do not score them as losses or
   draws, silently discard pairs, or claim acceptance from an incomplete study.
   Evaluate the fixed samples once; no optional stopping at a favorable p-value.
5. Up to two hours of local match work, stopping new batches at the budget and
   retaining any incomplete study. No paid compute. Ordinary correctness checks,
   independent review and required native CI remain separate obligations. If the
   bar or budget is unmet, preserve unfinished status rather than lower it.

This proposal needs owner selection; its feasibility is not established by the
small calibration. A stronger external/club-level target requires a different
explicit benchmark and measured implementation/budget scope. Current material
and full-width code are not presented as meeting either target.

The current Chess external adapter has one shared WorkLimit, so implementing
this proposed per-participant study requires a narrow effective-limit/report
extension. The native experiment path already supports per-participant limits
but has no Stockfish adapter. These are real consumer requirements, not a reason
to introduce another engine protocol or application runtime. Current UCI can
emit a labelled legal fallback when no iteration completes; strength eligibility
must audit completed depth and diagnostics, rather than equate any bestmove with
completed search. Native internal execution retains honest Incomplete results.

## Handoff

MSH-49 remains unfinished while the owner decision is pending. No candidate
search/evaluation improvement, accepted-strength study, learned evaluator,
training or other game task has begun. The bounded baseline and reviewed
claim-tail correction are useful preparation, not substitute acceptance.

After selection, record the decided bar before evaluation, implement the minimum
measured conventional improvement using native history/claims/terminal/undo,
verify it with independent oracles and actual execution, and run the approved
study. Preserve the frozen baseline and historical reports. MSH-50–55/104 remain
outside this assignment.
