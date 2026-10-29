// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { XiangqiApp } from "./XiangqiApp";
import type { XiangqiSnapshot as GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import standardJson from "../__fixtures__/shared-xiangqi-standard.json";
import playedJson from "../__fixtures__/shared-xiangqi-played.json";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const standard = standardJson as unknown as GameSnapshot;
afterEach(cleanup);
function app(snapshot = standard) {
  const session: GameSession = { snapshot, busy: false, error: "", closeRequested: false,
    command: vi.fn().mockResolvedValue(snapshot), file: vi.fn().mockResolvedValue(snapshot),
    newGame: vi.fn().mockResolvedValue(snapshot), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true) };
  const props = { snapshot, session, onNewGame: vi.fn(), onOpen: vi.fn(), blocked: false };
  return { ...render(<XiangqiApp {...props} />), props, session };
}
function point(origin: string) {
  const node = document.querySelector<SVGGElement>(`[data-intersection="${origin}"]`);
  if (!node) throw Error(`Missing intersection ${origin}`); return node;
}
it("uses the accepted shell, actual native play limits and shared document/status hierarchy", () => {
  const { container } = app();
  expect(container.querySelector(".game-shell")?.getAttribute("data-theme")).toBe("mshogi-dark");
  expect(screen.getByRole("heading", { name: "mShogi", level: 1 })).toBeTruthy();
  expect(screen.getByText("XIANGQI · UNTIMED")).toBeTruthy();
  expect(screen.getByRole("heading", { name: "Untitled xiangqi game" })).toBeTruthy();
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved");
  expect(container.querySelector("[data-play-limits]")?.textContent).toBe("Play limit: depth 1 · 128 nodes");
  expect([...container.querySelectorAll("[data-shell-section]")].map((node) => node.getAttribute("data-shell-section"))).toEqual(["analysis", "history", "actions"]);
  expect(container.querySelector(".shell-sidebar > details")?.textContent).toContain("About & technical details");
  expect([...container.querySelector(".shell-game-area")!.children].map((node) => node.getAttribute("data-game-slot"))).toEqual(["upper-player", "board", "lower-player", "footer"]);
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Your turnLive position · 0 plies");
  for (const name of ["New game", "Save", "Save as…", "Open…"]) expect(screen.getByRole("button", { name }).closest(".shell-header")).toBeTruthy();
  expect(screen.queryByText(/Entering.king|pieces in hand|Promote/)).toBeNull();
  expect(screen.queryByRole("spinbutton")).toBeNull();
  expect((screen.getByRole("button", { name: "Analyze position" }) as HTMLButtonElement).disabled).toBe(true);
});
it("sends the opaque native action through the Xiangqi command wrapper, never generic Shogi play", () => {
  const { session } = app();
  fireEvent.click(point("e3")); fireEvent.click(point("e4"));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "xiangqi", command: { kind: "play", action: "e3e4" } });
});
it("flips using native Xiangqi preferences while preserving existing document and play controls", () => {
  const { session, props } = app();
  fireEvent.click(screen.getByRole("button", { name: "Flip board; currently Red at bottom" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "xiangqi_preferences", settings: { orientation: "black", analysis_defaults: { depth: 1, nodes: 128 } } });
  fireEvent.click(screen.getByRole("button", { name: "New game" })); expect(props.onNewGame).toHaveBeenCalledOnce();
  fireEvent.click(screen.getByRole("button", { name: "Save" })); expect(session.file).toHaveBeenLastCalledWith(standard, "save");
  fireEvent.click(screen.getByRole("button", { name: "Save as…" })); expect(session.file).toHaveBeenLastCalledWith(standard, "save_as");
  fireEvent.click(screen.getByRole("button", { name: "Open…" })); expect(props.onOpen).toHaveBeenCalledOnce();
});
it("shows real engine work and cancellation without fabricated telemetry or analysis execution", () => {
  const snapshot = { ...standard, phase: "thinking" as const, execution_purpose: "engine_turn", capabilities: { ...standard.capabilities, board_action: false } };
  const { session } = app(snapshot);
  expect(screen.getByRole("status", { name: "Engine turn status" }).textContent).toBe("Searching");
  expect(screen.queryByText("Completed depth")).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" })); expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Cancel search" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "cancel" });
});
it("keeps resource-limit refusal distinct and retains a complete Save path", () => {
  const snapshot = { ...standard, capacity: { ...standard.capacity, status: "paused" as const, message: "256 observations" } };
  const { session } = app(snapshot);
  expect(screen.getByRole("heading", { name: "Application limit reached" })).toBeTruthy();
  expect(screen.getByText(/saving and reopening does not reset this limit/)).toBeTruthy();
  fireEvent.click(point("e3")); fireEvent.click(point("e4")); expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Save" })); expect(session.file).toHaveBeenCalledExactlyOnceWith(snapshot, "save");
});
it("shows native readable capture history without exposing the canonical diagnostic payload", () => {
  const snapshot = playedJson as unknown as GameSnapshot;
  app(snapshot);
  const history = screen.getByRole("region", { name: "Game history" });
  expect(history.textContent).toContain("Red: b2×b9");
  expect(history.textContent).toContain("Black: a9×b9");
  expect(history.textContent).not.toMatch(/Some\(|BoardAction|captured_id|moving_id/);
  expect(snapshot.events.map((e) => [e.index, e.board_ply])).toEqual([[1, 1], [2, 2]]);
  expect(snapshot.context).toEqual(playedJson.context);
});
