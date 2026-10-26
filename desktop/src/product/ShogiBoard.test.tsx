// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ShogiBoard } from "./ShogiBoard";
import type { GameSnapshot } from "./native";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
import promotionJson from "../__fixtures__/shared-shogi-promotion.json";
import forcedJson from "../__fixtures__/shared-shogi-forced.json";
import dropsJson from "../__fixtures__/shared-shogi-drops.json";
afterEach(cleanup);
const standard = standardJson as GameSnapshot;
function square(key: string) { const node = document.querySelector<SVGGElement>(`.shogi-square[data-square="${key}"]`); if (!node) throw Error(`Missing square ${key}`); return node; }
function board(snapshot = standard) { const onPlay = vi.fn(); const props = { snapshot, disabled: false, onPlay }; return { ...render(<ShogiBoard {...props} />), props, onPlay }; }
it("renders native identities independently of Japanese decoration and flips ownership correctly", () => {
  const { props, rerender } = board();
  expect(document.querySelectorAll(".shogi-square")).toHaveLength(81);
  expect(screen.getByRole("group", { name: "Shogi board, Sente at bottom" })).toBeTruthy();
  expect(square("5i").getAttribute("aria-label")).toBe("5i, Sente king");
  expect(square("5a").getAttribute("aria-label")).toBe("5a, Gote king");
  expect(square("5a").querySelector(".shogi-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  expect(square("5i").querySelector(".shogi-piece")?.getAttribute("transform")).toBeNull();
  expect(document.querySelectorAll(".hand-piece")).toHaveLength(0);
  const flipped = { ...standard, document: { ...standard.document, settings: { ...standard.document.settings, settings: { ...standard.document.settings.settings, orientation: "gote" } } } };
  rerender(<ShogiBoard {...props} snapshot={flipped} />);
  expect(screen.getByRole("group", { name: "Shogi board, Gote at bottom" })).toBeTruthy();
  expect(document.querySelector(".shogi-square")?.getAttribute("data-square")).toBe("1i");
  expect(square("5a").getAttribute("aria-label")).toBe("5a, Gote king");
  expect(square("5a").querySelector(".shogi-piece")?.getAttribute("transform")).toBeNull();
  expect(square("5i").querySelector(".shogi-piece")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  fireEvent.keyDown(square("5g"), { key: "ArrowDown" }); expect(document.activeElement).toBe(square("5f"));
});
it("submits exactly the native board choice and cancels partial keyboard selection", () => {
  const { onPlay } = board();
  square("7g").focus(); fireEvent.keyDown(square("7g"), { key: "Enter" });
  expect(square("7f").getAttribute("aria-label")).toContain("legal destination");
  expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square("7g"), { key: "Escape" });
  fireEvent.click(square("7f")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("7g")); fireEvent.click(square("7f"));
  expect(onPlay).toHaveBeenCalledExactlyOnceWith("7g7f");
});
it.each([["Promote", "5d5c+"], ["Do not promote", "5d5c"]])("keeps optional promotion atomic: %s", (label, expected) => {
  const { onPlay } = board(promotionJson as GameSnapshot);
  fireEvent.click(square("5d")); fireEvent.click(square("5c"));
  const dialog = screen.getByRole("dialog", { name: "Choose promotion" });
  expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(dialog, { key: "Escape" });
  expect(screen.queryByRole("dialog")).toBeNull(); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("5d")); fireEvent.click(square("5c"));
  fireEvent.click(screen.getByRole("button", { name: label })); expect(onPlay).toHaveBeenCalledExactlyOnceWith(expected);
});
it("commits a sole compulsory-promotion action without a redundant chooser", () => {
  const { onPlay } = board(forcedJson as GameSnapshot);
  fireEvent.click(square("5b")); fireEvent.click(square("5a"));
  expect(screen.queryByRole("dialog")).toBeNull(); expect(onPlay).toHaveBeenCalledExactlyOnceWith("5b5a+");
});
it("uses native nonzero hands/counts and keyboard drop selection without frontend legality", async () => {
  const { onPlay } = board(dropsJson as GameSnapshot); const user = userEvent.setup();
  expect(screen.queryByRole("button", { name: /Gote .* in hand/ })).toBeNull();
  expect(screen.getByRole("button", { name: "Sente rook in hand, 2" })).toBeTruthy();
  const pawn = screen.getByRole("button", { name: "Sente pawn in hand, 1" });
  pawn.focus(); await user.keyboard("{Enter}"); expect(onPlay).not.toHaveBeenCalled();
  expect(pawn.getAttribute("aria-pressed")).toBe("true");
  expect(document.querySelectorAll('.shogi-square[tabindex="0"]')).toHaveLength(1);
  fireEvent.keyDown(pawn, { key: "ArrowDown" }); expect(document.activeElement).toBe(square("5g"));
  fireEvent.keyDown(square("5g"), { key: "Escape" }); expect(document.activeElement).toBe(pawn);
  expect(onPlay).not.toHaveBeenCalled(); await user.keyboard("{Enter}");
  fireEvent.click(square("5e")); expect(onPlay).toHaveBeenCalledExactlyOnceWith("P*5e");
});
it("retires a pending selection when native context changes or interaction is disabled", () => {
  const { props, rerender, onPlay } = board(promotionJson as GameSnapshot);
  fireEvent.click(square("5d")); fireEvent.click(square("5c"));
  const snapshot = { ...props.snapshot, token: { ...props.snapshot.token, selection: props.snapshot.token.selection + 1 } };
  rerender(<ShogiBoard {...props} snapshot={snapshot} />);
  expect(screen.queryByRole("dialog")).toBeNull(); fireEvent.click(square("5c")); expect(onPlay).not.toHaveBeenCalled();
  rerender(<ShogiBoard {...props} snapshot={snapshot} disabled />);
  fireEvent.click(square("5d")); fireEvent.click(square("5c")); expect(onPlay).not.toHaveBeenCalled();
});
