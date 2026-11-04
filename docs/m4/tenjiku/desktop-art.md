# Tenjiku desktop lettering and visual contract

MSH-44, 4 November 2026. Implementation starts from `84d7aa8` on
`codex/rewrite`. This document records artwork naming, asset provenance and
the bounded verification scope. It does not establish owner visual approval,
normal chooser activation, complete product delivery or historical-facsimile
agreement.

## Traditional names and source alignment

The owner-selected default is traditional variant-appropriate kanji, as recorded
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

Adams's English face names and the native `Face` codes are identified in
`profile.json.pieces`; [sources.md](sources.md) identifies the inspected English
sources. The following lettering aligns those identities with the Japanese
names. Existing Chu spelling is preserved, including 龍, 酔, 鯨鯢 and 奔猪;
Onuma's 雜将 is retained with U+96DC 雜 rather than silently normalized to 雑.
The named modern sources do not prove the exact lettering of an unacquired
historical witness.

| Native face | English identity | Lettering |
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

## Native ownership and rendering

`TenjikuBoard.tsx` decorates the existing shared `NativeBoard` inside the
accepted `GameFrame`. It reads `tenjiku.selected.identities` by square; native
`face` supplies lettering while native original `base` and `promoted` remain
separate. Native selected `royals` supplies royal metadata. No frontend promotion
table, movement rule, check filter, second controller, hands or alternate shell
is introduced. The two immutable profiles use the same lettering; their native
choices and effects remain distinct.

This matters for original LHk and promoted Ln→LHk, original FEg and promoted
FK→FEg, and every other shared movement face. Promoted engraving is dark red
with two corner chevrons, retaining a non-color distinction. King 玉将 and
Prince 太子 retain different names. Ownership rotates the entire pentagonal
tile and engraving 180 degrees. Native square labels remain the accessible
names; decorative SVG is hidden from assistive technology. Unknown faces render
no substitute piece or raw diagnostic abbreviation.

The tile geometry reuses original project Chu artwork and its accepted warm
wood, bevel and shadow treatment. Dog uses a large single character. Two-letter
names use 47-unit vertical engraving. Heavenly Tetrarchs retains all three
characters 四天王 in source order at 32 units, with baselines 33/63/93. No
unsupported one-character abbreviation is substituted. Pinned font outlines at
those baselines have nonoverlapping vertical bounds and remain within the
96-unit tile body. Actual reading comfort is subject to the owner checkpoint.

Selected squares, destinations, pending routes, completed origin/destination,
stationary capture, burn, promotion and mover destruction keep the existing
native-choice/effect architecture. Tenjiku-scoped CSS uses the accepted Chu
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
PYTHONPATH=/path/to/brotli python3 desktop/src/assets/fonts/subset-tenjiku.py \
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

## Responsive risk and actual measurement

Source-derived upper bounds in the current shared shell are approximately:

| Logical content | Board | Cell | Two-character engraving | Three-character engraving |
|---|---:|---:|---:|---:|
| 1180×820 | 462 px | 27.5 px | 12.9 px | 8.8 px |
| 960×720 | 381 px | 22.7 px | 10.7 px | 7.3 px |
| 760×650 | 343 px | 20.4 px | 9.6 px | 6.5 px |

These are calculations, not application observations. Additional historical
notices or a verification banner may reduce space. `Board.tsx` gives the 16×16
board a 1680-unit viewBox including outside coordinates. Minimum-size density,
particularly 四天王 and complex names such as 獅鷹 and 鯨鯢, is a material
reading-comfort risk. Fit and glyph coverage alone do not establish usability.
Actual captures must make any problem reviewable; changing the minimum size or
abbreviating source names requires an owner decision.

Actual macOS verification must use the fixed QA slot and primary-checkout helper
from [packaged-gui-verification.md](../../desktop/packaged-gui-verification.md),
selecting the external fixture while stopped. The native QA menu establishes
1180×820, 960×720 and 760×650 content sizes and records measured content/frame
dimensions in its title. Retain real screenshots and accessibility observations
for both orientations, standard density, all faces, promotions, both royals,
ordered routes, stationary capture, burns and mover destruction.

For native WebDriver campaigns, reuse the actual resize, shared shell and
256-position ancestor-clipping measurement patterns in
`desktop/scripts/gui-smoke.py`: record actual viewport, board/cell rectangles,
all 256 visible targets, ancestor overflow and scroll positions, footer/control
visibility, font loaded state and glyph bounds. The current shared completed
route numbers are 12 SVG units and are also a minimum-size legibility risk.
Measurements must distinguish content size, native frame size and Retina scale.
No native UI evidence is claimed by jsdom component tests.

