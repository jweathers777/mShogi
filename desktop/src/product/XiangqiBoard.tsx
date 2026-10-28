import { useId, useLayoutEffect, useRef, useState, type KeyboardEvent } from "react";
import type { XiangqiSnapshot as GameSnapshot, Side } from "./native";
import type { InspectMovement, MovementInspection } from "./movementInspection";
import { GameArea, GamePositionStatus, PlayerArea } from "./ApplicationShell";
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

export function XiangqiBoard({ snapshot, disabled, onPlay, onInspect, inspectionDisabled = disabled }: {
  snapshot: GameSnapshot; disabled: boolean; onPlay: (action: string) => void;
  onInspect?: InspectMovement; inspectionDisabled?: boolean;
}) {
  const id = useId().replaceAll(":", "");
  const topology = snapshot.board.intersections;
  const bottom = snapshot.document.settings.orientation;
  const orientation = snapshot.orientations.find((o) => o.side.id.side === bottom) ?? snapshot.orientations[0]!;
  const top = snapshot.sides.find((s) => s.key !== orientation.side.key)!;
  const points = orientation.ranks.flatMap((rank) => orientation.files.map((file) => `${file}${rank}`));
  const columns = orientation.files.length;
  const [selected, setSelected] = useState<string | null>(null);
  const [focused, setFocused] = useState(snapshot.board.initial_focus);
  const [inspection, setInspection] = useState<MovementInspection | null>(null);
  const [loading, setLoading] = useState(false), [announcement, setAnnouncement] = useState("");
  const query = useRef(0), refs = useRef(new Map<string, SVGGElement>());
  const hadFocus = useRef(false), lastDestination = useRef(snapshot.board.initial_focus);
  const context = JSON.stringify(snapshot.token);
  useLayoutEffect(() => {
    query.current++; setSelected(null); setInspection(null); setLoading(false); setAnnouncement("");
    if (hadFocus.current) refs.current.get(lastDestination.current)?.focus();
    return () => { query.current++; };
  }, [context, bottom, disabled, inspectionDisabled, onInspect]);
  if (!topology) return <p role="alert">The native intersection board is unavailable.</p>;

  const position = (point: string): [number, number] => {
    const index = points.indexOf(point);
    return [90 + (index % columns) * 100, 90 + Math.floor(index / columns) * 100];
  };
  const choices = inspection?.choices ?? (disabled ? [] : snapshot.choices);
  const candidates = choices.filter((choice) => choice.from === selected);
  const destinations = candidates.map((choice) => choice.steps[0]);
  function focus(point: string) { setFocused(point); refs.current.get(point)?.focus(); }
  function clear(message = "Selection cancelled.") {
    query.current++; setSelected(null); setInspection(null); setLoading(false); setAnnouncement(message);
  }
  function cancel() { const origin = selected ?? lastDestination.current; clear(); focus(origin); }
  function select(point: string) {
    if (disabled && (inspectionDisabled || !onInspect)) return;
    hadFocus.current = true; focus(point);
    const completions = candidates.filter((choice) => choice.steps.length === 1 && choice.steps[0] === point);
    // An enemy occupying an actionable destination remains a capture, not a mode switch.
    if (completions.length) {
      if (inspection) { setAnnouncement("Read-only inspection destination. No action was submitted."); return; }
      if (!disabled && completions.length === 1 && snapshot.choices.some((choice) => choice.action === completions[0]!.action)) {
        lastDestination.current = point; clear("Move submitted."); onPlay(completions[0]!.action);
      }
      return;
    }
    if (selected === point) { clear(); return; }
    const piece = snapshot.pieces.find((p) => p.square === point);
    if (!disabled && piece?.owner === snapshot.human.key && snapshot.choices.some((choice) => choice.from === point)) {
      clear(); setSelected(point); setAnnouncement("Piece selected. Choose a marked legal intersection.");
    } else if (piece && onInspect && !inspectionDisabled) {
      clear(); setSelected(point); setLoading(true); setAnnouncement("Loading read-only movement inspection.");
      const request = ++query.current;
      void onInspect(point).then((result) => {
        if (request !== query.current) return;
        setLoading(false);
        if (result && result.origin === point) {
          setInspection(result); setAnnouncement(`Inspecting ${result.label}. Read-only movement choices; no action can be played.`);
        } else { setSelected(null); setAnnouncement("Movement inspection is unavailable for this position."); }
      });
    } else setAnnouncement("Select a piece or one of its marked destinations. Escape cancels.");
  }
  function key(event: KeyboardEvent<SVGGElement>, point: string) {
    const offset = ({ ArrowLeft: -1, ArrowRight: 1, ArrowUp: -columns, ArrowDown: columns } as Record<string, number>)[event.key];
    if (offset !== undefined) {
      event.preventDefault(); const index = points.indexOf(point), next = index + offset;
      if (next >= 0 && next < points.length && (Math.abs(offset) === columns || Math.floor(index / columns) === Math.floor(next / columns))) focus(points[next]!);
    } else if (event.key === "Enter" || event.key === " ") { event.preventDefault(); select(point); }
    else if (event.key === "Escape") { event.preventDefault(); cancel(); }
  }
  function player(side: Side) {
    return <PlayerArea side={side.label} human={side.key === snapshot.human.key} playerKey={side.id.side}
      toMove={!snapshot.status.concluded && snapshot.turn.key === side.key} />;
  }
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
          aria-disabled={disabled && (inspectionDisabled || !onInspect)} aria-pressed={selected === point}
          aria-label={`${point}, ${piece ? `${owner?.label ?? "Player"} ${piece.kind}` : "empty"}${selected === point ? inspection ? `, Inspecting ${inspection.label}` : loading ? ", loading movement inspection" : ", selected" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}`}
          ref={(node) => { if (node) refs.current.set(point, node); else refs.current.delete(point); }}
          onFocus={() => setFocused(point)} onClick={() => select(point)} onKeyDown={(event) => key(event, point)}>
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
    </svg>
  </div>;
  const footer = <>
    <div className="shell-board-help"><div className={`board-help xiangqi-help${inspection || loading ? " movement-inspection-status" : ""}`}>
      <span>{inspection ? `Inspecting ${inspection.label} · Read-only · Hollow rings` : loading ? "Loading read-only movement inspection…" : "Select a piece, then a marked intersection."}</span><span>Arrows · Enter · Esc</span>
    </div></div>
    <GamePositionStatus historical={snapshot.historical} concluded={snapshot.status.concluded} paused={snapshot.paused}
      humanTurn={snapshot.turn.key === snapshot.human.key} thinking={snapshot.phase === "thinking" && snapshot.execution_purpose === "engine_turn"} ply={snapshot.board_ply} />
    <span className="sr-only" role="status" aria-live="polite">{announcement}</span>
  </>;
  return <GameArea className="xiangqi-stage" upperPlayer={player(top)} board={board} lowerPlayer={player(orientation.side)} footer={footer}
    onFocusCapture={() => { hadFocus.current = true; }}
    onBlurCapture={(event) => { if (!event.currentTarget.contains(event.relatedTarget as Node | null)) hadFocus.current = false; }} />;
}
