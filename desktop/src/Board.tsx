import { useId } from "react";
import { createPortal } from "react-dom";
import type { BoardPiece, Color } from "./native";
import type { BoardGeometry, CompletedActionGeometry, NativeChoice, NativeHand, NativePiece } from "./boardPresentation";
import { gridNeighbor, useNativeChoiceController } from "./interaction/useNativeChoiceController";
import { inspectionStatus, type InspectMovement } from "./product/movementInspection";
import "./movement-inspection.css";

interface Props {
  pieces: (BoardPiece | NativePiece)[];
  choices: NativeChoice[];
  orientation: Color;
  version: number;
  disabled: boolean;
  onPlay: (action: string) => void;
  geometry?: BoardGeometry;
  glyphs?: Record<string, string>;
  profile?: string;
  nativeOrientation?: { files: string[]; ranks: string[]; label: string; bottomOwner: string };
  hands?: NativeHand[];
  nativeFirstOwner?: string;
  humanOwner?: string;
  onInspect?: InspectMovement | undefined;
  inspectionDisabled?: boolean;
  inspectionContextKey?: string;
  completedAction?: CompletedActionGeometry | null;
  // Product shells place live help in their stable footer. Standalone fixtures
  // retain inline help. Completed history geometry remains separate from input.
  helpTarget?: HTMLElement | null;
}
const chessGlyphs: Record<string, string> = {
  king: "♚", queen: "♛", rook: "♜", bishop: "♝", knight: "♞", pawn: "♟",
};
const chessGeometry: BoardGeometry = {
  files: [..."abcdefgh"], ranks: [..."12345678"], label: "Chess board", initialFocus: "e2",
};
export function displayedSquares(orientation: Color, geometry = chessGeometry): string[] {
  const files = orientation === "white" ? geometry.files : [...geometry.files].reverse();
  const ranks = orientation === "white" ? [...geometry.ranks].reverse() : geometry.ranks;
  return ranks.flatMap((rank) => files.map((file) => `${file}${rank}`));
}
export function Board({ pieces, choices, orientation, version, disabled,
  onPlay, geometry = chessGeometry, glyphs = {}, profile = "chess", nativeOrientation, hands = [], nativeFirstOwner, humanOwner, onInspect, inspectionDisabled = disabled, inspectionContextKey, helpTarget }: Props) {
  const completionTitle = useId();
  const squares = nativeOrientation ? nativeOrientation.ranks.flatMap(rank => nativeOrientation.files.map(file => `${file}${rank}`)) : displayedSquares(orientation, geometry);
  const columns = geometry.files.length, rows = geometry.ranks.length;
  const interaction = useNativeChoiceController({
    contextKey: JSON.stringify([version, profile, geometry, orientation, nativeOrientation, inspectionContextKey]),
    choices, origins: [...pieces.map(piece => ({ key: piece.square, owner: "owner" in piece ? piece.owner : piece.color, inspectable: true })),
      ...hands.map(hand => ({ key: hand.origin, owner: hand.owner, inspectable: hand.count > 0 }))],
    humanOwner, disabled, inspectionDisabled, onInspect, onPlay, initialFocus: geometry.initialFocus, boardKeys: squares,
    handGroups: [...new Set(hands.map(hand => hand.owner))].map(owner => hands.filter(hand => hand.owner === owner && hand.count > 0).map(hand => hand.origin)),
    neighbor: (key, direction) => gridNeighbor(squares, columns, key, direction),
  });
  const { selected, prefix, completions, inspection, inspectionPending, destinations } = interaction;
  const promotionDialog = completions.length > 0 && profile === "chess" && completions.every(choice => choice.completion !== null);
  const displayFiles = nativeOrientation?.files ?? (orientation === "white" ? geometry.files : [...geometry.files].reverse());
  const displayRanks = nativeOrientation?.ranks ?? (orientation === "white" ? [...geometry.ranks].reverse() : geometry.ranks);
  const width = 40 + columns * 100, height = 40 + rows * 100;
  const help = <p className={`board-help${inspection || inspectionPending ? " movement-inspection-status" : ""}`}>{inspection ? inspectionStatus(inspection.label) : inspectionPending ? "Loading read-only movement inspection…" : "Select a piece, then a destination. Arrow keys move focus · Enter or Space selects · Escape cancels."}</p>;
  return <div className="board-wrap" onFocusCapture={interaction.onFocusCapture} onBlurCapture={interaction.onBlurCapture}>
    {hands.length > 0 && <div role="group" aria-label="Native hands">
      {hands.map(hand => <button key={hand.origin} ref={node => interaction.register(hand.origin, node)}
        aria-label={`${hand.label}${inspection?.origin === hand.origin ? `, Inspecting ${inspection.label}` : ""}`} aria-pressed={selected === hand.origin}
        disabled={completions.length > 0 || hand.count <= 0 || !interaction.canInteractOrigin(hand.origin)}
        onFocus={() => interaction.onFocus(hand.origin)} onClick={() => interaction.select(hand.origin)} onKeyDown={event => interaction.keyDown(event, hand.origin)}>{hand.label}</button>)}
    </div>}
    <div className="board-fit">
      <svg className="chess-board" viewBox={`0 0 ${width} ${height}`} role="group" aria-label={`${geometry.label}, ${nativeOrientation?.label ?? orientation} at bottom`}>
        <rect width={width} height={height} rx="8" className="board-border" />
        {squares.map((square, index) => {
          const piece = pieces.find(piece => piece.square === square);
          const row = Math.floor(index / columns), column = index % columns;
          const target = destinations.includes(square), available = choices.some(choice => choice.from === square);
          const label = `${piece?.label ?? `${square}, empty`}${selected === square ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected" : ""}${prefix.includes(square) ? ", selected route step" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}`;
          return <g key={square} ref={node => interaction.register(square, node)} transform={`translate(${20 + column * 100}, ${20 + row * 100})`}
            role="button" tabIndex={(squares.includes(interaction.focused) ? interaction.focused : geometry.initialFocus) === square ? 0 : -1} aria-label={label} aria-pressed={selected === square} aria-disabled={interaction.interactionDisabled}
            className={`square ${(row + column) % 2 === 0 ? "light" : "dark"} ${selected === square && !inspection && !inspectionPending ? "selected" : ""} ${target && !inspection ? "target" : ""} ${available && !disabled ? "available" : ""}`}
            onFocus={() => interaction.onFocus(square)} onClick={() => interaction.select(square)} onKeyDown={event => interaction.keyDown(event, square)}>
            <rect width="100" height="100" className="square-fill" />
            {target && !inspection && <circle cx="50" cy="50" r={piece ? "40" : "12"} className={piece ? "capture-marker" : "move-marker"} />}
            {piece && <text x="50" y="74" textAnchor="middle" data-owner={"owner" in piece ? piece.owner : piece.color} className={`piece ${"owner" in piece ? (piece.owner === nativeFirstOwner ? "native-first-owner" : "native-second-owner") : piece.color}`} aria-hidden="true">{"text" in piece ? piece.text : glyphs[piece.kind] ?? chessGlyphs[piece.kind] ?? "?"}</text>}
            {inspection?.origin === square && <rect className="inspection-origin-ring" x="7" y="7" width="86" height="86" rx="5" aria-hidden="true" />}
            {target && inspection && <circle cx="50" cy="50" r={piece ? "43" : "19"} className="inspection-marker" aria-hidden="true" />}
            {column === 0 && <text x="7" y="20" className="coordinate" aria-hidden="true">{displayRanks[row]}</text>}
            {row === rows - 1 && <text x="82" y="93" className="coordinate" aria-hidden="true">{displayFiles[column]}</text>}
            <rect x="3" y="3" width="94" height="94" className="focus-ring" />
          </g>;
        })}
      </svg>
    </div>
    {helpTarget === undefined ? help : helpTarget ? createPortal(help, helpTarget) : null}
    {interaction.continuation && <div className="route-selection" role="group" aria-label="Partial route selection" onKeyDown={event => { if (event.key === "Escape") { event.preventDefault(); interaction.cancel(); } }}>
      <p>Selected route: {selected} → {prefix.join(" → ")}. No action submitted.</p>
      {interaction.terminals.length > 0 && !inspection && <button disabled={disabled} onClick={interaction.completeHere}>Complete action here</button>}
      <button className="quiet" onClick={interaction.cancel}>Cancel selection</button>
    </div>}
    <span className="sr-only" role="status">{interaction.announcement}</span>
    {completions.length > 0 && <div className="modal-backdrop"><div className="promotion-dialog" role="dialog" aria-modal="true" aria-labelledby={completionTitle}
      ref={interaction.completionRef} onKeyDown={interaction.completionKey}>
      <p className="eyebrow">COMPLETE YOUR MOVE</p><h2 id={completionTitle}>{promotionDialog ? "Choose promotion" : "Choose complete action"}</h2>
      <p>The action is applied only after you choose.</p>
      <div className="promotion-choices">{completions.map(choice => <button key={choice.action} onClick={() => interaction.chooseCompletion(choice.action)}>
        {choice.completion?.glyph && <span aria-hidden="true">{choice.completion.glyph}</span>}{choice.completion?.label ?? choice.action}
      </button>)}</div>
      <button className="quiet" onClick={interaction.cancel}>{promotionDialog ? "Cancel promotion" : "Cancel selection"}</button>
    </div></div>}
  </div>;
}
