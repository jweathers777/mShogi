# MSH-66 independent completeness-fixture expected-fact review

Date: 2026-10-26. Reviewer: fresh Codex review agent, separate from the fixture
and native legality authors. This is an independent agent review, not a human
review. Only this report and the separately requested reference-domain review
supplement are owned by this reviewer.

**Verdict: PASS for the 68 independently expected records in
`tests/fixtures/shogi/legal-completeness.json`.** Approval was sent to the author
before reference or native execution of these records. No expected answer was
derived from either implementation's output. This approves concrete expected
board facts; actual execution, native legality and MSH-66 acceptance remain
separate evidence.

## Provenance and method

Read the active MSH-66 criteria and selected `mshogi.shogi.csa36.v1` contract,
the prior inventory/coverage and reference review, and every new fixture record.
The rule sources remain the primary JSA movement, promotion, check/mate and
prohibited-play material whose retrieved bytes are fingerprinted by MSH-64.
The source interpretations and independent reasoning are continuous with
`reference-review.md`; the late old-root admission correction is recorded
separately in `reference-domain-review.md`.

Before opening the new candidate, independently derived the central/edge move
counts, promotion additions, auxiliary king moves and 30-by-30 start-tree proof.
Then compared every exact array against independently handwritten target maps;
a separate review script expanded those maps into USI choices and their
180-degree ownership-reversed counterparts. This script did not import cshogi,
call the native library or read an implementation's move output. The six
blocker records and four empty-set records were checked by explicit ray,
occupancy, attack and escape/capture reasoning.

The first approved 68-record candidate's full SHA-256 was
`1bd1b58375ce289221d45c6a33e4cf549617162a3d16a73f4dc71b23c43c8792`.
Its review fields were still pending at that snapshot. Review metadata and
observations may change the file's whole-byte hash without changing the
approved facts. The canonical JSON projection of each of these 68 cases'
`id`, `version`, `rules_profile`, `root_sfen`, `moves`, and `expected`, preserving
case order and using sorted keys with compact separators, has SHA-256
`0f7a5c3e64e8b8143d98d1decc1e2e5fce83a361bb1ac90f8568865e9ac3435d`.
The two temporary F022 safe-root duplicates are excluded from this projection;
they belong to the separately corrected MSH-64 corpus.
The final 68-case file, after approval metadata and removal of those temporary
duplicates, has SHA-256
`a562a265da4c49ca09702b14c2ca16b1a9eb7db9b779f426d0da3cd782cfc902`;
the canonical expected-fact projection remains identical.

## Exact central/edge sets

There are 56 exact sets: eight base kinds and six promoted kinds, central/edge
placement, and both owners. Sente central pieces start at 5e, with K9h and the
opponent K2a; edge pieces start at 1e, with K9h and opponent K8a. King matrices
replace K9h by the test king. Opponent kings are off the initial attack rays and
too far away to constrain any listed king step. This avoids the unsuitable
default K1a diagonal in a central-bishop diagram.

For every non-king matrix, K9h independently contributes exactly
`9h8g 9h8h 9h8i 9h9g 9h9i`. Rotation transforms those to the five K1b choices.
All other moves must originate at the single test piece. The following totals
include those five auxiliary moves, except in the king rows:

| Test kind | Central complete choices | Edge complete choices |
| --- | ---: | ---: |
| King | 8 | 5 |
| Gold | 11 | 9 |
| Silver | 10 | 8 |
| Knight | 9 | 7 |
| Lance | 11 | 11 |
| Pawn | 6 | 6 |
| Rook | 24 | 24 |
| Bishop | 27 | 16 |
| Promoted pawn/lance/knight/silver, each | 11 | 9 |
| Horse | 25 | 16 |
| Dragon | 25 | 23 |

Gold and promoted minor movement use the six side-relative steps; silver uses
forward plus four diagonals; knight jumps two forward and one sideways; pawn
has one forward choice. Lance reaches four forward ranks, with optional
promotion on ranks c/b and compulsory promotion at a. The edge keeps that
forward ray and clips the other pieces' off-board steps without wrapping.

A central rook has 16 geometric ray targets and three extra promotion choices;
the edge rook still has 16 targets and three extra choices. A central bishop
has 16 diagonal targets and six extra promotion choices; the edge bishop has
eight targets and three extra choices. Horse adds orthogonal adjacent steps
without promotion suffixes: four centrally, three at the edge. Dragon adds
diagonal adjacent steps: four centrally, two at the edge. None of those steps
duplicates a ray move. All arrays, including the exact promotion suffixes and
both directions' clipped edge squares, match these independently listed facts.

