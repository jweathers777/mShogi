# Modern source reconciliation

MSH-82, 2026-11-02. [Current acquisition metadata](sources.json) records exact
public response hashes/editions/inspection. [Original research](../research/chu.md)
and its [32-GET provenance](../research/chu-provenance.json) remain unchanged.
We did not contact an institution, acquire a new historical facsimile, run another
engine or treat software agreement as rules authority.

| ID | Primary/source identity | Role and actual inspection |
| --- | --- | --- |
| C1 | JCSA [general rules2019](https://www.chushogi-renmei.com/kouza/rule2007.htm), corrected2019-09-14 | Adopted modern authority, I–IX Japanese text; source URL year2007 is not edition. Lion diagrams and IX example independently inspected. |
| C2 | JCSA [supplement](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm), updated2004-06-06 | Compatible explanations and exception priority. All five §2 images reconstructed; §1/3/4/5 checked. Earlier date does not give blanket precedence over C1. |
| C3 | JCSA [setup/movement](https://www.chushogi-renmei.com/kouza/koma.htm), revised2004-04-22 | All11 actual images inspected by integrator, including full setup and every movement/promotion family; Lion/SE/HF separately cross-reviewed. |
| H1 | NDL DOI[10.11501/1869566](https://doi.org/10.11501/1869566), *雑芸叢書第一*,1915, pp212–217/images121–123 | Previously directly inspected historical typeset witness and source of original seeds. Not original1703 printing; no historical-conformance claim requires it to be one. |
| T1 | JCSA [Okazaki transcription](https://www.chushogi-renmei.com/kouza/Okazaki.htm), web2001-02-14 | Corroborates compatible restricted forward jumps/return and two-royal treatment. Not an inspected original booklet or authority for superseded Lion rules. |
| L1 | [Lishogi's own Chu rules](https://lishogi.org/variant/chushogi), retrieved2026-11-02 | Named modern alternative and primary description of that service. No source/binary audit, comparator admission, execution or parity claim. |

These are working Japanese interpretations independently checked by separate
agents against actual text and diagrams, not certified human translations. The
specification states exact mechanics and distinguishing examples; no agreement
between researchers or software is substituted for source evidence.

## Reconciliation outcomes

| Issue | Current contract and supporting evidence |
| --- | --- |
| Initial central asymmetry | C3 actual setup/H1p214: Sente K7十二/DE6十二, Ln7十/FK6十, Ki7十一/Ph6十一. Count12+8+12+12+2=46 per side. |
| 29 versus39 identities | C1 counts movement faces. Eighteen original promotion mappings plus21 originals require39 identity states; once-promoted origins never chain. C3 diagrams/prose and L1's explicit example agree on one flip. |
| Distant defended Lion | C1 VII.2–3 includes defense exposed by vacating the attacking Lion. Intermediate adjacency does not waive the restriction (C2§2case4/companion). |
| Extra capture/recapture | C2§2cases1–3/§3 explicitly establish qualifying extra capture above protection and no new protection after Lion takes Lion. Pawn/GB are excluded extras. |
| Sole defender removed | C2case4 rejects removing the sole defending GB and then treating the target as newly undefended. Full action is one turn. Companion Pawn example rejects the newly-adjacent argument. |
| Non-Lion window | C1 VII.4/C2§3.6 require defended own Lion; capturing Kirin's promotion can create one. L1's unconditional window, even undefended, is an alternative, not selected. |
| Jump versus two captures | C3 image/prose and compatible T1 movement permit jumping over either owner's intermediate piece; direct jump preserves it, actual two-step captures it. |
| Quiet return | C1 VIII/C2§4 require a real empty in-board pivot. Occupied pivot can be captured for igui, not ignored as quiet pass. Initial surrounded Lion cannot pass. |
| Rescue/promotion | C1 IV/C2§1: optional Pawn last-rank rescue; no GB rescue; Lance rescue only correspondence. L1's ordinary Lance rescue is not selected. No hidden decline flag, compulsory flip or compound-promotable identity invented. |
| Royal admission | C1 II/C2§5 allow capture/no announcement; owner selects capture-based admission without Chess self-check. C1 IX actually shows King capture while Prince survives. |
| Mate | Published immediate mate implemented by C06's finite actual-threat/complete-response test. No forced-mate tree. Prince rescue/all-royals-in-one-action fixture challenges a false single-royal filter. |
| Quiet no action | No extra JCSA clause established. Report unresolved/unscored blocked board, preserving terminal commands, rather than add a loss/draw. |
| Repetition | C1 VI requires the responsible party to vary before4. Owner rejects automatic forfeit proposal. C07 last-opportunity native refusal preserves prefixes and targets the correct party; VIII's first passer is actual streak origin, not rotated anchor. |
| Action versus cycle identity | Keep every native route. Compare specified semantic complete-turn transitions for repetition; equivalent empty returns remain jitto and cannot evade VIII. No inter-leg occurrence. |
| Material | C1 II.4–5 narrow count, III royal classification and IX actual immediate recapture. Live P/GB count but cannot be winning extra; dead P/L ignored for counting only. L1 broader bare-royal rule not selected. |
| Draw | II.6 requires agreement; IX supplies the exhaustion example. Preserve eligibility/offer/consent/concluded draw separately; no automatic bare-royal result. |
| Physical/historical conventions | Touch-move/foul forgiveness and historical twenty-move interpretations do not enter the digital profile. Invalid requests reject atomically. No new historical acquisition gate. |

## Exact diagram reconstructions

Coordinates are file/rank integers, Sente forward rank−1. These are **partial
source diagrams**, not historical reachability claims. Native fixtures explicitly
add remote King/Pawn completion. A pictured second Lion is assigned promoted
Kirin origin when needed; this preserves its source movement face without falsely
attributing that hidden base identity to the drawing.

- C2case1: S Ln[3,3]/G[1,3]; G G[2,1]/G[2,2]/Ln[1,2].
  S Ln×[2,2]×[1,2]; G G[2,1]×[1,2] allowed.
- C2case2: S Ln[3,3]; G G[1,1]/Ln[2,2]/Ln[1,2].
  S Ln×[2,2]×[1,2] allowed despite both defended.
- C2case3: S VM[5,1]/Si[4,3]/Ln[5,4]/R[1,4]; G Ln[1,1]/Ln[3,2].
  S R×[1,1]; G Ln×[4,3]×[5,4]; S VM×[5,4], all allowed.
- C2case4: S Ln[3,3]; G Ln[2,1]/GB[2,2].
  S Ln×GB[2,2]×Ln[2,1] forbidden, despite removing sole defender.
- C2companion: S Ln[3,3]; G DK[4,1]/Ln[2,1]/P[2,2].
  Same two-leg attempt forbidden; intermediate adjacency is not a new turn.
- C1 hidden defense: S Ln[1,1]/B[7,7]; G Ln[3,3]/R[1,9].
  G Ln→[1,1] forbidden; remove B[7,7] and it is allowed. Vacating[3,3]
  exposes the defender's diagonal to[1,1].
- C1 IX image005: G K[5,1]/Ln[4,6]; S K[2,5]/DE[1,5].
  S DE→[1,4]成; G Ln×K[2,5]; S Prince×Ln[2,5]. Do not conclude on
  King loss or award material before actual recapture; final draw needs agreement.

Historical alternatives and unacquired books retain their original limits in
the old research; none is claimed reconciled by pretending to inspect it. The
selected modern authority and owner direction resolve implementation requirements.
