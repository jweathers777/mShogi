import { useRef, useState } from "react";
import { claimName } from "../presentation";
import { TakeBackControl } from "./TakeBackControl";
import type { GameSnapshot, Operation } from "./native";
export interface Confirmation { source: GameSnapshot; operation: Operation; title: string; action: string; meaning: string }
export function GameActions({ snapshot, disabled, command, confirm }: {
  snapshot: GameSnapshot; disabled: boolean; command: (operation: Operation) => Promise<unknown>;
  confirm: (value: Confirmation) => void;
}) {
  const [claimIndex, setClaimIndex] = useState(0);
  const resigning = useRef(false);
  async function resign() {
    if (disabled || resigning.current) return;
    resigning.current = true;
    try { await command(snapshot.game === "xiangqi" ? { kind: "xiangqi", command: { kind: "resign" } } : { kind: "resign" }); }
    finally { resigning.current = false; }
  }
  const ask = (operation: Operation, title: string, action: string, meaning: string) => confirm({ source: snapshot, operation, title, action, meaning });
  const terminal = snapshot.game === "xiangqi" ? snapshot.xiangqi.terminal_actions : null;
  const claims = snapshot.game === "chess" ? snapshot.chess.claims : [];
  const claim = claims[claimIndex] ?? claims[0];
  return <section className="shogi-game-actions" data-focus-after="outcome">
    <TakeBackControl available={snapshot.capabilities.take_back && !snapshot.historical} disabled={disabled}
      contextKey={JSON.stringify(snapshot.token)} restore={() => command({ kind: "take_back" })} />
    {snapshot.game === "chess" && !snapshot.historical && claims.length > 0 && <div className="claims">
      <h3 role="status">Draw claim available</h3><p className="caption">An opportunity to claim, while the game remains ongoing. An intended move is not played by claiming.</p>
      {claim && snapshot.human.key === snapshot.turn.key ? <><label className="sr-only" htmlFor="draw-claim">Draw claim</label>
        <select id="draw-claim" value={claims[claimIndex] ? claimIndex : 0} onChange={e => setClaimIndex(Number(e.target.value))}>
          {claims.map((c, i) => <option value={i} key={`${c.reason}-${c.intended}`}>{claimName(c)}</option>)}</select>
        <p className="claim-scope">{claim.intended === null ? "Current-position claim" : `Intended-move claim · ${claim.intended}`}</p>
        <p className="caption">{claim.intended === null ? "This position qualifies for the selected claim." : "This action is an unplayed intention. Claiming does not add this move to the board or move history."}</p>
        <button className="secondary" disabled={disabled || snapshot.paused || !snapshot.capabilities.claims} onClick={() => { void command({ kind: "chess_claim", claim }); }}>Claim selected draw</button></> : <p>mShogi may claim this draw.</p>}
    </div>}
    {snapshot.game === "shogi" && snapshot.capabilities.declaration && <><p>Entering-king prerequisites are satisfied. Eligibility alone does not conclude the game.</p>
      <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "declare" }, "Declare entering king?", "Confirm declaration", "This formally submits the declaration. Eligibility advice by itself has not ended the game. An invalid declaration loses under the selected rules.")}>Declare entering king…</button></>}
    {snapshot.game === "xiangqi" && snapshot.capabilities.claims && <><p>A natural-count draw claim is available to {snapshot.human.label}. It has not concluded the game.</p>
      <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "xiangqi", command: { kind: "claim_natural_count" } }, "Claim a natural-count draw?", "Claim draw", "Submit your natural-count claim. A valid claim ends the game as a draw.")}>Claim natural-count draw…</button></>}
    {terminal?.offer_draw && <button className="quiet" disabled={disabled} onClick={() => ask({ kind: "xiangqi", command: { kind: "offer_draw" } }, "Offer a draw?", "Offer draw", "An offer alone does not end the game. Your opponent must accept it.")}>Offer draw…</button>}
    {terminal?.accept_draw && <button className="secondary" disabled={disabled} onClick={() => ask({ kind: "xiangqi", command: { kind: "accept_draw" } }, "Accept the draw?", "Accept draw", "Accepting the outstanding offer ends this game as a draw.")}>Accept draw…</button>}
    {terminal?.decline_draw && <button className="quiet" disabled={disabled} onClick={() => ask({ kind: "xiangqi", command: { kind: "decline_draw" } }, "Decline the draw?", "Decline draw", "The game will continue without this offer.")}>Decline draw…</button>}
    {snapshot.capabilities.resign && <button className="quiet resign-button" disabled={disabled} onClick={() => { void resign(); }}>Resign</button>}
  </section>;
}
