# MSH-81 historical-source acquisition report — 2026-10-31

Worker: tori_historical_sources; root remains sole durable integrator on `codex/rewrite`. This worker changed no repository/task/product record and adopted no profile. Scope: exact Kagawa 1833 and alleged Tohoku 1828 leads. No institutional messages, purchases, login, or bypass occurred.

## Concrete result

A public institutional facsimile **was acquired**: Tohoku University Library's *禽象戲圖解*, Kano collection, record **10010000019229**, call number **5/17701/1**, OPAC **TT21260078**, microfilm **EKH0010084**, sequence **05-002120**, digital image prefix **THKK-52120**, 15 images. All 15 images were downloaded from the manifest's own URLs and visually inspected. The rules conclude with **文政十一年 戊子孟春** (1828, early spring), image 9 left. This is an internal text date, **not proof of the physical copy's publication date**: the institutional metadata explicitly says `[出版年不明]`. The title, author, collection, source image identifiers, and internal date now replace the previous non-institutional lead.

Kagawa's exact OPAC now resolves successfully in a normal browser to **volume/513862**. It confirms the 1833 holding, location and non-circulation status. Its facsimile remains unacquired. The two witnesses' bibliographical or author-name relationship is not established merely by their similar titles.

## Exact institutional sources and acquisition status

