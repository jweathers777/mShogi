import type { FocusEvent, FocusEventHandler, ReactNode, Ref } from "react";
import { documentState } from "../documentState";
import "./shell.css";

export function DocumentState({ document }: { document: { path: string | null; dirty: boolean } }) {
  const state = documentState(document);
  return <span className={`document-dirty${state.kind !== "unmodified" ? " dirty" : ""}`} data-document-state={state.kind}>{state.label}</span>;
}

export function NativeEnginePanel({ status, limits, children }: {
  status: string; limits: { depth: number; nodes: number }; children: ReactNode;
}) {
  return <section className="card engine-card">
    <div className="card-title"><h2>Engine</h2><span className="engine-phase" role="status" aria-label="Engine turn status">{status}</span></div>
    <p className="caption" data-play-limits>Play limit: depth {limits.depth} · {limits.nodes.toLocaleString()} nodes</p>
    {children}
  </section>;
}

export function GamePositionStatus({ historical, concluded, paused, humanTurn, thinking, ply }: {
  historical: boolean; concluded: boolean; paused: boolean; humanTurn: boolean; thinking: boolean; ply: number;
}) {
  return <div className="game-footnote" data-position-status>
    <span>{historical ? "History · return to live to play" : concluded ? "Game concluded" : paused ? "Play paused · resume explicitly to continue" : humanTurn ? "Your turn" : thinking ? "mShogi is searching within fixed work limits" : "Engine turn paused"}</span>
    <span>{historical ? "Selected position" : "Live position"} · {ply} {ply === 1 ? "ply" : "plies"}</span>
  </div>;
}

export function LiveGameHeading({ eyebrow, status, player, flip, statusRef }: {
  eyebrow: string; status: string; player: ReactNode; flip: ReactNode; statusRef?: Ref<HTMLHeadingElement>;
}) {
  return <div className="shell-live-heading board-heading shogi-status-line">
    <div><p className="eyebrow">{eyebrow}</p><h2 ref={statusRef} role="status" aria-live="polite" tabIndex={-1} data-game-status>{status}</h2><p className="player-line">{player}</p></div>{flip}
  </div>;
}

export function PlayerArea({ side, human, toMove, playerKey, className = "", children }: {
  side: string; human: boolean; toMove: boolean; playerKey?: string; className?: string; children?: ReactNode;
}) {
  return <div className={`shell-player ${className}${toMove ? " to-move" : ""}`} data-player={playerKey}>
    <div className="shell-player-identity shogi-player-identity"><span className="player-avatar" aria-hidden="true">{human ? "Y" : "m"}</span><div><strong>{human ? "You" : "mShogi"}<span>{side}</span></strong><small>{human ? "Human player" : "Your opponent"}</small></div>{toMove && <span className="turn-indicator">To move</span>}</div>
    {children}
  </div>;
}

// Both games supply native content to these same physical slots. Board selection
// state remains inside its game adapter; this component owns only composition.
export function GameArea({ upperPlayer, board, lowerPlayer, footer, className = "", onFocusCapture, onBlurCapture }: {
  upperPlayer: ReactNode; board: ReactNode; lowerPlayer: ReactNode; footer: ReactNode; className?: string;
  onFocusCapture?: FocusEventHandler<HTMLDivElement>; onBlurCapture?: FocusEventHandler<HTMLDivElement>;
}) {
  return <div className={`shell-game-area ${className}`} onFocusCapture={onFocusCapture} onBlurCapture={onBlurCapture}>
    <div className="shell-player-slot" data-game-slot="upper-player">{upperPlayer}</div>
    <div className="shell-board-slot" data-game-slot="board">{board}</div>
    <div className="shell-player-slot" data-game-slot="lower-player">{lowerPlayer}</div>
    <div className="shell-game-footer" data-game-slot="footer">{footer}</div>
  </div>;
}

