import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { App } from "../App";
import type { Color } from "../native";
import { Dialog } from "./Dialog";
import { ShogiApp } from "./ShogiApp";
import { XiangqiApp } from "./XiangqiApp";
import { useGameSession } from "./useGameSession";
import type { GameKind, GameSnapshot, NativeSide, OpenToken } from "./native";
import "./shogi.css";
const games: { key: GameKind; label: string; description: string; first: string; second: string }[] = [
  { key: "chess", label: "Chess", description: "The classic 8 × 8 game", first: "White", second: "Black" },
  { key: "shogi", label: "Shogi", description: "The Japanese game of captured pieces", first: "Sente", second: "Gote" },
  { key: "xiangqi", label: "Xiangqi", description: "The Chinese game across the river", first: "Red", second: "Black" },
];
function radioKeys(event: KeyboardEvent<HTMLDivElement>) {
  const offsets: Record<string, number> = { ArrowLeft: -1, ArrowUp: -1, ArrowRight: 1, ArrowDown: 1 };
  const offset = offsets[event.key];
  if (offset === undefined) return;
  const buttons = [...event.currentTarget.querySelectorAll<HTMLButtonElement>('button[role="radio"]:not(:disabled)')];
  if (!buttons.length) return;
  event.preventDefault();
  const next = buttons[(buttons.indexOf(event.target as HTMLButtonElement) + offset + buttons.length) % buttons.length];
  next?.focus(); next?.click();
}
const same = (a: OpenToken, b: OpenToken) => a.document === b.document && a.version === b.version && a.document_revision === b.document_revision;
export function ProductApp() {
  const session = useGameSession(), { snapshot } = session;
  const [pending, setPending] = useState<{ intent: "new" | "open" | "quit"; source: GameSnapshot; color?: Color } | null>(null);
  const [choice, setChoice] = useState<{ source: GameSnapshot; game: GameKind; side: "first" | "second" | "random" } | null>(null);
  const [message, setMessage] = useState("");
  const deciding = useRef(false), handledClose = useRef(false);
  async function execute(intent: "new" | "open" | "quit", source: GameSnapshot, color?: Color) {
    if (intent === "new") { setChoice({ source, game: source.game, side: color ? color === "white" ? "first" : "second" : source.human.first_mover ? "first" : "second" }); return true; }
    if (intent === "open") return (await session.file(source, "open")) !== null;
    return session.finishClose(source);
  }
  function begin(intent: "new" | "open" | "quit", source = snapshot, color?: Color) {
    if (!source) return; setMessage("");
    if (source.document.dirty || intent === "quit") setPending({ intent, source, ...(color ? { color } : {}) });
    else void execute(intent, source, color);
  }
  useEffect(() => {
    if (!session.closeRequested) handledClose.current = false;
    if (session.closeRequested && !handledClose.current && snapshot) {
      handledClose.current = true; setChoice(null); begin("quit", snapshot);
    }
  }, [session.closeRequested, snapshot]);
  const stale = pending !== null && snapshot !== null && !same(pending.source.open_token, snapshot.open_token);
  async function cancelGuard() {
    if (deciding.current) return;
    if (pending?.intent === "quit" && !(await session.cancelClose())) return;
    setPending(null); setMessage("");
  }
  async function decide(save: boolean) {
    if (!pending || !snapshot || stale || deciding.current) return;
    deciding.current = true; setMessage("");
    try {
      let source = snapshot;
      if (save) {
        const saved = await session.file(source, "save");
        if (!saved || saved.document.dirty) { setMessage("The game was not fully saved. Nothing has been discarded; keep this game open to review the file operation."); return; }
        source = saved; setPending({ ...pending, source });
      }
      if (await execute(pending.intent, source, pending.color)) setPending(null);
    } finally { deciding.current = false; }
  }
  async function start() {
    if (!choice || !snapshot || session.busy || !same(choice.source.open_token, snapshot.open_token)) return;
    const first = choice.side === "random" ? (crypto.getRandomValues(new Uint8Array(1))[0]! & 1) === 0 : choice.side === "first";
    const human: NativeSide = choice.game === "chess" ? { game_kind: "chess", side: first ? "white" : "black" }
      : choice.game === "shogi" ? { game_kind: "shogi", side: first ? "sente" : "gote" }
      : { game_kind: "xiangqi", side: first ? "red" : "black" };
    const next = await session.newGame(choice.source.open_token, choice.game, human);
    if (next) setChoice(null);
  }
  if (!snapshot) return <main className="loading"><h1>mShogi</h1><p role="status">Opening your game…</p>{session.error && <p role="alert">The native application could not connect.</p>}</main>;
  const blocked = pending !== null || choice !== null;
  const selectedGame = games.find((g) => g.key === choice?.game) ?? games[0]!;
  return <><div inert={blocked}>{snapshot.game === "chess" ? <App externalCloseGuard sharedSnapshot={snapshot} onChooseGame={(color) => begin("new", snapshot, color)} /> : snapshot.game === "shogi" ? <ShogiApp snapshot={snapshot} session={session} onNewGame={() => begin("new")} onOpen={() => begin("open")} blocked={blocked} /> : <XiangqiApp snapshot={snapshot} session={session} onNewGame={() => begin("new")} onOpen={() => begin("open")} blocked={blocked} />}</div>
    {pending && <Dialog title={pending.source.document.dirty ? "Save your changes?" : "Close mShogi?"} className="document-guard" cancel={() => { if (!session.busy) void cancelGuard(); }}><p>{pending.source.document.dirty ? `Save this game before ${pending.intent === "new" ? "starting another game" : pending.intent === "open" ? "opening another game" : "closing mShogi"}, or explicitly discard its unsaved changes.` : "Your game is saved. You can close the application or keep playing."}</p>{stale && <p role="alert">The game changed. Cancel and review the current position.</p>}{message && <p role="alert">{message}</p>}<div className="dialog-actions"><button className="secondary" disabled={session.busy || stale || !snapshot.document.dirty} onClick={() => { void decide(true); }}>Save &amp; continue</button><button className="danger" disabled={session.busy || stale} onClick={() => { void decide(false); }}>{snapshot.document.dirty ? "Discard & continue" : "Continue"}</button><button data-cancel className="quiet" disabled={session.busy} onClick={() => { void cancelGuard(); }}>Cancel</button></div></Dialog>}
    {choice && <Dialog title="New game" cancel={() => { if (!session.busy) setChoice(null); }} className="new-game-dialog"><p>Choose a game and your side. mShogi plays the other side.</p><div className="game-choices" role="radiogroup" aria-label="Game" onKeyDown={radioKeys}>{games.map((game) => <button role="radio" tabIndex={choice.game === game.key ? 0 : -1} data-game={game.key} aria-checked={choice.game === game.key} key={game.key} className={choice.game === game.key ? "chosen" : ""} disabled={session.busy} onClick={() => setChoice({ ...choice, game: game.key })}><span className={`game-mark ${game.key}`} aria-hidden="true">{game.key === "chess" ? "♜" : game.key === "shogi" ? "歩" : "帥"}</span><strong>{game.label}</strong><small>{game.description}</small></button>)}</div><h3>Your side</h3><div className="side-choices" role="radiogroup" aria-label="Your side" onKeyDown={radioKeys}>{(["first", "second", "random"] as const).map((side) => <button key={side} role="radio" tabIndex={choice.side === side ? 0 : -1} aria-checked={choice.side === side} data-side={side === "random" ? "random" : (side === "first" ? selectedGame.first : selectedGame.second).toLowerCase()} className={choice.side === side ? "chosen" : ""} disabled={session.busy} onClick={() => setChoice({ ...choice, side })}>{side === "first" ? selectedGame.first : side === "second" ? selectedGame.second : "Random"}</button>)}</div><p className="new-game-note">Untimed · your pieces start at the bottom of the board.</p>{!same(choice.source.open_token, snapshot.open_token) && <p role="alert">The current game changed. Cancel and review it before starting another game.</p>}{session.error && <p role="alert">The new game could not start. Your current game is retained.</p>}<div className="dialog-actions"><button className="quiet" data-cancel onClick={() => setChoice(null)} disabled={session.busy}>Cancel</button><button className="primary" data-start-game disabled={session.busy || !same(choice.source.open_token, snapshot.open_token)} onClick={() => { void start(); }}>Start game</button></div></Dialog>}
  </>;
}
