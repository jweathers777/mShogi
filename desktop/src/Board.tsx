import { useEffect, useLayoutEffect, useRef, useState, type KeyboardEvent } from "react";
import type { BoardPiece, Color, LegalChoice } from "./native";
import type { InspectMovement, MovementInspection } from "./product/movementInspection";
import "./movement-inspection.css";

export interface BoardGeometry {
  files: string[];
  ranks: string[];
  label: string;
  initialFocus: string;
}
export interface BoardChoice {
  action: string;
  from: string;
  steps: string[];
  completion: { label: string; glyph: string | null } | null;
}
export interface NativePiece { square: string; owner: string; kind: string; promoted: boolean; text: string; label: string }
export interface NativeHand { origin: string; owner: string; kind: string; count: number; text: string; label: string }
interface SelectionChoice extends BoardChoice { promotion: boolean }
interface Props {
  pieces: (BoardPiece | NativePiece)[];
  choices: LegalChoice[] | BoardChoice[];
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
export function Board({ pieces, choices: supplied, orientation, version, disabled,
  onPlay, geometry = chessGeometry, glyphs = {}, profile = "chess", nativeOrientation, hands = [], nativeFirstOwner, humanOwner, onInspect, inspectionDisabled = disabled, inspectionContextKey }: Props) {
  const squares = nativeOrientation ? nativeOrientation.ranks.flatMap((rank) => nativeOrientation.files.map((file) => `${file}${rank}`)) : displayedSquares(orientation, geometry);
  const columns = geometry.files.length;
  const rows = geometry.ranks.length;
  // Standard 8x8/9x9 boards fit the normal panel; the 16x16 fixture scrolls.
  const large = columns > 9 || rows > 9;
  const geometryKey = `${geometry.label}|${geometry.files.join(",")}|${geometry.ranks.join(",")}`;
  const choices: SelectionChoice[] = supplied.map((choice) => "steps" in choice
    ? { ...choice, promotion: false }
    : { action: choice.action, from: choice.from, steps: [choice.to],
        completion: choice.promotion === null ? null
          : { label: choice.promotion, glyph: chessGlyphs[choice.promotion] ?? null },
        promotion: choice.promotion !== null });
  const [selected, setSelected] = useState<string | null>(null);
  const [prefix, setPrefix] = useState<string[]>([]);
  const [focused, setFocused] = useState(geometry.initialFocus);
  const [dialogChoices, setDialogChoices] = useState<SelectionChoice[]>([]);
  const [announcement, setAnnouncement] = useState("");
  const [inspection, setInspection] = useState<MovementInspection | null>(null);
  const [inspectionPending, setInspectionPending] = useState(false);
  const inspectionRequest = useRef(0);
  const refs = useRef(new Map<string, SVGGElement | HTMLButtonElement>());
  const dialog = useRef<HTMLDivElement>(null);
  const returnFocus = useRef(geometry.initialFocus);
  useLayoutEffect(() => {
    inspectionRequest.current++; setInspection(null); setInspectionPending(false);
    setSelected(null); setPrefix([]); setDialogChoices([]); setAnnouncement("");
    setFocused((current) => squares.includes(current) ? current : geometry.initialFocus);
    return () => { inspectionRequest.current++; };
  // Native versions/profile and presentation geometry retire partial choices.
  }, [version, disabled, profile, geometryKey, orientation, nativeOrientation?.label, inspectionContextKey, onInspect, inspectionDisabled]);
  useEffect(() => {
    if (dialogChoices.length > 0)
      dialog.current?.querySelector<HTMLButtonElement>("button")?.focus();
  }, [dialogChoices]);
  const matchesPrefix = (choice: SelectionChoice, route = prefix) =>
    route.every((step, index) => choice.steps[index] === step);
  const activeChoices: SelectionChoice[] = inspection ? inspection.choices.map((choice) => ({ ...choice, promotion: false })) : disabled ? [] : choices;
  const candidates = activeChoices.filter((choice) => choice.from === selected && matchesPrefix(choice));
  const terminals = prefix.length > 0
    ? candidates.filter((choice) => choice.steps.length === prefix.length) : [];
  const hasContinuation = candidates.some((choice) => choice.steps.length > prefix.length);
  const promotionDialog = dialogChoices.length > 0 && dialogChoices.every((choice) => choice.promotion);
  function focusSquare(square: string) {
    setFocused(square);
    const element = refs.current.get(square);
    element?.focus();
    if (large) element?.scrollIntoView?.({ block: "nearest", inline: "nearest" });
  }
  function clearSelection(message = "Selection cleared.") {
    inspectionRequest.current++; setInspection(null); setInspectionPending(false);
    setSelected(null); setPrefix([]); setAnnouncement(message);
  }
  function closeDialog() {
    setDialogChoices([]); clearSelection(); focusSquare(returnFocus.current);
  }
  function commit(choice: SelectionChoice) {
    if (inspection || disabled || !choices.some((current) => current.action === choice.action)) return;
    setDialogChoices([]); clearSelection("Complete native action submitted.");
    focusSquare(returnFocus.current); onPlay(choice.action);
  }
  function complete(matches: SelectionChoice[], square: string) {
    returnFocus.current = square;
    if (matches.length === 1 && matches[0]!.completion === null) commit(matches[0]!);
    else setDialogChoices(matches);
  }
  function select(square: string) {
    if ((disabled && (inspectionDisabled || !onInspect)) || dialogChoices.length > 0) return;
    focusSquare(square);
    const piece = pieces.find((entry) => entry.square === square);
    const hand = hands.find((entry) => entry.origin === square && entry.count > 0);
    const owner = piece ? ("owner" in piece ? piece.owner : piece.color) : hand?.owner;
    const actionableOrigin = !disabled && (humanOwner === undefined || owner === humanOwner) && choices.some((choice) => choice.from === square);
    if (inspection && actionableOrigin) {
      clearSelection(); setSelected(square); setAnnouncement(`${square} selected. Choose a highlighted destination.`); return;
    }
    // Continuation takes precedence over toggling the origin: routes can return.
    const next = candidates.filter((choice) => choice.steps[prefix.length] === square);
    if (next.length > 0) {
      const route = [...prefix, square];
      setPrefix(route);
      const terminal = next.filter((choice) => choice.steps.length === route.length);
      const longer = next.some((choice) => choice.steps.length > route.length);
      if (longer) {
        setAnnouncement(`${selected} → ${route.join(" → ")} selected. Continue to a highlighted square${terminal.length > 0 && !inspection ? ", or choose Complete action here" : ""}. No action has been submitted.`);
      } else if (terminal.length > 0) {
        if (inspection) setAnnouncement(`Inspection route complete. ${terminal.map((choice) => choice.completion?.label ?? "Complete route").join(" or ")}. Read-only; no action submitted.`);
        else complete(terminal, square);
      }
      return;
    }
    if (square === selected) { clearSelection(); return; }
    if (actionableOrigin) {
      clearSelection(); setSelected(square); setPrefix([]);
      setAnnouncement(`${square} selected. Choose a highlighted destination.`);
    } else if (onInspect && !inspectionDisabled && (piece || hand)) {
      clearSelection(); setSelected(square); setInspectionPending(true);
      setAnnouncement("Loading read-only movement inspection.");
      const request = ++inspectionRequest.current;
      void onInspect(square).then((result) => {
        if (request !== inspectionRequest.current) return;
        setInspectionPending(false);
        if (result && result.origin === square) {
          setInspection(result); setAnnouncement(`Inspecting ${result.label}. Read-only movement choices; no action can be played.`);
        } else { setSelected(null); setAnnouncement("Movement inspection is unavailable for this context. Select the piece again to query the current position."); }
      });
    } else setAnnouncement(selected === null
      ? "Choose a piece with a native legal action."
      : "That destination is unavailable. Choose a highlighted square, or Escape to cancel.");
  }
  function key(event: KeyboardEvent<SVGGElement>, square: string) {
    const offsets: Record<string, number> = {
      ArrowLeft: -1, ArrowRight: 1, ArrowUp: -columns, ArrowDown: columns,
    };
    const offset = offsets[event.key];
    if (offset !== undefined) {
      event.preventDefault();
      const index = squares.indexOf(square), next = index + offset;
      if (next >= 0 && next < squares.length && (Math.abs(offset) === columns
        || Math.floor(next / columns) === Math.floor(index / columns))) focusSquare(squares[next]!);
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault(); select(square);
    } else if (event.key === "Escape") {
      event.preventDefault(); const origin = selected; clearSelection();
      if (origin !== null) focusSquare(origin);
    }
  }
  function dialogKey(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") { event.preventDefault(); closeDialog(); }
    if (event.key === "Tab") {
      const buttons = [...(dialog.current?.querySelectorAll<HTMLButtonElement>("button") ?? [])];
      const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
      event.preventDefault();
      buttons[(index + (event.shiftKey ? -1 : 1) + buttons.length) % buttons.length]?.focus();
    }
  }
  const displayFiles = nativeOrientation?.files ?? (orientation === "white" ? geometry.files : [...geometry.files].reverse());
  const displayRanks = nativeOrientation?.ranks ?? (orientation === "white" ? [...geometry.ranks].reverse() : geometry.ranks);
  const width = 40 + columns * 100, height = 40 + rows * 100;
  return (
    <div className={`board-wrap${large ? " large-board-wrap" : ""}`}>
      {hands.length > 0 && <div role="group" aria-label="Native hands">
        {hands.map((hand, index) => <button key={hand.origin} ref={(element) => { if (element) refs.current.set(hand.origin, element); else refs.current.delete(hand.origin); }}
          aria-label={`${hand.label}${inspection?.origin === hand.origin ? `, Inspecting ${inspection.label}` : ""}`} aria-pressed={selected === hand.origin}
          disabled={dialogChoices.length > 0 || hand.count <= 0 || ((disabled || !choices.some((choice) => choice.from === hand.origin)) && (!onInspect || inspectionDisabled))}
          onClick={() => select(hand.origin)} onKeyDown={(event) => {
            if (event.key === "Escape") { event.preventDefault(); clearSelection(); focusSquare(hand.origin); }
            if (event.key === "ArrowDown") { event.preventDefault(); focusSquare(geometry.initialFocus); }
            if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
              event.preventDefault();
              const direction = event.key === "ArrowLeft" ? -1 : 1;
              for (let step = 1; step < hands.length; step++) {
                const next = hands[(index + direction * step + hands.length) % hands.length]!;
                if (next.count > 0 && ((!disabled && choices.some((choice) => choice.from === next.origin)) || (onInspect && !inspectionDisabled))) { focusSquare(next.origin); break; }
              }
            }
          }}>{hand.label}</button>)}
      </div>}
      <div className="board-scroll">
        <svg className="chess-board" viewBox={`0 0 ${width} ${height}`} role="group"
          aria-label={`${geometry.label}, ${nativeOrientation?.label ?? orientation} at bottom`}>
          <rect width={width} height={height} rx="8" className="board-border" />
          {squares.map((square, index) => {
            const piece = pieces.find((piece) => piece.square === square);
            const row = Math.floor(index / columns), column = index % columns;
            const target = candidates.some((choice) => choice.steps[prefix.length] === square);
            const available = choices.some((choice) => choice.from === square);
            const label = `${piece?.label ?? `${square}, empty`}${selected === square ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected" : ""}${prefix.includes(square) ? ", selected route step" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}`;
            return (
              <g key={square} ref={(element) => {
                if (element) refs.current.set(square, element); else refs.current.delete(square);
              }} transform={`translate(${20 + column * 100}, ${20 + row * 100})`}
                role="button" tabIndex={focused === square ? 0 : -1} aria-label={label}
                aria-pressed={selected === square} aria-disabled={(disabled && (inspectionDisabled || !onInspect)) || dialogChoices.length > 0}
                className={`square ${(row + column) % 2 === 0 ? "light" : "dark"} ${selected === square && !inspection && !inspectionPending ? "selected" : ""} ${target && !inspection ? "target" : ""} ${available && !disabled ? "available" : ""}`}
                onFocus={() => setFocused(square)} onClick={() => select(square)}
                onKeyDown={(event) => key(event, square)}>
                <rect width="100" height="100" className="square-fill" />
                {target && !inspection && <circle cx="50" cy="50" r={piece ? "40" : "12"}
                  className={piece ? "capture-marker" : "move-marker"} />}
                {piece && <text x="50" y="74" textAnchor="middle" data-owner={"owner" in piece ? piece.owner : piece.color} className={`piece ${"owner" in piece ? (piece.owner === nativeFirstOwner ? "native-first-owner" : "native-second-owner") : piece.color}`} aria-hidden="true">
                  {"text" in piece ? piece.text : glyphs[piece.kind] ?? chessGlyphs[piece.kind] ?? "?"}
                </text>}
                {inspection?.origin === square && <rect className="inspection-origin-ring" x="7" y="7" width="86" height="86" rx="5" aria-hidden="true" />}
                {target && inspection && <circle cx="50" cy="50" r={piece ? "43" : "19"} className="inspection-marker" aria-hidden="true" />}
                {column === 0 && <text x="7" y="20" className="coordinate" aria-hidden="true">{displayRanks[row]}</text>}
                {row === rows - 1 && <text x="82" y="93" className="coordinate" aria-hidden="true">{displayFiles[column]}</text>}
                <rect x="3" y="3" width="94" height="94" className="focus-ring" />
              </g>
            );
          })}
        </svg>
      </div>
      <p className={`board-help${inspection || inspectionPending ? " movement-inspection-status" : ""}`}>{inspection ? `Inspecting ${inspection.label} · Read-only · ${inspection.choices.length ? "Hollow rings show movement choices" : "No native movement choices"} · Esc clears` : inspectionPending ? "Loading read-only movement inspection…" : "Select a piece, then a destination. Arrow keys move focus · Enter or Space selects · Escape cancels."}</p>
      {large && <p className="board-help">The large board scrolls to keep squares readable. Arrow keys bring the focused square into view.</p>}
      {selected !== null && prefix.length > 0 && hasContinuation && (
        <div className="route-selection" role="group" aria-label="Partial route selection"
          onKeyDown={(event) => {
            if (event.key === "Escape") {
              event.preventDefault(); const origin = selected; clearSelection(); focusSquare(origin);
            }
          }}>
          <p>Selected route: {selected} → {prefix.join(" → ")}. No action submitted.</p>
          {terminals.length > 0 && !inspection && <button disabled={disabled} onClick={() => complete(terminals, prefix[prefix.length - 1]!)}>Complete action here</button>}
          <button className="quiet" onClick={() => { const origin = selected; clearSelection(); focusSquare(origin); }}>Cancel selection</button>
        </div>
      )}
      <span className="sr-only" role="status">{announcement}</span>
      {dialogChoices.length > 0 && (
        <div className="modal-backdrop">
          <div className="promotion-dialog" role="dialog" aria-modal="true"
            aria-labelledby="complete-choice-title" ref={dialog} onKeyDown={dialogKey}>
            <p className="eyebrow">COMPLETE YOUR MOVE</p>
            <h2 id="complete-choice-title">{promotionDialog ? "Choose promotion" : "Choose complete action"}</h2>
            <p>The action is applied only after you choose.</p>
            <div className="promotion-choices">
              {dialogChoices.map((choice) => <button key={choice.action} onClick={() => commit(choice)}>
                {choice.completion?.glyph && <span aria-hidden="true">{choice.completion.glyph}</span>}
                {choice.completion?.label ?? choice.action}
              </button>)}
            </div>
            <button className="quiet" onClick={closeDialog}>{promotionDialog ? "Cancel promotion" : "Cancel selection"}</button>
          </div>
        </div>
      )}
    </div>
  );
}
