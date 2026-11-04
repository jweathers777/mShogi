import { useState } from "react";
import type { CompletedActionGeometry } from "../boardPresentation";
import { NativeBoard } from "../NativeBoard";
import type { TenjikuSnapshot } from "./native";
import type { InspectMovement } from "./movementInspection";
import { GameFrame } from "./GameFrame";
import { TenjikuPiece } from "./TenjikuPiece";
import "./tenjiku.css";

// Artwork decorates the surviving shared native controller. The selected native
// identity, including a promoted piece's original base, is never reconstructed.
export function TenjikuBoard({ snapshot, disabled, inspectionDisabled, onPlay, onInspect, completedAction }: {
  snapshot: TenjikuSnapshot; disabled: boolean; inspectionDisabled: boolean;
  onPlay: (action: string) => void; onInspect: InspectMovement;
  completedAction?: CompletedActionGeometry | null;
}) {
  const [help, setHelp] = useState<HTMLDivElement | null>(null);
  const bottom = snapshot.sides.find(side => side.id.side === snapshot.document.settings.settings.orientation)!;
  return <GameFrame className="tenjiku-stage" snapshot={snapshot} bottom={bottom}
    board={<div className="tenjiku-board-frame"><NativeBoard snapshot={snapshot} bottomSide={bottom.key}
      disabled={disabled} inspectionDisabled={inspectionDisabled} coordinateFrame onInspect={onInspect} onPlay={onPlay}
      completedAction={completedAction ?? null} helpTarget={help} renderNativePiece={(piece, rotated) => {
        const identity = snapshot.tenjiku.selected.identities.find(identity => identity.square === piece.square);
        return identity ? <TenjikuPiece face={identity.face} base={identity.piece.base}
          promoted={identity.piece.promoted} rotated={rotated} owner={piece.owner}
          royal={snapshot.tenjiku.selected.royals.some(royal => royal.piece.id === identity.piece.id)} /> : null;
      }} /></div>}
    help={<div ref={setHelp} className="tenjiku-board-help shell-board-help" />} />;
}
