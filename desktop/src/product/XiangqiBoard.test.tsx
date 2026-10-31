// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { XiangqiBoard } from "./XiangqiBoard";
import type { XiangqiSnapshot as GameSnapshot } from "./native";
import type { MovementInspection } from "./movementInspection";
import standardJson from "../__fixtures__/shared-xiangqi-standard.json";
import inspectionJson from "../__fixtures__/shared-xiangqi-inspection.json";

// Both payloads are emitted by the native fixture generator. Expected legal
// actions below are selected from those payloads, never calculated in JS.
const standard = standardJson as unknown as GameSnapshot;
const inspection = inspectionJson as MovementInspection;
afterEach(cleanup);
function point(key: string) {
  const node = document.querySelector<SVGGElement>(`.xiangqi-point[data-intersection="${key}"]`);
  if (!node) throw Error(`Missing native intersection ${key}`);
  return node;
}
function flipped(snapshot = standard) {
  return { ...snapshot, document: { ...snapshot.document, settings: { ...snapshot.document.settings, orientation: "black" } } } as GameSnapshot;
}
function board(snapshot = standard) {
  const props = { snapshot, disabled: false, onPlay: vi.fn(), onInspect: vi.fn().mockResolvedValue(inspection), inspectionDisabled: false };
  return { ...render(<XiangqiBoard {...props} />), props };
}

it("renders all native intersections, river and palace diagonals without square cells or foreign game controls", () => {
  const { container } = board();
  expect(container.querySelectorAll(".xiangqi-point")).toHaveLength(90);
  expect([...container.querySelectorAll(".xiangqi-point")].map((node) => node.getAttribute("data-intersection"))).toEqual(standard.board.intersections!.points);
  expect(container.querySelectorAll(".xiangqi-piece")).toHaveLength(32);
  expect(container.querySelector("svg")?.getAttribute("viewBox")).toBe("0 0 980 1080");
  expect(container.querySelectorAll("[data-palace]")).toHaveLength(4);
  const geometry = (key: string) => point(key).getAttribute("transform")!.match(/\d+/g)!.join(",");
  for (const palace of standard.board.intersections!.palaces) {
    const drawn = [...container.querySelectorAll(`[data-palace="${palace.side.key}"]`)];
    expect(drawn.map((line) => line.getAttribute("points"))).toEqual(palace.diagonals.map((line) => line.map(geometry).join(" ")));
  }
  expect(container.querySelectorAll('.xiangqi-grid line[y1="90"][y2="490"]')).toHaveLength(7);
  expect(container.querySelectorAll('.xiangqi-grid line[y1="590"][y2="990"]')).toHaveLength(7);
  expect(screen.getByText("楚河")).toBeTruthy(); expect(screen.getByText("漢界")).toBeTruthy();
  expect(container.querySelector(".shogi-hand, .shogi-square, .board-square")).toBeNull();
  expect(screen.queryByRole("dialog")).toBeNull();
});

it("uses approved differentiated piece art while semantic names and ownership stay native in both orientations", () => {
  const { props, rerender, container } = board();
  const art = { a0: "俥", b0: "傌", c0: "相", d0: "仕", e0: "帥", b2: "炮", a3: "兵", a9: "車", b9: "馬", c9: "象", d9: "士", e9: "將", b7: "砲", a6: "卒" };
  for (const [origin, glyph] of Object.entries(art)) {
    const native = standard.pieces.find((piece) => piece.square === origin)!;
    expect(point(origin).querySelector(".xiangqi-piece text")?.textContent).toBe(glyph);
    expect(point(origin).getAttribute("aria-label")).toContain(native.kind);
    expect(point(origin).querySelector(".xiangqi-piece")?.getAttribute("transform")).toBeNull();
  }
  expect(screen.getByRole("group", { name: "Xiangqi board, Red at bottom" })).toBeTruthy();
  expect(point("a0").getAttribute("transform")).toBe("translate(90 990)");
  expect(container.querySelector('[data-game-slot="lower-player"] [data-player="red"]')).toBeTruthy();
  rerender(<XiangqiBoard {...props} snapshot={flipped()} />);
  expect(screen.getByRole("group", { name: "Xiangqi board, Black at bottom" })).toBeTruthy();
  expect(point("a0").getAttribute("transform")).toBe("translate(890 90)");
  expect(container.querySelector('[data-game-slot="lower-player"] [data-player="black"]')).toBeTruthy();
  expect(point("e9").querySelector("text")?.textContent).toBe("將");
});

