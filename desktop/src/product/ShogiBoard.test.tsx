// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ShogiBoard } from "./ShogiBoard";
import type { ChessShogiSnapshot as GameSnapshot } from "./native";
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

// Mock native query replies test read-only presentation. Native/session tests own
// movement/drop expectations; these tests do not implement shogi legality.
const gote = standard.sides.find((side) => side.id.side === "gote")!;
const inspectionReply = { game: "shogi" as const, token: standard.token, origin: "3c", side: gote, label: "Gote pawn", read_only: true as const, choices: [{ action: "3c3d", from: "3c", steps: ["3d"], completion: null }] };
it.each(["sente", "gote"])("uses read-only board inspection and actionable friendly switching in %s orientation", async (orientation) => {
  const onPlay = vi.fn(), onInspect = vi.fn().mockImplementation(async (origin: string) => ({ ...inspectionReply, origin, label: origin === "4a" ? "Gote gold" : "Gote pawn", choices: origin === "4a" ? [] : inspectionReply.choices }));
  const snapshot = { ...standard, document: { ...standard.document, settings: { ...standard.document.settings, settings: { ...standard.document.settings.settings, orientation } } } };
  render(<ShogiBoard snapshot={snapshot} disabled={false} onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  square("3c").focus(); fireEvent.keyDown(square("3c"), { key: "Enter" }); const status = await screen.findByText("Inspecting Gote pawn · Esc clears");
  expect(status.textContent).toBe("Inspecting Gote pawn · Esc clears");
  expect(status.textContent).not.toMatch(/Read-only|Hollow rings/);
  expect(screen.getByRole("status").textContent).toContain("Read-only movement choices; no action can be played.");
  expect(square("3c").getAttribute("aria-label")).toContain("Inspecting Gote pawn"); expect(square("3d").getAttribute("aria-label")).toContain("read-only inspection destination");
  expect(square("3d").querySelector(".inspection-marker")).toBeTruthy(); expect(square("3d").querySelector(".shogi-destination")).toBeNull();
  fireEvent.click(square("3d")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("4a")); await screen.findByText(/Inspecting Gote gold · Esc clears/); expect(document.querySelectorAll(".inspection-marker")).toHaveLength(0);
  fireEvent.click(square("7g")); expect(square("7f").getAttribute("aria-label")).toContain("legal destination");
  fireEvent.keyDown(square("7g"), { key: "Escape" }); expect(square("7g").getAttribute("aria-pressed")).toBe("false");
  fireEvent.click(square("7g")); fireEvent.click(square("7f")); expect(onPlay).toHaveBeenCalledExactlyOnceWith("7g7f");
});
it("inspects available opponent hand drops by keyboard during historical viewing without committing", async () => {
  const source = dropsJson as GameSnapshot;
  const snapshot = { ...source, historical: true, capabilities: { ...source.capabilities, board_action: false }, hands: [...source.hands,
    { origin: "gote-pawn-hand", owner: gote.key, kind: "pawn", count: 1, text: "P", label: "Gote pawn in hand" },
    { origin: "gote-rook-zero", owner: gote.key, kind: "rook", count: 0, text: "R", label: "Gote rook in hand" },
  ] };
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue({ ...inspectionReply, token: snapshot.token, origin: "gote-pawn-hand", label: "Gote pawn in hand", choices: [{ action: "P*5e", from: "gote-pawn-hand", steps: ["5e"], completion: null }] });
  render(<ShogiBoard snapshot={snapshot} disabled onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  expect(screen.queryByRole("button", { name: /Gote rook in hand/ })).toBeNull();
  const pawn = screen.getByRole("button", { name: "Gote pawn in hand, 1" });
  expect(pawn.hasAttribute("disabled")).toBe(false); pawn.focus(); await userEvent.setup().keyboard("{Enter}"); await screen.findByText(/Inspecting Gote pawn in hand · Esc clears/);
  expect(pawn.getAttribute("aria-label")).toContain("Inspecting Gote pawn in hand");
  fireEvent.keyDown(pawn, { key: "ArrowDown" }); expect(document.activeElement).toBe(square("5g"));
  fireEvent.click(square("5e")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square("5e"), { key: "Escape" }); expect(document.activeElement).toBe(pawn); expect(screen.queryByText(/Inspecting Gote pawn in hand · Esc clears/)).toBeNull();
});
it("retires asynchronous inspection after friendly switching and native historical selection", async () => {
  let resolve!: (value: typeof inspectionReply) => void;
  const onInspect = vi.fn().mockReturnValue(new Promise((r) => { resolve = r; })), onPlay = vi.fn();
  const props = { snapshot: standard, disabled: false, onPlay, onInspect, inspectionDisabled: false };
  const { rerender } = render(<ShogiBoard {...props} />);
  fireEvent.click(square("3c")); fireEvent.click(square("7g"));
  await act(async () => resolve(inspectionReply)); expect(screen.queryByText(/Inspecting Gote pawn · Esc clears/)).toBeNull(); expect(square("7f").getAttribute("aria-label")).toContain("legal destination");
  onInspect.mockResolvedValue(inspectionReply); fireEvent.click(square("3c")); await screen.findByText(/Inspecting Gote pawn · Esc clears/);
  rerender(<ShogiBoard {...props} snapshot={{ ...standard, historical: true, token: { ...standard.token, selection: standard.token.selection + 1 } }} disabled />);
  expect(screen.queryByText(/Inspecting Gote pawn · Esc clears/)).toBeNull(); expect(onPlay).not.toHaveBeenCalled();
});
it("keeps occupied native inspection destinations readable and never prompts to play promotion", async () => {
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue({ ...inspectionReply, origin: "5a", label: "Gote king", choices: [
    { action: "synthetic-route-a", from: "5a", steps: ["5b", "5i"], completion: { label: "Promote", glyph: null } },
    { action: "synthetic-route-b", from: "5a", steps: ["5b", "5i"], completion: { label: "Do not promote", glyph: null } },
  ] });
  render(<ShogiBoard snapshot={standard} disabled onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(square("5a")); await screen.findByText(/Inspecting Gote king · Esc clears/); fireEvent.click(square("5b"));
  expect(square("5i").querySelector(".shogi-piece")).toBeTruthy(); expect(square("5i").querySelector(".inspection-marker")?.getAttribute("r")).toBe("46");
  fireEvent.click(square("5i")); expect(onPlay).not.toHaveBeenCalled(); expect(screen.queryByRole("dialog")).toBeNull(); expect(screen.getByRole("status").textContent).toContain("Promote or Do not promote");
});
it("uses the shared player/board/footer slots while retaining native hands and keyboard focus", async () => {
  const snapshot = dropsJson as GameSnapshot;
  const onPlay = vi.fn(); const props = { snapshot, disabled: false, onPlay };
  const { rerender } = render(<ShogiBoard {...props} />);
  const area = document.querySelector(".shell-game-area.shogi-stage")!;
  expect(area).toBeTruthy();
  expect([...area.children].map((node) => node.getAttribute("data-game-slot"))).toEqual(["upper-player", "board", "lower-player", "footer"]);
  const upper = area.querySelector('[data-game-slot="upper-player"]')!, lower = area.querySelector('[data-game-slot="lower-player"]')!;
  expect(upper.querySelector('[data-player="gote"]')).toBeTruthy();
  expect(lower.querySelector('[data-player="sente"]')).toBeTruthy();
  expect(area.querySelectorAll('[data-game-slot="board"] .shogi-square')).toHaveLength(81);
  expect(area.querySelector('[data-game-slot="footer"] .shogi-board-help')).toBeTruthy();
  const pawn = screen.getByRole("button", { name: "Sente pawn in hand, 1" });
  pawn.focus(); await userEvent.setup().keyboard("{Enter}");
  fireEvent.keyDown(pawn, { key: "ArrowDown" }); expect(document.activeElement).toBe(square("5g"));
  fireEvent.keyDown(square("5g"), { key: "Escape" }); expect(document.activeElement).toBe(pawn);
  const flipped = { ...snapshot, document: { ...snapshot.document, settings: { ...snapshot.document.settings, settings: { ...snapshot.document.settings.settings, orientation: "gote" } } } };
  rerender(<ShogiBoard {...props} snapshot={flipped} />);
  expect(upper.querySelector('[data-player="sente"]')).toBeTruthy(); expect(lower.querySelector('[data-player="gote"]')).toBeTruthy();
  const movedPawn = screen.getByRole("button", { name: "Sente pawn in hand, 1" });
  movedPawn.focus(); await userEvent.setup().keyboard("{Enter}"); fireEvent.keyDown(movedPawn, { key: "ArrowDown" });
  expect(document.activeElement).toBe(square("5g")); fireEvent.click(square("5e")); expect(onPlay).toHaveBeenCalledExactlyOnceWith("P*5e");
});
