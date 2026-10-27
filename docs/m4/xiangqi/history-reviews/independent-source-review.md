# MSH-94 independent source and fixture review

Reviewer: `/root/xq_history_reviewer`, separate from root builder and both source-fixture authors. 2026-10-27. This is an in-progress review; no product implementation has been assessed or accepted by this document.

## Actual method

Read the user batch request, MSH-94 acceptance criteria, selected XIANGQI-P1 profile/owner decisions, source reconciliation and fixture ledger. Independently read cached primary WXF2018 EN definitions/articles3/8/19/20 and relevant chapter5 examples. Visually inspected original renders EN14/15,44,46,66,106/107 and ZH27/28/29,35/36/37/38,49; independently rendered ZH85 at300dpi and inspected both diagrams109/110, their text and ownership glyphs. EN/ZH governing PDF identities are the existing recorded SHA256 values; no comparator history routine or engine adjudication was used.

Paper-traced complete C/Q/D/T routes and the constructed material/pin/root cases; native probe observations are separately labeled board-legality observations only. Awaiting corrected final source-fixture artifacts and G-NOWIN proposal before final verdict. Source review is independent of author self-review; no claim of product conformance follows from these research checks.

## G-CYCLE finding

WXF19.8 requires continuous checking on every move,19.10 successive pursuit of the same actual piece,19.16/20.2 distinguish permitted mixed conduct. Source therefore forbids cherry-picking noncontiguous C subsequences through Q/T. Changed literal coordinate words do not cure unchanged checking responsibility: C+C+D is the important eight-ply alternate-route witness. A warning is historical and must not be erased; after non-check continuation it cannot independently justify a later all-check loss.

The primary text does not prescribe first-versus-latest anchor data structures. The proposed whole contiguous first→third witness and preserved witness plus contiguous continuation to literal fourth board occurrence is a consistent minimal deterministic completion of the already selected software3rd/4th-occurrence policy. Document that as software witness policy, not a quotation or official WXF anchor algorithm. I have found no concrete contradiction requiring another owner decision in the C/Q/D/T families. Review remains contingent on actual final fixtures and native handling of all relevant conduct, not a generic all-check-only shortcut.

Do not overstate Q as definitely Idle. It has non-check moves and no non-General victim, sufficient to refute all-check/all-chase; WXF19.2 permits a mating threat via a series of checks, and19.7 Idle excludes all other categories. Its kill/idle distinction is outcome-equivalent under20.2 for these particular overlap witnesses. The source worker accepted this correction.

## Material fixture findings (sent to author before product coding)

1. `horse-protected-rook`: original proposed Black Horse d7 blocks the advertised Black Rook h7→b7 recapture. Thus the claimed two real roots are false. Suggested simpler corrected fixture has protector Black Rook b9, target Black Rook b7/b6, no d7 Horse; both squares are genuinely protected down the b-file. The original expectation must not be silently accepted or copied into product tests.
2. `diagram109`: original FEN incorrectly assigns Rook f8 to Black. The primary Chinese high-resolution pixel has unfilled Red Rook f8; Black Rook b9 is filled. Red Rook f8 also makes the published mating threat concrete, rather than allowing a misleading test with a different tactical position.
3. `diagram110`: original FEN incorrectly assigns Cannon c4 Red, Cannon h8 Black, Horse g8 Black. High-resolution primary shows c4 filled Black; h8 and g8 unfilled Red. Correctly, Red Cannon h8 persistently attacks Black Rook f8 over Red Horse g8. This resolves the initially observed non-collinearity; it is a transcription error, not a demonstrated rulebook contradiction. Preserve correction provenance.

The primary bilingual example110 explicitly excludes that persistent Cannon attack from this cycle: Red moving Rook alternates chase/idle, while Black Horse perpetually pursues Red Rook. All-postmove legal capture sets alone are therefore insufficient to classify a move as a causal chase.

## Other independently checked initial facts

