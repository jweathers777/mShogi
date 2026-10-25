// @vitest-environment jsdom
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { useNativeSession } from "./useNativeSession";
import type { DesktopCommand, Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const standard = standardJson as Snapshot;
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => {
    resolve = done;
  });
  return { promise, resolve };
}
beforeEach(() => {
  vi.useFakeTimers();
  vi.mocked(invoke).mockReset();
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});
describe("native async session boundary", () => {
  it("cannot replace a newer command snapshot with a delayed older poll", async () => {
    const olderPoll = deferred<Snapshot>();
    const newer = { ...standard, revision: 5, version: 4, history: ["e2e4"] };
    vi.mocked(invoke)
      .mockResolvedValueOnce(standard)
      .mockReturnValueOnce(olderPoll.promise)
      .mockResolvedValueOnce(newer);
    const { result } = renderHook(useNativeSession);
    await act(async () => {
      await Promise.resolve();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    await act(async () => {
      await result.current.command({
        kind: "play",
        version: standard.version,
        action: "e2e4",
      });
    });
    await act(async () => {
      olderPoll.resolve({ ...standard, revision: 2 });
      await Promise.resolve();
    });
    expect(result.current.snapshot).toBe(newer);
  });
  it("never upgrades an old rendered action to a newer observed version before React commits", async () => {
    const nextPoll = deferred<Snapshot>();
    const newer = { ...standard, revision: 5, version: 4 };
    vi.mocked(invoke)
      .mockResolvedValueOnce(standard)
      .mockReturnValueOnce(nextPoll.promise)
      .mockRejectedValueOnce("The game changed.");
    const { result } = renderHook(useNativeSession);
    await act(async () => {
      await Promise.resolve();
    });
    const renderedCommand: DesktopCommand = {
      kind: "play",
      version: result.current.snapshot!.version,
      action: "e2e4",
    };
    const renderedHandler = () => result.current.command(renderedCommand);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(250);
    });
    await act(async () => {
      nextPoll.resolve(newer);
      await Promise.resolve(); // native response observed; React updates remain batched
      await renderedHandler();
    });
    expect(vi.mocked(invoke)).toHaveBeenLastCalledWith("desktop_command", {
      command: { kind: "play", version: 1, action: "e2e4" },
    });
    expect(result.current.snapshot).toBe(newer);
    expect(result.current.error).toBe("The game changed.");
  });
});
