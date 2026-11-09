# Classical Tactical Forecasting — MSH-152

The investigation found a concrete selective-search failure and a separate
promotion-exchange defect. It did **not** establish that every optimistic forecast
is a wrong move, nor that these corrections raise overall playing strength.
The complete receipts, source identities and local checks are in
[evidence](evidence/msh152/verification.json).

## What the historical forecasts actually show

Completed depth, score and PV belong to the last completed iterative-deepening
iteration. Visit counts and counters include subsequent interrupted work.
Scores are from the root side; a PV is a searched continuation, not a promise of
best play. The diagnostic harness replays complete histories, validates roots,
natively admits every PV and verifies restoration. `Exact` mode uses static
leaves rather than the conventional quiescent frontier, so it is not a clean
pruning-only comparison. A default-off null-pruning control closes that gap in
otherwise comparable conventional experiments.

| Position | Controlled finding | Interpretation |
| --- | --- | --- |
| MSH-151 Goldfish, White 27.e4 | Original depth 8 and roots after 1/2/5 forecast plies at remaining depths 7/6/3 all back up White +17, with the same remaining line. Extending the postexchange root to depth 8 changes this to White −83. | No sign/backup inconsistency on this tested line. Farther consequences matter. Fresh-root searches reset ordering/table/extension context, so this agreement is evidence for this case, not general equivalence. |
| MSH-151 Oxidation, Black 31...Ke8 | At completed depth 6, ordinary search chooses Ke8. Quiet-omission-off and null-off retain it; LMR-off chooses Rxf5. The trace shows Rb8 at the Ke8 child, a PV node at depth 5, reduced by 1: score 123 fails below alpha 125 and receives no full-depth verification. Qa8 also receives a reduced fail-low. | A concrete selective-admission failure: important quiet alternatives receive inadequate investigation. Different windows/order in the no-LMR run preclude calling the score difference an exact minimax error. |
| MSH-144 White 44.Ne5 root | Current search chooses Kf5 at both 1 s/depth 8 and 5 s/depth 9; score −68 becomes −88. | The historical exchange is no longer selected. This materially gated ending does not implicate ordinary LMR; no new change is justified from its score drift alone. |
| MSH-139 first-loss root | Rf2/+35/depth 8 at 1 s; Rh2/+21/depth 9 at 5 s. | Finite-horizon ranking instability remains; no independently proven best move or first losing move was established. |

Six bounded three-second Stockfish 19 MultiPV analyses supplement these facts.
The pinned full-strength reference also prefers **e4**, evaluating the position
as already badly lost (about −672 cp); after the forecast exchange it reports about
−744 cp. Thus e4 is not a demonstrated decision error. Its residual disagreement
with the deeper conventional −83 remains a mixture of possible horizon and
classical valuation limitations. After Ke8 the reference strongly prefers Qa8+
over Rb8 and evaluates both far worse for Black than Rxf5 at the original root.
These are reference opinions, not native outcomes or WDL proofs. The embedded
reference network is not part of mShogi's evaluator or ordinary test dependencies.

## Corrections and measured tradeoffs

**PV-sensitive late-move reductions.** Plausible quiet alternatives at a PV node
receive one more ply than a non-PV cut probe. Deep, late PV moves can still be
reduced; ordinary non-PV reductions and all native/check/claim/material guards
remain. This is a general confidence policy, independently implemented with
precedent in the classical [Stockfish 11 search](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/search.cpp),
not a pin detector or a special case for Ke8.

At depth 6 the changed search chooses Rxf5 using 115,107 visits versus 87,818 for
Ke8 before: **31% more work**. Blanket LMR removal needs 216,561 visits, about 2.47×
the original. The targeted policy still completes depth 6 and chooses Rxf5 at 1 s.
It costs one completed ply in the Goldfish root, older MSH-139 root, sacrifice
control f3apJ and STS exchange position, plus independent puzzle TnyRw. Higher
nominal depth is not the decision-quality objective, but this cost remains real.
The trace proves the reduced fail-low path; the practical choice improvement is
reference-supported, not an independently proven change from loss to draw/win.

**Promotion recaptures in exchange estimation.** Least-attacker ordering formerly
broke same-pawn promotion ties alphabetically, selecting Bishop before Queen.
In the independently accounted Rxa1 example, this returns +170 instead of −400
and admits a losing capture at the quiescent frontier. The correction evaluates
all native legal promotion completions of the selected attacker and takes its
best continued material return, including declining the exchange. Ordinary
least-attacker selection remains unchanged. Tests cover both colors, recaptured
promotions, two promoting origins, cancellation and restoration. A fixed
back-rank capture target permits at most two promoting pawn origins, bounding
this exceptional branch expansion to 16 variant paths under the existing
16-capture limit. This defect is real but is not asserted to have caused the
historical losses.

## Decision-quality checks and scope

The existing independent 20-position panel retains all 12 published tactical
preferred moves or independently replayed immediate-mate alternatives, and all 8
complete-tablebase WDL choices. The published quiet Rd5 mating attack remains
selected at depths 3/4/5. Native replay proves the supplied line's mate, not that
it exhausts all defences. The STS exchange preference and unique rook winning
move remain selected. Both implementations already passed these controls;
retention is a regression guard, not newly demonstrated improvement.

Focused regressions preserve complete histories, draw-claim semantics, real
non-PV reductions, interior cancellation and exact cursor/evaluator restoration.
The deliberately failing promotion tests are retained alongside corrected runs.
Level 1 local search/native-consumer checks and independent review apply: no
platform-specific code, protocol, persistence or desktop boundary changed.
Linux/Windows/full macOS CI and packaged GUI verification were not run for this
increment; broader compatibility remains for the next integration/release
checkpoint. No matches or rating study ran.

The investigation stopped after controlled comparisons localized a useful
selectivity correction, matched-horizon probes disconfirmed a backup defect,
and independent controls bounded the changes. It did not expand into an
exhaustive parameter sweep or add an unsubstantiated positional bonus.
[MSH-153 (Classical Compensation Calibration)](../../backlog/tasks/msh-153%20-%20Classical-Compensation-Calibration.md)
tracks the remaining horizon/compensation question across independent controls.
A formal strength comparison is premature on these limited decision observations.
After further meaningful progress, propose matched earlier/current versions with
shared openings, conditions and opponents where appropriate, and a sample suited
to the effect of interest, for owner approval. No automatic small absolute-rating
study, arbitrary strength target, learned evaluation or cross-game transfer follows.

Accepted source: `6c903e43594c9ed859a6af43618b3c28487d5805`, integrated on
`codex/rewrite`. The local search/engine/console checks passed **385 tests**, with
2 preexisting ignored tests; the search library contributes 113 of those passes.
Workspace all-target Clippy and formatting passed. Independent final review
approved all three acceptance criteria and reconciled the source and receipts.
