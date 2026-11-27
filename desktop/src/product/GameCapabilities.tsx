import type { GameSnapshot, GameCommand, Operation } from "../sessionTypes";
import { useRef, useState } from "react";
import { claimName } from "../presentation";
import { TakeBackControl } from "./TakeBackControl";

export interface Confirmation { source: GameSnapshot; operation: Operation; title: string; action: string; meaning: string }
export function GameActions({ snapshot, disabled, resignDisabled = disabled, command, confirm }: {
  snapshot: GameSnapshot; disabled: boolean; resignDisabled?: boolean; command: (operation: Operation) => Promise<unknown>;
  confirm: (value: Confirmation) => void;
}) {
  const [claimIndex, setClaimIndex] = useState(0);
  const resigning = useRef(false);
  async function resign() {
    if (resignDisabled || resigning.current) return;
    resigning.current = true;
    try { await command({ kind: "resign" }); }
    finally { resigning.current = false; }
  }
  // The session derives each command's actor: the local player or the opponent of a draw offer.
  const ask = (command: GameCommand, title: string, action: string, meaning: string) => confirm({ source: snapshot, operation: { kind: "command", command }, title, action, meaning });
  const terminal = snapshot.game === "xiangqi" ? snapshot.xiangqi.terminal_actions : null;
  const claims = snapshot.game === "chess" ? snapshot.chess.claims : [];
  const claim = claims[claimIndex] ?? claims[0];
  const offerName = terminal && snapshot.play_mode === "two_humans" ? `Offer draw for ${terminal.offer_actor.label}` : "Offer draw";
  const responseName = terminal && snapshot.play_mode === "two_humans" ? `Accept draw for ${terminal.response_actor.label}` : "Accept draw";
  const declineName = terminal && snapshot.play_mode === "two_humans" ? `Decline draw for ${terminal.response_actor.label}` : "Decline draw";
  const drawOffer = snapshot.game === "chu" ? snapshot.chu.live.draw_offer : snapshot.game === "tenjiku" ? snapshot.tenjiku.live.draw_offer : null;
  const drawResponder = snapshot.play_mode === "two_humans" && drawOffer !== null
    ? snapshot.sides.find(side => side.id.side !== drawOffer)! : snapshot.local_actor;
  return <section className="shogi-game-actions" data-focus-after="outcome">
    <TakeBackControl available={snapshot.capabilities.take_back && !snapshot.historical} disabled={disabled}
      contextKey={JSON.stringify(snapshot.token)} restore={() => command({ kind: "take_back" })} />
    {snapshot.game === "chess" && !snapshot.historical && claims.length > 0 && <div className="claims">
      <h3 role="status">Draw claim available</h3><p className="caption">An opportunity to claim, while the game remains ongoing. An intended move is not played by claiming.</p>
      {claim && snapshot.local_actor.key === snapshot.turn.key ? <><label className="sr-only" htmlFor="draw-claim">Draw claim</label>
        <select id="draw-claim" value={claims[claimIndex] ? claimIndex : 0} onChange={e => setClaimIndex(Number(e.target.value))}>
          {claims.map((c, i) => <option value={i} key={`${c.reason}-${c.intended}`}>{claimName(c)}</option>)}</select>
        <p className="claim-scope">{claim.intended === null ? "Current-position claim" : `Intended-move claim · ${claim.intended}`}</p>
        <p className="caption">{claim.intended === null ? "This position qualifies for the selected claim." : "This action is an unplayed intention. Claiming does not add this move to the board or move history."}</p>
        <button className="secondary" disabled={disabled || snapshot.paused || !snapshot.capabilities.claims} onClick={() => { void command({ kind: "command", command: { claim: { reason: claim.reason, intended: claim.intended } } }); }}>Claim selected draw</button></> : <p>mShogi may claim this draw.</p>}
    </div>}
    {snapshot.game === "shogi" && snapshot.capabilities.declaration && <><p>Entering-king prerequisites are satisfied. Eligibility alone does not conclude the game.</p>
      <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "declare_entering_king" }, "Declare entering king?", "Confirm declaration", "This formally submits the declaration. Eligibility advice by itself has not ended the game. An invalid declaration loses under the selected rules.")}>Declare entering king…</button></>}
    {snapshot.game === "xiangqi" && snapshot.capabilities.claims && <><p>A natural-count draw claim is available to {snapshot.local_actor.label}. It has not concluded the game.</p>
      <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "claim_natural_count" }, "Claim a natural-count draw?", "Claim draw", "Submit your natural-count claim. A valid claim ends the game as a draw.")}>Claim natural-count draw…</button></>}
    {terminal?.offer_draw && <button className="quiet" disabled={disabled} onClick={() => ask({ kind: "offer_draw" }, snapshot.play_mode === "two_humans" ? `${offerName}?` : "Offer a draw?", offerName, "An offer alone does not end the game. Your opponent must accept it.")}>{offerName}…</button>}
    {terminal?.accept_draw && <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "accept_draw" }, snapshot.play_mode === "two_humans" ? `${responseName}?` : "Accept the draw?", responseName, "Accepting the outstanding offer ends this game as a draw.")}>{responseName}…</button>}
    {terminal?.decline_draw && <button className="quiet" disabled={disabled} onClick={() => ask({ kind: "decline_draw" }, snapshot.play_mode === "two_humans" ? `${declineName}?` : "Decline the draw?", declineName, "The game will continue without this offer.")}>{declineName}…</button>}
    {(snapshot.game === "chu" || snapshot.game === "tenjiku") && !snapshot.historical && !snapshot.status.concluded && <GameDrawControls
      offeredBy={snapshot.game === "chu" ? snapshot.chu.live.draw_offer : snapshot.tenjiku.live.draw_offer}
      human={drawResponder.id.side} disabled={disabled}
      offer={() => ask({ kind: "offer_draw" }, "Offer a draw?", "Offer draw", "An offer alone does not end the game. Your opponent must accept it.")}
      accept={() => ask({ kind: "accept_draw" }, "Accept the draw?", "Accept draw", "Accepting the outstanding offer ends this game by agreement.")}
      decline={() => { void command({ kind: "command", command: { kind: "decline_draw" } }); }} />}
    {snapshot.capabilities.resign && <button className="quiet resign-button" disabled={resignDisabled} onClick={() => { void resign(); }}>{snapshot.play_mode === "two_humans" ? `Resign ${snapshot.local_actor.label}` : "Resign"}</button>}
  </section>;
}

function GameDrawControls({ offeredBy, human, disabled, offer, accept, decline }: {
  offeredBy: string | null; human: string; disabled: boolean; offer: () => void; accept: () => void; decline: () => void;
}) {
  return <>{offeredBy === null && <button className="quiet" disabled={disabled} onClick={offer}>Offer draw…</button>}{offeredBy !== null && offeredBy !== human && <><button className="secondary" disabled={disabled} onClick={accept}>Accept draw…</button><button className="quiet" disabled={disabled} onClick={decline}>Decline draw</button></>}</>;
}
