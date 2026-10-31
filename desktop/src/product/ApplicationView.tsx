import { useEffect, useState } from "react";
import { ApplicationShell, DocumentState, HistoricalNotice, LiveGameHeading } from "./ApplicationShell";
import { GameBoard } from "./GameBoard";
import { GameActions, type Confirmation } from "./GameCapabilities";
import { NativeAnalysis, NativeHistory, NativeRecovery, nativeCapacityReached } from "./NativeSessionPanels";
import { Dialog } from "./Dialog";
import { useApplicationFocus } from "./useApplicationFocus";
import type { GameSnapshot, Operation } from "./native";
import type { GameSession } from "./useGameSession";
import { gameStatus } from "../presentation";
import { SupportDialog } from "./SupportDialog";
import "./shogi.css";
import "./xiangqi.css";
export function ApplicationView({ snapshot, session, onNewGame, onOpen, blocked }: {
  snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean;
}) {
  const [supportOpen, setSupportOpen] = useState(false);
  const [confirmation, setConfirmation] = useState<Confirmation | null>(null);
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening", "viewing"].includes(snapshot.io.phase);
  const disabled = busy || blocked || confirmation !== null || supportOpen;
  const focus = useApplicationFocus(blocked || confirmation !== null || supportOpen);
  useEffect(() => { if (blocked) { setConfirmation(null); setSupportOpen(false); } }, [blocked]);
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
  return <ApplicationShell game={snapshot.game} historical={snapshot.historical} blocked={confirmation !== null || supportOpen} onFocusCapture={focus}
    runtime={thinking ? "mShogi is thinking" : analyzing ? "Analyzing selected position" : "Local engine"}
    attributes={{ "data-current-event": snapshot.event_index, "data-live-events": snapshot.events.filter(event => event.index !== 0).length, "data-paused": snapshot.paused, "data-orientation": orientation }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled game"}</h2>
      {["saving", "opening", "viewing"].includes(snapshot.io.phase) && <p role="status">{snapshot.io.phase === "saving" ? "Saving your game…" : snapshot.io.phase === "opening" ? "Opening your game…" : "Opening the selected position…"}</p>}</>}
    actions={<><button className="primary" onClick={onNewGame} disabled={disabled}>New game</button>
      <button className="quiet" onClick={() => { void session.file(snapshot, "save"); }} disabled={disabled || !snapshot.capabilities.save}>Save</button>
      <button className="quiet" onClick={() => { void session.file(snapshot, "save_as"); }} disabled={disabled || !snapshot.capabilities.save}>Save as…</button>
      <button className="quiet" onClick={onOpen} disabled={disabled}>Open…</button></>}
    heading={<LiveGameHeading eyebrow={snapshot.historical ? "HISTORY · READ ONLY" : snapshot.status.concluded ? "GAME CONCLUDED" : "YOUR GAME"}
      status={snapshot.historical ? "Viewing history" : thinking ? "mShogi is thinking" : (snapshot.game === "chess" ? gameStatus({ conclusion: snapshot.chess.selected_conclusion, turn: snapshot.turn.id.side, in_check: snapshot.chess.in_check }) : snapshot.status.label)}
      player={<>You play {snapshot.human.label} against mShogi</>}
      flip={<button className="quiet flip-button" disabled={disabled} aria-label={`Flip board; currently ${side.label} at bottom`} onClick={() => { void flip(); }}>↕ <span>Flip board</span></button>} />}
    positionNotice={snapshot.historical && <HistoricalNotice action={<button disabled={disabled} onClick={() => { void command({ kind: "return_live" }); }}>Return to live</button>}>
      <p>Your live game is unchanged.</p></HistoricalNotice>}
    board={<GameBoard snapshot={snapshot} disabled={disabled || capacity || !snapshot.capabilities.board_action}
      inspectionDisabled={disabled} command={command} />}
    notices={<NativeRecovery snapshot={snapshot} error={session.error} disabled={disabled} command={command} />}
    analysis={<NativeAnalysis snapshot={snapshot} defaults={defaults} disabled={disabled || capacity} command={command}
      maxDepth={snapshot.game === "chess" ? 8 : snapshot.game === "shogi" ? 4 : 3}
      maxNodes={snapshot.game === "chess" ? 5000000 : snapshot.game === "shogi" ? 20000 : 512} minNodes={snapshot.game === "xiangqi" ? 1 : 0}
      unavailable={!exactPrefix ? "Analysis is unavailable between a move and its following event. Select a completed event or return to live." : undefined} />}
    history={<NativeHistory snapshot={snapshot} disabled={disabled} command={command} />}
    gameActions={<GameActions snapshot={snapshot} disabled={disabled || capacity} command={command} confirm={setConfirmation} />}
    support={<button className="support-trigger" data-support-trigger aria-label="About mShogi" title="About mShogi" disabled={disabled} onClick={() => setSupportOpen(true)}><svg width="18" height="18" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><circle cx="12" cy="12" r="9" fill="none" stroke="currentColor" strokeWidth="1.6" /><path d="M12 10v7" fill="none" stroke="currentColor" strokeWidth="1.8" /><circle cx="12" cy="7" r="1" fill="currentColor" /></svg></button>}
    overlays={<>{!blocked && supportOpen && <SupportDialog snapshot={snapshot} session={session} close={() => setSupportOpen(false)} />}{!blocked && confirmation && <Dialog title={confirmation.title} cancel={() => { if (!busy) setConfirmation(null); }}>
      <p>{confirmation.meaning}</p><div className="dialog-actions"><button data-cancel className="quiet" disabled={busy} onClick={() => setConfirmation(null)}>Cancel</button>
        <button className="danger" disabled={busy || JSON.stringify(confirmation.source.token) !== JSON.stringify(snapshot.token)} onClick={() => {
          const captured = confirmation; setConfirmation(null); void session.command(captured.source, captured.operation);
        }}>{confirmation.action}</button></div></Dialog>}</>} />;
}
