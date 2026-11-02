"""Build the isolated mShogi Tori Pieces subset; see docs/m4/tori/piece-artwork.md."""
import hashlib
import sys

from fontTools import subset
from fontTools.pens.boundsPen import BoundsPen
from fontTools.ttLib import TTFont

CHARS = "鵬鷹鵰鶴雉左鶉右燕鴈"
SOURCE_SHA256 = "d9854c7a8ef170b5a7932558856fd64eb8de0b007cd823fed6f9f514ad2803d3"
FAMILY = "mShogi Tori Pieces"
POSTSCRIPT = "mShogiToriPieces-Regular"


def check_outlines(font):
    cmap = font.getBestCmap()
    glyphs = font.getGlyphSet()
    for character in CHARS:
        pen = BoundsPen(glyphs)
        glyphs[cmap[ord(character)]].draw(pen)
        if pen.bounds is None or pen.bounds[0] >= pen.bounds[2] or pen.bounds[1] >= pen.bounds[3]:
            raise ValueError(f"Empty outline for U+{ord(character):04X}")


def main():
    source, destination = sys.argv[1:]
    with open(source, "rb") as source_file:
        if hashlib.sha256(source_file.read()).hexdigest() != SOURCE_SHA256:
            raise ValueError("Source does not match pinned Noto Serif CJK JP Regular 2.003")
    font = TTFont(source, recalcTimestamp=False)
    missing = set(map(ord, CHARS)) - font.getBestCmap().keys()
    if missing:
        raise ValueError(f"Source is missing glyphs: {missing}")
    check_outlines(font)
    options = subset.Options()
    options.layout_features = []
    options.name_IDs = [0, 1, 2, 3, 4, 5, 6, 13, 14]
    options.name_legacy = True
    options.name_languages = [0x409]
    subsetter = subset.Subsetter(options=options)
    subsetter.populate(text=CHARS)
    subsetter.subset(font)
    names = {
        1: FAMILY, 2: "Regular", 3: FAMILY + " 1.0", 4: FAMILY + " Regular", 6: POSTSCRIPT,
    }
    for record in font["name"].names:
        if record.nameID in names:
            record.string = names[record.nameID].encode(record.getEncoding())
    if "CFF " in font:
        cff = font["CFF "].cff
        cff.fontNames = [POSTSCRIPT]
        cff.topDictIndex[0].FamilyName = FAMILY
        cff.topDictIndex[0].FullName = FAMILY + " Regular"
    font.flavor = "woff2"
    font.save(destination)
    verified = TTFont(destination, recalcTimestamp=False)
    if set(verified.getBestCmap()) != set(map(ord, CHARS)):
        raise ValueError("Subset Unicode coverage differs from the ten selected Tori characters")
    check_outlines(verified)
    for name_id in (1, 3, 4, 6):
        if {record.toUnicode() for record in verified["name"].names if record.nameID == name_id} != {names[name_id]}:
            raise ValueError(f"Unrenamed font name {name_id}")
    if not any(record.nameID == 13 and "SIL" in record.toUnicode() for record in verified["name"].names):
        raise ValueError("SIL Open Font License metadata was not preserved")
    print(f"Verified {len(set(CHARS))} codepoints; {len(verified.getGlyphOrder())} glyphs; all outlines nonempty; family renamed")
    print(" ".join(f"U+{ord(char):04X}" for char in sorted(set(CHARS))))
    with open(destination, "rb") as output:
        contents = output.read()
    print(f"{len(contents)} bytes; SHA256 {hashlib.sha256(contents).hexdigest()}")


if __name__ == "__main__":
    main()