it("submits exactly the native complete move, preserving capture precedence over opponent inspection", () => {
  const { props } = board();
  const capture = standard.choices.find((choice) => choice.action === "b2b9")!;
  expect(capture).toBeTruthy();
  fireEvent.click(point(capture.from));
  expect(point(capture.steps[0]!).getAttribute("aria-label")).toContain("legal destination");
  expect(point("b9").querySelector(".xiangqi-destination circle")?.getAttribute("r")).toBe("46");
  fireEvent.click(point("b9"));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(capture.action);
  expect(props.onInspect).not.toHaveBeenCalled();
});

it.each(["red", "black"])("supports keyboard navigation, focus and Escape without committing partial selection in %s orientation", (orientation) => {
  const { props } = board(orientation === "black" ? flipped() : standard);
  act(() => point("e3").focus());
  fireEvent.keyDown(point("e3"), { key: "Enter" });
  expect(point("e3").getAttribute("aria-pressed")).toBe("true");
  fireEvent.keyDown(point("e3"), { key: "ArrowRight" });
  expect(document.activeElement).toBe(point(orientation === "red" ? "f3" : "d3"));
  fireEvent.keyDown(document.activeElement!, { key: "Escape" });
  expect(document.activeElement).toBe(point("e3"));
  expect(point("e3").getAttribute("aria-pressed")).toBe("false");
  expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(point("e3"), { key: " " });
  fireEvent.keyDown(point("e3"), { key: orientation === "red" ? "ArrowUp" : "ArrowDown" });
  expect(document.activeElement).toBe(point("e4"));
  fireEvent.keyDown(point("e4"), { key: "Enter" });
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(standard.choices.find((choice) => choice.from === "e3")!.action);
});

it.each(["red", "black"])("presents native read-only inspection with hollow and occupied markers in %s orientation", async (orientation) => {
  const { props, container } = board(orientation === "black" ? flipped() : standard);
  fireEvent.click(point(inspection.origin));
  const status = await screen.findByText("Inspecting Black cannon · Esc clears");
  expect(status.textContent).toBe("Inspecting Black cannon · Esc clears");
  expect(status.textContent).not.toMatch(/Read-only|Hollow rings/);
  expect(screen.getByRole("status").textContent).toContain("Read-only movement choices; no action can be played.");
  expect(props.onInspect).toHaveBeenCalledExactlyOnceWith(inspection.origin);
  expect(container.querySelectorAll(".inspection-marker")).toHaveLength(inspection.choices.length);
  expect(point("b3").querySelector(".inspection-marker")?.getAttribute("r")).toBe("19");
  expect(point("b0").querySelector(".inspection-marker")?.getAttribute("r")).toBe("47");
  expect(point("b0").querySelector(".xiangqi-piece")).toBeTruthy();
  expect(point("b0").querySelector(".xiangqi-destination")).toBeNull();
  fireEvent.click(point("b0")); expect(props.onPlay).not.toHaveBeenCalled();
  expect(screen.queryByRole("dialog")).toBeNull();
  fireEvent.keyDown(point("b0"), { key: "Escape" });
  expect(document.activeElement).toBe(point("b7"));
  expect(container.querySelectorAll(".inspection-marker")).toHaveLength(0);
});

