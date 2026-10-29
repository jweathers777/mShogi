// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { NativeRecovery } from "./NativeSessionPanels";
import { ShogiApp } from "./ShogiApp";
import { XiangqiApp } from "./XiangqiApp";
import type { ChessShogiSnapshot, GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
import played from "../__fixtures__/shared-xiangqi-played.json";

afterEach(cleanup);
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

// Actual native reasons from shared_archive::{counts,check_capacity,replay_work}
// and shared_xiangqi_archive::{check_capacity,raw_work}. The controller regression
// verification_control_limits_* executes both real zero-observation refusals.
const reasons = [
  "The supported shogi archive retains256 observations and8192 controls, including a stop/close reservation.",
  "The archive retains256 total ordinary/exact observations and8192 controls including stop/close.",
  "The complete shogi archive would exceed16MiB, including identity and stop/close headroom.",
  "The actual complete encoded archive exceeds16MiB including reserved identity/stop/close headroom.",
  "Shogi history exceeds its native321-event parser bound.",
  "Native Xiangqi command/event bound exceeded.",
  "The document exceeds the bounded16,000 native replay-action work budget.",
  "The complete document exceeds16,000 weighted native replay work units.",
  "An actual encoded canonical wrapper exceeds1MiB including archive indentation.",
];
const diagnostic = (reason: string) => `Application capacity: ${reason}`;
function paused(game: "shogi" | "xiangqi", message: string): GameSnapshot {
  const native = game === "shogi" ? shogi : xiangqi;
  return { ...native, document: { ...native.document }, capacity: { id: 1, status: "paused", message } } as unknown as GameSnapshot;
}
function product(snapshot: GameSnapshot, error = "") {
  const session: GameSession = { snapshot, busy: false, error, closeRequested: false,
    command: vi.fn(), file: vi.fn(), newGame: vi.fn(), cancelClose: vi.fn(), finishClose: vi.fn() };
  const props = { session, onNewGame: vi.fn(), onOpen: vi.fn(), blocked: false };
  render(snapshot.game === "xiangqi" ? <XiangqiApp {...props} snapshot={snapshot} />
    : <ShogiApp {...props} snapshot={snapshot as ChessShogiSnapshot} />);
  return props;
}
for (const game of ["shogi", "xiangqi"] as const) {
  for (const reason of reasons) {
    it(`${game} keeps ambiguous/control/byte/event/replay refusal truthful: ${reason}`, () => {
      const snapshot = paused(game, diagnostic(reason));
      const { session, onOpen } = product(snapshot);
      const alert = screen.getByRole("alert");
      expect(alert.textContent).toContain("Your current game can still be saved.");
      expect(alert.textContent).toContain("Saving and reopening does not reset this limit.");
      expect(alert.textContent).not.toMatch(/256|8192|recorded-operation limit|recorded-observation storage|16 MiB/);
      expect(screen.queryByText("Engine connection failed")).toBeNull();
      expect(screen.queryByText(/illegal action/i)).toBeNull();
      for (const [name, action] of [["Save", "save"], ["Save as…", "save_as"]] as const) {
        fireEvent.click(screen.getByRole("button", { name }));
        expect(session.file).toHaveBeenLastCalledWith(snapshot, action);
      }
      fireEvent.click(screen.getByRole("button", { name: "Open…" })); expect(onOpen).toHaveBeenCalledOnce();
      expect(document.querySelector(".shell-technical")?.textContent).toContain(reason);
    });
  }
}

// Count-only presentation DTOs exercise accounting, not canonical admission or
// resumable observations. Real256/257 and mixed exact admission remain native tests.
function counted(game: "shogi" | "xiangqi", ordinary: number, exact: number, message: string): GameSnapshot {
  const snapshot = paused(game, message);
  snapshot.observations = Array.from({ length: ordinary }, () => played.observations[0]!);
  if (snapshot.game === "xiangqi") snapshot.xiangqi = { ...snapshot.xiangqi, exact_observations: Array(exact).fill(null) };
  return snapshot;
}
for (const [game, ordinary, exact, message] of [
  ["shogi", 256, 0, "Application capacity: All256 observation slots are retained; no new work was started."],
  ["shogi", 256, 0, "Application capacity: all256 observations are retained. Save this complete document, then start a new document to continue."],
  ["xiangqi", 256, 0, "Application capacity: retained observations are at their limit; accepted history is saveable. Saving/reopening cannot enlarge the limit."],
  ["xiangqi", 255, 1, "Application capacity: native consumer observation retention limit reached; accepted history and observations are preserved. No retry can enlarge this document."],
  ["xiangqi", 0, 256, "Application capacity: retained observations are at their limit; accepted history is saveable. Saving/reopening cannot enlarge the limit."],
] as const) {
  it(`${game} counts real native retention advice using ${ordinary} ordinary + ${exact} exact observations`, () => {
    render(<NativeRecovery snapshot={counted(game, ordinary, exact, message)} error="" disabled={false} command={vi.fn()} />);
    expect(screen.getByRole("alert").textContent).toContain("limit of 256 recorded engine observations");
  });
}
it("does not turn retention byte capacity into a fictional count", () => {
  render(<NativeRecovery snapshot={counted("xiangqi", 3, 1, "Application capacity: retained observations are at their limit; accepted history is saveable.")} error="" disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("alert").textContent).toContain("recorded-observation storage limit");
  expect(screen.getByRole("alert").textContent).not.toContain("256");
});
it("does not infer the cause of combined diagnostics even when256 observations are present", () => {
  render(<NativeRecovery snapshot={counted("xiangqi", 255, 1, diagnostic(reasons[1]!))} error="" disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("alert").textContent).toContain("Your current game can still be saved.");
});
for (const game of ["shogi", "xiangqi"] as const) {
  it(`${game} keeps failed Save feedback and unsaved state visible alongside capacity`, () => {
    const snapshot = paused(game, diagnostic(reasons[1]!));
    snapshot.document.dirty = true;
    snapshot.io = { id: 2, phase: "failed", message: "File write failed" };
    product(snapshot);
    expect(screen.getAllByRole("alert")).toHaveLength(2);
    expect(screen.getByText(/file operation could not complete/)).toBeTruthy();
    expect(screen.getByText("Unsaved")).toBeTruthy();
    expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
  });
}
