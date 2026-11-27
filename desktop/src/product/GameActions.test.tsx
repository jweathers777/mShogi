// @vitest-environment jsdom
import { deferred, fakeSession, renderApplication } from "../testing/helpers";
import type { GameSnapshot } from "../sessionTypes";
import { afterEach, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, screen } from "@testing-library/react";
import { ApplicationView } from "./ApplicationView";

import chess from "../__fixtures__/shared-chess-standard.json";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
import unresolved from "../__fixtures__/shared-chess-unresolved.json";
import resigned from "../__fixtures__/shared-xiangqi-resigned.json";
afterEach(cleanup);
function app(snapshot: GameSnapshot) {
  return renderApplication(snapshot, fakeSession(snapshot, { command: vi.fn().mockResolvedValue(null) }));
}
it.each([chess, shogi, xiangqi])("submits $game Resign directly once with captured session authority and preserves a refused game", async fixture => {
  const source = fixture as GameSnapshot; const { session } = app(source);
  const receipt = deferred<GameSnapshot | null>(); vi.mocked(session.command).mockReturnValue(receipt.promise);
  const resign = screen.getByRole("button", { name: "Resign" });
  fireEvent.click(resign); fireEvent.click(resign);
  expect(session.command).toHaveBeenCalledExactlyOnceWith(source, { kind: "resign" });
  expect(screen.queryByRole("dialog")).toBeNull(); expect(screen.queryByRole("region", { name: "Game outcome" })).toBeNull();
  await act(async () => { receipt.resolve(null); await receipt.promise; });
  expect(screen.getByRole("button", { name: "Resign" })).toBe(resign); expect(session.snapshot).toBe(source);
});
it.each([chess, shogi, xiangqi])("does not admit $game Resign while another shared operation owns execution", fixture => {
  const { session, props, rerender } = app(fixture as GameSnapshot);
  session.busy = true; rerender(<ApplicationView {...props} />);
  const resign = screen.getByRole("button", { name: "Resign" }) as HTMLButtonElement;
  expect(resign.disabled).toBe(true); fireEvent.click(resign); expect(session.command).not.toHaveBeenCalled();
});
it("presents actual unresolved Chess resignation without inventing a winner or score", () => {
  app(unresolved as GameSnapshot);
  const outcome = screen.getByRole("region", { name: "Game outcome" });
  expect(outcome.textContent).toContain("Unresolved — unscored");
  expect(outcome.textContent).toContain("resigned"); expect(outcome.textContent).toContain("no score is assigned");
  expect(outcome.textContent).not.toMatch(/wins|White loses|Black loses|rules determine/);
  expect(screen.queryByRole("button", { name: "Resign" })).toBeNull();
});
it("presents the actual Xiangqi resignation conclusion rather than an optimistic local result", () => {
  const snapshot = resigned as unknown as GameSnapshot; app(snapshot);
  expect(screen.getByRole("region", { name: "Game outcome" }).textContent).toContain(snapshot.status.label);
  expect(screen.queryByRole("button", { name: "Resign" })).toBeNull();
});
