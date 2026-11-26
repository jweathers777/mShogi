import type { FocusEventHandler, ReactNode } from "react";
import { GameArea, GamePositionStatus, PlayerArea } from "./ApplicationShell";
import type { GameSnapshot, Side } from "./native";

/** The side that the document settings place at the bottom of the board. */
export function bottomSide(snapshot: GameSnapshot): Side {
  return snapshot.sides.find(side => side.id.side === snapshot.document.settings.orientation.side)!;
}

// Application-owned player/status placement; hands and board help remain native
// renderer content. The shared native-choice controller owns board interaction;
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
      thinking={snapshot.capabilities.engine_turn && snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn"} ply={snapshot.board_ply} /></>} />;
}
