# MSH-82 bounded source research — 2026-10-27

Research only, prepared independently of product implementation at repository revision `14b9643`, primary checkout `codex/rewrite`. No profile is adopted, no acceptance criterion is declared complete, and no source gap blocks MSH-80. Root is the durable integrator. Backlog 1.55.0 was used only for reads; recorded project baseline is 1.55.1. Existing dirty task records were preserved.

## Evidence acquired and its authority

| ID | Exact source and locator | What was actually inspected |
| --- | --- | --- |
| C1 | JCSA, [中将棋連盟競技規則・総則編（2019年度版）](https://www.chushogi-renmei.com/kouza/rule2007.htm), corrected 2019-09-14, I–IX | Entire Japanese HTML text. URL year is not edition. Association rules, not proof of historical practice. |
| C2 | JCSA, [中将棋対局規定 補足編](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm), updated 2004-06-06, §§1–6 | Entire Japanese HTML text, including all four multi-lion case explanations. Associated diagrams have exact source URLs but were not all visually transcribed. |
| C3 | JCSA, [駒の動き方](https://www.chushogi-renmei.com/kouza/koma.htm), revised 2004-04-22 | Entire Japanese HTML, initial diagram viewed in browser. Detailed movement image URLs located. This linked companion has its own older date. |
| H1 | 国書刊行会編, *雑芸叢書 第一*, 国書刊行会, 1915, NDL 790.8-Z14-K, DOI [10.11501/1869566](https://doi.org/10.11501/1869566); [official item record](https://ndlsearch.ndl.go.jp/books/R100000039-I1869566) | Actual scan displayed and read at [image121, pp212–213](https://dl.ndl.go.jp/pid/1869566/1/121), [image122, pp214–215](https://dl.ndl.go.jp/pid/1869566/1/122), [image123, pp216–217](https://dl.ndl.go.jp/pid/1869566/1/123). NDL identifies this as Internet-public, protection period expired. |
| T1 | JCSA, [中将棋の指し方 七段 岡崎史明](https://www.chushogi-renmei.com/kouza/Okazaki.htm), web update 2001-02-14 | Full text read. JCSA expressly describes an original-text transcription with most diagrams omitted. A transcription witness, not a verified facsimile of an identified booklet edition. |
| B1 | JCSA, [古典将棋に関する文献一覧](https://www.chushogi-renmei.com/koramu/koramu-shiryou.htm), updated 2005-09-03 | Bibliographic leads only: *中将棊指南抄* 1703; *中将棊絹篩* 1818; Matsuda Shōichi *中将棋全集* 1977, eleven volumes with a reported possible twelfth. These dates/holdings claims are not direct inspection of those originals. |

H1 is a **1915 typeset witness of an older text**, with an internal Genroku16 colophon visible at p213. It is not a scan of the original 1703 printing. The 1979 reproduction is a different publication: [NDL record](https://ndlsearch.ndl.go.jp/books/R100000002-I000001437192). A [Miyagi Library reference response](https://crd.ndl.go.jp/reference/entry/index.php?id=1000213532&page=ref_view) located pp212–217 for rules; the page numbers and contents were then verified directly in H1. No English translation or software was used as rules authority.

## C1 working clause digest and fixture expectations

The following is a compressed first-pass translation, not an adopted executable contract.

- I: 12×12, 46/side; captures discarded.
- II.1–2/III: king mate/capture wins unless prince remains; royalty classification changes with survivors.
- II.4–6/IX: material exhaustion has special outcomes; exclude dead pawns/lances. Immediate recapture qualifies the apparent material-win threshold; other exhaustion draws require agreement.
- IV.1–5: entry into four-rank enemy zone permits promotion; after decline, re-entry or capture within/from zone renews eligibility. Pawn last-rank rescue; unpromoted pawn/lance can become dead. Lance rescue is correspondence-only.
- VI.1a–d: before fourth matching position/sequence, responsible side must vary: continuous checker; sole attacker; otherwise side to move; first stationary mover.
- VII.1–5: adjacent lion capture allowed; distant defended lion barred, including revealed sliding defense. Non-lion capture grants defended own lion immediate protection; capturing-and-promoting kirin qualifies. Extra capture excludes pawn/go-between.
- VIII: quiet return requires an empty reachable intermediate square; surrounded initial lion cannot pass.
- V/VI.3: physical commitment/foul recovery includes opponent discretion.

[C1, cited sections](https://www.chushogi-renmei.com/kouza/rule2007.htm). These are conditional fixture seeds; full positions/history and independent translation review remain required.

## C2 reconciliation and directly expected lion seeds

§1 retains pawn rescue, removes analogous go-between treatment, and limits proposed lance rescue to correspondence. §2 case1: `▲33 Lion ×22 Gold ×12 Lion`, then `▽21 Gold ×12 Lion` is allowed despite protection. Case2: `▲33 Lion ×22 Lion ×12 Lion` is allowed despite both defenders. Case3: after `▲14 Rook ×11 Lion`, `▽32 Lion ×43 Silver ×54 Lion` is allowed: extra capture outranks immediate protection; ensuing lion capture is allowed. Case4 rejects taking a sole defending go-between and then its distant lion: evaluate one complete action, not newly adjacent intermediate subturns. The companion pawn illustration also rejects that adjacency argument. Exact full-board reconstruction requires the linked diagrams.

§3 says JCSA moved from Okazaki interpretation toward *全集/指南抄*, with extra-capture priority its own addition; it acknowledges potentially different historical go-between treatment. §4 rejects quiet return through occupied/offboard intermediate squares. Capture-return remains distinct. §5 explains why JCSA did not adopt a historical twenty-move announcement condition. §6 adds competition handling.

[C2, §§1–6](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm). No direct contradiction with C1 was established here; C2 supplies exception precedence that C1 alone leaves terse. This does not prove that every unmentioned 2004 detail remained adopted in 2019.

## Companion movement/setup observations

C3 lists all promotion relationships; lion, king/prince and free king have no further promotion. An already-promoted identity must not be promoted again merely because that movement type also occurs initially. Its lion explanation permits up to two king steps, a jump to the outer ring, and capture-return. Soaring eagle restricts lion-like directions to forward diagonals; horned falcon restricts them to forward. C3 explicitly allows jumping a friendly intermediate piece on these two-step forward directions. Quiet return restrictions must be read with C1 VIII/C2 §4. [C3, 大走り and 獅子 paragraphs](https://www.chushogi-renmei.com/kouza/koma.htm).

Initial setup image: [rule/game1.jpg](https://www.chushogi-renmei.com/kouza/rule/game1.jpg). It agrees visually with H1 p214 on the central asymmetric pairs. A machine manifest has not been independently checked. The complete setup transcription below is grounded additionally in H1 p214, which was inspected at readable scale.

## H1 direct historical text observations

H1 p213 contains the opening rules, promotion/decline passage, lion law, and twenty-move passage cited by C2. Pp214–215 show setup and promoted arrangement; pp216–217 provide movement diagrams and explanations. This establishes a direct historical-text witness for comparison, rather than relying only on C2's account.

First-pass reading of p217's prince paragraph: prince moves like king; when a prince exists the king may be captured, with the prince then treated as king. This directly supports continued play after loss of one royal, but does **not** itself define all self-exposure or mate cases. The p213 twenty-move passage concerns overlooking a running-piece attack and capture victory; no executable twenty-move rule is proposed from it. The promotion wording and several archaic lion expressions need independent Japanese review before precise historical-profile claims.

Setup seed, H1 p214: columns below run **file12 to file1**, rows named by the modern board's ranks as a convenience; Black moves toward decreasing ranks. The historical coordinate system instead uses zodiac files, shown on p212. Do not silently call this transcription historical notation.

```
rank12: L  FL C  S  G  K  DE G  S  C  FL L
rank11: RC .  B  .  BT Ki Ph BT .  B  .  RC
rank10: SM VM R  DH DK Ln FK DK DH R  VM SM
rank 9: P  P  P  P  P  P  P  P  P  P  P  P
rank 8: .  .  .  GB .  .  .  .  GB .  .  .
rank 7: empty
rank 6: empty
rank 5: opposing GB at files9 and4
rank 4..1: opposite side rotated 180 degrees
```

Count independently: 12+8+12+12+2=46/side. Abbreviations: L lance, FL ferocious leopard, C copper, S silver, G gold, K king, DE drunk elephant, RC reverse chariot, B bishop, BT blind tiger, Ki kirin, Ph phoenix, SM side mover, VM vertical mover, R rook, DH dragon horse, DK dragon king, Ln lion, FK free king, P pawn, GB go-between. P214 shows Black king file7 and elephant file6; p214 shows lion file7 and free king file6 on rank10.

Movement seed inventory transcribed from H1 pp216–217 (forward is owner-relative; `step` means one square; `slide` means an unobstructed ray; these shorthand semantics are the researcher's translation). These are independent-of-code expectations pending diagram cross-check, not an implemented profile:

| Identity | Expected movement | Promotion if initially unpromoted |
| --- | --- | --- |
| Pawn | forward step | Gold |
| Go-between | forward/backward step | Drunk elephant |
| Lance | forward slide | White horse |
| Reverse chariot | vertical slide | Whale |
| Copper | forward three + backward step | Side mover |
| Silver | forward three + backward diagonals step | Vertical mover |
| Gold | orthogonal + forward diagonal step | Rook |
| Ferocious leopard | forward/backward three step | Bishop |
| Blind tiger | seven adjacent except forward | Flying stag |
| Drunk elephant | seven adjacent except backward | Prince |
| King/prince | eight adjacent | none |
| Kirin | diagonal step; orthogonal jump two | Lion |
| Phoenix | orthogonal step; diagonal jump two | Free king |
| Side mover | horizontal slide; vertical step | Free boar |
| Vertical mover | vertical slide; horizontal step | Flying ox |
| Rook | orthogonal slide | Dragon king |
| Bishop | diagonal slide | Dragon horse |
| Dragon king | rook + diagonal step | Soaring eagle |
| Dragon horse | bishop + orthogonal step | Horned falcon |
| Flying stag | vertical slide + remaining tiger steps | none |
| White horse | forward three/backward vertical slide | none |
| Whale | backward three/forward vertical slide | none |
| Flying ox | diagonal + vertical slide | none |
| Free boar | diagonal + horizontal slide | none |
| Free king | eight rays | none |
| Lion | up to two king steps; outer jump; capture-return | none |
| Soaring eagle | six rays excluding forward diagonals; restricted two-step/jump/capture-return there | none |
| Horned falcon | seven rays excluding forward; restricted two-step/jump/capture-return there | none |

H1 p217's pawn text separately describes last-rank rescue; its prince text separately describes royal succession. Exact promotion conditions and special lion capture law must not be replaced with the above movement summary. H1 p216 includes prose explanatory additions whose typography/parenthetical treatment should be checked against the underlying original printing before attributing every explanatory word to 1703.

## Material decisions/gaps for the owning task

These are proposals/questions, not owner decisions. Recommend discussing an explicitly **JCSA association** profile using C1 with specifically enumerated compatible C2/C3 clauses. Keep any Okazaki or historical reconstruction separate. Do not call it universal historical Chu.

1. **Acquisition/translation:** retain H1's exact reprint identity. Acquire the actual identified *指南抄* original/copy and *全集* rule-volume pages before asserting those books agree. *絹篩* originals and Okazaki booklet edition remain unacquired. T1 is useful but unverified transcription. Have a separate reader check Japanese translation and fixture diagrams.
2. **Royal safety / no-action:** C1 II and H1 p217 do not completely specify whether a sole royal may deliberately enter attack, ignore attack, remain attacked, or how a last royal trapped without an attack ends. A chess self-check filter and a capture-only generator produce different legal sets; neither is justified merely by absence of announcement. Ask for a sourced explicit ruling or ratified adaptation. Test one/two royals, loss of each, simultaneous threats, last-royal mate and capture separately.
3. **Material termination:** formalize C1 II.4 versus IX before implementing an automatic predicate. The immediate-recapture example rules out a naive piece-count trigger. Unclear coverage includes side-to-move dependence, other forced exchanges, multiple princes, promoted GB and dead pieces. Obtain ruling or enumerate and ratify a sound bounded adaptation; generic chess insufficient-material logic is unsuitable.
4. **Repetition:** the Japanese identity phrase includes placement and move sequence; define cycle matching, start/length, transient lion rights, promotion eligibility, and responsibility when categories overlap. Decide whether prevented/rejected fourth recurrence or a committed foul/adjudication models the competition rule. No automatic neutral draw should be inferred. Board-only keys cannot encode every history question.
5. **Complete-action timing:** C2 case4 provides firm anti-subturn evidence, but full promotion timing through enter/exit/return paths, defended-lion assessment with removed blockers, multi-lion protection target(s), and a kirin becoming lion still need explicit full-board fixtures. Do not project a single-lion assumption into multi-lion cases.
6. **Physical rules/software:** recommend atomic invalid-input rejection for product actions, with a clearly named adaptation for touch-move/foul recovery and agreement commands. This proposal is not source adoption. Correspondence lance rescue versus ordinary play is a real profile choice; do not apply it merely because play is software-mediated.
7. **Notation/consumer:** source uses ordered multi-captures and historical zodiac coordinates. Recommend records preserve source/target/intermediate route, promotion decision, original piece identity, necessary history and versioned profile; effects remain rules-derived. Adapter notation is not selected here. Native/shared extension work stays MSH-86/MSH-78; no speculative implementation was performed.

Useful next verification: independent reconstruction of C2's five images, C1 IX images, and C3 movement diagrams against H1; then a bounded profile discussion presenting only unresolved material choices. This pass supplies acquisition evidence and supported seeds. It does not finish MSH-82 AC1–4, approve a profile, or satisfy independent profile/fixture review.

Acquisition evidence is in `chu-provenance.json`: 32 public GETs returned HTTP200, including exact C1/C2/C3 pages, transcription, setup, linked diagrams, official IIIF manifest and three historical images. It records SHA256 of original response bytes, content types, byte lengths and UTC retrieval times. Bytes were read in memory and not retained; a successful GET is not a claim that each image was visually interpreted. Browser screenshots were used to read H1 and C3 setup. Repository tests were not run because no implementation changed. No institution was contacted.
