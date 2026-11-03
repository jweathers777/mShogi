// @vitest-environment jsdom
// Actual Rust/controller DTOs; this tests consumers, not independently new rules.
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { NativeBoard } from "../NativeBoard";
import type { ChuSnapshot } from "./native";
import { NativeAnalysis, NativeRecovery, NativeHistory } from "./NativeSessionPanels";
import { GameBoard } from "./GameBoard";
import inspectionJson from "../__fixtures__/shared-chu-inspection.json";
import type { MovementInspection } from "./movementInspection";
import routesJson from "../__fixtures__/shared-chu-routes.json";
import promotionJson from "../__fixtures__/shared-chu-promotion.json";
import royalsJson from "../__fixtures__/shared-chu-royals.json";
import counterJson from "../__fixtures__/shared-chu-counter.json";
import capturedJson from "../__fixtures__/shared-chu-captured.json";
const routes = routesJson as unknown as ChuSnapshot, promotion = promotionJson as unknown as ChuSnapshot;
afterEach(cleanup);
function square(key: string) {
  const node = document.querySelector<SVGGElement>(`svg [role="button"][aria-label^="${key},"]`);
  if (!node) throw new Error(`Missing native square ${key}`); return node;
}
function board(snapshot = routes) {
  const onPlay = vi.fn(); const props = { snapshot, onPlay, bottomSide: snapshot.orientations[0]!.side.key };
  return { ...render(<NativeBoard {...props} />), props, onPlay };
}
it("preserves actual 12x12 geometry, native ownership, both orientations and multiple royal identities", () => {
  const { props, rerender } = board(royalsJson as unknown as ChuSnapshot);
  expect(document.querySelectorAll("svg [role=button]")).toHaveLength(144);
  expect(square("12a")).toBeTruthy(); expect(square("1l")).toBeTruthy();
  expect(props.snapshot.chu.selected.royals).toHaveLength(3);
  expect(props.snapshot.chu.selected.identities.some(i => i.piece.promoted && i.face === "Prince")).toBe(true);
  expect(props.snapshot.hands).toEqual([]); expect(props.snapshot.capabilities.drops).toBe(false);
  expect(document.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^12a,/);
  rerender(<NativeBoard {...props} bottomSide={props.snapshot.orientations[1]!.side.key} />);
  expect(document.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^1l,/);
  fireEvent.keyDown(square("11l"), { key: "ArrowRight" }); expect(document.activeElement).toBe(square("12l"));
});
it("selects different real Lion returns sharing an endpoint and never submits a prefix", () => {
  const { props, rerender, onPlay } = board();
  const returns = routes.choices.filter(c => c.from === "6f" && c.steps.length === 2 && c.steps[1] === "6f");
  expect(returns.length).toBeGreaterThan(1);
  const capture = returns.find(c => c.steps[0] === "7e")!, quiet = returns.find(c => c.steps[0] === "5f")!;
  for (const choice of [capture, quiet]) {
    fireEvent.click(square(choice.from)); fireEvent.click(square(choice.steps[0]!)); expect(onPlay).not.toHaveBeenCalledTimes(choice === capture ? 1 : 2);
    expect(screen.getByRole("group", { name: "Partial route selection" })).toBeTruthy();
    fireEvent.click(square(choice.steps[1]!)); expect(onPlay).toHaveBeenLastCalledWith(choice.action);
    rerender(<NativeBoard {...props} snapshot={{ ...props.snapshot, token: { ...props.snapshot.token, selection: props.snapshot.token.selection + (choice === capture ? 1 : 2) } }} />);
  }
  expect(onPlay).toHaveBeenCalledTimes(2); expect(capture.action).not.toBe(quiet.action);
});
it("cancels keyboard routes, resets stale selections, and chooses exact promotion identities", () => {
  const { props, rerender, onPlay } = board();
  fireEvent.keyDown(square("6f"), { key: "Enter" }); fireEvent.keyDown(square("7e"), { key: " " }); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square("7e"), { key: "Escape" }); expect(document.activeElement).toBe(square("6f")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("6f"));fireEvent.click(square("7e"));
  rerender(<NativeBoard {...props} snapshot={{ ...routes, token: { ...routes.token, selection: routes.token.selection + 1 } }} />);
  fireEvent.click(square("6f")); expect(onPlay).not.toHaveBeenCalled();
  rerender(<NativeBoard {...props} snapshot={promotion} bottomSide={promotion.orientations[0]!.side.key} />);
  fireEvent.click(square("6e"));fireEvent.click(square("6d")); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" }); expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("6e"));fireEvent.click(square("6d"));fireEvent.click(screen.getByRole("button", { name: "Promote" }));
  const choice = promotion.choices.find(c => c.from === "6e" && c.steps[0] === "6d" && c.completion?.label === "Promote")!;
  expect(onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
});
it("decorates ordered native captures and full routes independently of selection", () => {
  const s = capturedJson as unknown as ChuSnapshot;
  const { container } = render(<NativeBoard snapshot={s} bottomSide={s.orientations[0]!.side.key} onPlay={vi.fn()} completedAction={s.moves.last_action} />);
  expect([...container.querySelectorAll("[data-native-capture]")].map(n => n.getAttribute("data-native-capture"))).toEqual(["2b", "1b"]);
  expect(container.querySelector("[data-route]")?.getAttribute("data-route")).toBe("3c>2b>1b");
  expect(container.querySelector('[data-last-move="origin"]')?.getAttribute("data-location")).toBe("3c");
  expect(container.querySelector('[data-last-move="destination"]')?.getAttribute("data-location")).toBe("1b");
});
it("labels the infrastructure surface and never advertises Chu execution or invented scores", () => {
  const command = vi.fn();render(<><GameBoard snapshot={routes} disabled={false} inspectionDisabled={false} command={command} />
    <NativeRecovery snapshot={routes} error="" disabled={false} command={command} />
    <NativeAnalysis snapshot={routes} defaults={routes.play_limits} disabled={false} command={command} maxDepth={1} minNodes={1} maxNodes={1} /></>);
  expect(screen.getByText(/CHU DEVELOPMENT CONTRACT/)).toBeTruthy();expect(screen.getByText(/Chu execution is unsupported/)).toBeTruthy();
  expect(screen.queryByRole("button", { name: /Resume|Retry|Start move analysis/ })).toBeNull();
  expect(screen.queryByText(/Play limit: depth 0/)).toBeNull(); expect(command).not.toHaveBeenCalled();
});
it("distinguishes native terminal prefix from continued action and invalidates history/inspection work", async () => {
  const { props, rerender, onPlay } = board();
  const short = routes.choices.find(c => c.from === "6f" && c.steps.length === 1 && c.steps[0] === "7e")!;
  expect(short).toBeTruthy();
  fireEvent.click(square("6f"));fireEvent.click(square("7e"));expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Complete action here" }));expect(onPlay).toHaveBeenCalledExactlyOnceWith(short.action);
  const response = inspectionJson as unknown as MovementInspection;
  let resolveOld!: (value: MovementInspection) => void;
  const old = new Promise<MovementInspection>(resolve => { resolveOld = resolve; });
  const onInspect = vi.fn().mockReturnValueOnce(old);
  const historical = { ...routes, historical: true, capabilities: { ...routes.capabilities, board_action: false }, token: { ...routes.token, selection: routes.token.selection + 1 } };
  rerender(<NativeBoard {...props} snapshot={historical} onInspect={onInspect} inspectionDisabled={false} />);
  fireEvent.click(square("11a"));
  expect(onInspect).toHaveBeenCalledExactlyOnceWith("11a");
  const fresh = { ...historical, token: { ...historical.token, selection: historical.token.selection + 1 } };
  rerender(<NativeBoard {...props} snapshot={fresh} onInspect={onInspect} inspectionDisabled={false} />);
  resolveOld(response);
  await vi.waitFor(() => expect(document.querySelectorAll(".inspection-marker")).toHaveLength(0));
  onInspect.mockResolvedValue({ ...response, token: fresh.token });fireEvent.click(square("11a"));
  await vi.waitFor(() => expect(document.querySelectorAll(".inspection-marker")).toHaveLength(1));
  expect(screen.getByText("Inspecting Gote Pawn · Esc clears")).toBeTruthy();
  fireEvent.click(square("11b"));expect(onPlay).toHaveBeenCalledTimes(1);
  fireEvent.keyDown(square("11b"), { key: "Escape" });expect(document.querySelectorAll(".inspection-marker")).toHaveLength(0);
});

it("keeps exact declared counters and native provenance without fabricating prior move rows", () => {
  const s = counterJson as unknown as ChuSnapshot;
  expect(s.chu.declared_root_plies).toBe("18446744073709551614");
  expect(s.chu.selected.declared_total_plies).toBe("18446744073709551615");
  expect(s.chu.earlier_history_unknown).toBe(true);expect(s.chu.historical_reachability_proven).toBe(false);
  expect(s.moves.entries).toHaveLength(1);expect(s.moves.entries[0]!.number).toBe(1);
  render(<NativeHistory snapshot={s} disabled={false} command={vi.fn()} />);
  expect(screen.getByText(/known-root move pairs · declared root plies 18446744073709551614/)).toBeTruthy();
  expect(screen.getByRole("button", { name: /^1. Sente:/ })).toBeTruthy();
});
