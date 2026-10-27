// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { act, cleanup, renderHook } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { useMovementInspection } from "./useMovementInspection";
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
it("retains callback and request on identical polling context, retires it on history change", async () => {
  const pending = deferred<MovementInspection>(); vi.mocked(invoke).mockReturnValue(pending.promise);
  const { result, rerender } = renderHook(({ value }) => useMovementInspection(value), { initialProps: { value: context } });
  const callback = result.current; const request = callback("5a");
  rerender({ value: { ...context, token: { ...context.token } } }); expect(result.current).toBe(callback);
  rerender({ value: { ...context, token: { ...context.token, selection: 3 } } });
  await act(async () => pending.resolve(reply)); expect(await request).toBeNull();
});
it("rejects superseded origin requests and requests retired by unmount or replacement", async () => {
  const first = deferred<MovementInspection>(), second = deferred<MovementInspection>();
  vi.mocked(invoke).mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
  const { result, unmount } = renderHook(() => useMovementInspection(context));
  const old = result.current("5a"), current = result.current("4a");
  first.resolve(reply); expect(await old).toBeNull();
  unmount(); second.resolve({ ...reply, origin: "4a" }); expect(await current).toBeNull();
});
it("contains native query rejection without routing a command or fabricating choices", async () => {
  vi.mocked(invoke).mockRejectedValue("stale native selection");
  const { result } = renderHook(() => useMovementInspection(context)); expect(await result.current("5a")).toBeNull();
  expect(invoke).toHaveBeenCalledTimes(1);
});
