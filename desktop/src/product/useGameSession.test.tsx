// @vitest-environment jsdom
import { StrictMode } from "react";
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useGameSession } from "./useGameSession";
import type { GameSnapshot } from "./native";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn() }));
const standard = standardJson as GameSnapshot;
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>((done) => { resolve = done; }); return { resolve, promise }; }
beforeEach(() => { vi.useFakeTimers(); vi.mocked(invoke).mockReset(); vi.mocked(listen).mockReset().mockResolvedValue(() => {}); });
afterEach(() => { cleanup(); vi.useRealTimers(); });
const flush = async () => { await act(async () => { await Promise.resolve(); }); };
it("keeps exact rendered authority when a newer native poll precedes an old click", async () => {
  const poll = deferred<GameSnapshot>(); const next = { ...standard, token: { ...standard.token, revision: standard.token.revision + 1, selection: standard.token.selection + 1 } };
  vi.mocked(invoke).mockResolvedValueOnce(standard).mockReturnValueOnce(poll.promise).mockRejectedValueOnce("Stale command");
  const { result } = renderHook(useGameSession); await flush();
  const oldHandler = () => result.current.command(standard, { kind: "play", action: "7g7f" });
  await act(async () => { await vi.advanceTimersByTimeAsync(180); });
  await act(async () => { poll.resolve(next); await Promise.resolve(); await oldHandler(); });
  expect(invoke).toHaveBeenLastCalledWith("game_command", { commandJson: JSON.stringify({ expected: standard.token, game: "shogi", operation: { kind: "play", action: "7g7f" } }) });
  expect(result.current.snapshot).toBe(next); expect(result.current.error).toContain("Stale");
});
it("does not let a delayed clean Save authorize discarding newer unsaved work", async () => {
  const save = deferred<GameSnapshot>(), poll = deferred<GameSnapshot>();
  const clean = { ...standard, token: { ...standard.token, revision: standard.token.revision + 1 }, document: { ...standard.document, dirty: false } };
  const dirty = { ...standard, token: { ...standard.token, revision: standard.token.revision + 2 }, document: { ...standard.document, dirty: true, revision: standard.document.revision + 1 } };
  vi.mocked(invoke).mockResolvedValueOnce(standard).mockReturnValueOnce(save.promise).mockReturnValueOnce(poll.promise);
  const { result } = renderHook(useGameSession); await flush();
  let pending!: Promise<GameSnapshot | null>; act(() => { pending = result.current.file(standard, "save"); });
  expect(invoke).toHaveBeenLastCalledWith("game_file", { action: "save", expected: standard.open_token });
  await act(async () => { await vi.advanceTimersByTimeAsync(180); poll.resolve(dirty); await Promise.resolve(); });
  let returned: GameSnapshot | null = null; await act(async () => { save.resolve(clean); returned = await pending; });
  expect(returned).toBe(dirty); expect(result.current.snapshot).toBe(dirty);
});
it("never returns a receipt from a replaced document as current", async () => {
  const action = deferred<GameSnapshot>(), poll = deferred<GameSnapshot>();
  const replacement = { ...standard, token: { ...standard.token, document: standard.token.document + 1, revision: standard.token.revision + 3 } };
  vi.mocked(invoke).mockResolvedValueOnce(standard).mockReturnValueOnce(action.promise).mockReturnValueOnce(poll.promise);
  const { result } = renderHook(useGameSession); await flush();
  let pending!: Promise<GameSnapshot | null>; act(() => { pending = result.current.command(standard, { kind: "analyze", operation: { kind: "moves", depth: 1, nodes: 10 } }); });
  await act(async () => { await vi.advanceTimersByTimeAsync(180); poll.resolve(replacement); await Promise.resolve(); });
  let returned: GameSnapshot | null = standard; await act(async () => { action.resolve(standard); returned = await pending; });
  expect(returned).toBeNull(); expect(result.current.snapshot).toBe(replacement);
});
it("serializes commands and binds new-game and file operations to native preconditions", async () => {
  const command = deferred<GameSnapshot>(); vi.mocked(invoke).mockResolvedValueOnce(standard).mockReturnValueOnce(command.promise).mockResolvedValue(standard);
  const { result } = renderHook(useGameSession); await flush();
  let pending!: Promise<GameSnapshot | null>; act(() => { pending = result.current.command(standard, { kind: "resume" }); });
  expect(result.current.busy).toBe(true);
  await act(async () => { expect(await result.current.file(standard, "open")).toBeNull(); });
  expect(invoke).toHaveBeenCalledTimes(2);
  await act(async () => { command.resolve(standard); await pending; });
  await act(async () => { await result.current.newGame(standard.open_token, "shogi", { game_kind: "shogi", side: "gote" }); });
  expect(invoke).toHaveBeenLastCalledWith("game_new", { expected: standard.open_token, game: "shogi", human: { game_kind: "shogi", side: "gote" } });
});
it("fetches authoritative paused shogi metadata before opening the native quit guard", async () => {
  let close!: () => void;
  vi.mocked(listen).mockImplementationOnce(async (_name, handler) => { close = handler as typeof close; return () => {}; });
  const stopped = { ...standard, token: { ...standard.token, revision: standard.token.revision + 1 }, paused: true, document: { ...standard.document, revision: standard.document.revision + 1 } };
  vi.mocked(invoke).mockResolvedValueOnce(standard).mockResolvedValueOnce(stopped);
  const { result } = renderHook(useGameSession); await flush();
  await act(async () => { close(); await Promise.resolve(); });
  expect(result.current.snapshot).toBe(stopped); expect(result.current.closeRequested).toBe(true);
  vi.mocked(invoke).mockResolvedValueOnce(undefined);
  await act(async () => { await result.current.finishClose(stopped); });
  expect(invoke).toHaveBeenLastCalledWith("desktop_finish_close", { expectedDocumentRevision: stopped.document.revision });
});
it("reports native close-protection registration failure without an unhandled rejection", async () => {
  vi.mocked(listen).mockRejectedValueOnce("Listener unavailable"); vi.mocked(invoke).mockResolvedValue(standard);
  const { result } = renderHook(useGameSession); await flush();
  expect(result.current.error).toMatch(/close|listener/i);
});

