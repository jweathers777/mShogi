import { TakeBackControl } from "./TakeBackControl";
import { useState } from "react";
import { XiangqiBoard } from "./XiangqiBoard";
import { ApplicationShell, DocumentState, HistoricalNotice, LiveGameHeading } from "./ApplicationShell";
import { NativeAnalysis, NativeHistory, NativeRecovery, nativeCapacityReached, useNativeCapacityError, useNativeWorkCommand } from "./NativeSessionPanels";
import { Dialog } from "./Dialog";
import { useMovementInspection } from "./useMovementInspection";
import type { XiangqiSnapshot as GameSnapshot, Operation } from "./native";
import type { GameSession } from "./useGameSession";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";

type Terminal = "resign" | "offer_draw" | "accept_draw" | "decline_draw" | "claim_natural_count";
const terminalLabels: Record<Terminal, { title: string; action: string; meaning: string }> = {
  resign: { title: "Resign this game?", action: "Resign game", meaning: "You will resign this game." },
  offer_draw: { title: "Offer a draw?", action: "Offer draw", meaning: "An offer alone does not end the game. Your opponent must accept it." },
  accept_draw: { title: "Accept the draw?", action: "Accept draw", meaning: "Accepting the outstanding offer ends this game as a draw." },
  decline_draw: { title: "Decline the draw?", action: "Decline draw", meaning: "The game will continue without this offer." },
  claim_natural_count: { title: "Claim a natural-count draw?", action: "Claim draw", meaning: "Submit your native-verified natural-count claim. A valid claim ends the game as a draw." },
};
const sideName = (side: "red" | "black") => side === "red" ? "Red" : "Black";
export function XiangqiApp({ snapshot, session, onNewGame, onOpen, blocked }: {
  snapshot: GameSnapshot; session: GameSession; onNewGame: () => void; onOpen: () => void; blocked: boolean;
}) {
  const onInspect = useMovementInspection({ token: snapshot.token, game: snapshot.game });
  const busy = session.busy || snapshot.capacity.status === "checking" || ["saving", "opening"].includes(snapshot.io.phase);
  const disabled = busy || blocked;
  const capacityError = useNativeCapacityError(snapshot, session.error);
  const capacity = nativeCapacityReached(snapshot, capacityError);
  const [confirmation, setConfirmation] = useState<{ source: GameSnapshot; kind: Terminal } | null>(null);
  const { statusRef, command } = useNativeWorkCommand(snapshot, session);
  const thinking = snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn";
  const analyzing = snapshot.phase === "thinking" && snapshot.execution_purpose === "move_analysis";
  const facts = snapshot.xiangqi, actions = facts.terminal_actions;
  const orientation = snapshot.document.settings.orientation;
  const filename = snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled xiangqi game";
  const confirm = (kind: Terminal) => setConfirmation({ source: snapshot, kind });
  const eligible = (kind: Terminal) => kind === "resign" ? snapshot.capabilities.resign : kind === "claim_natural_count" ? snapshot.capabilities.claims : actions[kind];
  const nativePrefixIncomplete = facts.canonical_context_scope !== "selected_committed";
  const warning = facts.warnings.filter((w) => w.active);
  return <ApplicationShell game="xiangqi" runtime={thinking ? "mShogi is thinking" : analyzing ? "Analyzing selected position" : "Local engine"} historical={snapshot.historical} blocked={confirmation !== null}
    attributes={{ "data-current-event": snapshot.event_index, "data-live-events": snapshot.events.length, "data-paused": snapshot.paused, "data-orientation": orientation }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{filename}</h2>{["saving", "opening"].includes(snapshot.io.phase) && <p role="status">{snapshot.io.phase === "saving" ? "Saving your game…" : "Opening your game…"}</p>}</>}
    actions={<><button className="primary" onClick={onNewGame} disabled={disabled}>New game</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save"); }}>Save</button><button className="quiet" disabled={disabled} onClick={() => { void session.file(snapshot, "save_as"); }}>Save as…</button><button className="quiet" disabled={disabled} onClick={onOpen}>Open…</button></>}
    heading={<LiveGameHeading statusRef={statusRef} eyebrow={snapshot.historical ? "HISTORY · READ ONLY" : snapshot.status.concluded ? "GAME CONCLUDED" : "YOUR GAME"}
      status={snapshot.historical ? `Viewing event ${snapshot.event_index}` : thinking ? "mShogi is thinking" : snapshot.status.label} player={<>You play {snapshot.human.label} against mShogi</>}
      flip={<button className="quiet flip-button" aria-label={`Flip board; currently ${sideName(orientation)} at bottom`} disabled={disabled} onClick={() => {
        void command({ kind: "xiangqi_preferences", settings: { analysis_defaults: snapshot.document.settings.analysis_defaults, orientation: orientation === "red" ? "black" : "red" } });
      }}>↕ <span>Flip board</span></button>} />}
    positionNotice={snapshot.historical && <HistoricalNotice action={<button disabled={disabled} onClick={() => { void command({ kind: "return_live" }); }}>Return to live</button>}><p>Viewing event {snapshot.event_index} · the live game is unchanged</p></HistoricalNotice>}
    board={<XiangqiBoard snapshot={snapshot} disabled={disabled || capacity || !snapshot.capabilities.board_action || confirmation !== null}
      inspectionDisabled={disabled || confirmation !== null} onInspect={onInspect} onPlay={(action) => { void command({ kind: "xiangqi", command: { kind: "play", action } }); }} />}
    notices={<><NativeRecovery snapshot={snapshot} error={session.error} capacityError={capacityError} disabled={disabled} command={command} pending={facts.pending !== null} />
      {facts.pending && <section className="shogi-notice" role="status"><h3>Adjudication unresolved</h3><p>Play is stopped. This game is unscored; no win, loss or draw has been established.</p></section>}
      {!snapshot.status.concluded && warning.length > 0 && <section className="shogi-notice" role="status"><h3>Repetition warning</h3>{warning.map((w) => <p key={w.event_index}>{sideName(w.side)} must vary {w.fault === "perpetual_check" ? "perpetual check" : "perpetual chase"}. Continuing the prohibited repetition can lose the game.</p>)}</section>}
      {facts.offered_by !== null && !snapshot.status.concluded && <section className="shogi-notice" role="status"><h3>Draw offered by {sideName(facts.offered_by)}</h3><p>The offer has not concluded the game.</p></section>}
    </>}
    analysis={<NativeAnalysis snapshot={snapshot} defaults={snapshot.document.settings.analysis_defaults} disabled={disabled || capacity} command={command} maxDepth={3} maxNodes={512} minNodes={1}
      unavailable={nativePrefixIncomplete ? "Analysis is unavailable inside a recorded command. Select a completed event or return to live." : undefined} />}
    history={<NativeHistory snapshot={snapshot} disabled={disabled} command={command} />}
    gameActions={<section className="shogi-game-actions"><TakeBackControl available={snapshot.capabilities.take_back && !snapshot.historical} disabled={disabled || capacity} contextKey={JSON.stringify(snapshot.token)} restore={() => command({ kind: "take_back" })} />
      {snapshot.capabilities.claims && <><p>A natural-count draw claim is available to {snapshot.human.label}. It has not concluded the game.</p><button className="secondary" disabled={disabled || capacity} onClick={() => confirm("claim_natural_count")}>Claim natural-count draw…</button></>}
      {actions.offer_draw && <button className="quiet" disabled={disabled || capacity} onClick={() => confirm("offer_draw")}>Offer draw…</button>}
      {actions.accept_draw && <button className="secondary" disabled={disabled || capacity} onClick={() => confirm("accept_draw")}>Accept draw…</button>}
      {actions.decline_draw && <button className="quiet" disabled={disabled || capacity} onClick={() => confirm("decline_draw")}>Decline draw…</button>}
      {snapshot.capabilities.resign && <button className="quiet" disabled={disabled || capacity} onClick={() => confirm("resign")}>Resign game…</button>}
    </section>}
    technical={<><p>mShogi · untimed xiangqi</p><p>Game records, legality and movement inspection are owned by the native application.</p>
      <details><summary>Natural board-win assessment</summary><p>{facts.no_win_assessment.kind === "proven_neither_can_win" ? "Sound fast proof: neither side can reach a natural board win." : facts.no_win_assessment.kind === "proven_natural_win" ? `Sound fast witness for ${sideName(facts.no_win_assessment.winner!)}. A possible win is not a forced win.` : facts.no_win_assessment.kind === "failure" ? "Assessment failed; no game result is implied." : "Unknown. No exact proof is available from ordinary search. This does not establish a win or a draw."}</p><p>Exact reports, when present, are saved evidence, not pending work or a new game outcome.</p><pre>{JSON.stringify(facts.exact_observations, null, 2)}</pre></details>
      <details><summary>Typeface license</summary><p>{fontNotice}</p><pre>{fontLicense}</pre></details>
      <details><summary>Native session and diagnostics</summary><pre>{JSON.stringify({ token: snapshot.token, profile: snapshot.profile, document: snapshot.document, context: snapshot.context, xiangqi: facts, capacity: snapshot.capacity, capacityError, diagnostic: snapshot.diagnostic, io: snapshot.io, error: session.error, observations: snapshot.observations }, null, 2)}</pre></details>
      <button className="quiet" disabled={disabled} onClick={() => { void import("@tauri-apps/api/core").then(({ invoke }) => invoke("game_request_close")); }}>Quit application</button></>}
    overlays={confirmation && <Dialog title={terminalLabels[confirmation.kind].title} cancel={() => setConfirmation(null)}><p>{terminalLabels[confirmation.kind].meaning}</p><div className="dialog-actions"><button data-cancel className="quiet" onClick={() => setConfirmation(null)}>Cancel</button><button className={confirmation.kind === "resign" ? "danger" : "primary"} disabled={disabled || capacity || !eligible(confirmation.kind) || JSON.stringify(confirmation.source.token) !== JSON.stringify(snapshot.token)} onClick={() => { const captured = confirmation; setConfirmation(null); void session.command(captured.source, { kind: "xiangqi", command: { kind: captured.kind } } as Operation); }}>{terminalLabels[confirmation.kind].action}</button></div></Dialog>}
  />;
}
