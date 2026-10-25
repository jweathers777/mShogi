// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { Board } from "./Board";
import userEvent from "@testing-library/user-event";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import promotionJson from "./__fixtures__/promotion.json";
const standard = standardJson as Snapshot;
const promotion = promotionJson as Snapshot;
afterEach(cleanup);
function board(snapshot = standard) {
  const onPlay = vi.fn();
  const props = {
    pieces: snapshot.pieces,
    choices: snapshot.legal_choices,
    version: snapshot.version,
    disabled: false,
    orientation: "white" as const,
    onPlay,
  };
  return { ...render(<Board {...props} />), props, onPlay };
}
function square(name: string) {
  return screen.getByRole("button", { name: new RegExp(`^${name},`) });
}
describe("native complete choice selection", () => {
  it("sends only a supplied native action, not an unavailable destination", () => {
    const { onPlay } = board();
    fireEvent.click(square("e2"));
    expect(square("e4").getAttribute("aria-label")).toContain(
      "legal destination",
    );
    fireEvent.click(square("e5"));
    expect(onPlay).not.toHaveBeenCalled();
    fireEvent.click(square("e4"));
    expect(onPlay).toHaveBeenCalledExactlyOnceWith("e2e4");
  });
  it("shows four native promotion choices and commits only the explicitly selected piece", () => {
    const { onPlay } = board(promotion);
    fireEvent.click(square("a7"));
    fireEvent.click(square("a8"));
    expect(onPlay).not.toHaveBeenCalled();
    const queen = screen.getByRole("button", { name: "queen" });
    expect(document.activeElement).toBe(queen);
    for (const name of ["queen", "rook", "bishop", "knight"])
      expect(screen.getByRole("button", { name })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "knight" }));
    expect(onPlay).toHaveBeenCalledExactlyOnceWith("a7a8n");
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(document.activeElement).toBe(square("a8"));
  });
  it("Escape cancels a promotion without a game command and returns board focus", () => {
    const { onPlay } = board(promotion);
    fireEvent.click(square("a7"));
    fireEvent.click(square("a8"));
    fireEvent.keyDown(screen.getByRole("button", { name: "queen" }), {
      key: "Escape",
    });
    expect(onPlay).not.toHaveBeenCalled();
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(document.activeElement).toBe(square("a8"));
  });
  it("keeps all promotion choices and cancellation reachable by keyboard", async () => {
    const { onPlay } = board(promotion);
    fireEvent.click(square("a7"));
    fireEvent.click(square("a8"));
    fireEvent.keyDown(screen.getByRole("button", { name: "queen" }), {
      key: "Tab",
      shiftKey: true,
    });
    expect(document.activeElement).toBe(
      screen.getByRole("button", { name: "Cancel promotion" }),
    );
    fireEvent.keyDown(
      screen.getByRole("button", { name: "Cancel promotion" }),
      { key: "Tab" },
    );
    expect(document.activeElement).toBe(
      screen.getByRole("button", { name: "queen" }),
    );
    fireEvent.keyDown(screen.getByRole("button", { name: "queen" }), {
      key: "Tab",
    });
    expect(document.activeElement).toBe(
      screen.getByRole("button", { name: "rook" }),
    );
    expect(onPlay).not.toHaveBeenCalled();
    await userEvent.setup().keyboard("{Enter}");
    expect(onPlay).toHaveBeenCalledExactlyOnceWith("a7a8r");
    expect(document.activeElement).toBe(square("a8"));
  });
  it("moves focus by visual orientation and supports Enter, Space, and Escape", () => {
    const { onPlay, rerender, props } = board();
    square("e2").focus();
    fireEvent.keyDown(square("e2"), { key: "Enter" });
    fireEvent.keyDown(square("e2"), { key: "ArrowUp" });
    expect(document.activeElement).toBe(square("e3"));
    fireEvent.keyDown(square("e3"), { key: " " });
    expect(onPlay).toHaveBeenCalledExactlyOnceWith("e2e3");
    rerender(<Board {...props} orientation="black" />);
    fireEvent.keyDown(square("e3"), { key: "ArrowUp" });
    expect(document.activeElement).toBe(square("e2"));
    fireEvent.keyDown(square("e2"), { key: "Enter" });
    fireEvent.keyDown(square("e2"), { key: "Escape" });
    expect(square("e2").getAttribute("aria-pressed")).toBe("false");
    expect(screen.getByRole("group").getAttribute("aria-label")).toBe(
      "Chess board, black at bottom",
    );
  });
  it("clears partial selection on a newer native session and rejects disabled clicks", () => {
    const { onPlay, rerender, props } = board();
    fireEvent.click(square("e2"));
    rerender(<Board {...props} version={props.version + 1} disabled />);
    expect(square("e2").getAttribute("aria-pressed")).toBe("false");
    fireEvent.click(square("e2"));
    fireEvent.click(square("e4"));
    expect(onPlay).not.toHaveBeenCalled();
  });
});
