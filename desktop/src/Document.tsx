import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type KeyboardEvent,
} from "react";
import type { Color, Snapshot } from "./native";
import type { useNativeSession } from "./useNativeSession";
export type NativeActions = Pick<
  ReturnType<typeof useNativeSession>,
  | "command"
  | "save"
  | "saveAs"
  | "open"
  | "finishClose"
  | "cancelClose"
  | "requestClose"
>;
type Intent =
  { kind: "new_game"; human: Color } | { kind: "open" } | { kind: "quit" };
interface Pending {
  intent: Intent;
  version: number;
  documentRevision: number;
}
export function useDocumentGuard(
  snapshot: Snapshot | null,
  closeRequest: Snapshot | null,
  actions: NativeActions,
) {
  const [pending, setPending] = useState<Pending | null>(null);
  const [message, setMessage] = useState("");
  const [working, setWorking] = useState(false);
  const operating = useRef(false);
  const handledClose = useRef<Snapshot | null>(null);
  const execute = useCallback(
    async (intent: Intent, source: Snapshot): Promise<boolean> => {
      if (intent.kind === "quit")
        return actions.finishClose(source.document.revision);
      if (intent.kind === "open")
        return (
          (await actions.open(source.version, source.document.revision)) !==
          null
        );
      return (
        (await actions.command({
          kind: "new_game",
          version: source.version,
          document_revision: source.document.revision,
          human: intent.human,
        })) !== null
      );
    },
    [actions.command, actions.open, actions.finishClose],
  );
  const begin = useCallback(
    (intent: Intent, source: Snapshot) => {
      setMessage("");
      if (source.document.dirty) {
        setPending({
          intent,
          version: source.version,
          documentRevision: source.document.revision,
        });
        return;
      }
      void execute(intent, source).then((ok) => {
        if (!ok && intent.kind === "quit") {
          setPending({
            intent,
            version: source.version,
            documentRevision: source.document.revision,
          });
          setMessage("Closing was not confirmed. Your game remains open.");
        }
      });
    },
    [execute],
  );
  useEffect(() => {
    if (closeRequest !== null && handledClose.current !== closeRequest) {
      handledClose.current = closeRequest;
      begin({ kind: "quit" }, closeRequest);
    }
  }, [closeRequest, begin]);
  const requestNewGame = useCallback(
    (human: Color) => {
      if (snapshot !== null) begin({ kind: "new_game", human }, snapshot);
    },
    [snapshot, begin],
  );
  const requestOpen = useCallback(() => {
    if (snapshot !== null) begin({ kind: "open" }, snapshot);
  }, [snapshot, begin]);
  const stale =
    pending !== null &&
    snapshot !== null &&
    (pending.version !== snapshot.version ||
      pending.documentRevision !== snapshot.document.revision);
  async function decide(save: boolean) {
    if (pending === null || snapshot === null || stale || operating.current)
      return;
    operating.current = true;
    setWorking(true);
    setMessage("");
    let source = snapshot;
    try {
      if (save) {
        const saved = await actions.save();
        if (saved === null || saved.document.dirty) {
          setMessage(
            "The current game was not fully saved. The action has not continued. Keep this game open and review the file error.",
          );
          return;
        }
        source = saved;
        setPending({
          ...pending,
          version: saved.version,
          documentRevision: saved.document.revision,
        });
      }
      if (await execute(pending.intent, source)) setPending(null);
      else
        setMessage(
          "The action could not complete. The current game has been retained.",
        );
    } finally {
      operating.current = false;
      setWorking(false);
    }
  }
  async function cancel() {
    if (operating.current || pending === null) return;
    if (pending.intent.kind === "quit") {
      operating.current = true;
      setWorking(true);
      const result = await actions.cancelClose();
      operating.current = false;
      setWorking(false);
      if (result === null) {
        setMessage(
          "The native close request could not be cancelled. Your game is still open; try again.",
        );
        return;
      }
    }
    setPending(null);
    setMessage("");
  }
  return {
    pending,
    stale,
    working,
    message,
    requestNewGame,
    requestOpen,
    saveAndContinue: () => decide(true),
    discardAndContinue: () => decide(false),
    cancel,
  };
}
export function UnsavedDialog({
  guard,
  dirty,
  busy,
}: {
  guard: ReturnType<typeof useDocumentGuard>;
  dirty: boolean;
  busy: boolean;
}) {
  const container = useRef<HTMLDivElement>(null);
  const previousFocus = useRef<HTMLElement | SVGElement | null>(null);
  useLayoutEffect(() => {
    previousFocus.current =
      (document.activeElement instanceof HTMLElement ||
        document.activeElement instanceof SVGElement) &&
      typeof document.activeElement.focus === "function"
        ? document.activeElement
        : null;
    container.current
      ?.querySelector<HTMLButtonElement>("button[data-cancel]")
      ?.focus();
    return () => {
      if (previousFocus.current?.isConnected) previousFocus.current.focus();
      else
        document.querySelector<SVGGElement>('.square[tabindex="0"]')?.focus();
    };
  }, []);
  if (guard.pending === null) return null;
  const disabled = busy || guard.working;
  const action =
    guard.pending.intent.kind === "quit"
      ? "closing the application"
      : guard.pending.intent.kind === "open"
        ? "opening another game"
        : "starting a new game";
  function key(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") {
      event.preventDefault();
      if (!disabled) void guard.cancel();
    }
    if (event.key === "Tab") {
      const buttons = [
        ...(container.current?.querySelectorAll<HTMLButtonElement>(
          "button:not(:disabled)",
        ) ?? []),
      ];
      const index = buttons.indexOf(
        document.activeElement as HTMLButtonElement,
      );
      event.preventDefault();
      buttons[
        (index + (event.shiftKey ? -1 : 1) + buttons.length) % buttons.length
      ]?.focus();
    }
  }
  return (
    <div className="modal-backdrop">
      <div
        className="promotion-dialog document-guard"
        role="dialog"
        aria-modal="true"
        aria-labelledby="unsaved-title"
        aria-describedby="unsaved-description"
        ref={container}
        onKeyDown={key}
      >
        <p className="eyebrow">PROTECT YOUR GAME</p>
        <h2 id="unsaved-title">
          {dirty ? "Save your changes?" : "Continue with this action?"}
        </h2>
        <p id="unsaved-description">
          {dirty
            ? `This game has unsaved changes. Save them before ${action}, or explicitly discard them.`
            : `This game is saved. You can continue ${action}, or cancel.`}
        </p>
        {guard.stale && (
          <p className="guard-message" role="alert">
            The game or document settings changed while this decision was open.
            Cancel and review the current game before trying again.
          </p>
        )}
        {guard.message && (
          <p className="guard-message" role="alert">
            {guard.message}
          </p>
        )}
        <div className="dialog-actions">
          <button
            className="secondary"
            disabled={disabled || guard.stale || !dirty}
            onClick={() => {
              void guard.saveAndContinue();
            }}
          >
            Save &amp; continue
          </button>
          <button
            className="danger"
            disabled={disabled || guard.stale}
            onClick={() => {
              void guard.discardAndContinue();
            }}
          >
            {dirty ? "Discard & continue" : "Continue"}
          </button>
          <button
            className="quiet"
            data-cancel
            disabled={disabled}
            onClick={() => {
              void guard.cancel();
            }}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
export function DocumentToolbar({
  snapshot,
  busy,
  connected,
  actions,
  onOpen,
}: {
  snapshot: Snapshot;
  busy: boolean;
  connected: boolean;
  actions: NativeActions;
  onOpen: () => void;
}) {
  const filename =
    snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled game";
  return (
    <section className="card document-card" aria-label="Game file">
      <div className="card-title">
        <h2 title={snapshot.document.path ?? undefined}>{filename}</h2>
        <span
          className={`document-dirty ${snapshot.document.dirty ? "dirty" : ""}`}
        >
          {snapshot.document.dirty
            ? "Unsaved"
            : snapshot.document.path === null
              ? "Unmodified"
              : "Saved"}
        </span>
      </div>
      <div className="document-actions">
        <button
          className="secondary"
          disabled={busy || !connected}
          onClick={() => {
            void actions.save();
          }}
        >
          Save
        </button>
        <button
          className="quiet"
          disabled={busy || !connected}
          onClick={() => {
            void actions.saveAs();
          }}
        >
          Save as…
        </button>
        <button
          className="quiet"
          disabled={busy || !connected}
          onClick={onOpen}
        >
          Open…
        </button>
      </div>
      {snapshot.io.phase !== "idle" && (
        <p
          className={`caption ${snapshot.io.phase === "failed" ? "io-failure" : ""}`}
          role={snapshot.io.phase === "failed" ? "alert" : "status"}
        >
          {snapshot.io.message ??
            `${snapshot.io.phase === "complete" ? "File operation completed" : snapshot.io.phase === "saving" ? "Saving the native game document…" : snapshot.io.phase === "opening" ? "Validating and reopening the game…" : snapshot.io.phase === "viewing" ? "Reconstructing historical position…" : "File operation failed"}`}
        </p>
      )}
      <details className="document-details">
        <summary>File details</summary>
        <p>{snapshot.document.path ?? "No file selected"}</p>
        <p>
          Document revision {snapshot.document.revision}. Modes, limits, match
          events and observations are saved by the native host.
        </p>
        <button
          className="quiet"
          disabled={busy || !connected}
          onClick={() => {
            void actions.requestClose();
          }}
        >
          Quit application
        </button>
      </details>
    </section>
  );
}
