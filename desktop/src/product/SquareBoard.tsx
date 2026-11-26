import { Board } from "../Board";
import type { CompletedActionGeometry } from "../boardPresentation";
import type { InspectMovement } from "./movementInspection";
import { GameFrame, bottomSide } from "./GameFrame";
import { Piece } from "./Piece";
import type { ChessSnapshot, ChuSnapshot, TenjikuSnapshot } from "./native";
import "./large-board.css";

const chessGlyphs: Record<string, string> = { king: "♚", queen: "♛", rook: "♜", bishop: "♝", knight: "♞", pawn: "♟" };

// The square boards of Chess and of the large shogi variants. Chu and Tenjiku
// draw the native face of each selected identity, including a promoted
// piece's original base; nothing is reconstructed from the board.
export function SquareBoard({ snapshot, disabled, onPlay, onInspect, inspectionDisabled = disabled, completedAction = null }: {
  snapshot: ChessSnapshot | ChuSnapshot | TenjikuSnapshot; disabled: boolean; onPlay: (action: string) => void;
  onInspect?: InspectMovement | undefined; inspectionDisabled?: boolean; completedAction?: CompletedActionGeometry | null;
}) {
  const bottom = bottomSide(snapshot);
  const orientation = snapshot.orientations.find(o => o.side.key === bottom.key)!;
  const game = snapshot.game;
  const identities = game === "chu" ? snapshot.chu.selected.identities : game === "tenjiku" ? snapshot.tenjiku.selected.identities : [];
  return <Board pieces={snapshot.pieces} choices={snapshot.choices} files={orientation.files} ranks={orientation.ranks}
    label={`${snapshot.board.label}, ${orientation.side.label} at bottom`} initialFocus={snapshot.board.initial_focus}
    bottomOwner={bottom.key} firstOwner={snapshot.sides.find(side => side.first_mover)!.key}
    contextKey={JSON.stringify([game, snapshot.profile, snapshot.token])} humanOwner={snapshot.local_actor.key}
    disabled={disabled || !snapshot.capabilities.board_action} inspectionDisabled={inspectionDisabled} onInspect={onInspect} onPlay={onPlay} completedAction={completedAction}
    glyphs={chessGlyphs} promotion={game === "chess"} coordinateFrame={game !== "chess"}
    {...(game === "chess" ? {} : { renderPiece: (piece, rotated) => {
      const identity = identities.find(identity => identity.square === piece.square);
      return identity ? <Piece game={game} face={identity.face} base={identity.piece.base} promoted={identity.piece.promoted} rotated={rotated} /> : null;
    } })}
    frame={(board, footer, focus) => <GameFrame snapshot={snapshot}
      board={game === "chess" ? <div className="board-wrap">{board}</div> : <div className={`large-board-frame ${game}-board-frame`}><div className="board-wrap">{board}</div></div>}
      help={footer} {...focus} />} />;
}
