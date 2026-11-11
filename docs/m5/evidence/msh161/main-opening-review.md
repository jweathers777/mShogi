# MSH-161 (Competitive Gameplay Diagnosis): independent main-opening admission review

**Pass for the stated finite screening and coverage contract; no selection error found.** Reviewed 2026-11-11 independently of source selection. No engines run and no admission files modified. This review does not certify theoretical equality or statistical representativeness.

Inputs: `sources/main-roots.json`, `sources/main-selection.json`, the original 330 and supplemental 20 candidate records, four qualification exclusions, raw selected balance receipts, source/screen/supplement/confirmation manifests, and `screen_v1.describe`. Reviewed `final_admission.py` for policy consistency; independently replayed and checked selected records rather than treating its assertions as evidence.

Verification used `/private/tmp/msh139/venv/bin/python` with python-chess, JSON/gzip/hashlib and the existing `screen_v1.describe`, in a single read-only batch:

- All 90 IDs and resulting board/turn/castling/legal-en-passant root identities are unique. Every root has the complete 16-ply legal standard-start history. Each history exactly matches its recorded master-game prefix and candidate record; every resulting source FEN matches, including clocks and FEN en-passant representation.
- Replayed all 90 complete recorded source-game histories: 8,063 legal plies. Source archive/game/header/history identities match the selected candidate records. This verifies the retained source evidence, not a fresh archive download.
- All four qualification root IDs are excluded. Twenty families have four roots each; two have five, covering all 22 families with at least four each.
- Independently recomputed structural descriptors from the boards and actual castling moves: **16 symmetric pawn structures / minimum 12; 12 opposite-wing castled / minimum 10; 15 closed centers / minimum 15; 30 open centers / minimum 15**. Counts match the selection file. “Closed” uses both d/e pawn locks; “open” uses no such lock and at most two total d/e pawns. These are explicit board descriptors, not universal opening labels.
- Every selected raw receipt passes the stated finite gate: complete, zero exit, settled process group, untruncated raw record, independently replayable selected PV, aligned nonbound cp within ±60, White WDL expectation within [0.40, 0.60], last-two-distinct-depth spread ≤0.025, and no directional final-bound conflict. Receipt histories match the admitted roots. Receipt engine identity/options/node budget match their manifests.
- **61 of 90 selections retain the sole `last_score_update_not_selected_exact_score` flag.** None has another blocking flag. This is allowed by the recorded v1 contract; no raw flags were cleared. Stability and absence of directional conflicts remain heuristics, not proof that the selected score is the completed final iteration or that the position is objectively equal.
- Six selected roots use the deeper 5M receipts. Superseded receipt references/hashes are intact. Verified hashes for all selected receipts, main roots, original candidate collection, screen method, original screen manifest, final admission script, supplement manifest, confirmation manifest, and supplement candidate collection; the supplement's original source-manifest hash also matches.

Reviewed identities:

- `main-roots.json`: `8e2defc63c839d215ccbd4f1f667b777bfdfced74ef44a90a6698a8cca8de332`
- `main-selection.json`: `6e50ef5f46c7e782d32be6149dcbe0d14fc81af2b2cca311d46fba4a22bd8f4b`

The source-order/structural-coverage selection code does not inspect competitive results or sort accepted roots by favorable numerical score. Admission can proceed under this documented finite-screen policy, with the retained flags and provenance preserved. This review makes no additional strength, opening soundness, or production-mechanism claim.
