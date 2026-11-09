# MSH-142 (Classical Endgame Judgment)

This increment supplies material-specific conventional evaluation where generic
material and placement could not distinguish important endgames. It does not
declare Chess mature or establish a rating gain.

## Implemented knowledge

- **KPK:** an original 24 KiB win/draw bitbase, generated offline from native
  legal positions and actions. Its 196,608 normalized states include turn identity;
  165,676 are valid native positions and 111,282 are pawn-side wins. The monotone
  retrograde solver seeds native mate and safe queen/rook promotion, excludes
  capturable promotions and stalemate, and treats unresolved defensive cycles as
  draws. Generation uses no imported code or table data and is not a runtime step.
- **Wrong-bishop rook pawn:** zero scaling requires exactly bishop and one rook
  pawn against a bare king, wrong bishop color, and the defending king already on
  the promotion corner. Other files, extra material and a distant defending king
  do not match. This intentionally narrow guard avoids claiming broader fortress
  recognition.
- **Bishop-and-knight conversion:** original finite scores encourage bringing the
  kings together and driving the bare king toward a bishop-colored corner.
  Hanging pieces, stalemate and draw rights remain search responsibilities.

These techniques follow established material-specific dispatch and conversion
ideas in classical [Stockfish 11 endgames](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/endgame.cpp)
and [KPK generation](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/bitbase.cpp).
No source, coefficients or upstream bitbase were copied.

KPK membership is board-only knowledge, ignoring clocks and repetition. Winning
scores are original heuristic centipawns, not distance-to-mate or native wins;
draw scores are evaluation advice, not adjudication. Search still admits every
actual action and consults native terminal outcomes and claims first. Winning
KPK scores remain below rook material to avoid discouraging safe underpromotion.
Raw en-passant after a lone pawn double push is immaterial only to this lookup:
there is no enemy pawn that could capture. Native saved state and history retain it.

## Verification boundary

Independent Syzygy receipts cover opposition with either side to move, rook-pawn
corners, captured promotions, queen-promotion stalemate versus winning rook
underpromotion, wrong/right bishop and material/king-location counterexamples.
Expectations are retained locally; normal tests need no network or external engine.
Color/file reflection, unchanged input, incremental undo, native claims and
cancellation are checked. Three bounded native search journeys preserve the KPK
win and reach safe promotion, excluding immediate capture and stalemate.

The generator's `--check` mode regenerates the exact bytes and verifies lookup for
every valid state. This self-consistency check supplements the independent
fixtures; it does not replace them. The single structured record is
[verification.json](evidence/msh142/verification.json).

Existing independently established sacrifice, quiet-threat and check-evasion
tests remain. A new rook-defence fixture verifies all three ways to prevent an
immediate queen mate at the initial frontier, including quiet Ra7. Testing it at
the final defensive frontier also revealed a remaining non-contact-mate horizon
weakness: Rg8 permits Qh5/Qh4/Qh3 mate, missed beyond the optional quiet-check
cutoff. The failed case is preserved for MSH-143 (Chess Search Efficiency), not
presented as fixed by this endgame increment.

KRKP remains unresolved: the historical unique ...Re1 win and drawn advanced-pawn
position need more than the narrow material classes implemented here. No exact
rook-versus-pawn recognizer or general pawn-ending solver is claimed. Fresh
adaptive strength reassessment follows the combined endgame and efficiency work.

The final combined source `f9747769c6e0b4f08a2cb893bdaa696b1317faee`
passed all required Linux, macOS 15 and Windows native CI jobs in
[run 42674054950, attempt 2](https://github.com/jweathers777/mShogi/actions/runs/42674054950).
Attempt 1 failed inside GitHub before creating any jobs; that provider error and
the identical-source retry are retained in the structured evidence.
