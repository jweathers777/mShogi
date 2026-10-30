import { useEffect, useState } from "react";
import { ApplicationShell, DocumentState, HistoricalNotice, LiveGameHeading } from "./ApplicationShell";
import { GameBoard } from "./GameBoard";
import { GameActions, type Confirmation } from "./GameCapabilities";
import { NativeAnalysis, NativeHistory, NativeRecovery, nativeCapacityReached } from "./NativeSessionPanels";
import { Dialog } from "./Dialog";
import { useApplicationFocus } from "./useApplicationFocus";
import type { GameSnapshot, Operation } from "./native";
import type { GameSession } from "./useGameSession";
import { conclusionName } from "../presentation";
import licenseUrl from "../assets/fonts/OFL.txt?url";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";
import "./shogi.css";
import "./xiangqi.css";
export function ApplicationView({ snapshot, session, onNewGame, onOpen, blocked }: {
  snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean;
}) {
  const [confirmation, setConfirmation] = useState<Confirmation | null>(null);
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening", "viewing"].includes(snapshot.io.phase);
  const disabled = busy || blocked || confirmation !== null;
  const focus = useApplicationFocus(blocked || confirmation !== null);
  useEffect(() => { if (blocked) setConfirmation(null); }, [blocked]);
  const command = (operation: Operation) => session.command(snapshot, operation);
  const capacity = nativeCapacityReached(snapshot);
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const analyzing = snapshot.execution_purpose === "move_analysis" || snapshot.game === "chess" && snapshot.chess.analysis.status === "thinking";
  const orientation = snapshot.game === "xiangqi" ? snapshot.document.settings.orientation : snapshot.document.settings.settings.orientation;
  const side = snapshot.sides.find(s => s.id.side === orientation)!;
  function flip() {
    if (snapshot.game === "chess") return command({ kind: "chess_preferences", orientation: orientation === "white" ? "black" : "white", analysis_defaults: snapshot.document.settings.settings.analysis_defaults });
    if (snapshot.game === "shogi") return command({ kind: "shogi_preferences", settings: { ...snapshot.document.settings.settings, orientation: orientation === "sente" ? "gote" : "sente" } });
    return command({ kind: "xiangqi_preferences", settings: { analysis_defaults: snapshot.document.settings.analysis_defaults, orientation: orientation === "red" ? "black" : "red" } });
  }
  const defaults = snapshot.game === "xiangqi" ? snapshot.document.settings.analysis_defaults : snapshot.document.settings.settings.analysis_defaults;
  const exactPrefix = snapshot.game !== "xiangqi" || snapshot.xiangqi.canonical_context_scope === "selected_committed";
  return <ApplicationShell game={snapshot.game} historical={snapshot.historical} blocked={confirmation !== null} onFocusCapture={focus}
    runtime={thinking ? "mShogi is thinking" : analyzing ? "Analyzing selected position" : "Local engine"}
    attributes={{ "data-current-event": snapshot.event_index, "data-live-events": snapshot.events.filter(event => event.index !== 0).length, "data-paused": snapshot.paused, "data-orientation": orientation }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled game"}</h2>
      {snapshot.io.phase !== "idle" && snapshot.io.phase !== "failed" && <p role="status">{snapshot.io.message ?? (snapshot.io.phase === "saving" ? "Saving your game…" : snapshot.io.phase === "opening" ? "Opening your game…" : "File operation in progress…")}</p>}</>}
    actions={<><button className="primary" onClick={onNewGame} disabled={disabled}>New game</button>
      <button className="quiet" onClick={() => { void session.file(snapshot, "save"); }} disabled={disabled || !snapshot.capabilities.save}>Save</button>
      <button className="quiet" onClick={() => { void session.file(snapshot, "save_as"); }} disabled={disabled || !snapshot.capabilities.save}>Save as…</button>
      <button className="quiet" onClick={onOpen} disabled={disabled}>Open…</button></>}
    heading={<LiveGameHeading eyebrow={snapshot.historical ? "HISTORY · READ ONLY" : snapshot.status.concluded ? "GAME CONCLUDED" : "YOUR GAME"}
      status={snapshot.historical ? `Viewing event ${snapshot.event_index}` : thinking ? "mShogi is thinking" : (snapshot.game === "chess" && snapshot.chess.selected_conclusion ? conclusionName(snapshot.chess.selected_conclusion) : snapshot.status.label)}
      player={<>You play {snapshot.human.label} against mShogi</>}
      flip={<button className="quiet flip-button" disabled={disabled} aria-label={`Flip board; currently ${side.label} at bottom`} onClick={() => { void flip(); }}>↕ <span>Flip board</span></button>} />}
    positionNotice={snapshot.historical && <HistoricalNotice action={<button disabled={disabled} onClick={() => { void command({ kind: "return_live" }); }}>Return to live</button>}>
      <p>Viewing event {snapshot.event_index} · the live game is unchanged. Analysis uses this exact native history prefix.</p></HistoricalNotice>}
    board={<GameBoard snapshot={snapshot} disabled={disabled || capacity || !snapshot.capabilities.board_action}
      inspectionDisabled={disabled} command={command} />}
    notices={<NativeRecovery snapshot={snapshot} error={session.error} disabled={disabled} command={command} />}
    analysis={<NativeAnalysis snapshot={snapshot} defaults={defaults} disabled={disabled || capacity} command={command}
      maxDepth={snapshot.game === "chess" ? 8 : snapshot.game === "shogi" ? 4 : 3}
      maxNodes={snapshot.game === "chess" ? 5000000 : snapshot.game === "shogi" ? 20000 : 512} minNodes={snapshot.game === "xiangqi" ? 1 : 0}
      unavailable={!exactPrefix ? "Analysis is unavailable inside a recorded command. Select a completed event or return to live." : undefined} />}
    history={<NativeHistory snapshot={snapshot} disabled={disabled} command={command} />}
    gameActions={<GameActions snapshot={snapshot} disabled={disabled || capacity} command={command} confirm={setConfirmation} />}
    technical={<><p>mShogi · untimed {snapshot.game}</p><p>Game records, legality and movement inspection are owned by the native application.</p>
      {snapshot.game === "xiangqi" && <details><summary>Natural board-win assessment</summary><p>{snapshot.xiangqi.no_win_assessment.kind === "proven_neither_can_win" ? "Sound fast proof: neither side can reach a natural board win." : snapshot.xiangqi.no_win_assessment.kind === "proven_natural_win" ? "Sound fast witness: a possible natural board win is not a forced win." : snapshot.xiangqi.no_win_assessment.kind === "failure" ? "Assessment failed; no game result is implied." : "Unknown. No exact proof is available from ordinary search. This does not establish a win or a draw."}</p><p>Exact reports are historical evidence, never pending work or a new game outcome.</p><pre>{JSON.stringify(snapshot.xiangqi.exact_observations, null, 2)}</pre></details>}
      {snapshot.game !== "chess" && <details><summary>Typeface license</summary><p>{fontNotice}</p><a href={licenseUrl} download="mshogi-pieces-OFL.txt">Save the font license</a><pre>{fontLicense}</pre></details>}
      <details><summary>Native session and diagnostics</summary><pre>{JSON.stringify({ token: snapshot.token, profile: snapshot.profile, document: snapshot.document, context: snapshot.context, capacity: snapshot.capacity, diagnostic: snapshot.diagnostic, io: snapshot.io, error: session.error, analysis: snapshot.game === "chess" ? snapshot.chess.analysis : snapshot.analysis, observations: snapshot.observations, ...(snapshot.game === "xiangqi" ? { xiangqi: snapshot.xiangqi } : snapshot.game === "chess" ? { chess: snapshot.chess, chess_match: snapshot.chess_match } : {}) }, null, 2)}</pre></details>
      <button className="quiet" disabled={disabled} onClick={() => { void session.requestClose(); }}>Quit application</button></>}
    overlays={!blocked && confirmation && <Dialog title={confirmation.title} cancel={() => { if (!busy) setConfirmation(null); }}>
      <p>{confirmation.meaning}</p><div className="dialog-actions"><button data-cancel className="quiet" disabled={busy} onClick={() => setConfirmation(null)}>Cancel</button>
        <button className="danger" disabled={busy || JSON.stringify(confirmation.source.token) !== JSON.stringify(snapshot.token)} onClick={() => {
          const captured = confirmation; setConfirmation(null); void session.command(captured.source, captured.operation);
        }}>{confirmation.action}</button></div></Dialog>} />;
}
