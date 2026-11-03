import { useId, type ReactNode } from "react";
import { createPortal } from "react-dom";
import type { BoardPiece, Color } from "./native";
import type { BoardGeometry, CompletedActionGeometry, NativeChoice, NativeHand, NativePiece } from "./boardPresentation";
import { gridNeighbor, useNativeChoiceController } from "./interaction/useNativeChoiceController";
import { inspectionStatus, type InspectMovement } from "./product/movementInspection";
import { CompletedActionCue } from "./CompletedActionCue";
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
  // Native identity decoration only; the shared controller still owns choices.
  renderNativePiece?: (piece: NativePiece, rotated: boolean) => ReactNode;
  // Layout only: keep dense native-board coordinates outside occupied squares.
  coordinateFrame?: boolean;
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
  onPlay, geometry = chessGeometry, glyphs = {}, profile = "chess", nativeOrientation, hands = [], nativeFirstOwner, humanOwner, onInspect, inspectionDisabled = disabled, inspectionContextKey, helpTarget, completedAction, renderNativePiece, coordinateFrame = false }: Props) {
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
  const padding = coordinateFrame ? 40 : 20;
  const width = padding * 2 + columns * 100, height = padding * 2 + rows * 100;
  const help = <p className={`board-help${inspection || inspectionPending ? " movement-inspection-status" : ""}`}>{inspection ? inspectionStatus(inspection.label) : inspectionPending ? "Loading read-only movement inspection…" : "Arrows: move focus · Enter/Space: select · Esc: cancel"}</p>;
  const routeControls = interaction.continuation && <div className="route-selection" role="group" aria-label="Partial route selection" onKeyDown={event => { if (event.key === "Escape") { event.preventDefault(); interaction.cancel(); } }}>
    <p>Selected route: {selected} → {prefix.join(" → ")}. No action submitted.</p>
    {interaction.terminals.length > 0 && !inspection && <button disabled={disabled} onClick={interaction.completeHere}>Complete action here</button>}
    <button className="quiet" onClick={interaction.cancel}>Cancel selection</button>
  </div>;
  return <div className="board-wrap" onFocusCapture={interaction.onFocusCapture} onBlurCapture={interaction.onBlurCapture}>
    {hands.length > 0 && <div role="group" aria-label="Native hands">
      {hands.map(hand => <button key={hand.origin} ref={node => interaction.register(hand.origin, node)}
        aria-label={`${hand.label}${inspection?.origin === hand.origin ? `, Inspecting ${inspection.label}` : ""}`} aria-pressed={selected === hand.origin}
        disabled={completions.length > 0 || hand.count <= 0 || !interaction.canInteractOrigin(hand.origin)}
        onFocus={() => interaction.onFocus(hand.origin)} onClick={() => interaction.select(hand.origin)} onKeyDown={event => interaction.keyDown(event, hand.origin)}>{hand.label}</button>)}
    </div>}
    <div className="board-fit">
      <svg className={`chess-board${coordinateFrame ? " outside-coordinates" : ""}`} viewBox={`0 0 ${width} ${height}`} role="group" aria-label={`${geometry.label}, ${nativeOrientation?.label ?? orientation} at bottom`}>
        <rect width={width} height={height} rx="8" className="board-border" />
        {squares.map((square, index) => {
          const piece = pieces.find(piece => piece.square === square);
          const row = Math.floor(index / columns), column = index % columns;
          const target = destinations.includes(square), available = choices.some(choice => choice.from === square);
          const label = `${piece?.label ?? `${square}, empty`}${selected === square ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected" : ""}${prefix.includes(square) ? ", selected route step" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}`;
          return <g key={square} ref={node => interaction.register(square, node)} transform={`translate(${padding + column * 100}, ${padding + row * 100})`}
            role="button" tabIndex={(squares.includes(interaction.focused) ? interaction.focused : geometry.initialFocus) === square ? 0 : -1} aria-label={label} aria-pressed={selected === square} aria-disabled={interaction.interactionDisabled}
            className={`square ${(row + column) % 2 === 0 ? "light" : "dark"} ${selected === square && !inspection && !inspectionPending ? "selected" : ""} ${target && !inspection ? "target" : ""} ${available && !disabled ? "available" : ""}`}
            onFocus={() => interaction.onFocus(square)} onClick={() => interaction.select(square)} onKeyDown={event => interaction.keyDown(event, square)}>
            <rect width="100" height="100" className="square-fill" />
            {target && !inspection && <circle cx="50" cy="50" r={piece ? "40" : "12"} className={piece ? "capture-marker" : "move-marker"} />}
            {piece && (renderNativePiece && "owner" in piece ? renderNativePiece(piece, piece.owner !== nativeOrientation?.bottomOwner) : <text x="50" y="74" textAnchor="middle" data-owner={"owner" in piece ? piece.owner : piece.color} className={`piece ${"owner" in piece ? (piece.owner === nativeFirstOwner ? "native-first-owner" : "native-second-owner") : piece.color}`} aria-hidden="true">{"text" in piece ? piece.text : glyphs[piece.kind] ?? chessGlyphs[piece.kind] ?? "?"}</text>)}
            {inspection?.origin === square && <rect className="inspection-origin-ring" x="7" y="7" width="86" height="86" rx="5" aria-hidden="true" />}
            {target && inspection && <circle cx="50" cy="50" r={piece ? "43" : "19"} className="inspection-marker" aria-hidden="true" />}
            {!coordinateFrame && column === 0 && <text x="7" y="20" className="coordinate" aria-hidden="true">{displayRanks[row]}</text>}
            {!coordinateFrame && row === rows - 1 && <text x="82" y="93" className="coordinate" aria-hidden="true">{displayFiles[column]}</text>}
            <rect x="3" y="3" width="94" height="94" className="focus-ring" />
          </g>;
        })}
        {coordinateFrame && <g className="native-coordinate-frame" aria-hidden="true" pointerEvents="none">
          {displayFiles.map((file, index) => <text key={`file-${index}`} className="coordinate" data-coordinate-axis="file" x={padding + index * 100 + 50} y={height - 8} textAnchor="middle">{file}</text>)}
          {displayRanks.map((rank, index) => <text key={`rank-${index}`} className="coordinate" data-coordinate-axis="rank" x={width - 20} y={padding + index * 100 + 62} textAnchor="middle">{rank}</text>)}
        </g>}
        {renderNativePiece && selected && prefix.length > 0 && !inspection && <g className="native-selected-route" aria-hidden="true" pointerEvents="none" data-selected-route={[selected, ...prefix].join(">")}>
          <polyline points={[selected, ...prefix].map(key => { const i = squares.indexOf(key); return `${padding + 50 + i % columns * 100},${padding + 50 + Math.floor(i / columns) * 100}`; }).join(" ")} />
          {prefix.map((key, index) => { const i = squares.indexOf(key); return <g key={index} transform={`translate(${padding + 84 + i % columns * 100} ${padding + 16 + Math.floor(i / columns) * 100})`}><circle r="14" /><text textAnchor="middle" y="5">{index + 1}</text></g>; })}
        </g>}
        <CompletedActionCue action={completedAction} center={key => { const index = squares.indexOf(key); return index < 0 ? null : [padding + 50 + index % columns * 100, padding + 50 + Math.floor(index / columns) * 100]; }} />
      </svg>
    </div>
    {helpTarget === undefined ? <>{help}{routeControls}</> : helpTarget ? createPortal(<>{help}{routeControls}</>, helpTarget) : null}
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
