# Independent mechanical behavior aggregate review

Pass. Exact input and output hashes are recorded in the JSON. The aggregate contains all 589 unique supported candidate roots across 175 native games, with no duplicate or missing root. The three source packets contribute 240, 151 and 198 roots. All six prior and five new corrections are applied in source order, with exact before/after categories and retained correction records.

Independent set-based tallies reproduce every category move/game count, family breakdown, phase/color/result/severity count, leave-one-family denominator, threshold subset and phase-sensitivity count. Every row joins the correct native family, candidate color, result and frozen phase. Threshold counts use the completed audit-threshold-normalization artifact, not provisional ranges. Uniform tolerance and additional unresolved/straddling rows remain separate.

Unknown remains 86 moves in 50 games, split 82/3/1 across the packets. The packet coding granularity caveat is explicit. Planning has an explicit zero coded count. Category tags overlap, so their counts must not be summed as independent errors or read as a precise defect-frequency ranking. Threshold counts within each category apply only to the baseline 589 coded roots; the aggregate does not assign invented categories to additional threshold-supported or unresolved roots.

The field families_by_phase_color_result reports per-family marginal counts, not a three-way cross-tab; individual events retain the joint fields. This is a mechanical quantitative approval. It does not replace the separate scientific challenge or claim exhaustive common-rule recoding. No inputs were modified and no engines, searches or cloud jobs ran.
