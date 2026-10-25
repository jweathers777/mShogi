import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { newestSnapshot, type DesktopCommand, type Snapshot } from "./native";

export function useNativeSession() {
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const latest = useRef<Snapshot | null>(null);
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
    void poll();
    return () => {
      stopped = true;
      alive.current = false;
      clearTimeout(timeout);
    };
  }, [accept]);
  const command = useCallback(
    async (command: DesktopCommand) => {
      const current = latest.current;
      if (current === null || commanding.current || !alive.current) return;
      commanding.current = true;
      setBusy(true);
      setError("");
      try {
        accept(await invoke<Snapshot>("desktop_command", { command }));
      } catch (reason: unknown) {
        if (alive.current) setError(String(reason));
      } finally {
        commanding.current = false;
        if (alive.current) setBusy(false);
      }
    },
    [accept],
  );
  return { snapshot, error, connected, busy, command };
}
