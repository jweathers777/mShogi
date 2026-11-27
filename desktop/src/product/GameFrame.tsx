import type { GameSnapshot, Side } from "../sessionTypes";
import type { FocusEventHandler, ReactNode } from "react";
import { bottomSide, sessionStatus } from "../sessionState";
import { GameArea, GamePositionStatus, PlayerArea } from "./ApplicationShell";

// Application-owned player/status placement; hands and board help remain
// renderer content. The shared choice controller owns board interaction;
// this frame owns no selection, inspection or completion state.
export function GameFrame({ snapshot, board, help, upperContent, lowerContent, className = "", onFocusCapture, onBlurCapture }: {
  snapshot: GameSnapshot; board: ReactNode; help: ReactNode;
  upperContent?: ReactNode; lowerContent?: ReactNode; className?: string;
  onFocusCapture?: FocusEventHandler<HTMLDivElement>; onBlurCapture?: FocusEventHandler<HTMLDivElement>;
}) {
  const bottom = bottomSide(snapshot);
  const upper = snapshot.sides.find(side => side.key !== bottom.key)!;
  const player = (side: Side, content: ReactNode) => <PlayerArea side={side.label}
    human={snapshot.play_mode === "two_humans" || side.key === snapshot.human.key} participant={snapshot.play_mode === "two_humans" ? "Human" : undefined} playerKey={side.id.side}
    className={(snapshot.game === "shogi" || snapshot.game === "tori") ? "shogi-player" : ""}
    toMove={!snapshot.status.concluded && side.key === snapshot.turn.key}>{content}</PlayerArea>;
  return <GameArea className={className} upperPlayer={player(upper, upperContent)}
    lowerPlayer={player(bottom, lowerContent)} board={board} {...(onFocusCapture ? { onFocusCapture } : {})} {...(onBlurCapture ? { onBlurCapture } : {})}
    footer={<>{help}<GamePositionStatus historical={snapshot.historical} concluded={snapshot.status.concluded}
      paused={snapshot.paused} humanTurn={snapshot.local_actor.key === snapshot.turn.key} stateLabel={snapshot.play_mode === "two_humans" && !snapshot.historical && !snapshot.status.concluded && !snapshot.paused ? `${snapshot.turn.label} to move` : undefined}
      thinking={sessionStatus(snapshot) === "thinking"} ply={snapshot.board_ply} /></>} />;
}
