// @vitest-environment jsdom
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { ApplicationView } from "./ApplicationView";
import type { GameSnapshot, ChessSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import standardJson from "../__fixtures__/shared-chess-standard.json";
import claimJson from "../__fixtures__/shared-chess-claim.json";
import claimedJson from "../__fixtures__/shared-chess-claimed.json";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
const standard = standardJson as ChessSnapshot;
afterEach(cleanup);
function app(snapshot: GameSnapshot = standard) {
  const session: GameSession = { snapshot, busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(snapshot), file: vi.fn().mockResolvedValue(snapshot), newGame: vi.fn().mockResolvedValue(snapshot), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
  const props = { snapshot, session, onNewGame: vi.fn(), onOpen: vi.fn(), blocked: false };
  return { ...render(<ApplicationView {...props} />), props, session };
}
it.each([standardJson, shogiJson, xiangqiJson])("recovers a removed Cancel control to common analysis, without stealing input focus", fixture => {
  const base = fixture as GameSnapshot;
  const working = { ...base, phase: "thinking" as const, execution_purpose: "move_analysis", paused: true, ...(base.game === "chess" ? { chess: { ...base.chess, analysis: { ...base.chess.analysis, status: "thinking" as const } } } : {}) } as GameSnapshot;
  const { props, session, rerender } = app(working);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const cancel = screen.getByRole("button", { name: "Cancel analysis" }); cancel.focus(); fireEvent.click(cancel);
  expect(session.command).toHaveBeenCalledExactlyOnceWith(working, { kind: "cancel" });
  rerender(<ApplicationView {...props} snapshot={base} />);
  expect(document.activeElement).toBe(screen.getByRole("button", { name: "Start move analysis" }));
  const input = screen.getByLabelText("Node limit"); input.focus();
  rerender(<ApplicationView {...props} snapshot={{ ...base, token: { ...base.token, revision: base.token.revision + 1 } }} />);
  expect(document.activeElement).toBe(input);
});
it("restores a button blurred by WebKit while disabled, without overriding newly focused input", () => {
  const { props, session, rerender } = app(); fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const start = screen.getByRole("button", { name: "Start move analysis" }); start.focus();
  rerender(<ApplicationView {...props} session={{ ...session, busy: true }} />);
  document.body.tabIndex = -1; document.body.focus(); document.body.removeAttribute("tabindex");
  rerender(<ApplicationView {...props} />); expect(document.activeElement).toBe(start);
  rerender(<ApplicationView {...props} session={{ ...session, busy: true }} />); start.blur();
  const input = screen.getByLabelText("Node limit"); input.focus();
  rerender(<ApplicationView {...props} />); expect(document.activeElement).toBe(input);
});
it("recovers engine cancellation to Retry, then to the board after the control disappears", () => {
  const working = { ...standard, human: standard.sides[1]!, phase: "thinking" as const, execution_purpose: "engine_turn" };
  const { props, rerender } = app(working); screen.getByRole("button", { name: "Cancel search" }).focus();
  rerender(<ApplicationView {...props} snapshot={{ ...working, phase: "cancelled" }} />);
  const retry = screen.getByRole("button", { name: "Retry engine turn" }); expect(document.activeElement).toBe(retry);
  rerender(<ApplicationView {...props} snapshot={standard} />);
  expect(document.activeElement).toBe(document.querySelector('[data-game-slot="board"] [tabindex="0"]'));
});
it("focuses the readable native conclusion when claiming removes its control", () => {
  const { props, rerender } = app(claimJson as ChessSnapshot);
  const claim = screen.getByRole("button", { name: "Claim selected draw" }); claim.focus(); fireEvent.click(claim);
  rerender(<ApplicationView {...props} snapshot={claimedJson as ChessSnapshot} />);
  expect(document.activeElement).toBe(document.querySelector("[data-game-status]"));
  expect(document.activeElement?.textContent).toContain("Draw");
});
it("recovers Resume and selected-position focus through the retained shared board controller", () => {
  const { props, rerender } = app({ ...standard, paused: true }); screen.getByRole("button", { name: "Resume play" }).focus();
  rerender(<ApplicationView {...props} snapshot={standard} />);
  const original = document.querySelector('[data-game-slot="board"] [tabindex="0"]'); expect(document.activeElement).toBe(original);
  fireEvent.click(original!); expect(original?.getAttribute("aria-pressed")).toBe("true");
  rerender(<ApplicationView {...props} snapshot={{ ...standard, token: { ...standard.token, selection: standard.token.selection + 1 } }} />);
  const current = document.querySelector('[data-game-slot="board"] [tabindex="0"]'); expect(current).toBe(original); expect(document.activeElement).toBe(current); expect(current?.getAttribute("aria-pressed")).toBe("false");
});
it("does not reclaim focus from a native close or replacement dialog", () => {
  const { props, rerender } = app(); screen.getByRole("button", { name: "New game" }).focus();
  rerender(<><ApplicationView {...props} blocked /><button data-testid="guard">Guard control</button></>);
  const guard = screen.getByTestId("guard"); guard.focus();
  rerender(<><ApplicationView {...props} blocked snapshot={{ ...standard, token: { ...standard.token, revision: standard.token.revision + 1 } }} /><button data-testid="guard">Guard control</button></>);
  expect(document.activeElement).toBe(guard);
});
