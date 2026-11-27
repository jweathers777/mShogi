// @vitest-environment jsdom
import type { ChessSnapshot, GameSnapshot, SearchView } from "../sessionTypes";
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { GameAnalysis, GameHistory, GameRecovery, Observation } from "./GameSessionPanels";

import standardJson from "../__fixtures__/shared-chess-standard.json";
import movesJson from "../__fixtures__/shared-chess-analysis-moves.json";
import mateJson from "../__fixtures__/shared-chess-analysis-mate.json";
import matedJson from "../__fixtures__/shared-chess-analysis-mated.json";
import cancelledJson from "../__fixtures__/shared-chess-analysis-cancelled.json";
import claimedJson from "../__fixtures__/shared-chess-claimed.json";
import unresolvedJson from "../__fixtures__/shared-chess-unresolved.json";
import warningJson from "../__fixtures__/shared-xiangqi-warning.json";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
afterEach(cleanup);
const chess = (value: unknown) => structuredClone(value) as ChessSnapshot;
function panel(snapshot = chess(standardJson)) {
  const command = vi.fn(); const props = { snapshot, defaults: snapshot.document.settings.analysis_defaults, command, disabled: false, minNodes: 0, maxDepth: 8, maxNodes: 5000000 };
  const view = render(<GameAnalysis {...props} />); fireEvent.click(screen.getByRole("button", { name: "Analyze position" })); return { ...view, command, props };
}
it("viewing Chess analysis never starts work; ordinary work uses the current read-only command", () => {
  const { command } = panel(); expect(command).not.toHaveBeenCalled(); fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "analyze", operation: { depth: 4, nodes: 100000 } });
});
it("rejects unsupported limits without clamping or issuing work", () => {
  const { command } = panel(); fireEvent.change(screen.getByRole("spinbutton", { name: "Depth" }), { target: { value: "9" } }); fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).not.toHaveBeenCalled(); expect(screen.getByRole("alert").textContent).toContain("depth from 1 to 8");
});
it("displays actual move telemetry and perspective without playing suggestions", () => {
  const { command } = panel(chess(movesJson)); const result = screen.getByLabelText("Analysis result"); expect(result.textContent).toContain("Suggested move:"); expect(result.textContent).toContain("not played"); expect(result.textContent).toContain("pawns from White perspective"); expect(command).not.toHaveBeenCalled();
});
it.each([[mateJson, "Mate for White"], [matedJson, "Mate for White · already checkmate"]])("keeps mating-distance score meaning and perspective", (fixture, expected) => {
  panel(chess(fixture)); expect(screen.getByLabelText("Analysis result").textContent).toContain(expected);
});
it("cancels and replaces active analysis work through common commands without an engine-turn shortcut", () => {
  const snapshot = chess(movesJson); snapshot.chess.analysis.status = "thinking"; snapshot.chess.analysis.result = null;
  const { command } = panel(snapshot); fireEvent.click(screen.getByRole("button", { name: "Cancel analysis" })); expect(command).toHaveBeenLastCalledWith({ kind: "cancel" }); fireEvent.click(screen.getByRole("button", { name: "Replace analysis" })); expect(command).toHaveBeenLastCalledWith({ kind: "analyze", operation: snapshot.document.settings.analysis_defaults });
});
it.each(["selection", "version", "purpose", "event", "board", "side", "limits"] as const)("withholds analysis when %s does not match the accepted selected context", (field) => {
  const snapshot = chess(movesJson); const purpose = snapshot.chess.analysis.context!.purpose;
  if (field === "selection") snapshot.token.selection++;
  if (field === "version") snapshot.token.version++;
  if (field === "purpose") snapshot.chess.analysis.context!.purpose = { kind: "engine_turn" };
  if (purpose.kind === "move_analysis") {
    if (field === "event") purpose.target.event_index++;
    if (field === "board") purpose.target.board_ply++;
    if (field === "side") purpose.target.turn = { game_kind: "chess", side: "black" };
  }
  if (field === "limits") snapshot.chess.analysis.context!.limits.nodes++;
  panel(snapshot); expect(within(screen.getByLabelText("Analysis result")).queryByText(/Suggested move:/)).toBeNull();
});
it("does not automatically resume cancelled analysis or offer engine retry", () => {
  const snapshot = chess(cancelledJson); panel(snapshot); expect(screen.getByText(/Analysis cancelled. No conclusion was established/)).toBeTruthy();
  render(<GameRecovery snapshot={snapshot} error="" disabled={false} command={vi.fn()} />); expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
});
it("uses game event identities including a non-move claim", () => {
  const snapshot = chess(claimedJson), command = vi.fn(); render(<GameHistory snapshot={snapshot} disabled={false} command={command} />);
  fireEvent.click(screen.getAllByRole("button").at(-1)!); expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "inspect", event_index: snapshot.events.at(-1)!.index }); expect(snapshot.board_ply).toBe(0);
});
it("shows unresolved resignation without inventing a win, draw or score", () => {
  render(<GameRecovery snapshot={chess(unresolvedJson)} error="" disabled={false} command={vi.fn()} />); expect(screen.getByLabelText("Game outcome").textContent).toContain("without an established win or draw"); expect(screen.getByLabelText("Game outcome").textContent).toContain("no score is assigned");
});
it.each(["failed", "cancelled", "incomplete"] as const)("keeps %s engine work distinct from a game outcome and retries explicitly", (phase) => {
  const snapshot = chess(standardJson); snapshot.phase = phase; snapshot.turn = snapshot.sides.find((side) => side.key !== snapshot.human.key)!; snapshot.capabilities.engine_turn = true; const command = vi.fn(); render(<GameRecovery snapshot={snapshot} error="" disabled={false} command={command} />);
  expect(screen.queryByLabelText("Game outcome")).toBeNull(); fireEvent.click(screen.getByRole("button", { name: "Retry engine turn" })); expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "retry" });
});
it("retains Xiangqi warnings and unresolved pending semantics without a fabricated result", () => {
  const snapshot = structuredClone(warningJson) as unknown as GameSnapshot; if (snapshot.game !== "xiangqi") throw new Error("rules fixture"); snapshot.xiangqi.pending = { reason: "unknown", occurrences: [] }; snapshot.paused = true;
  render(<GameRecovery snapshot={snapshot} error="" disabled={false} command={vi.fn()} />); expect(screen.getByText("Repetition warning")).toBeTruthy(); expect(screen.getByText(/Red must vary perpetual check/)).toBeTruthy(); expect(screen.getByText(/unscored; no win, loss or draw/)).toBeTruthy(); expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
});
it("uses the same game history selection operation for Shogi", () => {
  const snapshot = structuredClone(shogiJson) as unknown as GameSnapshot; const command = vi.fn(); render(<GameHistory snapshot={snapshot} disabled={false} command={command} />); fireEvent.click(screen.getByRole("button", { name: /Starting position/ })); expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "inspect", event_index: 0 });
});

