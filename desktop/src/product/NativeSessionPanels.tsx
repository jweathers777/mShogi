import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { AnalysisActions, InspectorSwitch, NativeEnginePanel } from "./ApplicationShell";
import type { GameSnapshot, Operation, SearchView } from "./native";
import { resourceExplanation } from "./resourcePresentation";
import { ChessAnalysisContent, ChessSavedEvidence } from "./ChessAnalysisContent";
import type { AnalysisOperation } from "../native";
import { colorName, conclusionName } from "../presentation";

// Only native state/facts establish capacity; command errors remain unclassified.
export function nativeCapacityReached(snapshot: GameSnapshot) {
  return snapshot.capacity.status === "paused" || Boolean(snapshot.capacity.resource);
}
export function publicSessionError(_message: string) {
  return "The action could not complete. Your current game has been kept; details are available below.";
}
export function Observation({ view, game = "shogi", rejected = false }: { view: SearchView; game?: "chess" | "shogi" | "xiangqi"; rejected?: boolean }) {
  if (rejected) return <div className="shogi-search-observation"><p>Rejected engine response · event {view.event_index}. No valid score or proof is presented.</p></div>;
  return <div className="shogi-search-observation" data-search-purpose={view.purpose}>
    <p className="search-target">{view.historical ? "Saved observation" : view.purpose === "engine_turn" ? "Last engine turn" : view.purpose === "chess_full_reachability" ? "Full cooperative reachability" : "Move analysis"} · event {view.event_index}</p>
    <p className="shogi-evaluation">{view.score ?? "No completed evaluation"}</p>
    <p className="analysis-perspective">{view.perspective} perspective · {view.status}</p>
    <dl className="shogi-telemetry"><div><dt>Completed depth</dt><dd>{view.completed_depth ?? "—"}</dd></div><div><dt>Nodes</dt><dd>{view.nodes?.toLocaleString() ?? "—"}</dd></div></dl>
    {view.variation.length > 0 && <p className="shogi-pv"><span>Continuation</span>{view.variation.join(" · ")}</p>}
    {view.purpose === "move_analysis" && <p className="analysis-readonly">Read-only analysis. No move or {game === "shogi" ? "declaration" : "claim"} was played.</p>}
  </div>;
}
export function NativeHistory({ snapshot, disabled, command }: { snapshot: GameSnapshot; disabled: boolean; command: (op: Operation) => unknown }) {
  const { root, entries } = snapshot.moves;
  const moveScroll = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    const container = moveScroll.current, selected = container?.querySelector<HTMLButtonElement>('button[aria-current="step"]');
    if (!container || !selected) return;
    const frame = container.getBoundingClientRect(), target = selected.getBoundingClientRect();
    if (target.top < frame.top) container.scrollTop -= frame.top - target.top;
    else if (target.bottom > frame.bottom) container.scrollTop += target.bottom - frame.bottom;
  }, [snapshot.event_index, entries.length]);
  const first = snapshot.sides.find(side => side.first_mover)!, second = snapshot.sides.find(side => !side.first_mover)!;
  const rows = [...new Set(entries.map(entry => entry.number))];
  const details = snapshot.events.filter(event => event.index !== 0 && !entries.some(entry => entry.event_index === event.index));
  const select = (event_index: number) => { void command({ kind: "inspect", event_index }); };
  return <section className="shogi-history moves-panel" data-current-event={snapshot.event_index} aria-label="Moves">
    <div className="shogi-section-heading"><h2>Moves</h2><span>{entries.length} {entries.length === 1 ? "move" : "moves"}</span></div>
    <button className="moves-root" data-event={0} aria-current={snapshot.event_index === 0 ? "step" : undefined} disabled={disabled} onClick={() => select(0)}>{root.label}</button>
    <p className="moves-numbering">{root.numbering}</p>
    <div className="moves-scroll" ref={moveScroll}><table className="moves-table"><thead><tr><th scope="col"><span className="sr-only">Move number</span></th><th scope="col">{first.label}</th><th scope="col">{second.label}</th></tr></thead><tbody>
      {rows.map(number => <tr key={number}><th scope="row">{number}.</th>{([0, 1] as const).map(column => {
        const entry = entries.find(candidate => candidate.number === number && candidate.column === column);
        if (!entry) return <td key={column} />;
        const associated = details.filter(event => event.index > entry.command_start && event.index <= entry.command_end);
        return <td key={column} data-associated-selection={associated.some(event => event.index === snapshot.event_index) || undefined}>
          <button data-event={entry.event_index} data-native-action={entry.action} aria-current={snapshot.event_index === entry.event_index ? "step" : undefined} disabled={disabled} onClick={() => select(entry.event_index)} aria-label={`${number}. ${column === 0 ? first.label : second.label}: ${entry.notation}`}>{entry.notation}</button>
          {associated.length > 0 && <span className="move-event-count" title={associated.map(event => event.label).join(" · ")} aria-label={`${associated.length} associated position ${associated.length === 1 ? "event" : "events"}; available in Position events`}>*</span>}
        </td>;
      })}</tr>)}
    </tbody></table>{entries.length === 0 && <p className="moves-empty">No moves in known history.</p>}</div>
    {details.length > 0 && <details className="moves-events" open={details.some(event => event.index === snapshot.event_index) || undefined}><summary>Position events ({details.length})</summary><ol>{details.map(event => {
      const entry = entries.find(candidate => event.index > candidate.command_start && event.index <= candidate.command_end);
      return <li key={event.index}><button data-event={event.index} aria-current={snapshot.event_index === event.index ? "step" : undefined} disabled={disabled} onClick={() => select(event.index)}><span className="event-number">{event.index}</span><span>{entry && <small>After {entry.number}. {entry.notation} · </small>}{event.label}</span></button></li>;
    })}</ol></details>}
  </section>;
}
export function NativeRecovery({ snapshot, error, disabled, command, pending = false }: { snapshot: GameSnapshot; error: string; disabled: boolean; command: (op: Operation) => unknown; pending?: boolean }) {
  const capacity = nativeCapacityReached(snapshot);
  const adjudicationPending = pending || (snapshot.game === "xiangqi" && snapshot.xiangqi.pending !== null);
  const explanation = resourceExplanation(snapshot.capacity.resource);
  const capacityNotice = useRef<HTMLElement>(null);
  useLayoutEffect(() => { if (capacity) capacityNotice.current?.focus(); }, [snapshot.capacity.id, capacity]);
  const analyzing = snapshot.game === "chess" ? snapshot.chess.analysis.status === "thinking" : snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const fileFailed = snapshot.io.phase === "failed";
  const failure = fileFailed || (error && !capacity);
  const conclusion = snapshot.game === "chess" ? snapshot.chess?.selected_conclusion : null;
  const fileResource = fileFailed ? snapshot.io.resource : null;
  return <>
    {snapshot.status.concluded && <section className="shogi-notice outcome-card" aria-label="Game outcome"><h3>{conclusion ? conclusionName(conclusion) : snapshot.status.label}</h3><p>{conclusion && "unresolved" in conclusion ? `${colorName(conclusion.unresolved.actor)} ${conclusion.unresolved.trigger === "resignation" ? "resigned" : "ran out of time"}. The M2 profile could not establish whether the opponent can mate. Play has stopped without an established win or draw, and no score is assigned.` : "Play has ended. The native outcome is separate from engine process status and evaluation."}</p></section>}
    {snapshot.game === "xiangqi" && snapshot.xiangqi.pending && <section className="shogi-notice" role="status"><h3>Adjudication unresolved</h3><p>Play is stopped. This game is unscored; no win, loss or draw has been established.</p></section>}
    {snapshot.game === "xiangqi" && !snapshot.status.concluded && snapshot.xiangqi.warnings.filter((warning) => warning.active).map((warning) => <section className="shogi-notice" role="status" key={warning.event_index}><h3>Repetition warning</h3><p>{warning.side === "red" ? "Red" : "Black"} must vary {warning.fault === "perpetual_check" ? "perpetual check" : "perpetual chase"}. Continuing the prohibited repetition can lose the game.</p></section>)}
    {snapshot.game === "xiangqi" && !snapshot.status.concluded && snapshot.xiangqi.offered_by !== null && <section className="shogi-notice" role="status"><h3>Draw offered by {snapshot.xiangqi.offered_by === "red" ? "Red" : "Black"}</h3><p>The offer has not concluded the game.</p></section>}
    {failure && <section className="shogi-notice error" role="alert"><strong>{fileResource ? "Application limit reached" : fileFailed ? "The file operation could not complete. Your current game and unsaved changes have been kept." : publicSessionError(error)}</strong>{fileResource && <p>{resourceExplanation(fileResource)}</p>}</section>}
    {snapshot.capacity.status === "checking" && <section className="shogi-notice" role="status"><strong>Checking complete save capacity</strong><p>The native host is checking this operation before adding it to the saved document.</p></section>}
    {capacity && <section className="shogi-notice capacity-notice" role="alert" aria-label="Document capacity" ref={capacityNotice} tabIndex={-1}><h3>Application limit reached</h3><p>{explanation} Save or Save as to keep the complete current game, then start a new game. Saving and reopening does not reset this limit.</p><p>This is an application resource limit. No game result or illegal action is implied. Your complete accepted game and observations are preserved.</p></section>}
    {["failed", "incomplete", "cancelled"].includes(snapshot.phase) && !capacity && <section className="shogi-notice" role="status"><h3>{snapshot.phase === "failed" ? "Engine connection failed" : snapshot.phase === "incomplete" ? "Search incomplete" : "Search stopped"}</h3><p>{snapshot.phase === "failed" ? "The current game is intact. No game result is implied." : `No uncompleted move or ${snapshot.game === "shogi" ? "declaration" : "claim"} was played.`}</p>{!snapshot.historical && !snapshot.status.concluded && !adjudicationPending && !(snapshot.game === "chess" && snapshot.chess.analysis.status !== "idle") && snapshot.execution_purpose !== "move_analysis" && snapshot.execution_purpose !== "chess_full_reachability" && snapshot.turn.key !== snapshot.human.key && <button data-focus-after="engine" disabled={disabled} onClick={() => { void command({ kind: "retry" }); }}>Retry engine turn</button>}</section>}
    {snapshot.paused && !snapshot.historical && (!snapshot.status.concluded || snapshot.game === "chess") && (!analyzing || snapshot.game === "chess") && !capacity && !adjudicationPending && <section className="shogi-resume play-paused"><p role="status">{snapshot.status.concluded ? "The game remains concluded. Finishing analysis does not reopen it." : snapshot.diagnostic === "Position restored. Resume play when ready." ? snapshot.diagnostic : "Play is paused."}</p><button data-focus-after="board" className="primary" disabled={disabled} onClick={() => { void command({ kind: "resume" }); }}>{snapshot.status.concluded ? "Finish analysis" : "Resume play"}</button></section>}
    {thinking && <section className="shogi-notice" role="status"><h3>mShogi is thinking</h3><p>The engine is choosing a move within its work limits.</p><button data-focus-after="engine" disabled={disabled} onClick={() => { void command({ kind: "cancel" }); }}>Cancel search</button></section>}
  </>;
}
export function NativeAnalysis({ snapshot, defaults, disabled, command, maxDepth, maxNodes, minNodes, unavailable }: {
  snapshot: GameSnapshot; defaults: AnalysisOperation | { depth: number; nodes: number }; disabled: boolean; command: (op: Operation) => unknown;
  maxDepth: number; maxNodes: number; minNodes: number; unavailable?: string | undefined;
}) {
  const movesDefaults = "depth" in defaults ? defaults : { depth: 4, nodes: 100000 };
  const [validation, setValidation] = useState("");
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const [depth, setDepth] = useState(String(movesDefaults.depth)), [nodes, setNodes] = useState(String(movesDefaults.nodes));
  useEffect(() => { setDepth(String(movesDefaults.depth)); setNodes(String(movesDefaults.nodes)); }, [snapshot.token.document]);
  const analyzing = snapshot.game === "chess" ? snapshot.chess.analysis.status === "thinking" : snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const current = snapshot.search_observations.find((v) => v.selected);
  const lastEngine = [...snapshot.search_observations].reverse().find((v) => v.purpose === "engine_turn" && !v.historical);
  const game = snapshot.game;
  return <><InspectorSwitch value={inspector} onChange={setInspector} /><section className="shogi-analysis" aria-label="Engine analysis" data-phase={analyzing ? "analyzing" : snapshot.phase}>
    {inspector === "analysis" ? <><div className="shogi-section-heading"><strong>Read-only analysis</strong></div><p className="analysis-location">Selected event {snapshot.event_index} · {snapshot.turn.label} perspective</p>
      <form noValidate onSubmit={(e) => { e.preventDefault(); if (!e.currentTarget.checkValidity()) { setValidation(`Move analysis requires a whole-number depth from 1 to ${maxDepth} and a node budget from ${minNodes.toLocaleString()} to ${maxNodes.toLocaleString()}.`); return; } setValidation(""); if (!disabled && snapshot.capacity.status !== "paused" && snapshot.capabilities.move_analysis) void command({ kind: "analyze", operation: { kind: "moves", depth: Number(depth), nodes: Number(nodes) } }); }}><div className="shogi-analysis-fields"><label>Depth<input name="depth" type="number" min="1" max={maxDepth} step="1" value={depth} onChange={(e) => setDepth(e.target.value)} required /></label><label>Node limit<input name="nodes" type="number" min={minNodes} max={maxNodes} step="1" value={nodes} onChange={(e) => setNodes(e.target.value)} required /></label></div><AnalysisActions><button className="secondary" data-analysis-start={game} disabled={disabled || snapshot.capacity.status === "paused" || !snapshot.capabilities.move_analysis}>{analyzing ? "Replace analysis" : "Start move analysis"}</button>{analyzing && <button className="quiet" type="button" onClick={() => { void command({ kind: "cancel" }); }} disabled={disabled}>Cancel analysis</button>}</AnalysisActions></form>
      {validation && <p role="alert" className="validation-error">{validation}</p>}
      <p className="shogi-analysis-state" role="status" aria-live="polite">{unavailable ?? (analyzing ? "Analyzing the selected position…" : snapshot.phase === "cancelled" ? "Search stopped" : current ? "Analysis result available" : "Choose Analyze to inspect this position.")}</p>
      {snapshot.game === "chess" ? <ChessAnalysisContent snapshot={snapshot} disabled={disabled || snapshot.capacity.status === "paused" || !snapshot.capabilities.chess_full} command={command} defaults={"kind" in defaults ? defaults : { kind: "moves", ...defaults }} /> : current && <Observation view={current} game={game} rejected={Boolean(snapshot.observations[current.observation_index]?.application_error)} />}
      {snapshot.game === "chess" && snapshot.observations.length > 0 && <details className="recorded-chess-evidence"><summary>Recorded native evidence ({snapshot.observations.length})</summary>{snapshot.observations.map((entry, index) => <article key={index}><ChessSavedEvidence observation={entry.observation} rejected={entry.application_error !== null} /></article>)}</details>}
      {snapshot.search_observations.length > 0 && <details className="recorded-searches"><summary>Recorded searches ({snapshot.search_observations.length})</summary>{snapshot.search_observations.map((v) => <article key={v.observation_index}><Observation view={v} game={game} rejected={Boolean(snapshot.observations[v.observation_index]?.application_error)} /></article>)}</details>}
    </> : <NativeEnginePanel limits={snapshot.play_limits} status={thinking ? "Searching" : snapshot.paused ? "Play paused" : snapshot.phase}>{lastEngine ? <Observation view={lastEngine} game={game} rejected={Boolean(snapshot.observations[lastEngine.observation_index]?.application_error)} /> : <p className="muted">No engine search has completed in this game.</p>}</NativeEnginePanel>}
  </section></>;
}
