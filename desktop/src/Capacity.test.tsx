// @vitest-environment jsdom
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { CapacityNotice } from "./Capacity";
afterEach(cleanup);

it("distinguishes an application capacity pause from chess and preserves an actionable save path", () => {
  render(<CapacityNotice capacity={{ id: 257, status: "paused", message: "Application capacity: the document already contains 256 observations." }} />);
  const notice = screen.getByRole("alert");
  expect(notice.textContent).toContain("Application limit reached");
  expect(notice.textContent).toContain("No chess result or illegal move is implied");
  expect(notice.textContent).toContain("complete accepted game and observations are preserved");
  expect(notice.textContent).toContain("Save or Save as");
  expect(notice.textContent).toContain("does not reset their limits");
  expect(notice.textContent).not.toMatch(/retry/i);
});
it("reports native admission work separately from search progress", () => {
  render(<CapacityNotice capacity={{ id: 1, status: "checking", message: null }} />);
  expect(screen.getByRole("status").textContent).toContain("Checking complete save capacity");
  expect(screen.queryByRole("alert")).toBeNull();
});
it("has no capacity alarm when native admission is available", () => {
  render(<CapacityNotice capacity={{ id: 1, status: "available", message: null }} />);
  expect(screen.queryByLabelText("Document capacity")).toBeNull();
});

it("reveals a new native resource pause once and preserves focus through subsequent polls", () => {
  const view = (id: number, status: "available" | "paused") => (
    <><button>Save</button><CapacityNotice capacity={{ id, status, message: null }} /></>
  );
  const { rerender } = render(view(1, "available"));
  const save = screen.getByRole("button", { name: "Save" });
  save.focus();
  rerender(view(2, "paused"));
  const notice = screen.getByRole("alert");
  expect(document.activeElement).toBe(notice);
  expect(notice.tabIndex).toBe(-1);
  save.focus();
  rerender(view(2, "paused"));
  expect(document.activeElement).toBe(save);
  rerender(view(3, "paused"));
  expect(document.activeElement).toBe(notice);
});