1. [Tohoku record](https://touda.tohoku.ac.jp/portal/item/10010000019229). Title 禽象戲圖解, reading キンショウギズカイ; maker 豐田四郎兵衞安親 / トヨタシロウベエヤスチカ; unknown publication place/publisher/year; printed material, one Japanese volume, 準貴重図書; former owner 狩野亨吉; 同人版. 本館2号館, 5/17701/1, TT21260078. Notes say NIJL historical-classics network digitization. Manifest records photography **1991–1993**, **15** monochrome images, TIFF record format. Do not confuse modern scanning dates with the book's date.
2. [Tohoku IIIF manifest](https://touda.tohoku.ac.jp/collection/iiif/metadata/10010000019229/manifest.json), saved as `tohoku-10010000019229-manifest.json`. It is IIIF Presentation 3; its internal `id` includes `/collection/iiif/0/metadata/...`, whereas the exposed and retrieved URL omits `/0/`. Images come directly from each canvas `body.id`; no guessed or protected endpoints used. `viewingDirection` is left-to-right for the microfilm image sequence; text columns themselves are read right-to-left.
3. [Tohoku image-use conditions](https://touda.tohoku.ac.jp/collection/database/library/use_of_materials). Item and manifest explicitly label images **CC BY**; record's linked badge resolves to CC BY 4.0. Metadata **CC0**. The institution permits reproduction, modification and distribution under the stated terms without special permission for CC BY items. Required attribution near reused images: **『禽象戲圖解』（東北大学附属図書館所蔵）**; retain call number 5/17701/1 to distinguish copies. No contact was sent.
4. [Kagawa precise holding](https://opac.lib.kagawa-u.ac.jp/opac/volume/513862). Live browser inspected title 禽象戯, 豊田玄龍, `[出版地不明] : [出版者不明], 天保4 [1833] 跋`, one volume; **中央館3F神原文庫**, **796 / 2**, **2111124178**, **禁帯出**, **配架済**. No facsimile link appears in this record. CiNii link now resolves after the dynamic page loads; the prior web-tool cache error did not establish nonexistence.
5. [CiNii NCID BB11784195](https://ci.nii.ac.jp/ncid/BB11784195), HTML acquired. Metadata still separately describes this old-book copy through retrospective data; the publication statement is based on a postscript. It does not establish identity with Tohoku.
6. [Kagawa arts catalogue](https://opac.lib.kagawa-u.ac.jp/KMBR/mokuroku/KCCC1_W7.html), acquired original HTML. Raw UTF-8 line 557 reads `<dt>禽象戯</dt><dd>豊田玄龍　(天保4自跋刊)</dd><dd>1冊　796</dd>`. Web extraction did not find 禽, but the actual HTML does; this is a tool extraction limitation. An initial attempted Shift-JIS conversion failed because the acquired document is already UTF-8; the incomplete conversion file is excluded from evidence.
7. [Kagawa digital-archive search for 禽](https://digarc.kagawa-u.ac.jp/search?kywd=%E7%A6%BD&title=&alt_title=&creator=&publish_name=&date=&photocopy=&resource=All&asset_num=): live search showed **0 results** and 該当する資料はございません. This is a bounded negative result for this query/catalogue, not proof that no scan exists anywhere. Archive home says only part of the Kanbara collection is digitized. Older repository links returned 429 via the web tool and were not hammered.
8. [Kagawa physical viewing/reproduction process](https://opac.lib.kagawa-u.ac.jp/KMBR/eturan.html), read via web extraction. Advance reservation and confirmation for physical access, generally one week ahead; reproduction application identifying catalogue title and classification number; content/state may limit access. Hard copies are limited to already digitized or microfilmed content. Exact outstanding request target, if later separately authorized: 禽象戯, 神原文庫796/2, item2111124178, OPAC513862, NCIDBB11784195. No application submitted.

## What was actually viewed in the Tohoku images

Images are JPEG derivatives served by the institution's IIIF `full/max/0/default.jpg` URL, 3871×2581 pixels in the downloaded files (canvas metadata describes 6240×4160). They are not downloaded TIFF masters. Filenames below abbreviate `tohoku-THKK-52120-NNNN.jpg`. The public image itself bears the attribution requirement above.

| Image | Visually inspected contents | Reading coverage |
| --- | --- | --- |
| 0001 | Outer cover *禽象戲圖解*, library label 狩5門17701,1冊 | Title/label read |
| 0002 | Cover/interior blank, damaged title strip | Visually inspected; no rules |
| 0003 | Large display title, collection/condition notes and seals | Title and collection note inspected; seals not deciphered |
| 0004 | Whole setup diagram headed 禽象戲駒備方之圖; numerals and movement marks | Whole board inspected; mapping needs normalized orientation audit |
| 0005 | 駒利樣之圖解: 鵬; 鶴; 雉 diagrams and prose | Movement prose and diagrams read |
| 0006 | Both handed 鶉; 鷹 diagram/prose | Diagrams and relevant prose read |
| 0007 | Promoted 鷹→鵰; 燕 prose and diagram | Diagrams and relevant prose read |
| 0008 | 燕→鴈 diagram; 指方之法 starts | Rule prose read, continues onto 0009 |
| 0009 | Remaining play rules, repetition, drops, promotions; 文政十一年戊子孟春 | Full rules page read; translation below is provisional |
| 0010 | 豐田四郎兵衞安親撰; association with late 大橋宗英; literary text starts opposite | Author/attribution read; not a claim to translate literary appendix |
| 0011–0013 | Classical literary appendix, ending 欽水散人 (name reading uncertain) | Visually inspected; no full translation or rule claims from appendix |
| 0014 | Interior blank, stamped accession number 7765; final name | Visually inspected |
| 0015 | Rear cover | Visually inspected |

All 15 images are preserved; “all inspected” does not mean all cursive/literary text has been transcribed or translated. No iroha game record or sample game appears in this acquired image set. The report cannot attest to absent/unfilmed material beyond the 15-image manifest.

## Direct rules evidence and translation confidence

This is a source acquisition and preliminary reading, **not an independently approved scholarly transcription**. Ordinary Japanese glyph and diagram readings below are high confidence where identified; period syntax and semantic generalization are medium until a fresh reviewer checks the images. No software output supplied expected results.

### Setup and movements

- **0004 and 0008 left:** diagram shows **7×7** board; prose says `盤面縱横七目駒數三十二枚`. High confidence: 49 cells and 32 pieces total. Whole setup is available for a separately audited fixture; quail left/right must be mapped relative to the owner, not screenshot left/right. The diagram has a full seven-swallow line and one advanced swallow for each player. Coordinate presentation in the sideways book diagram should not be silently converted to modern notation without review.
- **0005 right:** 鵬 is compared with 玉将, with `利ハ八方一間宛ノ利也`; diagram marks all eight adjacent squares. High confidence: one-step eight-direction royal movement. This alone does not settle all check or ending conventions.
- **0005 left:** 鶴 diagram has six adjacent dots: forward/backward straight and all four diagonals, no lateral steps. Prose says six directions one square. High confidence geometric seed.
- **0005 left:** 雉 diagram marks forward-two, hollow intermediate forward-one, and backward-diagonal-one on each side. Prose uses `飛` for the forward movement. High confidence displacement seed; independently check leap/blocker terminology before full legality fixtures.
- **0006 right:** two 鶉 diagrams and back labels `裏ニ右ノ字ヲ記ス` / `裏ニ左ノ字ヲ記ス`. Prose describes two running directions and one one-square direction; forward movement same, left/right movement changes. Diagram reading: right quail runs forward straight and backward-left diagonal, steps backward-right; left is the mirror. High confidence diagram geometry; normalize player-relative orientation independently. The next rules page explicitly says these back labels indicate their differing movement, **not promotion**.
- **0006 left:** 鷹 has seven adjacent dots, excluding straight backward. Prose says seven directions one square; it becomes 鵰. High confidence displacement and promotion identities.
- **0007 right:** 鵰 diagram has forward-diagonal rays, backward-straight dots to distance two with no hollow intermediate, forward-straight and lateral adjacent dots, and backward-diagonal dots to distances one and two. **Correction for fresh review:** visually audit exactly which of these three ray/step directions the prose's `三方走三方一間宛二方二間宛` describes. Preliminary conventional reading is forward diagonals and straight backward running; forward-straight/laterals one; backward diagonals up to two. Source differentiates this from the explicit hollow-circle leap diagrams. Do not call backward-diagonal-two a leap without confirming blockers.
- **0007 left:** 燕 has one forward adjacent dot and prose `利ハ一方一間宛ノ利也`, becomes 鴈. High confidence.
- **0008 right:** 鴈 has solid destination dots at forward-diagonal-two and straight-backward-two with hollow intermediate squares, prose three-direction flight. High confidence destination geometry and promotion identity; inspect jumping semantics independently.

### Captured-piece reuse, swallow multiplicity, repetition

- **0008 left opening:** `駒ハ取盡ス事無ク小棋ノ式ニ從ヒ再ビ打換ル也` (some glyph normalization; exact visual authority is image). Interpretation, medium/high confidence: captured pieces are reused/dropped following small shogi. The paragraph discusses the two opposed advanced swallows and how captures depend on who plays first. This is direct historical evidence for reuse, not a modern software assumption.
- **0008 left:** `燕ノ遣ヒ方ハ小碁ノ歩兵ニ不役重テ打遣フ事ハ免ス二枚ニ限ル` (small-shogi glyph needs transcription review). Interpretation, medium: repeated swallow drops are allowed unlike the small-shogi pawn restriction, limited to two. This is stronger than the old JSA caption alone, but exact file/side/unpromoted counting and move-versus-drop scope require period terminology review; do not assert a comprehensive predicate from the numeral alone.
- **0008 left final column → 0009 right first column:** `同手三度ニ及時ハ仕掛ケ方ヨリ手ヲ換ル也`. High confidence reading, medium semantic precision: when the same play reaches three times, the initiating side changes its move. This supports an obligation to vary, **not** an automatic draw or immediate loss and not a fully specified modern repetition-state identity. It does not explicitly define hands/turn equality, cycles with several initiators, perpetual check exception, or forced lack of alternatives. Small margin annotation uses 千日手; do not assume every modern 千日手 convention.

### Drop mate and promotion

- **0009 right:** `燕ニテ鵬ヲ打詰ル事ヲ禁ズ` — high confidence direct prohibition on mating 鵬 with a dropped 燕. Following clause distinguishes pushing an on-board swallow to mate, promotion marks, and the promoted 鴈. Preliminary interpretation: a swallow pushed into the promotion area promotes immediately, and a goose may mate according to small-shogi practice. Exact adjacency/direct-check scope still deserves fresh translation.
- **0009 right:** `其ノ他ニ成ナキ者 ...` followed by free use; paired with promotion statements for only 鷹 and 燕. High confidence there are only these two promoted piece classes in this text; no promotion of quails merely because their reverse bears 左/右.
- **0009 right→left:** the author calls attention to the quails' reverse-side 左/右 as movement markings, `非成` (not promotion). High confidence distinction; retention or swapping of quail identity after capture still requires explicit interpretation with piece orientation.
- **0009 left:** `惣而成リ方ノ儀ハ敵地成ノ印江入ニハ不成ニテ行事ヲ禁ズ` — high confidence direct wording prohibits entering the marked enemy promotion region unpromoted. **Mandatory-promotion reading has actual historical support.** It should not be presented as a new convention picked merely to match existing software.
- **0009 left continuation:** `敵地ノ成ノ印ヘ入テハ敵ノ駒ヲ取リ又ハ鵬ノ頭ニテモ直ニ成ル也` — preliminary high-confidence reading that immediate promotion still applies on capture or at the royal's head. Translate exact condition/scope independently, especially moves already in zone and leaving the zone.
- **0009 left:** `敵地ヘ駒打時ハ成ノ印ノ内ニテモ鷹燕共ニ表ノ方ヲ打次ニ成ルナリ` — direct evidence that hawk and swallow drops even inside the marked area use the front/unpromoted face and promote on the next move. This needs a precise interpretation of “next”: any next move including leaving zone versus only a qualifying next move. It is not evidence that the drop itself promotes.
- **0009 left ending:** `其外指方小将棋ノ意ヲ以テ可有含者也` (provisional transcription) explicitly directs other play according to the sense of small shogi. This makes “no historical inheritance exists” incorrect. It **does not identify a modern JSA rule edition** or settle which 1828 small-shogi conventions should be encoded for self-check, stalemate, entering royals, demotion, forced cycles, administrative fouls and notation.

## Material gaps and consequences

1. Independent Japanese/source review is necessary before ratifying the exact clause semantics. Root has the full images so this is now a translation/interpretation task, not an access blocker for the Tohoku text.
2. Kagawa 1833 actual pages remain unavailable in this pass. Establish author-name relationship (豊田玄龍 versus 豐田四郎兵衞安親), revision/additional records, textual differences and colophon through the real Kagawa copy. Do not treat 玄亀 from an old secondary bibliography as a correction of 玄龍.
3. Tohoku's physical issue date and edition relationship remain unverified despite the internal 1828 date. No cover imprint or dated publication colophon was found in the 15 images.
4. Repetition text lacks an implementation-complete state/initiator/forced-cycle/perpetual-check algorithm. A reviewer and ultimately owner may need explicit reconstruction choices. Existing software cannot provide historical authority.
5. Historical text's small-shogi reference is real but underspecified for a current full profile: royal safety, mate/capture convention, stalemate/no-legal-action, entering royals, exceptional draws, resignation/agreements and administrative sanctions need a justified source/interpretation boundary. No claim that the literary appendix resolves these.
6. No standard canonical move notation, iroha replay record, turn/hand serialization or native software identity is supplied by this copy. Those would be explicitly proposed adapters with fixture provenance, not historical quotations.
7. The source now supports many independently expected **geometry/restriction seeds**: board/cardinality; all movement destination shapes; only hawk/swallow promotion classes; quail reverse labels not promotion; unpromoted zone drops; forced promotion reading; dropped-swallow mate prohibition; third same-play initiator change. Full-action legal-position expectations need the unresolved broader predicates first.

## Retrieval and reproducibility

Directory: `/private/tmp/msh81-historical` (temporary research cache, not integrated durable storage). Core retrieval commands executed successfully with public network approval, on macOS, 2026-10-31:

```sh
curl -L --fail --max-time 30 'https://touda.tohoku.ac.jp/collection/iiif/metadata/10010000019229/manifest.json' -o /private/tmp/msh81-historical/tohoku-10010000019229-manifest.json
curl -sS -L --fail --max-time 60 "$CANVAS_BODY_ID" -o "$IMAGE_PATH"
```

For the second command `$CANVAS_BODY_ID` was read from each `manifest.items[n].items[0].items[0].body.id`; exact expanded URLs, local files, curl exits (all0), sizes and **SHA-256** are in `acquisition-images.json`. No response status code was captured for those first image downloads; do not fabricate numeric HTTP codes. Curl `--fail` returned0 for every completed image retrieval. An initial sandbox curl DNS failure was resolved by ordinary network escalation; it was not an institution access restriction. Subsequent metadata fetches record actual final HTTP codes in `acquisition-metadata-additional.json`. `source-cache-sha256.json` includes manifest and metadata hashes plus local inspection coverage; all images are retained exactly as downloaded.

No PDF was acquired/read here, so the PDF skill's rendering workflow was not invoked. Nothing was installed or generated to replace a missing historical original. No broad GUI/product checks, native implementation, record-format change, commit, push or Backlog update was performed by this worker.