## Blockers and empty legal sets

The bishop has a friendly P3c on its 4d–3c ray and an opposing S7g on its
6f–7g–8h–9i ray. It can reach 4d but cannot land on/pass P3c; it can capture
S7g after 6f but cannot pass it. Its other diagonal rays stay open. Only
7c/8b/9a permit promotion among those asserted destinations. These are selected
bishop assertions, not an exact whole-position set: the opposing silver also
attacks the auxiliary king's possible 8h destination. The mirrored assertions
preserve those occupancy and promotion facts.

For lance/friendly-pawn positions, the exact set consists of L5e–5d, P5c–5b
with either promotion flag and the auxiliary king's five moves. For the enemy
silver blocker, it consists of L5e–5d, L5e–5c with either flag and those five
king moves. Nothing crosses the blocker. Both rotated eight-choice sets are
correct, including gote promotion at g/h/i.

SH-F031's checked king K5a has four neighboring squares occupied by its own
L4a/L6a/P4b/P6b. Its remaining destination 5b contains the checking pawn and is
protected by G5c. No other defender captures that pawn, lance jumps are
unavailable, pawn advances leave the check intact, adjacent-pawn check cannot
be interposed against, and there is no hand. The legal set is exactly empty
with check true. Rotation establishes the opposite checked side.

SH-F032's K1a is not itself attacked by G2c/N3c. Its only destinations are
1b/2a/2b: the gold attacks 1b/2b and the knight attacks 2a. There are no other
owned pieces or drops, so its exact legal set is empty with check false. The
rotated K9i counterpart has the same proof. These four records assert board
check/choice facts only; they do not assign a CSA result or a chess stalemate
result.

## Independent shallow perft/divide proof

Both start orientations have the exact independently enumerated 30 actions:
nine pawns, two lances, four silvers, six golds, three king steps and six rook
shifts. The complete USI lists and divide keys were checked against those
groups and their rotation.

No first action captures, promotes, gains a hand, or moves an opponent piece
or blocker. Pawn advances end on f; all other sente actions stay on h behind
the pawn line. Thus gote retains exactly its initial movement opportunities.
The only potentially opened long attack is the bishop ray after 7g–7f:
8h–7g–6f–5e–4d–3c–2b. It misses K5a and all three 4b/5b/6b king destinations;
gote's own pawn/bishop occupancy still bounds that ray. The 9g–9f opening goes
to the board edge and cannot attack gote's king or its candidate destinations.
No first move checks or pins that king or invalidates any initial reply, and
none creates a new gote move.

Consequently every one of the 30 first actions has exactly 30 replies: perft
depth 1 is 30, each depth-2 divide entry is 30, and depth 2 is **900**.
Ownership reversal and board rotation prove the white-to-move version with
the same total. No deeper perft total is invented or approved by this report.

## Subsequent reference corroboration

After sending the independent expected-fact approval, the reviewer executed:

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --fixtures tests/fixtures/shogi/legal-completeness.json \
  --report /private/tmp/msh66-completeness-review-current.json
```

Result: 68 fixtures passed, zero failed, with 13 reference-domain rejection
sentinels and four API-gap sentinels. The output is byte-identical to retained
`docs/m4/reference/completeness-reference-run.json`. This command checks board
assertions; perft/divide expectations are separately checked by the perft tool.

The reviewer also independently enumerated LegalAll at both reviewed start
roots, with a reserved two-ply reference counter budget. For each of their 30
first actions, pushed the enumerated integer, counted the opponent's LegalAll
set, popped and compared the complete reference snapshot. Both yielded exactly
30 first choices, 30 replies per choice and 900 depth-2 nodes, matching every
independently expected divide entry and restoring the root exactly. This is
subsequent corroboration, not the source of the expected 900 or 30 values.

## Remaining evidence boundary

An independent geometric root audit found no already-attacked nonmoving king
among these 68 records. That audit found a separate defect in two historical
MSH-64 bishop-evasion roots; their correction and reference-domain admission
are covered by the separate supplement, preserving historical evidence.

The author was authorized to run the admitted reference only after this
expected-fact review approval. Reference results must stay separate from these
expected arrays and totals. Native complete-action comparison, all-prefix
state transitions, apply/undo restoration, request atomicity, additional
king-safety/pawn-drop edge cases and MSH-66 closure belong to the integrator's
native test/runtime evidence and fresh implementation review. History and
outcome semantics remain MSH-67.
