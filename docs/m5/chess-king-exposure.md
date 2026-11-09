# MSH-149 (Classical King Exposure)

The previous evaluator multiplied all king-safety pressure by the global material
phase and supplied no endgame danger component. With one queen per side only
28.125% survived; with one rook per side, 15.625%. These are source-arithmetic
facts, not proof that a particular historical move lost the game. In addition,
the cap of two checking origins could mask a newly protected contact check:
the independent Qg7 mate and Kxg7 counterexample both had the same old middle term.

The correction preserves middle-game shelter, storms, pressure, phase and king
placement. A modest endgame penalty now uses existing ring pressure, safe-check
geometry and supported adjacent checks, counted before the old checking-origin
cap. An enemy rook or queen must actually participate in ring pressure or a safe
checking origin; mere presence is insufficient. Each king's added end component
is capped at 128 centipawns. Original coefficients are an engineering starting
judgment, not fitted weights or a proved optimal value. No extra legal-move
query, frontier extension, history cache or adjudication path is introduced.

Independent legal enumeration distinguishes protected queen/rook checks from
king captures, a rook defending the queen's checking square, blocked major
pieces, active minor-ending kings and a queen exchange into a native bare-king
draw. Rh7 is protected in its support fixture but is not mate; Ra8 is mate.
Defending g7 does not erase other queen mates. These distinctions are retained
in the tests instead of treating a geometric danger estimate as a game result.
Existing KQK/KRK approach, KBN conversion, native mate/claims, cancellation,
quiet-threat, sacrifice, check-evasion and defensive-resource regressions remain.

The [canonical evidence](evidence/msh149/verification.json) records the frozen
baseline, independent controls, actual local verification and comparisons.
Level 1 applies to this portable evaluation change. No opponent match, full
platform or packaged-GUI campaign belongs to this increment. MSH-151 (Chess
Coherent-Round Assessment) evaluates the combined work with fresh paired openings
and independent opponents; these component changes alone establish no rating gain.

Tested source `639b6465be384330c9fd7d49580c13bb0c314bf2` passes 103 release
search tests (two pre-existing ignored), workspace all-target clippy and formatting.
All 28 diagnostic conditions preserve native root/PV integrity, all 16 tablebase
WDL choices, three published tactical moves, the two added immediate mates and
the defensive rook choice. Candidate execution took 29.63 seconds. The original
central-king test omitted a pawn placement contribution; its failed receipt and
corrected expectation are preserved, with no product change to satisfy it.

Supported versus capturable queen contact now yields end-danger −51 versus −27,
while both middle terms remain −109. The rook counterpart yields −46 versus −14.
The historical queen/rook root changes at five seconds from Nxa5/+23/depth9 to
Ra7/−32/depth10; there is no independent proof that this move is better. Drawn
rook endings remain overoptimistic (+159 and +311 in two controls), so neither
complete endgame understanding nor stronger match play is established here.

Independent final source and evidence reconciliation approved this increment;
no material findings remain. MSH-151 now measures the combined practical result.
