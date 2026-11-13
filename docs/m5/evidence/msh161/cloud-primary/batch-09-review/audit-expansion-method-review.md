# MSH-161 (Competitive Gameplay Diagnosis): independent audit-expansion method review

Passed before launch. This review binds the exact current `method-amendment-v1.1.json`, `audit-expansion-selection.json`, proposal and original consequential selection by SHA256 in the JSON record. The corrected original-method path `../../main-method.md` resolves and its hash passes. The coordinator refreshed the proposal hash before this review; all internal bindings now match.

The direct 20M substitution is sound and bounded. Reuse the 19 existing matched 5M roots in main-09-00 game0, reference Black/endgame. Add only two roots: ply57 with forced `a1c1` and `g7f8`, and ply75 with same-choice `c8c6`. Three new singleton queries complete 21/21 cell roots at at least 5M without repeating existing work or adding a separate 5M stage. Histories, options, engine identities and initial comparisons remain unchanged. Frozen v1 stays immutable; v1.1 records the expansion explicitly.

The ply65 `h7h5` versus `c3b3` trigger remains supported at 5M by point loss .067 and conservative margin .0635. Both matched 20M moves remain in `selection-00-04.json`. Retain the original trigger even if 20M withdraws support. High remaining mover value does not waive the expansion; it does limit any lost-game or conversion interpretation.

Coverage has mixed budgets: 19 reused 5M roots and two new 20M roots. Do not describe it as a uniform 5M sample, infer a cell error rate, or certify other untested cells. Retain unstable and conflicting results as unresolved. No new analysis, query, retrieval, code change or scientific scope was introduced by this method review. The coordinator may admit the bound overlay and prepare the merged plan.
