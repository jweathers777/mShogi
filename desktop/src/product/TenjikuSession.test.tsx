// @vitest-environment jsdom
// Transport identities may be forged for negative cases; native choices are never invented.
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { act, cleanup, renderHook } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useGameSession } from "./useGameSession";
import { useMovementInspection } from "./useMovementInspection";
import { inspectionContextKey, type MovementInspection } from "./movementInspection";
import { gridNeighbor, useNativeChoiceController } from "../interaction/useNativeChoiceController";
import { nativeSnapshotIsNewer } from "../NativeBoard";
import type { GameSnapshot, TenjikuSnapshot } from "./native";
import adamsStandard from "../__fixtures__/shared-tenjiku-adams-standard.json";
import mullerStandard from "../__fixtures__/shared-tenjiku-muller-standard.json";
import adamsReset from "../__fixtures__/shared-tenjiku-adams-reset.json";
import adamsSelected from "../__fixtures__/shared-tenjiku-adams-selected-root.json";
import mullerSelected from "../__fixtures__/shared-tenjiku-muller-selected-root.json";
import adamsInspection from "../__fixtures__/shared-tenjiku-adams-inspection.json";
import mullerInspection from "../__fixtures__/shared-tenjiku-muller-inspection.json";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn() }));
const adams = adamsStandard as unknown as TenjikuSnapshot, muller = mullerStandard as unknown as TenjikuSnapshot;
const reset = adamsReset as unknown as TenjikuSnapshot;
const cases = [
  { source: adams, other: muller, selected: adamsSelected as unknown as TenjikuSnapshot, foreign: mullerSelected as unknown as TenjikuSnapshot, reply: adamsInspection as unknown as MovementInspection },
  { source: muller, other: adams, selected: mullerSelected as unknown as TenjikuSnapshot, foreign: adamsSelected as unknown as TenjikuSnapshot, reply: mullerInspection as unknown as MovementInspection },
];
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(done => { resolve = done; }); return { promise, resolve }; }
const flush = async () => { await act(async () => { await Promise.resolve(); }); };
beforeEach(() => { vi.useFakeTimers(); vi.mocked(invoke).mockReset(); vi.mocked(listen).mockReset().mockResolvedValue(() => {}); });
afterEach(() => { cleanup(); vi.useRealTimers(); });

it("accepts actual native fresh New profile epochs and resetting to the first profile", async () => {
  expect(muller.token.document).toBeGreaterThan(adams.token.document);
  expect(reset.token.document).toBeGreaterThan(muller.token.document);
  expect(reset.profile).toBe(adams.profile);
  expect(nativeSnapshotIsNewer(adams, muller)).toBe(true);
  expect(nativeSnapshotIsNewer(muller, reset)).toBe(true);
  vi.mocked(invoke).mockResolvedValueOnce(adams).mockResolvedValueOnce(muller).mockResolvedValueOnce(reset);
  const { result } = renderHook(() => useGameSession()); await flush();
  await act(async () => { expect(await result.current.newGame(adams.open_token, "tenjiku", adams.human.id, muller.profile)).toBe(muller); });
  expect(invoke).toHaveBeenLastCalledWith("game_new", { expected: adams.open_token, game: "tenjiku", profile: muller.profile, human: adams.human.id });
  expect(result.current.snapshot).toBe(muller);
  await act(async () => { expect(await result.current.newGame(muller.open_token, "tenjiku", muller.human.id, reset.profile)).toBe(reset); });
  expect(invoke).toHaveBeenLastCalledWith("game_new", { expected: muller.open_token, game: "tenjiku", profile: reset.profile, human: muller.human.id });
  expect(result.current.snapshot).toBe(reset);
});

it.each(cases)("rejects a same-epoch $source.profile response carrying the other native profile", async ({ source, other }) => {
  const forged = { ...other, token: { ...other.token, document: source.token.document, revision: source.token.revision + 1 } };
  expect(nativeSnapshotIsNewer(source, forged)).toBe(false);
  vi.mocked(invoke).mockResolvedValueOnce(source).mockResolvedValueOnce(forged);
  const { result } = renderHook(() => useGameSession()); await flush();
  await act(async () => { expect(await result.current.command(source, { kind: "resume" })).toBeNull(); });
  expect(result.current.snapshot).toBe(source);
  expect(result.current.error).toContain("immutable rules profile");
});

