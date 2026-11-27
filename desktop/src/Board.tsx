import type { CompletedActionGeometry, BoardChoice } from "./sessionTypes";
import type { InspectMovement } from "./product/movementInspection";
import type { FocusEventHandler, ReactNode } from "react";

import { gridNeighbor, useBoardChoiceController } from "./interaction/useBoardChoiceController";

import { BoardFooter, CompletionDialog, pointLabel } from "./BoardControls";
import { CompletedActionCue } from "./CompletedActionCue";
import "./movement-inspection.css";

/** A piece on a grid board: its square, owner key and accessible label. */
export interface GridPiece { square: string; owner: string; kind: string; label: string; text?: string }
export interface BoardFocus { onFocusCapture: FocusEventHandler<HTMLDivElement>; onBlurCapture: FocusEventHandler<HTMLDivElement> }
interface Props<P extends GridPiece> {
  pieces: P[];
  choices: BoardChoice[];
  // Coordinates in display order, bottom owner's view.
  files: string[];
  ranks: string[];
  label: string;
  initialFocus: string;
  bottomOwner: string;
  firstOwner: string;
  contextKey: string;
  disabled: boolean;
  onPlay: (action: string) => void;
  humanOwner?: string | undefined;
  onInspect?: InspectMovement | undefined;
  inspectionDisabled?: boolean;
  completedAction?: CompletedActionGeometry | null;
  // Text glyphs by piece kind; the piece text otherwise.
  glyphs?: Record<string, string>;
  // Piece artwork that replaces the glyph; the controller still owns choices.
  renderPiece?: (piece: P, rotated: boolean) => ReactNode;
  // Layout only: keep dense board coordinates outside occupied squares.
  coordinateFrame?: boolean;
  promotion?: boolean;
  // Places the board, its footer and focus handlers; inline by default.
  frame?: (board: ReactNode, footer: ReactNode, focus: BoardFocus) => ReactNode;
}
/** A square grid for Chess, Chu, Tenjiku and the synthetic fixture. */
export function Board<P extends GridPiece>({ pieces, choices, files, ranks, label, initialFocus, bottomOwner, firstOwner, contextKey, disabled,
  onPlay, humanOwner, onInspect, inspectionDisabled = disabled, completedAction, glyphs = {}, renderPiece, coordinateFrame = false, promotion = false,
  frame = (board, footer, focus) => <div className="board-wrap" {...focus}>{board}{footer}</div> }: Props<P>) {
  const squares = ranks.flatMap(rank => files.map(file => `${file}${rank}`));
  const columns = files.length, rows = ranks.length;
  const interaction = useBoardChoiceController({
    contextKey: JSON.stringify([contextKey, files, ranks]),
    choices, origins: pieces.map(piece => ({ key: piece.square, label: piece.label, owner: piece.owner, inspectable: true })),
    humanOwner, disabled, inspectionDisabled, onInspect, onPlay, initialFocus, boardKeys: squares,
    neighbor: (key, direction) => gridNeighbor(squares, columns, key, direction),
  });
  const { selected, prefix, inspection, inspectionPending, destinations } = interaction;
  const selectedChoices = (inspection?.choices ?? choices).filter(choice => choice.from === selected && prefix.every((step, index) => choice.steps[index] === step));
  const stationarySelection = selectedChoices.length > 0 && selectedChoices.every(choice => choice.tenjiku?.kind === "stationary_capture");
  const padding = coordinateFrame ? 40 : 20;
  const width = padding * 2 + columns * 100, height = padding * 2 + rows * 100;
  const center = (key: string): [number, number] | null => { const index = squares.indexOf(key); return index < 0 ? null : [padding + 50 + index % columns * 100, padding + 50 + Math.floor(index / columns) * 100]; };
  const board = <div className="board-fit">
    <svg className={`chess-board${coordinateFrame ? " outside-coordinates" : ""}`} viewBox={`0 0 ${width} ${height}`} role="group" aria-label={label}>
      <rect width={width} height={height} rx="8" className="board-border" />
      {squares.map((square, index) => {
        const piece = pieces.find(piece => piece.square === square);
        const row = Math.floor(index / columns), column = index % columns;
        const target = destinations.includes(square), available = choices.some(choice => choice.from === square);
        return <g key={square} ref={node => interaction.register(square, node)} transform={`translate(${padding + column * 100}, ${padding + row * 100})`}
          role="button" tabIndex={(squares.includes(interaction.focused) ? interaction.focused : initialFocus) === square ? 0 : -1}
          aria-label={pointLabel(square, piece, interaction)} aria-pressed={selected === square} aria-disabled={interaction.interactionDisabled}
          className={`square ${(row + column) % 2 === 0 ? "light" : "dark"} ${selected === square && !inspection && !inspectionPending ? "selected" : ""} ${target && !inspection ? "target" : ""} ${available && !disabled ? "available" : ""}`}
          onFocus={() => interaction.onFocus(square)} onClick={() => interaction.select(square)} onKeyDown={event => interaction.keyDown(event, square)}>
          <rect width="100" height="100" className="square-fill" />
          {target && !inspection && !piece && <circle cx="50" cy="50" r="12" className="move-marker" aria-hidden="true" pointerEvents="none" />}
          {piece && (renderPiece ? renderPiece(piece, piece.owner !== bottomOwner)
            : <text x="50" y="74" textAnchor="middle" data-owner={piece.owner} className={`piece ${piece.owner === firstOwner ? "first-owner" : "second-owner"}`} aria-hidden="true">{glyphs[piece.kind] ?? piece.text ?? "?"}</text>)}
          {target && !inspection && piece && <circle cx="50" cy="50" r="40" className="capture-marker" aria-hidden="true" pointerEvents="none" />}
          {inspection?.origin === square && <rect className="inspection-origin-ring" x="7" y="7" width="86" height="86" rx="5" aria-hidden="true" />}
          {target && inspection && <circle cx="50" cy="50" r={piece ? "43" : "19"} className="inspection-marker" aria-hidden="true" />}
          {!coordinateFrame && column === 0 && <text x="7" y="20" className="coordinate" aria-hidden="true">{ranks[row]}</text>}
          {!coordinateFrame && row === rows - 1 && <text x="82" y="93" className="coordinate" aria-hidden="true">{files[column]}</text>}
          <rect x="3" y="3" width="94" height="94" className="focus-ring" />
        </g>;
      })}
      {coordinateFrame && <g className="coordinate-frame" aria-hidden="true" pointerEvents="none">
        {files.map((file, index) => <text key={`file-${index}`} className="coordinate" data-coordinate-axis="file" x={padding + index * 100 + 50} y={height - 8} textAnchor="middle">{file}</text>)}
        {ranks.map((rank, index) => <text key={`rank-${index}`} className="coordinate" data-coordinate-axis="rank" x={width - 20} y={padding + index * 100 + 62} textAnchor="middle">{rank}</text>)}
      </g>}
      {selected && prefix.length > 0 && !inspection && !stationarySelection && <g className="selected-route" aria-hidden="true" pointerEvents="none" data-selected-route={[selected, ...prefix].join(">")}>
        <polyline points={[selected, ...prefix].map(key => center(key)?.join(",")).join(" ")} />
        {prefix.map((key, index) => { const [x, y] = center(key)!; return <g key={index} transform={`translate(${x + 34} ${y - 34})`}><circle r="14" /><text textAnchor="middle" y="5">{index + 1}</text></g>; })}
      </g>}
      <CompletedActionCue action={completedAction} center={center} />
    </svg>
    <CompletionDialog interaction={interaction} promotion={promotion} />
  </div>;
  return frame(board, <BoardFooter interaction={interaction} disabled={disabled} />, { onFocusCapture: interaction.onFocusCapture, onBlurCapture: interaction.onBlurCapture });
}
