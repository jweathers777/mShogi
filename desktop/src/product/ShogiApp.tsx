import { TakeBackControl } from "./TakeBackControl";
import { useState } from "react";
import { ShogiBoard } from "./ShogiBoard";
import { ApplicationShell, DocumentState, HistoricalNotice, LiveGameHeading } from "./ApplicationShell";
import { useMovementInspection } from "./useMovementInspection";
import { Dialog } from "./Dialog";
import type { ChessShogiSnapshot as GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import licenseUrl from "../assets/fonts/OFL.txt?url";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";
import { NativeAnalysis, NativeHistory, NativeRecovery, nativeCapacityReached, useNativeCapacityError, useNativeWorkCommand } from "./NativeSessionPanels";
export function ShogiApp({ snapshot, session, onNewGame, onOpen, blocked }: { snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean }) {
  const onInspect = useMovementInspection({ token: snapshot.token, game: snapshot.game });
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening"].includes(snapshot.io.phase);
  const disabled = busy || blocked;
  const capacityError = useNativeCapacityError(snapshot, session.error);
  const capacity = nativeCapacityReached(snapshot, capacityError);
  const [confirmation, setConfirmation] = useState<{ source: GameSnapshot; kind: "resign" | "declare" } | null>(null);
  const { statusRef: boardStatus, command } = useNativeWorkCommand(snapshot, session);
  const analyzing = snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const status = snapshot.historical ? `Viewing event ${snapshot.event_index}` : snapshot.status.concluded ? snapshot.status.label : thinking ? "mShogi is thinking" : snapshot.status.label;
  const filename = snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled shogi game";
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
    notices={<NativeRecovery snapshot={snapshot} error={session.error} capacityError={capacityError} disabled={disabled} command={command} />}
    analysis={<NativeAnalysis snapshot={snapshot} defaults={snapshot.document.settings.settings.analysis_defaults} disabled={disabled || capacity} command={command} maxDepth={4} maxNodes={20000} minNodes={0} />}
    history={<NativeHistory snapshot={snapshot} disabled={disabled} command={command} />}
    gameActions={<>        <section className="shogi-game-actions"><TakeBackControl available={snapshot.capabilities.take_back && !snapshot.historical} disabled={disabled || capacity} contextKey={JSON.stringify(snapshot.token)} restore={() => command({ kind: "take_back" })} />{snapshot.capabilities.declaration && <><p>Entering-king prerequisites are satisfied. Eligibility alone does not conclude the game.</p><button className="secondary" disabled={disabled || capacity} onClick={() => setConfirmation({ source: snapshot, kind: "declare" })}>Declare entering king…</button></>}{snapshot.capabilities.resign && <button className="quiet" disabled={disabled || capacity} onClick={() => setConfirmation({ source: snapshot, kind: "resign" })}>Resign game…</button>}</section>
</>}
    technical={<><p>mShogi · untimed standard shogi</p><p>Game records and legality are owned by the native application.</p><details><summary>Typeface license</summary><p>{fontNotice}</p><a href={licenseUrl} download="mshogi-pieces-OFL.txt">Save the font license</a><pre>{fontLicense}</pre></details><details><summary>Native session and diagnostics</summary><pre>{JSON.stringify({ token: snapshot.token, profile: snapshot.profile, document: snapshot.document, capacity: snapshot.capacity, capacityError, diagnostic: snapshot.diagnostic, error: session.error, analysis: snapshot.analysis, observations: snapshot.observations }, null, 2)}</pre></details><button className="quiet" disabled={disabled} onClick={() => { void import("@tauri-apps/api/core").then(({ invoke }) => invoke("desktop_request_close")); }}>Quit application</button></>}
    overlays={<>    {confirmation && <Dialog title={confirmation.kind === "resign" ? "Resign this game?" : "Declare entering king?"} cancel={() => setConfirmation(null)}><p>{confirmation.kind === "resign" ? `${snapshot.human.label} will resign this game.` : "This formally submits the declaration. Eligibility advice by itself has not ended the game. An invalid declaration loses under the selected rules."}</p><div className="dialog-actions"><button data-cancel className="quiet" onClick={() => setConfirmation(null)}>Cancel</button><button className={confirmation.kind === "resign" ? "danger" : "primary"} disabled={busy || confirmation.source.token.document !== snapshot.token.document || confirmation.source.token.version !== snapshot.token.version || confirmation.source.token.selection !== snapshot.token.selection || confirmation.source.token.revision !== snapshot.token.revision} onClick={() => { const c = confirmation; setConfirmation(null); void session.command(c.source, { kind: c.kind }); }}>{confirmation.kind === "resign" ? "Resign game" : "Confirm declaration"}</button></div></Dialog>}</>}
  />;
}
