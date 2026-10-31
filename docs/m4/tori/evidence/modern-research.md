# MSH-81 modern-source acquisition report

Research worker: tori_modern_sources. Root remains sole durable integrator on codex/rewrite. Retrieved 2026-10-31. No repository/task edits, contacts, purchases, profile adoption, product changes or implementation. Downloaded files are temporary acquisition evidence; public access is not a redistribution licence.

## Findings that advance the prior pass

### Okano 2008: precise access policy, article still unread

[NDL article record](https://ndlsearch.ndl.go.jp/books/R000000004-I9501307) was downloaded successfully (the web reader sometimes returned cache miss). It confirms 岡野 伸, 禽将棋, 大阪商業大学アミューズメント産業研究所紀要 10 (2008-03), pp.241–265, ISSN 1881-1949, NDL 9501307, call Z71-C554 and Kansai storage item 1200800536862. The record describes paper, not a retrieved digital article.

[Publisher contents](https://ouc.daishodai.ac.jp/ams_labo/publication/bulletin/info2004-2008.html#amu10) identifies the title/author in issue 10 but provides no article link. Crucially, the [publisher bulletin landing page](https://ouc.daishodai.ac.jp/ams_labo/publication/bulletin/) states repository publication starts with issue 16, June 2014, and directs acquisition inquiries to the institute. This is an exact acquisition path; it does not prove that issue 10 has never been digitized elsewhere. The general repository and exact catalogue/title search requests returned 503 through curl, and the web tool returned 429/access errors. Exact-title/author searches found no public first-party article file.

**No pages of Okano's article were acquired/read.** Its relationship to any 1828/1833 witness, and its rules/transcription/interpretation, remain unestablished. A secondary blog's assertion that it quotes the historical book cannot establish those facts. Next lawful options: owner-authorized institution request for issue 10/article scan and applicable terms, or owner/library-mediated lawful consultation/copying using the exact NDL identifiers. No inquiry or purchase was initiated. The general publisher sales page concerns paid books/catalogues; do not infer this journal's price or availability from it.

### New OUC institutional historical holding metadata

The collection's actual public keyword form ([search for 禽](https://jmapps.ne.jp/ams_labo/list.html?keywords=%E7%A6%BD&kwd_and_or=and&list_type=LLA001&list_count=50&title_query=yes)) returns three items. Two are books:

| Record | Institutional fields actually read | Limit |
| --- | --- | --- |
| [T010299 / data_id 47770](https://jmapps.ne.jp/ams_labo/det.html?data_id=47770) | 禽象戯圖解; 豊田 四郎兵衛安親（撰）; Japan; 1828; classified 戦術本 | Primary institutional metadata now supports a concrete 1828 holding. Collection field blank; no folios/images; do not equate this copy to Tohoku or claim its contents. |
| [T008141 / data_id 22509](https://jmapps.ne.jp/ams_labo/det.html?data_id=22509) | 禽象戯図解; author blank; Japan; date 不明; ルール; 日本将棋連盟寄贈コレクション | Undated institutional holding; no evidence for an 1833 identification. |

Both record HTMLs contain no-image placeholders rather than source facsimiles. The site says image use, viewing and loans require advance institute procedure/contact, including scheduling for viewing. Its exact repository link is https://ouc.repo.nii.ac.jp/search?page=1&size=20&sort=custom_sort&search_type=2&q=504 (collection catalogues, not an article PDF). No contact was made.

### Hamburg: actual first-party PDF acquired and visually read

[Fabian Krahe author post](https://www.shogihamburg.de/2019/09/06/tori-shogi-%E7%A6%BD%E5%B0%86%E6%A3%8B-spielregeln/) displays 2019-09-06 and update 2020-05-02. Its [linked PDF](https://www.shogihamburg.de/wp-content/uploads/2019/09/Tori-Sh%C3%B4gi-Spielregeln-Sh%C3%B4gi-Hamburg.pdf) was downloaded: 425,899 bytes, two A4 pages. PDF metadata author Fabian Krahe, created/modified 2019-09-08; footer © Fabian Krahe für Shôgi Hamburg 2019. Thus the 2020 web update does not independently date a changed PDF edition. Both complete pages were rendered with pdftoppm and visually inspected; pypdf extracts saved separately. No historical edition is cited.

Bounded rule summary (English interpretation, not an authoritative translation): p1 gives 7×7, 16 pieces/side; captures transfer ownership, drops use empty squares and unpromoted pieces. Swallow drops forbid last rank, a file already containing two swallows, and immediate mate; goose excluded. Promotion covers moves within/into/out of the last two ranks, using **kann** (may/can). Same position repeated **dreimal** obliges the sequence initiator to vary; identity specifies board and hands. Perpetual check is forbidden. Check must be answered; checkmate loses; royal capture wins; illegal move loses; agreed draw forbidden. Notation refers to Japanese/Western shogi. P2 diagrams give all piece movement and setup. Right-quail prose omits backward direction; its diagram supplies it. Pheasant/goose explicitly jump; eagle says up to two backward-diagonal squares without explicit jumping.

The preceding is a named modern author's presentation, not historical ratification or a complete software profile. Root can inspect the acquired pages directly for additional clause analysis.

### JSA narrow corroboration

[JSA 2010-10-26 event report](https://kifulog.shogi.or.jp/ryuou/2010/10/post-902b.html) was acquired as HTML and read. Its swallow caption says 二歩 is permitted but 三歩 prohibited. This directly corroborates a two-versus-three distinction in demonstrated modern practice, but omits full counting/drop scope, sanctions and promoted exceptions. It supplies no promotion, repetition or complete ending code. File/side/unpromoted interpretation needs corroboration rather than attribution to that caption.

## Material interpretation gates for integrator

- **Promotion:** Hamburg's permissive verb is evidence against casually citing it for mandatory entry promotion. It does not expressly resolve mandatory last-rank promotion or an unpromoted last-rank swallow after an ordinary move. Compare an actual different source before framing sourced alternatives; none from Okano can be offered yet.
- **Multiplicity:** distinguish the source's explicit drop context from extending a general at-most-two invariant to ordinary moves/captures. Own-side counting is plausible but not explicit in Hamburg's sentence. The initial diagram and JSA terminology are contextual evidence, not a complete adjudication clause.
- **Cycles:** preserve the wording “same position is repeated three times” rather than silently selecting third occurrence versus initial-plus-three repetitions. Responsibility, side to move, forced cycles, precedence against perpetual check and outcome when no variation exists need definition from stronger evidence or explicit owner conventions.
- **Royal ending:** deciding whether software stops at mate or allows royal capture cannot be settled by citing both formulations without explaining their relationship. No-legal-move without check, entering royals and exceptional draws remain unsupported in this source set.
- **Movement:** dots/arrows corroborate destinations; they do not independently establish eagle two-square leaping, blocking or an intermediate capture action. The explicit jump wording elsewhere strengthens the need to resolve that difference.
- **Record semantics:** generic notation reference supplies neither a canonical Tori record grammar nor durable action/history/repetition identity.

## Useful independent fixture seeds, gated on profile decision

These are restriction/geometry seeds from direct published material, not adopted whole-action expectations: (1) JSA two-versus-three multiplicity; (2) Hamburg p1 empty-square/unpromoted drop; (3) Hamburg p1 swallow last-rank drop rejection and immediate drop-mate restriction; (4) Hamburg p2 goose/pheasant leaps over an intervening piece; (5) Hamburg p2 mirror-distinct quail destinations; (6) Hamburg p2 initial diagram inventory/setup. Do not assert legal whole positions, mandatory promotion, cycle adjudication or terminal results from these isolated clauses. Expected outcomes are publication-derived, not generated by an implementation.

## Audit artifacts

Directory: `/private/tmp/msh81-modern`. Machine-readable source/status/hash ledger: `/private/tmp/msh81-modern/manifest.json`. Original public HTML/PDF saved; rendered PNGs and extracted text marked derivatives. Report covers only this worker's source path. Root's independent modern and historical investigations may add comparison evidence. No rules were adopted and MSH-81 cannot be completed from this source set alone.
