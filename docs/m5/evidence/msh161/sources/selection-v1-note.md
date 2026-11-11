# Reviewed finite-screen admission and structural supplement

This author handoff records source selection only, without reading competitive
results. It is not independent review or a claim that any root is proved equal.
The original source/v0 artifacts remain unchanged.

`selection-v1.py` uses the reviewed `screen_v1.describe` interpretation. Admission
requires an aligned nonbound estimate with absolute White cp<=60 and White
expected-score proxy in[.40,.60], the last two distinct-depth aligned estimates
within.025, no directional final-bound conflict, legal retained PV and complete,
untruncated, successfully settled execution. The stale-final-report flag alone
is retained but does not prevent finite balance screening. Other flags block
admission. These are operational finite-analysis conditions, not confidence
intervals or game-theoretic proof.

The original330 receipts yield255 provisionally accepted,48 unresolved and27
outside the balance window. Excluding all four qualification roots irrespective
of their games' outcomes leaves an accepted pool with only8 opposite-castled and14
closed-center positions. Thus a90-root main freeze satisfying10/15 respectively
is impossible from those accepted records alone. `main-roots-provisional.json`
has90 accepted histories across22 families (at least four each),17 symmetric,
8 opposite-castled,14 closed and29 open roots; **it must not launch the main study**.
`selection-v1.json` retains every classification and all original uncertainty.

## Bounded gap follow-up, frozen before answers

`next-balance-roots-v1.json` contains32 standard-start histories:12 unresolved
roots carrying an actual deficient structural kind, followed by20 supplementary
roots from the **same original13,036-root master-source pool**. No new archive or
opening corpus was acquired. The20 consist of12 opposite-castled and8 closed
positions. Within each kind, take one root per family in original family order,
then repeat if needed; within family use original selection-hash order. Exclude
all330 original reserve roots and repeats. No reference or competitive result
enters supplementary selection.

Full source-game facts and histories are preserved in
`supplement-candidates-v1.json.gz`, with hashes and the selection method in
`supplement-manifest-v1.json`. All20 source games and exact16-ply roots independently
replay, and no root duplicates the original330 (`supplement-validation-v1.json`).
The20 were frozen before their reference answers; the32 input SHA-256 is
`162bea12c8f1a0a3a0123cb9a9319e6fab8e9232abf7937027520ee3e006fbed`.
The coordinator owns their fixed5M-node execution and resource records.

## Final-admission command

After that entire bounded screen completes:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/sources/final_admission.py /absolute/path/to/completed-32-root-screen
```

The helper validates complete follow-up identity coverage and unchanged reference
binary/options, requires5M requested work, and preserves earlier receipts while
using the higher-work report for those32 roots. It applies the same finite
admission method. It cannot select using game outcomes or numerical score size.
It selects scarce structural kinds first, original source order within kind,
four/family then two structural gap fills. All four qualification roots remain
excluded, including if otherwise balanced. It requires90 unique roots,
22 families with at least four each,12 symmetric,10 opposite-castled,15 closed
and15 open; criteria are never reduced to force completion.

Successful outputs are `main-roots.json` (runner id/fen/moves) and
`main-selection.json` (all350 dispositions, selected source facts, exact receipt
hashes, superseded evidence, quotas and exclusions). They remain pending
independent admission review before main-study launch. The helper refuses to
overwrite existing final outputs. If structural coverage still fails it writes
`main-admission-unresolved.json` and stops; that is not a freeze or authority to
relax gates. Unknown or failed new reports cannot silently fall back to a more
favorable older estimate.