it("announces suggestions once with named target and no synthetic progress", async () => {
  const snapshot = chess(movesJson); const { rerender, props } = panel(snapshot);
  const status = screen.getByRole("status", { name: "Analysis status" });
  expect(status.getAttribute("aria-atomic")).toBe("true"); expect(status.textContent).toContain(`Move analysis completed for ${snapshot.chess.analysis.target!.label}`); expect(status.textContent).toContain("not played");
  const changed = vi.fn(), observer = new MutationObserver(changed); observer.observe(status, { childList: true, subtree: true, characterData: true });
  await act(async () => rerender(<GameAnalysis {...props} snapshot={structuredClone(snapshot)} />)); expect(changed).not.toHaveBeenCalled(); observer.disconnect(); expect(screen.queryByRole("progressbar")).toBeNull();
});
it("announces replacement by exact request identity and retires stale target language", () => {
  const snapshot = chess(movesJson); snapshot.chess.analysis.status = "thinking"; snapshot.chess.analysis.result = null;
  const { rerender, props } = panel(snapshot); const status = screen.getByRole("status", { name: "Analysis status" }); expect(status.textContent).toContain("Move analysis started for");
  const replacement = structuredClone(snapshot); replacement.chess.analysis.context!.request_id++;
  rerender(<GameAnalysis {...props} snapshot={replacement} />); expect(status.textContent).toContain("replacement started for");
  const stale = chess(movesJson); stale.token.selection++; rerender(<GameAnalysis {...props} snapshot={stale} />); expect(status.textContent).toContain("recorded target"); expect(status.textContent).toContain("does not describe the current position");
});
it("does not display rejected archived scores as accepted analysis", () => {
  const snapshot = chess(movesJson); snapshot.observations.forEach((entry) => { entry.application_error = "Rejected engine result"; }); snapshot.chess.analysis = { ...snapshot.chess.analysis, target: null, result: null, status: "idle" };
  panel(snapshot); expect(screen.getByText("Rejected response. No valid score or proof is presented.")).toBeTruthy(); expect(screen.queryByText(/Suggested move:/)).toBeNull();
});
it("restores saved analysis defaults on a fresh document while ordinary polls preserve typed work limits", () => {
  const snapshot = chess(standardJson); snapshot.document.settings.analysis_defaults = { depth: 3, nodes: 4321 };
  const { props, rerender } = panel(snapshot); const nodes = screen.getByRole("spinbutton", { name: "Node limit" }) as HTMLInputElement; expect(nodes.value).toBe("4321"); fireEvent.change(nodes, { target: { value: "9999" } });
  rerender(<GameAnalysis {...props} snapshot={{ ...snapshot, token: { ...snapshot.token, revision: snapshot.token.revision + 1 } }} />); expect(nodes.value).toBe("9999");
  rerender(<GameAnalysis {...props} snapshot={{ ...snapshot, token: { ...snapshot.token, document: snapshot.token.document + 1 } }} />); expect(nodes.value).toBe("4321");
});

