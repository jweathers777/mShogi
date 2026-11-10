# Final changed-decision reference adjudication

The frozen rule selected two development changes for new adjudication. All80 held-out primary choices were unchanged, so no held-out pair was eligible. These are2 of240 primary predecessor/candidate decisions;238 choices were identical. The analysis supports one local positional improvement and a small preference on the other pair. It does **not** establish broad independent decision improvement or stronger match play.

Four Stockfish19 queries completed successfully at common3s, one thread, Hash16MB, cold processes and no tablebases, preserving each parent's complete available history and forcing one root move. Every selected final nonbound event aligned with actual bestmove and had a legally replayed PV. Last-two-distinct-depth scores varied by at most3cp. Different depths and finite search still limit the conclusions.

| Source identifier | Predecessor | Candidate | Finite interpretation |
| --- | --- | --- | --- |
| STS Open Files and Diagonals.037 | Qb4+,0cp, depth34 | Bb1,+117cp, depth24 | Supported local preference for retaining bishop pressure; not a proved lost win or exact centipawn loss. |
| STS Knight Outposts/Centralization/Repositioning.023 | Rh8,+17cp, depth29 | Ng5,+48cp, depth25 | Small finite preference; insufficient to call a consequential decision improvement or condemn Rh8. |

**Open Files:** the predecessor's forcing queen check leads to queen exchange, a rook invasion and kingside pawn captures. White receives active king/rook play and an advanced e-pawn; the reference line remains balanced. Bb1 instead repositions the bishop through e4, keeps pressure while White organizes castling, and supports a later favorable liquidation and recovery of the e-pawn. The candidate's published preference is thus supported by an independently examined continuation, not merely matching an answer key. It remains a finite positional judgment; neither line proves game-theoretic WDL or actual conversion.

**Knight Outposts:** Ng5 repositions through e6/c5 while Black organizes the rooks; Rh8 permits a queenside pawn break and exchanges into a different minor-piece structure. Both analyzed continuations remain plausible with small positive root-side scores. The31cp reporting difference and published10-point preference do not establish that the predecessor move was materially inferior. Keep this result as a modest finite preference with unresolved practical significance.

The candidate was frozen before these queries and before held-out evaluation. No tuning or production change follows from this adjudication. The broad decision-quality gate remains unmet: local evidence on two development examples cannot replace improvement on diverse untouched validation cases. The exact native cost correction may be retained on its verified execution merits without claiming a demonstrated general playing-strength gain.

## Receipts and source handling

Selection: `/tmp/msh157-final-review/selection.json`; frozen input and both comparison-ledger hashes: `/tmp/msh157-reference-final-candidates.json`; raw ledger: `/tmp/msh157-reference-final/runs.jsonl`. Structured sanitized findings and verification are `/tmp/msh157-final-reference-findings.json` and `/tmp/msh157-final-reference-receipt.json`.

The four queries used **12.003445707 active seconds**,13.593692 measured worker CPU seconds and13.818763875 process-wall seconds. Combined initial/final reference charge is60.009475457 seconds, below the existing160-second reservation. All four succeeded; no further reference search is planned. The global shared lock/reservation and pending-attempt crash protection were retained.

This summary omits source FENs and full upstream fixture rows. Detailed local raw data stays outside Git where upstream redistribution permission is unresolved; committed evidence should retain identifiers, hashes and reproducible acquisition instead of those raw rows. Short independently generated continuation findings do not change or relicense the underlying corpus.
