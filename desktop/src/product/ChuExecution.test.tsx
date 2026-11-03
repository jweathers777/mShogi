// @vitest-environment jsdom
// Successful telemetry comes only from Rust snapshots generated through the real
// Chu subprocess. Explicit status/error constructions below test presentation
// intents; they are not additional native execution or rule-conformance evidence.
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { NativeAnalysis, NativeRecovery } from "./NativeSessionPanels";
import { GameActions } from "./GameCapabilities";
import type { ChuSnapshot } from "./native";
import completeJson from "../__fixtures__/shared-chu-analysis-complete.json";
import incompleteJson from "../__fixtures__/shared-chu-analysis-incomplete.json";
const complete = completeJson as unknown as ChuSnapshot;
const incomplete = incompleteJson as unknown as ChuSnapshot;
afterEach(cleanup);
function analysis(snapshot = complete) {
  const command = vi.fn();
  const props = { snapshot, defaults: snapshot.play_limits, disabled: false, command, maxDepth: 2, maxNodes: 2000, minNodes: 1 };
  const view = render(<NativeAnalysis {...props} />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  return { ...view, props, command };
}
function currentObservation(container: HTMLElement) {
  const node = container.querySelector<HTMLElement>(".shogi-analysis>.shogi-search-observation");
  if (!node) throw new Error("Missing selected analysis observation");
  return within(node);
}
it("renders the actual Chu root-side completed observation, exact target and native continuation", () => {
  const selected = complete.search_observations.find(view => view.selected)!;
  const { container, command } = analysis();const observation = currentObservation(container);
  expect(screen.getByText(`Selected event ${complete.event_index} · ${complete.turn.label} perspective`)).toBeTruthy();
  expect(observation.getByText(`Move analysis · event ${selected.event_index}`)).toBeTruthy();
  expect(observation.getByText(selected.score!)).toBeTruthy();
  expect(observation.getByText(`${selected.perspective} perspective · ${selected.status}`)).toBeTruthy();
  expect(observation.getByText(String(selected.completed_depth))).toBeTruthy();
  expect(observation.getByText(selected.nodes!.toLocaleString())).toBeTruthy();
  expect(observation.getByText(selected.variation.join(" · "))).toBeTruthy();
  expect(observation.getByText("No move was played.")).toBeTruthy();
  expect(container.querySelector("[data-search-purpose=move_analysis]")).toBeTruthy();
  expect(command).not.toHaveBeenCalled();
});
it("shows actual incomplete work unscored while keeping earlier successful observations historical", () => {
  const selected = incomplete.search_observations.find(view => view.selected)!;
  expect(selected.score).toBeNull();expect(selected.completed_depth).toBe(0);expect(selected.variation).toEqual([]);
  const { container, command } = analysis(incomplete);const observation = currentObservation(container);
  expect(observation.getByText("No completed evaluation")).toBeTruthy();
  expect(observation.getByText(`${selected.perspective} perspective · ${selected.status}`)).toBeTruthy();
  expect(observation.queryByText(/Pawns|wins|draw|mate/i)).toBeNull();
  expect(observation.queryByText("Continuation")).toBeNull();
  expect(observation.getByText("No move was played.")).toBeTruthy();
  expect(container.querySelector(".recorded-searches summary")?.textContent).toBe(`Recorded searches (${incomplete.search_observations.length})`);
  expect(command).not.toHaveBeenCalled();
});
it("offers conservative default limits, admits bounded analysis only and does not issue board actions", () => {
  const { command } = analysis();
  const depth = screen.getByRole("spinbutton", { name: "Depth" }) as HTMLInputElement;
  const nodes = screen.getByRole("spinbutton", { name: "Node limit" }) as HTMLInputElement;
  expect(depth.value).toBe("1");expect(nodes.value).toBe("512");
  expect([depth.min, depth.max, depth.step]).toEqual(["1", "2", "1"]);
  expect([nodes.min, nodes.max, nodes.step]).toEqual(["1", "2000", "1"]);
  fireEvent.change(depth, { target: { value: "2" } });fireEvent.change(nodes, { target: { value: "2000" } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "analyze", operation: { kind: "moves", depth: 2, nodes: 2000 } });
});
it.each([[3, 512], [1, 2001], [1, 0], [1, 1.5]])("rejects unsupported depth/node limits %s/%s without sending execution", (depth, nodes) => {
  const { command } = analysis();
  fireEvent.change(screen.getByRole("spinbutton", { name: "Depth" }), { target: { value: String(depth) } });
  fireEvent.change(screen.getByRole("spinbutton", { name: "Node limit" }), { target: { value: String(nodes) } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(screen.getByRole("alert").textContent).toContain("depth from 1 to 2");
  expect(screen.getByRole("alert").textContent).toContain("1 to 2,000");expect(command).not.toHaveBeenCalled();
});
it("replaces and cancels analysis with distinct read-only operations and no synthetic progress", () => {
  // Status-only pending-work construction, with no invented successful telemetry.
  const thinking: ChuSnapshot = { ...complete, phase: "thinking", execution_purpose: "move_analysis" };
  const { command, container } = analysis(thinking);
  fireEvent.click(screen.getByRole("button", { name: "Replace analysis" }));
  expect(command).toHaveBeenNthCalledWith(1, { kind: "analyze", operation: { kind: "moves", depth: 1, nodes: 512 } });
  fireEvent.click(screen.getByRole("button", { name: "Cancel analysis" }));
  expect(command).toHaveBeenNthCalledWith(2, { kind: "cancel" });expect(command).toHaveBeenCalledTimes(2);
  expect(screen.queryByRole("progressbar")).toBeNull();expect(container.querySelector(".shogi-pv")?.textContent).toContain(complete.search_observations[0]!.variation[0]);
});
it("keeps Resume and Retry engine-turn intents distinct from analysis cancellation", () => {
  const command = vi.fn();const { rerender } = render(<NativeRecovery snapshot={complete} error="" disabled={false} command={command} />);
  fireEvent.click(screen.getByRole("button", { name: "Resume play" }));expect(command).toHaveBeenNthCalledWith(1, { kind: "resume" });
  // Deliberate error-state construction for recovery wording and routing only.
  const engineTurn: ChuSnapshot = { ...complete, paused: false, human: complete.sides.find(side => side.key !== complete.turn.key)!, phase: "failed", execution_purpose: "engine_turn",
    observations: [{ ...complete.observations[0]!, observation: { ...complete.observations[0]!.observation, result: { kind: "failure", failure: { kind: "search", message: "Explicit frontend search-error probe" } } } }] };
  rerender(<NativeRecovery snapshot={engineTurn} error="" disabled={false} command={command} />);
  expect(screen.getByRole("heading", { name: "Engine search could not complete" })).toBeTruthy();
  expect(screen.queryByText("Engine connection failed")).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Retry engine turn" }));expect(command).toHaveBeenNthCalledWith(2, { kind: "retry" });
  rerender(<NativeRecovery snapshot={{ ...engineTurn, execution_purpose: "move_analysis" }} error="" disabled={false} command={command} />);
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
});
it("uses game-qualified native draw commands and retains the shared direct resign intent", () => {
  const command = vi.fn().mockResolvedValue(null), confirm = vi.fn();
  const { rerender } = render(<GameActions snapshot={complete} disabled={false} command={command} confirm={confirm} />);
  fireEvent.click(screen.getByRole("button", { name: "Offer draw…" }));
  expect(confirm).toHaveBeenLastCalledWith(expect.objectContaining({ source: complete, operation: { kind: "chu", actor: "sente", command: { kind: "offer_draw" } } }));
  expect(command).not.toHaveBeenCalled();
  // Construct only the authoritative presentation field for a pending Gote offer;
  // native offer legality and actual agreement are verified by controller tests.
  const offered: ChuSnapshot = { ...complete, chu: { ...complete.chu, live: { ...complete.chu.live, draw_offer: "gote" } } };
  rerender(<GameActions snapshot={offered} disabled={false} command={command} confirm={confirm} />);
  fireEvent.click(screen.getByRole("button", { name: "Accept draw…" }));
  expect(confirm).toHaveBeenLastCalledWith(expect.objectContaining({ source: offered, operation: { kind: "chu", actor: "sente", command: { kind: "accept_draw" } } }));
  fireEvent.click(screen.getByRole("button", { name: "Decline draw" }));
  expect(command).toHaveBeenNthCalledWith(1, { kind: "chu", actor: "sente", command: { kind: "decline_draw" } });
  fireEvent.click(screen.getByRole("button", { name: "Resign" }));
  // Generic shared intent; the native runtime supplies the current human actor.
  expect(command).toHaveBeenNthCalledWith(2, { kind: "resign" });
});
