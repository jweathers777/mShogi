import { useState } from "react";
import type { CompletedActionGeometry } from "../boardPresentation";
import type { ToriAction } from "./native";
import { TenjikuBoard } from "./TenjikuBoard";
import { ChuBoard } from "./ChuBoard";
import { Board } from "../Board";
import { ShogiBoard } from "./ShogiBoard";
import { XiangqiBoard } from "./XiangqiBoard";
import { GameFrame } from "./GameFrame";
import { useMovementInspection } from "./useMovementInspection";
import type { GameSnapshot, Operation } from "./native";
export function GameBoard({ snapshot, disabled, inspectionDisabled, command, completedAction }: {
  snapshot: GameSnapshot; disabled: boolean; inspectionDisabled: boolean; command: (operation: Operation) => unknown;
  completedAction?: CompletedActionGeometry | null;
}) {
  const lastAction = completedAction === undefined ? snapshot.moves.last_action : completedAction;
  const inspect = useMovementInspection({ token: snapshot.token, game: snapshot.game, ...(snapshot.game === "tenjiku" ? { profile: snapshot.profile } : {}) });
  const [help, setHelp] = useState<HTMLDivElement | null>(null);
  if (snapshot.game === "shogi" || snapshot.game === "tori") return <ShogiBoard completedAction={lastAction} snapshot={snapshot} disabled={disabled} inspectionDisabled={inspectionDisabled}
    inspectionContextKey={JSON.stringify(snapshot.token)} onInspect={inspect} onPlay={action => {
      void command(snapshot.game === "tori"
        ? { kind: "tori", actor: snapshot.human.id.side as "sente" | "gote", command: { kind: "play", action: JSON.parse(action) as ToriAction } }
        : { kind: "play", action });
    }} />;
  if (snapshot.game === "xiangqi") return <XiangqiBoard completedAction={lastAction} snapshot={snapshot} disabled={disabled} inspectionDisabled={inspectionDisabled}
    onInspect={inspect} onPlay={action => { void command({ kind: "xiangqi", command: { kind: "play", action } }); }} />;
  if (snapshot.game === "tenjiku") return <TenjikuBoard snapshot={snapshot} completedAction={lastAction}
    disabled={disabled} inspectionDisabled={inspectionDisabled} onInspect={inspect}
    onPlay={action => { void command({ kind: "tenjiku", actor: snapshot.human.id.side as "sente" | "gote", command: { kind: "play", action: JSON.parse(action) } }); }} />;
  if (snapshot.game === "chu") return <ChuBoard snapshot={snapshot} completedAction={lastAction}
    disabled={disabled} inspectionDisabled={inspectionDisabled} onInspect={inspect}
    onPlay={action => { void command({ kind: "chu", actor: snapshot.human.id.side as "sente" | "gote", command: { kind: "play", action: JSON.parse(action) } }); }} />;
  const side = snapshot.document.settings.settings.orientation;
  const bottom = snapshot.sides.find(s => s.id.side === side)!;
  return <GameFrame snapshot={snapshot} bottom={bottom} help={<div ref={setHelp} className="shell-board-help" />}
    board={<Board completedAction={lastAction} pieces={snapshot.pieces.map(piece => ({ square: piece.square, kind: piece.kind, label: piece.label, color: snapshot.sides.find(owner => owner.key === piece.owner)!.id.side as "white" | "black" }))} choices={snapshot.choices}
      orientation={side} version={snapshot.token.version} disabled={disabled} onInspect={inspect}
      humanOwner={snapshot.human.id.side} inspectionDisabled={inspectionDisabled} inspectionContextKey={JSON.stringify(snapshot.token)}
      helpTarget={help} onPlay={action => { void command({ kind: "chess_play", action }); }} />} />;
}
