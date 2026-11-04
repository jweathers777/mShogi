// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { TenjikuPiece } from "./TenjikuPiece";
import profile from "../../../docs/m4/tenjiku/profile.json";
import chu from "../../../docs/m4/chu/profile.json";

afterEach(cleanup);
// Independent artwork expectations: accepted Chu profile lettering plus the
// Japanese names cited in desktop-art.md. These do not establish movement rules.
const expected = {
  K: "玉将", FiD: "火鬼", GGn: "大将", VGn: "副将", RGn: "飛将", BGn: "角将",
  LHk: "獅鷹", Ln: "獅子", WBf: "水牛", FEg: "奔鷲", FK: "奔王", HT: "四天王",
  SE: "飛鷲", HF: "角鷹", ChS: "車兵", DK: "龍王", DH: "龍馬", FO: "飛牛",
  FB: "奔猪", R: "飛車", B: "角行", Ph: "鳳凰", VSo: "竪兵", VM: "竪行",
  SSo: "横兵", SM: "横行", WH: "白駒", W: "鯨鯢", Ky: "麒麟", CP: "太子",
  FS: "飛鹿", RC: "反車", L: "香車", MGn: "雜将", G: "金将", DE: "酔象",
  BT: "盲虎", FL: "猛豹", S: "銀将", C: "銅将", I: "鉄将", N: "桂馬",
  Dg: "犬", P: "歩兵",
};

it("engraves every sourced native face, including all eight promoted-only names", () => {
  expect(Object.keys(expected).sort()).toEqual(profile.pieces.map(piece => piece.face).sort());
  const { container } = render(<svg>{profile.pieces.map(({ face }) => <TenjikuPiece key={face} base={face} face={face} />)}</svg>);
  for (const [face, name] of Object.entries(expected)) {
    const piece = container.querySelector(`[data-tenjiku-face="${face}"]`)!;
    expect(piece.textContent).toBe(name);
    expect(piece.querySelector(".tenjiku-piece-body")).toBeTruthy();
    expect(piece.getAttribute("aria-hidden")).toBe("true");
  }
  expect(container.querySelectorAll("[data-tenjiku-face]")).toHaveLength(44);
  expect(container.querySelector("image,use,title")).toBeNull();
  // All reused Chu spellings are preserved; its Go-Between is absent in Tenjiku.
  const shared = Object.values(expected).filter(name => chu.pieces.some(piece => piece.kanji === name));
  expect(shared).toHaveLength(28);
  expect(container.textContent).not.toContain("仲人");
});

it("keeps the complete Heavenly Tetrarch name in three ordered glyphs and the Dog in one", () => {
  const { container } = render(<svg><TenjikuPiece face="HT" base="ChS" promoted /><TenjikuPiece face="Dg" base="Dg" /></svg>);
  const tetrarch = container.querySelector('[data-tenjiku-face="HT"]')!;
  expect([...tetrarch.querySelectorAll("text")].map(node => [node.textContent, node.getAttribute("y")]))
    .toEqual([["四", "33"], ["天", "63"], ["王", "93"]]);
  expect(tetrarch.querySelector(".tenjiku-face-3")).toBeTruthy();
  expect(container.querySelector('[data-tenjiku-face="Dg"] .tenjiku-face-1')?.textContent).toBe("犬");
});

it("retains native original identity and a non-color promotion mark when movement lettering matches", () => {
  const { container } = render(<svg>
    <TenjikuPiece face="LHk" base="LHk" owner="sente" />
    <TenjikuPiece face="LHk" base="Ln" owner="gote" promoted rotated />
    <TenjikuPiece face="FEg" base="FEg" />
    <TenjikuPiece face="FEg" base="FK" promoted />
    <TenjikuPiece face="G" base="P" promoted />
  </svg>);
  const original = container.querySelector('[data-tenjiku-piece="LHk"]')!;
  const promoted = container.querySelector('[data-tenjiku-piece="Ln"]')!;
  expect(original.textContent).toBe("獅鷹"); expect(promoted.textContent).toBe("獅鷹");
  expect(original.querySelector(".tenjiku-promotion-mark")).toBeNull();
  expect(promoted.querySelector(".tenjiku-promotion-mark")).toBeTruthy();
  expect(promoted.classList.contains("promoted")).toBe(true);
  expect(promoted.getAttribute("data-owner")).toBe("gote");
  expect(original.getAttribute("transform")).toBeNull();
  expect(promoted.getAttribute("transform")).toBe("rotate(180 50 50)");
  expect([...container.querySelectorAll('[data-tenjiku-face="FEg"]')].map(node => node.textContent)).toEqual(["奔鷲", "奔鷲"]);
  expect(container.querySelector('[data-tenjiku-piece="P"]')?.textContent).toBe("金将");
});

it("takes every once-promoted native identity directly without promoting its movement face again", () => {
  const pairs = Object.entries(profile.promotion);
  expect(pairs).toHaveLength(30);
  const { container } = render(<svg>{pairs.map(([base, face]) => <TenjikuPiece key={base} base={base} face={face} promoted />)}</svg>);
  for (const [base, face] of pairs) {
    const piece = container.querySelector(`[data-tenjiku-piece="${base}"]`)!;
    expect(piece.getAttribute("data-tenjiku-face")).toBe(face);
    expect(piece.textContent).toBe(expected[face as keyof typeof expected]);
    expect(piece.querySelector(".tenjiku-promotion-mark")).toBeTruthy();
  }
});

it("preserves distinct King and Prince names and native royalty metadata", () => {
  const { container } = render(<svg><TenjikuPiece face="K" base="K" royal /><TenjikuPiece face="CP" base="DE" royal promoted rotated /></svg>);
  expect([...container.querySelectorAll('[data-royal="true"]')].map(node => node.textContent)).toEqual(["玉将", "太子"]);
  expect(container.querySelector('[data-tenjiku-piece="DE"]')?.getAttribute("data-promoted")).toBe("true");
});

it.each(["__proto__", "unknown", "King", "GoBetween"])("does not substitute another piece for unknown native face %s", face => {
  const { container } = render(<svg><TenjikuPiece face={face} base="K" /></svg>);
  expect(container.querySelector("[data-tenjiku-face]")).toBeNull();
  expect(container.textContent).toBe("");
});
