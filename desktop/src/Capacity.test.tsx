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
