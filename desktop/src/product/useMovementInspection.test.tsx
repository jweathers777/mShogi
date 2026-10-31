// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { act, cleanup, renderHook } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { useMovementInspection } from "./useMovementInspection";
import { useNativeChoiceController } from "../interaction/useNativeChoiceController";
import { inspectionContextKey } from "./movementInspection";
import type { InspectionContext, MovementInspection } from "./movementInspection";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const context: InspectionContext = { game: "shogi", token: { document: 4, version: 8, revision: 12, selection: 2 } };
const reply: MovementInspection = { ...context, origin: "5a", side: { id: { game_kind: "shogi", side: "gote" }, key: "gote", label: "Gote", first_mover: false }, label: "Gote king", choices: [], read_only: true };
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>((r) => { resolve = r; }); return { promise, resolve }; }
beforeEach(() => { vi.mocked(invoke).mockReset(); }); afterEach(cleanup);
it("queries the read-only boundary with the exact native selection token", async () => {
  vi.mocked(invoke).mockResolvedValue(reply);
  const { result } = renderHook(() => useMovementInspection(context));
  expect(await result.current("5a")).toEqual(reply);
  expect(invoke).toHaveBeenCalledExactlyOnceWith("game_inspection", { queryJson: JSON.stringify({ expected: context.token, origin: "5a" }) });
});
it.each(["document", "version", "revision", "selection"] as const)("rejects mismatched native %s", async (field) => {
  vi.mocked(invoke).mockResolvedValue({ ...reply, token: { ...reply.token, [field]: reply.token[field] + 1 } });
  const { result } = renderHook(() => useMovementInspection(context));
  expect(await result.current("5a")).toBeNull();
});
it.each([{ game: "chess" }, { origin: "4a" }, { read_only: false }])("rejects conflicting inspection identity %j", async (difference) => {
  vi.mocked(invoke).mockResolvedValue({ ...reply, ...difference });
  const { result } = renderHook(() => useMovementInspection(context)); expect(await result.current("5a")).toBeNull();
});
// Lifecycle cases now exercise the sole interaction owner, not a second query
// generation inside the stateless native identity adapter.
function useInspectionController(value: InspectionContext, play: () => void) {
  const inspect = useMovementInspection(value);
  const interaction = useNativeChoiceController({ contextKey: inspectionContextKey(value),
    choices: [], origins: ["5a", "4a"].map(key => ({ key, owner: "gote", inspectable: true })),
    humanOwner: "sente", disabled: true, inspectionDisabled: false, onInspect: inspect,
    onPlay: play, initialFocus: "5a", boardKeys: ["5a", "4a"], neighbor: () => undefined });
  return { inspect, interaction };
}
it("retains the native adapter and pending controller inspection through identical polls, retires on history change", async () => {
  const pending = deferred<MovementInspection>(); vi.mocked(invoke).mockReturnValue(pending.promise);
  const play = vi.fn();
  const { result, rerender } = renderHook(({ value }) => useInspectionController(value, play), { initialProps: { value: context } });
  const callback = result.current.inspect;
  act(() => result.current.interaction.select("5a"));
  rerender({ value: { ...context, token: { ...context.token } } });
  expect(result.current.inspect).toBe(callback); expect(result.current.interaction.inspectionPending).toBe(true);
  rerender({ value: { ...context, token: { ...context.token, selection: 3 } } });
  await act(async () => pending.resolve(reply));
  expect(result.current.interaction.inspection).toBeNull(); expect(result.current.interaction.inspectionPending).toBe(false);
  expect(play).not.toHaveBeenCalled();
});
it("retires superseded origin queries and unmounted work in the controller", async () => {
  const first = deferred<MovementInspection>(), second = deferred<MovementInspection>();
  vi.mocked(invoke).mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
  const play = vi.fn();
  const { result, unmount } = renderHook(() => useInspectionController(context, play));
  act(() => result.current.interaction.select("5a")); act(() => result.current.interaction.select("4a"));
  await act(async () => first.resolve(reply)); expect(result.current.interaction.inspection).toBeNull();
  expect(result.current.interaction.selected).toBe("4a");
  unmount(); await act(async () => second.resolve({ ...reply, origin: "4a" })); expect(play).not.toHaveBeenCalled();
});
it("contains native query rejection without routing a command or fabricating choices", async () => {
  vi.mocked(invoke).mockRejectedValue("stale native selection");
  const { result } = renderHook(() => useMovementInspection(context)); expect(await result.current("5a")).toBeNull();
  expect(invoke).toHaveBeenCalledTimes(1);
});
