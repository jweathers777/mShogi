// @vitest-environment jsdom
// Real Rust-produced snapshots; renderer tests do not establish game legality.
import { renderBoard, square } from "../testing/helpers";
import type { ChuSnapshot, MovementInspection } from "../sessionTypes";
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { SquareBoard } from "./SquareBoard";

import routesJson from "../__fixtures__/shared-chu-routes.json";
import royalsJson from "../__fixtures__/shared-chu-royals.json";
import promotionJson from "../__fixtures__/shared-chu-promotion.json";
import inspectionJson from "../__fixtures__/shared-chu-inspection.json";

import capturedJson from "../__fixtures__/shared-chu-captured.json";
const routes = routesJson as unknown as ChuSnapshot;
import { oriented } from "../testing/snapshots";
afterEach(cleanup);

const board = (snapshot = routes) =>
  renderBoard(SquareBoard, { snapshot, inspectionDisabled: false, onInspect: vi.fn() });
it("uses shared player/board/footer slots, full board geometry, traditional faces and no hands", () => {
  const { container, props, rerender } = board(royalsJson as unknown as ChuSnapshot);
  expect(container.querySelectorAll("svg [role=button]")).toHaveLength(144);
  expect(container.querySelector('[data-face="King"]')?.textContent).toBe("玉将");
  expect(container.querySelector('[data-face="Prince"]')?.textContent).toBe("太子");
  expect(container.querySelector('[data-position-status]')).toBeTruthy();
  expect(container.querySelector('[data-game-slot="upper-player"]')).toBeTruthy();
  expect(container.querySelector('[data-game-slot="lower-player"]')).toBeTruthy();
  expect(container.querySelector(".shogi-hand")).toBeNull();
  expect(square("6e").querySelector(".piece-tile")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  const flipped = oriented(props.snapshot, "gote");
  rerender(<SquareBoard {...props} snapshot={flipped} />);
  expect(container.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^1l,/);
  expect(square("6e").querySelector(".piece-tile")?.getAttribute("transform")).toBeNull();
  expect(square("1l").querySelector(".piece-tile")?.getAttribute("transform")).toBe("rotate(180 50 50)");
});
it("decorates partial routes without submitting and commits exact same-endpoint return choices", () => {
  const { props, rerender, container } = board();
  for (const pivot of ["7e", "5f"]) {
    const choice = routes.choices.find(c => c.from === "6f" && c.steps.length === 2 && c.steps[0] === pivot && c.steps[1] === "6f")!;
    fireEvent.click(square("6f"));fireEvent.click(square(pivot));
    expect(props.onPlay).toHaveBeenCalledTimes(pivot === "7e" ? 0 : 1);
    expect(container.querySelector("[data-selected-route]")?.getAttribute("data-selected-route")).toBe(`6f>${pivot}`);
    expect(container.querySelector('[data-game-slot="footer"] [aria-label="Partial route selection"]')).toBeTruthy();
    fireEvent.click(square("6f"));expect(props.onPlay).toHaveBeenLastCalledWith(choice.action);
    rerender(<SquareBoard {...props} snapshot={{ ...routes, token: { ...routes.token, selection: routes.token.selection + (pivot === "7e" ? 1 : 2) } }} />);
  }
  expect(props.onPlay).toHaveBeenCalledTimes(2);
});
it("preserves keyboard cancellation/focus and explicit promotion choice", () => {
  const { props, rerender, container } = board();
  fireEvent.keyDown(square("6f"), { key: "Enter" });fireEvent.keyDown(square("7e"), { key: " " });
  expect(container.querySelector("[data-selected-route]")).toBeTruthy();
  fireEvent.keyDown(square("7e"), { key: "Escape" });
  expect(document.activeElement).toBe(square("6f"));expect(props.onPlay).not.toHaveBeenCalled();
  const promotion = promotionJson as unknown as ChuSnapshot;rerender(<SquareBoard {...props} snapshot={promotion} />);
  fireEvent.click(square("6e"));fireEvent.click(square("6d"));
  expect(props.onPlay).not.toHaveBeenCalled();expect(screen.getByRole("dialog")).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Promote" }));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(promotion.choices.find(c => c.from === "6e" && c.steps[0] === "6d" && c.completion?.label === "Promote")!.action);
});
it("preserves ordered intermediate captures and complete historical geometry", () => {
  const s = capturedJson as unknown as ChuSnapshot;
  const { container } = render(<SquareBoard snapshot={s} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} completedAction={s.moves.last_action} />);
  expect([...container.querySelectorAll("[data-capture]")].map(node => node.getAttribute("data-capture"))).toEqual(["2b", "1b"]);
  expect(container.querySelector("[data-route]")?.getAttribute("data-route")).toBe("3c>2b>1b");
  expect(container.querySelector('[data-last-move="origin"]')?.getAttribute("data-location")).toBe("3c");
  expect(container.querySelector('[data-last-move="destination"]')?.getAttribute("data-location")).toBe("1b");
  expect(container.querySelector('[data-capture="2b"]')?.getAttribute("transform")).toBe("translate(1090 190)");
  expect(container.querySelector('[data-last-move="origin"]')?.getAttribute("d")).toMatch(/^M951 263/);
});

