// Separate QA surface, deliberately excluded from normal distributables.
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { NativeBoard, nativeSnapshotIsNewer, type NativeBoardSnapshot } from "../NativeBoard";
import type { Snapshot } from "../native";
interface ContractSnapshot extends NativeBoardSnapshot {
  status: { label: string }; phase: string; diagnostic: string | null;
  event_index: number; historical: boolean; paused: boolean;
  events: { index: number; label: string }[];
  analysis: unknown; declaration_advice: unknown;
}
export function SharedContractApp() {
  const [snapshot, setSnapshot] = useState<ContractSnapshot | null>(null);
  const latest = useRef<ContractSnapshot | null>(null);
  const [bottom, setBottom] = useState("");
  const [error, setError] = useState("");
  const [close, setClose] = useState<Snapshot | null>(null);
  const active = useRef(true);
  const busy = useRef(false);
  function accept(next: ContractSnapshot) {
    if (active.current && nativeSnapshotIsNewer(latest.current, next)) { latest.current = next; setSnapshot(next); }
  }
  useEffect(() => {
    active.current = true;
    let timer: ReturnType<typeof setTimeout>;
    let stop: (() => void) | undefined;
    async function poll() {
      try { accept(await invoke<ContractSnapshot>("native_contract_snapshot")); } catch (e) { if (active.current) setError(String(e)); }
      if (active.current) timer = setTimeout(() => { void poll(); }, 250);
    }
    void poll();
    void listen<Snapshot>("desktop-close-requested", (e) => { if (active.current) setClose(e.payload); }).then((unlisten) => { if (active.current) stop = unlisten; else unlisten(); });
    return () => { active.current = false; clearTimeout(timer); stop?.(); };
  }, []);
  async function command(operation: Record<string, unknown>) {
    if (!latest.current || busy.current) return;
    busy.current = true; setError("");
    try { accept(await invoke<ContractSnapshot>("native_contract_command", { commandJson: JSON.stringify({ expected: latest.current.token, game: latest.current.game, operation }) })); }
    catch (e) { setError(String(e)); } finally { busy.current = false; }
  }
  if (!snapshot) return <main><h1>MSH-72 DEVELOPMENT CONTRACT</h1><p role="status">Loading native session…</p><p role="alert">{error}</p></main>;
  return <main><h1>MSH-72 DEVELOPMENT CONTRACT</h1>
    <p>Native session and shared selection verification. Shogi archive and complete desktop delivery remain MSH-73/74.</p>
    <p role="status">{snapshot.status.label} · {snapshot.phase} · {snapshot.historical ? `Read-only history event ${snapshot.event_index}` : "Live game"}{snapshot.paused ? " · paused" : ""}</p>
    <p role="alert">{error || snapshot.diagnostic}</p>
    <label>Bottom side <select value={bottom || snapshot.orientations[0]!.side.key} onChange={(e) => setBottom(e.target.value)}>{snapshot.orientations.map((o) => <option key={o.side.key} value={o.side.key}>{o.side.label}</option>)}</select></label>
    <NativeBoard snapshot={snapshot} bottomSide={bottom || snapshot.orientations[0]!.side.key} onPlay={(action) => { void command({ kind: "play", action }); }} />
    <button onClick={() => { void command({ kind: "resume" }); }}>Resume</button>
    <button onClick={() => { void command({ kind: "analyze", operation: { kind: "moves", depth: 1, nodes: 1000 } }); }}>Analyze selected event (depth 1 / 1000 nodes)</button>
    <button onClick={() => { void command({ kind: "cancel" }); }}>Cancel work</button>
    <button onClick={() => { void command({ kind: "return_live" }); }}>Return to live</button>
    <button onClick={() => { void command({ kind: "undo" }); }}>Undo event</button>
    <button onClick={() => { void command({ kind: "declare" }); }}>Formal declaration (invalid declaration loses)</button>
    <button onClick={() => { void command({ kind: "resign" }); }}>Resign</button>
    <ol><li><button onClick={() => { void command({ kind: "inspect", event_index: 0 }); }}>Root event</button></li>{snapshot.events.map((e) => <li key={e.index}><button onClick={() => { void command({ kind: "inspect", event_index: e.index }); }}>{e.index}: {e.label}</button></li>)}</ol>
    <details><summary>Actual analysis / declaration advice / technical details</summary><pre>{JSON.stringify({ analysis: snapshot.analysis, advice: snapshot.declaration_advice, token: snapshot.token }, null, 2)}</pre></details>
    {close && <div role="dialog" aria-modal="true" aria-label="Discard development session?">
      <p>This unsaved shogi development session has no archive support yet.</p>
      <button onClick={() => { void invoke("desktop_cancel_close").then(() => setClose(null)); }}>Keep open</button>
      <button onClick={() => { void invoke("desktop_finish_close", { expectedDocumentRevision: close.document.revision }).catch((e: unknown) => setError(String(e))); }}>Discard development session and close</button>
    </div>}
  </main>;
}
