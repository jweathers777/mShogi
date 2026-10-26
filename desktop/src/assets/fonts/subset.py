"""Rebuild mShogi Pieces from the pinned Noto Serif CJK JP source (see shogi-font.md)."""
import sys
from fontTools import subset
from fontTools.ttLib import TTFont

CHARS = "玉王飛車角行金将銀桂馬香歩兵龍竜全圭杏と成二字"
source, destination = sys.argv[1:]
font = TTFont(source, recalcTimestamp=False)
missing = set(map(ord, CHARS)) - font.getBestCmap().keys()
if missing:
    raise ValueError(f"Source is missing glyphs: {missing}")
options = subset.Options()
options.layout_features = []
options.name_IDs = [0, 1, 2, 3, 4, 5, 6, 13, 14]
options.name_legacy = True
options.name_languages = [0x409]
subsetter = subset.Subsetter(options=options)
subsetter.populate(text=CHARS)
subsetter.subset(font)
# Use a distinct name for this modified subset, leaving upstream credit/license intact.
for record in font["name"].names:
    names = {1: "mShogi Pieces", 2: "Regular", 3: "mShogi Pieces 1.0", 4: "mShogi Pieces Regular", 6: "mShogiPieces-Regular"}
    if record.nameID in names:
        record.string = names[record.nameID].encode(record.getEncoding())
if "CFF " in font:
    cff = font["CFF "].cff
    cff.fontNames = ["mShogiPieces-Regular"]
    cff.topDictIndex[0].FamilyName = "mShogi Pieces"
    cff.topDictIndex[0].FullName = "mShogi Pieces Regular"
font.flavor = "woff2"
font.save(destination)
verified = TTFont(destination)
assert set(verified.getBestCmap()) == set(map(ord, CHARS))
print(f"Verified {len(set(CHARS))} codepoints; {len(verified.getGlyphOrder())} glyphs")
print(" ".join(f"U+{ord(char):04X}" for char in sorted(set(CHARS))))
