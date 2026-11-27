// @vitest-environment jsdom
import { fakeSession, renderApplication } from "../testing/helpers";
import type { GameSnapshot } from "../sessionTypes";
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, screen } from "@testing-library/react";

import chess from "../__fixtures__/shared-chess-standard.json";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
import tori from "../__fixtures__/shared-tori-contract.json";
import chu from "../__fixtures__/shared-chu-routes.json";
afterEach(cleanup);
const fixtures = [chess, shogi, xiangqi, tori, chu] as unknown as GameSnapshot[];
function paused(fixture: GameSnapshot) {
  const snapshot = structuredClone(fixture);
  snapshot.capacity = { id: 7, status: "paused", message: "Ordinary work cannot be saved within capacity.",
    resource: { fact: { category: "game_root_bytes", measurement: "candidate", count: 8192, reserved: 1, limit: 8192,
      scope: snapshot.game, unit: null }, diagnostic: "Control limit; reserved endings remain." } };
  return snapshot;
}
function app(snapshot: GameSnapshot, busy = false, blocked = false) {
  const command = vi.fn().mockResolvedValue(null);
  return { ...renderApplication(snapshot, fakeSession(snapshot, { busy, command }), blocked), command };
}
it.each(fixtures)("keeps $game resignation and Save available at an ordinary capacity pause", fixture => {
  const snapshot = paused(fixture), before = structuredClone(snapshot.context);
  expect(snapshot.capabilities.resign).toBe(true);
  const { command, container } = app(snapshot);
  const resign = screen.getByRole("button", { name: "Resign" }) as HTMLButtonElement;
  expect(resign.disabled).toBe(false);
  expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect((container.querySelector('button[data-analysis-start]') as HTMLButtonElement).disabled).toBe(true);
  expect([...container.querySelectorAll<HTMLButtonElement>('.shogi-game-actions button:not(.resign-button)')].every(button => button.disabled)).toBe(true);
  fireEvent.click(resign); fireEvent.click(resign);
  expect(command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "resign" });
  // A refused/pending command cannot optimistically conclude the game.
  expect(snapshot.context).toEqual(before);
  expect(screen.queryByLabelText("Game outcome")).toBeNull();
});
it.each(fixtures)("retains $game busy, modal and admission protection for reserved resignation", fixture => {
  const snapshot = paused(fixture);
  for (const [busy, blocked, checking] of [[true, false, false], [false, true, false], [false, false, true]]) {
    const state = structuredClone(snapshot);
    if (checking) state.capacity = { id: 8, status: "checking", message: null, resource: null };
    const { command, unmount } = app(state, busy, blocked);
    const resign = screen.getByRole("button", { name: "Resign" }) as HTMLButtonElement;
    expect(resign.disabled).toBe(true); fireEvent.click(resign); expect(command).not.toHaveBeenCalled(); unmount();
  }
});