export function InspectorSwitch({ value, onChange, analysisDisabled = false }: { value: "play" | "analysis"; onChange: (value: "play" | "analysis") => void; analysisDisabled?: boolean }) {
  return <div className="inspector-switch" role="group" aria-label="Inspector view">
    <button aria-pressed={value === "play"} onClick={() => onChange("play")}>Play details</button>
    <button aria-pressed={value === "analysis"} disabled={analysisDisabled} onClick={() => onChange("analysis")}>Analyze position</button>
  </div>;
}

export function HistoricalNotice({ action, children, className = "" }: { action: ReactNode; children: ReactNode; className?: string }) {
  return <section className={`notice historical-notice shell-history-notice ${className}`}>
    <div className="shell-history-notice-heading"><strong>Historical position</strong>{action}</div>
    <div className="shell-history-notice-content">{children}</div>
  </section>;
}

export function AnalysisActions({ children }: { children: ReactNode }) {
  return <div className="shell-analysis-actions">{children}</div>;
}

export function ApplicationShell({ game, runtime, document, actions, heading, board, positionNotice, notices, analysis, history, gameActions, support, overlays, blocked, historical, attributes = {}, onFocusCapture }: {
  game: "chess" | "shogi" | "xiangqi"; runtime: string; document: ReactNode; actions: ReactNode;
  heading: ReactNode; board: ReactNode; positionNotice?: ReactNode; notices?: ReactNode; analysis: ReactNode;
  history: ReactNode; gameActions: ReactNode; support: ReactNode; overlays?: ReactNode;
  blocked?: boolean; historical?: boolean; attributes?: Record<string, string | number | boolean>;
  onFocusCapture?: FocusEventHandler<HTMLElement>;
}) {
  return <main className={`game-shell shell-${game} ${game === "chess" ? "application" : "shogi-application"}`} data-theme="mshogi-dark" data-historical={historical} {...attributes} onFocusCapture={onFocusCapture}>
    <div className="shell-content" inert={blocked}>
      <header className="shell-header app-header shogi-header">
        <div className="product-brand brand"><span className="brand-mark" aria-hidden="true">歩</span><div><div className="brand-name"><h1>mShogi</h1>{support}</div><p>{game.toUpperCase()}</p><span className="shell-runtime" role="status">{runtime}</span></div></div>
        <div className="shell-document document-card" aria-label="Game file">{document}</div><div className="product-file-actions shell-file-actions">{actions}</div>
      </header>
      <div className="shell-workspace workspace shogi-workspace">
        <section className="shell-board-panel board-panel shogi-play" aria-label={historical ? game === "shogi" ? "Historical shogi position" : "Historical position" : game === "shogi" ? "Live shogi game" : "Live game"}>{heading}<div className="shell-position-notice">{positionNotice}</div>{board}</section>
        <aside className="shell-sidebar sidebar shogi-sidebar" aria-label="Game controls and inspection" onFocusCapture={revealSidebarFocus}>
          <div className="shell-notices">{notices}</div>
          <section className="shell-card shell-analysis" data-shell-section="analysis"><h2 className="shell-section-title">Analysis</h2>{analysis}</section>
          <section className="shell-card shell-actions" data-shell-section="actions"><h2 className="shell-section-title">Game Actions</h2>{gameActions}</section>
          <section className="shell-card shell-history" data-shell-section="history">{history}</section>
        </aside>
      </div>
    </div>{overlays}
  </main>;
}

// Non-board controls only. Reveal the entire existing outward focus ring through
// each nested scroll viewport, rather than hiding/insetting the indicator.
function revealSidebarFocus(event: FocusEvent<HTMLElement>) {
  if (!(event.target instanceof HTMLElement)) return;
  const target = event.target, margin = 8;
  for (let viewport = target.parentElement; viewport; viewport = viewport.parentElement) {
    const style = getComputedStyle(viewport);
    if (/^(auto|scroll)$/.test(style.overflowY)) {
      const frame = viewport.getBoundingClientRect(), rect = target.getBoundingClientRect();
      if (rect.top - margin < frame.top) viewport.scrollTop -= frame.top - rect.top + margin;
      else if (rect.bottom + margin > frame.bottom) viewport.scrollTop += rect.bottom + margin - frame.bottom;
    }
    if (viewport === event.currentTarget) break;
  }
}
