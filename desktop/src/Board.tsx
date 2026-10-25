import { useEffect, useLayoutEffect, useRef, useState, type KeyboardEvent } from "react";
import type { BoardPiece, Color, LegalChoice } from "./native";

interface Props {
  pieces: BoardPiece[];
  choices: LegalChoice[];
  orientation: Color;
  version: number;
  disabled: boolean;
  onPlay: (action: string) => void;
}
const glyphs: Record<string, string> = {
  king: "♚",
  queen: "♛",
  rook: "♜",
  bishop: "♝",
  knight: "♞",
  pawn: "♟",
};
export function displayedSquares(orientation: Color): string[] {
  const files = orientation === "white" ? "abcdefgh" : "hgfedcba";
  const ranks = orientation === "white" ? "87654321" : "12345678";
  return [...ranks].flatMap((rank) =>
    [...files].map((file) => `${file}${rank}`),
  );
}
export function Board({
  pieces,
  choices,
  orientation,
  version,
  disabled,
  onPlay,
}: Props) {
  const squares = displayedSquares(orientation);
  const [selected, setSelected] = useState<string | null>(null);
  const [focused, setFocused] = useState("e2");
  const [promotions, setPromotions] = useState<LegalChoice[]>([]);
  const [announcement, setAnnouncement] = useState("");
  const refs = useRef(new Map<string, SVGGElement>());
  const dialog = useRef<HTMLDivElement>(null);
  const returnFocus = useRef("e2");
  useLayoutEffect(() => {
    setSelected(null);
    setPromotions([]);
    setAnnouncement("");
  }, [version, disabled]);
  useEffect(() => {
    if (promotions.length > 0)
      dialog.current?.querySelector<HTMLButtonElement>("button")?.focus();
  }, [promotions]);
  const destinations = choices.filter((choice) => choice.from === selected);
  function focusSquare(square: string) {
    setFocused(square);
    refs.current.get(square)?.focus();
  }
  function closePromotion() {
    setPromotions([]);
    focusSquare(returnFocus.current);
  }
  function select(square: string) {
    if (disabled || promotions.length > 0) return;
    focusSquare(square);
    const matches = destinations.filter((choice) => choice.to === square);
    if (matches.length === 1) {
      const choice = matches[0]!;
      // A promotion remains explicit even if a profile supplies only one option.
      if (choice.promotion !== null) {
        returnFocus.current = square;
        setPromotions(matches);
      } else {
        setSelected(null);
        onPlay(choice.action);
      }
      return;
    }
    if (matches.length > 1) {
      returnFocus.current = square;
      setPromotions(matches);
      return;
    }
    if (square === selected) {
      setSelected(null);
      setAnnouncement("Selection cleared.");
      return;
    }
    if (choices.some((choice) => choice.from === square)) {
      setSelected(square);
      setAnnouncement(`${square} selected. Choose a highlighted destination.`);
    } else {
      setAnnouncement(
        selected === null
          ? "Choose a piece with a native legal action."
          : "That destination is unavailable. Choose a highlighted square, or Escape to cancel.",
      );
    }
  }
  function key(event: KeyboardEvent<SVGGElement>, square: string) {
    const offsets: Record<string, number> = {
      ArrowLeft: -1,
      ArrowRight: 1,
      ArrowUp: -8,
      ArrowDown: 8,
    };
    const offset = offsets[event.key];
    if (offset !== undefined) {
      event.preventDefault();
      const index = squares.indexOf(square);
      const next = index + offset;
      if (
        next >= 0 &&
        next < 64 &&
        (Math.abs(offset) === 8 ||
          Math.floor(next / 8) === Math.floor(index / 8))
      )
        focusSquare(squares[next]!);
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      select(square);
    } else if (event.key === "Escape") {
      event.preventDefault();
      setSelected(null);
      setAnnouncement("Selection cleared.");
    }
  }
  function promotionKey(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") {
      event.preventDefault();
      closePromotion();
    }
    if (event.key === "Tab") {
      const buttons = [
        ...(dialog.current?.querySelectorAll<HTMLButtonElement>("button") ??
          []),
      ];
      const index = buttons.indexOf(
        document.activeElement as HTMLButtonElement,
      );
      event.preventDefault();
      buttons[
        (index + (event.shiftKey ? -1 : 1) + buttons.length) % buttons.length
      ]?.focus();
    }
  }
  return (
    <div className="board-wrap">
      <svg
        className="chess-board"
        viewBox="0 0 840 840"
        role="group"
        aria-label={`Chess board, ${orientation} at bottom`}
      >
        <rect width="840" height="840" rx="8" className="board-border" />
        {squares.map((square, index) => {
          const piece = pieces.find((piece) => piece.square === square);
          const row = Math.floor(index / 8);
          const column = index % 8;
          const target = destinations.some((choice) => choice.to === square);
          const available = choices.some((choice) => choice.from === square);
          const label = `${piece?.label ?? `${square}, empty`}${selected === square ? ", selected" : ""}${target ? ", legal destination" : ""}`;
          return (
            <g
              key={square}
              ref={(element) => {
                if (element) refs.current.set(square, element);
                else refs.current.delete(square);
              }}
              transform={`translate(${20 + column * 100}, ${20 + row * 100})`}
              role="button"
              tabIndex={focused === square ? 0 : -1}
              aria-label={label}
              aria-pressed={selected === square}
              aria-disabled={disabled || promotions.length > 0}
              className={`square ${(row + column) % 2 === 0 ? "light" : "dark"} ${selected === square ? "selected" : ""} ${target ? "target" : ""} ${available && !disabled ? "available" : ""}`}
              onFocus={() => setFocused(square)}
              onClick={() => select(square)}
              onKeyDown={(event) => key(event, square)}
            >
              <rect width="100" height="100" className="square-fill" />
              {target && (
                <circle
                  cx="50"
                  cy="50"
                  r={piece ? "40" : "12"}
                  className={piece ? "capture-marker" : "move-marker"}
                />
              )}
              {piece && (
                <text
                  x="50"
                  y="74"
                  textAnchor="middle"
                  className={`piece ${piece.color}`}
                  aria-hidden="true"
                >
                  {glyphs[piece.kind] ?? "?"}
                </text>
              )}
              {column === 0 && (
                <text x="7" y="17" className="coordinate" aria-hidden="true">
                  {square[1]}
                </text>
              )}
              {row === 7 && (
                <text x="88" y="93" className="coordinate" aria-hidden="true">
                  {square[0]}
                </text>
              )}
              <rect x="3" y="3" width="94" height="94" className="focus-ring" />
            </g>
          );
        })}
      </svg>
      <p className="board-help">
        Select a piece, then a destination. Arrow keys move focus · Enter or
        Space selects · Escape cancels.
      </p>
      <span className="sr-only" role="status">
        {announcement}
      </span>
      {promotions.length > 0 && (
        <div className="modal-backdrop">
          <div
            className="promotion-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="promotion-title"
            ref={dialog}
            onKeyDown={promotionKey}
          >
            <p className="eyebrow">COMPLETE YOUR MOVE</p>
            <h2 id="promotion-title">Choose promotion</h2>
            <p>The move is applied after you choose a piece.</p>
            <div className="promotion-choices">
              {promotions.map((choice) => (
                <button
                  key={choice.action}
                  onClick={() => {
                    setPromotions([]);
                    setSelected(null);
                    focusSquare(returnFocus.current);
                    onPlay(choice.action);
                  }}
                >
                  <span aria-hidden="true">
                    {glyphs[choice.promotion ?? ""]}
                  </span>
                  {choice.promotion}
                </button>
              ))}
            </div>
            <button className="quiet" onClick={closePromotion}>
              Cancel promotion
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
