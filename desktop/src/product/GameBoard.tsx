import type { CompletedActionGeometry, GameSnapshot, Operation } from "../sessionTypes";

import { ShogiBoard } from "./ShogiBoard";
import { SquareBoard } from "./SquareBoard";
import { XiangqiBoard } from "./XiangqiBoard";
import { useMovementInspection } from "./useMovementInspection";

// Every game plays the opaque action of a game choice; the renderers differ
// only in artwork, hands and the Xiangqi intersection board.
export function GameBoard({ snapshot, disabled, inspectionDisabled, command, completedAction }: {
  snapshot: GameSnapshot; disabled: boolean; inspectionDisabled: boolean; command: (operation: Operation) => unknown;
  completedAction?: CompletedActionGeometry | null;
}) {
  const inspect = useMovementInspection({ token: snapshot.token, game: snapshot.game, ...(snapshot.game === "tenjiku" ? { profile: snapshot.profile } : {}) });
  const props = {
    disabled, inspectionDisabled, onInspect: inspect,
    onPlay: (action: string) => { void command({ kind: "play", action }); },
    completedAction: completedAction === undefined ? snapshot.moves.last_action : completedAction,
  };
  if (snapshot.game === "shogi" || snapshot.game === "tori") return <ShogiBoard snapshot={snapshot} {...props} />;
  if (snapshot.game === "xiangqi") return <XiangqiBoard snapshot={snapshot} {...props} />;
  return <SquareBoard snapshot={snapshot} {...props} />;
}
