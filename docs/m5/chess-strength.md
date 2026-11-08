# MSH-49: Chess baseline and release-strength decision

**In Progress: the owner-approved first-release study is being implemented.**
This record preserves pre-target calibration and the necessary comparison-harness
correction, followed by the approved protocol and candidate work. It is not
a Chess release-strength verdict or M5 acceptance.
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

## Owner-approved first-release criteria — 8 November 2026

MSH-49 AC2/AC4 and design D-09 explicitly require an owner-selected release bar
and uncertainty criteria. After calibration, John selected the first option in the concrete second
question: “Okay. Let’s proceed with your first option for question 2.” This
explicitly approves the criteria and budget below; it supersedes the earlier
unanswered/replacement state.

Approved first measured conventional-engine release, with **no human-Elo claim**:

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

Feasibility is not established by the small calibration. The approved bar
remains binding if a candidate fails it. A stronger external/club-level target requires a different
explicit benchmark and measured implementation/budget scope. Current material
and full-width code are not presented as meeting either target.

The Chess external adapter now accepts optional per-participant effective limits,
indexed by participant across color swaps; omitted limits preserve the historical
shared configuration and digest. Strict replay binds each actual go command to
the correct native actor and limit. The native experiment path already supports per-participant limits
but has no Stockfish adapter. These are real consumer requirements, not a reason
to introduce another engine protocol or application runtime. Current UCI can
emit a labelled legal fallback when no iteration completes; strength eligibility
must audit completed depth and diagnostics, rather than equate any bestmove with
completed search. Native internal execution retains honest Incomplete results.

## Handoff

### Development observations; excluded from acceptance

The protocol, sampler and12-root public development catalog were frozen at
`7004d47` before candidate match evaluation. No formal seed or roots have been
generated. Initial lexical alpha-beta plus the original positional evaluator
won8/8 games against the frozen baseline on the first4 development roots.
On the first2 roots against the pinned depth2 Stockfish, it scored0W3D1L (37.5%);
the frozen baseline scored0W0D4L. All16 development games concluded natively,
with completed-search observations and no protocol, cleanup or eligibility
failure. These small selected development results are not acceptance estimates.

The [development source reports](evidence/msh49/development/) retain the actual
results, executable hashes and exact7 production-source overrides verified
against their pre-match manifest. Candidate development matches consumed827.281s;
with the conservatively charged54s calibration, total charged work is881.281s.
Most time was native claim enumeration and immutable baseline search. The
current candidate adds capture/promotion ordering; its static evaluator is
unchanged. A native necessary-condition shortcut avoids projecting all intended
claims below halfmove99 when every known identity has occurred only once.
Independent proofs and complete ordered-catalog tests preserve all claim
semantics, history/undo/replay, threshold/EP/rights and overflow behavior.

The old claim-admission correction at `926ae8f` actually passed required native
Linux/macOS15/Windows CI; this does not cover the subsequent candidate source.
Independent source review approved candidate search/evaluation, production
routing, native shortcut, per-participant limits and strict offline study audit.
Final source platform receipts and the fixed held-out strength study remain
required.

MSH-49 remains unfinished until the approved criteria and verification pass.
The reviewed conventional candidate is integrated and pushed at
`7818c3371758cee8e9624b58b0edb1073c32c89d`.
Its [clean-source artifact freeze](evidence/msh49/study/candidate-freeze.json)
precedes any held-out seed. The new development pair won both games in90.206s,
versus168.570s initially; total charged work is now971.486s. Independent review
reconciled all9development pairs and source/artifact identities. The final
runner has its own artifact hash, distinct from the development runner.

Required native [CI42469534590](https://github.com/jweathers777/mShogi/actions/runs/42469534590)
is running at that exact source, with all3hosted runners acquired. Its outcome
is pending. No formal holdout roots or acceptance matches have been generated.
Measured costs still project beyond the approved two-hour budget. John has been
asked to amend only the total local match budget to four hours, including work
already charged, before held-out sampling. No amendment is inferred; the
two-hour cap remains binding pending that decision.
No learned evaluator, training or other game task has begun. The bounded baseline and reviewed
claim-tail correction are useful preparation, not substitute acceptance.

The immutable protocol/sampler/development freeze is recorded in
[study/protocol-freeze.json](evidence/msh49/study/protocol-freeze.json), with the
[full approved test rules](evidence/msh49/study/protocol-v1.json). The sign tests
concern positive versus negative nonzero paired differences, not a mean-score
confidence interval; score floors and positive external mean are separate gates.
Total charged match time above 7,200 seconds makes the study ineligible.

Implement the minimum
measured conventional improvement using native history/claims/terminal/undo,
verify it with independent oracles and actual execution, and run the approved
study. Preserve the frozen baseline and historical reports. MSH-50–55/104 remain
outside this assignment.
