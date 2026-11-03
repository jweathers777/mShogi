import "./chu.css";

// The profile's traditional face names are artwork, never a promotion table.
// The authoritative native selected-position identity supplies `face` directly.
const faces = {
  Pawn: "歩兵", GoBetween: "仲人", Lance: "香車", ReverseChariot: "反車",
  Copper: "銅将", Silver: "銀将", Gold: "金将", Leopard: "猛豹", Tiger: "盲虎",
  Elephant: "酔象", King: "玉将", Kirin: "麒麟", Phoenix: "鳳凰",
  SideMover: "横行", VerticalMover: "竪行", Rook: "飛車", Bishop: "角行",
  DragonHorse: "龍馬", DragonKing: "龍王", Lion: "獅子", FreeKing: "奔王",
  WhiteHorse: "白駒", Whale: "鯨鯢", FlyingStag: "飛鹿", Prince: "太子",
  FlyingOx: "飛牛", FreeBoar: "奔猪", SoaringEagle: "飛鷲", HornedFalcon: "角鷹",
} as const;

export function ChuPiece({ face, promoted = false, rotated = false }: {
  face: string; promoted?: boolean; rotated?: boolean;
}) {
  if (!Object.hasOwn(faces, face)) return null;
  const glyphs = [...faces[face as keyof typeof faces]];
  return <g className={`chu-piece${promoted ? " promoted" : ""}`} data-chu-face={face}
    transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="chu-piece-shadow" d="M50 3 88 23 95 98 5 98 12 23Z" />
    <path className="chu-piece-body" d="M50 1 86 21 93 96 7 96 14 21Z" />
    <path className="chu-piece-bevel" d="M50 5 82 24 89 92 11 92 18 24Z" />
    {glyphs.map((glyph, index) => <text key={index} x="50" y={47 + index * 41} textAnchor="middle">{glyph}</text>)}
    {promoted && <path className="chu-promotion-mark" d="M12 84 15 80 18 84M82 84 85 80 88 84" />}
  </g>;
}
