# MSH-67 mutual-check resolution

Current verdict: **HIST-INTERVAL's mutual-continuous-check obligation is resolved
by a reviewed impossibility argument for the admitted live-root domain.**
The selected `mshogi.shogi.csa36.v1` rules remain unchanged. No mutual-case game
result, tie-break, tournament penalty or new reachability requirement is added.

The [original incomplete review](mutual-check-proof-review.md) is preserved
unchanged, including its failed proof repairs and precise missing implication.
The owner subsequently supplied the [proposed supplement](mutual-check-supplement/README.md).
The archive's SHA-256 is
`a96bc888fec6c3a0da9c375cd398881d64454c67fd965f82c372acdc6eee33f8`;
its seven source/result files match the retained `SHA256SUMS`. The supplied
review claims were treated as provenance, not proof authority.

Two fresh independent AI sessions reviewed the actual argument:

- [Complete-chain review](mutual-check-chain-review.md): reconstructed fixed
  kings, a single active checker, gaze classification, discovered-check
  necessity, stock reduction, all geometric cases and the mixed-radius lemma.
  Nongazing counts and angle multisets replace any assumption that identical
  physical piece labels return individually. Captures, drops, promotions and
  changing checking sources are included.
- [Geometry and supplement review](mutual-check-geometry-review.md): checked
  both programs' necessary constraints, all revised cases, every mixed-radius
  implication and the side-exchanged orientation. It independently justified
  simultaneous source inventory: while a bishop blocks a source ray, its own
  direct-checking pieces cannot replenish that outer ray. A source needed at
  the bishop's next departure must already have existed at arrival.

The new mixed-radius exclusion uses the forced capture of a knight, restoration
of the knight hand count, the unique checking knight-drop square, and a strictly
decreasing count of owned rook base pieces on a ray that cannot be replenished.
This permits arbitrary intervening direct checks, source ownership changes and
promotions. It does not track individually labeled rooks.

`python3 docs/m4/mutual-check-supplement/verify_geometry.py` passes: **23
horizontally normalized pairings across 20 offsets, 46 with reflections**. Their
disposition is five half-plane contradictions, two mixed-radius exclusions,
two simultaneous-three-rook contradictions and fourteen locked identical-endpoint
cases. The additional graph audit finds 84 cyclic fixed-king/side/radius graphs,
each with one two-vertex simple cycle, excluding a longer walk assembled from
different two-cycles. This is a finite calculation of necessary geometry after
the preceding reductions, **not a search of all legal shogi states**.

The theorem applies to reduced-material roots and unknown prehistory as well
as standard roots. It needs standard piece-stock bounds, exactly one king per
side, legal complete actions and an unattacked nonmoving king; it does not need
reachability from the initial placement. `GameRoot` admission, checked `Game`
transitions and authoritative record replay enforce those assumptions. Broad
structural `Position` analysis is preserved outside that live-game admission.

Consequently both continuous-check flags cannot be true in an admitted history.
If a cache defect or corruption nevertheless computes that condition, native
code returns `GameError::MutualCheckInvariant` with complete candidate context,
SFEN, occurrences and interval observations. It commits no candidate and assigns
no score or game outcome. Prefix and canonical replay retain internal diagnostics
instead of collapsing them into ordinary malformed-input errors. Private unit
fault tests inject cache corruption; they are not mutual-check examples and
add no test instrumentation to normal builds.

This is a reasoned, independently reviewed AI mathematical argument, not formal
proof-assistant certification or human expert review. Implementation verification
and task integration are separate evidence, recorded in
[history validation](history-validation.md) and MSH-67. The earlier incomplete
review remains a historical result rather than the current gate verdict.
