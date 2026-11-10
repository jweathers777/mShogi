# MSH-156 (Independent Chess Benchmark): independent grader review

Verdict: **approved**, after the one identified edge case was corrected and retested.

Reviewer: benchmark data custodian, independent of grader author. Scope: `/tmp/msh156-grade.py`, its synthetic tests and development-only schema receipt. No heldout fixture or result was accessed for this review. No engine searches or heavy validation performed.

Reviewed grader SHA256 `02c3cc20fb8292d353b931711e635f86a340122b3307aba037a47c2e876b2e9e`; tests SHA256 `46f4a457ede10dc0232f08bf5b275bf621e408a3788d720b0b24100a96c557a3`.

The grader preserves the frozen distinctions: exact tablebase move values compare WDL without pretending DTZ is a strength score; every complete-map omission is invalid. Exhaustive immediate-mate alternatives pass, other legal moves miss that immediate objective without being called lost wins. Published STS points remain a separate preference tier. Unlisted STS/puzzle/evaluation alternatives remain unresolved; moving from an unresolved move to a listed preference is not automatically labeled genuine improvement. Changes between listed acceptable alternatives are retained as such. Missing board moves and failed runs require separate review rather than being scored as losses. Exact and preference tiers remain separate in summaries.

Finding resolved: comparison originally returned unchanged-decision for an identical invalid exact-map move before inspecting the invalid-map judgment. The author moved the invalid-map check before identical-move handling and added same-invalid WDL and mate-one assertions. Re-ran `python3 /tmp/msh156-grade-tests.py` successfully and separately confirmed the invalid same-move result is invalid-exact-comparison. Other fixture-semantic cases passed.

Boundary: the grader consumes already validated fixtures and admitted legal native results. It does not itself establish completeness of tablebase maps, prove mates, adjudicate unlisted positional continuations, or establish that predecessor/candidate manifests use equal times/source conditions. Those guarantees remain with corpus validation, native driver assertions and coordinator manifest review. This separation is explicit and sufficient for the bounded current use. No further correction requested.
