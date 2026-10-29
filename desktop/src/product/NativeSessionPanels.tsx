import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { AnalysisActions, InspectorSwitch, NativeEnginePanel } from "./ApplicationShell";
import type { GameSnapshot, Operation, SearchView } from "./native";
import type { GameSession } from "./useGameSession";

// Shared interaction language only. Scores, targets, choices and admission stay native.
export function nativeCapacityReached(snapshot: GameSnapshot, error: string) {
  return snapshot.capacity.status === "paused" || /application capacity/i.test(error);
}
// Retained observations never shrink in one document. Remember that native
// structural refusal across Save; a fresh Open/New document gets a fresh key.
export function useNativeCapacityError(snapshot: GameSnapshot, error: string) {
  const [retained, setRetained] = useState<{ document: number; error: string } | null>(null);
  useEffect(() => {
    if (/application capacity:.*(?:retained observations|observation retention)/i.test(error))
      setRetained({ document: snapshot.token.document, error });
  }, [snapshot.token.document, error]);
  return /application capacity/i.test(error) ? error : retained?.document === snapshot.token.document ? retained.error : "";
}
export function publicSessionError(message: string) {
  if (/capacity|limit/i.test(message)) return "An application limit was reached. Your current game is retained. Save it before starting another game.";
  if (/stale|changed|superseded/i.test(message)) return "The game changed. Review the current position and try the action again.";
  if (/unsupported|wrong game|profile|version/i.test(message)) return "This game or saved mode is not supported. Your current game has been kept.";
  if (/save|file|open|archive|corrupt|metadata/i.test(message)) return "The file operation could not complete. Your current game and unsaved changes have been kept.";
  return "The action could not complete. Your current game has been kept; details are available below.";
}
export function Observation({ view, game = "shogi" }: { view: SearchView; game?: "shogi" | "xiangqi" }) {
  return <div className="shogi-search-observation" data-search-purpose={view.purpose}>
    <p className="search-target">{view.historical ? "Saved observation" : view.purpose === "engine_turn" ? "Last engine turn" : "Move analysis"} · event {view.event_index}</p>
    <p className="shogi-evaluation">{view.score ?? "No completed evaluation"}</p>
    <p className="analysis-perspective">{view.perspective} perspective · {view.status}</p>
    <dl className="shogi-telemetry"><div><dt>Completed depth</dt><dd>{view.completed_depth ?? "—"}</dd></div><div><dt>Nodes</dt><dd>{view.nodes?.toLocaleString() ?? "—"}</dd></div></dl>
    {view.variation.length > 0 && <p className="shogi-pv"><span>Continuation</span>{view.variation.join(" · ")}</p>}
    {view.purpose === "move_analysis" && <p className="analysis-readonly">Read-only analysis. No move or {game === "shogi" ? "declaration" : "claim"} was played.</p>}
  </div>;
}
export function useNativeWorkCommand(snapshot: GameSnapshot, session: GameSession) {
  const statusRef = useRef<HTMLHeadingElement>(null), restore = useRef(false);
  useLayoutEffect(() => { if (restore.current) { restore.current = false; statusRef.current?.focus(); } }, [snapshot.token.revision]);
  const command = async (operation: Operation) => {
    if (["return_live", "resume", "retry", "cancel"].includes(operation.kind)) restore.current = true;
    const next = await session.command(snapshot, operation);
    if (!next) restore.current = false;
    return next;
  };
  return { statusRef, command };
}
export function NativeHistory({ snapshot, disabled, command }: { snapshot: GameSnapshot; disabled: boolean; command: (op: Operation) => unknown }) {
  return <section className="shogi-history" data-current-event={snapshot.event_index} aria-label="Game history"><div className="shogi-section-heading"><h2>Game history</h2><span>{snapshot.events.length} {snapshot.events.length === 1 ? "event" : "events"}</span></div><ol>{[{ index: 0, label: "Starting position" }, ...snapshot.events].map((event) => <li key={event.index}><button data-event={event.index} aria-current={snapshot.event_index === event.index ? "step" : undefined} disabled={disabled} onClick={() => { void command({ kind: "inspect", event_index: event.index }); }}><span className="event-number">{event.index}</span><span>{event.label}</span></button></li>)}</ol></section>;
}
export function NativeRecovery({ snapshot, error, disabled, command, pending = false, capacityError = "" }: { snapshot: GameSnapshot; error: string; disabled: boolean; command: (op: Operation) => unknown; pending?: boolean; capacityError?: string }) {
  const capacity = nativeCapacityReached(snapshot, capacityError || error), message = snapshot.capacity.message ?? (capacityError || error);
  const observationCount = snapshot.observations.length + (snapshot.game === "xiangqi" ? snapshot.xiangqi.exact_observations.length : 0);
  // Only these native preflight reasons identify observation retention alone.
  // Archive diagnostics may mention several bounds; their numbers cannot tell
  // us which resource refused the candidate. Preserve those details below.
  const observationStorage = message.startsWith("Application capacity: retained observations are at their limit;")
    || message.startsWith("Application capacity: native consumer observation retention limit reached;")
    || message.startsWith("Application capacity: All256 observation slots are retained;")
    || message.startsWith("Application capacity: all256 observations are retained.");
  const explanation = observationStorage && observationCount === 256 ? "This game has reached the limit of 256 recorded engine observations."
    : observationStorage ? "This game has reached its recorded-observation storage limit."
    : "Your current game can still be saved.";
  const analyzing = snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const failure = error || (snapshot.io.phase === "failed" ? snapshot.io.message ?? "File operation failed" : "");
  return <>
    {failure && !(capacity && /application capacity/i.test(failure)) && <section className="shogi-notice error" role="alert"><strong>{publicSessionError(failure)}</strong></section>}
    {capacity && <section className="shogi-notice capacity-notice" role="alert"><h3>Application limit reached</h3><p>{explanation} Save the complete current game, then start a new game. Saving and reopening does not reset this limit.</p></section>}
    {["failed", "incomplete", "cancelled"].includes(snapshot.phase) && !capacity && <section className="shogi-notice" role="status"><h3>{snapshot.phase === "failed" ? "Engine connection failed" : snapshot.phase === "incomplete" ? "Search incomplete" : "Search stopped"}</h3><p>{snapshot.phase === "failed" ? "The current game is intact. No game result is implied." : `No uncompleted move or ${snapshot.game === "xiangqi" ? "claim" : "declaration"} was played.`}</p>{!snapshot.historical && !snapshot.status.concluded && !pending && snapshot.turn.key !== snapshot.human.key && <button disabled={disabled} onClick={() => { void command({ kind: "retry" }); }}>Retry engine turn</button>}</section>}
    {snapshot.paused && !snapshot.historical && !snapshot.status.concluded && !analyzing && !capacity && !pending && <section className="shogi-resume play-paused"><p role="status">{snapshot.diagnostic === "Position restored. Resume play when ready." ? snapshot.diagnostic : "Play is paused."}</p><button className="primary" disabled={disabled} onClick={() => { void command({ kind: "resume" }); }}>Resume play</button></section>}
    {thinking && <section className="shogi-notice" role="status"><h3>mShogi is thinking</h3><p>The engine is choosing a move within its work limits.</p><button disabled={disabled} onClick={() => { void command({ kind: "cancel" }); }}>Cancel search</button></section>}
  </>;
}
export function NativeAnalysis({ snapshot, defaults, disabled, command, maxDepth, maxNodes, minNodes, unavailable }: {
  snapshot: GameSnapshot; defaults: { depth: number; nodes: number }; disabled: boolean; command: (op: Operation) => unknown;
  maxDepth: number; maxNodes: number; minNodes: number; unavailable?: string | undefined;
}) {
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const [depth, setDepth] = useState(String(defaults.depth)), [nodes, setNodes] = useState(String(defaults.nodes));
  useEffect(() => { setDepth(String(defaults.depth)); setNodes(String(defaults.nodes)); }, [snapshot.token.document]);
  const analyzing = snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const current = snapshot.search_observations.find((v) => v.selected);
  const lastEngine = [...snapshot.search_observations].reverse().find((v) => v.purpose === "engine_turn" && !v.historical);
  const game = snapshot.game === "xiangqi" ? "xiangqi" : "shogi";
  return <><InspectorSwitch value={inspector} onChange={setInspector} /><section className="shogi-analysis" aria-label="Engine analysis" data-phase={analyzing ? "analyzing" : snapshot.phase}>
    {inspector === "analysis" ? <><div className="shogi-section-heading"><strong>Read-only analysis</strong></div><p className="analysis-location">Selected event {snapshot.event_index} · {snapshot.turn.label} perspective</p>
      <form onSubmit={(e) => { e.preventDefault(); if (e.currentTarget.checkValidity() && !disabled && snapshot.capacity.status !== "paused" && snapshot.capabilities.move_analysis) void command({ kind: "analyze", operation: { kind: "moves", depth: Number(depth), nodes: Number(nodes) } }); }}><div className="shogi-analysis-fields"><label>Depth<input name="depth" type="number" min="1" max={maxDepth} step="1" value={depth} onChange={(e) => setDepth(e.target.value)} required /></label><label>Node limit<input name="nodes" type="number" min={minNodes} max={maxNodes} step="1" value={nodes} onChange={(e) => setNodes(e.target.value)} required /></label></div><AnalysisActions><button className="secondary" data-analysis-start={game} disabled={disabled || snapshot.capacity.status === "paused" || !snapshot.capabilities.move_analysis}>{analyzing ? "Replace analysis" : "Start move analysis"}</button>{analyzing && <button className="quiet" type="button" onClick={() => { void command({ kind: "cancel" }); }} disabled={disabled}>Cancel analysis</button>}</AnalysisActions></form>
      <p className="shogi-analysis-state" role="status" aria-live="polite">{unavailable ?? (analyzing ? "Analyzing the selected position…" : snapshot.phase === "cancelled" ? "Search stopped" : current ? "Analysis result available" : "Choose Analyze to inspect this position.")}</p>
      {current && <Observation view={current} game={game} />}
      {snapshot.search_observations.length > 0 && <details className="recorded-searches"><summary>Recorded searches ({snapshot.search_observations.length})</summary>{snapshot.search_observations.map((v) => <article key={v.observation_index}><Observation view={v} game={game} /></article>)}</details>}
    </> : <NativeEnginePanel limits={snapshot.play_limits} status={thinking ? "Searching" : snapshot.paused ? "Play paused" : snapshot.phase}>{lastEngine ? <Observation view={lastEngine} game={game} /> : <p className="muted">No engine search has completed in this game.</p>}</NativeEnginePanel>}
  </section></>;
}
