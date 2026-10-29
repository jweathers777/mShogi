// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
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

// These supplied inspection replies exercise presentation dispatch, not a second
// movement oracle. Native rules/session tests establish the returned legality.
const inspectedPawn = {
  game: "chess" as const, token: { document: 1, version: 0, revision: 0, selection: 0 },
  origin: "e7", side: { id: { game_kind: "chess" as const, side: "black" as const }, key: "black", label: "Black", first_mover: false },
  label: "Black pawn", read_only: true as const,
  choices: ["e6", "e5"].map((destination) => ({ action: `e7${destination}`, from: "e7", steps: [destination], completion: null })),
};
it.each(["white", "black"] as const)("inspects an opponent in %s orientation, switches to friendly action, and clears with Escape", async (orientation) => {
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(inspectedPawn);
  render(<Board pieces={standard.pieces} choices={standard.legal_choices} version={0} orientation={orientation} disabled={false} humanOwner="white" onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  square("e7").focus(); fireEvent.keyDown(square("e7"), { key: "Enter" });
  const status = await screen.findByText("Inspecting Black pawn · Esc clears");
  expect(status.textContent).toBe("Inspecting Black pawn · Esc clears");
  expect(status.textContent).not.toMatch(/Read-only|Hollow rings/);
  expect(screen.getByRole("status").textContent).toContain("Read-only movement choices; no action can be played.");
  expect(square("e7").getAttribute("aria-label")).toContain("Inspecting Black pawn");
  expect(square("e6").getAttribute("aria-label")).toContain("read-only inspection destination");
  expect(square("e6").querySelector(".inspection-marker")).toBeTruthy();
  expect(square("e6").querySelector(".move-marker")).toBeNull();
  fireEvent.click(square("e6")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("e2")); expect(square("e4").getAttribute("aria-label")).toContain("legal destination");
  expect(screen.queryByText(/Inspecting Black pawn · Esc clears/)).toBeNull();
  fireEvent.keyDown(square("e2"), { key: "Escape" }); expect(square("e2").getAttribute("aria-pressed")).toBe("false");
  fireEvent.click(square("e2")); fireEvent.click(square("e4")); expect(onPlay).toHaveBeenCalledExactlyOnceWith("e2e4");
});
it("keeps historical/engine-turn inspection available when board actions are disabled and retires late results", async () => {
  let resolve!: (value: typeof inspectedPawn) => void;
  const onInspect = vi.fn().mockReturnValue(new Promise((r) => { resolve = r; })), onPlay = vi.fn();
  const props = { pieces: standard.pieces, choices: standard.legal_choices, version: 0, orientation: "white" as const, disabled: true, humanOwner: "white", onPlay, onInspect, inspectionDisabled: false, inspectionContextKey: "historical-1" };
  const { rerender } = render(<Board {...props} />);
  fireEvent.click(square("e7")); expect(onInspect).toHaveBeenCalledWith("e7");
  rerender(<Board {...props} inspectionContextKey="historical-2" />);
  await act(async () => resolve(inspectedPawn)); expect(screen.queryByText(/Inspecting Black pawn · Esc clears/)).toBeNull();
  onInspect.mockResolvedValue(inspectedPawn); fireEvent.click(square("e7")); await screen.findByText(/Inspecting Black pawn · Esc clears/);
  fireEvent.click(square("e6")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square("e6"), { key: "Escape" }); expect(screen.queryByText(/Inspecting Black pawn · Esc clears/)).toBeNull();
});
it("retains complete compound inspection routes and completion alternatives without executable controls", async () => {
  const onPlay = vi.fn();
  const onInspect = vi.fn().mockResolvedValue({ ...inspectedPawn, choices: [
    { action: "synthetic-route-a", from: "e7", steps: ["e6", "d6"], completion: { label: "Promote", glyph: null } },
    { action: "synthetic-route-b", from: "e7", steps: ["e6", "d6"], completion: { label: "Do not promote", glyph: null } },
    { action: "synthetic-route-stop", from: "e7", steps: ["e6"], completion: null },
  ] });
  render(<Board pieces={standard.pieces} choices={standard.legal_choices} version={0} orientation="white" disabled={false} humanOwner="white" onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(square("e7")); await screen.findByText(/Inspecting Black pawn · Esc clears/); fireEvent.click(square("e6"));
  expect(square("d6").getAttribute("aria-label")).toContain("read-only inspection destination");
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.click(square("d6")); expect(onPlay).not.toHaveBeenCalled();
  expect(screen.getByRole("status").textContent).toContain("Promote or Do not promote"); expect(screen.queryByRole("dialog")).toBeNull();
});
it("preserves actionable capture destination precedence over enemy-origin inspection", () => {
  const onPlay = vi.fn(), onInspect = vi.fn();
  // Synthetic adapter choice isolates event precedence, not chess legality.
  render(<Board pieces={standard.pieces} choices={[{ action: "capture-contract", from: "e2", steps: ["e7"], completion: null }]} version={0} orientation="white" disabled={false} humanOwner="white" onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(square("e2")); fireEvent.click(square("e7")); expect(onPlay).toHaveBeenCalledExactlyOnceWith("capture-contract"); expect(onInspect).not.toHaveBeenCalled();
});
it("switches from an inspection capture marker to the friendly piece's actionable choices", async () => {
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue({ ...inspectedPawn, choices: [{ action: "inspection-capture-contract", from: "e7", steps: ["e2"], completion: null }] });
  render(<Board pieces={standard.pieces} choices={standard.legal_choices} version={0} orientation="white" disabled={false} humanOwner="white" onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(square("e7")); await screen.findByText(/Inspecting Black pawn · Esc clears/);
  fireEvent.click(square("e2")); expect(square("e2").getAttribute("aria-label")).toContain("selected"); expect(square("e4").getAttribute("aria-label")).toContain("legal destination"); expect(onPlay).not.toHaveBeenCalled();
});
it("keeps live inspection help in the shell footer without changing board selection or keyboard clearing", async () => {
  const helpTarget = document.createElement("div");
  document.body.append(helpTarget);
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(inspectedPawn);
  const { container, unmount } = render(<Board pieces={standard.pieces} choices={standard.legal_choices} version={0} orientation="white" disabled={false} humanOwner="white" onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} helpTarget={helpTarget} />);
  expect(helpTarget.textContent).toContain("Select a piece");
  expect(container.querySelector(".board-wrap .board-help")).toBeNull();
  fireEvent.click(square("e7"));
  await screen.findByText(/Inspecting Black pawn · Esc clears/);
  expect(helpTarget.querySelector(".movement-inspection-status")).not.toBeNull();
  expect(document.activeElement).toBe(square("e7"));
  fireEvent.keyDown(square("e7"), { key: "Escape" });
  expect(helpTarget.textContent).toContain("Select a piece");
  expect(onPlay).not.toHaveBeenCalled();
  unmount();
  expect(helpTarget.textContent).toBe("");
  helpTarget.remove();
});
