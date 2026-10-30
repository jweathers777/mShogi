import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Board } from "../Board";
import type { Color } from "../native";
import type { GameSnapshot } from "../product/native";
import type { SyntheticCommand, SyntheticSnapshot } from "./native";
import "./synthetic.css";

function CloseDialog({ busy, onCancel, onClose }: { busy: boolean; onCancel: () => void; onClose: () => void }) {
  const dialog = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const prior = document.activeElement;
    dialog.current?.querySelector<HTMLButtonElement>("button")?.focus();
    return () => {
      if (prior instanceof HTMLElement || prior instanceof SVGElement) prior.focus();
    };
  }, []);
  function key(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape" && !busy) { event.preventDefault(); onCancel(); }
    if (event.key === "Tab") {
      event.preventDefault();
      const buttons = [...(dialog.current?.querySelectorAll<HTMLButtonElement>("button:not(:disabled)") ?? [])];
      const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
      buttons[(index + (event.shiftKey ? -1 : 1) + buttons.length) % buttons.length]?.focus();
    }
  }
  return <div className="modal-backdrop"><div className="promotion-dialog" role="dialog" aria-modal="true" aria-labelledby="synthetic-close-title" ref={dialog} onKeyDown={key}>
    <h2 id="synthetic-close-title">Close synthetic fixture?</h2>
    <p>Accepted actions are temporary verification records. They cannot be saved as chess documents.</p>
    <button className="secondary" disabled={busy} onClick={onCancel}>Cancel close</button>
    <button className="quiet" disabled={busy} onClick={onClose}>Close fixture</button>
  </div></div>;
}

export function SyntheticApp() {
  const [snapshot, setSnapshot] = useState<SyntheticSnapshot | null>(null);
  const [orientation, setOrientation] = useState<Color>("white");
  const [error, setError] = useState("");
  const [busy, setBusy] = useState(false);
  const [closeRequest, setCloseRequest] = useState<GameSnapshot | null>(null);
  const latest = useRef<SyntheticSnapshot | null>(null);
  const closing = useRef<GameSnapshot | null>(null);
  const performing = useRef(false);
  const alive = useRef(false);
  function accept(incoming: SyntheticSnapshot) {
    if (!alive.current) return;
    if (latest.current === null || incoming.version >= latest.current.version) {
      latest.current = incoming;
      setSnapshot(incoming);
    }
  }
  useEffect(() => {
    alive.current = true;
    let stopped = false;
    let unlisten: (() => void) | undefined;
    void listen<GameSnapshot>("game-close-requested", (event) => {
      if (stopped) return;
      closing.current = event.payload;
      setCloseRequest(event.payload);
    }).then(async (stop) => {
      if (stopped) { stop(); return; }
      unlisten = stop;
      try {
        const incoming = await invoke<SyntheticSnapshot>("synthetic_snapshot");
        if (!stopped) accept(incoming);
      } catch (reason: unknown) {
        if (!stopped) setError(String(reason));
      }
    }).catch((reason: unknown) => {
      if (!stopped) setError(`Native close protection: ${String(reason)}`);
    });
    return () => { stopped = true; alive.current = false; unlisten?.(); };
  }, []);
  async function command(command: SyntheticCommand) {
    if (performing.current || closing.current !== null || !alive.current) return;
    performing.current = true;
    setBusy(true);
    setError("");
    try { accept(await invoke<SyntheticSnapshot>("synthetic_command", { command })); }
    catch (reason: unknown) { if (alive.current) setError(String(reason)); }
    finally { performing.current = false; if (alive.current) setBusy(false); }
  }
  async function finishClose(cancel: boolean) {
    const request = closing.current;
    if (request === null || performing.current || !alive.current) return;
    performing.current = true;
    setBusy(true);
    setError("");
    try {
      if (cancel) {
        await invoke("game_cancel_close");
        closing.current = null;
        if (alive.current) setCloseRequest(null);
        accept(await invoke<SyntheticSnapshot>("synthetic_snapshot"));
      } else {
        await invoke("game_finish_close", { expected: request.open_token });
      }
    } catch (reason: unknown) { if (alive.current) setError(String(reason)); }
    finally { performing.current = false; if (alive.current) setBusy(false); }
  }
  const blocked = busy || closeRequest !== null;
  return <main className="application synthetic-application">
    <header className="app-header"><div><p className="eyebrow">SYNTHETIC DEVELOPMENT FIXTURE</p><h1>Invented strider · 16 by 16</h1></div><span className="synthetic-badge">Verification build</span></header>
    <p className="synthetic-introduction">This invented fixture tests the shared board and staged selection. It is not a real shogi variant. Rust supplies complete choices and accepts one complete action; partial selection never changes its board or record.</p>
    {error && <p className="notice" role="alert">{error}</p>}
    {snapshot === null ? <p role="status">Connecting to the native synthetic session…</p> : <div className="synthetic-workspace">
      <section className="synthetic-board" aria-label="Synthetic fixture board">
        <div className="board-heading"><h2>{snapshot.complete ? "Complete action accepted" : "Choose a complete route"}</h2><button className="quiet" disabled={blocked} onClick={() => setOrientation((value) => value === "white" ? "black" : "white")} aria-label={`Flip board; currently ${orientation} at bottom`}>↕ Flip board</button></div>
        <Board geometry={{ ...snapshot.geometry, initialFocus: snapshot.geometry.initial_focus }} profile={snapshot.profile} pieces={snapshot.pieces} choices={snapshot.choices} glyphs={{ strider: "S", stone: "●" }} orientation={orientation} version={snapshot.version} disabled={blocked || snapshot.complete} onPlay={(action) => { void command({ kind: "play", version: snapshot.version, action }); }} />
      </section>
      <aside className="synthetic-inspector" aria-label="Synthetic fixture controls and record">
        <section className="card"><h2>Invented rules</h2><p>The strider S takes one or two adjacent steps. Each enemy stone on its chosen route is captured. It can return to its origin. Stones do not move.</p><p>Choose i9, then j10, then i9 to capture and return. The alternative i9 → h9 → i9 ends on the same square and leaves the stone intact.</p><p>If a step can finish now or continue, choose <strong>Complete action here</strong> to stop at that step. Escape abandons the selection.</p></section>
        <section className="card"><h2>Native acceptance</h2><p role="status">Version {snapshot.version} · Accepted actions {snapshot.record.actions.length}</p><p>{snapshot.complete ? "The native fixture accepted a complete action. Reset to compare another route." : "No complete action has been accepted in this demonstration."}</p><button className="secondary" disabled={blocked} onClick={() => { void command({ kind: "reset", version: snapshot.version }); }}>Reset synthetic fixture</button><p className="caption">Reset starts another temporary demonstration and advances the native version.</p></section>
        <details className="card"><summary>Separate synthetic record</summary><p className="caption">Temporary verification data; not a chess document.</p><pre>{JSON.stringify(snapshot.record, null, 2)}</pre></details>
      </aside>
    </div>}
    {closeRequest !== null && <CloseDialog busy={busy} onCancel={() => { void finishClose(true); }} onClose={() => { void finishClose(false); }} />}
  </main>;
}
