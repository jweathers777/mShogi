// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { Board } from "./Board";
import type { CompletedActionGeometry } from "./boardPresentation";
import { ShogiBoard } from "./product/ShogiBoard";
import { XiangqiBoard } from "./product/XiangqiBoard";
import { GameBoard } from "./product/GameBoard";
import type { ChessSnapshot } from "./product/native";
import type { ShogiSnapshot, XiangqiSnapshot } from "./product/native";
import shogiJson from "./__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "./__fixtures__/shared-xiangqi-standard.json";
import chessJson from "./__fixtures__/shared-chess-standard.json";
afterEach(cleanup);
const action: CompletedActionGeometry = { action: "opaque-native-action", event_index: 11, origin: { kind: "board", square: "e2" }, steps: ["e4"], effects: [] };
function marks(kind: string) { return [...document.querySelectorAll(`[data-last-move="${kind}"]`)].map(node => node.getAttribute("data-location")); }
it("draws native castling effects without parsing the action or changing available actions", () => {
  const onPlay = vi.fn();
  render(<Board pieces={[]} choices={[]} orientation="white" version={1} disabled={false} onPlay={onPlay} completedAction={{ ...action, action: "unparsed-castling", origin: { kind: "board", square: "e1" }, steps: ["g1"], effects: [{ from: "h1", to: "f1" }] }} />);
  expect(marks("origin")).toEqual(["e1", "h1"]); expect(marks("destination")).toEqual(["g1", "f1"]);
  expect(document.querySelector(".completed-action-cue")?.getAttribute("pointer-events")).toBe("none");
  expect(document.querySelector(".completed-action-cue")?.getAttribute("aria-hidden")).toBe("true");
  fireEvent.click(document.querySelector('[data-location="g1"]')!); expect(onPlay).not.toHaveBeenCalled();
  expect(document.querySelectorAll('[role="button"]')).toHaveLength(64);
});
it("persists during thinking/polling and changes layout only through orientation, then retires at a native empty prefix", () => {
  const props = { pieces: [], choices: [], orientation: "white" as const, version: 1, disabled: true, onPlay: vi.fn() };
  const { rerender } = render(<Board {...props} completedAction={action} />);
  const before = document.querySelector('[data-last-move="destination"]')?.getAttribute("d");
  rerender(<Board {...props} version={2} completedAction={structuredClone(action)} />);
  expect(document.querySelector('[data-last-move="destination"]')?.getAttribute("d")).toBe(before);
  rerender(<Board {...props} orientation="black" completedAction={action} />);
  expect(marks("origin")).toEqual(["e2"]); expect(marks("destination")).toEqual(["e4"]);
  expect(document.querySelector('[data-last-move="destination"]')?.getAttribute("d")).not.toBe(before);
  rerender(<Board {...props} completedAction={null} />); expect(document.querySelector(".completed-action-cue")).toBeNull();
});
it("distinguishes ordered same-endpoint compound actions including return to origin, while partial selection cannot replace a completed cue", () => {
  const choices = [{ action: "future", from: "e2", steps: ["e3", "e4"], completion: null }], onPlay = vi.fn();
  const props = { pieces: [], choices, orientation: "white" as const, version: 1, disabled: false, onPlay };
  const { rerender } = render(<Board {...props} completedAction={{ ...action, steps: ["d3", "e2"] }} />);
  expect(document.querySelector(".completed-action-route")?.getAttribute("data-route")).toBe("e2>d3>e2");
  expect(marks("destination")).toEqual(["e2"]);
  const origin = document.querySelector('[role="button"][aria-label^="e2,"]')!;
  fireEvent.click(origin); fireEvent.keyDown(origin, { key: "Escape" });
  expect(document.querySelector(".completed-action-route")?.getAttribute("data-route")).toBe("e2>d3>e2"); expect(onPlay).not.toHaveBeenCalled();
  rerender(<Board {...props} completedAction={{ ...action, steps: ["f3", "e2"] }} />);
  expect(document.querySelector(".completed-action-route")?.getAttribute("data-route")).toBe("e2>f3>e2");
});
it("draws a Shogi drop destination only and retains separate inspection/selection semantics", () => {
  render(<ShogiBoard snapshot={shogiJson as ShogiSnapshot} disabled={false} onPlay={vi.fn()} completedAction={{ ...action, origin: { kind: "hand", owner: "sente", piece_kind: "pawn" }, steps: ["5e"] }} />);
  expect(marks("origin")).toEqual([]); expect(marks("destination")).toEqual(["5e"]);
  const cue = document.querySelector('[data-last-move="destination"]')!;
  expect(cue.getAttribute("data-cue-signature")).toBe("double-underline");
  // Actual two parallel bars, independently visible from legal capture corners.
  const bars = [...cue.getAttribute("d")!.matchAll(/M(-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)h(\d+(?:\.\d+)?)/g)].slice(-2);
  expect(bars).toHaveLength(2);
  expect(bars[0]![1]).toBe(bars[1]![1]);
  expect(bars[0]![3]).toBe(bars[1]![3]);
  expect(Number(bars[0]![2]) - Number(bars[1]![2])).toBe(6);
  expect(document.querySelector(".inspection-marker")).toBeNull();
  expect(document.querySelectorAll('.shogi-square[aria-pressed="true"]')).toHaveLength(0);
});
it("uses Xiangqi intersection layout and exact native target rather than square/chess semantics", () => {
  render(<XiangqiBoard snapshot={xiangqiJson as unknown as XiangqiSnapshot} disabled={true} onPlay={vi.fn()} completedAction={{ ...action, origin: { kind: "intersection", point: "b2" }, steps: ["e2"] }} />);
  expect(marks("origin")).toEqual(["b2"]); expect(marks("destination")).toEqual(["e2"]);
  expect(document.querySelector('[data-last-move="destination"]')?.getAttribute("d")).toContain("M443 764");
  expect(document.querySelectorAll(".xiangqi-point")).toHaveLength(90);
});
it("takes native selected-prefix geometry through the actual game adapter for paused/reopened/history/Take back states", () => {
  const initial = { ...chessJson, moves: { root: { label: "Starting position", numbering: "Known history" }, entries: [], last_action: action }, paused: true } as unknown as ChessSnapshot;
  const command = vi.fn(), props = { disabled: false, inspectionDisabled: false, command };
  const { rerender } = render(<GameBoard {...props} snapshot={initial} />);
  expect(marks("destination")).toEqual(["e4"]);
  const historical = { ...initial, historical: true, event_index: 9, token: { ...initial.token, selection: initial.token.selection + 1 }, moves: { ...initial.moves, last_action: { ...action, event_index: 9, origin: { kind: "board" as const, square: "g1" }, steps: ["f3"] } } };
  rerender(<GameBoard {...props} snapshot={historical} />);
  expect(marks("origin")).toEqual(["g1"]); expect(marks("destination")).toEqual(["f3"]);
  rerender(<GameBoard {...props} snapshot={{ ...initial, token: { ...initial.token, selection: historical.token.selection + 1 } }} />);
  expect(marks("destination")).toEqual(["e4"]);
  rerender(<GameBoard {...props} snapshot={{ ...initial, moves: { ...initial.moves, last_action: null }, token: { ...initial.token, document: initial.token.document + 1 } }} />);
  expect(document.querySelector(".completed-action-cue")).toBeNull(); expect(command).not.toHaveBeenCalled();
});