- Unrooted Rook/Horse chase: a4c4 threatens Horse c5, c5a6 escapes; c4a4 threatens that same Horse a6, a6c5 escapes. No root, same-kind or Soldier exception.
- Real-root Horse: Black Rooks h5 and h6 can legally recapture a Red Rook on c5 and a6 respectively; no discovered General exposure.
- Fake-root Horse: c9c5 after capture vacates the a9→f9 pin line and exposes Black General f9. Geometric recapture is not a real root.
- Same-type free Rooks: reciprocal legal capture, permitted20.7/20.8.
- Same-type pinned Rooks: target c9/d9 can shift along rank9 but cannot capture the chasing Rook below because that leaves its General exposed to a9 Rook. The20.7 pinned-line exception applies.
- Published37: Red Rook b0/b1 pursues Cannon a1/a0. Cannon a0 checks General e0 over Elephant c0; Cannon a1 chases Horse i1 over Horse e1. This is the published permitted Black check/chase mixture, not Black all-check.
- Published38: printed initial Rook i6 is setup; stable repeated root Rook h6 after the first pass must be labeled as such. Its c6/h6 moves pursue Cannon c7/h7. Do not pretend the initial four-ply path returns to the printed initial board.
- Published109 chase target is the same Black Elephant g9 via discovered Cannon g2 attack. Moving Horse g6/h8 and Black Elephant g5/e7 alter the screens; legal protection is tested after actual capture. The more severe chase meaning controls despite simultaneous mating threat.

## Semantic guard for implementation review

A bounded mate-in-N detector missing a kill cannot label the move Idle. WXF19.2 includes a series of checks leading to mate. Unknown work is distinct from a proven absence of mating threat; all classification and fast no-win claims must retain their exact proof scope. G-NOWIN remains under review, and the separate full exact backend105 is not dependency-ready.

## Follow-up source gate decisions and corrected fixture review

The source author corrected every finding above with explicit old FEN/finding metadata. Re-reviewed corrected `conduct-fixture-proposals.json`:12 cases, including additional stationary-Cannon mounted pursuit of crossed versus uncrossed Soldier. On the b9 protector correction, recaptures b9b7 and b9b6 remain legal. In corrected109 Red Rook f8 is essential for the printed mating threat; discovered Cannon g2 chase targets Black Elephant g9. In corrected110 Red Cannon h8 captures f8 over Red Horse g8 at every cycle phase; Black Cannon c4 is unrelated to that persistent Red attack. Source110 still requires the unchanged Cannon attack to be excluded.

The two new screen fixtures are independently coherent: Cannon a0 is stationary, Rook b1/a1 and opposing Horse c1/a2 alternately remove/add screens, and the same Soldier remains a4 or a6. After each Red move the Cannon has exactly one screen; after each Black reply it has zero or two. Black Soldier a4 has crossed, a6 has not. WXF20.3 and chapter5 section20 make the respective prohibited/permitted distinction. No rooted defender can legally recapture on either target square.

G-CYCLE research is acceptable as the documented minimal software witness rule, subject to tests of actual native behavior. The source does not uniquely prescribe the anchor algorithm; do not represent it as a discovered WXF quotation. The independent correction of Q's overstrong Idle label was incorporated.12 conduct cases plus11 overlap histories are a useful accepted seed, not proof of complete WXF classification. Source author's explicit remaining families remain required before MSH94 full conformance can be claimed.

## G-NOWIN semantic definition reviewed

Accept the source worker's explicit existential question as consistent with both3.2A wordings: for each side, does some finite admitted board-action continuation produce its native win by opponent checkmate, stalemate or selected continued-prohibited-conduct loss? The state includes native board/turn/stable piece identities, complete known history, warning state, selected claim policy and prior conclusion. Every prefix obeys earlier automatic outcomes, including forced selected auto-claim draws. Optional claims do not produce wins and may be omitted in existential exploration. Exclude voluntary resignation/agreement, tournament/administrative sanctions and the recursive3.2A result being decided. This is possibility, not optimal-resistance forced win.

This definition is an operational reading of the approved source obligation, not a change of profile. The distinct3.1 resignation/admin grounds and impossibility wording justify excluding those voluntary/administrative witnesses; otherwise every position would trivially appear win-capable. Include actual selected history-fault wins because they are native rule consequences of locally legal actions. The F002 C^3 Black-win witness catches a mating-only replacement of this definition. It does not claim a general verifier exists.

Independently sound negative seed: bare Generals d0/f9. Palaces are disjoint, no captures/checks/pursuits are possible in legal positions; each General has a remaining legal palace exit (facing-file illegality cannot eliminate all exits). Neither board mate/stalemate nor unilateral prohibited history can arise. This proof supports only the stated class, not a larger unproved material table. Backend105 and its feasibility remain unstarted.

## Natural-count fixture review

Read selected Art8 reconciliation and reviewed4 concrete fixtures with claimant-specific formula `known capture-free plies - max(0,own checks -10)`, eligible at100. Independently executed `python3 /private/tmp/msh94-research/reviewer_count_checks.py`: PASS. This reviewer-owned script replays placement only and directly computes straight-line Rook checks, captures, board+turn equality and count arithmetic; it invokes neither native product nor reference engine. Artifact `reviewer-count-observations.json` records full Red10/11-check ply lists, Black0, no repeated board in101-ply walks,99/100/101 boundaries and capture at27 reset. Imported declared100 with no known preceding actions has verified count0, not eligibility. These observations independently support arithmetic/history facts, not native outcome execution.

