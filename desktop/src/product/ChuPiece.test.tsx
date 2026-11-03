// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { ChuPiece } from "./ChuPiece";
import profile from "../../../docs/m4/chu/profile.json";

afterEach(cleanup);
// Native face identities are deliberately separate from base/promotion mapping.
const identities = ["Pawn", "GoBetween", "Lance", "ReverseChariot", "Copper", "Silver", "Gold", "Leopard", "Tiger", "Elephant", "King", "Kirin", "Phoenix", "SideMover", "VerticalMover", "Rook", "Bishop", "DragonHorse", "DragonKing", "Lion", "FreeKing", "WhiteHorse", "Whale", "FlyingStag", "Prince", "FlyingOx", "FreeBoar", "SoaringEagle", "HornedFalcon"];
it("uses all 29 independently sourced profile kanji faces without Latin substitutions", () => {
  expect(profile.pieces).toHaveLength(29);
  const { container } = render(<svg>{identities.map(face => <ChuPiece key={face} face={face} />)}</svg>);
  expect([...container.querySelectorAll(".chu-piece")].map(piece => piece.textContent)).toEqual(profile.pieces.map(piece => piece.kanji));
  expect(container.querySelector("image,use")).toBeNull();
});
it("rotates complete pentagonal artwork and distinguishes native King and Prince", () => {
  const { container } = render(<svg><ChuPiece face="King" /><ChuPiece face="Prince" promoted rotated /></svg>);
  const king = container.querySelector('[data-chu-face="King"]')!, prince = container.querySelector('[data-chu-face="Prince"]')!;
  expect(king.textContent).toBe("玉将");expect(prince.textContent).toBe("太子");
  expect(king.getAttribute("transform")).toBeNull();expect(prince.getAttribute("transform")).toBe("rotate(180 50 50)");
  expect(prince.querySelector(".chu-promotion-mark")).toBeTruthy();
  expect(king.querySelector(".chu-promotion-mark")).toBeNull();
});
it("takes promoted native face directly without promoting a derived face again", () => {
  const { container } = render(<svg><ChuPiece face="Gold" promoted /><ChuPiece face="Lion" promoted /></svg>);
  expect([...container.querySelectorAll(".chu-piece")].map(piece => piece.textContent)).toEqual(["金将", "獅子"]);
  expect(container.querySelectorAll(".chu-piece.promoted")).toHaveLength(2);
});
it.each(["__proto__", "unknown", "K", "DE"])("does not invent a native movement face for %s", face => {
  const { container } = render(<svg><ChuPiece face={face} /></svg>);expect(container.textContent).toBe("");
});
