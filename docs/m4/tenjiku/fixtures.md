# Independent two-profile Tenjiku expectations

The [corpus](fixtures.json) has 350 rows, including 44 owner-exchanged
rotations. There are 204 rules boundary rows, 129 documentary and property
annotations, and 17 consumer obligations. Each row applies to both profiles
with separately tagged expected values.

[Profile data](profile.json) preserves 156 setup placements,36 originals,30
once-only promotion pairs and 44 named movement faces. Fixture coordinates use
[file,rank]1..16; they are development notation, not a frozen public Rust/console API.
The thin adapter must project actual observations, not echo expectations.

## Expectation provenance and reviewed coverage

A's main rule paragraphs, movement legend and chapter 3 diagrams supply the
profile 1 tables. M's explicit differences supply profile 2, with named shared A/S
supplements. Source passages and differences are in [sources.md](sources.md).
Fixed empty-diagram combinations were expanded before implementation;
no arbitrary-position Python rules engine, legacy output or external engine
provided expected answers. Two independent source/rules readers checked the
transcriptions; one independently reconstructed **all 176 geometry sets**
(44 faces×2 profiles×2 owners), not just their counts.

- **Setup:** exact source rows, asymmetry and full 180° opposite setup.
- **Geometry:**44 original/promoted movement faces and 44 rotated counterparts;
  complete route sets distinguish multiple paths and source-specific alternatives.
  Ordinary blockers/captures, ranked overflight/target comparisons, Lion/LionHawk,
  restricted Falcon/Eagle, area stopping and Tetrarch igui have constructed probes.
- **Promotion:**30 pairs with concrete legal accept/decline actions; supplementary
  no-chain obligations, mandatory P/L/I/N cases and actual ordered refusal/expiry/
  capture-exemption examples. B/BGn use diagonal 8f→7e, not forward movement.
- **Burn/royals:** surviving/dying mover, opposing/friendly/multiple Demons,
  direct Demon capture, no death cascade, final-only transit hazard, promotion
  suppression/suicide, ordered victims, final-royal burns and simultaneous loss.
- **History/endings:** concrete first-repeat cycle and rights-different attempted
  repeat; accepted state remains the third prefix after rejection. Actual bare
  loss/equalization/Prince response and royal-winning escape, exposure/surviving
  Prince, and an honestly imported unreachable no-action diagram are explicit.
- **Complete Game sets/diagnostics:** cornerKing+remotePawn diagram has exactly
  four Sente actions, each followed by exactly four Gote actions. Hand-established
  perft 0/1/2=1/4/16 and four divide entries of 4; no promotion/capture/repetition/
  bare condition occurs in two plies. Geometry counts are not this Game LegalAll.
- **Consumer separation:** both-direction profile-mixing, game record rejection
  versus dirty/file guarantees, intentional fresh New versus forbidden in-place
  mutation, actual game choices/history/inspection, resource/saveability,
  search/notation and final16×16 kanji/platform obligations remain 79/44 work.

Reviewed empty-center counts: FireDemon622, ViceGeneral609, Lion88, ModernLionHawk109,
FreeEagle59/75 and Tetrarch29/42 travel choices for profiles 1/2 (plus 8 potential
Tetrarch igui targets, requiring actual adjacent victims). FD=584 area paths+
44 rays−6 identical single-step overlaps; VG=584+29−4; Lion=8 steps+16outer jumps+
64two-step routes. These are explicit local-geometry contracts. Game
LegalAll must additionally enforce profile, turn, rights, history and outcome.

Rotations preserve route order. Re-sort simultaneous burn effects in
canonical order after transformation. The actual asymmetric setup does
not have horizontal mirror symmetry.

## Data admission

`python3 docs/m4/tenjiku/validate.py` checks strict recursive duplicate rejection,
unknown/identity fields, game root IDs/rights references, coordinates, exact
profile applicability, rotations, coverage and reviewed byte hashes. Twenty-five
negative/mutation tests exercise the data boundary:
`python3 -m unittest discover -s docs/m4/tenjiku -p test_validate.py`.
The 958275-byte corpus uses a2MiB **development-data** bound; no rules, transport or archive
constant changes. The audit lock binds raw data bytes and
separate per-profile expected projections. Changing expectations requires review
and deliberate lock update, never automatic regeneration from output.
