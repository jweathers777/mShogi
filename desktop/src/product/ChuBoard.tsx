import { useState } from "react";
import type { CompletedActionGeometry } from "../boardPresentation";
import { NativeBoard } from "../NativeBoard";
import type { ChuSnapshot } from "./native";
import type { InspectMovement } from "./movementInspection";
import { GameFrame } from "./GameFrame";
import { ChuPiece } from "./ChuPiece";

// Chu artwork and square layout decorate the existing native board adapter.
// No second selection controller, movement rules, hands or royal-check filter.
export function ChuBoard({ snapshot, disabled, inspectionDisabled, onPlay, onInspect, completedAction }: {
  snapshot: ChuSnapshot; disabled: boolean; inspectionDisabled: boolean;
  onPlay: (action: string) => void; onInspect: InspectMovement;
  completedAction?: CompletedActionGeometry | null;
}) {
  const [help, setHelp] = useState<HTMLDivElement | null>(null);
  const bottom = snapshot.sides.find(side => side.id.side === snapshot.document.settings.settings.orientation)!;
  const board = <div className="chu-board-frame">
    <NativeBoard snapshot={snapshot} bottomSide={bottom.key} disabled={disabled} inspectionDisabled={inspectionDisabled}
      onInspect={onInspect} onPlay={onPlay} completedAction={completedAction ?? null} helpTarget={help}
      renderNativePiece={(piece, rotated) => {
        const nativeIdentity = snapshot.chu.selected.identities.find(identity => identity.square === piece.square);
        return nativeIdentity ? <g transform="translate(3 1) scale(.94)"><ChuPiece face={nativeIdentity.face} promoted={nativeIdentity.piece.promoted} rotated={rotated} /></g> : null;
      }} />
  </div>;
  return <GameFrame className="chu-stage" snapshot={snapshot} bottom={bottom} board={board}
    help={<div ref={setHelp} className="chu-board-help shell-board-help" />} />;
}