## Bounded implementation evidence

`TenjikuPiece.test.tsx` checks all forty-four source-aligned face names, reused
Chu spelling, full three-character ordering, original/promoted identities with
the same face, non-color promotion marks, whole-piece rotation, native royal
metadata and unknown-face rejection. `TenjikuArtwork.test.tsx` checks both
native profiles, authoritative selected identities despite conflicting
diagnostic presentation fields, accessible square-label preservation, royalty,
256 targets, shared footer and orientation.

The existing native-generated board tests continue to exercise complete routes,
promotion, igui, profile-specific effects, ranked jumps, historical inspection
and cancellation. Component/font verification is recorded separately from the
subsequent actual visual checkpoint and broader task verification record.

Local checks on 4 November 2026 used macOS arm64, Node 24.22.1, Python 3.11.0,
fontTools 4.47.2 and Brotli 1.1.0. The base is `84d7aa8` plus the uncommitted
MSH-44 implementation; no future integration hash is implied. Node was selected
from the existing Codex runtime dependency directory. An earlier run also
passed on the shell's Node 23.11.0; supported Node 24 evidence follows.

- `npm --prefix desktop test -- src/product/TenjikuPiece.test.tsx src/product/TenjikuArtwork.test.tsx src/product/TenjikuBoard.test.tsx src/product/TenjikuRender.test.tsx`: **41 passed in four files** on Node 24.22.1 after native fixture regeneration (earlier 39-test pass retained as prior evidence). Includes all thirty native once-promoted identities and both native all-faces diagrams: 67 original physical identities, 44 movement faces, 30 promotion marks and three surviving royal identities. The existing board test now expects mShogi and actual engine/analysis capabilities after functional engine integration.
- `npm --prefix desktop run check`: **passed** on Node 24.22.1.
- Two `subset-tenjiku.py` runs using the pinned source and temporary Brotli path, followed by `cmp`: **passed**, byte-identical output with the hash above.
- Direct fontTools inspection: **passed**. All forty-four component labels match exactly the subset's fifty-two codepoints; every outline is nonempty, CFF family/PostScript identity is renamed, OFL metadata remains, and full 四天王 outline bounds at 32 units are `[8.872,34.376]`, `[37.272,65.624]`, `[67.016,93.960]` vertically within the tile.
- Existing Shogi/Chu/Tori/Xiangqi WOFF2 files and `OFL.txt`: **unchanged** in Git.
- Actual native window rendering, minimum-size comfort, all-platform visual verification and owner approval: **not run by this component/asset assignment**. They remain part of the primary MSH-44 checkpoint and verification record.

## Actual application checkpoint supersedes size estimates

The primary MSH-44 verifier subsequently launched the real fixed-slot macOS app.
The [genuine gallery](evidence/msh44-macos/gallery.md),
[artifact provenance](evidence/msh44-macos/artifact-manifest.json) and single
[product record](product-verification.json) distinguish the component assignment
above from actual native rendering and interaction. The final reviewed production
source is `2c00e2107ac74334bce5d55813e5be25666d7c46`; the final captured package
hashes equal the exact postcommit rebuild. Earlier all-faces/tall comparisons
retain their explicitly recorded source scope.

| Native preset | Actual WebView | Board | Cell | Minimum all-faces em |
|---|---|---:|---:|---:|
| 1180×820 | 1180×788 | 399px | 23.750px | 7.600px |
| 960×720 | 960×688 | 350px | 20.833px | 6.667px |
| 760×650 | 760×618 | 278px | 16.548px | 5.295px |
| Proposed960×1000, live | 960×968 | 598px | 35.595px | 11.390px |
| Proposed960×1000, History | 960×968 | 534px | 31.786px | 10.171px |

Each measured view retains all256 cells and the bundled font. Independent visual
review found the current minimum insufficient for comfortable dense-kanji reading;
it found the concrete taller proposal materially clearer in both orientations.
This is an unresolved owner product decision. No minimum, three-character name,
no-scroll/no-zoom requirement or shared layout architecture has changed. The
QA-only tall native preset permits review without implementing that policy.

Actual pointer/keyboard compound capture, stationary capture, native promotion
choice/refusal, paired profile-specific Fire Demon burns/mover destruction, engine
reply, read-only analysis, history and file/recovery controls are represented in
the gallery. Native readable effect descriptions accompany accessible Moves,
including simultaneous burn batches and promotion/original identity. Canonical
actions, effects and outcomes still come only from Rust. Owner visual approval,
normal selector activation and final acceptance remain pending.
