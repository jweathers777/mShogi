import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { nativeSnapshotIsNewer } from "../NativeBoard";
import type { GameKind, GameSnapshot, NativeSide, OpenToken, Operation } from "./native";
export function useGameSession() {
  const [snapshot, setSnapshot] = useState<GameSnapshot | null>(null);
  const latest = useRef<GameSnapshot | null>(null);
  const alive = useRef(false), operating = useRef(false);
  const [busy, setBusy] = useState(false), [error, setError] = useState("");
  const [closeRequested, setCloseRequested] = useState(false);
  const accept = useCallback((incoming: GameSnapshot) => {
    if (alive.current && nativeSnapshotIsNewer(latest.current, incoming)) {
      latest.current = incoming; setSnapshot(incoming);
    }
  }, []);
  useEffect(() => {
    alive.current = true; let stopped = false;
    let timer: ReturnType<typeof setTimeout>, stop: (() => void) | undefined;
    async function poll() {
      try { const next = await invoke<GameSnapshot>("game_snapshot"); if (!stopped) accept(next); }
      catch (e) { if (!stopped) setError(String(e)); }
      if (!stopped) timer = setTimeout(() => { void poll(); }, 180);
    }
    void listen("game-close-requested", () => {
      // Open/New Game may have changed games before the next poll. The product
      // shell owns close for both games, even while a game view is remounting.
      if (!stopped) {
        void invoke<GameSnapshot>("game_snapshot").then((next) => { if (!stopped) { accept(next); setCloseRequested(true); } }).catch((e: unknown) => { if (!stopped) setError(String(e)); });
      }
    }).then((unlisten) => { if (!stopped) stop = unlisten; else unlisten(); })
      .catch((e: unknown) => { if (!stopped) setError(String(e)); });
    void poll();
    return () => { stopped = true; alive.current = false; clearTimeout(timer); stop?.(); };
  }, [accept]);
  const perform = useCallback(async (name: string, args?: Record<string, unknown>) => {
    if (!alive.current || operating.current) return null;
    operating.current = true; setBusy(true); setError("");
    try {
      const next = await invoke<GameSnapshot>(name, args); accept(next);
      // Polling can observe a later native state before an operation's receipt
      // arrives. A delayed clean Save must not authorize discarding newer work.
      const current = latest.current;
      return alive.current && current?.token.document === next.token.document ? current : null;
    }
    catch (e) { if (alive.current) setError(String(e)); return null; }
    finally { operating.current = false; if (alive.current) setBusy(false); }
  }, [accept]);
  // Every operation uses the exact snapshot on which its UI was rendered. A
  // newer poll must never upgrade an old click's authority.
  const command = (source: GameSnapshot, operation: Operation) => perform("game_command", {
    commandJson: JSON.stringify({ expected: source.token, game: source.game, operation }),
  });
  const file = (source: GameSnapshot, action: "save" | "save_as" | "open") => perform("game_file", { action, expected: source.open_token });
  const newGame = (expected: OpenToken, game: GameKind, human: NativeSide) => perform("game_new", { expected, game, human });
  const cancelClose = async () => {
    try { await invoke("game_cancel_close"); setCloseRequested(false); accept(await invoke<GameSnapshot>("game_snapshot")); return true; }
    catch (e) { setError(String(e)); return false; }
  };
  const finishClose = async (source: GameSnapshot) => {
    try { await invoke("game_finish_close", { expected: source.open_token }); return true; }
    catch (e) { setError(String(e)); return false; }
  };
  return { snapshot, busy, error, command, file, newGame, closeRequested, cancelClose, finishClose };
}
export type GameSession = ReturnType<typeof useGameSession>;
