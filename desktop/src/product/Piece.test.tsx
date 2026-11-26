// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { Piece, type PieceGame } from "./Piece";
import chuProfile from "../../../docs/m4/chu/profile.json";
import tenjikuProfile from "../../../docs/m4/tenjiku/profile.json";

afterEach(cleanup);
const piece = (face: string) => document.querySelector(`[data-face="${face}"]`)!;
const texts = (node: Element) => [...node.querySelectorAll("text")].map(text => text.textContent);

// Independent artwork expectations: profile lettering and the names cited in
// docs/m4/*/piece-artwork.md and desktop-art.md. They establish no movement rules.
const tori = [["Ph", "鵬"], ["Fa", "鷹"], ["Ea", "鵰"], ["Cr", "鶴"], ["Pt", "雉"], ["LQ", "左鶉"], ["RQ", "右鶉"], ["Sw", "燕"], ["Go", "鴈"]];
const chuFaces = ["Pawn", "GoBetween", "Lance", "ReverseChariot", "Copper", "Silver", "Gold", "Leopard", "Tiger", "Elephant", "King", "Kirin", "Phoenix", "SideMover", "VerticalMover", "Rook", "Bishop", "DragonHorse", "DragonKing", "Lion", "FreeKing", "WhiteHorse", "Whale", "FlyingStag", "Prince", "FlyingOx", "FreeBoar", "SoaringEagle", "HornedFalcon"];
const tenjiku = {
  K: "玉将", FiD: "火鬼", GGn: "大将", VGn: "副将", RGn: "飛将", BGn: "角将",
  LHk: "獅鷹", Ln: "獅子", WBf: "水牛", FEg: "奔鷲", FK: "奔王", HT: "四天王",
  SE: "飛鷲", HF: "角鷹", ChS: "車兵", DK: "龍王", DH: "龍馬", FO: "飛牛",
  FB: "奔猪", R: "飛車", B: "角行", Ph: "鳳凰", VSo: "竪兵", VM: "竪行",
  SSo: "横兵", SM: "横行", WH: "白駒", W: "鯨鯢", Ky: "麒麟", CP: "太子",
  FS: "飛鹿", RC: "反車", L: "香車", MGn: "雜将", G: "金将", DE: "酔象",
  BT: "盲虎", FL: "猛豹", S: "銀将", C: "銅将", I: "鉄将", N: "桂馬",
  Dg: "犬", P: "歩兵",
};

