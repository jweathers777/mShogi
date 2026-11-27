"""Rebuild a bundled piece font from the pinned Noto Serif CJK source."""

import argparse
import hashlib
from pathlib import Path

from fontTools import subset
from fontTools.pens.boundsPen import BoundsPen
from fontTools.ttLib import TTFont

JAPANESE = "d9854c7a8ef170b5a7932558856fd64eb8de0b007cd823fed6f9f514ad2803d3"
TRADITIONAL_CHINESE = "234301038e76e7c35c43113785024700c4e4fe7bdce1d1fbbc42fca7e6683798"

# Each entry gives the source hash, subset family, and complete set of characters.
FONTS = {
    "shogi": (JAPANESE, "mShogi Pieces", "玉王飛車角行金将銀桂馬香歩兵龍竜全圭杏と成二字"),
    "xiangqi": (TRADITIONAL_CHINESE, "mShogi Xiangqi Pieces", "帥將仕士相象傌馬俥車炮砲兵卒楚河漢界"),
    "tori": (JAPANESE, "mShogi Tori Pieces", "鵬鷹鵰鶴雉左鶉右燕鴈"),
    "chu": (JAPANESE, "mShogi Chu Pieces", "歩兵仲人香車反銅将銀金猛豹盲虎酔象玉麒麟鳳凰横行竪飛角龍馬王獅子奔白駒鯨鯢鹿太牛猪鷲鷹"),
    "tenjiku": (JAPANESE, "mShogi Tenjiku Pieces", "玉将火鬼大副飛角獅鷹子水牛奔鷲王四天車兵龍馬猪鳳凰竪行横白駒鯨鯢麒麟太鹿反香雜金酔象盲虎猛豹銀銅鉄桂犬歩"),
}


def check_outlines(font, codepoints):
    cmap = font.getBestCmap()
    glyphs = font.getGlyphSet()
    for codepoint in sorted(codepoints):
        pen = BoundsPen(glyphs)
        glyphs[cmap[codepoint]].draw(pen)
        if pen.bounds is None or pen.bounds[0] >= pen.bounds[2] or pen.bounds[1] >= pen.bounds[3]:
            raise ValueError(f"Empty outline for U+{codepoint:04X}")


def rebuild(source, destination, source_digest, family, characters):
    if hashlib.sha256(source.read_bytes()).hexdigest() != source_digest:
        raise ValueError("Source does not match the pinned Noto Serif CJK Regular 2.003")
    codepoints = set(map(ord, characters))
    postscript = family.replace(" ", "") + "-Regular"
    names = {1: family, 2: "Regular", 3: family + " 1.0", 4: family + " Regular", 6: postscript}
    with TTFont(source, recalcTimestamp=False) as font:
        missing = codepoints - font.getBestCmap().keys()
        if missing:
            raise ValueError(f"Source is missing glyphs: {missing}")
        check_outlines(font, codepoints)
        options = subset.Options()
        options.layout_features = []
        options.name_IDs = [0, 1, 2, 3, 4, 5, 6, 13, 14]
        options.name_legacy = True
        options.name_languages = [0x409]
        subsetter = subset.Subsetter(options=options)
        subsetter.populate(text=characters)
        subsetter.subset(font)
        for record in font["name"].names:
            if record.nameID in names:
                record.string = names[record.nameID].encode(record.getEncoding())
        cff = font["CFF "].cff
        cff.fontNames = [postscript]
        cff.topDictIndex[0].FamilyName = family
        cff.topDictIndex[0].FullName = names[4]
        font.flavor = "woff2"
        font.save(destination)

    with TTFont(destination, recalcTimestamp=False) as verified:
        if verified.getBestCmap().keys() != codepoints:
            raise ValueError(f"Subset Unicode coverage differs for {family}")
        check_outlines(verified, codepoints)
        records = verified["name"].names
        for name_id, name in names.items():
            if {record.toUnicode() for record in records if record.nameID == name_id} != {name}:
                raise ValueError(f"Unexpected font name {name_id} for {family}")
        if not any(record.nameID == 13 and "SIL" in record.toUnicode() for record in records):
            raise ValueError("SIL Open Font License metadata was not preserved")
        print(f"{family}: verified {len(codepoints)} codepoints and {len(verified.getGlyphOrder())} glyphs")
    contents = destination.read_bytes()
    print(f"{len(contents)} bytes; SHA256 {hashlib.sha256(contents).hexdigest()}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("game", choices=FONTS)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    args = parser.parse_args()
    rebuild(args.source, args.destination, *FONTS[args.game])


if __name__ == "__main__":
    main()
