"""Rebuild mShogi Xiangqi Pieces from the pinned Noto Serif CJK TC source (MSH-101 font audit)."""
import sys
from fontTools import subset
from fontTools.ttLib import TTFont

CHARS = "帥將仕士相象傌馬俥車炮砲兵卒楚河漢界"
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
    names = {1: "mShogi Xiangqi Pieces", 2: "Regular", 3: "mShogi Xiangqi Pieces 1.0", 4: "mShogi Xiangqi Pieces Regular", 6: "mShogiXiangqiPieces-Regular"}
    if record.nameID in names:
        record.string = names[record.nameID].encode(record.getEncoding())
if "CFF " in font:
    cff = font["CFF "].cff
    cff.fontNames = ["mShogiXiangqiPieces-Regular"]
    cff.topDictIndex[0].FamilyName = "mShogi Xiangqi Pieces"
    cff.topDictIndex[0].FullName = "mShogi Xiangqi Pieces Regular"
font.flavor = "woff2"
font.save(destination)
verified = TTFont(destination)
assert set(verified.getBestCmap()) == set(map(ord, CHARS))
print(f"Verified {len(set(CHARS))} codepoints; {len(verified.getGlyphOrder())} glyphs")
print(" ".join(f"U+{ord(char):04X}" for char in sorted(set(CHARS))))