it("shows the Chess root once without a Start bookkeeping event", () => {
  const snapshot = chess(standardJson); const command = vi.fn(); const { container } = render(<GameHistory snapshot={snapshot} disabled={false} command={command} />);
  expect(container.querySelectorAll("[data-event='0']")).toHaveLength(1); expect(screen.getAllByRole("button")).toHaveLength(snapshot.events.length + 1);
});

it("finishes concluded Chess analysis without reopening the game outcome", () => {
  const snapshot = chess(matedJson); const command = vi.fn();
  render(<GameRecovery snapshot={snapshot} error="" disabled={false} command={command} />);
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  expect(screen.getByText("The game remains concluded. Finishing analysis does not reopen it.")).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Finish analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "resume" });
  expect(screen.getByLabelText("Game outcome").textContent).toContain("Checkmate");
});
it("offers finishing concluded Chess analysis only while an actual analysis exists", () => {
  const snapshot = chess(unresolvedJson); snapshot.paused = true; const command = vi.fn();
  const { rerender } = render(<GameRecovery snapshot={snapshot} error="" disabled={false} command={command} />);
  expect(screen.queryByRole("button", { name: "Finish analysis" })).toBeNull();
  snapshot.chess.analysis.status = "thinking";
  rerender(<GameRecovery snapshot={snapshot} error="" disabled={false} command={command} />);
  fireEvent.click(screen.getByRole("button", { name: "Finish analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "resume" });
  expect(screen.getByLabelText("Game outcome").textContent).toContain("no score is assigned");
});

it("shows reported selective depth including zero and omits it when unreported", () => {
  const view: SearchView = { selected: true, observation_index: 0, purpose: "move_analysis", event_index: 0, board_ply: 0, perspective: "White", engine: "mShogi", score: "+0.44 pawns", completed_depth: 1, selective_depth: 5, nodes: 21, variation: ["b1c3"], status: "Complete", historical: false };
  const { rerender } = render(<Observation view={view} game="chess" />);
  expect(screen.getByText("Selective depth").nextElementSibling?.textContent).toBe("5");
  rerender(<Observation view={{ ...view, selective_depth: 0 }} game="chess" />);
  expect(screen.getByText("Selective depth").nextElementSibling?.textContent).toBe("0");
  const unreported = { ...view }; delete unreported.selective_depth;
  rerender(<Observation view={unreported} game="chess" />);
  expect(screen.queryByText("Selective depth")).toBeNull();
});
