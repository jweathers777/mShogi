// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { ShogiApp } from "./ShogiApp";
import type { ChessShogiSnapshot as GameSnapshot, SearchView } from "./native";
import type { GameSession } from "./useGameSession";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
afterEach(cleanup);
const standard = standardJson as GameSnapshot;
it("uses disk-aware document labels, supplied play budget and native selected board ply", () => {
  const snapshot = { ...standard, document: { ...standard.document, path: null, dirty: false }, play_limits: { depth: 3, nodes: 1234 } };
  const { container, props, rerender } = app(snapshot);
  fireEvent.click(screen.getByRole("button", { name: "Play details" }));
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved");
  expect(container.querySelector("[data-play-limits]")?.textContent).toBe("Play limit: depth 3 · 1,234 nodes");
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Your turnLive position · 0 plies");
  const history = { ...snapshot, historical: true, event_index: 2, board_ply: 1, paused: true, document: { ...snapshot.document, path: "/games/shogi.mshogi", dirty: false } };
  rerender(<ShogiApp {...props} snapshot={history} />);
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unmodified");
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Read-only history · return to live to playSelected position · 1 ply");
  rerender(<ShogiApp {...props} snapshot={{ ...history, document: { ...history.document, dirty: true } }} />);
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved changes");
});
function app(snapshot = standard, error = "") {
  const session: GameSession = { snapshot, busy: false, error, closeRequested: false, command: vi.fn().mockResolvedValue(snapshot), file: vi.fn().mockResolvedValue(snapshot), newGame: vi.fn().mockResolvedValue(snapshot), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true) };
  const props = { snapshot, session, onNewGame: vi.fn(), onOpen: vi.fn(), blocked: false };
  const rendered = render(<ShogiApp {...props} />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  return { ...rendered, props, session };
}
it("allows clearing and typing numeric work limits without inserting a leading zero", () => {
  const { session } = app();
  const depth = screen.getByRole("spinbutton", { name: "Depth" }) as HTMLInputElement;
  const nodes = screen.getByRole("spinbutton", { name: "Node limit" }) as HTMLInputElement;
  fireEvent.change(depth, { target: { value: "" } });
  expect(depth.value).toBe("");
  fireEvent.change(depth, { target: { value: "4" } });
  fireEvent.change(nodes, { target: { value: "" } });
  expect(nodes.value).toBe("");
  fireEvent.change(nodes, { target: { value: "20000" } });
  expect(depth.value).toBe("4"); expect(nodes.value).toBe("20000");
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "analyze", operation: { kind: "moves", depth: 4, nodes: 20000 } });
});
it("routes bounded analysis through the read-only operation and cancellation path", () => {
  const { session, props, rerender } = app();
  fireEvent.change(screen.getByRole("spinbutton", { name: "Depth" }), { target: { value: "3" } });
  fireEvent.change(screen.getByRole("spinbutton", { name: "Node limit" }), { target: { value: "700" } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "analyze", operation: { kind: "moves", depth: 3, nodes: 700 } });
  const analyzing = { ...standard, phase: "thinking" as const, execution_purpose: "move_analysis", paused: true };
  rerender(<ShogiApp {...props} snapshot={analyzing} />);
  expect(screen.getByText("Analyzing the selected position…")).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Cancel search" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Replace analysis" }));
  expect(session.command).toHaveBeenLastCalledWith(analyzing, { kind: "analyze", operation: { kind: "moves", depth: 3, nodes: 700 } });
  fireEvent.click(screen.getByRole("button", { name: "Cancel analysis" }));
  expect(session.command).toHaveBeenLastCalledWith(analyzing, { kind: "cancel" });
});
it("keeps engine-turn cancellation distinct from analysis and invents no progress", () => {
  const snapshot = { ...standard, phase: "thinking" as const, execution_purpose: "engine_turn" };
  const { session } = app(snapshot);
  expect(screen.getByRole("button", { name: "Cancel search" })).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Cancel analysis" })).toBeNull();
  expect(screen.queryByText("Completed depth")).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Cancel search" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "cancel" });
});
it("selects exact history event identities and requires explicit Return to live and Resume", () => {
  // Presentation-only DTO states: same board ply, distinct native event indices.
  const snapshot = { ...standard, events: [{ index: 1, board_ply: 0, label: "Sente resigned" }, { index: 2, board_ply: 0, label: "Recorded command" }], event_index: 1, historical: true, paused: true, capabilities: { ...standard.capabilities, board_action: false } };
  const { session, props, rerender } = app(snapshot);
  expect(screen.getByRole("region", { name: "Historical shogi position" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: /Recorded command/ }));
  expect(session.command).toHaveBeenLastCalledWith(snapshot, { kind: "inspect", event_index: 2 });
  const returnLive = screen.getByRole("button", { name: "Return to live" });
  expect(returnLive.closest(".shell-history-notice-heading")).not.toBeNull();
  fireEvent.click(returnLive);
  expect(session.command).toHaveBeenLastCalledWith(snapshot, { kind: "return_live" });
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  const live = { ...snapshot, historical: false, event_index: 2 };
  rerender(<ShogiApp {...props} snapshot={live} />);
  fireEvent.click(screen.getByRole("button", { name: "Resume play" })); expect(session.command).toHaveBeenLastCalledWith(live, { kind: "resume" });
});
it("formats only supplied observations with position, perspective and incomplete-result meaning", () => {
  const view: SearchView = { selected: true, observation_index: 0, purpose: "move_analysis", event_index: 0, board_ply: 0, perspective: "Sente", engine: "mShogi", score: "+42 heuristic units", completed_depth: 2, nodes: 321, variation: ["7g7f", "3c3d"], status: "Node limit reached", historical: false };
  const snapshot = { ...standard, search_observations: [view] };
  const { props, rerender } = app(snapshot);
  const current = document.querySelector<HTMLElement>(".shogi-analysis > .shogi-search-observation")!;
  expect(within(current).getByText("+42 heuristic units")).toBeTruthy();
  expect(within(current).getByText("Sente perspective · Node limit reached")).toBeTruthy();
  expect(within(current).getByText("Move analysis · event 0")).toBeTruthy();
  expect(within(current).getByText("Read-only analysis. No move or declaration was played.")).toBeTruthy();
  // A newly selected position must not show the old observation as its result.
  rerender(<ShogiApp {...props} snapshot={{ ...snapshot, historical: true, event_index: 1, search_observations: [{ ...view, selected: false, historical: true }] }} />);
  expect(document.querySelector(".shogi-analysis > .shogi-search-observation")).toBeNull();
  expect(screen.getByText("Saved observation · event 0")).toBeTruthy();
});
it("requires native declaration capability plus a formal confirmation and preserves captured authority", () => {
  const { props, session, rerender } = app();
  expect(screen.queryByRole("button", { name: "Declare entering king…" })).toBeNull();
  const eligible = { ...standard, capabilities: { ...standard.capabilities, declaration: true } };
  rerender(<ShogiApp {...props} snapshot={eligible} />);
  fireEvent.click(screen.getByRole("button", { name: "Declare entering king…" }));
  expect(session.command).not.toHaveBeenCalled();
  expect(screen.getByRole("dialog", { name: "Declare entering king?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Confirm declaration" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(eligible, { kind: "declare" });
});
it("cancels resignation and refuses a confirmation after native selection changes", () => {
  const { props, session, rerender } = app();
  fireEvent.click(screen.getByRole("button", { name: "Resign game…" }));
  fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Resign game…" }));
  const next = { ...standard, token: { ...standard.token, selection: standard.token.selection + 1 } };
  rerender(<ShogiApp {...props} snapshot={next} />);
  expect((screen.getByRole("button", { name: "Resign game" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Resign game" })); expect(session.command).not.toHaveBeenCalled();
});
it("distinguishes application capacity from engine failure and keeps the complete save path", () => {
  const snapshot = { ...standard, phase: "incomplete" as const, capacity: { ...standard.capacity, status: "paused" as const, message: "256 observation limit" } };
  const { session } = app(snapshot);
  expect(screen.getByRole("heading", { name: "Application limit reached" })).toBeTruthy();
  expect(screen.getByText(/Saving and reopening does not reset this limit/)).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
  expect((screen.getByRole("button", { name: "Start move analysis" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Save" })); expect(session.file).toHaveBeenCalledExactlyOnceWith(snapshot, "save");
});
it("presents failures as recoverable execution problems, never as a chess result", () => {
  const snapshot = { ...standard, phase: "failed" as const, turn: standard.sides[1]! };
  const { session } = app(snapshot, "Engine /private/machine/path failed");
  expect(screen.getByText("The current game is intact. No game result is implied.")).toBeTruthy();
  const alert = screen.getByRole("alert"); expect(alert.textContent).not.toContain("/private/machine/path");
  fireEvent.click(screen.getByRole("button", { name: "Retry engine turn" })); expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "retry" });
});

it("uses native Take back availability and one complete shared command, never raw Undo", () => {
  const snapshot = { ...standard, capabilities: { ...standard.capabilities, take_back: true } };
  const { session, props, rerender } = app(snapshot);
  const control = screen.getByRole("button", { name: "Take back" });
  expect(control.closest("[data-shell-section]")?.getAttribute("data-shell-section")).toBe("actions");
  fireEvent.click(control);
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "take_back" });
  const historical = { ...snapshot, historical: true };
  rerender(<ShogiApp {...props} snapshot={historical} />);
  expect((control as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(control); expect(session.command).toHaveBeenCalledTimes(1);
  rerender(<ShogiApp {...props} snapshot={{ ...snapshot, capabilities: { ...snapshot.capabilities, take_back: false } }} />);
  expect((control as HTMLButtonElement).disabled).toBe(true);
});
