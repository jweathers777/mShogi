# Behavioral coding challenge

The 391 supported candidate roots reconcile exactly: 240 in Batches 01–04 and 151 in Batches 05–07. Query hashes and all 820 stored reference lines replay correctly from the supplied boards. Category counts were recomputed using distinct roots for moves and distinct pair/game keys for games. Multiple tags overlap; this arithmetic is suitable, but the category attribution rules are not yet uniform.

This is an independent review of the 05–07 packet and a self-consistency review of the 01–04 packet, which I authored. A fresh reviewer must still challenge the latter before final priorities. Neither source packet was changed.

## C01: cross-packet scope

Unknown counts are 82/240 in A and 3/151 in B. A leaves many general activity comparisons unknown; B commonly labels a stronger route as coordination and adds tactical/pawn/endgame tags. This is coder threshold variation, not measured behavioral variation.

Do not compare category rates or interpret unknown disparity as a batch effect. Use a common category evidence rule. Retain unknowns unless exact context supports a category; do not fill A to match B.

## C02: sensitivity population

main-05-01/g1/p49, main-05-03/g0/p42, main-06-00/g1/p49, main-06-04/g0/p52, main-06-04/g1/p17, main-07-02/g0/p18, main-07-02/g0/p24, main-07-02/g1/p25. A includes stable straddlers at any .04/.05/.06 threshold. B lists seven unresolved .05 cases, omitting at least these eight source-tagged subthreshold .04 straddlers.

Use one separate sensitivity population across packets. Add these eight source-tagged cases to a harmonized sensitivity overlay; retain supported_subthreshold at .05 and do not add them to supported category counts. Reconcile old schema sensitivities from raw retained ranges rather than assuming identical field names.

## C03: category overreach

main-05-00/g0/p24. The rationale describes Qb3 queen activity. The board and replay show Qd3 permits ...Rd8 and later exchanges, but the text does not isolate a forced tactical loss or necessary tactical resource.

Retain coordination/activity as a moderate descriptive tag. Remove tactical calculation from the conservative tally unless a specific verified threat or exchange dependency is supplied. A legal PV and a large margin alone do not establish the tactical category.

## C04: category overreach

main-05-03/g0/p20. h3 attacks the bishop on g4 before Nxc5. This is a concrete tempo and exchange-order contrast, not evidence that pawn structure is the issue.

Remove pawn structures/breaks from the conservative tally. Retain tactical calculation only in its descriptive move-order sense: h3 Be6 Nxc5 Bxc4 Qa4. No mechanism claim follows.

## C05: category refinement

main-05-04/g0/p84. After h6 the tested line allows ...Nxf5 and ...Nxh4+, whereas Ng3 protects f5 and h5 and supports king activity. The old phrase handles the pawn structure does not state this interaction.

Replace pawn structures/breaks with coordination/activity; retain endgame decisions. Use the concrete knight coverage and opposing captures as rationale. The ending phase alone is not the evidence.

## C06: category refinement

main-07-unaided/g0/p14. The alternative Ng5 moves an already-developed f3 knight. Be3 develops the c1 bishop; the alternative line instead brings both knights toward g5/b5 and c7.

Retain coordination/activity. Remove opening/development from a conservative defect tally unless the category explicitly includes opening move choice without implying inadequate development. Do not infer delayed development from the phase label.

## C07: opponent attribution

main-05-03/g1/p23. The rationale says after the reference concession, but its supplied selected anchors omit the immediately preceding reference root p22. The native board verifies ...Bh3 as a legal local resource, not that the previous reference move created it.

Remove after the reference concession unless bound earlier/alternative evidence establishes it. Retain the local resource description. Separate opportunity response meaning a missed available resource from a confirmed opponent-created opportunity.

## C08: category refinement

main-07-08/g0/p80. Rb7+ gains checking activity before the rook reaches d7 and captures d5. The played h4 line also captures d5, but with different timing and pawn placement. A pawn move by itself does not establish a pawn-structure failure.

Retain endgame decisions and the concrete checking/timing description. Remove pawn structures/breaks from the conservative tally unless the h-pawn placement consequence is shown. The tactical tag is a descriptive forcing sequence, not proven search failure.