it.each(cases)("keeps exact $source.profile rendered command and normal file authority", async ({ source }) => {
  vi.mocked(invoke).mockResolvedValue(source);
  const { result } = renderHook(() => useGameSession()); await flush();
  await act(async () => { await result.current.command(source, { kind: "resume" }); });
  expect(invoke).toHaveBeenLastCalledWith("game_command", { commandJson: JSON.stringify({ expected: source.token, game: "tenjiku", operation: { kind: "resume" } }) });
  await act(async () => { await result.current.file(source, "open"); });
  expect(invoke).toHaveBeenLastCalledWith("game_file", { action: "open", expected: source.open_token });
});

it.each(cases)("does not let a delayed $source.profile receipt restore an earlier profile after fresh New", async ({ source, other }) => {
  const fresh = other.token.document > source.token.document ? other : reset;
  expect(fresh.token.document).toBeGreaterThan(source.token.document);
  const receipt = deferred<GameSnapshot>(), poll = deferred<GameSnapshot>();
  vi.mocked(invoke).mockResolvedValueOnce(source).mockReturnValueOnce(receipt.promise).mockReturnValueOnce(poll.promise);
  const { result } = renderHook(() => useGameSession()); await flush();
  let pending!: Promise<GameSnapshot | null>;
  act(() => { pending = result.current.command(source, { kind: "resume" }); });
  await act(async () => { await vi.advanceTimersByTimeAsync(180); poll.resolve(fresh); await Promise.resolve(); });
  await act(async () => { receipt.resolve(source); expect(await pending).toBeNull(); });
  expect(result.current.snapshot).toBe(fresh);
});

it.each(cases)("validates exact native $source.profile history inspection including profile identity", async ({ selected, foreign, reply }) => {
  expect(reply.profile).toBe(selected.profile);
  expect(reply.token).toEqual(selected.token);
  vi.mocked(invoke).mockResolvedValueOnce(reply).mockResolvedValueOnce({ ...reply, profile: foreign.profile });
  const { result } = renderHook(() => useMovementInspection(selected));
  expect(await result.current(reply.origin)).toEqual(reply);
  expect(invoke).toHaveBeenLastCalledWith("game_inspection", { queryJson: JSON.stringify({ expected: selected.token, origin: reply.origin }) });
  expect(await result.current(reply.origin)).toBeNull();
  for (const field of ["document", "version", "revision", "selection"] as const) {
    vi.mocked(invoke).mockResolvedValueOnce({ ...reply, token: { ...reply.token, [field]: reply.token[field] + 1 } });
    expect(await result.current(reply.origin)).toBeNull();
  }
});

function useHistoricalInspection(source: TenjikuSnapshot, play: () => void) {
  const inspect = useMovementInspection(source), orientation = source.orientations[0]!;
  const keys = orientation.ranks.flatMap(rank => orientation.files.map(file => `${file}${rank}`));
  return useNativeChoiceController({ contextKey: inspectionContextKey(source), choices: source.choices,
    origins: source.pieces.map(piece => ({ key: piece.square, owner: piece.owner, inspectable: true })),
    humanOwner: source.human.key, disabled: true, inspectionDisabled: false, onInspect: inspect, onPlay: play,
    initialFocus: source.board.initial_focus, boardKeys: keys,
    neighbor: (key, direction) => gridNeighbor(keys, orientation.files.length, key, direction) });
}

it.each(cases)("retires pending $source.profile inspection on native history/profile change without creating actions", async ({ selected, foreign, reply }) => {
  const pending = deferred<MovementInspection>(), play = vi.fn();
  vi.mocked(invoke).mockReturnValueOnce(pending.promise);
  const { result, rerender } = renderHook(({ source }) => useHistoricalInspection(source, play), { initialProps: { source: selected } });
  act(() => result.current.select(reply.origin));
  expect(result.current.inspectionPending).toBe(true);
  rerender({ source: { ...selected, token: { ...selected.token } } });
  expect(result.current.inspectionPending).toBe(true);
  rerender({ source: foreign });
  await act(async () => pending.resolve(reply));
  expect(result.current.inspection).toBeNull();
  expect(result.current.inspectionPending).toBe(false);
  expect(result.current.selected).toBeNull();
  expect(play).not.toHaveBeenCalled();
});
