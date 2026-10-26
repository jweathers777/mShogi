import { useLayoutEffect, useRef, useState, type KeyboardEvent } from "react";
import type { BoardChoice, NativePiece } from "../Board";
import type { GameSnapshot, Side } from "./native";

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
export function ShogiBoard({ snapshot, disabled, onPlay }: { snapshot: GameSnapshot; disabled: boolean; onPlay: (action: string) => void }) {
  const bottom = snapshot.document.settings.settings.orientation;
  const orientation = snapshot.orientations.find((o) => o.side.id.side === bottom) ?? snapshot.orientations[0]!;
  const topSide = snapshot.sides.find((s) => s.key !== orientation.side.key)!;
  const squares = orientation.ranks.flatMap((rank) => orientation.files.map((file) => `${file}${rank}`));
  const columns = orientation.files.length, rows = orientation.ranks.length;
  const [selected, setSelected] = useState<string | null>(null), [prefix, setPrefix] = useState<string[]>([]);
  const [focused, setFocused] = useState(snapshot.board.initial_focus);
  const [completions, setCompletions] = useState<BoardChoice[]>([]);
  const [announcement, setAnnouncement] = useState("");
  const refs = useRef(new Map<string, SVGGElement | HTMLButtonElement>());
  const chooser = useRef<HTMLDivElement>(null), lastDestination = useRef(snapshot.board.initial_focus);
  const hadFocus = useRef(false);
  useLayoutEffect(() => {
    setSelected(null); setPrefix([]); setCompletions([]); setAnnouncement("");
    if (hadFocus.current) refs.current.get(lastDestination.current)?.focus();
  }, [snapshot.token.document, snapshot.token.selection, snapshot.token.version, orientation.side.key, disabled]);
  useLayoutEffect(() => { if (completions.length) chooser.current?.querySelector<HTMLButtonElement>("button")?.focus(); }, [completions]);
  const candidates = snapshot.choices.filter((c) => c.from === selected && prefix.every((step, i) => c.steps[i] === step));
  const destinations = candidates.map((c) => c.steps[prefix.length]);
  const terminal = prefix.length ? candidates.filter((c) => c.steps.length === prefix.length) : [];
  const continuation = prefix.length > 0 && candidates.some((c) => c.steps.length > prefix.length);
  function focus(key: string) { setFocused(key); refs.current.get(key)?.focus(); }
  function clear(message = "Selection cancelled.") { setSelected(null); setPrefix([]); setCompletions([]); setAnnouncement(message); }
  function cancel() { const target = selected ?? lastDestination.current; clear(); focus(target); }
  function commit(choice: BoardChoice) {
    if (disabled || !snapshot.choices.some((c) => c.action === choice.action)) return;
    clear("Move submitted."); focus(lastDestination.current); onPlay(choice.action);
  }
  function finish(choices: BoardChoice[]) {
    // A single native complete action includes compulsory promotion. Optional
    // promotion supplies two distinct actions and gets an explicit chooser.
    if (choices.length === 1) commit(choices[0]!); else setCompletions(choices);
  }
  function select(key: string) {
    if (disabled || completions.length) return;
    hadFocus.current = true; focus(key);
    const next = candidates.filter((c) => c.steps[prefix.length] === key);
    if (next.length) {
      lastDestination.current = key;
      const route = [...prefix, key]; setPrefix(route);
      if (next.some((c) => c.steps.length > route.length)) {
        setAnnouncement("Route selected. Continue to a marked square or complete here; no move has been played.");
      } else finish(next.filter((c) => c.steps.length === route.length));
    } else if (selected === key) clear();
    else if (snapshot.choices.some((c) => c.from === key)) { setSelected(key); setPrefix([]); setAnnouncement("Piece selected. Choose a marked legal destination."); }
    else setAnnouncement("Choose a piece or one of its marked destinations. Escape cancels.");
  }
  function boardKey(e: KeyboardEvent<SVGGElement>, square: string) {
    const offsets: Record<string, number> = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -columns, ArrowDown: columns };
    const offset = offsets[e.key];
    if (offset !== undefined) {
      e.preventDefault(); const index = squares.indexOf(square), next = index + offset;
      if (next >= 0 && next < squares.length && (Math.abs(offset) === columns || Math.floor(next / columns) === Math.floor(index / columns))) focus(squares[next]!);
    } else if (e.key === "Enter" || e.key === " ") { e.preventDefault(); select(square); }
    else if (e.key === "Escape") { e.preventDefault(); cancel(); }
  }
  function tray(side: Side) {
    const hands = snapshot.hands.filter((h) => h.owner === side.key && h.count > 0);
    const human = side.key === snapshot.human.key;
    return <div className={`shogi-player${snapshot.turn.key === side.key && !snapshot.status.concluded ? " to-move" : ""}`} data-player={side.id.side}>
      <div className="shogi-player-identity"><span className="player-avatar" aria-hidden="true">{human ? "Y" : "m"}</span><div><strong>{human ? "You" : "mShogi"}<span>{side.label}</span></strong><small>{human ? "Human player" : "Your opponent"}</small></div>{snapshot.turn.key === side.key && !snapshot.status.concluded && <span className="turn-indicator">To move</span>}</div>
      <div className="shogi-hand" role="group" aria-label={`${side.label} pieces in hand`}>
        {!hands.length && <span className="empty-hand">No pieces in hand</span>}
        {hands.map((h) => <button key={h.origin} className={`hand-piece${selected === h.origin ? " selected" : ""}`} ref={(node) => { if (node) refs.current.set(h.origin, node); else refs.current.delete(h.origin); }}
          disabled={disabled || completions.length > 0 || !snapshot.choices.some((c) => c.from === h.origin)}
          data-hand={h.kind} data-hand-piece={h.kind} data-owner={side.id.side} aria-label={`${side.label} ${h.kind} in hand, ${h.count}${selected === h.origin ? ", selected for drop" : ""}`} aria-pressed={selected === h.origin}
          onClick={() => select(h.origin)} onKeyDown={(e) => {
            if (e.key === "Escape") { e.preventDefault(); cancel(); }
            else if (e.key === "ArrowDown" || e.key === "ArrowUp") { e.preventDefault(); focus(snapshot.board.initial_focus); }
            else if (e.key === "ArrowRight" || e.key === "ArrowLeft") { e.preventDefault(); const available = hands.filter((hand) => snapshot.choices.some((c) => c.from === hand.origin)); const index = available.findIndex((hand) => hand.origin === h.origin); const next = available[(index + (e.key === "ArrowRight" ? 1 : -1) + available.length) % available.length]; if (next) focus(next.origin); }
          }}><svg viewBox="0 0 100 100" aria-hidden="true"><ShogiPiece kind={h.kind} first={side.first_mover} rotated={side.key !== orientation.side.key} /></svg><span className="hand-count">{h.count}</span></button>)}
      </div>
    </div>;
  }
  const width = columns * 100 + 56, height = rows * 100 + 56;
  const destinationIndex = squares.indexOf(lastDestination.current);
  return <div className="shogi-stage" onFocusCapture={() => { hadFocus.current = true; }} onBlurCapture={(e) => { if (!e.currentTarget.contains(e.relatedTarget as Node | null)) hadFocus.current = false; }}>
    {tray(topSide)}
    <div className="shogi-board-frame">
      <svg className="shogi-board" viewBox={`0 0 ${width} ${height}`} role="group" aria-label={`Shogi board, ${orientation.side.label} at bottom`}>
        <defs><linearGradient id="shogi-wood" x1="0" x2="1"><stop stopColor="#e4c38d" /><stop offset="0.5" stopColor="#ebd09d" /><stop offset="1" stopColor="#dfbc82" /></linearGradient></defs>
        <rect className="shogi-field" width={width} height={height} rx="7" />
        {squares.map((square, index) => {
          const row = Math.floor(index / columns), column = index % columns;
          const piece = snapshot.pieces.find((p) => p.square === square), target = destinations.includes(square);
          return <g key={square} className={`shogi-square${selected === square ? " selected" : ""}${target ? " target" : ""}`} data-square={square}
            transform={`translate(${28 + column * 100},${28 + row * 100})`} ref={(node) => { if (node) refs.current.set(square, node); else refs.current.delete(square); }}
            role="button" tabIndex={(squares.includes(focused) ? focused : snapshot.board.initial_focus) === square ? 0 : -1} aria-disabled={disabled || completions.length > 0} aria-pressed={selected === square}
            aria-label={`${piece ? pieceName(piece, snapshot.sides) : `${square}, empty`}${selected === square ? ", selected" : ""}${target ? ", legal destination" : ""}${prefix.includes(square) ? ", route step" : ""}`}
            onFocus={() => setFocused(square)} onClick={() => select(square)} onKeyDown={(e) => boardKey(e, square)}>
            <rect className="shogi-cell" width="100" height="100" />
            <rect className="shogi-selection-ring" x="5" y="5" width="90" height="90" rx="3" />
            {piece && <g transform="translate(5 3) scale(.9)"><ShogiPiece kind={piece.kind} promoted={piece.promoted} first={snapshot.sides.find((s) => s.key === piece.owner)?.first_mover ?? true} rotated={piece.owner !== orientation.side.key} /></g>}
            {target && <g className="shogi-destination" aria-hidden="true"><path d="M9 28V9H28 M72 9H91V28 M91 72V91H72 M28 91H9V72" />{!piece && <circle cx="50" cy="50" r="9" />}</g>}
            <rect className="shogi-focus-ring" x="3" y="3" width="94" height="94" rx="3" />
          </g>;
        })}
        {orientation.files.map((file, i) => <text className="shogi-coordinate" key={file} x={78 + i * 100} y={height - 7} textAnchor="middle" aria-hidden="true">{file}</text>)}
        {orientation.ranks.map((rank, i) => <text className="shogi-coordinate" key={rank} x={width - 13} y={87 + i * 100} textAnchor="middle" aria-hidden="true">{rank}</text>)}
      </svg>
      {completions.length > 0 && <div ref={chooser} role="dialog" aria-modal="true" aria-label="Choose promotion" className="shogi-promotion" style={{ top: `${Math.floor(destinationIndex / columns) < rows / 2 ? 28 : 54}%` }} onKeyDown={(e) => {
        if (e.key === "Escape") { e.preventDefault(); cancel(); }
        if (e.key === "Tab") { const buttons = [...(chooser.current?.querySelectorAll<HTMLButtonElement>("button") ?? [])]; const index = buttons.indexOf(document.activeElement as HTMLButtonElement); e.preventDefault(); buttons[(index + (e.shiftKey ? -1 : 1) + buttons.length) % buttons.length]?.focus(); }
      }}><strong>Promotion</strong><p>Choose how this piece completes its move.</p><div>{completions.map((c) => <button key={c.action} className="primary" onClick={() => commit(c)}>{c.completion?.label ?? "Complete move"}</button>)}</div><button className="quiet" onClick={cancel}>Cancel promotion</button></div>}
    </div>
    {tray(orientation.side)}
    <div className="shogi-board-help"><span>Select a piece or hand piece, then a marked square.</span><span>Arrows · Enter · Esc</span></div>
    {continuation && <div className="shogi-route"><p>{announcement}</p>{terminal.length > 0 && <button onClick={() => finish(terminal)}>Complete action here</button>}<button onClick={cancel}>Cancel selection</button></div>}
    <span className="sr-only" role="status" aria-live="polite">{announcement}</span>
  </div>;
}
