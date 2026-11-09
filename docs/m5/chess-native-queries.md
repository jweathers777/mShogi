# MSH-150 (Lazy Native Move Queries)

The native existence query previously generated every candidate into a vector
before stopping checked legality at its first witness. It now stops candidate
generation itself. One private short-circuit visitor serves the existing public
candidate lists, filtered legal queries and existence query. Piece, ray, promotion,
castling and en-passant order are preserved. Counter normalization, the checked
native resolver, history, claims and public contracts are unchanged. Full legal
queries collect directly; no blanket allocation-count saving is claimed, since
the former iterator could reuse its input vector.

Corrected source `0401b828c472f32759afba1a407093cb99313d5a` passed 697 local
core/search/console tests, with four pre-existing ignored tests. The separate
ignored deep-perft test also passed: independent initial, Kiwipete and
promotion/castling depth-four divides total 4,705,217 leaves, checked by both
divide and perft. Workspace all-target clippy and formatting passed. Visitor
termination is checked inside rays, promotion groups, queen directions and
ordinary/castling/en-passant boundaries; pre-change order snapshots supplement
independently established legal sets and full-public-transition comparisons.

In 36 alternated paired runs over 12 positions, completed results, PVs, node work
and **every search counter are identical**. The sum of per-position median times
falls from 4,447.261 to 3,901.971 milliseconds: **12.26% less time** at fixed work.
Eleven medians improve; the remaining rook/pawn tactical case is 0.7% slower,
within ordinary timing noise. This small local desktop sample establishes an
execution-cost benefit, not a rating gain. All 16 paired timed choices remain
unchanged; two one-second samples complete an extra ply. Six independent endgame
choices preserve WDL. Total comparison execution was 94.65 seconds.

The [canonical evidence](evidence/msh150/verification.json) preserves sources,
binary/input receipts, commands, independent source review and measured limits.
Independent final measurement reconciliation approved every receipt and reported limitation. Level 1 broadened local native
verification applies; no other operating system or packaged GUI check is claimed.
No opponent matches ran. Continue MSH-145 (Pawn Threat and Exchange Judgment),
then MSH-149 (Classical King Exposure), before the coherent-round reassessment.
