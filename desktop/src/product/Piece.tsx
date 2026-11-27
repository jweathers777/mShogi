import "./pieces.css";

// Decoration only: one pentagonal tile for the four shogi-family games, with
// each game's traditional face lettering. Data supplies face, promotion
// and ownership; these tables are never promotion pairings or movement rules.
// Sources and font provenance: docs/m4/tori/piece-artwork.md and
// docs/m4/tenjiku/desktop-art.md.
export type PieceGame = "shogi" | "tori" | "chu" | "tenjiku";
const faces: Record<PieceGame, Record<string, string>> = {
  // Shogi names its pieces by kind; promotion changes the lettering.
  shogi: { king: "玉将", rook: "飛車", bishop: "角行", gold: "金将", silver: "銀将", knight: "桂馬", lance: "香車", pawn: "歩兵" },
  // The selected profile T02 names.
  tori: { Ph: "鵬", Fa: "鷹", Ea: "鵰", Cr: "鶴", Pt: "雉", LQ: "左鶉", RQ: "右鶉", Sw: "燕", Go: "鴈" },
  chu: {
    Pawn: "歩兵", GoBetween: "仲人", Lance: "香車", ReverseChariot: "反車",
    Copper: "銅将", Silver: "銀将", Gold: "金将", Leopard: "猛豹", Tiger: "盲虎",
    Elephant: "酔象", King: "玉将", Kirin: "麒麟", Phoenix: "鳳凰",
    SideMover: "横行", VerticalMover: "竪行", Rook: "飛車", Bishop: "角行",
    DragonHorse: "龍馬", DragonKing: "龍王", Lion: "獅子", FreeKing: "奔王",
    WhiteHorse: "白駒", Whale: "鯨鯢", FlyingStag: "飛鹿", Prince: "太子",
    FlyingOx: "飛牛", FreeBoar: "奔猪", SoaringEagle: "飛鷲", HornedFalcon: "角鷹",
  },
  tenjiku: {
    K: "玉将", FiD: "火鬼", GGn: "大将", VGn: "副将", RGn: "飛将", BGn: "角将",
    LHk: "獅鷹", Ln: "獅子", WBf: "水牛", FEg: "奔鷲", FK: "奔王", HT: "四天王",
    SE: "飛鷲", HF: "角鷹", ChS: "車兵", DK: "龍王", DH: "龍馬", FO: "飛牛",
    FB: "奔猪", R: "飛車", B: "角行", Ph: "鳳凰", VSo: "竪兵", VM: "竪行",
    SSo: "横兵", SM: "横行", WH: "白駒", W: "鯨鯢", Ky: "麒麟", CP: "太子",
    FS: "飛鹿", RC: "反車", L: "香車", MGn: "雜将", G: "金将", DE: "酔象",
    BT: "盲虎", FL: "猛豹", S: "銀将", C: "銅将", I: "鉄将", N: "桂馬",
    Dg: "犬", P: "歩兵",
  },
};
// Chu and Tenjiku tiles fill more of their dense boards' squares.
const shapes = {
  standard: {
    shadow: "M50 5 83 28 91 96 9 96 17 28Z", shadowShift: "translate(0 2)", body: "M50 3 82 26 90 94 10 94 18 26Z",
    bevel: "M50 7 78 29 85 89 15 89 22 29Z", mark: "M20 86 24 82 28 86M72 86 76 82 80 86", lines: [[], [77], [47, 81]],
  },
  large: {
    shadow: "M50 3 88 23 95 98 5 98 12 23Z", shadowShift: undefined, body: "M50 1 86 21 93 96 7 96 14 21Z",
    bevel: "M50 5 82 24 89 92 11 92 18 24Z", mark: "M12 84 15 80 18 84M82 84 85 80 88 84", lines: [[], [78], [47, 88], [33, 63, 93]],
  },
};
const shogiPromoted: Record<string, string> = { rook: "龍王", bishop: "龍馬", silver: "成銀", knight: "成桂", lance: "成香", pawn: "と金" };
function lettering(game: PieceGame, face: string, promoted: boolean, first: boolean) {
  if (!Object.hasOwn(faces[game], face)) return null;
  if (game === "shogi" && promoted && Object.hasOwn(shogiPromoted, face)) return shogiPromoted[face]!;
  if (game === "shogi" && face === "king" && !first) return "王将";
  return faces[game][face]!;
}

/** A shogi-family piece in a 100×100 cell; `first` names the first player's king in Shogi. */
export function Piece({ game, face, base = face, promoted = false, rotated = false, first = true }: {
  game: PieceGame; face: string; base?: string; promoted?: boolean; rotated?: boolean; first?: boolean;
}) {
  const name = lettering(game, face, promoted, first);
  if (name === null) return null;
  const glyphs = [...name];
  const shape = game === "chu" || game === "tenjiku" ? "large" : "standard";
  const outline = shapes[shape];
  return <g className={`piece-tile ${game} ${shape} glyphs-${glyphs.length}${promoted ? " promoted" : ""}`}
    data-face={face} data-base={base} data-promoted={promoted}
    transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="piece-tile-shadow" d={outline.shadow} transform={outline.shadowShift} />
    <path className="piece-tile-body" d={outline.body} />
    <path className="piece-tile-bevel" d={outline.bevel} />
    <g className="piece-tile-face">
      {glyphs.map((glyph, index) => <text key={index} x="50" y={outline.lines[glyphs.length]?.[index]} textAnchor="middle">{glyph}</text>)}
    </g>
    {promoted && game !== "shogi" && <path className="piece-tile-mark" d={outline.mark} />}
  </g>;
}
