import { useRef } from "react";
import { XiangqiBoard } from "./XiangqiBoard";
import { ApplicationShell, DocumentState, InspectorSwitch, LiveGameHeading, NativeEnginePanel } from "./ApplicationShell";
import { Observation } from "./ShogiApp";
import { useMovementInspection } from "./useMovementInspection";
import type { XiangqiSnapshot as GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";

// Product board/play checkpoint. Full analysis, outcome/recovery and document/history
// journeys remain MSH-102. Existing native contracts and admission are unchanged.
export function XiangqiApp({ snapshot, session, onNewGame, onOpen, blocked }: {
  snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean;
}) {
  const onInspect = useMovementInspection({ token: snapshot.token, game: snapshot.game });
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening"].includes(snapshot.io.phase);
  const disabled = busy || blocked;
  const capacity = snapshot.capacity.status === "paused";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const lastEngine = [...snapshot.search_observations].reverse().find((view) => view.purpose === "engine_turn" && !view.historical);
  const statusRef = useRef<HTMLHeadingElement>(null);
  const orientation = snapshot.document.settings.orientation === "black" ? "black" : "red";
  const filename = snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled xiangqi game";
  return <ApplicationShell game="xiangqi" runtime={thinking ? "mShogi is thinking" : "Local engine"} historical={snapshot.historical}
    attributes={{ "data-current-event": snapshot.event_index, "data-live-events": snapshot.events.length, "data-paused": snapshot.paused, "data-orientation": orientation }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{filename}</h2>{["saving", "opening"].includes(snapshot.io.phase) && <p role="status">{snapshot.io.phase === "saving" ? "Saving your game…" : "Opening your game…"}</p>}</>}
    actions={<><button className="primary" onClick={onNewGame} disabled={disabled}>New game</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save"); }}>Save</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save_as"); }}>Save as…</button><button className="quiet" disabled={disabled} onClick={onOpen}>Open…</button></>}
    heading={<LiveGameHeading statusRef={statusRef} eyebrow={snapshot.historical ? "HISTORY · READ ONLY" : snapshot.status.concluded ? "GAME CONCLUDED" : "YOUR GAME"}
      status={thinking ? "mShogi is thinking" : snapshot.status.label} player={<>You play {snapshot.human.label} against mShogi</>}
      flip={<button className="quiet flip-button" aria-label={`Flip board; currently ${orientation === "red" ? "Red" : "Black"} at bottom`} disabled={disabled} onClick={() => {
        void session.command(snapshot, { kind: "xiangqi_preferences", settings: { analysis_defaults: snapshot.document.settings.analysis_defaults, orientation: orientation === "red" ? "black" : "red" } });
      }}>↕ <span>Flip board</span></button>} />}
    board={<XiangqiBoard snapshot={snapshot} disabled={disabled || capacity || !snapshot.capabilities.board_action}
      inspectionDisabled={disabled} onInspect={onInspect} onPlay={(action) => { void session.command(snapshot, { kind: "xiangqi", command: { kind: "play", action } }); }} />}
    notices={<>
      {session.error && <section className="shogi-notice error" role="alert"><strong>The action could not complete. Your current game has been kept.</strong><p>Details are available below.</p></section>}
      {capacity && <section className="shogi-notice" role="alert"><h3>Application limit reached</h3><p>The complete current game can still be saved. Save it before starting a new game; saving and reopening does not reset this limit.</p></section>}
      {["failed", "incomplete", "cancelled"].includes(snapshot.phase) && <section className="shogi-notice" role="status"><h3>{snapshot.phase === "failed" ? "Engine connection failed" : snapshot.phase === "incomplete" ? "Search incomplete" : "Search stopped"}</h3><p>Your game is intact. No game result is implied.</p></section>}
      {thinking && <section className="shogi-notice" role="status"><h3>mShogi is thinking</h3><p>The engine is choosing a move within its work limits.</p><button disabled={disabled} onClick={() => { void session.command(snapshot, { kind: "cancel" }); }}>Cancel search</button></section>}
    </>}
    analysis={<><InspectorSwitch value="play" onChange={() => {}} analysisDisabled /><NativeEnginePanel limits={snapshot.play_limits} status={thinking ? "Searching" : snapshot.paused ? "Play paused" : snapshot.phase}>
      {lastEngine ? <Observation view={lastEngine} /> : <p className="muted">No engine search has completed in this game.</p>}
    </NativeEnginePanel></>}
    history={<section className="shogi-history" aria-label="Game history"><div className="shogi-section-heading"><h2>Game history</h2><span>{snapshot.events.length} {snapshot.events.length === 1 ? "event" : "events"}</span></div>
      <ol><li><div className="native-history-row" aria-current={snapshot.event_index === 0 ? "step" : undefined}><span className="event-number">0</span><span>Starting position</span></div></li>{snapshot.events.map((event) => <li key={event.index}><div className="native-history-row" aria-current={snapshot.event_index === event.index ? "step" : undefined}><span className="event-number">{event.index}</span><span>{event.label}</span></div></li>)}</ol></section>}
    gameActions={<section className="shogi-game-actions"><button className="quiet" disabled title="Additional game actions are not available in this preview.">Resign game…</button></section>}
    technical={<><p>mShogi · untimed xiangqi</p><p>Game records, legality and movement inspection are owned by the native application.</p>
      <details><summary>Typeface license</summary><p>{fontNotice}</p><pre>{fontLicense}</pre></details>
      <details><summary>Native session and diagnostics</summary><pre>{JSON.stringify({ token: snapshot.token, profile: snapshot.profile, document: snapshot.document, context: snapshot.context, diagnostic: snapshot.diagnostic, error: session.error, observations: snapshot.observations }, null, 2)}</pre></details>
      <button className="quiet" disabled={disabled} onClick={() => { void import("@tauri-apps/api/core").then(({ invoke }) => invoke("desktop_request_close")); }}>Quit application</button></>}
  />;
}
