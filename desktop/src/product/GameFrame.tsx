import type { FocusEventHandler, ReactNode } from "react";
import { GameArea, GamePositionStatus, PlayerArea } from "./ApplicationShell";
import type { GameSnapshot, Side } from "./native";

// Application-owned player/status placement; hands and board help remain native
// renderer content. The shared native-choice controller owns board interaction;
// this frame owns no selection, inspection or completion state.
export function GameFrame({ snapshot, bottom, board, help, upperContent, lowerContent, className = "", onFocusCapture, onBlurCapture }: {
  snapshot: GameSnapshot; bottom: Side; board: ReactNode; help: ReactNode;
  upperContent?: ReactNode; lowerContent?: ReactNode; className?: string;
  onFocusCapture?: FocusEventHandler<HTMLDivElement>; onBlurCapture?: FocusEventHandler<HTMLDivElement>;
}) {
  const upper = snapshot.sides.find(side => side.key !== bottom.key)!;
  const player = (side: Side, content: ReactNode) => <PlayerArea side={side.label}
    human={side.key === snapshot.human.key} playerKey={side.id.side}
    className={snapshot.game === "shogi" ? "shogi-player" : ""}
    toMove={!snapshot.status.concluded && side.key === snapshot.turn.key}>{content}</PlayerArea>;
  return <GameArea className={className} upperPlayer={player(upper, upperContent)}
    lowerPlayer={player(bottom, lowerContent)} board={board} {...(onFocusCapture ? { onFocusCapture } : {})} {...(onBlurCapture ? { onBlurCapture } : {})}
    footer={<>{help}<GamePositionStatus historical={snapshot.historical} concluded={snapshot.status.concluded}
      paused={snapshot.paused} humanTurn={snapshot.human.key === snapshot.turn.key}
      thinking={snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn"} ply={snapshot.board_ply} /></>} />;
}
