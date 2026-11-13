# Primary Batches 01–04: behavioral coding

This review reconciles all **240 candidate supported finite events** in the cumulative ledger: 58 in Batch 01, 52 in Batch 02, 61 in Batch 03 and 69 in Batch 04. A further 19 stable threshold-straddling comparisons are separate sensitivity cases. They retain their original dispositions and do not enter the supported-event count.

The structured file contains one entry for each of these 259 roots. It retains the exact final source row, source-file hash, native report hash, full native prefix, root board, played move, tested alternative and legal reference continuations. Every source query hash, native prefix and continuation was checked offline. The supported-event key set exactly equals the cumulative source key set, with no duplicate roots.

Behavior is attributed for 158 supported events. The remaining 82 use explicit unknown coding. Unknown does not withdraw numerical support. It means that this pass does not assign a specific behavioral category from the retained context. The 19 sensitivity-only cases also use unknown coding. No category is inferred solely from phase, capture type or pawn movement.

| Category | Supported moves | Affected games |
|---|---:|---:|
| advantage preservation/conversion | 3 | 1 |
| coordination/activity | 54 | 25 |
| endgame decisions | 41 | 20 |
| exchanges | 20 | 18 |
| king safety | 18 | 13 |
| opening/development | 20 | 10 |
| opportunity response | 30 | 16 |
| pawn structures/breaks | 19 | 16 |
| quiet defense | 65 | 35 |
| tactical calculation | 32 | 21 |
| unknown | 82 | 46 |

Multiple tags overlap. These are descriptive counts within the reviewed selected sample, not all-move error rates or a priority order. No margins are summed. No planning label was forced from an isolated alternative. The narrow conversion tag refers only to the three tested promotion opportunities in one game.

Two repeated motifs have explicit bounded anchors. In main-02-00 game 1, plies 81, 91 and 113 retain immediate promotion as the tested resource. In main-02-05 game 0, plies 46, 50, 52, 54, 60 and 62 retain Bxh7. The records link the selected opponent and candidate roots around these intervals. They do not count adjacent reversals as independent causes or claim that these are complete favorable episodes.

Other event records include nearby selected candidate and opponent anchors. These are navigation and context, not proof of a gift, recovery or causal chain. In particular, a point score alone does not satisfy the frozen two-turn favorable-episode criterion. No first or largest cause is asserted.

Contrary native results are retained explicitly: main-01-08 game 0 ends in bishop-and-knight mate; main-02-05 game 0 promotes two pawns and mates; main-03-01 game 1 finishes its rook-and-bishop ending with mate; main-04-06 game 0 mates after the opposing h-pawn promotes. These outcomes coexist with the coded earlier errors. They do not establish that every later move was correct.

The source secondary tag is retained as recorded. An absent tag in an older schema is not evidence of fresh primary causation. The record also marks low alternative point-score context separately. Native automatic claims and dead-position endings remain authoritative; this coding does not turn reference continuations beyond termination into played events. Claim-limited roots are outside the supported-finite inclusion set.

Earlier, linked and unselected uncertainty remains in the preserved final source rows. The repeated unaided opening roots are not independent population observations. This artifact makes no mechanism claim, no ranking and no engineering recommendation.

For reproducible counts, filter `events` by `inclusion == "supported_finite"`, expand each `categories` list, count roots for moves and distinct `(pair, game)` values for games. Keep the sensitivity-only rows separate. `reviewed_motif_episodes` and `contrary_native_successes` retain their own source and scope limits.
