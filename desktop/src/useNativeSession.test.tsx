// @vitest-environment jsdom
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useNativeSession } from "./useNativeSession";
import type { DesktopCommand, Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));
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

it("returns current dirty state rather than an older successful-save response", async () => {
  const saved = deferred<Snapshot>();
  const polled = deferred<Snapshot>();
  const clean = {
    ...standard,
    revision: 4,
    document: { ...standard.document, dirty: false },
  };
  const newer = {
    ...standard,
    revision: 5,
    document: { ...standard.document, revision: 8, dirty: true },
  };
  vi.mocked(invoke)
    .mockResolvedValueOnce(standard)
    .mockReturnValueOnce(saved.promise)
    .mockReturnValueOnce(polled.promise);
  const { result } = renderHook(useNativeSession);
  await act(async () => {
    await Promise.resolve();
  });
  let completion!: Promise<Snapshot | null>;
  act(() => {
    completion = result.current.save();
  });
  await act(async () => {
    await vi.advanceTimersByTimeAsync(250);
    polled.resolve(newer);
    await Promise.resolve();
  });
  let observed: Snapshot | null = null;
  await act(async () => {
    saved.resolve(clean);
    observed = await completion;
  });
  expect(observed).toBe(newer);
  expect(result.current.snapshot).toBe(newer);
});
it("applies the native close snapshot before exposing the close request", async () => {
  vi.mocked(invoke).mockResolvedValue(standard);
  let close!: (event: { payload: Snapshot }) => void;
  vi.mocked(listen).mockImplementationOnce(async (_name, handler) => {
    close = handler as typeof close;
    return () => {};
  });
  const { result } = renderHook(useNativeSession);
  await act(async () => {
    await Promise.resolve();
  });
  const paused = {
    ...standard,
    version: 7,
    revision: 12,
    play_paused: true,
    document: { ...standard.document, dirty: true, revision: 9 },
  };
  act(() => {
    close({ payload: paused });
  });
  expect(result.current.snapshot).toBe(paused);
  expect(result.current.closeRequest).toBe(paused);
});
it("uses exact native Open preconditions and refreshes forms only after a completed Open, not picker cancellation", async () => {
  const opened = {
    ...standard,
    revision: 8,
    io: { id: 2, phase: "complete" as const, message: null },
  };
  vi.mocked(invoke)
    .mockResolvedValueOnce(standard)
    .mockResolvedValueOnce(standard)
    .mockResolvedValueOnce(opened);
  const { result } = renderHook(useNativeSession);
  await act(async () => {
    await Promise.resolve();
  });
  await act(async () => {
    await result.current.open(standard.version, standard.document.revision);
  });
  expect(result.current.documentEpoch).toBe(0);
  expect(vi.mocked(invoke)).toHaveBeenLastCalledWith("desktop_open", {
    expectedVersion: standard.version,
    expectedDocumentRevision: standard.document.revision,
  });
  await act(async () => {
    await result.current.open(standard.version, standard.document.revision);
  });
  expect(result.current.documentEpoch).toBe(1);
});
it("does not enable game interaction when native close protection cannot register", async () => {
  vi.mocked(listen).mockRejectedValueOnce("Listener unavailable");
  const { result } = renderHook(useNativeSession);
  await act(async () => {
    await Promise.resolve();
  });
  expect(result.current.connected).toBe(false);
  expect(result.current.snapshot).toBeNull();
  expect(result.current.error).toContain("Native close protection");
  expect(invoke).not.toHaveBeenCalled();
});
