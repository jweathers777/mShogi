# Classical evaluation diagnosis: development endgames

## Evidence and limits

Read-only inspection uses the MSH-158 traced predecessor (`17529cd+trace`, binary prefix6391d9e5), its `endpoints15` native events, current `chess_conventional_eval.rs`, and the full-history MSH-157 reference queries000–003. No engines, builds, heldout data or production edits were used. Reference lines are legal, finite, reviewed supporting analysis; they do not establish minimax proof of every alternative. Scores below are always normalized to the named original side. A static ranking disagreement is a diagnostic observation, not automatically a defect.

## 0UOyM: route before race

The retained three-second forced-root reference prefers exf3+ to Kg6 and supplies a concrete difference: Kg6 g4 closes access and leads to a maneuvering continuation; exf3+ Kxf3 Kh6 permits a king route to capture g5 and a g-pawn promotion before White's queenside promotion. Its stronger-defense line starts g4, not the published Ke3. Analysis must therefore include g4/Kf4/g5 and not assume the published reply is uniquely best. The reference eventually has Black use its new queen to stop White's pawns; promotion priority alone is insufficient evidence of conversion.

Actual native static evidence:

| Position/history suffix | Static score, Black | Terms explaining it |
| --- | ---: | --- |
| Root | +28 | Material equal; pawn terms +40 offset placement−12 |
| Kg6 g4 (Black to move) | +48 | Material equal; placement+6 and pawn terms+42 |
| exf3+ Kxf3 (Black to move) | −52 | Material and pawn terms equal; placement alone−52 |
| exf3+ Kxf3 Kh6 (White to move) | −38 | Equal material/pawn terms; placement−38 |
| exf3+ Kxf3 g6 (White to move) | −42 | Equal material/pawn terms; placement−42 |

The same-side-to-move comparison Kg6 g4 versus exf3+ Kxf3 gives a clear native static ordering opposite the supported root continuation preference. It remains a different-ply, different-material-position comparison, not an exact interchangeable search leaf. The narrower sibling Kh6 versus g6 ranking is already correct by4cp; the main weakness suggested is missing long-term route utility, not necessarily a wrong local king-step ordering.

The evaluator rewards endgame king centrality, gives king-distance escort only to strict passers, and has no general king-to-vulnerable-pawn activity term. Before a passer exists, the useful flank route has little representation. In the actual Kh6 line, the search admits the route unreduced and eventually uses a depth6 TT lower bound+55 for White at node6309613. That is not a directly evaluated leaf. The alternative g6 branch's directly evaluated leaf (node5973826) favors White+43 and has White a5 promoting first; it is valid counterevidence to a blanket Black race bonus.

The prior300cp clear-passer race experiment changed neither the five-second root nor its score. Do not repeat it as an established fix. Earlier route valuation and safe capture opportunities are a distinct hypothesis; the unresolved search horizon and bound propagation remain relevant.

## 0m5mI: the bishop's job versus its mobility

The full-history reference's Kc3 line continues f4 exf4 e3 Bc1 e2 Bd2 **Ba6** Kb4 Kf5 Kc5 Ke4 Kb6 Kxd4 Kxa6. This defense differs from the published Kf6 and must be represented. White then combines bishop control of e1 with king penetration; later it sacrifices the bishop for the d-pawn, promotes the a-pawn with a checking queen maneuver, and retains a winning-looking f-pawn race. This is coherent finite evidence, not a proof that every intervening Black alternative loses.

The native trace already reaches Bc1 at completed iteration11, unreduced childdepth6. The passed-extension trial's depth7 witnesses are from its unfinished iteration11, and are not stronger completed-search proof.

At the actual position after Kc3 f4 exf4 e3 (`8/8/6k1/P2p3p/2bP1PpP/B1K1p1P1/8/8 w - - 0 49`), White static is+27. Sibling successors with Black to move are Bc1:+12 and Kc2:+14: a slight static preference for king retreat. Bc1 costs20 endgame mobility cp and11 middle placement cp, with only6 threat cp returned. The useful defensive role is not separately represented.

