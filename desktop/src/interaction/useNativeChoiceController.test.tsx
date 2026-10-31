// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { act, cleanup, renderHook } from "@testing-library/react";
import { gridNeighbor, useNativeChoiceController, type ChoiceControllerOptions } from "./useNativeChoiceController";
import type { GameSnapshot, ChessSnapshot } from "../product/native";
import type { MovementInspection } from "../product/movementInspection";
import chessJson from "../__fixtures__/shared-chess-standard.json";
import promotionJson from "../__fixtures__/shared-chess-promotion.json";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";

const chess = chessJson as ChessSnapshot;
afterEach(cleanup);
function options(snapshot: GameSnapshot = chess): ChoiceControllerOptions {
  const orientation = snapshot.orientations[0]!;
  const keys = orientation.ranks.flatMap(rank => orientation.files.map(file => `${file}${rank}`));
  return {
    contextKey: JSON.stringify([snapshot.game, snapshot.profile, snapshot.token, orientation]), choices: snapshot.choices,
    origins: [...snapshot.pieces.map(piece => ({ key: piece.square, owner: piece.owner, inspectable: true })),
      ...snapshot.hands.filter(hand => hand.count > 0).map(hand => ({ key: hand.origin, owner: hand.owner, inspectable: true }))],
    humanOwner: snapshot.human.key, disabled: false, inspectionDisabled: false, onPlay: vi.fn(),
    initialFocus: snapshot.board.initial_focus, boardKeys: keys,
    neighbor: (key, direction) => gridNeighbor(keys, orientation.files.length, key, direction),
  };
}
const inspection: MovementInspection = {
  game: "chess", token: chess.token, origin: "e7", label: "Black pawn", read_only: true,
  side: chess.sides.find(side => side.id.side === "black")!,
  choices: [{ action: "e7e6", from: "e7", steps: ["e6"], completion: null }],
};
function deferred() { let resolve!: (value: MovementInspection) => void; const promise = new Promise<MovementInspection>(r => { resolve = r; }); return { promise, resolve }; }