it.each(["context", "orientation", "origin", "friendly", "cancel"])("retires a deferred native inspection after %s changes", async (change) => {
  let resolve!: (reply: MovementInspection) => void;
  const onInspect = vi.fn().mockImplementation(() => new Promise<MovementInspection>((done) => { resolve = done; }));
  const props = { snapshot: standard, disabled: false, onPlay: vi.fn(), onInspect, inspectionDisabled: false };
  const { rerender, container } = render(<XiangqiBoard {...props} />);
  fireEvent.click(point("b7"));
  const oldResolve = resolve;
  if (change === "context") rerender(<XiangqiBoard {...props} snapshot={{ ...standard, token: { ...standard.token, selection: standard.token.selection + 1 } }} />);
  else if (change === "orientation") rerender(<XiangqiBoard {...props} snapshot={flipped()} />);
  else if (change === "origin") fireEvent.click(point("h7"));
  else if (change === "friendly") fireEvent.click(point("e3"));
  else fireEvent.keyDown(point("b7"), { key: "Escape" });
  await act(async () => oldResolve(inspection));
  expect(screen.queryByText(/Inspecting Black cannon · Esc clears/)).toBeNull();
  expect(container.querySelectorAll(".inspection-marker")).toHaveLength(0);
  expect(props.onPlay).not.toHaveBeenCalled();
  if (change === "friendly") expect(point("e4").getAttribute("aria-label")).toContain("legal destination");
  if (change === "origin") expect(point("h7").getAttribute("aria-label")).toContain("loading movement inspection");
});

it("allows native inspection of a historical position while all game actions remain disabled", async () => {
  const snapshot = { ...standard, historical: true, capabilities: { ...standard.capabilities, board_action: false } };
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(inspection);
  render(<XiangqiBoard snapshot={snapshot} disabled onPlay={onPlay} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(point("b7")); await screen.findByText(/Inspecting Black cannon · Esc clears/);
  fireEvent.click(point("b0")); expect(onPlay).not.toHaveBeenCalled();
  expect(document.querySelector("[data-position-status]")?.textContent).toContain("Read-only history");
});

// Renderer contract probes, not additional Xiangqi rules: complete native choice
// identifiers and ordered routes must survive presentation without interpretation.
it("preserves ordered routes, terminal-prefix alternatives and native completion identities", () => {
  const snapshot: GameSnapshot = { ...standard, choices: [
    { action: "contract-short-A", from: "a3", steps: ["a4"], completion: { label: "Finish A", glyph: null } },
    { action: "contract-short-B", from: "a3", steps: ["a4"], completion: { label: "Finish B", glyph: null } },
    { action: "contract-long", from: "a3", steps: ["a4", "b4", "a3"], completion: null },
  ] };
  const { props } = board(snapshot);
  fireEvent.click(point("a3")); fireEvent.click(point("a4"));
  expect(props.onPlay).not.toHaveBeenCalled(); expect(point("b4").getAttribute("aria-label")).toContain("legal destination");
  fireEvent.click(screen.getByRole("button", { name: "Complete action here" }));
  const chooser = screen.getByRole("dialog", { name: "Choose complete action" });
  expect(props.onPlay).not.toHaveBeenCalled(); expect(document.activeElement).toBe(screen.getByRole("button", { name: "Finish A" }));
  fireEvent.keyDown(chooser, { key: "Escape" }); expect(screen.queryByRole("dialog")).toBeNull(); expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(point("a3")); fireEvent.click(point("a4")); fireEvent.click(point("b4"));
  expect(props.onPlay).not.toHaveBeenCalled(); fireEvent.click(point("a3"));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith("contract-long");
});
it("keeps different native routes with identical endpoints distinct", () => {
  const snapshot: GameSnapshot = { ...standard, choices: [
    { action: "contract-via-a4", from: "a3", steps: ["a4", "b4"], completion: null },
    { action: "contract-via-b3", from: "a3", steps: ["b3", "b4"], completion: null },
  ] };
  const { props } = board(snapshot); fireEvent.click(point("a3")); fireEvent.click(point("b3"));
  expect(props.onPlay).not.toHaveBeenCalled(); expect(point("b4").getAttribute("aria-label")).toContain("legal destination");
  fireEvent.click(point("b4")); expect(props.onPlay).toHaveBeenCalledExactlyOnceWith("contract-via-b3");
});
it("keeps rendered read-only completed geometry separate from current selection", () => {
  const { props, rerender } = board(); fireEvent.click(point("a3"));
  rerender(<XiangqiBoard {...props} completedAction={{ origin: { kind: "intersection", point: "a6" }, steps: ["a5"] }} />);
  expect(point("a4").getAttribute("aria-label")).toContain("legal destination");
  expect(document.querySelector('[data-last-move="destination"]')?.getAttribute("data-location")).toBe("a5");
  fireEvent.click(point("a4")); expect(props.onPlay).toHaveBeenCalledExactlyOnceWith("a3a4");
});