After Bc1 e2, White static becomes−50: Black's pawn endgame total rises146→214. Bc1 controls e3 before the advance, but does NOT control e1 after e2. The −50 is therefore not evidence of undervalued existing promotion control; Bd2 must first be played with tempo. The current passer formula discounts any unsafe path by only one quarter of its advancement base; it distinguishes neither stable promotion-square blockade from transient attack nor the cost of dislodging the controller. After Bd2, White static is+21 and Black pawn-end total143. Bd2 is protected by Kc3 and controls e1, freeing the king. At that same decision, native static **already prefers Bd2(+21) over Kd2(+5)**. Do not claim it misses all bishop-versus-king blockade rankings.

Thus the strongest supported feature deficiency is a hypothesis of limited valuation of stable promotion control once Bd2 is actually established and a bishop's useful defensive role; the root cause is not yet proven. Native centrality and mobility can favor a bishop/king maneuver with many available squares over a constrained but useful blockade. The reference root preference alone does not prove every such static comparison wrong.

## Focused experiments, separately measured

1. **Secure promotion-control discount.** Reduce the advancement component of an advanced passed pawn when a safe enemy minor controls its promotion/front square and the pawn cannot change files to bypass it. Use native geometry, bounded scores, and inspectable terms. Start with promotion-square control by a protected bishop in minor-only endings; distinguish a currently occupied blockade from control at a distance. Do not treat this as a draw recognizer. Expected effect: remove excessive apparent urgency of e2 when Bd2 can maintain control, improving quiet defensive allocation. Negatives: undefended or pinned controller, king able to attack it, pawn capture promotion on another file, supporting rook/queen, two separated passers, same-colored bishop exchange of the blockader. Compare Bd2 against Kd2 under both Kf5 and Ba6 defense, not only the published line.

2. **Pawn-ending king activity toward vulnerable pawns.** Add a bounded, endgame-only proximity/activity term for approaching enemy pawn targets that lack pawn protection, with current pawn attack barriers represented conservatively. Keep own pawn defense relevant and avoid blindly rewarding central squares or the nearest inaccessible target. This is distinct from an unstoppable-passer bonus: it can matter before the kings create the passer. Expected effect: value the useful flank route in 0UOyM and other pawn endings. Negatives: poisoned pawn capture, square-of-pawn defense duties, protected chains, remote king on wrong side of a pawn barrier, opposition/zugzwang; preserve exact KPK specialization. Test both g4 and Ke3 defenses. A distance-only term remains a heuristic and may fail because topology/tempo dominate; do not replace it with unbounded static search.

3. **Role-sensitive minor activity, only if experiment1 exposes a residual ranking failure.** Give modest credit for a safely maintained pawn blockade or promotion-control duty rather than only counting attacked squares. Keep this isolated from the passer discount to expose double counting. Reward one verifiable role, not generic bishop presence or exact square patterns. Controls must include a genuinely trapped bishop, a blockader that can be exchanged, mobile bishop attacking pawns on both wings, and a blockader tying down its own king. If experiment1 already supplies the same benefit, omit this trial rather than stack bonuses.

The first two are distinct useful hypotheses. Neither requires accepting a reference centipawn scale. Retain only supported static rankings with independently justified consequences and five-second decision benefit (or clearly established evaluation benefit supporting those decisions), with relevant negative controls. Keep 28pJK's corrected Qc8 decision as a control; its remaining numeric gap alone does not authorize compensation tuning.

## Conventional precedent and next evidence

[Stockfish11 classical evaluation](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/evaluate.cpp) distinguishes passer advancement, king proximity and attacked/defended paths, and separately assesses minor activity. These concepts motivate separating the mechanisms; its weights and scale are not targets and its code is not a correctness oracle.

For each selected intermediate position, preserve the complete original history plus legal continuation, record exact native static terms, and ask the reference only the smallest necessary sibling/defense questions. Most useful missing evidence is the ranking of **Bc1 versus Kc2 after e3**, then **Bd2/Kd2 against Ba6/Kf5**, and **Kh6's g4 response versus the closed Kg6 g4 position**. Existing traces establish that relevant branches are reached, but do not establish exact terminal outcomes or that the decisive score came from every reported PV endpoint. If a robust ranking cannot be established or meaningful decisions do not improve, report remaining coverage/valuation interaction instead of manufacturing a retained correction.
