# Chess decision corrections

## Problem

The previous round did not establish broad decision improvement. Three development failures distinguish quiet pawn-ending conversion (0UOyM, Kg6 rather than exf3+), bishop blockade/king penetration (0m5mI, Bd6 rather than Kc3), and tactical defensive coverage (28pJK, Qf4 rather than Qc8). Published puzzle moves and finite reference opinions are evidence, not complete root WDL maps.

## Objective

Find and test general corrections to the actual interior decisions under five-second primary thinking time. Retain a correction only with a meaningful continuation improvement and preserved development controls. The candidate and evaluation procedure were frozen before held-out access.

## Approach

Bounded actual-root traces preserve paths, node/parent identities, nominal and effective child depth, windows, PV status, reductions, extensions, TT context/bounds, and static terms. A cached PV is not necessarily the evaluated leaf that established a returned bound; a narrow-window fail-low is not an exact score. Frozen experiment manifests and source/binary hashes are `control/experiment-{1,2,3,4,5}.json` in the [evidence archive](evidence/msh158/README.md); sequential attempt receipts are archived under `native/`. Original absolute paths remain execution provenance; use the archive manifest to map retained bytes.

**0UOyM:** The important exf3+ Kxf3 Kh6 line is admitted and searched without reduction. In the traced predecessor’s completed iteration 14, after Kh6 Ke3 Kg5 Kd4 Kg4 Kc5 Kxg3, node 6309613 uses a same-context depth 6 TT lower bound of +55 for White. Its cached continuation Kb6 g5 Ka6 g4 Kb5 Kf4 suggests a pawn race, but does not establish a directly evaluated decisive leaf. In contrast, an actual quiescent endpoint on the different g7g6 branch is White Kb6/a5/a3 versus Black Kf3/b5/g5, White to move; it returns White +43 via stand-pat fail-high, not an exact minimax value. Here White's a-pawn promotes sooner, so a sound race forecast must favor White, contrary to the desired Black root result. The obstacle is distinguishing quiet king-route, clearance and promotion consequences across competing branches before static material/king placement dominates; blanket root-admission or LMR explanations do not fit the observed route. Evidence: `endpoints15/cell-0000.native.jsonl`, especially nodes6309607–6309613 and5973826.

**0m5mI:** At five seconds, Kc3 is cut off by an earlier king defense and a return to Bd6. At fifteen seconds, the baseline's completed iteration 11 reaches Kc3 f4 exf4 e3 Bc1; Bc1 is unreduced at child depth 6 (nodes4506278/4868355, four credits). Merely reaching it does not show that the subsequent blockade and king penetration were valued correctly. The advanced-passer trial selects Kc3 at five seconds, but retains +1 and a PV returning through Bd6/Be5 and king retreats: a tied-score tree change without demonstrated understanding of the blockade. Its fifteen-second Bc1 depth 7 witness belongs to an **incomplete** iteration 11; the last completed iteration remains10. It is diagnostic coverage, not a completed-depth improvement. The unresolved obstacle is adequate best-defense coverage and valuation of the later quiet blockade/king invasion. Evidence: `endpoints15/cell-0001.native.jsonl`, `bishop-candidate-trace15/cell-0000.native.jsonl`, and `passed-extension/runs.jsonl`.

**28pJK:** Initial five-second traces show Qc1 and Qc8 reduced and their fail-lows accepted, motivating two LMR trials. Their Qf6 choice remains inadequate. Trial 3 exposes a more specific horizon inconsistency, subsequently confirmed directly in the original baseline: iteration 6 node 60936 after Qc8 searches a synthetic null and detects a mate-band threat; PV verification node 103005 has the same real native context, nominal depth 5, four credits and window[−38,282], but loses that extension. Baseline b3 children144702/148180 enter depth 4/four credits. The isolated correction reuses the cached threat and its b3 children180479/196708 enter depth 5/three credits. Earlier children change the b3 search windows, so this is direct evidence of preserved horizon, not a one-variable score experiment. The later baseline iteration 7 still accepts a reduced Qc8 fail-low; the diagnosis does not attribute the entire final decision to a single score substitution. The original depth 6 witness at nodes738092/964255 belongs specifically to trial 3.

The correction caches only matching native context, nominal depth and extension credits, retaining the existing shared credit bound and threat protections. The five-second choice becomes Qc8 with b3 Rd2 Qc1 Qa6 in the continuation; fifteen seconds also retains Qc8. Independent full-history reference analysis supports Qc8 over Qf4: Qc1 counters the latter, while Qc8 organizes rook invasion and queen checks. Native negative scores remain heuristic estimates and show that valuation is still imperfect. Evidence: `trace5`, `queen-candidate-trace5`, `threat-trace5`, `threat-trace15`, and the [independent trace review](evidence/msh158/threat-trace-review.md).

