import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { newestSnapshot, type DesktopCommand, type Snapshot } from "./native";

export function useNativeSession() {
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const latest = useRef<Snapshot | null>(null);
  const [documentEpoch, setDocumentEpoch] = useState(0);
  const [closeRequest, setCloseRequest] = useState<Snapshot | null>(null);
  const [error, setError] = useState("");
  const [connected, setConnected] = useState(false);
  const [busy, setBusy] = useState(false);
  const commanding = useRef(false);
  const alive = useRef(false);
  const accept = useCallback((incoming: Snapshot) => {
    if (!alive.current) return;
    latest.current = newestSnapshot(latest.current, incoming);
    setSnapshot(latest.current);
    setConnected(true);
    setError((current) =>
      current.startsWith("Native connection:") ? "" : current,
    );
  }, []);
  useEffect(() => {
    alive.current = true;
    let stopped = false;
    let timeout: ReturnType<typeof setTimeout>;
    let unlisten: (() => void) | undefined;
    async function poll() {
      try {
        const incoming = await invoke<Snapshot>("desktop_snapshot");
        if (!stopped) accept(incoming);
      } catch (reason: unknown) {
        if (!stopped) {
          setConnected(false);
          setError(`Native connection: ${String(reason)}`);
        }
      }
      if (!stopped)
        timeout = setTimeout(() => {
          void poll();
        }, 250);
    }
    void listen<Snapshot>("desktop-close-requested", (event) => {
      if (stopped) return;
      accept(event.payload);
      setCloseRequest(event.payload);
    })
      .then((stop) => {
        if (stopped) stop();
        else {
          unlisten = stop;
          void poll();
        }
      })
      .catch((reason: unknown) => {
        if (!stopped) {
          setConnected(false);
          setError(`Native close protection: ${String(reason)}`);
        }
      });
    return () => {
      stopped = true;
      alive.current = false;
      clearTimeout(timeout);
      unlisten?.();
    };
  }, [accept]);
  const perform = useCallback(
    async (
      name: string,
      args?: Record<string, unknown>,
    ): Promise<Snapshot | null> => {
      if (latest.current === null || commanding.current || !alive.current)
        return null;
      commanding.current = true;
      setBusy(true);
      setError("");
      try {
        const incoming = await invoke<Snapshot>(name, args);
        accept(incoming);
        return latest.current;
      } catch (reason: unknown) {
        if (alive.current) setError(String(reason));
        return null;
      } finally {
        commanding.current = false;
        if (alive.current) setBusy(false);
      }
    },
    [accept],
  );
  const command = useCallback(
    (command: DesktopCommand) => perform("desktop_command", { command }),
    [perform],
  );
  const save = useCallback(() => perform("desktop_save"), [perform]);
  const saveAs = useCallback(() => perform("desktop_save_as"), [perform]);
  const open = useCallback(
    async (version: number, documentRevision: number) => {
      const priorIo = latest.current?.io.id;
      const result = await perform("desktop_open", {
        expectedVersion: version,
        expectedDocumentRevision: documentRevision,
      });
      if (
        result !== null &&
        result.io.phase === "complete" &&
        result.io.id !== priorIo
      )
        setDocumentEpoch((epoch) => epoch + 1);
      return result;
    },
    [perform],
  );
  const cancelClose = useCallback(async () => {
    const result = await perform("desktop_cancel_close");
    if (result !== null) setCloseRequest(null);
    return result;
  }, [perform]);
  const finishClose = useCallback(
    async (documentRevision: number): Promise<boolean> => {
      if (!alive.current) return false;
      try {
        await invoke("desktop_finish_close", {
          expectedDocumentRevision: documentRevision,
        });
        return true;
      } catch (reason: unknown) {
        if (alive.current) setError(String(reason));
        return false;
      }
    },
    [],
  );
  const requestClose = useCallback(async () => {
    try {
      await invoke("desktop_request_close");
    } catch (reason: unknown) {
      if (alive.current) setError(String(reason));
    }
  }, []);
  return {
    snapshot,
    documentEpoch,
    error,
    connected,
    busy,
    command,
    save,
    saveAs,
    open,
    closeRequest,
    cancelClose,
    finishClose,
    requestClose,
  };
}
