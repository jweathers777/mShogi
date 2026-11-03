// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { DiagnosticText, DIAGNOSTIC_PAGE_CHARACTERS } from "./DiagnosticText";
import { SupportDialog } from "./SupportDialog";
import type { GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import chess from "../__fixtures__/shared-chess-standard.json";
import actualCapacityArchive from "../../scripts/fixtures/chu-capacity256.mshogi?raw";

afterEach(cleanup);
it("renders a bounded page of the real 256-observation archive and reconstructs every byte", () => {
  const archive: unknown = JSON.parse(actualCapacityArchive), text = JSON.stringify(archive);
  const { container } = render(<DiagnosticText text={text} />);
  let complete = "", pages = 0;
  do {
    const pre = container.querySelector("pre")!;
    expect(pre.textContent!.length).toBeLessThanOrEqual(DIAGNOSTIC_PAGE_CHARACTERS);
    expect(container.querySelectorAll("pre")).toHaveLength(1);
    expect(Number(pre.dataset.diagnosticsPage)).toBe(pages++);
    complete += pre.textContent;
    const next = screen.getByRole("button", { name: "Next diagnostics page" }) as HTMLButtonElement;
    if (next.disabled) break;
    fireEvent.click(next);
  } while (true);
  expect(pages).toBeGreaterThan(30);
  expect(complete).toBe(text);
  expect(JSON.parse(complete)).toEqual(archive);
  fireEvent.click(screen.getByRole("button", { name: "Previous diagnostics page" }));
  expect(Number(container.querySelector("pre")!.dataset.diagnosticsPage)).toBe(pages - 2);
});
it("keeps a supplementary Unicode character intact at a page boundary", () => {
  const prefix = "a".repeat(DIAGNOSTIC_PAGE_CHARACTERS - 1), { container } = render(<DiagnosticText text={prefix + "🦁終"} />);
  expect(container.querySelector("pre")!.textContent).toBe(prefix);
  fireEvent.click(screen.getByRole("button", { name: "Next diagnostics page" }));
  expect(container.querySelector("pre")!.textContent).toBe("🦁終");
});
it("captures one complete diagnostic snapshot on disclosure and refreshes only after closing it", async () => {
  const session = { error: "first", requestClose: vi.fn() } as unknown as GameSession;
  const props = { snapshot: chess as GameSnapshot, session, close: vi.fn() };
  const { container, rerender } = render(<SupportDialog {...props} />);
  expect(container.querySelector("pre[data-native-diagnostics]")).toBeNull();
  const summary = screen.getByText("Diagnostics");
  fireEvent.click(summary);
  await waitFor(() => expect(container.querySelector("[data-native-diagnostics]")).toBeTruthy());
  const first = container.querySelector("[data-native-diagnostics]")!.textContent;
  rerender(<SupportDialog {...props} session={{ ...session, error: "new observation" }} />);
  expect(container.querySelector("[data-native-diagnostics]")!.textContent).toBe(first);
  fireEvent.click(summary);
  await waitFor(() => expect(container.querySelector("[data-native-diagnostics]")).toBeNull());
  fireEvent.click(summary);
  await waitFor(() => expect(container.querySelector("[data-native-diagnostics]")!.textContent).toContain("new observation"));
});
