# MSH-161 opening-source research

These are **candidate source records, not a balance-cleared playing sample**.
No engine ran in source preparation. The complete-game study must freeze its
chosen roots and Stockfish balance-screen results before the main sample.

## Sources and permission scope

Research/access date: 2026-11-10.

- [Lichess opening names](https://github.com/lichess-org/chess-openings), revision
  recorded in `manifest.json`, supplies recognized ECO/name/PGN facts. Its
  [README](https://github.com/lichess-org/chess-openings/blob/master/README.md)
  explicitly describes well-known or master-common sequences and dedicates the
  dataset to CC0. It is an opening-name source, not proof that a supplied root is
  balanced. Exact downloaded TSV hashes, revision and CC0 notice hash are retained.
- [PGN Mentor public download page](https://www.pgnmentor.com/files.html) explicitly
  offers its PGN files for free download. Six player archives (Caruana, Karpov,
  Judit Polgar, Kramnik, Anand, Carlsen) supply actual master-game move facts,
  players, event, date and reported ratings. The source page does **not** state
  a CC0 or other open-data license: do not label it CC0 or apply Lichess's license
  to it. Source archive URLs and exact hashes support reproduction. Raw whole
  archives remain outside Git; selected factual moves and identification fields
  are retained here without annotations, commentary, artwork or page text. This
  is a limited factual research extract, not redistribution of the source
  collection or a claim to license its compilation. Source attribution must
  accompany the study's derived records.
- [Lichess API tips](https://lichess.org/page/api-tips) require serial requests and
  a full-minute pause after HTTP 429. A single anonymous master-explorer query
  on this date returned HTTP 401. Headers/body hashes are preserved; no private
  token was read and no authentication bypass was attempted. Explorer access is
  unavailable in this source preparation, so master-frequency claims must be
  restricted to the explicitly acquired sample. The explorer's AGPL software
  license is not a license for its master-game database.
- The existing [Stockfish books](https://github.com/official-stockfish/books)
  checkout at `a804a97e709f40aa75a00f9d19e84915b7a3a45c` is CC0, and its
  `8moves_v3.pgn.zip` SHA-256 is
  `7e1e9dd118b4bb97d8a8b5b8a790c86e21f8509d59a27d2883767d94477be02e`.
  Inspected PGNs identify both players as Stockfish and omit source master
  identities. This material is **not accepted as demonstrated master-game
  provenance** for the new study merely because earlier experiments used it.

## Selection and limitations

`select_openings.py` produces deterministic candidate ordering without engine
judgments or mShogi game outcomes. It accepts standard-start legal games with at
least 24 plies and both reported ratings at least 2400, then retains the complete
first 16 plies as the opening. Board, side, rights and legal en-passant identity
remove root duplicates. Full selected source-game move sequences and headers are
retained as provenance. Source-game result is never a selection filter.

The latest recognized opening board reached by ply16 supplies ECO/name, permitting
transpositions. The source game's own ECO field remains provenance only; later
moves cannot mislabel the supplied root. Aliases resolve in fixed file/row order.
The family mapping is deliberately broad. Classification tags describe actual
pawn closure, mirror symmetry, castling, development, captures and checks; they
are not evaluations or proof that play is tactical/quiet.

Revision2 addresses independent source review before any main-study freeze.
The original reserve, selector, manifest and validation are preserved as `v1-*`;
its compressed candidate SHA-256 is
`9ce186924ee8326a9b8aea97b15e5683b26a996b6efda37a5c82ea2ecbebf62d`.
No engine or competitive output informed either source selection.

The revised selector preassigns structural minima of12 symmetric,10 opposite-wing
castled,15 closed-center and15 open-center roots across families. Each requirement
is allocated once per family in the listed family order until its minimum is met,
only where source records of that kind exist. Every family thus has at most four
required kinds. Within a family, choose the root satisfying the most unmet kinds,
then the most unseen symmetry/center/castling descriptors, then SHA ordering.
This satisfies all assigned kinds within its first four roots. Up to15 candidates
per family provide reserves for independent balance screening. This deliberately
oversamples structural diversity; it is not frequency-weighted master sampling.

The first four candidates per family total88: **18 exact mirror-symmetric pawn
structures,10 opposite-wing castled positions,17 closed centers,28 open centers,
and43 mixed centers**. Both kings are already castled in30 and neither in21.
The remaining two main-study roots and balance-screen replacements must preserve
these minima. Available source counts and family quotas are in `manifest.json`.
Sicilian B60–B99's coarse bucket now explicitly includes Richter-Rauzer rather
than misleadingly naming only Najdorf/Dragon/Scheveningen.

`center_regime` is an operational board descriptor: closed means both d- and
e-file white pawns are directly blocked by black pawns; open means no such
central lock and at most two pawns total on d/e files; mixed is everything else.
The more general `central_locked_white_pawns` field also records c/f locks.
Neither merely lacking locks nor an opening's name establishes an open center.
Captures/checks, development and king placements are separate descriptive proxies;
no tag alone proves that a position demands tactical or quiet play. The study's
coverage review must examine those distinctions before accepting the main sample.

A possible90-root allocation remains four per family plus two declared structural
gap-fill roots. The parent protocol must freeze actual allocation, structural
minimums and reject/replace rules. Symmetry and closed/open/opposite-castling
coverage must be checked again after balance screening, replacing only within the
predeclared structural/family rules. Source-game legality does not waive screening.

The six elite repertoires are purposive and correlated. Duplicates across player
archives inflate `archive_occurrences`; this field is an archive-occurrence count,
**not independent game frequency or opening popularity**. Metadata/ratings may
contain transcription errors and the sample includes different eras/time controls.
Records with absent ratings are excluded, biasing older coverage. A legal
master-game opening is not necessarily sound. Independent root assessment must
record supplied advantage and exclude demonstrably unsuitable roots before play.
Equal color pairing does not erase a large prepared disadvantage.

## Reproduction

Raw downloads live at `/private/tmp/msh161-sources`; `manifest.json` binds bytes.
Download each listed archive URL serially and the five Lichess TSVs, README and
COPYING at the recorded revision. Run with python-chess:

```sh
python select_openings.py /path/to/downloads /path/to/output
```

The manifest additionally expects `source-ref.json`, the GitHub commit API
response, and records available access receipt/page hashes. Production native
replay remains the authoritative game admission check; Python parsing here is
independent source validation only. Neither method nor source scripts change
production rules, evaluation or search.
