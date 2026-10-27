import type { FocusEventHandler, ReactNode, Ref } from "react";
import "./shell.css";

export function LiveGameHeading({ eyebrow, status, player, flip, statusRef }: {
  eyebrow: string; status: string; player: ReactNode; flip: ReactNode; statusRef?: Ref<HTMLHeadingElement>;
}) {
  return <div className="shell-live-heading board-heading shogi-status-line"><div><p className="eyebrow shogi-eyebrow">{eyebrow}</p><h2 ref={statusRef} role="status" aria-live="polite" tabIndex={-1} data-game-status>{status}</h2><p className="player-line">{player}</p></div>{flip}</div>;
}
export function ApplicationShell({ game, runtime, document, actions, heading, board, notices, analysis, history, gameActions, technical, overlays, blocked, historical, attributes = {}, onFocusCapture }: {
  game: "chess" | "shogi"; runtime: string; document: ReactNode; actions: ReactNode;
  heading: ReactNode; board: ReactNode; notices?: ReactNode; analysis: ReactNode;
  history: ReactNode; gameActions: ReactNode; technical: ReactNode; overlays?: ReactNode;
  blocked?: boolean; historical?: boolean; attributes?: Record<string, string | number | boolean>;
  onFocusCapture?: FocusEventHandler<HTMLElement>;
}) {
  return <main className={`game-shell shell-${game} ${game === "chess" ? "application" : "shogi-application"}`} data-historical={historical} {...attributes} onFocusCapture={onFocusCapture}>
    <div className="shell-content" inert={blocked}>
      <header className="shell-header app-header shogi-header"><div className="product-brand brand"><span className="brand-mark" aria-hidden="true">{game === "chess" ? "♜" : "将"}</span><div><h1>mShogi</h1><p>{game === "chess" ? "CHESS" : "SHOGI"} · UNTIMED</p><span className="shell-runtime" role="status">{runtime}</span></div></div><div className="shell-document document-card" aria-label="Game file">{document}</div><div className="product-file-actions shell-file-actions">{actions}</div></header>
      <div className="shell-workspace workspace shogi-workspace"><section className="shell-board-panel board-panel shogi-play" aria-label={historical ? game === "shogi" ? "Historical shogi position" : "Historical position" : game === "shogi" ? "Live shogi game" : "Live game"}>{heading}{board}</section>
        <aside className="shell-sidebar sidebar shogi-sidebar" aria-label="Game controls and inspection"><div className="shell-notices">{notices}</div><section className="shell-analysis" data-shell-section="analysis"><h2 className="shell-section-title">Analysis</h2>{analysis}</section><section className="shell-history" data-shell-section="history">{history}</section><section className="shell-actions" data-shell-section="actions"><h2 className="shell-section-title">Game actions</h2>{gameActions}</section><details className="shell-technical shogi-technical"><summary>About &amp; technical details</summary>{technical}</details></aside>
      </div>
    </div>{overlays}
  </main>;
}
