// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { TakeBackControl } from "./TakeBackControl";
afterEach(cleanup);
it("admits one native transaction and restores focus only after a new accepted context", async () => {
  const restore = vi.fn().mockResolvedValue({});
  const props = { available: true, disabled: false, contextKey: "before", restore };
  const view = (next = props) => <><h2 data-game-status tabIndex={-1}>Your turn</h2><TakeBackControl {...next} /></>;
  const { rerender } = render(view());
  const button = screen.getByRole("button", { name: "Take back" }); button.focus();
  fireEvent.click(button); expect(restore).toHaveBeenCalledOnce();
  rerender(view({ ...props, disabled: true })); expect(document.activeElement).toBe(button);
  await Promise.resolve();
  rerender(view({ ...props, contextKey: "after", available: false }));
  expect(document.activeElement).toBe(screen.getByRole("heading"));
  expect((button as HTMLButtonElement).disabled).toBe(true);
});
it("disables unavailable and busy work, with no confirmation or history reconstruction", () => {
  const restore = vi.fn();
  const { rerender } = render(<TakeBackControl available={false} disabled={false} contextKey="a" restore={restore} />);
  fireEvent.click(screen.getByRole("button")); expect(restore).not.toHaveBeenCalled();
  rerender(<TakeBackControl available disabled contextKey="a" restore={restore} />);
  fireEvent.click(screen.getByRole("button")); expect(restore).not.toHaveBeenCalled();
  expect(screen.queryByRole("dialog")).toBeNull();
});
it("does not move focus after native refusal even when a later snapshot arrives", async () => {
  const restore = vi.fn().mockResolvedValue(null);
  const { rerender } = render(<><h2 data-game-status tabIndex={-1}>Your turn</h2><TakeBackControl available disabled={false} contextKey="a" restore={restore} /></>);
  const button = screen.getByRole("button"); button.focus(); fireEvent.click(button); await Promise.resolve();
  rerender(<><h2 data-game-status tabIndex={-1}>Your turn</h2><TakeBackControl available disabled={false} contextKey="b" restore={restore} /></>);
  expect(document.activeElement).toBe(button);
});