it.each([
  [chessJson, "e2", "e4", "e2e4"],
  [shogiJson, "7g", "7f", "7g7f"],
  [xiangqiJson, "e3", "e4", "e3e4"],
] as const)("submits exactly one supplied %j complete native action", (json, from, to, action) => {
  const props = options(json as GameSnapshot), play = vi.mocked(props.onPlay);
  const { result } = renderHook(() => useNativeChoiceController(props));
  act(() => result.current.select(from)); expect(play).not.toHaveBeenCalled();
  const destinationHandler = result.current.select;
  act(() => { destinationHandler(to); destinationHandler(to); });
  expect(play).toHaveBeenCalledExactlyOnceWith(action);
  act(() => { result.current.select(from); result.current.select(to); });
  expect(play).toHaveBeenCalledTimes(1);
});
it("retains current selection on semantically identical polling, retires changed native complete choices", () => {
  const props = options();
  const { result, rerender } = renderHook(value => useNativeChoiceController(value), { initialProps: props });
  act(() => result.current.select("e2"));
  rerender({ ...props, choices: structuredClone(props.choices), origins: structuredClone(props.origins), boardKeys: [...props.boardKeys] });
  expect(result.current.selected).toBe("e2"); expect(result.current.destinations).toContain("e4");
  rerender({ ...props, choices: props.choices.filter(choice => choice.action !== "e2e4") });
  expect(result.current.selected).toBeNull();
  act(() => result.current.select("e4")); expect(props.onPlay).not.toHaveBeenCalled();
});
it("does not revive a retired promotion completion handler after cancellation or a different origin", () => {
  const props = options(promotionJson as ChessSnapshot);
  const { result } = renderHook(() => useNativeChoiceController(props));
  act(() => result.current.select("a7")); act(() => result.current.select("a8"));
  expect(result.current.completions.map(c => c.action)).toEqual(["a7a8q", "a7a8r", "a7a8b", "a7a8n"]);
  const retired = result.current.chooseCompletion;
  act(() => result.current.cancel()); act(() => retired("a7a8n"));
  expect(props.onPlay).not.toHaveBeenCalled(); expect(result.current.completions).toEqual([]);
  act(() => result.current.select("a7")); act(() => result.current.select("a8"));
  const current = result.current.chooseCompletion;
  act(() => { current("a7a8n"); current("a7a8q"); });
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith("a7a8n");
});
it.each(["document", "version", "revision", "selection"] as const)("retires a pending query and old handlers after native %s changes", async field => {
  const pending = deferred(), props = { ...options(), onInspect: vi.fn(() => pending.promise) };
  const { result, rerender } = renderHook(value => useNativeChoiceController(value), { initialProps: props });
  act(() => result.current.select("e7")); const oldSelect = result.current.select;
  rerender({ ...props, contextKey: JSON.stringify({ ...chess.token, [field]: chess.token[field] + 1 }) });
  await act(async () => pending.resolve(inspection));
  expect(result.current.inspection).toBeNull(); expect(result.current.inspectionPending).toBe(false);
  act(() => oldSelect("e2")); expect(result.current.selected).toBeNull(); expect(props.onPlay).not.toHaveBeenCalled();
});
it.each(["orientation", "disabled", "inspectionDisabled", "provider", "owner", "origins"] as const)("retires query/selection on changed %s", async change => {
  const pending = deferred(), props = { ...options(), onInspect: vi.fn(() => pending.promise) };
  const { result, rerender } = renderHook(value => useNativeChoiceController(value), { initialProps: props });
  act(() => result.current.select("e7"));
  const next = { ...props };
  if (change === "orientation") next.boardKeys = [...props.boardKeys].reverse();
  if (change === "disabled") next.disabled = true;
  if (change === "inspectionDisabled") next.inspectionDisabled = true;
  if (change === "provider") next.onInspect = vi.fn(async () => inspection);
  if (change === "owner") next.humanOwner = "black";
  if (change === "origins") next.origins = props.origins.filter(origin => origin.key !== "e7");
  rerender(next); await act(async () => pending.resolve(inspection));
  expect(result.current.inspection).toBeNull(); expect(result.current.selected).toBeNull(); expect(props.onPlay).not.toHaveBeenCalled();
});
it("keeps a pending query through identical polls but retires superseded origins, Escape and unmount", async () => {
  const first = deferred(), second = deferred();
  const onInspect = vi.fn().mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
  const props = { ...options(), onInspect };
  const { result, rerender, unmount } = renderHook(value => useNativeChoiceController(value), { initialProps: props });
  act(() => result.current.select("e7")); rerender({ ...props, choices: structuredClone(props.choices) });
  expect(result.current.inspectionPending).toBe(true);
  act(() => result.current.select("d7"));
  await act(async () => first.resolve(inspection)); expect(result.current.inspection).toBeNull(); expect(result.current.selected).toBe("d7");
  act(() => result.current.cancel()); await act(async () => second.resolve({ ...inspection, origin: "d7", choices: [] }));
  expect(result.current.selected).toBeNull(); expect(result.current.inspection).toBeNull();
  const late = deferred(); onInspect.mockReturnValueOnce(late.promise); act(() => result.current.select("e7"));
  unmount(); await act(async () => late.resolve(inspection)); expect(props.onPlay).not.toHaveBeenCalled();
});
it.each(["read_only", "origin", "choice_origin"] as const)("never accepts a malformed %s inspection as actionable work", async difference => {
  const reply = structuredClone(inspection);
  if (difference === "read_only") Object.assign(reply, { read_only: false });
  if (difference === "origin") reply.origin = "d7";
  if (difference === "choice_origin") reply.choices[0]!.from = "d7";
  const props = { ...options(), onInspect: vi.fn(async () => reply) };
  const { result } = renderHook(() => useNativeChoiceController(props));
  await act(async () => result.current.select("e7"));
  expect(result.current.inspection).toBeNull(); expect(result.current.selected).toBeNull(); expect(props.onPlay).not.toHaveBeenCalled();
});
it("contains rejected inspection and preserves historical read-only route traversal", async () => {
  const onInspect = vi.fn().mockRejectedValueOnce(new Error("native capacity")).mockResolvedValue(inspection);
  const props = { ...options(), disabled: true, onInspect };
  const { result } = renderHook(() => useNativeChoiceController(props));
  await act(async () => result.current.select("e7")); expect(result.current.inspectionPending).toBe(false);
  await act(async () => result.current.select("e7")); expect(result.current.inspection?.read_only).toBe(true);
  act(() => result.current.select("e6")); expect(props.onPlay).not.toHaveBeenCalled(); expect(result.current.completions).toEqual([]);
});
