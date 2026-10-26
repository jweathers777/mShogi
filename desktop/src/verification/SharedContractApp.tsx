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
  document: { revision: number; dirty: boolean; path: string | null; settings: { game_kind: string; settings: { orientation: string; analysis_defaults: { depth: number; nodes: number } } } };
  observations: { historical: boolean; response: unknown }[];
  io: { phase: string; message: string | null };
  capacity: { status: string; message: string | null };
}
export function SharedContractApp() {
  const [snapshot, setSnapshot] = useState<ContractSnapshot | null>(null);
  const latest = useRef<ContractSnapshot | null>(null);
  const [confirmOpen, setConfirmOpen] = useState(false);
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
  async function file(action: "save" | "save_as" | "open") {
    const current = latest.current;
    if (!current || busy.current) return null;
    busy.current = true; setError("");
    try {
      const next = await invoke<ContractSnapshot>("native_contract_file", { action, expected: { document: current.token.document, version: current.token.version, document_revision: current.document.revision } });
      accept(next); return next;
    } catch (e) { setError(String(e)); return null; } finally { busy.current = false; }
  }
  async function finishClose(saved = false) {
    const current = saved ? await file("save") : latest.current;
    if (!current || (saved && current.document.dirty)) return;
    try { await invoke("desktop_finish_close", { expectedDocumentRevision: current.document.revision }); }
    catch (e) { setError(String(e)); }
  }
  if (!snapshot) return <main><h1>MSH-73 DEVELOPMENT CONTRACT</h1><p role="status">Loading native session…</p><p role="alert">{error}</p></main>;
  return <main><h1>MSH-73 DEVELOPMENT CONTRACT</h1>
    <p>Development-only session/archive verification harness. This is not the MSH-74 shogi visual design.</p>
    <p role="status">{snapshot.status.label} · {snapshot.phase} · {snapshot.historical ? `Read-only history event ${snapshot.event_index}` : "Live game"}{snapshot.paused ? " · paused" : ""}</p>
    <p role="alert">{error || snapshot.diagnostic}</p>
    <p>{snapshot.document.dirty ? "Unsaved changes" : "Saved"} · {snapshot.document.path || "Untitled native game"} · {snapshot.io.phase}</p>
    <p role="status">{snapshot.capacity.message || snapshot.io.message}</p>
    <button onClick={() => { void file("save"); }}>Save</button>
    <button onClick={() => { void file("save_as"); }}>Save As</button>
    <button onClick={() => { if (snapshot.document.dirty) setConfirmOpen(true); else void file("open"); }}>Open</button>
    <label>Bottom side <select value={snapshot.orientations.find((o) => o.side.key.endsWith(`:${snapshot.document.settings.settings.orientation}`))?.side.key || snapshot.orientations[0]!.side.key} onChange={(e) => { void command({ kind: "shogi_preferences", settings: { ...snapshot.document.settings.settings, orientation: e.target.value.endsWith(":gote") ? "gote" : "sente" } }); }}>{snapshot.orientations.map((o) => <option key={o.side.key} value={o.side.key}>{o.side.label}</option>)}</select></label>
    <NativeBoard snapshot={snapshot} bottomSide={snapshot.orientations.find((o) => o.side.key.endsWith(`:${snapshot.document.settings.settings.orientation}`))?.side.key || snapshot.orientations[0]!.side.key} onPlay={(action) => { void command({ kind: "play", action }); }} />
    <button onClick={() => { void command({ kind: "resume" }); }}>Resume</button>
    <button onClick={() => { void command({ kind: "analyze", operation: { kind: "moves", depth: 1, nodes: 1000 } }); }}>Analyze selected event (depth 1 / 1000 nodes)</button>
    <button onClick={() => { void command({ kind: "cancel" }); }}>Cancel work</button>
    <button onClick={() => { void command({ kind: "return_live" }); }}>Return to live</button>
    <button onClick={() => { void command({ kind: "undo" }); }}>Undo event</button>
    <button onClick={() => { void command({ kind: "declare" }); }}>Formal declaration (invalid declaration loses)</button>
    <button onClick={() => { void command({ kind: "resign" }); }}>Resign</button>
    <ol><li><button onClick={() => { void command({ kind: "inspect", event_index: 0 }); }}>Root event</button></li>{snapshot.events.map((e) => <li key={e.index}><button onClick={() => { void command({ kind: "inspect", event_index: e.index }); }}>{e.index}: {e.label}</button></li>)}</ol>
    <details><summary>Actual analysis / declaration advice / technical details</summary><pre>{JSON.stringify({ analysis: snapshot.analysis, observations: snapshot.observations, advice: snapshot.declaration_advice, token: snapshot.token }, null, 2)}</pre></details>
    {confirmOpen && <div role="dialog" aria-modal="true" aria-label="Replace unsaved native document?">
      <p>Save the complete current native document before replacement, or explicitly discard unsaved changes.</p>
      <button onClick={() => setConfirmOpen(false)}>Keep current document</button>
      <button onClick={() => { void file("save").then((next) => { if (next && !next.document.dirty) { setConfirmOpen(false); void file("open"); } }); }}>Save then Open</button>
      <button onClick={() => { setConfirmOpen(false); void file("open"); }}>Discard changes then Open</button>
    </div>}
    {close && <div role="dialog" aria-modal="true" aria-label="Discard development session?">
      <p>Save the complete native document before closing, or explicitly discard unsaved changes.</p>
      <button onClick={() => { void invoke("desktop_cancel_close").then(() => setClose(null)); }}>Keep open</button>
      <button onClick={() => { void finishClose(true); }}>Save and close</button>
      <button onClick={() => { void finishClose(); }}>Discard development session and close</button>
    </div>}
  </main>;
}
