# MSH-158 (Chess Decision Corrections): actual cached-threat trace review

Read-only review of threat-trace5 and threat-trace15 native outputs, compared with previously reviewed queen-candidate-trace5. No new engine calls, builds, or held-out reads.

Verdict: actual interior evidence supports the intended correction: a null-mate-derived extra ply is preserved at a compatible PV re-search rather than silently removed. Decision-quality retention still depends on independent continuation evidence and wider controls.

Both isolated candidate traces show this sequence at iteration6:

- Node60936 enters f5c8 with depth5, credits4, nonPV window[161,162]. No previous matching TT entry exists.
- Its explicitly synthetic null child60937 enters depth1 and returns999996, a mate-band selective bound. The real node returns162/lower with c3b5 first in its PV.
- Later PV node103005 enters the same real f5c8 FEN/path, depth5/credits4 and window[-38,282]. Its TT probe identifies same_context=true and stored depth5/credits4/lower162. It emits cached=true extension=1.
- The b2b3 branch is searched at child depth5 with credits3, rather than nominal depth4/credits4. Its narrow child180479 and full-window child196708 both enter depth5/credits3. Parent score174 returns and node103005 finishes174/exact-window.

The5s trace contains448 stored events with dropped0, selects f5c8 at completed depth8 and score−122. The15s trace contains609 events with dropped0, selects f5c8 at completed depth10 and score−69. Both reported PVs include f5c8,b2b3,d8d2,b1c1,c8a6: the important Qc1 reply is represented in the candidate continuation, followed by invasion. This alone does not establish that every consequence was valued correctly.

The earlier near-alpha-verification experiment exhibited a different trajectory: node738092 at f5c8 depth6/four credits searched b3 at depth6/three credits; subsequent PV964255 searched b3 at depth5/four credits. Its b3 branch scores were179 and181 respectively, while the first node's overall180 came from b1a1. Candidate evidence is an analogous iteration6 preservation event, not an identical-node replay or clean causal score delta. In candidate narrow node60936, the selected trace does not show b3 searched before its cutoff, so no paired narrow/full b3 score claim is justified.

Trace filters and zero drops establish completeness only within the selected paths/iterations. Node IDs denote invocation, while child-site events can carry the pushed child FEN/path; child entry events above independently confirm depth and credits. Null scores remain hypothetical and are never native outcomes. Tracing has execution cost; use trace-off decision runs for primary comparison, and do not claim a formal strength improvement from these two traces.

## Supplemental baseline witness

A later read-only check found the same mechanism directly in traced baseline /tmp/msh158/runs/trace5/cell-0002.native.jsonl, not only the near-alpha experiment. At iteration6 the baseline shares candidate prefix nodes60936/60937/103005 and their entry windows. Synthetic60937 returns999996. PV103005 reads the same depth5/credits4 Lower162 entry, but baseline b3 is searched at child depth4/credits4 (entries144702 and148180), returning overall229. Candidate103005 reuses the evidence and searches b3 at depth5/credits3, returning174. Both traces have dropped0. This directly corroborates baseline horizon loss at an earlier iteration, while the later baseline reduced fail-low still explains why its final decision cannot be reduced to a single score substitution. The original depth6 nodes738092/964255 remain specifically from the near-alpha experiment.

Baseline/candidate shared entry details: node60936 is nonPV depth5/credits4 at f5c8, window[161,162]; null60937 is explicitly path[f5c8,null], nonPV depth1/credits4, window[−162,−161]. Node103005 is PV depth5/credits4, window[−38,282], at the identical real FEN; its tt_probe_no_cutoff explicitly records same_context=true, stored(depth5,credits4,Lower,162), hash2137395063331867560. Baseline b3's current parent alpha is159 (beta282), narrow child144702 window[−160,−159] and full child148180[−282,−159]. Candidate b3's parent alpha is50 (beta282), narrow child180479[−51,−50] and full child196708[−282,−50]. Thus child horizon/credit preservation is directly established, while child windows already differ after earlier move searches: neither branch-score delta nor subsequent root decision should be treated as a one-variable identical-window experiment. Baseline iteration7 later enters Qc8 at reduced depth5 despite the root's nominal depth7; that separate final fail-low observation remains valid.
