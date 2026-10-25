// @vitest-environment jsdom
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  within,
} from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import {
  useDocumentGuard,
  UnsavedDialog,
  type NativeActions,
} from "./Document";
import { mockSession } from "./__tests__/session";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
const standard = standardJson as Snapshot;
const dirty = () => ({
  ...standard,
  document: { ...standard.document, dirty: true, revision: 10 },
});
afterEach(cleanup);
function Harness({
  snapshot,
  actions,
  closeRequest = null,
}: {
  snapshot: Snapshot;
  actions: NativeActions;
  closeRequest?: Snapshot | null;
}) {
  const guard = useDocumentGuard(snapshot, closeRequest, actions);
  return (
    <>
      <button onClick={() => guard.requestNewGame("black")}>New</button>
      <button onClick={guard.requestOpen}>Open</button>
      {guard.pending !== null && (
        <UnsavedDialog
          guard={guard}
          dirty={snapshot.document.dirty}
          busy={false}
        />
      )}
    </>
  );
}
it("requires explicit discard or save before replacing a dirty game, and cancellation preserves it", async () => {
  const snapshot = dirty();
  const actions = mockSession(snapshot);
  render(<Harness snapshot={snapshot} actions={actions} />);
  fireEvent.click(screen.getByText("New"));
  expect(actions.command).not.toHaveBeenCalled();
  const dialog = screen.getByRole("dialog");
  expect(document.activeElement).toBe(within(dialog).getByText("Cancel"));
  fireEvent.click(screen.getByText("Cancel"));
  expect(screen.queryByRole("dialog")).toBeNull();
  expect(actions.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByText("New"));
  await act(async () => {
    fireEvent.click(screen.getByText("Discard & continue"));
  });
  expect(actions.command).toHaveBeenCalledExactlyOnceWith({
    kind: "new_game",
    human: "black",
    version: snapshot.version,
    document_revision: 10,
  });
});
it("binds unsaved decisions to document revision even when game version is unchanged", () => {
  const snapshot = dirty();
  const actions = mockSession(snapshot);
  const { rerender } = render(
    <Harness snapshot={snapshot} actions={actions} />,
  );
  fireEvent.click(screen.getByText("New"));
  rerender(
    <Harness
      snapshot={{
        ...snapshot,
        document: { ...snapshot.document, revision: 11 },
      }}
      actions={actions}
    />,
  );
  expect(
    (screen.getByText("Discard & continue") as HTMLButtonElement).disabled,
  ).toBe(true);
  expect(screen.getByRole("alert").textContent).toContain("settings changed");
  fireEvent.click(screen.getByText("Discard & continue"));
  expect(actions.command).not.toHaveBeenCalled();
});
it("does not continue after a failed or cancelled save or after saving an older revision", async () => {
  const snapshot = dirty();
  const actions = mockSession(snapshot);
  vi.mocked(actions.save)
    .mockResolvedValueOnce(null)
    .mockResolvedValueOnce(snapshot);
  render(<Harness snapshot={snapshot} actions={actions} />);
  fireEvent.click(screen.getByText("Open"));
  await act(async () => {
    fireEvent.click(screen.getByText("Save & continue"));
  });
  expect(actions.open).not.toHaveBeenCalled();
  expect(screen.getByRole("dialog")).toBeTruthy();
  await act(async () => {
    fireEvent.click(screen.getByText("Save & continue"));
  });
  expect(actions.open).not.toHaveBeenCalled();
  expect(screen.getByRole("alert").textContent).toContain("not fully saved");
});
it("awaits actual save completion and uses its exact current revision for Open; open failure retains the guard", async () => {
  const snapshot = dirty();
  const actions = mockSession(snapshot);
  let finish!: (snapshot: Snapshot) => void;
  vi.mocked(actions.save).mockReturnValue(
    new Promise((resolve) => {
      finish = resolve;
    }),
  );
  vi.mocked(actions.open).mockResolvedValue(null);
  render(<Harness snapshot={snapshot} actions={actions} />);
  fireEvent.click(screen.getByText("Open"));
  await act(async () => {
    fireEvent.click(screen.getByText("Save & continue"));
  });
  expect(actions.open).not.toHaveBeenCalled();
  const saved = {
    ...snapshot,
    document: { ...snapshot.document, dirty: false, revision: 12 },
  };
  await act(async () => {
    finish(saved);
    await Promise.resolve();
  });
  expect(actions.open).toHaveBeenCalledExactlyOnceWith(snapshot.version, 12);
  expect(screen.getByRole("dialog")).toBeTruthy();
  expect(screen.getByText(/current game has been retained/)).toBeTruthy();
});
it("uses the native paused close payload and cancels the native gate without losing the game", async () => {
  const snapshot = dirty();
  const close = {
    ...snapshot,
    version: 9,
    document: { ...snapshot.document, revision: 20 },
  };
  const actions = mockSession(close);
  render(<Harness snapshot={close} closeRequest={close} actions={actions} />);
  expect(screen.getByText(/closing the application/)).toBeTruthy();
  expect(actions.finishClose).not.toHaveBeenCalled();
  await act(async () => {
    fireEvent.click(screen.getByText("Cancel"));
  });
  expect(actions.cancelClose).toHaveBeenCalledExactlyOnceWith();
  expect(screen.queryByRole("dialog")).toBeNull();
});
it("saves before native Quit approval and passes the saved document revision", async () => {
  const close = dirty();
  const actions = mockSession(close);
  vi.mocked(actions.save).mockResolvedValue({
    ...close,
    document: { ...close.document, revision: 11, dirty: false },
  });
  render(<Harness snapshot={close} closeRequest={close} actions={actions} />);
  await act(async () => {
    fireEvent.click(screen.getByText("Save & continue"));
  });
  expect(actions.finishClose).toHaveBeenCalledExactlyOnceWith(11);
  expect(screen.queryByRole("dialog")).toBeNull();
});
