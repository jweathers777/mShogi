import "./tenjiku.css";

// Traditional face lettering only; native identities supply face and promotion.
// Sources and exact font provenance are recorded in docs/m4/tenjiku/desktop-art.md.
const faces = {
  K: "玉将", FiD: "火鬼", GGn: "大将", VGn: "副将", RGn: "飛将", BGn: "角将",
  LHk: "獅鷹", Ln: "獅子", WBf: "水牛", FEg: "奔鷲", FK: "奔王", HT: "四天王",
  SE: "飛鷲", HF: "角鷹", ChS: "車兵", DK: "龍王", DH: "龍馬", FO: "飛牛",
  FB: "奔猪", R: "飛車", B: "角行", Ph: "鳳凰", VSo: "竪兵", VM: "竪行",
  SSo: "横兵", SM: "横行", WH: "白駒", W: "鯨鯢", Ky: "麒麟", CP: "太子",
  FS: "飛鹿", RC: "反車", L: "香車", MGn: "雜将", G: "金将", DE: "酔象",
  BT: "盲虎", FL: "猛豹", S: "銀将", C: "銅将", I: "鉄将", N: "桂馬",
  Dg: "犬", P: "歩兵",
} as const;

export function TenjikuPiece({ face, base, promoted = false, rotated = false, owner, royal = false }: {
  face: string; base: string; promoted?: boolean; rotated?: boolean; owner?: string; royal?: boolean;
}) {
  if (!Object.hasOwn(faces, face)) return null;
  const glyphs = [...faces[face as keyof typeof faces]];
  return <g className={`tenjiku-piece${promoted ? " promoted" : ""}`}
    data-tenjiku-piece={base} data-tenjiku-face={face} data-promoted={promoted} data-owner={owner} data-royal={royal}
    transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="tenjiku-piece-shadow" d="M50 3 88 23 95 98 5 98 12 23Z" />
    <path className="tenjiku-piece-body" d="M50 1 86 21 93 96 7 96 14 21Z" />
    <path className="tenjiku-piece-bevel" d="M50 5 82 24 89 92 11 92 18 24Z" />
    <g className={`tenjiku-face tenjiku-face-${glyphs.length}`}>
      {glyphs.map((glyph, index) => <text key={index} x="50"
        y={glyphs.length === 1 ? 78 : glyphs.length === 3 ? 33 + index * 30 : 47 + index * 41}
        textAnchor="middle">{glyph}</text>)}
    </g>
    {promoted && <path className="tenjiku-promotion-mark" d="M12 84 15 80 18 84M82 84 85 80 88 84" />}
  </g>;
}
