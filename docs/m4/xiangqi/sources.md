# Xiangqi authority and edition reconciliation

Research date 2026-10-27. Exact acquired PDFs, page counts, bytes and SHA-256 are
in [source-manifest.json](source-manifest.json). Page references below are
**one-based PDF pages**, followed by printed pages where useful. Copyrighted
rulebooks, full OCR and rendered pages are not committed. Local OCR located
passages; the material interpretations were checked against rendered pixels.
Chinese readings are research translations, not an association-certified
translation or a ruling obtained from an arbiter. No institution was contacted.

## Authority inventory

| Source | Established identity and use | Unestablished claim / remaining obligation |
| --- | --- | --- |
| Official [WXF English 2018](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf), 116 pages; [Chinese 2018](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_Xiangqi_Rules_Chinese_2018.pdf), 92 pages | Named bilingual World Xiangqi Rules publication. English Articles1–3,8,19–20 and chapter5 examples supply board, history and event material. Selected governing bilingual edition under XIANGQI-P1. No blanket language precedence. | No located official erratum or controlling-language statement resolves every inconsistency. Failure to locate one is not proof none exists. Full diagram transcription/review is still fixture work. |
| Official [WXF introduction](https://www.wxf-xiangqi.org/index.php?Itemid=291&catid=133&id=269%3Aworld-xiangqi-rules-introductionen&lang=en&layout=default&option=com_content&view=article) | Describes 2017 preparation, AXF fourth-revision ancestry and English translation/proofreading. | Ancestry does not make every older AXF clause a WXF clause. No automatic language precedence follows. |
| Official [AXF introduction](https://www.asianxiangqi.org/English/AXFrulesEng.htm), [English chapter](https://www.asianxiangqi.org/English/AXF_rules_Eng.pdf), [Chinese detailed rules](https://www.asianxiangqi.org/AXF%20Rules%20of%20XQ%20Competition.pdf) | Introduction dates booklet1982, revisions1989/2003 and English chapter1996. Acquired linked English21-page and Chinese32-page files. Useful comparison witnesses. | Exact revision of these undated downloads is **unverified**; neither is identified as the fourth revision by the inspected document. Acquire a dated cover/colophon or association edition identification before asserting equivalence. |
| Official Chinese sports authority [2026 national event regulations](https://www.sport.gov.cn/qpzx/n5387/c29676858/content.html), 2026-06-05, VIII.1; [2026 team championship](https://www.sport.gov.cn/qpzx/n5387/c29447425/content.html), 2026-02-27, VII.1 | Both explicitly adopt the **2020** competition rules. This resolves “2026 adoption” as event adoption of2020, not evidence of a 2026 rules edition. | Authoritative complete CXA2020 rulebook, diagrams and subsequent amendments **not acquired**. Secondary transcriptions/retailer metadata cannot establish exact authoritative text. No CXA/WXF equivalence or hybrid is accepted. |

CXA acquisition target: the association's complete 2020 competition code and
official corrections/adoption notices; establish publisher/copy/edition and
all relevant diagrams before comparing outcome clauses. Secondary discovery
leads (cnchess.net, gdchess.com, retailer ISBN9787500958321) are not adopted
authority. The inability to obtain authoritative full text in this bounded
public search is retained as an acquisition finding. Under the owner's current
XIANGQI-P1 instruction, complete alternative corpora are not blanket prerequisites
for unaffected WXF board/state work. Use scoped acquisition for an actual WXF
ambiguity; future AXF/CXA profiles remain separately named.

## Reconciled facts and consequential ambiguities

| ID / clause | Primary evidence | Result and fixture consequence |
| --- | --- | --- |
| XQ-R1 repetition count and warning | EN3.1C p14/6;3.2B p15/7;19.8 p44/36. ZH3.1C/3.2B p10/9;19.8 p28/27. | Chinese19.8 has **no number**; English adds threefold. Chinese3.1C says three cycles, referee requires change, then unchanged continuation loses. Neutral3.2B retains four-times wording. The earlier proposal was not a source-proven universal trigger. XIANGQI-P1 now selects classification/warning at occurrence3, continuation loss or applicable draw at occurrence4. This reconciles prohibited-conduct and neutral-draw clauses as an explicit software interpretation, not an official erratum or generic repetition counter. |
| XQ-R1 diagrams | ZH diagrams1/2 p31/30; EN diagrams1/2 p48/40. | Chinese lists omit Black's sixth reply in both; English supplies it only in diagram1. Do not silently insert moves or treat the printed end states as identical. Preserve original and explicitly reviewed completion separately. |
| XQ-R2 natural counting units | ZH3.2D p11/10; Art8 p16/15; EN3.2D p15–16/7–8, Art8 p25–26/17–18. | **50 rounds = 100 total plies** without capture. Deduct checks above10 made by the **claimant**, not both sides globally. Need side-indexed exact histories. A pawn advance is not a capture reset. |
| XQ-R2 claim/event policy | Same Art8; ZH continuation p17/16, EN continuation p26/18. | Formal player request or arbiter initiative; failed request incurs technical foul and five minutes (possibly time loss). English continuation visually confirms the penalties/initiative, but says a count not exceeding100 fails, whereas Chinese says below100: an exact-threshold translation conflict. XIANGQI-P1 selects100 using Chinese3.2D/8 together with English Art8's100-step verification context, not blanket Chinese precedence. The selected untimed policy omits physical penalties, keeps manual claims and names optional automatic exercise; serialized ordering and claim-audit admission are in the profile. |
| XQ-R3 chase classification | EN19–20 p43–47/35–39; ZH p27–30/26–29. | Check, mating threat, chase, exchange, block, offer and idle differ. Real protection requires legal recapture; apparent pinned protection is insufficient. Persistent pursuer/victim identity, roots, constraints, river and piece exceptions matter.20.5's two-to-one is **counter-chase frequency** from19.18, not “two attacking pieces” suggested by the English wording. |
| XQ-R3 mixtures | EN20.2 enumeration omits alternate mating-threat/chase unlike old AXF General3.2. EN diagram38 p66/58; ZH diagram38 p49/48. | WXF's own diagram38 explicitly permits Black's **alternation** while Red's unilateral perpetual chase offends. Thus this case is supported within WXF; no AXF graft is needed. |
| XQ-R3 simultaneous labels | EN36.4 diagram109 p106/98; ZH p85/84. | A move simultaneously mating-threat and chase is treated with the more consequential chase classification; Red must vary. Distinguish this from the alternating diagram38 family. Full overlapping-classification fixture review remains required. |
| XQ-R4 inability to win | Art3.2A, ENp15/7, ZHp10/9 | The source includes neither side being able to win. It does not supply an executable proof algorithm or justify an engine insufficient-material shortcut. Establish independently reviewed positive/negative fixtures and the precise quantification of this condition. Scope includes required fast sound recognition94 and bounded/terminable exact verification105 in M4. Proven impossible, proven possible for at least one side, and unknown are distinct. G-NOWIN gates quantifier/eligible-win/history-domain interpretation before affected behavior; comparator material tables or a Chess mating-only goal cannot decide it. |

Articles19–20 and the chapter5 diagrams remain normative obligations for a
complete selected profile. Mating-threat resolution, legal roots, pinned and
blocked pieces, multiple threats and causal pursuit need exact definitions
grounded in examples. Research inventory is not a verified algorithm. Native
local legal moves and history-permitted continuation are separate contracts.

## Individual selected interpretations and scoped remaining gates

There is no controlling-language assertion. These are explicit profile decisions
using both texts and WXF context, with predecessor evidence kept separate:

| Discrepancy | Selected interpretation / supporting context |
| --- | --- |
| EN19.8 threefold, ZH19.8 no count,3.1 warning/refusal,3.2B fourfold | Owner XIANGQI-P1: third-occurrence classify/warn; fourth continued unilateral conduct loses, otherwise applicable WXF draw. Official diagram1/2 stopping differences remain faithfully recorded, not silently completed. |
| EN3.2D ambiguous50 moves versus ZH50rounds/100steps |100 total plies, supported by explicit Chinese units and English Art8 verification of100; no pawn reset. |
| EN26 failed claim “not exceeding100” versus ZH “below100” | Threshold is100, reconciling Chinese3.2D/8 and surrounding English Art8 rather than importing a general language preference. |
| EN20.5 two-piece phrasing versus ZH frequency | Use the two-to-one counter-chase frequency defined19.18 and supported by Chinese20.5, not a count of pursuer pieces. |
|20.2 omitted alternate mate-threat/chase listing | WXF's own diagram38 permits alternation; diagram109 simultaneous dual labels resolves to chase. No predecessor clause is silently grafted. |

- **R-EDITION (scoped research):** exact AXF download revisions and authoritative
  CXA2020 text/amendments remain unverified. No claim otherwise. Acquire them
  when relevant to an identified WXF provenance/interpretation question, not as
  a complete-corpus gate on91/92. CXA2026 event adoption of2020 is established.
- **G-CYCLE /94:** classify concrete mixed/overlapping F002 return paths; define
  witness/anchor selection, warning preservation and victim identity versus board
  equality. Third/fourth thresholds are settled. No default result for unresolved
  overlap; resolve/review or obtain owner decision before affected implementation.
- **R-CLASS /94:** instantiate applicable19/20 and chapter5 families; their source
  definitions remain required. This is not an all-family prerequisite for91/92.
- **G-NOWIN /94/105:** define3.2A proof quantifier and eligible native win events
  with full history/claim/warning context before recognizer/exact implementation.
  Older AXF English uses force-a-win language, useful comparison evidence but not
  authority to choose forced-win semantics for WXF. Resignation or clock/foul
  events cannot casually serve as a proof goal, making3.2A vacuous. Bring a
  material residual interpretation to the owner with concrete sources/fixtures.

These are explicit affected-work gates in [coverage](coverage.md), not omitted
rules or claims of executed conformance. MSH-80 can close its planning scope
under the supplied owner decisions; MSH-84/103/45 cannot close without actual
required3.2A implementation105. Source research and first board/kernel work
remain independent of unavailable unrelated alternative corpora.
