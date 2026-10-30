// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { NativeRecovery } from "./NativeSessionPanels";
import type { GameSnapshot } from "./native";
import type { ResourceFact } from "../resources";
import chess from "../__fixtures__/shared-chess-standard.json";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
afterEach(cleanup);
const base: ResourceFact = { category: "controls", measurement: "candidate", count: 8192, observations: null, reserved: 1, limit: 8192, scope: "shogi", unit: "controls" };
// Literal diagnostics from native archive admission. These deliberately mention
// other supported bounds: the typed facts alone establish the refused resource.
const diagnostic = "Application capacity: The archive retains256 total ordinary/exact observations and8192 controls including stop/close.";
function paused(game: "chess" | "shogi" | "xiangqi", facts: ResourceFact[] | null = [base]): GameSnapshot {
  const native = structuredClone(game === "chess" ? chess : game === "shogi" ? shogi : xiangqi);
  return { ...native, capacity: { id: 1, status: "paused", message: diagnostic, resource: facts && { facts: facts.map((fact) => ({ ...fact, scope: game })), omitted_facts: 0, diagnostic } } } as unknown as GameSnapshot;
}
function notice(snapshot: GameSnapshot, error = "") { return render(<NativeRecovery snapshot={snapshot} error={error} disabled={false} command={vi.fn()} />); }
for (const game of ["chess", "shogi", "xiangqi"] as const) {
  it(`${game} shows genuine control-only exhaustion with zero observations and no fictional256`, () => {
    const snapshot = paused(game); expect(snapshot.observations).toHaveLength(0); notice(snapshot);
    const message = screen.getByRole("alert", { name: "Document capacity" });
    expect(message.textContent).toContain("control capacity"); expect(message.textContent).toContain("current game can still be saved");
    expect(message.textContent).not.toContain("256"); expect(message.textContent).not.toContain("Engine connection failed");
    expect(message.textContent).toContain("Saving and reopening does not reset this limit");
    expect(document.activeElement).toBe(message);
  });
  it(`${game} cannot infer a limit from an ambiguous diagnostic without authoritative facts`, () => {
    notice(paused(game, null));
    expect(screen.getByRole("alert").textContent).toContain("Your current game can still be saved");
    expect(screen.getByRole("alert").textContent).not.toContain("256");
    expect(screen.getByRole("alert").textContent).not.toContain("control capacity");
  });
  it(`${game} keeps independent failed Save feedback visible alongside capacity`, () => {
    const snapshot = paused(game); snapshot.document.dirty = true; snapshot.io = { id: 2, phase: "failed", message: "Permission denied", resource: null };
    notice(snapshot);
    expect(screen.getAllByRole("alert")).toHaveLength(2); expect(screen.getByText(/file operation could not complete/)).toBeTruthy();
    expect(snapshot.document.dirty).toBe(true);
  });
}
it.each([[256, 0], [255, 1], [0, 256]])("reports actual combined Xiangqi ordinary/exact accounting %s/%s", (ordinary, exact) => {
  notice(paused("xiangqi", [{ ...base, category: "combined_observations", count: ordinary + exact, measurement: "accepted", observations: { ordinary, exact }, limit: 256, unit: "observations" }]));
  expect(screen.getByRole("alert").textContent).toContain("limit of 256 recorded engine observations");
  expect(screen.getByRole("alert").textContent).toContain(`${ordinary} ordinary and ${exact} exact`);
});
it("does not interpret257 candidate observations as accepted history", () => {
  notice(paused("shogi", [{ ...base, category: "ordinary_observations", count: 257, limit: 256, unit: "observations" }]));
  expect(screen.getByRole("alert").textContent).toContain("would exceed"); expect(screen.getByRole("alert").textContent).not.toContain("257 recorded");
});
it.each(["encoded_document_bytes", "native_events", "replay_work", "identity_headroom"] as const)("preserves %s as an independent resource", (category) => {
  notice(paused("xiangqi", [{ ...base, category }]));
  expect(screen.getByRole("alert").textContent).not.toContain("256 recorded"); expect(screen.getByRole("alert").textContent).not.toContain("control capacity");
});
it("keeps unknown command errors generic rather than classifying diagnostic substrings", () => {
  const snapshot = structuredClone(shogi) as unknown as GameSnapshot;
  notice(snapshot, "encoded limit256 application capacity illegal engine failure");
  expect(screen.getByRole("alert").textContent).toContain("action could not complete"); expect(screen.queryByText("Application limit reached")).toBeNull();
});
it("does not refocus capacity notices on identical polls", () => {
  const snapshot = paused("shogi"); const { rerender } = notice(snapshot);
  const elsewhere = document.createElement("button"); document.body.append(elsewhere); elsewhere.focus();
  rerender(<NativeRecovery snapshot={{ ...snapshot }} error="" disabled={false} command={vi.fn()} />);
  expect(document.activeElement).toBe(elsewhere); elsewhere.remove();
});

it("reports capacity checking separately from search progress and no alarm when available", () => {
  const snapshot = structuredClone(shogi) as unknown as GameSnapshot; snapshot.capacity = { id: 7, status: "checking", message: null, resource: null };
  const { rerender } = notice(snapshot); expect(screen.getByText("Checking complete save capacity")).toBeTruthy(); expect(screen.queryByRole("alert")).toBeNull();
  snapshot.capacity = { id: 8, status: "available", message: null, resource: null }; rerender(<NativeRecovery snapshot={snapshot} error="" disabled={false} command={vi.fn()} />); expect(screen.queryByLabelText("Document capacity")).toBeNull();
});
it("uses authoritative capacity instead of duplicating its failed command as an unrelated connection error", () => {
  notice(paused("xiangqi"), diagnostic); expect(screen.getAllByRole("alert")).toHaveLength(1); expect(screen.getByRole("alert").textContent).toContain("control capacity");
});
