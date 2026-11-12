# Frozen Oxidation draw-policy compatibility review

Result: **BLOCKED** for mandatory-threshold-only continuation with the unchanged opponent. Read-only source inspection and the completed failed Linux smoke establish that Oxidation 0.6.0 cannot continue at the optional fifty-move threshold. Source establishes the same terminal behavior at threefold repetition; no additional repetition execution was performed.

Source checkout `/private/tmp/msh139/oxidation060`, revision `46a705b80e7191e7c302838eb8b85053b5994d65`.

- `liberty_chess/src/liberty_chess.rs:331–332,350–351`: empty prior/duplicate hash lists at construction, followed by `update()`, including imported FEN roots.
- `liberty_chess/src/liberty_chess.rs:1192–1210`: after mate/stalemate handling, unconditional `halfmoves >= 100` sets `Gamestate::FiftyMove`. A hash already in `duplicates` sets `Gamestate::Repetition`; first occurrence enters `previous`, second enters `duplicates`, third terminates. Neither branch has a claim-availability or policy parameter. The `skip_checkmate` flag does not disable either draw.
- `liberty_chess/src/liberty_chess.rs:496–503`: pseudo-legal move validation rejects any non-InProgress state, even geometrically legal continuations.
- `oxidation/src/bin.rs:92–108`: an empty legal root move list returns `bestmove 0000`.
- `ulci/src/client.rs:224–270`: full position-history replay constructs the FEN board and rejects a later move from its terminal state, reports an illegal move and exits.
- `oxidation/src/lib.rs:471–484,624–625`: these states also produce draw evaluation and terminate interior search.
- Available runtime options are only `QDepth`, `Hash`, and `Prune` (`oxidation/src/bin.rs:44–63,125–158`). `ulci/src/client.rs:142–208` dispatches only registered options. No existing runtime switch disables optional-claim terminal treatment. Actual smoke handshake independently advertises only those three options.

Actual smoke evidence: `validation-results/report.json` and `receipt.json`. First pairing had Oxidation playing Black at imported halfmove100: immediate `bestmove 0000`, runner invalid_move failure. Second pairing had mShogi playing Black: searched `h8g7` at five seconds, accepted by native authority; Oxidation then rejected that move when replaying the full FEN history and exited successfully before a required bestmove (runner missing_response). Native replay process passed, but the paired smoke failed and does not establish opponent compatibility or an eight-ply successful journey.

This is not a resource timeout and cannot be repaired by an existing opponent option. Clearing halfmove/history would alter the preserved position and search semantics. No opponent patch, replacement, history rewrite, engine invocation, or retry was performed or authorized by this review. Admission of further games under the new policy remains blocked pending a concrete owner-authorized resolution. The earlier 40-game evidence remains historical under its recorded policy.
