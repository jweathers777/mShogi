// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { ApplicationView as XiangqiApp } from "./ApplicationView";
import type { XiangqiSnapshot as GameSnapshot, SearchView } from "./native";
import type { GameSession } from "./useGameSession";
import standardJson from "../__fixtures__/shared-xiangqi-standard.json";
import warningJson from "../__fixtures__/shared-xiangqi-warning.json";
import fourthJson from "../__fixtures__/shared-xiangqi-fourth.json";
import countJson from "../__fixtures__/shared-xiangqi-count.json";
import offerJson from "../__fixtures__/shared-xiangqi-offer.json";
import agreedJson from "../__fixtures__/shared-xiangqi-agreed.json";
import resignedJson from "../__fixtures__/shared-xiangqi-resigned.json";
import fastJson from "../__fixtures__/shared-xiangqi-fast-proof.json";
import playedJson from "../__fixtures__/shared-xiangqi-played.json";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const standard = standardJson as unknown as GameSnapshot;
afterEach(cleanup);
function app(snapshot = standard, error = "") {
  const session: GameSession = { snapshot, busy: false, error, closeRequested: false,
    command: vi.fn().mockResolvedValue(snapshot), file: vi.fn().mockResolvedValue(snapshot),
    newGame: vi.fn().mockResolvedValue(snapshot), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
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
  expect(screen.getByText("XIANGQI")).toBeTruthy();
  expect(screen.getByRole("heading", { name: "Untitled game" })).toBeTruthy();
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved");
  expect(container.querySelector("[data-play-limits]")?.textContent).toBe("Play limit: depth 1 · 128 nodes");
  expect([...container.querySelectorAll("[data-shell-section]")].map((node) => node.getAttribute("data-shell-section"))).toEqual(["analysis", "actions", "history"]);
  expect(container.querySelector(".shell-sidebar > details")).toBeNull();
  expect(screen.getByRole("button", { name: "About mShogi" }).closest(".shell-header")).toBeTruthy();
  expect([...container.querySelector(".shell-game-area")!.children].map((node) => node.getAttribute("data-game-slot"))).toEqual(["upper-player", "board", "lower-player", "footer"]);
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Your turnLive position · 0 plies");
  for (const name of ["New game", "Save", "Save as…", "Open…"]) expect(screen.getByRole("button", { name }).closest(".shell-header")).toBeTruthy();
  expect(screen.queryByText(/Entering.king|pieces in hand|Promote/)).toBeNull();
  expect(screen.queryByRole("spinbutton")).toBeNull();
  expect((screen.getByRole("button", { name: "Analyze position" }) as HTMLButtonElement).disabled).toBe(false);
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
  expect(screen.queryByRole("button", { name: "Cancel analysis" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Cancel search" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "cancel" });
});
it("keeps resource-limit refusal distinct and retains a complete Save path", () => {
  const snapshot = { ...standard, capacity: { ...standard.capacity, status: "paused" as const, message: "256 observations" } };
  const { session } = app(snapshot);
  expect(screen.getByRole("heading", { name: "Application limit reached" })).toBeTruthy();
  expect(screen.getByText(/Saving and reopening does not reset this limit/)).toBeTruthy();
  fireEvent.click(point("e3")); fireEvent.click(point("e4")); expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Save" })); expect(session.file).toHaveBeenCalledExactlyOnceWith(snapshot, "save");
});
it("shows native WXF capture notation without exposing the canonical diagnostic payload", () => {
  const snapshot = playedJson as unknown as GameSnapshot;
  app(snapshot);
  const history = screen.getByRole("region", { name: "Moves" });
  expect(history.textContent).toContain("C8+7");
  expect(history.textContent).toContain("R1=2");
  expect(history.textContent).not.toMatch(/Some\(|BoardAction|captured_id|moving_id/);
  expect(snapshot.events.map((e) => [e.index, e.board_ply])).toEqual([[1, 1], [2, 2]]);
  expect(snapshot.context).toEqual(playedJson.context);
});
it("submits bounded read-only work, replacement and cancellation with exact source authority", () => {
  const { session, props, rerender } = app();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const depth = screen.getByRole("spinbutton", { name: "Depth" }) as HTMLInputElement;
  const nodes = screen.getByRole("spinbutton", { name: "Node limit" }) as HTMLInputElement;
  expect(depth.max).toBe("3"); expect(nodes.max).toBe("512"); expect(nodes.min).toBe("1");
  fireEvent.change(depth, { target: { value: "3" } }); fireEvent.change(nodes, { target: { value: "512" } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "analyze", operation: { kind: "moves", depth: 3, nodes: 512 } });
  const analyzing = { ...standard, phase: "thinking" as const, execution_purpose: "move_analysis", paused: true };
  rerender(<XiangqiApp {...props} snapshot={analyzing} />);
  expect(screen.queryByRole("button", { name: "Cancel search" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Replace analysis" }));
  expect(session.command).toHaveBeenLastCalledWith(analyzing, { kind: "analyze", operation: { kind: "moves", depth: 3, nodes: 512 } });
  fireEvent.click(screen.getByRole("button", { name: "Cancel analysis" })); expect(session.command).toHaveBeenLastCalledWith(analyzing, { kind: "cancel" });
  fireEvent.change(nodes, { target: { value: "513" } });
  expect(nodes.checkValidity()).toBe(false);
  fireEvent.submit(nodes.closest("form")!); expect(session.command).toHaveBeenCalledTimes(3);
});
it("uses selected native observations only, preserving perspective, provenance and unscored incompleteness", () => {
  // SearchView rendering test: these values are supplied by native, not computed here.
  const view: SearchView = { selected: true, observation_index: 0, purpose: "move_analysis", event_index: 0, board_ply: 0, perspective: "Red", engine: "mShogi", score: "+30 heuristic units", completed_depth: 1, nodes: 45, variation: ["a3a4"], status: "Complete", historical: false };
  const { props, rerender } = app({ ...standard, search_observations: [view] });
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const current = document.querySelector<HTMLElement>(".shogi-analysis > .shogi-search-observation")!;
  expect(within(current).getByText("Red perspective · Complete")).toBeTruthy();
  expect(within(current).getByText("No move or claim was played.")).toBeTruthy();
  rerender(<XiangqiApp {...props} snapshot={{ ...standard, historical: true, event_index: 1, search_observations: [{ ...view, selected: false, historical: true, score: null, status: "Node limit reached", completed_depth: null }] }} />);
  expect(document.querySelector(".shogi-analysis > .shogi-search-observation")).toBeNull();
  expect(screen.getByText("Saved observation · event 0")).toBeTruthy();
  expect(screen.getByText("No completed evaluation")).toBeTruthy();
  expect(screen.queryByText(/forced win|draw by evaluation/i)).toBeNull();
});
it("navigates exact native event indices and keeps partial command prefixes inspectable but not analyzable", async () => {
  const native = warningJson as unknown as GameSnapshot;
  const { session, props, rerender } = app(native);
  const last = native.events.at(-1)!;
  fireEvent.click(screen.getByRole("button", { name: new RegExp(last.label.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')) }));
  expect(session.command).toHaveBeenLastCalledWith(native, { kind: "inspect", event_index: last.index });
  const history = { ...native, historical: true, paused: true, event_index: last.index - 1, capabilities: { ...native.capabilities, board_action: false, move_analysis: false }, xiangqi: { ...native.xiangqi, canonical_context_scope: "live_with_immutable_selection" as const, selected_canonical: null } };
  rerender(<XiangqiApp {...props} snapshot={history} />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(screen.getByText(/Analysis is unavailable between a move and its following event/)).toBeTruthy();
  expect((screen.getByRole("button", { name: "Start move analysis" }) as HTMLButtonElement).disabled).toBe(true);
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Return to live" }));
  expect(session.command).toHaveBeenLastCalledWith(history, { kind: "return_live" });
  const live = { ...native, paused: true };
  rerender(<XiangqiApp {...props} snapshot={live} />);
  fireEvent.click(screen.getByRole("button", { name: "Resume play" }));
  expect(session.command).toHaveBeenLastCalledWith(live, { kind: "resume" });
});
it("shows native warning and eligible claim as opportunities, not concluded outcomes", () => {
  const native = warningJson as unknown as GameSnapshot;
  const { props, rerender, session } = app(native);
  expect(screen.getByRole("heading", { name: "Repetition warning" })).toBeTruthy();
  expect(screen.getByText(/must vary perpetual check/)).toBeTruthy();
  const claim = countJson as unknown as GameSnapshot;
  rerender(<XiangqiApp {...props} snapshot={claim} />);
  fireEvent.click(screen.getByRole("button", { name: "Claim natural-count draw…" }));
  expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Claim draw" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(claim, { kind: "xiangqi", command: { kind: "claim_natural_count" } });
}, 15000);
it("adapts native terminal capabilities independently of board turn and protects captured confirmation", () => {
  const offer = offerJson as unknown as GameSnapshot;
  const { props, rerender, session } = app(offer);
  expect(screen.getByRole("heading", { name: "Draw offered by Black" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Accept draw…" }));
  const changed = { ...offer, token: { ...offer.token, selection: offer.token.selection + 1 } };
  rerender(<XiangqiApp {...props} snapshot={changed} />);
  expect((screen.getByRole("button", { name: "Accept draw" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  fireEvent.click(screen.getByRole("button", { name: "Decline draw…" }));
  fireEvent.click(screen.getByRole("button", { name: "Decline draw" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(changed, { kind: "xiangqi", command: { kind: "decline_draw" } });
});
for (const [label, json] of [["fourth occurrence", fourthJson], ["agreement", agreedJson], ["resignation", resignedJson], ["fast no-win proof", fastJson]] as const) {
  it(`presents the authoritative ${label} conclusion without heuristic adjudication`, () => {
    const native = json as unknown as GameSnapshot; app(native);
    expect(native.status.concluded).toBe(true);
    expect(document.querySelector("[data-game-status]")?.textContent).toBe(native.status.label);
    expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Resign game…" })).toBeNull();
    expect(screen.queryByRole("heading", { name: "Repetition warning" })).toBeNull();
  });
}
it("separates unresolved assessment and process failure and retains retry and Save", () => {
  const failed = { ...standard, phase: "failed" as const, turn: standard.sides[1]!, paused: true };
  const { props, session, rerender } = app(failed, "stale request refused");
  expect(screen.getByText(/The action could not complete/)).toBeTruthy();
  expect(screen.getByText("The current game is intact. No game result is implied.")).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Retry engine turn" })); expect(session.command).toHaveBeenCalledExactlyOnceWith(failed, { kind: "retry" });
  // Defensive presentation branch only: no admitted native fixture claims to reach this state.
  const pending = { ...standard, paused: true, xiangqi: { ...standard.xiangqi, pending: { reason: "classification required", occurrences: [0, 4, 8, 12] } } };
  rerender(<XiangqiApp {...props} snapshot={pending} />);
  expect(screen.getByText(/This game is unscored/)).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Save" })); expect(session.file).toHaveBeenLastCalledWith(pending, "save");
});
it("reports asynchronous native file failures without clearing document state", () => {
  app({ ...standard, io: { id: 1, phase: "failed", message: "Corrupt archive" }, document: { ...standard.document, dirty: true } });
  expect(screen.getByRole("alert").textContent).toContain("unsaved changes have been kept");
  expect(screen.getByText("Unsaved")).toBeTruthy();
});

it("does not guess capacity from untyped diagnostics and retains complete native controls", () => {
  const { session } = app(standard, "Application capacity: retained observations are at their limit; accepted history is saveable.");
  expect(screen.queryByRole("heading", { name: "Application limit reached" })).toBeNull();
  expect(screen.getByRole("alert").textContent).toContain("The action could not complete");
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect((screen.getByRole("button", { name: "Start move analysis" }) as HTMLButtonElement).disabled).toBe(false);
  fireEvent.click(screen.getByRole("button", { name: "Save" }));
  expect(session.file).toHaveBeenCalledExactlyOnceWith(standard, "save");
});

it("retains native structural refusal across successful Save and separates file failure", () => {
  const paused = { ...standard, capacity: { ...standard.capacity, status: "paused" as const, message: "Combined diagnostic; native category unavailable" } };
  const { props, rerender } = app(paused);
  const saved = { ...paused, document: { ...paused.document, path: "/games/retained.mshogi", dirty: false } };
  rerender(<XiangqiApp {...props} snapshot={saved} session={{ ...props.session, error: "" }} />);
  expect(screen.getByRole("heading", { name: "Application limit reached" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect((screen.getByRole("button", { name: "Start move analysis" }) as HTMLButtonElement).disabled).toBe(true);
  const failedSave = { ...saved, document: { ...saved.document, dirty: true }, io: { id: 2, phase: "failed", message: "File write failed" } };
  rerender(<XiangqiApp {...props} snapshot={failedSave} session={{ ...props.session, error: "Save failed" }} />);
  expect(screen.getByText(/file operation could not complete/)).toBeTruthy();
  expect(screen.getByRole("heading", { name: "Application limit reached" })).toBeTruthy();
  rerender(<XiangqiApp {...props} snapshot={{ ...standard, token: { ...standard.token, document: standard.token.document + 1 } }} session={{ ...props.session, error: "" }} />);
  expect(screen.queryByRole("heading", { name: "Application limit reached" })).toBeNull();
});

it("uses native Take back availability and one complete shared command, never raw Undo", () => {
  const snapshot = { ...standard, capabilities: { ...standard.capabilities, take_back: true } };
  const { session, props, rerender } = app(snapshot);
  const control = screen.getByRole("button", { name: "Take back" });
  expect(control.closest("[data-shell-section]")?.getAttribute("data-shell-section")).toBe("actions");
  fireEvent.click(control);
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "take_back" });
  const historical = { ...snapshot, historical: true };
  rerender(<XiangqiApp {...props} snapshot={historical} />);
  expect((control as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(control); expect(session.command).toHaveBeenCalledTimes(1);
  rerender(<XiangqiApp {...props} snapshot={{ ...snapshot, capabilities: { ...snapshot.capabilities, take_back: false } }} />);
  expect((control as HTMLButtonElement).disabled).toBe(true);
});
