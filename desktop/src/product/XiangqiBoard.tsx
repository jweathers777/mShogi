import { useId } from "react";
import type { CompletedActionGeometry } from "../boardPresentation";
import { gridNeighbor, useNativeChoiceController } from "../interaction/useNativeChoiceController";
import type { XiangqiSnapshot as GameSnapshot } from "./native";
import { inspectionStatus, type InspectMovement } from "./movementInspection";
import { GameFrame } from "./GameFrame";
import { CompletedActionCue } from "../CompletedActionCue";
import "../movement-inspection.css";
import "./xiangqi.css";

// Artwork only: kind, owner, coordinates and complete choices remain native.
export function xiangqiGlyph(kind: string, red: boolean): string {
  const glyphs: Record<string, [string, string]> = {
    general: ["帥", "將"], advisor: ["仕", "士"], elephant: ["相", "象"],
    horse: ["傌", "馬"], rook: ["俥", "車"], cannon: ["炮", "砲"], soldier: ["兵", "卒"],
  };
  return glyphs[kind]?.[red ? 0 : 1] ?? "?";
}

export function XiangqiBoard({ snapshot, disabled, onPlay, onInspect, inspectionDisabled = disabled, completedAction }: {
  snapshot: GameSnapshot; disabled: boolean; onPlay: (action: string) => void;
  onInspect?: InspectMovement; inspectionDisabled?: boolean;
  // Separate read-only presentation input for MSH-123; never controller state.
  completedAction?: CompletedActionGeometry | null;
}) {
  const id = useId().replaceAll(":", "");
  const topology = snapshot.board.intersections;
  const bottom = snapshot.document.settings.orientation;
  const orientation = snapshot.orientations.find((o) => o.side.id.side === bottom) ?? snapshot.orientations[0]!;
  const points = orientation.ranks.flatMap((rank) => orientation.files.map((file) => `${file}${rank}`));
  const columns = orientation.files.length;
  const interaction = useNativeChoiceController({
    contextKey: JSON.stringify([snapshot.game, snapshot.profile, snapshot.edition, snapshot.token, bottom]),
    choices: snapshot.choices,
    origins: snapshot.pieces.map((piece) => ({ key: piece.square, owner: piece.owner, inspectable: true })),
    humanOwner: snapshot.human.key, disabled, inspectionDisabled, onInspect, onPlay,
    initialFocus: snapshot.board.initial_focus, boardKeys: points,
    neighbor: (key, direction) => gridNeighbor(points, columns, key, direction),
  });
  const { selected, prefix, focused, inspection, inspectionPending: loading, announcement, destinations, completions, continuation, terminals } = interaction;
  if (!topology) return <p role="alert">The native intersection board is unavailable.</p>;

  const position = (point: string): [number, number] => {
    const index = points.indexOf(point);
    return [90 + (index % columns) * 100, 90 + Math.floor(index / columns) * 100];
  };
  const riverYs = topology.river_between_ranks.map((rank) => 90 + orientation.ranks.indexOf(String(rank)) * 100).sort((a, b) => a - b);
  const riverMiddle = (riverYs[0]! + riverYs[1]!) / 2;
  const board = <div className="xiangqi-board-frame">
    <svg className="xiangqi-board" viewBox="0 0 980 1080" role="group" aria-label={`Xiangqi board, ${orientation.side.label} at bottom`}>
      <defs>
        <linearGradient id={`${id}-wood`} x2="1" y2=".3"><stop stopColor="#e7c890" /><stop offset=".45" stopColor="#ecca91" /><stop offset="1" stopColor="#d6ae70" /></linearGradient>
        <radialGradient id={`${id}-ivory`} cx=".35" cy=".25" r=".8"><stop stopColor="#fff5d9" /><stop offset=".7" stopColor="#f5e5be" /><stop offset="1" stopColor="#d6be8b" /></radialGradient>
        <filter id={`${id}-grain`}><feTurbulence type="fractalNoise" baseFrequency=".025 .5" numOctaves="2" seed="7" stitchTiles="stitch" /><feColorMatrix type="saturate" values="0" /></filter>
      </defs>
      <rect width="980" height="1080" rx="12" fill={`url(#${id}-wood)`} />
      <rect className="xiangqi-grain" width="980" height="1080" rx="12" filter={`url(#${id}-grain)`} />
      <rect className="xiangqi-frame-line" x="18" y="18" width="944" height="1044" rx="5" />
      <g className="xiangqi-grid" aria-hidden="true">
        {orientation.ranks.map((rank, row) => <line key={rank} x1="90" x2="890" y1={90 + row * 100} y2={90 + row * 100} />)}
        {orientation.files.map((file, column) => <g key={file}>{column === 0 || column === columns - 1
          ? <line x1={90 + column * 100} x2={90 + column * 100} y1="90" y2="990" />
          : <><line x1={90 + column * 100} x2={90 + column * 100} y1="90" y2={riverYs[0]} /><line x1={90 + column * 100} x2={90 + column * 100} y1={riverYs[1]} y2="990" /></>}</g>)}
        {topology.palaces.flatMap((palace) => palace.diagonals.map((diagonal, index) => <polyline key={`${palace.side.key}-${index}`} data-palace={palace.side.key} points={diagonal.map((point) => position(point).join(",")).join(" ")} />))}
      </g>
      <g className="xiangqi-river" aria-hidden="true"><text x="280" y={riverMiddle + 13} textAnchor="middle">楚河</text><text x="700" y={riverMiddle + 13} textAnchor="middle">漢界</text></g>
      {orientation.files.map((file, i) => <text className="xiangqi-coordinate" key={file} x={90 + i * 100} y="1060" textAnchor="middle" aria-hidden="true">{file}</text>)}
      {orientation.ranks.map((rank, i) => <text className="xiangqi-coordinate" key={rank} x="40" y={97 + i * 100} textAnchor="middle" aria-hidden="true">{rank}</text>)}
      {topology.points.map((point) => {
        const [x, y] = position(point), piece = snapshot.pieces.find((p) => p.square === point);
        const target = destinations.includes(point), owner = snapshot.sides.find((side) => side.key === piece?.owner);
        return <g key={point} data-intersection={point} data-square={point} transform={`translate(${x} ${y})`}
          className={`xiangqi-point${selected === point && !inspection && !loading ? " selected" : ""}`}
          role="button" tabIndex={(points.includes(focused) ? focused : snapshot.board.initial_focus) === point ? 0 : -1}
          aria-disabled={interaction.interactionDisabled} aria-pressed={selected === point}
          aria-label={`${point}, ${piece ? `${owner?.label ?? "Player"} ${piece.kind}` : "empty"}${selected === point ? inspection ? `, Inspecting ${inspection.label}` : loading ? ", loading movement inspection" : ", selected" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}${prefix.includes(point) ? ", route step" : ""}`}
          ref={(node) => interaction.register(point, node)}
          onFocus={() => interaction.onFocus(point)} onClick={() => interaction.select(point)} onKeyDown={(event) => interaction.keyDown(event, point)}>
          <circle className="xiangqi-hit" r="49" />
          {piece && <g className={`xiangqi-piece ${owner?.id.side === "red" ? "red" : "black"}`} data-kind={piece.kind} data-owner={owner?.id.side} aria-hidden="true">
            <circle className="xiangqi-piece-shadow" cy="4" r="42" /><circle className="xiangqi-rim" r="42" />
            <circle className="xiangqi-disc" r="39" fill={`url(#${id}-ivory)`} /><circle className="xiangqi-engraving" r="34" />
            <text textAnchor="middle" y="18">{xiangqiGlyph(piece.kind, owner?.id.side === "red")}</text>
          </g>}
          {selected === point && !inspection && !loading && <circle className="xiangqi-selected" r="46" aria-hidden="true" />}
          {target && !inspection && <g className="xiangqi-destination" aria-hidden="true">{piece ? <circle r="46" /> : <circle r="10" />}</g>}
          {inspection?.origin === point && <circle className="inspection-origin-ring" r="46" aria-hidden="true" />}
          {target && inspection && <circle className="inspection-marker" r={piece ? "47" : "19"} aria-hidden="true" />}
          <circle className="xiangqi-focus" r="49" aria-hidden="true" />
        </g>;
      })}
      <CompletedActionCue action={completedAction} center={key => points.includes(key) ? position(key) : null} radius={47} />
    </svg>
    {completions.length > 0 && <div ref={interaction.completionRef} role="dialog" aria-modal="true" aria-label="Choose complete action" className="shogi-promotion xiangqi-completion" style={{ top: `${Math.floor(points.indexOf(interaction.completionTarget) / columns) < orientation.ranks.length / 2 ? 28 : 54}%` }} onKeyDown={interaction.completionKey}>
      <strong>Complete action</strong><p>Choose the native action for this route.</p><div>{completions.map((choice) => <button key={choice.action} className="primary" onClick={() => interaction.chooseCompletion(choice.action)}>{choice.completion?.label ?? "Complete action"}</button>)}</div><button className="quiet" onClick={interaction.cancel}>Cancel selection</button>
    </div>}
  </div>;
  const footer = <>
    <div className="shell-board-help"><div className={`board-help xiangqi-help${inspection || loading ? " movement-inspection-status" : ""}`}>
      <span>{inspection ? inspectionStatus(inspection.label) : loading ? "Loading read-only movement inspection…" : "Select a piece, then a marked intersection."}</span><span>Arrows · Enter · Esc</span>
    </div></div>
    {continuation && <div className="shogi-route xiangqi-route" onKeyDown={(event) => { if (event.key === "Escape") { event.preventDefault(); interaction.cancel(); } }}><p>{announcement}</p>{terminals.length > 0 && !inspection && <button onClick={interaction.completeHere}>Complete action here</button>}<button onClick={interaction.cancel}>Cancel selection</button></div>}
    <span className="sr-only" role="status" aria-live="polite">{announcement}</span>
  </>;
  return <GameFrame className="xiangqi-stage" snapshot={snapshot} bottom={orientation.side} board={board} help={footer}
    onFocusCapture={interaction.onFocusCapture}
    onBlurCapture={interaction.onBlurCapture} />;
}