## Current review verdict

Source/semantic gates: reviewed and acceptable within the explicitly documented scope above. All material findings in the12-case conduct seed were corrected with provenance. Remaining uninstantiated conduct families are openly recorded, and must not be hidden by declaring the12-case seed exhaustive. No completed product boundary or final MSH94 acceptance review has occurred yet. Unknown classification or proof work must remain unresolved/unscored; correct handling of unresolved cases is not itself evidence that all required categories are implemented.

## Controlling owner decision received after provisional G-NOWIN review

Root relayed the owner's explicit decision during this review: the3.2A winning-continuation proof goal is **natural board wins only**, WXF3.1A checkmate or stalemate-as-loss. Repetition-fault penalties are not positive winning-capability witnesses; nor are resignations, agreements, clock/admin/misconduct/invalid-claim outcomes or the3.2A conclusion being decided. Complete history remains authoritative for actual game outcomes.

This owner decision controls and supersedes the prior provisional proposal above that included continued-prohibited-conduct wins. That proposal is retained only as historical research; its F002 C^3 Black-win example remains a valid actual repetition outcome but **is not a Black positive no-win proof**. The revised definition is still existential natural-win possibility, not minimax forced win. Its textual rationale is the separate natural-course3.1A ground versus later penalty/voluntary grounds. No broader general exact verification claim is made;105 remains unstarted. Bare-General negative proof and genuine one-ply mate/stalemate positive proofs are consistent with the controlling scope. A recognizer miss stays unknown, never a proof of natural winning capability.

## Additional independently reviewed completion seeds

- Published108 equal prohibited chase, EN105/97: Red Rooks d8/f8 versus Black Cannon i8, with Black Advisor d9/e8 supplying screens. Independently traced exact changes: Rf8f6 uncovers Rd8→Ci8, Ad9e8 blocks it and enables BlackCi8→RedRd8 over e8; Rf6f8 creates RedRf8→Ci8 and second Cannon screen; Ae8d9 restores Black capture over f8. Both continuously chase one persistent victim, so no one-sided responsibility. This is not a constant-attack exception accidentally copied from110.
- General-only chase, crossed-Soldier-only chase, General+Rook combined chase, blocked Horse reciprocal legs, and real-root-on-one-phase pinned Rook were independently paper-reviewed before their native executions. Current corpus expanded beyond the initial12 seeds; each new fact was frozen before using output as differential observation. The existing Rook/Horse seed is also explicit two-to-one counter-chase: Black Horse a6 does not attack Red Rook c4, while Horse c5 attacks Rook a4 via empty b5. Red chases on both own moves, Black only every other move.
- Identical Rook permutation witness: a4b4 f9f8 a6a4 f8f9 b4b6 f9f8 b6a6 f8f9 returns board/turn with two Red Rook IDs exchanged. The native closing-reply diagnostic must project the observed complete action through current IDs, never copy the old identity map or append a future accepted event.
- Natural/repetition precedence34-ply route:17 unique Red Rook states and180-degree Black counterparts give34 distinct board+turn keys; rootthird68/fourth102, no captures/checks. Each General always retains e0/e9 exit. Explicit claims are available100 but unexercised; repetitiondraw102 precedes a later request. AutoValidClaims selects Red100 and stops before102. Independently reviewed route injectivity and arithmetic, separately from native/Pikafish observations.

### Concrete mutual all-check witness (reviewed before execution)

Root `3c5/9/C4k3/1N7/3n5/9/9/3K5/9/9 w - - 0 1`; Red General d2/Cannon a7/Horse b6, Black General f7/Cannon d9/Horse d5. Cycle `b6d7 d5e7 d7b6 e7d5`. Initially Red alone is checked over d5. Each move changes the intersection of the two Cannon lines:

1. Nb6d7: leg c6 clear; incoming Black Cannon has two screens d7/d5, outgoing Red Cannon one d7.
2. Nd5e7: leg d6 clear; incoming has one d7, outgoing two d7/e7.
3. Nd7b6: leg c7 clear; incoming zero, outgoing one e7.
4. Ne7d5: leg e6 clear; incoming one d5, outgoing zero; root restored.

Every action resolves the actor's check and checks the other General. Native palaces/facing restrictions hold. WXF20.1 mutual all-check means no one-sided warning at8 and draw at selected fourth occurrence12. This is a real Xiangqi legal witness; a Shogi mutual-check impossibility argument cannot substitute.
