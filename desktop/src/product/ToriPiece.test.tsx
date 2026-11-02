// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { ToriPiece } from "./ToriPiece";

afterEach(cleanup);
const expectedFaces = [
  ["Ph", "鵬"], ["Fa", "鷹"], ["Ea", "鵰"], ["Cr", "鶴"], ["Pt", "雉"],
  ["LQ", "左鶉"], ["RQ", "右鶉"], ["Sw", "燕"], ["Go", "鴈"],
];
it("renders all nine selected-profile bird names without substituting standard Shogi glyphs", () => {
  const { container } = render(<svg>{expectedFaces.map(([kind]) => <ToriPiece key={kind} kind={kind!} />)}</svg>);
  const pieces = [...container.querySelectorAll(".tori-piece")];
  expect(pieces).toHaveLength(9);
  expect(pieces.map(piece => [piece.getAttribute("data-tori-face"), piece.textContent])).toEqual(expectedFaces);
  expect(container.querySelector("image,use")).toBeNull();
  expect(pieces.every(piece => piece.getAttribute("aria-hidden") === "true")).toBe(true);
  expect(new Set(pieces.map(piece => piece.textContent)).size).toBe(9);
});
it("rotates the complete artwork for ownership, including its kanji", () => {
  const { container, rerender } = render(<svg><ToriPiece kind="Ph" /></svg>);
  expect(container.querySelector(".tori-piece")?.getAttribute("transform")).toBeNull();
  rerender(<svg><ToriPiece kind="Ph" rotated /></svg>);
  expect(container.querySelector(".tori-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  expect(container.querySelector("text")?.closest(".tori-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
});
it.each(["Ea", "Go"])("uses the authoritative promoted flag for %s, with a shape cue as well as color", kind => {
  const { container, rerender } = render(<svg><ToriPiece kind={kind} promoted /></svg>);
  expect(container.querySelector(".tori-piece.promoted")).toBeTruthy();
  expect(container.querySelector(".tori-promotion-mark")?.getAttribute("d")).toBeTruthy();
  expect(container.querySelector(".tori-piece")?.getAttribute("data-tori-face")).toBe(kind);
  rerender(<svg><ToriPiece kind={kind} /></svg>);
  expect(container.querySelector(".promoted,.tori-promotion-mark")).toBeNull();
});
it("keeps left and right quail identities explicit in both ownership orientations", () => {
  const { container } = render(<svg><ToriPiece kind="LQ" rotated /><ToriPiece kind="RQ" rotated /></svg>);
  const left = container.querySelector('[data-tori-face="LQ"]')!, right = container.querySelector('[data-tori-face="RQ"]')!;
  expect([...left.querySelectorAll("text")].map(node => node.textContent)).toEqual(["左", "鶉"]);
  expect([...right.querySelectorAll("text")].map(node => node.textContent)).toEqual(["右", "鶉"]);
  expect([...left.querySelectorAll("text")].map(node => node.getAttribute("y"))).toEqual(["45", "80"]);
  expect(left.getAttribute("transform")).toBe(right.getAttribute("transform"));
});
it.each(["pawn", "king", "__proto__", "unknown-face"])("does not invent an identity for unsupported native face %s", kind => {
  const { container } = render(<svg><ToriPiece kind={kind} /></svg>);
  expect(container.querySelector(".tori-piece")).toBeNull();
  expect(container.textContent).toBe("");
});