it("keeps enemy inspection visibly distinct and non-actionable in the actual Chu renderer", async () => {
  const s = { ...routes, historical: true, capabilities: { ...routes.capabilities, board_action: false } };
  const response = inspectionJson as unknown as MovementInspection;
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(response);
  const { container } = render(<SquareBoard snapshot={s} disabled inspectionDisabled={false} onPlay={onPlay} onInspect={onInspect} />);
  fireEvent.click(square("11a"));
  await vi.waitFor(() => expect(screen.getByText("Inspecting Gote Pawn · Esc clears")).toBeTruthy());
  expect(container.querySelectorAll(".inspection-marker")).toHaveLength(1);
  expect(container.querySelectorAll(".move-marker,.capture-marker")).toHaveLength(0);
  fireEvent.click(square("11b"));expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square("11b"), { key: "Escape" });expect(container.querySelectorAll(".inspection-marker")).toHaveLength(0);
});

it("commits the sole one-leg game choice from the footer and cancels either footer control with focus recovery", () => {
  const { props, container, rerender } = board();
  const short = routes.choices.filter(choice => choice.from === "6f" && choice.steps.length === 1 && choice.steps[0] === "7e");
  expect(short).toHaveLength(1);
  fireEvent.click(square("6f"));fireEvent.click(square("7e"));
  const complete = screen.getByRole("button", { name: "Complete action here" });
  expect(complete.closest('[data-game-slot="footer"]')).toBeTruthy();
  expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(complete);
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(short[0]!.action);
  expect(document.activeElement).toBe(square("7e"));
  expect(container.querySelector("[data-selected-route]")).toBeNull();
  for (const [index, control] of ["Complete action here", "Cancel selection"].entries()) {
    rerender(<SquareBoard {...props} snapshot={{ ...routes, token: { ...routes.token, selection: routes.token.selection + index + 1 } }} />);
    fireEvent.keyDown(square("6f"), { key: "Enter" });fireEvent.keyDown(square("7e"), { key: " " });
    const button = screen.getByRole("button", { name: control });
    button.focus();expect(document.activeElement).toBe(button);
    fireEvent.keyDown(button, { key: "Escape" });
    expect(document.activeElement).toBe(square("6f"));
    expect(container.querySelector("[data-selected-route]")).toBeNull();
    expect(screen.queryByRole("group", { name: "Partial route selection" })).toBeNull();
    expect(props.onPlay).toHaveBeenCalledTimes(1);
    expect(container.querySelector('[data-game-slot="footer"] [data-position-status]')).toBeTruthy();
  }
});

it("puts legible coordinate labels outside the Chu grid while retaining route and cue locations", () => {
  const { container, props, rerender } = board();
  expect(container.querySelector("svg")?.getAttribute("viewBox")).toBe("0 0 1280 1280");
  expect(square("12a").getAttribute("transform")).toBe("translate(40, 40)");
  expect(container.querySelectorAll(".square .coordinate")).toHaveLength(0);
  expect(container.querySelectorAll(".coordinate-frame .coordinate")).toHaveLength(24);
  const file = container.querySelector('[data-coordinate-axis="file"]')!, rank = container.querySelector('[data-coordinate-axis="rank"]')!;
  expect([file.textContent, file.getAttribute("x"), file.getAttribute("y")]).toEqual(["12", "90", "1272"]);
  expect([rank.textContent, rank.getAttribute("x"), rank.getAttribute("y")]).toEqual(["a", "1260", "102"]);
  fireEvent.click(square("6f"));fireEvent.click(square("7e"));
  const displayed = props.snapshot.orientations[0]!.ranks.flatMap(r => props.snapshot.orientations[0]!.files.map(f => `${f}${r}`));
  const center = (key: string) => { const i = displayed.indexOf(key);return `${90 + i % 12 * 100},${90 + Math.floor(i / 12) * 100}`; };
  expect(container.querySelector(".selected-route polyline")?.getAttribute("points")).toBe(["6f", "7e"].map(center).join(" "));
  expect(props.onPlay).not.toHaveBeenCalled();
  const flipped = oriented(routes, "gote");
  rerender(<SquareBoard {...props} snapshot={flipped} />);
  expect(container.querySelector('[data-coordinate-axis="file"]')?.textContent).toBe("1");
  expect(container.querySelector('[data-coordinate-axis="rank"]')?.textContent).toBe("l");
});
