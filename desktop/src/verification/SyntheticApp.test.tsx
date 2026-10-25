// @vitest-environment jsdom
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { SyntheticApp } from "./SyntheticApp";
import type { SyntheticSnapshot } from "./native";
import type { Snapshot } from "../native";
import syntheticJson from "../__fixtures__/synthetic.json";
import standardJson from "../__fixtures__/standard.json";

const bridge = vi.hoisted(() => ({ invoke: vi.fn(), listen: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: bridge.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: bridge.listen }));
let closeListener: (event: { payload: Snapshot }) => void;
beforeEach(() => {
  bridge.invoke.mockReset();
  bridge.listen.mockReset();
  bridge.listen.mockImplementation((_event: string, callback: typeof closeListener) => {
    closeListener = callback;
    return Promise.resolve(() => {});
  });
  bridge.invoke.mockImplementation((name: string, args?: { command?: { kind: string; version: number; action?: string } }) => {
    if (name === "synthetic_snapshot") return Promise.resolve(structuredClone(syntheticJson));
    if (name === "synthetic_command") {
      const snapshot = structuredClone(syntheticJson) as SyntheticSnapshot;
      snapshot.version += 1;
      if (args?.command?.kind === "play") {
        snapshot.record.actions = [args.command.action!];
        snapshot.complete = true;
        snapshot.choices = [];
      }
      return Promise.resolve(snapshot);
    }
    return Promise.resolve(standardJson);
  });
});
afterEach(cleanup);
function square(name: string) { return screen.getByRole("button", { name: new RegExp(`^${name},`) }); }
async function open() {
  render(<SyntheticApp />);
  await screen.findByRole("button", { name: /^i9,/ });
}
it("keeps staged selection local, then submits the exact complete native route", async () => {
  await open();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  expect(bridge.invoke.mock.calls.filter(([name]) => name === "synthetic_command")).toHaveLength(0);
  fireEvent.keyDown(square("j10"), { key: "Escape" });
  expect(bridge.invoke.mock.calls.filter(([name]) => name === "synthetic_command")).toHaveLength(0);
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  fireEvent.click(square("i9"));
  await waitFor(() => expect(bridge.invoke).toHaveBeenCalledWith("synthetic_command", { command: { kind: "play", version: 1, action: "136>153>136" } }));
  await screen.findByText("Version 2 · Accepted actions 1");
  expect(screen.queryByRole("button", { name: /save/i })).toBeNull();
  expect(screen.getByText("SYNTHETIC DEVELOPMENT FIXTURE")).toBeTruthy();
});
it("uses the intercepted native close Cancel route and retains the synthetic record", async () => {
  await open();
  await act(async () => closeListener({ payload: standardJson as Snapshot }));
  const cancel = screen.getByRole("button", { name: "Cancel close" });
  expect(document.activeElement).toBe(cancel);
  fireEvent.keyDown(cancel, { key: "Tab", shiftKey: true });
  expect(document.activeElement).toBe(screen.getByRole("button", { name: "Close fixture" }));
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
  await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  expect(bridge.invoke).toHaveBeenCalledWith("desktop_cancel_close");
  expect(screen.getByText("Version 1 · Accepted actions 0")).toBeTruthy();
  expect(bridge.invoke.mock.calls.filter(([name]) => name === "synthetic_command")).toHaveLength(0);
});
it("finishes native close with the exact hidden clean chess document revision", async () => {
  await open();
  await act(async () => closeListener({ payload: standardJson as Snapshot }));
  fireEvent.click(screen.getByRole("button", { name: "Close fixture" }));
  await waitFor(() => expect(bridge.invoke).toHaveBeenCalledWith("desktop_finish_close", { expectedDocumentRevision: standardJson.document.revision }));
});
