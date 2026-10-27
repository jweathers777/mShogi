# Later-game source and unresolved-rules inventory

Planning snapshot: 2026-10-26, MSH-62. This is a bounded research inventory,
not an adopted rules profile or implementation/conformance verdict. Standard
shogi remains first. The provisional later delivery order is **xiangqi, tori
shogi, chu shogi, tenjiku shogi**. Independent source acquisition can proceed
without serializing those deliveries. The findings below do not justify changing
that order now; a later change should record the actual blocker, source evidence,
shared-boundary impact and proposed order in the owning task. Difficulty obtaining
a later game's historical text does not block unaffected standard-shogi work.

**Current research addendum,2026-10-27:** preserve the original snapshot below.
MSH-80 acquired both official WXF2018 languages and the linked AXF PDFs;
[the current reconciliation](xiangqi/sources.md) supersedes the old Chinese
timeout as an acquisition status. It still has named AXF/CXA and interpretation
gaps and no selected Xiangqi profile. Independent research adds exact Tori
holdings/article identifiers in [MSH-81](research/tori.md), a directly inspected
1915 historical-text witness and current JCSA code in [MSH-82](research/chu.md),
and separates the Tenjiku catalogue/translation witnesses in
[MSH-83](research/tenjiku.md). None is a completed profile or a new dependency
between games. Provisional delivery order is unchanged.

## Authority and evidence policy

Keep four categories separate in every future profile and fixture manifest:

1. **Association rules:** a named organization, edition, language, amendments and
   scope of adoption. An association's rules are evidence of its own practice;
   they do not automatically govern every historical or modern interpretation.
2. **Historical primary evidence:** the actual treatise/manuscript or a faithful
   digital facsimile, identified by repository, edition/copy, volume and folio or
   image number. A catalogue establishes holdings, not the rule's content. A
   translation must identify its source copy and distinguish translator inference.
3. **Proposed reconstructions/revisions:** named authors' interpretations, proposed
   repairs or balance changes. These can be considered explicitly; modern
   plausibility is not proof that an original text says the same thing.
4. **Software conventions:** a pinned program's behavior, notation or protocol.
   These can support differential comparison only after profile compatibility is
   audited. Neither majority implementation behavior nor legacy mShogi is a rules
   authority.

Secondary indexes, Wikipedia and derivative rules summaries are discovery aids.
No source below silently promotes their claims to primary evidence. Retrieval
status describes this bounded pass, not permanent availability. Before adopting a
profile, preserve lawful reproducible locators/checksums, an interpretation log,
independently reviewed translations/expected cases and a coverage ledger. Do not
copy complete copyrighted rulebooks into the repository.

## Xiangqi — MSH-45

### Located authorities and editions