it("routes native close by the current game when Open precedes the next shared poll", async () => {
  let close!: () => void;
  vi.mocked(listen).mockImplementationOnce(async (_name, handler) => { close = handler as typeof close; return () => {}; });
  const chess = { ...standard, game: "chess" as const };
  const opened = { ...standard, token: { ...standard.token, document: standard.token.document + 1, revision: standard.token.revision + 1 }, paused: true };
  vi.mocked(invoke).mockResolvedValueOnce(chess).mockResolvedValueOnce(opened);
  const { result } = renderHook(useGameSession); await flush();
  expect(result.current.snapshot?.game).toBe("chess");
  await act(async () => { close(); await Promise.resolve(); });
  expect(result.current.snapshot).toBe(opened); expect(result.current.closeRequested).toBe(true);
});

it("retains the product close request when a Chess replacement precedes its view mounting", async () => {
  let close!: () => void;
  vi.mocked(listen).mockImplementationOnce(async (_name, handler) => { close = handler as typeof close; return () => {}; });
  const chess = { ...standard, game: "chess" as const, token: { ...standard.token, document: standard.token.document + 1, revision: standard.token.revision + 1 } };
  vi.mocked(invoke).mockResolvedValueOnce(standard).mockResolvedValueOnce(chess);
  const { result } = renderHook(useGameSession); await flush();
  await act(async () => { close(); await Promise.resolve(); });
  expect(result.current.snapshot).toBe(chess); expect(result.current.closeRequested).toBe(true);
});

it("retires StrictMode effect callbacks, poll loops and delayed native listeners", async () => {
  const retiredPoll = deferred<GameSnapshot>(), activePoll = deferred<GameSnapshot>();
  const retiredListener = deferred<() => void>(), activeListener = deferred<() => void>();
  const retiredStop = vi.fn(), activeStop = vi.fn();
  const callbacks: (() => void)[] = [];
  vi.mocked(listen).mockImplementation((_name, handler) => {
    callbacks.push(handler as () => void);
    return callbacks.length === 1 ? retiredListener.promise : activeListener.promise;
  });
  vi.mocked(invoke).mockReturnValueOnce(retiredPoll.promise).mockReturnValueOnce(activePoll.promise).mockResolvedValue(standard);
  const { result, unmount } = renderHook(useGameSession, { wrapper: StrictMode });
  expect(invoke).toHaveBeenCalledTimes(2);
  const retiredNewer = { ...standard, token: { ...standard.token, revision: standard.token.revision + 99 } };
  await act(async () => { retiredPoll.resolve(retiredNewer); retiredListener.resolve(retiredStop); activePoll.resolve(standard); activeListener.resolve(activeStop); await Promise.resolve(); });
  expect(retiredStop).toHaveBeenCalledTimes(1); expect(activeStop).not.toHaveBeenCalled();
  expect(result.current.snapshot).toBe(standard);
  const prior = vi.mocked(invoke).mock.calls.length;
  await act(async () => { callbacks[0]!(); await Promise.resolve(); });
  expect(invoke).toHaveBeenCalledTimes(prior); expect(result.current.closeRequested).toBe(false);
  await act(async () => { await vi.advanceTimersByTimeAsync(180); });
  expect(invoke).toHaveBeenCalledTimes(prior + 1);
  unmount(); expect(activeStop).toHaveBeenCalledTimes(1);
  await vi.advanceTimersByTimeAsync(1000); expect(invoke).toHaveBeenCalledTimes(prior + 1);
});
