# Balance-screen selection handoff

Source-selection author, 2026-11-11. This note is an author handoff, **not an
independent review**. No competitive game results or previous engineering
explanations were consulted, and no engine was run for selection.

`select_balanced.py` applies strict pilot-v0 acceptance to all330 source roots:
successful completed query, settled process group, untruncated transcript, legal
PV aligned to bestmove, no score bound or assessment uncertainty, absolute White
centipawn score at most60 and White WDL expected-score proxy between.40 and.60.
These thresholds describe suitability for a paired prepared-root survey, not a
proof of equality. Stockfish's expected-score proxy is not calibrated to the
competitive opponent. Each receipt hash, score/perspective, status, uncertainty,
family/rank and structural tags is preserved in `selection.json`.

The result is **70 accepted,244 uncertain,16 outside the declared balance window**.
Uncertain is not a synonym for poor or unbalanced:242 carry
`last_score_update_not_selected_exact_score`, and2 lack an aligned nonbound score.
Several families contain fewer than four strict-accepted roots. Consequently the
main90-root sample is **unfinished and not frozen**. A reviewed method amendment
may distinguish usable earlier exact iterations from later bound updates; the
original strict ledger must survive any amendment. No flag was cleared here.

## Qualification panel

`qualification-roots.json` contains four accepted complete histories for the
separate opponent-qualification experiment, with provenance/receipts in
`qualification-selection.json`. Each is the earliest accepted source-ranked root
matching a predeclared structural contrast; there is no score-size or competitive
result optimization within a contrast.

| Contrast | Root | White cp | White expected-score proxy |
|---|---|---:|---:|
| Symmetric open Spanish/Berlin | master-65b720cc9c80e96e |5|.5025|
| Closed French/Winawer | master-524a0e2ff6429b69 |26|.5170|
| Opposite-castled Semi-Slav | master-3a47a461d9dc2253 |−27|.4770|
| Asymmetric open Sicilian/Najdorf | master-5fec4590ee6aebc1 |27|.5185|

Panel SHA-256:
`aade3d136fa9fdb317e0ed85b19feb06378466d0071d4383bdc2d065d9c7e4f6`.
The engine conditions are the unchanged screen's Stockfish19/500000nodes,
one thread/64MB hash; each exact receipt retains options and binary identity.
All four have no assessment uncertainty and independently replay16 legal plies
from standard start to valid nonterminal positions. Native admission is still
the authoritative pregame check. This panel is not the main study and supplies
no inference about opponent suitability until its actual games are reviewed.

## Conditional confirmation shortlist

`confirmation-shortlist.json` identifies37 unresolved roots that could complete
four roots per family while maintaining the original per-family structural
quotas. It minimizes unresolved selections first, then chooses the
lexicographically earliest source ranks among feasible four-root combinations.
All sixteen outside-window roots are excluded from this provisional calculation;
all uncertain roots remain eligible for investigation without asserting balance.
There is no structural infeasibility at this provisional stage.

This is a **proposal pending method review, not an automatic5M-node run request**.
If the exact-final-score interpretation is revised, recalculate selection under
a separately recorded method and preserve the strict results. If confirmations
are used, their outcomes may require further same-family/structure replacements;
37 is not a promise that the final sample will be complete. Two gap-fill roots
are deliberately unselected until the acceptance method and coverage are settled.

## Verification performed

`python3 sources/select_balanced.py` completed without engines. It checks330
one-to-one histories, uniform reference identity/options/work, white perspective,
and WDL arithmetic against the stored score. Python-chess1.11.2 independently
replayed the four qualification histories. Original source artifacts, selection
ordering and earlier source-review evidence were not modified. This is local
analysis verification; no new production, cross-platform or GUI result is claimed.