| Reference | Classification and locator | Evidence/status |
| --- | --- | --- |
| X1 | WXF, *World XiangQi Rules*, 2018, [official English PDF](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf); [official Chinese PDF](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_Xiangqi_Rules_Chinese_2018.pdf). | Association text. English PDF retrieved and selected pages visually inspected; Chinese fetch timed out. Printed pagination differs from PDF page numbering. |
| X2 | WXF, [publication introduction](https://www.wxf-xiangqi.org/index.php?Itemid=291&catid=133&id=269%3Aworld-xiangqi-rules-introductionen&lang=en&layout=default&option=com_content&view=article). | Official provenance: WXF describes its text as developed from the fourth AXF revision. This does not establish identity with a CXA domestic edition. |
| X3 | AXF, [rules summary](https://www.asianxiangqi.org/axf_rules.htm), sections 1–4; linked [Chinese detailed diagrams PDF](https://www.asianxiangqi.org/AXF%20Rules%20of%20XQ%20Competition.pdf); [English chapter PDF](https://www.asianxiangqi.org/English/AXF_rules_Eng.pdf); [revision-history introduction](https://www.asianxiangqi.org/English/AXFrulesEng.htm). | Official association material. Summary read; linked PDFs located but not transcribed. The introduction lists the 1982 booklet, June 1989/March 2003 revisions and a 1996 English chapter; do not assume the undated summary/PDF equals the fourth revision. |
| X4 | CXA, *象棋竞赛规则（2020版）*, approved by 中国象棋协会, People's Sports Publishing House. Official [2026 competition regulation](https://www.sport.gov.cn/qpzx/n5387/c29676858/content.html), section 八（一）, explicitly adopts that edition. | Primary evidence of adoption, **not the complete rulebook**. Authoritative full-text acquisition and amendment check remain open. Unofficial transcriptions/scans are discovery aids, not a substitute for edition verification. |

X1's relevant locators are articles 1–3; 3.2.B (printed p.7/PDF page
15); article 8 (p.17/PDF 25); definitions 19.1–19.19 (pp.35–37/PDF
43–45); article 20 and chapter 5's adjudication examples (p.38 onward).
Check, mating threat and chase are separate classifications. The English text's
3.2.B describes four occurrences for a draw, whereas 19.8 describes threefold
repetition in defining perpetual check. These concern different clauses and need
comparison against Chinese, errata and intended application before a repetition
threshold is specified. Article 8 also qualifies its no-capture count and checks;
do not replace it with an unexplained generic chess counter. Article 3.1.A.II
classifies stalemate as a loss. These observations remain profile inputs, not an
adopted mShogi policy. [X1](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf)

X3's first section distinguishes check, chase, exchange, idle moves and genuine
versus ineffective protection; sections 2–3 treat mixed checking/chasing and
exceptions differently. A board-key repetition counter alone cannot establish
that adjudication. [X3](https://www.asianxiangqi.org/axf_rules.htm)

### Bounded next research deliverable

Acquire the Chinese X1 and edition-matched AXF/CXA texts; produce a small
clause-by-clause comparison of check/chase classifications, prohibited cycles,
neutral repetition, no-capture counting, automatic outcomes versus claims and
referee procedures. Include independently derived paired histories with the same
final board but different classification, counterexamples for protected/pinned
targets, and check/chase mixtures. Recommend one explicit base edition and any
software adjudication decisions; additional materially different profiles need
separate identities, records and evidence. Do not combine WXF movement with
convenient CXA repetition clauses under one unqualified “xiangqi” label. UCCI or
CECP opponent availability cannot decide those rules.

## Tori shogi — MSH-42

### Located evidence and unresolved primary acquisition

| Reference | Classification and locator | Evidence/status |
| --- | --- | --- |
| T1 | Kagawa University, [神原文庫分類目録・和漢書之部・芸術](https://opac.lib.kagawa-u.ac.jp/KMBR/mokuroku/KCCC1_W7.html), 将棋/796 entry **禽象戯**, 豊田玄龍, 天保4自跋刊, one volume. | Holding institution's primary catalogue metadata, pointing to an 1833 copy. Full treatise/facsimile and folio locators were not acquired; no rules claim is inferred from metadata. |
| T2 | Japanese Shogi Association's [2010-10-26 Ryūō event report](https://kifulog.shogi.or.jp/ryuou/2010/10/post-902b.html), photograph captions. | Firsthand modern report: two swallows on a file permitted, three forbidden. This narrow observation is not a comprehensive association rules edition. |
| T3 | *禽象戯図解* / *禽将棋図解*, proposed 1828 and 1833 editions; *禽将棋* attributed in secondary bibliographies to 岡野伸. | Acquisition targets, not primary texts reviewed here. [Historical-source discovery index](https://w.atwiki.jp/shogi-variants/pages/15.html) and [bibliographic discovery post](https://koshougi.seesaa.net/index-5.html) help locate copies but cannot settle rules or attribution. |

No complete official tori association code was established in this pass. In
particular, “1799” is an invention-date claim encountered in secondary discovery,
not a verified publication date for a tori *Shōgi Zushiki*. Do not conflate the
similarly titled historical books or assign tori rules to Okazaki without a
specific work and passage. JCSA's account of Okazaki Fumiaki concerns **chu**
rules; it cannot establish tori authority. [JCSA supplement, sections 1 and
3](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm)

### Bounded next research deliverable

Locate T1's actual copy or an institution-held facsimile of *禽象戯図解*, verify
title/author/colophon and distinguish the 1828/1833 witnesses. Request catalogue
identifiers or scans from Kagawa/NDL/Osaka University of Commerce as appropriate;
this document authorizes no messages to those institutions. Transcribe and
independently check passages for setup and handed quails, capture/demotion/hands,
swallow file/drop-mate/dead-rank restrictions, compulsory versus optional promotion
and exact zone timing, royal safety, repetition/perpetual check, no-action and
other terminal cases. Missing history/outcome text needs an explicit sourced
reconstruction proposal, not “all other rules are modern shogi.” Obtain any
claimed Kansai/Okazaki modern rulebook with author, edition and pages before
treating it as an adopted authority. Preserve disagreements as profile questions.

## Chu shogi — MSH-43

### Located association texts

| Reference | Classification and locator | Evidence/status |
| --- | --- | --- |
| C1 | 日本中将棋連盟 (JCSA), [中将棋連盟競技規則・総則編（2019年度版）](https://www.chushogi-renmei.com/kouza/rule2007.htm), corrected **2019-09-14**. | Official adopted association rules. The URL's “2007” is not the edition label. Sections I–IX and linked piece explanations are the acquisition scope. |
| C2 | JCSA, [中将棋対局規定・補足編](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm), updated **2004-06-06**. | Official explanations, revision rationale and case diagrams. Sections 1 (non-promotion), 2–3 (lion cases/revision), 4 (じっと) require reconciliation with C1 rather than assuming every older statement controls. |

C1 II distinguishes mate/royal capture and also requires removal of a prince
when present. IV covers promotion after declined entry and capture, pawn rescue,
dead pieces and a correspondence-only lance exception. VI covers repetition;
VII–VIII cover lion capture restrictions and stationary actions. Those clauses
make a universal one-royal or chess self-check filter unsafe; the selected
profile must specify permitted royal exposure, last-royal capture, mate and
prince treatment with explicit cases. No claim that “no check announcement”
alone establishes self-check legality is warranted. [C1](https://www.chushogi-renmei.com/kouza/rule2007.htm)

C2 explicitly describes revisions from Okazaki interpretations and its use of
*中将棋全集*/*中将棋指南抄*. Its multi-lion examples treat a compound choice as
one turn and discuss the priority of capture exceptions. These are evidence of
JCSA's modern adopted interpretation and reasoning, not direct inspection of
the cited historical books. [C2](https://www.chushogi-renmei.com/kouza/rule_hosoku2004.htm)

### Bounded next research deliverable

Recommend whether C1 plus reconciled C2 is the initial association profile;
record any material alternative separately. Acquire exact piece/setup pages and
the historical witnesses if making historical-conformance claims. Independently
translate fixtures for adjacent versus distant defended lions, hidden defenders,
non-lion capture/next-turn restrictions, kirin promotion during capture,
multi-lion exception priority, capture-and-return versus quiet return, promotion
decline/re-entry/capture and pawn/lance edge cases. Specify repetition responsibility
and match decisions, royal exposure with one/two royals, and C1's material-exhaustion
and agreed-draw provisions. Do not adopt a program's legal-set filter to resolve
an unanswered clause. Physical touch-move/foul recovery and software rejection
also need distinct treatment.

## Tenjiku shogi — MSH-44

### Historical evidence, reconstructions and implementation conventions

| Reference | Classification and locator | Evidence/status |
| --- | --- | --- |
| N1 | National Archives of Japan, [諸象戯図式 exhibit](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/popup_jousetsu_23_5_0002.html), linked [image 1](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/23_5_0002_1.jpg) and [image 2](https://www.archives.go.jp/exhibition/popup_jousetsu_23_1/23_5_0002_2.jpg). | Institution-hosted facsimile lead. Exhibition images do not establish a complete edition/folio inventory; no contested passage is claimed transcribed. |
| N2 | *諸象戯図式* (*Sho Shōgi Zushiki*), proposed 1694 first edition and 1696 reprint; *象戯図式* (*Shōgi Zushiki*), specific manuscript/copy still to identify. | Historical-primary acquisition targets. [NDL cooperative reference record 1000247371](https://crd.ndl.go.jp/reference/entry/index.php?id=1000247371&page=ref_view) points to a Matsuura copy at Kyoto Library/Archives, shelfmark 和/780/64, material ID 119040343. It also warns through its search history that the common title does not identify the intended work. This is holdings research, not the text itself. |
| N3 | Colin Adams, *The Struggle for Survival: Testing Times for Tenjiku Shogi* (1999), [PDF lead](http://history.chess.free.fr/papers/Tenjiku-Colin%20Adams.pdf), Appendix A, proposed pp.166–167 (Peter Banaschak translation). | Named modern author's reconstruction and translation lead; fetch returned 502. Those page locators were discovered through the [Ludii source index](https://ludii.games/data.php?keyword=Japan), not verified in the book. Retrieve the edition and verify its underlying source copy before citing translation as historical evidence. |
| N4 | H.G. Muller's [Tenjiku explanation](https://hgm.nubati.net/tenjiku/tenjiku.html), piece table and footnotes (*), (#). | First-party explanation of a modern software-oriented rules convention. Useful comparison candidate, not association adoption or an original Edo treatise. |

N4 specifies lion-plus-bishop lion hawks, ranked jumping generals and an effect
order for fire-demon burns, including promotion and surviving a move beside an
enemy demon. Its concrete choices show why an opponent's supported game name is
insufficient to establish compatibility. They are not selected here as mShogi
rules. [N4](https://hgm.nubati.net/tenjiku/tenjiku.html)

### Bounded next research deliverable

Acquire the exact N1/N2 catalogue records and complete relevant volume, distinguish
1694 from 1696, and verify setup/promotion tables and original glyphs/diagrams.
Compare N3's traditional/modern reconstruction choices only after retrieving it.
Prepare a dispute table for fire-demon range orientation, area-move stopping and
burn timing, mutual demons, promotion-to-demon effects, lion-hawk powers,
general rank/jump/capture restrictions and royal exceptions, free eagle/heavenly
tetrarch movement, promotion timing and game/history conclusions. Each row needs
original passage/folio, translation, named proposed revision, implementation
behavior and a recommended explicit profile decision. Do not select a revision
merely because it appears more balanced or assume JCSA chu rules automatically
govern tenjiku. Unavailable primary text remains a named conformance blocker for
this game, with acquisition/translation work as its concrete next action.

## Architecture pressure and delivery boundaries

These are planning risks to validate with real consumers, not requirements to
prebuild a universal board, rules DSL or transport interpreter:

| Game | Pressure on existing shared contracts | Verification that remains game-specific |
| --- | --- | --- |
| Xiangqi | 9 files × 10 ranks of **intersections**, river/palace presentation, oriented sides and notation; explicit check/chase history classifications. | Geometry/coordinates, complete movement constraints, history-sensitive adjudication and versioned profile compatibility. |
| Tori | 7×7 **squares**, hands/drops and handed piece identities; test reuse of standard-shogi extensions against actual differences. | Setup, drop restrictions, exact promotion and outcome/history policy; no inherited modern-shogi rule by default. |
| Chu | 12×12 squares; compound complete choices, same-endpoint routes, multiple captures, transient next-turn rights, additional royalty. | Atomic resolution/reversal, source-derived legal paths, lion history, promotion history and royal/outcome distinctions. |
| Tenjiku | 16×16 squares; long compound/area paths, ranked jumps, multiple derived removals and effect ordering, promotion changing effects. | Disputed-rule decisions, derived-effect ordering, royal/history semantics and independent complete-action fixtures. |

MSH-42–45 each retain their **entire** existing delivery: sourced/versioned complete
rules, state/actions/reversal, necessary history and automatic/claimable outcomes;
independent expected conformance evidence; independently usable console and
notation adapters; **mShogi's own native functional engine** using authoritative
history, safe requests/cancellation and reproducible play; lossless canonical
records/replay; actual accessible desktop human/engine play, inspection,
save/reopen and game-specific end-to-end evidence. External engines are
references/opponents, not replacements. Measured strength belongs to MSH-51–54
in M5; functional delivery cannot move there.

MSH-46 owns shared extensions once, including profile/game dispatch, complete
choices/results, supervision, common record compatibility/capacity and reusable
presentation; each game owns its semantics and concrete adapters. MSH-47 retains
the full six-game integration/three-platform closure verdict. Chess, completed
M2/M3 evidence, lossless admission, and synthetic compound/large-board fixtures
remain preserved. Synthetic fixtures establish boundary exercise only. This
inventory neither starts a later implementation nor marks a game, MSH-46,
MSH-47 or M4 complete.
