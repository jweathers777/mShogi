// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { ToriPiece } from "./ToriPiece";

afterEach(cleanup);
const faces = ["Ph", "Fa", "Ea", "Cr", "Pt", "LQ", "RQ", "Sw", "Go"];
it("gives all nine native faces their own original engraving and font-independent identity aid", () => {
  const { container } = render(<svg>{faces.map(kind => <ToriPiece key={kind} kind={kind} />)}</svg>);
  const pieces = [...container.querySelectorAll(".tori-piece")];
  expect(pieces).toHaveLength(9);
  expect(pieces.map(piece => piece.getAttribute("data-tori-face"))).toEqual(faces);
  expect(new Set(pieces.map(piece => piece.querySelector(".tori-bird")?.innerHTML)).size).toBe(9);
  expect(pieces.map(piece => piece.querySelector(".tori-identity")?.getAttribute("data-tori-mark"))).toEqual(["PH", "FA", "EA", "CR", "PT", "LQ", "RQ", "SW", "GO"]);
  expect(container.querySelector("text,image,use")).toBeNull();
  expect(pieces.every(piece => piece.getAttribute("aria-hidden") === "true")).toBe(true);
});
it("rotates the complete artwork for ownership, including the identity aid", () => {
  const { container, rerender } = render(<svg><ToriPiece kind="Ph" /></svg>);
  expect(container.querySelector(".tori-piece")?.getAttribute("transform")).toBeNull();
  rerender(<svg><ToriPiece kind="Ph" rotated /></svg>);
  expect(container.querySelector(".tori-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  expect(container.querySelector(".tori-identity")?.closest(".tori-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
});
it.each(["Ea", "Go"])("uses the authoritative promoted flag for %s, with a shape cue as well as color", kind => {
  const { container, rerender } = render(<svg><ToriPiece kind={kind} promoted /></svg>);
  expect(container.querySelector(".tori-piece.promoted")).toBeTruthy();
  expect(container.querySelector(".tori-promotion-mark")?.getAttribute("d")).toBeTruthy();
  expect(container.querySelector(".tori-piece")?.getAttribute("data-tori-face")).toBe(kind);
  rerender(<svg><ToriPiece kind={kind} /></svg>);
  expect(container.querySelector(".promoted,.tori-promotion-mark")).toBeNull();
});
it("keeps left and right quail distinct independently of owner orientation", () => {
  const { container } = render(<svg><ToriPiece kind="LQ" rotated /><ToriPiece kind="RQ" rotated /></svg>);
  const left = container.querySelector('[data-tori-face="LQ"]')!, right = container.querySelector('[data-tori-face="RQ"]')!;
  expect(left.querySelector(".tori-bird>g")?.getAttribute("transform")).toBeNull();
  expect(right.querySelector(".tori-bird>g")?.getAttribute("transform")).toBe("translate(100 0) scale(-1 1)");
  expect(left.querySelector(".tori-identity")?.getAttribute("data-tori-mark")).toBe("LQ");
  expect(right.querySelector(".tori-identity")?.getAttribute("data-tori-mark")).toBe("RQ");
  expect(left.getAttribute("transform")).toBe(right.getAttribute("transform"));
});
it.each(["pawn", "king", "__proto__", "unknown-face"])("does not invent an identity for unsupported native face %s", kind => {
  const { container } = render(<svg><ToriPiece kind={kind} /></svg>);
  expect(container.querySelector(".tori-piece")).toBeNull();
  expect(container.textContent).toBe("");
});
