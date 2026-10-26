# Bundled shogi piece typography

MSH-74 uses **mShogi Pieces Regular**, a 9,128-byte WOFF2 subset of
Noto Serif CJK JP Regular 2.003. The Japanese serif shapes are bundled so piece
rendering does not depend on a Japanese language pack being installed on Windows,
macOS or Linux. Native piece kind, promotion and ownership remain the semantic
identity; the font only renders the selected glyph. Accessible names use normal
application text. This subset is intended for piece faces, not general Japanese
text.

## Provenance and license

The official upstream repository is [notofonts/noto-cjk](https://github.com/notofonts/noto-cjk).
The [Serif2.003 release tag](https://github.com/notofonts/noto-cjk/releases/tag/Serif2.003)
resolves to commit `9b0f1436e455d902de067a2501422e5dc71ad16b`.
The pinned input is
[`Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf`](https://github.com/notofonts/noto-cjk/blob/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf),
24,573,864 bytes, SHA-256
`d9854c7a8ef170b5a7932558856fd64eb8de0b007cd823fed6f9f514ad2803d3`.
Its name table reports `Version 2.003;hotconv 1.1.1;makeotfexe 2.6.0` and
`© 2017-2024 Adobe (http://www.adobe.com/).`.

The [official SIL Open Font License 1.1](https://github.com/notofonts/noto-cjk/blob/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/LICENSE)
permits redistribution and modification subject to its conditions. The modified
font family and PostScript names are **mShogi Pieces**, avoiding reuse of the
upstream family name for the modified subset. The copyright and license metadata
are retained in the font. `desktop/src/assets/fonts/OFL.txt` contains the unchanged
upstream license; `NOTICE.txt` supplies the copyright, modification and source
notice. Both must accompany the bundled font in distributables. This is a bundled
application resource, never a system font installation or remote web request.

## Reproduction and coverage

Generation used Python 3.11, fontTools 4.47.2 and Brotli 1.1.0. The full upstream
font and temporary tool installations are not product dependencies and are not
checked into the repository. With those tools available:

```sh
curl --fail --location \
  https://raw.githubusercontent.com/notofonts/noto-cjk/9b0f1436e455d902de067a2501422e5dc71ad16b/Serif/OTF/Japanese/NotoSerifCJKjp-Regular.otf \
  --output /tmp/NotoSerifCJKjp-Regular.otf
python3 desktop/src/assets/fonts/subset.py \
  /tmp/NotoSerifCJKjp-Regular.otf \
  desktop/src/assets/fonts/mshogi-pieces.woff2
```

The script retains these 23 Unicode characters and verifies every one in the
reopened output: `玉王飛車角行金将銀桂馬香歩兵龍竜全圭杏と成二字`.
They cover the standard piece names, promoted piece forms and explicit promotion
wording; alternate spellings do not imply another game or rule profile.

```text
U+3068 U+4E8C U+5168 U+5175 U+572D U+5B57 U+5C06 U+6210
U+674F U+6842 U+6B69 U+7389 U+738B U+7ADC U+884C U+89D2
U+8ECA U+91D1 U+9280 U+98DB U+9999 U+99AC U+9F8D
```

The subset has 31 glyphs including dependencies; its output SHA-256 is
`c29fdb25d95d5699a90debc3e65871ef00022fa1e0a7a814852e928c08ab51e9`.
`OFL.txt` SHA-256 is
`6a73f9541c2de74158c0e7cf6b0a58ef774f5a780bf191f2d7ec9cc53efe2bf2`.
The script disables unused OpenType layout features and keeps the source timestamp
for reproducibility. It checks source coverage before generation and exact Unicode
coverage after writing.

The frontend should reference `mshogi-pieces.woff2` through a local `@font-face`
with family `mShogi Pieces`, weight 400 and format `woff2`. WOFF2 is the bundled
webview resource format; actual rendering evidence on each target platform belongs
to the packaged MSH-74 GUI verification, rather than being inferred from this
font-table check. Board SVG text should use the bundled face explicitly and may
fall back to `Noto Serif CJK JP`, `Yu Mincho`, `Hiragino Mincho ProN`, then `serif`.
