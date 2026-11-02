// Dormant original artwork alternative. Traditional kanji in ToriPiece is the product default.
import type { ReactNode } from "react";
import "./tori-birds.css";

// Original presentation artwork. These are native faces, not promotion rules or
// movement diagrams. The controller supplies the face, promotion flag and owner.
const faces = ["Ph", "Fa", "Ea", "Cr", "Pt", "LQ", "RQ", "Sw", "Go"] as const;
type ToriFace = typeof faces[number];

function bird(face: ToriFace): ReactNode {
  switch (face) {
    case "Ph": return <>
      <path d="M50 19 45 12M50 19V10M50 19 55 12M43 26Q50 17 57 26L55 35 50 40 45 35Z" />
      <path d="M45 31Q32 19 22 25L38 42 29 39 41 51 50 58 59 51 71 39 62 42 78 25Q68 19 55 31M50 40V52M43 56 40 61M57 56 60 61" />
      <circle className="tori-bird-eye" cx="48" cy="27" r="1.5" />
    </>;
    case "Fa": return <>
      <path d="M35 57Q27 45 34 32L42 24Q49 19 58 24L68 25 67 31 59 30Q64 36 59 43L52 55 45 61 46 50Q36 45 39 34M42 34Q52 33 55 39L44 51M36 58H52M48 56 54 60" />
      <circle className="tori-bird-eye" cx="57" cy="27" r="1.7" />
    </>;
    case "Ea": return <>
      <path d="M42 28Q48 19 57 24L65 24 63 31 57 29 54 38 50 46 46 37Z" />
      <path d="M44 30 23 21 20 28 31 36 22 34 27 42 40 47 35 45 39 52 47 53 43 60 50 57 57 60 53 53 61 52 65 45 60 47 73 42 78 34 69 36 80 28 77 21 56 30M50 43V53" />
      <circle className="tori-bird-eye" cx="55" cy="26" r="1.5" />
    </>;
    case "Cr": return <>
      <path d="M35 45Q28 36 23 32L37 34Q47 36 52 47L59 39 60 24Q60 18 66 19L75 22 67 24 65 41Q63 49 54 53L43 54Q36 52 35 45ZM44 53 41 62M53 53 54 62M37 62H45M51 62H58M36 39 47 46" />
      <circle className="tori-bird-eye" cx="65" cy="22" r="1.4" />
    </>;
    case "Pt": return <>
      <path d="M25 24 41 38 20 31 39 43 23 39 39 49Q47 56 58 49L64 38 62 26Q62 21 68 21L75 24 68 27 69 38Q70 48 59 54L47 55M48 55 46 62M58 54 61 61M44 62H50M59 62H65M42 42Q49 36 57 41L50 50" />
      <circle className="tori-bird-eye" cx="67" cy="24" r="1.4" />
    </>;
    case "LQ":
    case "RQ": return <g transform={face === "RQ" ? "translate(100 0) scale(-1 1)" : undefined}>
      <path d="M33 55Q25 49 28 38L25 34 34 34Q38 24 47 24L51 18 54 25Q64 29 66 39L72 48 65 49Q61 58 48 58ZM37 40Q46 31 55 38L48 50 37 47ZM38 58 36 63M55 58 57 63M33 63H40M54 63H61" />
      <circle className="tori-bird-eye" cx="40" cy="30" r="1.8" />
    </g>;
    case "Sw": return <>
      <path d="M46 29 23 19 35 40 46 46 42 62 50 55 58 62 54 46 65 40 77 19 54 29 52 24 48 24ZM35 31 46 36M65 31 54 36M47 46H53" />
      <path d="M48 25 50 20 52 25" />
    </>;
    case "Go": return <>
      <path d="M27 43 22 36 39 40Q48 39 54 45L60 38 59 25Q59 19 66 19L76 24 68 26 66 39Q65 49 56 54L39 55Q30 52 27 43ZM36 45Q45 39 53 46L44 51M38 56 36 62M54 55 55 62M33 62H41M52 62H60" />
      <circle className="tori-bird-eye" cx="65" cy="22" r="1.5" />
    </>;
  }
}

// Letter strokes, rather than platform fonts, keep the engraved identity aids
// stable on macOS, Windows and Linux. Their meaning is the notation abbreviation.
const letters: Record<string, string> = {
  P: "M0 18V0H7Q13 0 13 5T7 10H0",
  H: "M0 0V18M13 0V18M0 9H13",
  F: "M0 18V0H13M0 8H10",
  A: "M0 18 6.5 0 13 18M3 11H10",
  E: "M13 0H0V18H13M0 9H10",
  C: "M13 2Q10 0 6.5 0Q0 0 0 9T6.5 18Q10 18 13 16",
  R: "M0 18V0H7Q13 0 13 5T7 10H0M7 10 14 18",
  T: "M0 0H14M7 0V18",
  L: "M0 0V18H13",
  Q: "M13 9Q13 0 6.5 0T0 9T6.5 18T13 9M8 13 15 20",
  S: "M13 2Q10 0 6.5 0T0 4.5Q0 8 6.5 9T13 13.5Q13 18 6.5 18T0 16",
  W: "M0 0 3 18 8 7 13 18 16 0",
  G: "M13 2Q10 0 6.5 0Q0 0 0 9T6.5 18Q10 18 13 16V10H7",
  O: "M13 9Q13 0 6.5 0T0 9T6.5 18T13 9Z",
};
const identityMarks: Record<ToriFace, string> = { Ph: "PH", Fa: "FA", Ea: "EA", Cr: "CR", Pt: "PT", LQ: "LQ", RQ: "RQ", Sw: "SW", Go: "GO" };

export function ToriBirdPiece({ kind, promoted = false, rotated = false }: { kind: string; promoted?: boolean; rotated?: boolean }) {
  const face = faces.find(candidate => candidate === kind);
  // Unknown native data must never silently become another bird or a Debug label.
  if (!face) return null;
  return <g className={`tori-bird-piece${promoted ? " promoted" : ""}`} data-tori-bird-face={face} transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="tori-bird-piece-shadow" d="M50 5 83 28 91 96 9 96 17 28Z" transform="translate(0 2)" />
    <path className="tori-bird-piece-body" d="M50 3 82 26 90 94 10 94 18 26Z" />
    <path className="tori-bird-piece-bevel" d="M50 7 78 29 85 89 15 89 22 29Z" />
    <g className="tori-bird-emblem" data-tori-bird-emblem={face}>{bird(face)}</g>
    <g className="tori-bird-identity" data-tori-bird-mark={identityMarks[face]} transform="translate(33 68)">
      {[...identityMarks[face]].map((letter, i) => <path key={i} d={letters[letter]} transform={`translate(${i * 20} 0)`} />)}
    </g>
    {promoted && <path className="tori-bird-promotion-mark" d="M20 68 24 64 28 68M72 68 76 64 80 68" />}
  </g>;
}
