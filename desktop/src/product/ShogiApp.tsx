import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { ShogiBoard } from "./ShogiBoard";
import { AnalysisActions, ApplicationShell, DocumentState, HistoricalNotice, InspectorSwitch, LiveGameHeading, NativeEnginePanel } from "./ApplicationShell";
import { useMovementInspection } from "./useMovementInspection";
import { Dialog } from "./Dialog";
import type { ChessShogiSnapshot as GameSnapshot, Operation, SearchView } from "./native";
import type { GameSession } from "./useGameSession";
import licenseUrl from "../assets/fonts/OFL.txt?url";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";
function publicError(message: string) {
  if (/capacity|limit/i.test(message)) return "An application limit was reached. Your current game is retained. Save it before starting another game.";
  if (/stale|changed|superseded/i.test(message)) return "The game changed. Review the current position and try the action again.";
  if (/unsupported|wrong game|profile|version/i.test(message)) return "This game or saved mode is not supported. Your current game has been kept.";
  if (/save|file|open|archive|corrupt|metadata/i.test(message)) return "The file operation could not complete. Your current game and unsaved changes have been kept.";
  return "The action could not complete. Your current game has been kept; details are available below.";
}
export function Observation({ view }: { view: SearchView }) {
  return <div className="shogi-search-observation" data-search-purpose={view.purpose}>
    <p className="search-target">{view.historical ? "Saved observation" : view.purpose === "engine_turn" ? "Last engine turn" : "Move analysis"} · event {view.event_index}</p>
    <p className="shogi-evaluation">{view.score ?? "No completed evaluation"}</p>
    <p className="analysis-perspective">{view.perspective} perspective · {view.status}</p>
    <dl className="shogi-telemetry"><div><dt>Completed depth</dt><dd>{view.completed_depth ?? "—"}</dd></div><div><dt>Nodes</dt><dd>{view.nodes?.toLocaleString() ?? "—"}</dd></div></dl>
    {view.variation.length > 0 && <p className="shogi-pv"><span>Continuation</span>{view.variation.join(" · ")}</p>}
    {view.purpose === "move_analysis" && <p className="analysis-readonly">Read-only analysis. No move or declaration was played.</p>}
  </div>;
}
export function ShogiApp({ snapshot, session, onNewGame, onOpen, blocked }: { snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean }) {
  const onInspect = useMovementInspection({ token: snapshot.token, game: snapshot.game });
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening"].includes(snapshot.io.phase);
  const disabled = busy || blocked;
  const capacity = snapshot.capacity.status === "paused";
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const [depth, setDepth] = useState(String(snapshot.document.settings.settings.analysis_defaults.depth));
  const [nodes, setNodes] = useState(String(snapshot.document.settings.settings.analysis_defaults.nodes));
  const [confirmation, setConfirmation] = useState<{ source: GameSnapshot; kind: "resign" | "declare" } | null>(null);
  const boardStatus = useRef<HTMLHeadingElement>(null);
  const restoreWorkFocus = useRef(false);
  useLayoutEffect(() => {
    if (restoreWorkFocus.current) { restoreWorkFocus.current = false; boardStatus.current?.focus(); }
  }, [snapshot.token.revision]);
  useEffect(() => { setDepth(String(snapshot.document.settings.settings.analysis_defaults.depth)); setNodes(String(snapshot.document.settings.settings.analysis_defaults.nodes)); }, [snapshot.token.document]);
  const analyzing = snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const status = snapshot.historical ? `Viewing event ${snapshot.event_index}` : snapshot.status.concluded ? snapshot.status.label : thinking ? "mShogi is thinking" : snapshot.status.label;
  const currentAnalysis = snapshot.search_observations.find((v) => v.selected);
  const lastEngine = [...snapshot.search_observations].reverse().find((v) => v.purpose === "engine_turn" && !v.historical);
  const view = currentAnalysis ?? (snapshot.historical ? undefined : lastEngine);
  const stopped = snapshot.phase === "cancelled" || snapshot.phase === "incomplete" || snapshot.phase === "failed";
  const filename = snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled shogi game";
  const command = async (operation: Operation) => {
    if (["return_live", "resume", "retry", "cancel"].includes(operation.kind)) restoreWorkFocus.current = true;
    const next = await session.command(snapshot, operation);
    if (!next) restoreWorkFocus.current = false;
    return next;
  };
  const capacityMessage = snapshot.capacity.message ?? "";
  const limitExplanation = /256/.test(capacityMessage) ? "This game has reached the limit of 256 recorded engine observations."
    : /16MiB/.test(capacityMessage) ? "This game's complete record has reached the 16 MiB storage limit."
    : /replay/.test(capacityMessage) ? "This game has reached the application's recorded-history work limit."
    : /8192/.test(capacityMessage) ? "This game has reached the recorded-operation limit." : "This session has reached an application resource limit.";
  return <ApplicationShell game="shogi" runtime={thinking ? "mShogi is thinking" : analyzing ? "Analyzing selected position" : "Local engine"} historical={snapshot.historical} blocked={confirmation !== null}
    attributes={{ "data-current-event": snapshot.event_index, "data-live-events": snapshot.events.length, "data-paused": snapshot.paused, "data-orientation": snapshot.document.settings.settings.orientation }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{filename}</h2>{["saving", "opening"].includes(snapshot.io.phase) && <p role="status">{snapshot.io.phase === "saving" ? "Saving your game…" : "Opening your game…"}</p>}</>}
    actions={<><button className="primary" onClick={onNewGame} disabled={disabled}>New game</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save"); }}>Save</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save_as"); }}>Save as…</button><button className="quiet" disabled={disabled} onClick={onOpen}>Open…</button></>}
    heading={<LiveGameHeading statusRef={boardStatus} eyebrow={snapshot.historical ? "HISTORY · READ ONLY" : snapshot.status.concluded ? "GAME CONCLUDED" : "YOUR GAME"} status={status} player={<>You play {snapshot.human.label} against mShogi</>} flip={<><button className="quiet shogi-flip flip-button" aria-label={`Flip board; currently ${snapshot.document.settings.settings.orientation === "sente" ? "Sente" : "Gote"} at bottom`} disabled={disabled} onClick={() => { void command({ kind: "shogi_preferences", settings: { ...snapshot.document.settings.settings, orientation: snapshot.document.settings.settings.orientation === "sente" ? "gote" : "sente" } }); }}>↕ <span>Flip board</span></button></>} />}
    positionNotice={<>        {snapshot.historical && <HistoricalNotice className="shogi-history-banner" action={<button onClick={() => { void command({ kind: "return_live" }); }} disabled={disabled}>Return to live</button>}><p>Viewing event {snapshot.event_index} · the live game is unchanged</p></HistoricalNotice>}
</>}
    board={<>
        <ShogiBoard snapshot={snapshot} disabled={disabled || capacity || !snapshot.capabilities.board_action || confirmation !== null} onInspect={onInspect} inspectionDisabled={disabled || confirmation !== null} inspectionContextKey={JSON.stringify(snapshot.token)} onPlay={(action) => { void command({ kind: "play", action }); }} />
</>}
    notices={<>        {session.error && <section className="shogi-notice error" role="alert"><strong>{publicError(session.error)}</strong></section>}
        {capacity && <section className="shogi-notice capacity-notice" role="alert"><h3>Application limit reached</h3><p>{limitExplanation} Save the complete current game, then start a new game. Saving and reopening does not reset this limit.</p></section>}
        {stopped && !capacity && <section className="shogi-notice" role="status"><h3>{snapshot.phase === "failed" ? "Engine connection failed" : snapshot.phase === "incomplete" ? "Search incomplete" : "Search stopped"}</h3><p>{snapshot.phase === "failed" ? "The current game is intact. No game result is implied." : "No uncompleted move or declaration was played."}</p>{!snapshot.historical && !snapshot.status.concluded && snapshot.turn.key !== snapshot.human.key && <button disabled={disabled} onClick={() => { void command({ kind: "retry" }); }}>Retry engine turn</button>}</section>}
        {snapshot.paused && !snapshot.historical && !snapshot.status.concluded && !analyzing && !capacity && <section className="shogi-resume play-paused"><p>Play is paused.</p><button className="primary" disabled={disabled} onClick={() => { void command({ kind: "resume" }); }}>Resume play</button></section>}
        {thinking && <section className="shogi-notice" role="status"><h3>mShogi is thinking</h3><p>The engine is choosing a move within its work limits.</p><button disabled={disabled} onClick={() => { void command({ kind: "cancel" }); }}>Cancel search</button></section>}
</>}
    analysis={<><InspectorSwitch value={inspector} onChange={setInspector} />        <section className="shogi-analysis" aria-label="Engine analysis" data-phase={analyzing ? "analyzing" : snapshot.phase}>{inspector === "analysis" ? <><div className="shogi-section-heading"><strong>Read-only analysis</strong></div><p className="analysis-location">Selected event {snapshot.event_index} · {snapshot.turn.label} perspective</p>
          <form onSubmit={(e) => { e.preventDefault(); if (e.currentTarget.checkValidity()) void command({ kind: "analyze", operation: { kind: "moves", depth: Number(depth), nodes: Number(nodes) } }); }}><div className="shogi-analysis-fields"><label>Depth<input name="depth" type="number" min="1" max="4" step="1" value={depth} onChange={(e) => setDepth(e.target.value)} required /></label><label>Node limit<input name="nodes" type="number" min="0" max="20000" step="1" value={nodes} onChange={(e) => setNodes(e.target.value)} required /></label></div><AnalysisActions><button className="secondary" data-analysis-start="shogi" disabled={disabled || capacity || !snapshot.capabilities.move_analysis}>{analyzing ? "Replace analysis" : "Start move analysis"}</button>{analyzing && <button className="quiet" type="button" onClick={() => { void command({ kind: "cancel" }); }} disabled={disabled}>Cancel analysis</button>}</AnalysisActions></form>
          <p className="shogi-analysis-state" role="status" aria-live="polite">{analyzing ? "Analyzing the selected position…" : snapshot.phase === "cancelled" ? "Search stopped" : currentAnalysis ? "Analysis result available" : "Choose Analyze to inspect this position."}</p>
          {view && <Observation view={view} />}
          {snapshot.search_observations.length > 0 && <details className="recorded-searches"><summary>Recorded searches ({snapshot.search_observations.length})</summary>{snapshot.search_observations.map((v) => <article key={v.observation_index}><Observation view={v} /></article>)}</details>}
</> : <NativeEnginePanel limits={snapshot.play_limits} status={thinking ? "Searching" : snapshot.paused ? "Play paused" : snapshot.phase}>{lastEngine ? <Observation view={lastEngine} /> : <p className="muted">No engine search has completed in this game.</p>}</NativeEnginePanel>}
        </section>
</>}
    history={<>        <section className="shogi-history" data-current-event={snapshot.event_index} aria-label="Game history"><div className="shogi-section-heading"><h2>Game history</h2><span>{snapshot.events.length} {snapshot.events.length === 1 ? "event" : "events"}</span></div><ol><li><button data-event="0" aria-current={snapshot.event_index === 0 ? "step" : undefined} disabled={disabled} onClick={() => { void command({ kind: "inspect", event_index: 0 }); }}><span className="event-number">0</span><span>Starting position</span></button></li>{snapshot.events.map((event) => <li key={event.index}><button data-event={event.index} aria-current={snapshot.event_index === event.index ? "step" : undefined} disabled={disabled} onClick={() => { void command({ kind: "inspect", event_index: event.index }); }}><span className="event-number">{event.index}</span><span>{event.label}</span></button></li>)}</ol></section>
</>}
    gameActions={<>        <section className="shogi-game-actions">{snapshot.capabilities.declaration && <><p>Entering-king prerequisites are satisfied. Eligibility alone does not conclude the game.</p><button className="secondary" disabled={disabled || capacity} onClick={() => setConfirmation({ source: snapshot, kind: "declare" })}>Declare entering king…</button></>}{snapshot.capabilities.resign && <button className="quiet" disabled={disabled || capacity} onClick={() => setConfirmation({ source: snapshot, kind: "resign" })}>Resign game…</button>}</section>
</>}
    technical={<><p>mShogi · untimed standard shogi</p><p>Game records and legality are owned by the native application.</p><details><summary>Typeface license</summary><p>{fontNotice}</p><a href={licenseUrl} download="mshogi-pieces-OFL.txt">Save the font license</a><pre>{fontLicense}</pre></details><details><summary>Native session and diagnostics</summary><pre>{JSON.stringify({ token: snapshot.token, profile: snapshot.profile, document: snapshot.document, diagnostic: snapshot.diagnostic, error: session.error, analysis: snapshot.analysis, observations: snapshot.observations }, null, 2)}</pre></details><button className="quiet" disabled={disabled} onClick={() => { void import("@tauri-apps/api/core").then(({ invoke }) => invoke("desktop_request_close")); }}>Quit application</button></>}
    overlays={<>    {confirmation && <Dialog title={confirmation.kind === "resign" ? "Resign this game?" : "Declare entering king?"} cancel={() => setConfirmation(null)}><p>{confirmation.kind === "resign" ? `${snapshot.human.label} will resign this game.` : "This formally submits the declaration. Eligibility advice by itself has not ended the game. An invalid declaration loses under the selected rules."}</p><div className="dialog-actions"><button data-cancel className="quiet" onClick={() => setConfirmation(null)}>Cancel</button><button className={confirmation.kind === "resign" ? "danger" : "primary"} disabled={busy || confirmation.source.token.document !== snapshot.token.document || confirmation.source.token.version !== snapshot.token.version || confirmation.source.token.selection !== snapshot.token.selection || confirmation.source.token.revision !== snapshot.token.revision} onClick={() => { const c = confirmation; setConfirmation(null); void session.command(c.source, { kind: c.kind }); }}>{confirmation.kind === "resign" ? "Resign game" : "Confirm declaration"}</button></div></Dialog>}</>}
  />;
}
