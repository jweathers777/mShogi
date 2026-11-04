import { useState } from "react";
import type { CompletedActionGeometry } from "../boardPresentation";
import { NativeBoard } from "../NativeBoard";
import type { TenjikuSnapshot } from "./native";
import type { InspectMovement } from "./movementInspection";
import { GameFrame } from "./GameFrame";
import "./tenjiku.css";

// Technical native abbreviations only. Final Tenjiku artwork/notation belongs
// to MSH-44; all selection and completion use the surviving native controller.
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
      completedAction={completedAction ?? null} helpTarget={help} renderNativePiece={(piece, rotated) => <g
        data-tenjiku-piece={piece.kind} data-promoted={piece.promoted} data-owner={piece.owner}
        transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
        <path className="tenjiku-technical-piece" d="M50 10L83 26L89 88H11L17 26Z" />
        <text x="50" y="60" textAnchor="middle" className="tenjiku-technical-label">{piece.text}</text>
      </g>} /></div>}
    help={<div ref={setHelp} className="tenjiku-board-help shell-board-help" />} />;
}
