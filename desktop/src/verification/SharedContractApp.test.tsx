// @vitest-environment jsdom
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { SharedContractApp } from "./SharedContractApp";
import native from "../__fixtures__/shared-shogi-standard.json";
const bridge = vi.hoisted(() => ({ invoke: vi.fn(), listen: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: bridge.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: bridge.listen }));
let closeListener: (event: { payload: unknown }) => void;
let saveResult = structuredClone(native);
beforeEach(() => {
  bridge.invoke.mockReset(); bridge.listen.mockReset();
  saveResult = structuredClone(native);
  bridge.listen.mockImplementation((_name: string, callback: typeof closeListener) => { closeListener = callback; return Promise.resolve(() => {}); });
  bridge.invoke.mockImplementation((name: string) => Promise.resolve(name === "native_contract_file" ? saveResult : structuredClone(native)));
});
afterEach(cleanup);
it("does not close or replace unsaved work when the Save picker is cancelled", async () => {
  render(<SharedContractApp />);
  await screen.findByRole("button", { name: "Save As" });
  act(() => closeListener({ payload: native }));
  fireEvent.click(screen.getByRole("button", { name: "Save and close" }));
  await waitFor(() => expect(bridge.invoke).toHaveBeenCalledWith("native_contract_file", expect.objectContaining({ action: "save" })));
  await act(async () => {});
  expect(bridge.invoke.mock.calls.some(([name]) => name === "game_finish_close")).toBe(false);
  expect(screen.getByRole("button", { name: "Keep open" })).toBeTruthy();
});
it("closes only after successful save of the exact current document", async () => {
  saveResult.document.dirty = false; saveResult.token.revision += 1;
  render(<SharedContractApp />); await screen.findByRole("button", { name: "Save As" });
  act(() => closeListener({ payload: native }));
  fireEvent.click(screen.getByRole("button", { name: "Save and close" }));
  await waitFor(() => expect(bridge.invoke).toHaveBeenCalledWith("game_finish_close", {
    expected: { document: saveResult.token.document, version: saveResult.token.version, document_revision: saveResult.document.revision },
  }));
});
it("requires an explicit unsaved-change choice before Open and keeps work on cancelled Save", async () => {
  render(<SharedContractApp />); await screen.findByRole("button", { name: "Save As" });
  fireEvent.click(screen.getByRole("button", { name: "Open" }));
  expect(screen.getByRole("dialog", { name: "Replace unsaved native document?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Save then Open" }));
  await waitFor(() => expect(bridge.invoke).toHaveBeenCalledWith("native_contract_file", expect.objectContaining({ action: "save" })));
  await act(async () => {});
  expect(bridge.invoke.mock.calls.some(([name, args]) => name === "native_contract_file" && args.action === "open")).toBe(false);
});
