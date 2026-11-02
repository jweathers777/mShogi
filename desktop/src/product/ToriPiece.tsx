import "./tori.css";

// Decoration only. These are the selected profile T02 names, not promotion
// pairings or movement rules. Native data supplies face, promotion and ownership.
const faces = {
  Ph: "鵬", Fa: "鷹", Ea: "鵰", Cr: "鶴", Pt: "雉",
  LQ: "左鶉", RQ: "右鶉", Sw: "燕", Go: "鴈",
} as const;

export function ToriPiece({ kind, promoted = false, rotated = false }: { kind: string; promoted?: boolean; rotated?: boolean }) {
  if (!Object.hasOwn(faces, kind)) return null;
  const glyphs = [...faces[kind as keyof typeof faces]];
  return <g className={`tori-piece${promoted ? " promoted" : ""}`} data-tori-face={kind} transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="tori-piece-shadow" d="M50 5 83 28 91 96 9 96 17 28Z" transform="translate(0 2)" />
    <path className="tori-piece-body" d="M50 3 82 26 90 94 10 94 18 26Z" />
    <path className="tori-piece-bevel" d="M50 7 78 29 85 89 15 89 22 29Z" />
    <g className={`tori-face${glyphs.length === 2 ? " tori-face-pair" : ""}`}>
      {glyphs.map((glyph, index) => <text key={index} x="50" y={glyphs.length === 2 ? 45 + index * 35 : 77} textAnchor="middle">{glyph}</text>)}
    </g>
    {promoted && <path className="tori-promotion-mark" d="M20 86 24 82 28 86M72 86 76 82 80 86" />}
  </g>;
}
