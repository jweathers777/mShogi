import type { CompletedActionGeometry, ShogiSnapshot, ToriSnapshot, Side } from "../native";
import type { InspectMovement } from "./movementInspection";

import { gridNeighbor, useNativeChoiceController } from "../interaction/useNativeChoiceController";
import { BoardFooter, CompletionDialog, pointLabel } from "../BoardControls";

import { bottomSide } from "../sessionState";
import { GameFrame } from "./GameFrame";
import { Piece } from "./Piece";
import { CompletedActionCue } from "../CompletedActionCue";
import "../movement-inspection.css";

// Shogi and Tori share square/tray composition, not piece identities or rules.
// The native choices and the shared controller own every admitted interaction.
export function ShogiBoard({ snapshot, disabled, onPlay, onInspect, inspectionDisabled = disabled, completedAction = null }: {
  snapshot: ShogiSnapshot | ToriSnapshot; disabled: boolean; onPlay: (action: string) => void;
  onInspect?: InspectMovement | undefined; inspectionDisabled?: boolean;
  // Separate read-only presentation input for MSH-123; never controller state.
  completedAction?: CompletedActionGeometry | null;
}) {
  const game = snapshot.game, bottom = bottomSide(snapshot);
  const firstMover = (owner: string) => snapshot.sides.find(s => s.key === owner)?.first_mover ?? true;
  const orientation = snapshot.orientations.find(o => o.side.key === bottom.key)!;
  const topSide = snapshot.sides.find(s => s.key !== bottom.key)!;
  const squares = orientation.ranks.flatMap(rank => orientation.files.map(file => `${file}${rank}`));
  const columns = orientation.files.length, rows = orientation.ranks.length;
  const hands = snapshot.hands.filter(hand => hand.count > 0);
  const interaction = useNativeChoiceController({
    contextKey: JSON.stringify([game, snapshot.profile, snapshot.edition, snapshot.token, bottom.key]),
    choices: snapshot.choices,
    origins: [...snapshot.pieces.map(piece => ({ key: piece.square, label: piece.label, owner: piece.owner, inspectable: true })),
      ...hands.map(hand => ({ key: hand.origin, label: hand.label, owner: hand.owner, inspectable: true }))],
    humanOwner: snapshot.local_actor.key, disabled: disabled || !snapshot.capabilities.board_action, inspectionDisabled, onInspect, onPlay,
    initialFocus: snapshot.board.initial_focus, boardKeys: squares,
    neighbor: (key, direction) => gridNeighbor(squares, columns, key, direction),
    handGroups: [topSide, bottom].map(side => hands.filter(hand => hand.owner === side.key).map(hand => hand.origin)),
  });
  const { selected, focused, inspection, inspectionPending, destinations } = interaction;
  function tray(side: Side) {
    const own = hands.filter(h => h.owner === side.key);
    return <div className="shogi-hand" role="group" aria-label={`${side.label} pieces in hand`}>
      {!own.length && <span className="empty-hand">No pieces in hand</span>}
      {own.map(h => <button key={h.origin} className={`hand-piece${selected === h.origin ? inspection ? " inspecting" : inspectionPending ? "" : " selected" : ""}`} ref={node => interaction.register(h.origin, node)}
        disabled={interaction.interactionDisabled || !interaction.canInteractOrigin(h.origin)}
        title={h.label} data-hand={h.kind} data-hand-piece={h.kind} data-owner={side.id.side} aria-label={`${h.label}${selected === h.origin ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected for drop" : ""}`} aria-pressed={selected === h.origin}
        onFocus={() => interaction.onFocus(h.origin)} onClick={() => interaction.select(h.origin)} onKeyDown={event => interaction.keyDown(event, h.origin)}><svg viewBox="0 0 100 100" aria-hidden="true"><Piece game={game} face={h.kind} rotated={side.key !== bottom.key} first={side.first_mover} /></svg><span className="hand-count">{h.count}</span></button>)}
    </div>;
  }
  const width = columns * 100 + 56, height = rows * 100 + 56;
  const board = <div className={`shogi-board-frame${game === "tori" ? " tori-board-frame" : ""}`}>
    <svg className="shogi-board" viewBox={`0 0 ${width} ${height}`} role="group" aria-label={`${snapshot.board.label}, ${orientation.side.label} at bottom`}>
      <defs><linearGradient id="shogi-wood" x1="0" x2="1"><stop stopColor="#e4c38d" /><stop offset="0.5" stopColor="#ebd09d" /><stop offset="1" stopColor="#dfbc82" /></linearGradient></defs>
      <rect className="shogi-field" width={width} height={height} rx="7" />
      {squares.map((square, index) => {
        const row = Math.floor(index / columns), column = index % columns;
        const piece = snapshot.pieces.find(p => p.square === square), target = destinations.includes(square);
        return <g key={square} className={`shogi-square${selected === square && !inspection && !inspectionPending ? " selected" : ""}${target && !inspection ? " target" : ""}`} data-square={square}
          transform={`translate(${28 + column * 100},${28 + row * 100})`} ref={node => interaction.register(square, node)}
          role="button" tabIndex={(squares.includes(focused) ? focused : snapshot.board.initial_focus) === square ? 0 : -1} aria-disabled={interaction.interactionDisabled} aria-pressed={selected === square}
          aria-label={pointLabel(square, piece, interaction)}
          onFocus={() => interaction.onFocus(square)} onClick={() => interaction.select(square)} onKeyDown={event => interaction.keyDown(event, square)}>
          {piece && <title>{piece.label}</title>}
          <rect className="shogi-cell" width="100" height="100" />
          <rect className="shogi-selection-ring" x="5" y="5" width="90" height="90" rx="3" />
          {piece && <g transform="translate(5 3) scale(.9)"><Piece game={game} face={piece.kind} promoted={piece.promoted} rotated={piece.owner !== bottom.key} first={firstMover(piece.owner)} /></g>}
          {target && !inspection && <g className="shogi-destination" aria-hidden="true" pointerEvents="none"><path d="M9 28V9H28 M72 9H91V28 M91 72V91H72 M28 91H9V72" />{!piece && <circle cx="50" cy="50" r="9" />}</g>}
          {inspection?.origin === square && <rect className="inspection-origin-ring" x="5" y="5" width="90" height="90" rx="3" aria-hidden="true" />}
          {target && inspection && <circle className="inspection-marker" cx="50" cy="50" r={piece ? "46" : "19"} aria-hidden="true" />}
          <rect className="shogi-focus-ring" x="3" y="3" width="94" height="94" rx="3" />
        </g>;
      })}
      {orientation.files.map((file, i) => <text className="shogi-coordinate" key={file} x={78 + i * 100} y={height - 7} textAnchor="middle" aria-hidden="true">{file}</text>)}
      {orientation.ranks.map((rank, i) => <text className="shogi-coordinate" key={rank} x={width - 13} y={87 + i * 100} textAnchor="middle" aria-hidden="true">{rank}</text>)}
      <CompletedActionCue action={completedAction} center={key => { const index = squares.indexOf(key); return index < 0 ? null : [78 + index % columns * 100, 78 + Math.floor(index / columns) * 100]; }} />
    </svg>
    <CompletionDialog interaction={interaction} promotion />
  </div>;
  return <GameFrame className={`shogi-stage${game === "tori" ? " tori-stage" : ""}`} snapshot={snapshot} upperContent={tray(topSide)} board={board}
    lowerContent={tray(bottom)} help={<BoardFooter interaction={interaction} disabled={disabled} />}
    onFocusCapture={interaction.onFocusCapture} onBlurCapture={interaction.onBlurCapture} />;
}
