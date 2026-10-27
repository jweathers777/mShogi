import type { FocusEventHandler, ReactNode, Ref } from "react";
import "./shell.css";

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

export function InspectorSwitch({ value, onChange }: { value: "play" | "analysis"; onChange: (value: "play" | "analysis") => void }) {
  return <div className="inspector-switch" role="group" aria-label="Inspector view">
    <button aria-pressed={value === "play"} onClick={() => onChange("play")}>Play details</button>
    <button aria-pressed={value === "analysis"} onClick={() => onChange("analysis")}>Analyze position</button>
  </div>;
}

export function HistoricalNotice({ action, children, className = "" }: { action: ReactNode; children: ReactNode; className?: string }) {
  return <section className={`notice historical-notice shell-history-notice ${className}`}>
    <div className="shell-history-notice-heading"><strong>Viewing history · read only</strong>{action}</div>
    <div className="shell-history-notice-content">{children}</div>
  </section>;
}

export function AnalysisActions({ children }: { children: ReactNode }) {
  return <div className="shell-analysis-actions">{children}</div>;
}

export function ApplicationShell({ game, runtime, document, actions, heading, board, positionNotice, notices, analysis, history, gameActions, technical, overlays, blocked, historical, attributes = {}, onFocusCapture }: {
  game: "chess" | "shogi"; runtime: string; document: ReactNode; actions: ReactNode;
  heading: ReactNode; board: ReactNode; positionNotice?: ReactNode; notices?: ReactNode; analysis: ReactNode;
  history: ReactNode; gameActions: ReactNode; technical: ReactNode; overlays?: ReactNode;
  blocked?: boolean; historical?: boolean; attributes?: Record<string, string | number | boolean>;
  onFocusCapture?: FocusEventHandler<HTMLElement>;
}) {
  return <main className={`game-shell shell-${game} ${game === "chess" ? "application" : "shogi-application"}`} data-theme="mshogi-dark" data-historical={historical} {...attributes} onFocusCapture={onFocusCapture}>
    <div className="shell-content" inert={blocked}>
      <header className="shell-header app-header shogi-header">
        <div className="product-brand brand"><span className="brand-mark" aria-hidden="true">♜</span><div><h1>mShogi</h1><p>{game === "chess" ? "CHESS" : "SHOGI"} · UNTIMED</p><span className="shell-runtime" role="status">{runtime}</span></div></div>
        <div className="shell-document document-card" aria-label="Game file">{document}</div><div className="product-file-actions shell-file-actions">{actions}</div>
      </header>
      <div className="shell-workspace workspace shogi-workspace">
        <section className="shell-board-panel board-panel shogi-play" aria-label={historical ? game === "shogi" ? "Historical shogi position" : "Historical position" : game === "shogi" ? "Live shogi game" : "Live game"}>{heading}<div className="shell-position-notice">{positionNotice}</div>{board}</section>
        <aside className="shell-sidebar sidebar shogi-sidebar" aria-label="Game controls and inspection">
          <div className="shell-notices">{notices}</div>
          <section className="shell-card shell-analysis" data-shell-section="analysis"><h2 className="shell-section-title">Analysis</h2>{analysis}</section>
          <section className="shell-card shell-history" data-shell-section="history">{history}</section>
          <section className="shell-card shell-actions" data-shell-section="actions"><h2 className="shell-section-title">Game actions</h2>{gameActions}</section>
          <details className="shell-technical shogi-technical"><summary>About &amp; technical details</summary>{technical}</details>
        </aside>
      </div>
    </div>{overlays}
  </main>;
}