it("letters Shogi pieces by native kind, promotion and the first player's king", () => {
  render(<svg><Piece game="shogi" face="king" /><Piece game="shogi" face="rook" promoted /><Piece game="shogi" face="pawn" promoted /><Piece game="shogi" face="gold" promoted /></svg>);
  expect([...document.querySelectorAll(".piece-tile")].map(node => node.textContent)).toEqual(["玉将", "龍王", "と金", "金将"]);
  cleanup();
  render(<svg><Piece game="shogi" face="king" first={false} /></svg>);
  expect(piece("king").textContent).toBe("王将");
  // Shogi shows promotion by red lettering, without the shape mark of the other games.
  cleanup();
  render(<svg><Piece game="shogi" face="rook" promoted /></svg>);
  expect(piece("rook").classList.contains("promoted")).toBe(true);
  expect(piece("rook").querySelector(".piece-tile-mark")).toBeNull();
});
it("renders all nine selected-profile Tori bird names and keeps the quails explicit", () => {
  render(<svg>{tori.map(([face]) => <Piece key={face} game="tori" face={face!} rotated={face === "LQ" || face === "RQ"} />)}</svg>);
  expect(tori.map(([face]) => piece(face!).textContent)).toEqual(tori.map(([, name]) => name));
  expect(texts(piece("LQ"))).toEqual(["左", "鶉"]);
  expect(texts(piece("RQ"))).toEqual(["右", "鶉"]);
  expect(piece("LQ").getAttribute("transform")).toBe(piece("RQ").getAttribute("transform"));
});
it("uses all 29 independently sourced Chu profile faces", () => {
  expect(chuProfile.pieces).toHaveLength(29);
  render(<svg>{chuFaces.map(face => <Piece key={face} game="chu" face={face} />)}</svg>);
  expect([...document.querySelectorAll(".piece-tile")].map(node => node.textContent)).toEqual(chuProfile.pieces.map(p => p.kanji));
});
it("engraves every sourced Tenjiku face, including all eight promoted-only names", () => {
  expect(Object.keys(tenjiku).sort()).toEqual(tenjikuProfile.pieces.map(p => p.face).sort());
  render(<svg>{tenjikuProfile.pieces.map(({ face }) => <Piece key={face} game="tenjiku" face={face} />)}</svg>);
  for (const [face, name] of Object.entries(tenjiku)) expect(piece(face).textContent).toBe(name);
  expect(document.querySelectorAll(".piece-tile.tenjiku")).toHaveLength(44);
  // All reused Chu spellings are preserved; its Go-Between is absent in Tenjiku.
  expect(Object.values(tenjiku).filter(name => chuProfile.pieces.some(p => p.kanji === name))).toHaveLength(28);
  expect(document.body.textContent).not.toContain("仲人");
});
it("keeps the complete Heavenly Tetrarch name in three ordered glyphs and the Dog in one", () => {
  render(<svg><Piece game="tenjiku" face="HT" base="ChS" promoted /><Piece game="tenjiku" face="Dg" /></svg>);
  expect([...piece("HT").querySelectorAll("text")].map(node => [node.textContent, node.getAttribute("y")])).toEqual([["四", "33"], ["天", "63"], ["王", "93"]]);
  expect(piece("HT").classList.contains("glyphs-3")).toBe(true);
  expect(piece("Dg").classList.contains("glyphs-1")).toBe(true);
  expect(piece("Dg").textContent).toBe("犬");
});
it("takes every once-promoted native identity directly and keeps its original base", () => {
  const pairs = Object.entries(tenjikuProfile.promotion);
  expect(pairs).toHaveLength(30);
  render(<svg>{pairs.map(([base, face]) => <Piece key={base} game="tenjiku" base={base} face={face} promoted />)}</svg>);
  for (const [base, face] of pairs) {
    const node = document.querySelector(`[data-base="${base}"]`)!;
    expect(node.getAttribute("data-face")).toBe(face);
    expect(node.getAttribute("data-promoted")).toBe("true");
    expect(node.textContent).toBe(tenjiku[face as keyof typeof tenjiku]);
    expect(node.querySelector(".piece-tile-mark")).toBeTruthy();
  }
  cleanup();
  render(<svg><Piece game="chu" face="Gold" promoted /><Piece game="chu" face="Lion" promoted /></svg>);
  expect([...document.querySelectorAll(".piece-tile")].map(node => node.textContent)).toEqual(["金将", "獅子"]);
});
it.each([["tori", "Ea"], ["chu", "Prince"], ["tenjiku", "LHk"]] as const)("marks a promoted %s piece by shape as well as color and rotates the whole tile", (game, face) => {
  const { rerender } = render(<svg><Piece game={game} face={face} promoted rotated /></svg>);
  expect(piece(face).classList.contains("promoted")).toBe(true);
  expect(piece(face).querySelector(".piece-tile-mark")?.getAttribute("d")).toBeTruthy();
  expect(piece(face).getAttribute("transform")).toBe("rotate(180 50 50)");
  expect(piece(face).getAttribute("aria-hidden")).toBe("true");
  rerender(<svg><Piece game={game} face={face} /></svg>);
  expect(piece(face).querySelector(".piece-tile-mark")).toBeNull();
  expect(piece(face).getAttribute("transform")).toBeNull();
  expect(document.querySelector("image,use,title")).toBeNull();
});
it.each([
  ["shogi", "__proto__"], ["shogi", "king'"], ["tori", "pawn"], ["tori", "king"], ["chu", "K"], ["chu", "DE"],
  ["tenjiku", "King"], ["tenjiku", "GoBetween"], ["tenjiku", "unknown"],
] as [PieceGame, string][])("does not invent a %s face for %s", (game, face) => {
  const { container } = render(<svg><Piece game={game} face={face} /></svg>);
  expect(container.querySelector(".piece-tile")).toBeNull();
  expect(container.textContent).toBe("");
});
