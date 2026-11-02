# Tori piece presentation

## Traditional default

Traditional native kanji are the default Tori piece presentation. The owner
clarified this during the MSH-42 visual checkpoint: retain original bird artwork
as a possible future alternative, while traditional native glyphs remain the
default for Shogi variants. No artwork selector or new preference is introduced.

`ToriPiece.tsx` uses exactly the selected profile T02 bird names:

| Native face | Display | Native name |
| --- | --- | --- |
| Ph | 鵬 | Phoenix |
| Fa | 鷹 | Falcon |
| Ea | 鵰 | Eagle |
| Cr | 鶴 | Crane |
| Pt | 雉 | Pheasant |
| LQ | 左鶉 | Left Quail |
| RQ | 右鶉 | Right Quail |
| Sw | 燕 | Swallow |
| Go | 鴈 | Goose |

Single kanji use a large centered face; quails use two vertically arranged
characters, preserving explicit handedness. The warm pentagonal tile outline
and bevel reuse the project's existing `ShogiPiece` geometry. Ownership rotates
all of the tile and glyphs without exchanging Left Quail and Right Quail.
The native promoted flag supplies dark-red engraving plus two small corner
chevrons, so promotion is not conveyed by color alone. Eagle and Goose retain
their own proper names. The frontend neither infers promotion pairings nor
computes movement rules from these glyphs.

Accessible names come from the native board and hand presentation. Decorative
SVG content is hidden from assistive technology. Unknown native faces never
render as a different bird, a standard Shogi piece, or a raw diagnostic label.

## Bundled font and provenance

The default glyphs use **mShogi Tori Pieces Regular**, a separate 7,472-byte
WOFF2 subset of Noto Serif CJK JP Regular 2.003. This does not change the accepted
Shogi or Xiangqi subsets. The source is the same pinned official font documented
in [the Shogi typography audit](../shogi-font.md):

- Official upstream: [notofonts/noto-cjk](https://github.com/notofonts/noto-cjk).
- Source commit: `9b0f1436e455d902de067a2501422e5dc71ad16b` (Serif2.003).
- Input: [`Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf`](https://github.com/notofonts/noto-cjk/blob/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf), 24,573,864 bytes.
- Input SHA-256: `d9854c7a8ef170b5a7932558856fd64eb8de0b007cd823fed6f9f514ad2803d3`.
- Original copyright: © 2017–2024 Adobe (http://www.adobe.com/).
- License: SIL Open Font License 1.1, retained unchanged in `desktop/src/assets/fonts/OFL.txt` and the font metadata.
- Modified family: **mShogi Tori Pieces**; PostScript name: **mShogiToriPieces-Regular**.
- Output SHA-256: `aab996838bb6b9291206064fa4587f538877ce5a1192929224a29b45ff2a00b6`.

The renamed subset retains exactly these ten unique Unicode codepoints, with
14 total glyphs including dependencies:

```text
U+53F3 U+5DE6 U+71D5 U+96C9 U+9D08 U+9D6C U+9D70 U+9D89 U+9DB4 U+9DF9
```

The existing OFL and appended `NOTICE.txt` entry accompany the bundled font.
It is a local application resource, never a system-font installation, a remote
runtime request, or a new product dependency. The build-time audit used Python
3.11.0, fontTools 4.47.2 and Brotli 1.1.0, matching the existing font campaign.
The upstream font and temporary Brotli installation remain outside Git.

Reproduce with those tools available:

```sh
python3 desktop/src/assets/fonts/subset-tori.py \
  /tmp/NotoSerifCJKjp-Regular.otf \
  desktop/src/assets/fonts/mshogi-tori-pieces.woff2
```

The script verifies the pinned input SHA, source coverage, exact output Unicode
coverage, nonempty outlines for every character, renamed family/identity names,
and retained OFL metadata. Two actual generations were byte-identical (`cmp`),
with the output SHA shown above. The accepted Shogi and Xiangqi WOFF2 hashes and
the existing OFL hash remain unchanged. These font-table checks establish
coverage and reproducibility; actual application screenshots establish rendering.

## Dormant original alternative

`ToriBirdPiece.tsx` and `tori-birds.css` preserve the first MSH-42 original vector
bird engravings and small-cap identity aids as a possible future alternative.
The production board imports `ToriPiece`, not this component. No player-facing
selector, stored preference, or activation is included. Its class names are
isolated from the kanji default.

All bird, letter and promotion-mark paths in that alternative were drawn
specifically for mShogi as original project source. Tile geometry reuses the
existing project component. No third-party artwork, downloaded image, icon set
or additional asset license is involved. Its component tests preserve nine
explicit face identities, quail handedness, orientation and native promotion
presentation without establishing owner visual approval.

The finished presentation remains subject to review of genuine running
application captures at normal, narrow and minimum window sizes.
