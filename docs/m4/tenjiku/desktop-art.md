# Tenjiku desktop lettering and visual contract

This note describes Tenjiku artwork, board presentation and bundled lettering.

<a id="current-owner-approved-continuation--5-november-2026"></a>

## Current approved continuation — 5 November 2026

The recorded approval covers the overall traditional-kanji artwork and existing shared board
presentation. The [recorded decision](../../design.md#owner-acceptance-of-tenjiku-visuals-and-shared-identification-2026-11-05)
retains the 760×650 application minimum. All 256 Tenjiku cells fit, while dense
kanji and secondary route badges remain too small for comfortable reading in
some cases. Around 960×1000 is a recommended playing size when the display
permits, not a new minimum. No glyph, artwork, zoom or minimum-size change is
part of this continuation.

The existing footer now identifies the selected piece using the label
across all six games, and retains it through compound selection. Occupied legal
capture destinations have decorative outlined markers painted above the piece
artwork; the markers ignore pointer events. Inspection remains distinct from a
playable selection. The normal sixth-game chooser exposes both named immutable
profiles and normal Load preserves the saved profile.

## Traditional names and source alignment

The selected default is traditional variant-appropriate kanji, as recorded
in `docs/design.md` A-07. The accepted Chu renderer provides twenty-eight of
Tenjiku's forty-four face spellings; Chu's Go-Between is absent. The shared
Knight name is 桂馬. The fourteen Tenjiku-specific names are independently
listed in researcher Ryō Onuma's original
[18 November 2006 Tenjiku discussion](https://blog.livedoor.jp/r_onuma/archives/cat_50047311.html).
The Japanese Chu Association's
[authored piece survey](https://www.chushogi-renmei.com/koramu/EX1.htm)
also supplies modern 鉄将 and 竪兵 spelling. This naming evidence does not
replace either selected rules profile. Differences in those authors' movement
or promotion descriptions are not imported into the product.

Adams's English face names and the `Face` codes are identified in
`profile.json.pieces`; [sources.md](sources.md) identifies the inspected English
sources. The following lettering aligns those identities with the Japanese
names. Existing Chu spelling is preserved, including 龍, 酔, 鯨鯢 and 奔猪;
Onuma's 雜将 is retained with U+96DC 雜 rather than silently normalized to 雑.
The named modern sources do not prove the exact lettering of an unacquired
historical witness.

| Face | English identity | Lettering |
|---|---|---|
| K | King | 玉将 |
| FiD | Fire Demon | 火鬼 |
| GGn | Great General | 大将 |
| VGn | Vice General | 副将 |
| RGn | Rook General | 飛将 |
| BGn | Bishop General | 角将 |
| LHk | Lion Hawk | 獅鷹 |
| Ln | Lion | 獅子 |
| WBf | Water Buffalo | 水牛 |
| FEg | Free Eagle | 奔鷲 |
| FK | Free King | 奔王 |
| HT | Heavenly Tetrarchs | 四天王 |
| SE | Soaring Eagle | 飛鷲 |
| HF | Horned Falcon | 角鷹 |
| ChS | Chariot Soldier | 車兵 |
| DK | Dragon King | 龍王 |
| DH | Dragon Horse | 龍馬 |
| FO | Flying Ox | 飛牛 |
| FB | Free Boar | 奔猪 |
| R | Rook | 飛車 |
| B | Bishop | 角行 |
| Ph | Phoenix | 鳳凰 |
| VSo | Vertical Soldier | 竪兵 |
| VM | Vertical Mover | 竪行 |
| SSo | Side Soldier | 横兵 |
| SM | Side Mover | 横行 |
| WH | White Horse | 白駒 |
| W | Whale | 鯨鯢 |
| Ky | Kirin | 麒麟 |
| CP | Crown Prince | 太子 |
| FS | Flying Stag | 飛鹿 |
| RC | Reverse Chariot | 反車 |
| L | Lance | 香車 |
| MGn | Multi General | 雜将 |
| G | Gold General | 金将 |
| DE | Drunk Elephant | 酔象 |
| BT | Blind Tiger | 盲虎 |
| FL | Ferocious Leopard | 猛豹 |
| S | Silver General | 銀将 |
| C | Copper General | 銅将 |
| I | Iron General | 鉄将 |
| N | Knight | 桂馬 |
| Dg | Dog | 犬 |
| P | Pawn | 歩兵 |

## Game ownership and rendering

`SquareBoard.tsx` draws Tenjiku, like Chu, on the shared `Board` inside the
accepted `GameFrame`. It reads `tenjiku.selected.identities` by square;
`face` supplies lettering while original `base` and `promoted` remain
separate. The shared `Piece.tsx` tile holds the face table. No frontend promotion
table, movement rule, check filter, second controller, hands or alternate shell
is introduced. The two immutable profiles use the same lettering; their
choices and effects remain distinct.

This matters for original LHk and promoted Ln→LHk, original FEg and promoted
FK→FEg, and every other shared movement face. Promoted engraving is dark red
with two corner chevrons, retaining a non-color distinction. King 玉将 and
Prince 太子 retain different names. Ownership rotates the entire pentagonal
tile and engraving 180 degrees. Square labels remain the accessible
names; decorative SVG is hidden from assistive technology. Unknown faces render
no substitute piece or raw diagnostic abbreviation.

The tile geometry reuses original project Chu artwork and its accepted warm
wood, bevel and shadow treatment. Dog uses a large single character. Two-letter
names use 47-unit vertical engraving. Heavenly Tetrarchs retains all three
characters 四天王 in source order at 32 units, with baselines 33/63/93. No
unsupported one-character abbreviation is substituted. Pinned font outlines at
those baselines have nonoverlapping vertical bounds and remain within the
96-unit tile body. Actual reading comfort is subject to visual review.

Selected squares, destinations, pending routes, completed origin/destination,
stationary capture, burn, promotion and mover destruction keep the existing
choice/effect architecture. Tenjiku-scoped CSS uses the accepted Chu
target and route colors and compacts the existing route controls into the
shared footer. No board scrolling, zoom, minimum-window change, or separate
interaction architecture is introduced.

## Bundled licensed font

**mShogi Tenjiku Pieces Regular** is a separate 22,028-byte WOFF2 subset of
Noto Serif CJK JP Regular 2.003. Source and existing accepted subsets remain
unchanged.

- Official upstream: [notofonts/noto-cjk](https://github.com/notofonts/noto-cjk).
- Pinned commit: `9b0f1436e455d902de067a2501422e5dc71ad16b`.
- [Japanese Regular input](https://github.com/notofonts/noto-cjk/blob/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf), 24,573,864 bytes.
- Input SHA-256: `d9854c7a8ef170b5a7932558856fd64eb8de0b007cd823fed6f9f514ad2803d3`.
- Original copyright: © 2017–2024 Adobe (http://www.adobe.com/).
- [Pinned upstream license](https://github.com/notofonts/noto-cjk/blob/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/LICENSE): SIL Open Font License 1.1; existing `OFL.txt` retained unchanged.
- Renamed family: `mShogi Tenjiku Pieces`; PostScript name: `mShogiTenjikuPieces-Regular`.
- Output SHA-256: `fc32544085ef2647d7f5566beec4bfa19ad30a9c99df51947f7e28c8ddd41dda`.

The subset contains exactly 52 unique Unicode characters and 65 glyphs including
dependencies. Every character has nonempty source and output outlines. Chu's
subset lacks `副四大天桂水火犬鉄雜鬼`; ten of those are absent from every
existing subset. A separate complete Tenjiku subset prevents platform fallback
or reliance on another game's font-family chain.

Reproduce with Python 3.11.0, fontTools 4.47.2 and Brotli 1.1.0:

```sh
PYTHONPATH=/path/to/brotli python3 desktop/src/assets/fonts/subset.py tenjiku \
  /path/to/NotoSerifCJKjp-Regular.otf \
  desktop/src/assets/fonts/mshogi-tenjiku-pieces.woff2
```

The script verifies the input hash, source and exact output Unicode coverage,
nonempty outlines, renamed name/CFF identities and retained OFL metadata. Two
actual generations were byte-identical (`cmp`). Local verification reused the
existing pinned input `/private/tmp/msh42-NotoSerifCJKjp-Regular.otf` and temporary
Brotli directory `/private/tmp/msh42-font-python`; neither is a runtime
dependency or a repository asset. The font is bundled locally without system
installation or remote requests. The existing font license and appended
`NOTICE.txt` entry accompany it.