| Trial | General change | Five-second result | Decision |
| --- | --- | --- | --- |
| 1 | Disable PV-node LMR; retain non-PV reductions | Queen Qf6, depth 7, −181; endgames unchanged | Reject: changed move still weak; paired finite reference check does not establish adequate correction |
| 2 | One-ply advanced passed-pawn extension using existing credits | Bishop Kc3, depth 10, +1; pawn Kg6; queen Qf4 | Reject: tied root reorder without demonstrated blockade consequence |
| 3 | Full-depth verification for reduced PV results within 40cp of alpha | Same Qf6, depth 7, −181; endgames unchanged | Reject: narrower LMR policy still fails continuation objective |
| 4 | Preserve same-context null-mate-threat extension across PV re-search | Queen Qc8, depth 8, −122; endgames unchanged | Selected for frozen wider evaluation |
| 5 | Pawn-only clear-passer race forecast, ≥3-ply promotion lead, 300 cp cap, conservative interception and optimistic opponent tempo | Pawn remains Kg6, depth 13, −32; other target choices unchanged | Reject: synthetic behavior does not translate into target decision benefit |

Each eleven-root trial preserved all eight control decisions; the initial three-root threat trial was followed by the complete eleven-root `threat-controls` run. Control preservation is a small development check, not a broad-strength result. The six listed run folders contain58 successful five-second attempts totaling290.015844 active seconds; this is only this trial/control subset, excluding traces, references and all other campaign charges. The global registry remains the resource authority.

The trial concepts were informed by established conventional designs: [Stockfish11 search](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/search.cpp) contains advanced-passer extensions and PV/history-sensitive reductions; [Stockfish8 search](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_8/src/search.cpp) protects advanced pawn pushes from selected omissions; [Stockfish11 evaluation](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/evaluate.cpp) combines passer advance, king proximity and path control. These primary sources motivate hypotheses; they do not validate mShogi's implementation or establish success on these roots.

## Completion

Retained **one general correction** at source `e93a750`: preserve compatible detected
mate-threat context across verification. Four other trials were reverted. No production
tuning followed the candidate/procedure freeze. The outcome is a supported local
five-second decision improvement, **not broad decision or playing-strength improvement**.

| Frozen comparison | Result |
| --- | --- |
| Development, 160 roots at 5 s | Qf4 → Qc8 on 28pJK; other 159 choices unchanged |
| Held-out, 80 roots at 5 s | All 80 choices unchanged |
| Existing fast screen, 40 roots at 1 s | All 40 choices unchanged |
| Exact objectives, primary240 | All 48 tablebase WDL values and3 immediate-mate goals preserved |

The frozen preference-only grader leaves Qc8 unresolved; the separately reviewed
full-history, equal3s reference pair supports it through actual continuations
(Qf4 −438 cp versus Qc8 +287 cp, Black perspective). These are stable finite opinions,
not proved WDL or an exact loss value. No new final reference queries were needed.
Excluding both development roots in prior-exposed `near-root-cluster-001` gives the
same conclusion over 158 development / 80 held-out roots. This reuses the existing
holdout as requested; it is not a new population sample. No invalid moves, failed
native runs or changed exact outcomes appeared. See [all comparisons](evidence/msh158/comparison/summary.json)
and [independent adjudication](evidence/msh158/comparison-review.md).

Level 1 local verification on macOS arm64 passed 82 Chess search tests (one manual
profiling test ignored), the console claim-admission test, targeted cache-context
regressions and release build. Source hashes tie these checks to the frozen candidate.
Independent review confirms native outcome/claim precedence, history-key isolation,
bounded credits, restoration and cancellation. Initial temporary-test compilation
and an illegal trial fixture failed and were corrected; the trial was later removed.
No Linux/Windows or GUI campaign ran; broader compatibility remains at the next
significant integration/release checkpoint. See [verification](evidence/msh158/local-checks/receipts.json)
and [source review](evidence/msh158/final-source-review.md).

All 360 native attempts succeeded; 2 new finite-reference queries also succeeded.
New timed engine computation was 1,568.80 s. Including the conservative120s test
allowance gives 1,688.80 s; the registry also retains unused reference reservation,
for 1,802.80 s against the 2,400 s cap. No matches ran. The artifact audit verifies
2,894 original fingerprints, complete ledgers, exact input histories and no
native/reference overlap. Raw traces and rejected variants are in the
[evidence bundle](evidence/msh158/README.md).

Both endgame failures remain unresolved. Useful next evidence would distinguish
backed-up best-defense values along the quiet king routes from independently
established race/blockade consequences; merely reaching the thematic move is
insufficient. This round does not select a general material-compensation term:
MSH-153 (Classical Compensation Calibration) remains held. No further implementation,
benchmark expansion, tournament, learned evaluation or cross-game work was started.