## C09: episode and primary limits

main-02-00/g1/p81, main-02-00/g1/p91, main-02-00/g1/p113. A tags three immediate-promotion resources as advantage preservation/conversion and explicitly denies a newly certified favorable episode. B uses primary to mean absence of a secondary tag. Neither establishes a primary loss cause or a complete favorable episode.

Keep promotion-choice support and its three-root motif, but do not count three failed conversion episodes or a certified favorable episode. Rename primary counts nonsecondary-tagged. A missing historical secondary tag must remain unknown, not false.

## C10: consistent opportunity semantics

A often uses opportunity response for any concrete missed resource; B uses it sparingly and at least once ties it to a reference concession. These are different category scopes.

Use available concrete resource as a behavioral tag only with exact rationale. Add a separate opponent-created boolean with supported/unknown evidence, never infer it from adjacency. Do not aggregate current opportunity counts as a common rate before this distinction.

## C11: review independence

The challenger authored A. This is an independent review of B and a self-consistency review of A, not an independent challenge of both packets.

A still needs a fresh reviewer for the final independent challenge. Both packets also need common-rule sampling before category-based priorities. Existing source numerical support is unaffected.

The six representative board cases and full legal lines are bound in the JSON. These corrections concern behavioral attribution, not the retained 20M support. They do not establish search or evaluation mechanisms. Native claims, unresolved earlier roots, secondary comparisons and contrary successes retain their original limits. No new engine query or cloud action was used.

## Additive correction overlay

The JSON gives six exact before/after category rows, packet and source hashes, and revised category counts for those six corrections only. The reference-concession phrase at main-05-03/g1/p23 is withdrawn; its local resource tags remain. The original packets remain unchanged.

All non-finite candidate roots in Batches 05–07 were checked from retained raw evidence at .04/.05/.06, with the source stability and bound gates and 1e-9 comparison tolerance. There are 20 stable sensitivity straddlers and 14 are absent from the packet list. Exact rows and source/query hashes are in the overlay. These are sensitivity cases, not added .05 supported events.

- main-05-01/g1/p49: supported_subthreshold; range [0.024499999999999966, 0.04749999999999999]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-05-03/g0/p42: supported_subthreshold; range [0.03949999999999998, 0.045999999999999985]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-05-unaided/g1/p7: supported_subthreshold; range [0.033999999999999975, 0.044499999999999984]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-06-00/g1/p49: supported_subthreshold; range [0.03149999999999997, 0.044499999999999984]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-06-04/g0/p52: supported_subthreshold; range [0.0355, 0.042]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-06-04/g1/p17: supported_subthreshold; range [0.033499999999999974, 0.04149999999999998]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-06-05/g1/p33: supported_subthreshold; range [0.044499999999999984, 0.0595]; sensitivity {'0.04': 'supported', '0.05': 'straddles', '0.06': 'below'}.
- main-06-06/g1/p29: supported_subthreshold; range [0.04150000000000004, 0.063]; sensitivity {'0.04': 'supported', '0.05': 'straddles', '0.06': 'straddles'}.
- main-06-unaided/g1/p7: supported_subthreshold; range [0.033999999999999975, 0.044499999999999984]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-07-02/g0/p18: supported_subthreshold; range [0.024499999999999966, 0.04349999999999998]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-07-02/g0/p24: supported_subthreshold; range [0.02799999999999997, 0.04199999999999998]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-07-02/g1/p25: supported_subthreshold; range [0.02850000000000008, 0.043500000000000094]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-07-unaided/g0/p28: supported_subthreshold; range [0.02100000000000002, 0.040000000000000036]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.
- main-07-unaided/g1/p7: supported_subthreshold; range [0.033999999999999975, 0.044499999999999984]; sensitivity {'0.04': 'straddles', '0.05': 'below', '0.06': 'below'}.

The existing main-05-05/g1/p43 entry has a .05 lower edge represented as .04999999999999999. It is a boundary-convention case, not a strict straddler under the later-ledger 1e-9 tolerance. Retain its original unresolved status separately. Thus the preserved union has 21 sensitivity records: 20 strict straddlers plus this existing boundary case. No source status is upgraded.
