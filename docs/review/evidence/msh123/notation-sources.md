# MSH-123 independent notation research

Read-only research by `/root/notation_sources`, 2026-10-31. No source/task edits or comparator-output expectations. These fixtures are independently reasoned display facts; native admission must establish legality. Sources were consulted directly; WXF network retrieval timed out twice, so the already-acquired, hash-verified official PDFs were visually read.

## Sources and provenance

- PGN primary specification §8.2.3.1–5: https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm . English SAN construction, legal-move disambiguation, promotion and check/mate.
- JSA official recording guide, Basic matters and all ambiguity diagrams: https://www.shogi.or.jp/faq/kihuhyouki/ . It selects numeric file/rank, distinguishes publishing rank-kanji, uses preceding opponent destination for 同, contextual 打, promotion/nonpromotion, movement-first ambiguity, and explicitly forbids 直 for 竜/馬.
- WXF2018 official English Article7.1–7.5, printed15–17/PDF23–25: https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf . Cached `/private/tmp/msh80-research/xiangqi/wxf2018-en.pdf`, SHA256 `2b45534953bbfcca42dc2c52e06be48ff44ebe3bf7237248b12e39576c494cf3`; inspected rendered `/private/tmp/msh-ux-wxf-notation-023.png` through025.
- Corresponding Chinese Article7, PDF14–16/printed13–15: https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_Xiangqi_Rules_Chinese_2018.pdf . Cached SHA256 `f61f5c82ddaf2abb45eb25c24d58f2b278d2c150439f391d413c32c4ce275b3e`; independently rendered to `/private/tmp/msh123-wxfzh-*`. Same relevant specification, not a conflicting convention. Poppler emitted existing font/JP2 warnings but rendered readable pages. Text extraction misses image-based article content.

## Chess independently expected positions

| Native input/context | Expected SAN | Reason |
| --- | --- | --- |
| Initial e2e4 | e4 | Pawn quiet |
| Initial sequence e2e4,d7d5,e4d5 | exd5 | Pawn capture file |
| `7k/8/8/3pP3/8/8/8/7K w - d6 0 1`, e5d6 | exd6 | En passant has no separate suffix |
| `r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1`, e1g1/e1c1 | O-O / O-O-O | Letter O |
| `7k/P7/8/8/8/8/8/7K w - - 0 1`, a7a8n/a7a8r | a8=N / a8=R+ | Underpromotion and post-state check |
| `4k3/8/8/8/1b6/2N5/8/4K1N1 w - - 0 1`, g1e2 | Ne2 | c3 knight attacks e2 but pinned by b4 bishop; source's own legal-alternative example |
| Same position without b4 bishop, g1e2/c3e2 | Nge2 / Nce2 | Both now legal |
| Initial f2f3,e7e5,g2g4,d8h4 | Qh4# | Actual checkmate, not heuristic |

## Shogi independently expected geometry

Use fullwidth numeric coordinates for display, native coordinates remain USI. Sente moves toward rank a; Gote perspective rotates left/right and forward/backward. Omit actor triangles because separate native side columns identify actor.

Direct source-illustrated facts, with native coordinates supplied here: Sente Gold9c→8b versus Gold7b→8b distinguish advancing versus lateral motion; Gold9b/7b→8a distinguish left/right without redundant advance; Gold8i/7i→7h uses left/straight. Three Gold6c/5c/4c→5b uses left/straight/right. Goldlike promoted kinds remain their own named piece kinds, not merged with Gold.

Five promoted Pawns at7i,8i,9i,9h,8g targeting8h independently yield right, straight, left+advance, lateral, retreat respectively. Movement alone is first preferred where sufficient; otherwise side alone where sufficient; otherwise side+movement. Crucially, two Dragons9i/8i→8h use left/right, never straight; two Horses9a/8a→8b likewise left/right, never straight. The JSA Horse-D paragraph has an apparent piece-name typo in the second origin example; the section heading/diagram and first example establish Horses, so do not codify a Dragon moving bishopwise.

