# MSH-83 bounded Tenjiku source acquisition handoff

Research date: 2026-10-27 UTC. Researcher: `msh83_research`; root@codex remains sole integrator. Repository context read at `14b9643dac70f09394ccc60aea8b0f8c5bbd5b03`. This is independent research only, with no profile adoption, product implementation, task completion or change to the provisional Xiangqi → Tori → Chu → Tenjiku delivery order. It supplies partial evidence for MSH-83 AC1 and concrete choices for AC2; AC1–4 remain unfinished. No institution was contacted.

## Acquisition result and source identity

**The missing historical witness is now identified more precisely, but its Tenjiku text is still not acquired.** A catalogue record is not a transcription, and Appendix A's translated page numbers are not verified historical folios.

| ID | Source class and reproducible locator | What was actually obtained |
| --- | --- | --- |
| H1696 | National Archives of Japan, [諸象戯図式, file 3144172](https://www.digital.archives.go.jp/file/3144172). Cabinet Library → Japanese books → Japanese books excluding Tamon-yagura documents. Call number **199-0411**, author 西沢貞陣, printed Genroku 9 (**1696**), **4 volumes**, former owner 昌平坂学問所. | The institution's current catalogue was read in the browser, including “all information.” It gives public access and CC0 **metadata**, Japanese language, record level `file`; it exposes no image viewer or subordinate item list in this session. Full relevant volume, pagination and colophon remain unacquired. The web-fetch tool's 403 did not mean the browser catalogue was unavailable. |
| E1/E2 | National Archives [exhibition entry](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/popup_jousetsu_23_5_0002.html), [cover image](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/23_5_0002_1.jpg), [open-spread image](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/23_5_0002_2.jpg). | Both JPEGs fetched and visually inspected. E1 is a cover. E2 shows the **小象戯** spread, including diagrams with/without the elephant; it is **not Tenjiku movement/burn evidence**. Neither supplies a complete volume or a reliable Tenjiku folio locator. Checksums in JSON. |
| H1694 | Proposed 1694 edition of 諸象戯図式. [JCSA historical bibliography](https://www.chushogi-renmei.com/koramu/koramu-shiryou.htm) mentions Genroku 7 alongside the 1696 work and refers to Masukawa's *Shogi I*. | Discovery/bibliographic evidence only. No institution-held **1694 copy**, complete facsimile, colophon or copy-specific identifier was verified. The present 1696 catalogue must not be relabelled 1694. Failure in this bounded pass is not proof that no such copy exists. |
| K | [NDL cooperative reference 1000247371](https://crd.ndl.go.jp/reference/entry/index.php?id=1000247371&page=ref_view), supplied by Doshisha University Imadegawa Library, updated 2022-05-13. Kyoto Library and Archives, 象戯図式, **和/780/64**, material **119040343**, catalogue bibid **305666**. | Institutional reference record, not facsimile. It reports an acquisition annotation dated **1903-09-25**, not a verified date of composition or copying. Do not repeat the derivative “1909 manuscript” as established. The similarly titled material **1119040370** is a different lead. No link from this copy to Banaschak's particular source was established. |
| A1999 | Colin Adams, *The Struggle for Survival: Testing Times for Tenjiku Shogi*, [public PDF mirror](http://history.chess.free.fr/papers/Tenjiku-Colin%20Adams.pdf). | Full PDF acquired, **185 pages**, second edition **1999-02-14**, visually verified on PDF p.2. Authorial reconstruction/translation evidence, hosted by a third-party mirror; current author-hosted copy not acquired. SHA256 `a189a4a83f6e9554c4e8443241a5ab7307d6937f46b81f6c5ae35dc5063aef91`. |
| M | H.G. Muller, [Tenjiku Shogi, the wild Chess game](https://hgm.nubati.net/tenjiku/tenjiku.html). | First-party modern convention explanation, HTML fetched. No publication version is stated; content hash/date pin the inspected snapshot. This page does not prove any particular engine binary's behavior, association adoption or historical rules. |

## What the retrieved reconstruction actually establishes

A1999 Appendix A, printed **166–168** = PDF **177–179**, attributes its translation to Peter Banaschak but identifies an **undated Shōgi zushiki**, without repository/copy identity. Its “first/third/fourth/fifth page” references cannot identify H1696 folios. Footnotes qualify jump-capture, *odoru*, diagram and glyph interpretations. Section A.2 gives a further acquisition lead through Maruo Manabu's group and *Kiapitan*, without a specific issue.

A1999 main §3.1, printed12/PDF23, diagrams vertical fire-demon ranging; Appendix A printed167/PDF178 instead describes water-buffalo orientation. Main §1.2 rule4, printed4/PDF15, burns bystanders before the moving demon dies beside an enemy demon. Promotion does not burn immediately. Section3.6, printed17–18/PDF28–29, distinguishes TSA area-mover lion hawk from Adams's proposed lion-plus-bishop. Main printed4–5/PDF15–16 requires capture of king and prince, permits royal exposure, forbids recurrence, and distinguishes proposed tournament draw rules. These are authorial conventions, not newly verified Edo clauses.

Read-only PDF inspection used the PDF skill and bundled Poppler; direct `pypdf` text extraction produced character-encoding garbage. Complete relevant pages were rendered and inspected instead. Inspected PDF pages: 2,15–16,23–25,28–29,177–179. No full book or translation is copied into the repository/handoff.

## Software-convention comparison

M's piece table uses vertical/diagonal fire-demon rays. Its burn footnote (#) requires the moving demon to survive and excludes same-turn promotion; this conflicts with A1999's dying-demon bystander effect. M uses lion-plus-bishop, three-step area movement stopping at capture, ranked jumping and a special tetrarch move. Its prose forbids jumping equal/higher-ranked blockers, but footnote (*) mentions only higher rank; clarify this before differential testing. No immutable software release was audited.

## Required decision/reconciliation ledger

The following are **questions to resolve**, not an adopted table of answers. The retrieved sources identify meaningful alternatives; missing cells remain named obligations.

| Obligation | Concrete next evidence and owner choice |
| --- | --- |
| Fire-demon orientation | Obtain H1696's actual Tenjiku movement spread and compare the original glyphs/diagrams with A1999 §3.1 and Appendix A. Select vertical rays, horizontal rays, or an explicitly named alternative only after reconciliation. |
| Burn timing and mutual demons | Compare A1999 §1.2 rule4 against M footnote (#), then inspect the historical passage. Explicitly decide burn-before-death versus death-suppresses-burn; stationary versus moving demon immunity; whether simultaneous effects depend on enumeration order. |
| Promotion into fire demon | Independently inspect promotion and burn passages. Decide turn sequence, whether promotion can save a piece beside an enemy demon, whether effects occur on intermediate squares and whether a captured/burned piece may still promote. Agreement between modern sources does not settle historical silence. |
| Generals and royalty | Obtain full historical ranking/jump text, if any; inspect whether restrictions attach to crossed blockers, destination victims or both. Decide equal-rank blockers, jumping to higher-ranked victims, noncapturing jumps, king/prince exceptions and whether every royal has the same rank. A bare statement that a piece “flies over” cannot supply these distinctions. |
| Lion hawk | Compare actual source text with A1999 §3.6's named alternatives; decide whether it has lion double capture, igui, return/pass and jump powers or an area move stopping at capture. “Modern” and “traditional” are Adams's labels, not globally unique profile IDs. |
| Free eagle / heavenly tetrarch | Obtain original diagrams and explanatory text, compare Appendix A's uncertainty against M's exact move description. Resolve turning, intermediate occupancy, stationary capture, adjacent landing restrictions, skipped square and sideways-distance details. |
| Multi-stage actions | Define legal path prefixes and completed choices, route equivalence, revisits/pass, capture stopping, intermediate promotion-zone contact, and when burns/royal loss are evaluated. Independently reviewed source cases must support any claimed answer. |
| Promotion generally | Inspect original setup/promotion tables and any general clauses; resolve zone size, eligibility, deferred rights after refusal, compulsory promotion/dead pieces and each promotion identity. Do not silently import JCSA Chu or standard-shogi policy. |
| Royals, endings and history | Locate actual Tenjiku text beyond movement/setup. Explicitly decide capture-all-royals, check/exposure, simultaneous final royal destruction, bare-royal cases, stalemate, repetition identity and prohibition/outcome/claim rules. Do not label modern no-capture proposals as historical rules. |
| Notation / consumer contract | After a complete named profile exists, specify a complete action's ordered route and promotion choice, derived ordered effects, all royal identities, necessary historical rights and terminal outcomes. Coordinate notation must cover 16×16; a start/end-only notation cannot silently erase distinct actions. |

## Fixture expectations and limits

**No historical-conformance fixture is ready to adopt.** No Tenjiku folio from an identified historical copy was inspected. The Banaschak translation is a useful independent interpretation lead, not a verified facsimile substitute. Avoid converting Ludii's summary or legacy mShogi/software output into expected results.

Two useful **conditional comparison seeds**, with sparse coordinates `(file,rank)` and all royals elsewhere, can be prepared after selecting a modern convention:

1. A friendly fire demon moves to `(8,8)`, beside an enemy fire demon `(9,8)` and enemy pawn `(8,9)`. This isolates the ordering dispute above: record the pawn's survival explicitly under each candidate. Add a friendly bystander to ensure captures are side-specific. This is a modern-source comparison, not a historical expected result.
2. A water buffalo promotes at `(8,8)` beside an enemy pawn `(8,9)`, with no enemy demon nearby. Check the selected convention's same-turn burn policy, then a later demon move. Keep promotion-zone geometry explicit in the eventual full fixture.

These are research test designs, not ready conformance fixtures: legal pre-position/action details and independently reviewed expectations still need recording. Once H1696 is obtained, lower-ambiguity setup/promotion observations should become the first historical cases; reserve contested ordering cases for a recorded owner decision.

## Recommended owner options and concrete unblock steps

1. **Historical reconciliation first (recommended for the promised sourced profile):** acquire the complete relevant first volume of **199-0411**, including cover, title/preface, contents, Tenjiku setup, promotion and movement spreads, explanatory clauses and colophon. Preserve image numbers and recto/verso locators, lawful hashes and copy identity. Have an independent reader check transcription and translation. Obtain the Kyoto copy independently and establish whether it belongs to the translated textual family. Investigate the 1694 witness through edition bibliographies without conflating a preface date with a publication date. Acquisition requests/contact are a future owner-authorized step, not performed here.
2. **Named modern reconstruction:** explicitly select Adams's second-edition traditional/modern alternative as a reconstruction, or a fully specified Muller convention after clarifying its gaps. Record each consequential repair rather than claiming historical agreement. This requires an owner product/rules decision and a complete supplemental policy for clauses not settled by the selected source. It does not satisfy a requirement to acquire/reconcile the historical text by itself.

Recommendation: retain the historical acquisition/translation obligations and use modern sources now to make the dispute ledger concrete. Do not select semantics merely for engine availability or presumed balance. Continue unaffected MSH-80 Xiangqi work; nothing here demonstrates a reason to reorder delivery or add an MSH-83 dependency to Xiangqi.

## Verification and reproducibility

- `backlog instructions overview`, read-only `backlog task view MSH-83 --plain`, AGENTS.md, relevant design principles and later-game inventory read. No Backlog mutations.
- Institutional site search for `諸象戯図式` returned the single 199-0411 record; detail and “all information” views inspected. No full-volume viewer was exposed. This observation is narrower than “not digitized anywhere.”
- Bounded public web searches covered Japanese/English title variants, National Archives, NIJL, NDL/CiNii and Harvard leads. No additional complete 1694/1696 Tenjiku facsimile was verified. Derivative sites remained discovery aids.
- Public PDF retrieval: `curl -L --max-time 30 -f -o <temporary-pdf> 'http://history.chess.free.fr/papers/Tenjiku-Colin%20Adams.pdf'`, succeeded with 1,000,783 bytes. Web-fetch 502 was not a persistent availability conclusion.
- Rendering: `pdftoppm -f <first> -l <last> -scale-to 1600 -png <temporary-pdf> <temporary-prefix>` (edition/appendix also inspected at 1300/1700). The bundled runtime version was 26.1017.22136. This was source inspection, not software rules verification, platform verification or a test pass.
- SHA256s and retrieval statuses are in `tenjiku-provenance.json`. Only the requested Markdown/JSON handoff is retained; downloaded source/rendering intermediates were removed. Re-fetch public URLs to reproduce. No repository/task/code edits or commits were made by this worker.
