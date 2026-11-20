# MSH-185 (Chess Strength Anchor): method before play

Date: 2026-11-16. Owner: Codex, branch `codex/chess-strength-anchor`.
Candidate source: `f433ea13ef0c6682d7c84388142dc9c8238e4e16` (task-start main).
No engine changes are permitted in this measurement.

## Objective and comparison

Estimate present strength against independent, full-strength rated engines.
The previous conditional local estimate was about 1900, with a 1700–2200 range,
in MSH-139 (Chess Strength Assessment). MSH-161 (Competitive Gameplay Diagnosis)
scored 52.5% against Oxidation on prepared openings. Those controls and anchors
differ. A numeric subtraction is not a measured Elo gain.

Use the CCRL Blitz list computed 2026-11-14, archived alongside this file, from
https://computerchess.org.uk/404/rating_list_all.html . Use single-CPU rows only.
The candidate list and exact builds are recorded separately. The cloud-specific
x86-64 requirement in the task supersedes its earlier generic ARM64 wording.

## Conditions and eligibility

Both engines receive 60 seconds plus 0.6 seconds per move. Send UCI `wtime`,
`btime`, `winc` and `binc`; no depth, nodes or fixed move time. One engine thread,
64 MB hash where the rated release supports it, ponder and own book off. Record
any fixed-memory engine before selecting it; do not change its rated source.
No external tablebases. Each game gets one `c3-standard-4` VM in `us-central1`.
Request eight workers, never more. Record actual allocation and CPU model.

The native runner owns legality, history, automatic claim handling and outcomes.
Use `auto_claim`, consistent with accepted historical evidence. Each result must
pass native replay. Retain technical failures, capped games, truncated streams,
cleanup failures and any harness-caused time loss as unscored. Genuine native
clock losses can be scored after checking the timing path. Do not infer that a
replay success alone proves timing fairness. Inspect effective `go` commands,
engine identity/options and process health. Keep all attempts.

Use the CC0 Stockfish `8moves_v3` book at source
`a804a97e709f40aa75a00f9d19e84915b7a3a45c`. Its archive SHA-256 is
`7e1e9dd118b4bb97d8a8b5b8a790c86e21f8509d59a27d2883767d94477be02e`.
Reuse the MSH-139 parser: 34,699 distinct valid 16-ply roots, one terminal root
excluded before play. Shuffle without replacement with seed
`msh185-anchor-2026-11-16-v1`; freeze the first 200 roots in `openings.json`.
Consume these in order. Each pair has one root and reversed colors, with both
games run as separate jobs. The book is generic, not certified balanced.

## Joint model and uncertainty

`tools/strength/anchor_stats.py` defines the exact formulas. Fit one logistic
expected-score Elo parameter with fixed opponent ratings. Draws give half a
point. Regularize with Normal(2500, 500 squared); repeat at SD 250 and 1000 for
sensitivity. Include every scored game, including all bracketing games and a
scored singleton whose partner failed. Keep pair identity for the variance.

Use a cluster sandwich variance and a conservative tabulated Student-t critical
value by number of opening pairs minus one. Before 30 clusters, or with no
cluster variation, report the envelope with a pair-level profile quasi-likelihood
interval. This is an approximate pair-cluster 95% interval. It is not an exact
Bayesian credible interval or an anytime-valid confidence sequence. The broad
prior permits estimates outside 2000–3000; that range is an initial routing guide.

Fit main families separately and report any spread over 150 points. Report
per-release counts and scores as well. Anchor uncertainty, different hardware,
time control and family style are systematic limits outside the statistical
interval. This index is not an official CCRL rating or a human rating.

## Sequential selection and resource stops

Start with four pairs against Stash 20.0.1 (2511), near 2500. Its source uses
both Fischer increments; Mr Bob 0.7.0 (2505) ignores increments in its original
time policy and stays a reserve candidate. This is a pre-play choice, not a
response to a game result. After each
round of eight simultaneous games, refit and choose the eligible candidate
nearest the estimate. Stop bracketing when the interval is about +/-100 or at
one hour of campaign wall time (the task expects about three or four rounds). Do not claim
that four pairs establish precision.

Choose about three independent families within about 150 points. Rotate them
across main batches, normally 24 pairs (eight pairs per family). Refit after each
batch. Replace a release whose running score leaves 25–75% with a nearby release
or family from the frozen pool, and record the reason. Do not automatically repeat
failed games or discard valid results when changing opponents.

Stop for useful precision at a joint half-width of 40 points or less, after at
least 30 pairs and adequate main-family coverage (at least 20 pairs per family).
A 30-point interval is useful but not required. Aim for about 300–400 total games;
never exceed 400 scored games. Stop earlier at a resource cap and report the
available evidence, even if precision or family coverage is incomplete.

The original campaign clock starts at the first cloud match submission, including
smoke work. It never resets between rounds, failures or reruns. Hard limits are
six hours of wall time and US$15 estimated compute. Reserve time for termination
and deletion. Workers enforce the same absolute deadline; the local controller
waits on the existing Pub/Sub subscription and deletes its job on failure or
near the deadline. No automatic retry. A conservative cap check reserves eight
VMs for all remaining campaign time. This maximum is about US$9.68 for six hours
at US$0.201608 per VM-hour; actual VM lifecycle records refine the final cost.
Local build time has no cloud VM cost. Record smoke and failure compute separately
within the same total. At a forecast over 4.5 hours or US$10, inspect and record
the cause. If necessary, stop at about 250–300 games to stay within the limit.

## Verification and review

Measurement only. Focused offline tests check estimator edge cases and worker
identity, score mapping, replay and resource bounds. Independently review the
method and glue before play. Review the first cloud batch before more games.
The initial eight-game bracket also provides the end-to-end cloud check; retain
its valid scored games in the fit. Stop on a technical failure to inspect it.
Review final results and resource cleanup independently before integration.
No production engine, GUI or multi-platform acceptance claim is made here.
