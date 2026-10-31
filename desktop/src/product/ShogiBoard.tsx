import type { CompletedActionGeometry, NativePiece } from "../boardPresentation";
import { gridNeighbor, useNativeChoiceController } from "../interaction/useNativeChoiceController";
import type { ShogiSnapshot as GameSnapshot, Side } from "./native";
import { inspectionStatus, type InspectMovement } from "./movementInspection";
import { GameFrame } from "./GameFrame";
import "../movement-inspection.css";

// Decoration only. Native kind/promotion/owner remain the action and identity data.
export function pieceGlyph(kind: string, promoted = false, first = true): string[] {
  const normal: Record<string, string> = { king: first ? "玉将" : "王将", rook: "飛車", bishop: "角行", gold: "金将", silver: "銀将", knight: "桂馬", lance: "香車", pawn: "歩兵" };
  const promotion: Record<string, string> = { rook: "龍王", bishop: "龍馬", silver: "成銀", knight: "成桂", lance: "成香", pawn: "と金" };
  return [...((promoted ? promotion[kind] : normal[kind]) ?? normal[kind] ?? "?")];
}
export function ShogiPiece({ kind, promoted = false, rotated = false, first = true }: { kind: string; promoted?: boolean; rotated?: boolean; first?: boolean }) {
  const glyph = pieceGlyph(kind, promoted, first);
  return <g className={`shogi-piece${promoted ? " promoted" : ""}`} transform={rotated ? "rotate(180 50 50)" : undefined} aria-hidden="true">
    <path className="piece-shadow" d="M50 5 83 28 91 96 9 96 17 28Z" transform="translate(0 2)" />
    <path className="piece-body" d="M50 3 82 26 90 94 10 94 18 26Z" />
    <path className="piece-bevel" d="M50 7 78 29 85 89 15 89 22 29Z" />
    <text x="50" y="47" textAnchor="middle">{glyph[0]}</text>
    {glyph[1] && <text x="50" y="81" textAnchor="middle">{glyph[1]}</text>}
  </g>;
}
function pieceName(piece: NativePiece, sides: Side[]) {
  const owner = sides.find((s) => s.key === piece.owner)?.label ?? "Player";
  return `${piece.square}, ${owner} ${piece.promoted ? "promoted " : ""}${piece.kind}`;
}
export function ShogiBoard({ snapshot, disabled, onPlay, onInspect, inspectionDisabled = disabled, inspectionContextKey }: {
  snapshot: GameSnapshot; disabled: boolean; onPlay: (action: string) => void;
  onInspect?: InspectMovement; inspectionDisabled?: boolean; inspectionContextKey?: string;
  // Separate read-only presentation input for MSH-123; never controller state.
  completedAction?: CompletedActionGeometry | null;
}) {
  const bottom = snapshot.document.settings.settings.orientation;
  const orientation = snapshot.orientations.find((o) => o.side.id.side === bottom) ?? snapshot.orientations[0]!;
  const topSide = snapshot.sides.find((s) => s.key !== orientation.side.key)!;
  const squares = orientation.ranks.flatMap((rank) => orientation.files.map((file) => `${file}${rank}`));
  const columns = orientation.files.length, rows = orientation.ranks.length;
  const nonzeroHands = snapshot.hands.filter((hand) => hand.count > 0);
  const interaction = useNativeChoiceController({
    contextKey: JSON.stringify([snapshot.game, snapshot.profile, snapshot.edition, snapshot.token, bottom, inspectionContextKey]),
    choices: snapshot.choices,
    origins: [...snapshot.pieces.map((piece) => ({ key: piece.square, owner: piece.owner, inspectable: true })),
      ...nonzeroHands.map((hand) => ({ key: hand.origin, owner: hand.owner, inspectable: true }))],
    humanOwner: snapshot.human.key, disabled, inspectionDisabled, onInspect, onPlay,
    initialFocus: snapshot.board.initial_focus, boardKeys: squares,
    neighbor: (key, direction) => gridNeighbor(squares, columns, key, direction),
    handGroups: [topSide, orientation.side].map((side) => nonzeroHands.filter((hand) => hand.owner === side.key).map((hand) => hand.origin)),
  });
  const { selected, prefix, focused, completions, announcement, inspection, inspectionPending, destinations, continuation, terminals } = interaction;
  function tray(side: Side) {
    const hands = snapshot.hands.filter((h) => h.owner === side.key && h.count > 0);
    return <>
      <div className="shogi-hand" role="group" aria-label={`${side.label} pieces in hand`}>
        {!hands.length && <span className="empty-hand">No pieces in hand</span>}
        {hands.map((h) => <button key={h.origin} className={`hand-piece${selected === h.origin ? inspection ? " inspecting" : inspectionPending ? "" : " selected" : ""}`} ref={(node) => interaction.register(h.origin, node)}
          disabled={interaction.interactionDisabled || !interaction.canInteractOrigin(h.origin)}
          data-hand={h.kind} data-hand-piece={h.kind} data-owner={side.id.side} aria-label={`${side.label} ${h.kind} in hand, ${h.count}${selected === h.origin ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected for drop" : ""}`} aria-pressed={selected === h.origin}
          onFocus={() => interaction.onFocus(h.origin)} onClick={() => interaction.select(h.origin)} onKeyDown={(event) => interaction.keyDown(event, h.origin)}><svg viewBox="0 0 100 100" aria-hidden="true"><ShogiPiece kind={h.kind} first={side.first_mover} rotated={side.key !== orientation.side.key} /></svg><span className="hand-count">{h.count}</span></button>)}
      </div>
    </>;
  }
  const width = columns * 100 + 56, height = rows * 100 + 56;
  const destinationIndex = squares.indexOf(interaction.completionTarget);
  const board = <div className="shogi-board-frame">
      <svg className="shogi-board" viewBox={`0 0 ${width} ${height}`} role="group" aria-label={`Shogi board, ${orientation.side.label} at bottom`}>
        <defs><linearGradient id="shogi-wood" x1="0" x2="1"><stop stopColor="#e4c38d" /><stop offset="0.5" stopColor="#ebd09d" /><stop offset="1" stopColor="#dfbc82" /></linearGradient></defs>
        <rect className="shogi-field" width={width} height={height} rx="7" />
        {squares.map((square, index) => {
          const row = Math.floor(index / columns), column = index % columns;
          const piece = snapshot.pieces.find((p) => p.square === square), target = destinations.includes(square);
          return <g key={square} className={`shogi-square${selected === square && !inspection && !inspectionPending ? " selected" : ""}${target && !inspection ? " target" : ""}`} data-square={square}
            transform={`translate(${28 + column * 100},${28 + row * 100})`} ref={(node) => interaction.register(square, node)}
            role="button" tabIndex={(squares.includes(focused) ? focused : snapshot.board.initial_focus) === square ? 0 : -1} aria-disabled={interaction.interactionDisabled} aria-pressed={selected === square}
            aria-label={`${piece ? pieceName(piece, snapshot.sides) : `${square}, empty`}${selected === square ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}${prefix.includes(square) ? ", route step" : ""}`}
            onFocus={() => interaction.onFocus(square)} onClick={() => interaction.select(square)} onKeyDown={(event) => interaction.keyDown(event, square)}>
            <rect className="shogi-cell" width="100" height="100" />
            <rect className="shogi-selection-ring" x="5" y="5" width="90" height="90" rx="3" />
            {piece && <g transform="translate(5 3) scale(.9)"><ShogiPiece kind={piece.kind} promoted={piece.promoted} first={snapshot.sides.find((s) => s.key === piece.owner)?.first_mover ?? true} rotated={piece.owner !== orientation.side.key} /></g>}
            {target && !inspection && <g className="shogi-destination" aria-hidden="true"><path d="M9 28V9H28 M72 9H91V28 M91 72V91H72 M28 91H9V72" />{!piece && <circle cx="50" cy="50" r="9" />}</g>}
            {inspection?.origin === square && <rect className="inspection-origin-ring" x="5" y="5" width="90" height="90" rx="3" aria-hidden="true" />}
            {target && inspection && <circle className="inspection-marker" cx="50" cy="50" r={piece ? "46" : "19"} aria-hidden="true" />}
            <rect className="shogi-focus-ring" x="3" y="3" width="94" height="94" rx="3" />
          </g>;
        })}
        {orientation.files.map((file, i) => <text className="shogi-coordinate" key={file} x={78 + i * 100} y={height - 7} textAnchor="middle" aria-hidden="true">{file}</text>)}
        {orientation.ranks.map((rank, i) => <text className="shogi-coordinate" key={rank} x={width - 13} y={87 + i * 100} textAnchor="middle" aria-hidden="true">{rank}</text>)}
      </svg>
      {completions.length > 0 && <div ref={interaction.completionRef} role="dialog" aria-modal="true" aria-label="Choose promotion" className="shogi-promotion" style={{ top: `${Math.floor(destinationIndex / columns) < rows / 2 ? 28 : 54}%` }} onKeyDown={interaction.completionKey}><strong>Promotion</strong><p>Choose how this piece completes its move.</p><div>{completions.map((c) => <button key={c.action} className="primary" onClick={() => interaction.chooseCompletion(c.action)}>{c.completion?.label ?? "Complete move"}</button>)}</div><button className="quiet" onClick={interaction.cancel}>Cancel promotion</button></div>}
    </div>;
  const footer = <>
    <div className={`shogi-board-help${inspection || inspectionPending ? " movement-inspection-status" : ""}`}><span>{inspection ? inspectionStatus(inspection.label) : inspectionPending ? "Loading read-only movement inspection…" : "Select a piece or hand piece, then a marked square."}</span><span>Arrows · Enter · Esc</span></div>
    {continuation && <div className="shogi-route" onKeyDown={(e) => { if (e.key === "Escape") { e.preventDefault(); interaction.cancel(); } }}><p>{announcement}</p>{terminals.length > 0 && !inspection && <button onClick={interaction.completeHere}>Complete action here</button>}<button onClick={interaction.cancel}>Cancel selection</button></div>}
    <span className="sr-only" role="status" aria-live="polite">{announcement}</span>
  </>;
  return <GameFrame className="shogi-stage" snapshot={snapshot} bottom={orientation.side} upperContent={tray(topSide)} board={board}
    lowerContent={tray(orientation.side)} help={footer}
    onFocusCapture={interaction.onFocusCapture}
    onBlurCapture={interaction.onBlurCapture} />;
}