Basic independent expected strings: start7g7f=`７６歩`; known previous opponent destination8h followed2b8h+=`同角成`; unknown imported earlier history cannot produce 同. Pawn1b1a+=`１１歩成` even compulsory. Silver5d5c may select 成 or 不成 when optional. A hand Gold drop to5h requires 打 only when a board Gold can also legally reach5h; otherwise omit it. Do not suffix a board move with 打. Use actual Japanese promoted identities (と,成香,成桂,成銀,馬,竜), not inferred Latin abbreviations.

## Xiangqi independent fixture strategy

Mover files: Red a→i corresponds9→1; Black a→i corresponds1→9. Advance is Red increasing rank, Black decreasing rank. Horizontal suffix is destination file; vertical K/R/C/P suffix is distance; diagonal H/E/A suffix is destination file. Do not distinguish capture with another symbol or append chess check suffixes.

Initial g0e2=`E3+5`; b0c2=`H8+7`; h2e2=`C2=5`. Initial Black b9c7=`H2+3`, h7e7=`C8=5`. A Red rook e4e1=`R5-3`; Black rook e6e8=`R5-2` in independently admitted diagrams. Two same-file Red cannons e3/e6: front e6 horizontally to d6=`+C=6`; rear e3 to d3=`-C=6`. Reverse front/rear rank ordering for Black.

Source FigureB's Red stack is d8,d6,d5 (Red file6): d5e5=`36=5`; d6e6=`26=5`; d8e8 capturing Advisor=`16=5`. Preserve exact figure blockers/native kings when transcribing: Black General f9, Advisors d9/e8, Elephants c9/e7; Red General e0, Advisor d0, Elephant c0; Black rook a2. The English page's three examples plus Chinese diagram independently confirm these strings. No legal full-history reachability is inferred from the diagram.

### Rare-stack gap and smallest consistent completion

Both selected language editions explicitly state two same-file pieces use front/rear and three same-file soldiers use ordinal+file. Neither inspected Article7 specifies four/five or simultaneous paired files. Do not describe a comparator or older AXF alphabetic-pawn proposal as source authority here.

Recommended bounded display completion, clearly documented as inference: extend the illustrated ordinal+file form to four/five soldiers; ordinal counts front→rear within its file. For simultaneous paired files, use ordinal+file for each affected stack (or at minimum when ordinary front/rear strings collide). This retains the source's four-component language and unambiguous origin without changing rules or canonical identity. Example Red c6/c5 and g6/g5: c6c7=`17+1`, c5d5=`27=6`, g6g7=`13+1`, g5f5=`23=4`; plain front `+P+1` would collide for c6c7/g6g7. Three+two split: c8/c7/c6 and g8/g7 → first group's1/2/3 with file7 and second group's1/2 with file3. Four/five on d-file: front ordinal1 through rear4/5 with file6; e.g rear d4e4 in five-stack d8..d4=`56=5`. Mirror Black to test ordinal direction and file perspective.

This is a display normalization inference, not a material rules-profile change. Integrator may choose it autonomously if task permits explicit bounded completion. Any stricter claim of literal WXF text covering all rare cases would be unsupported. Native opaque action/origin and exact target prefix remain authoritative even if a rare display form needs future editorial adjustment.

## Fixture correction and subsequent owner decision

Integrator identified a concrete legality error in the candidate five-stack example above: Red d4 is not across the river and cannot traverse to e4. Use Red d9,d8,d7,d6,d5 and rear d5e5=`56=5`; Black mirror f0..f4 and f4e4=`56=5`. Preserve native legality assertions, never adjust rules to the candidate fixture. The integrator reports that the owner explicitly approved ordinal+file for four/five and simultaneous stacked files during MSH-123; the repository task records that approval. This supersedes the note's recommendation-only status, without turning it into a literal quotation of WXF.
