import { games, tenjikuProfiles, type TenjikuProfile, type GameKind } from "../games";
import type { GameSnapshot, TaggedSide, OpenToken, PlayMode } from "../sessionTypes";
import { useEffect, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import { ApplicationView } from "./ApplicationView";
import { Dialog } from "./Dialog";
import { useGameSession } from "./useGameSession";

import "./shogi.css";
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
export interface VerificationControlsProps { snapshot: GameSnapshot; session: ReturnType<typeof useGameSession>; blocked: boolean }
export function ProductApp({ verificationControls }: { verificationControls?: ((props: VerificationControlsProps) => ReactNode) | undefined } = {}) {
  const session = useGameSession(), { snapshot } = session;
  const [pending, setPending] = useState<{ source: GameSnapshot } | null>(null);
  const [choice, setChoice] = useState<{ source: GameSnapshot; game: GameKind; playMode: PlayMode; side: "first" | "second" | "random"; tenjikuProfile: TenjikuProfile } | null>(null);
  const [message, setMessage] = useState("");
  const deciding = useRef(false), handledClose = useRef(false);
  function begin(intent: "new" | "open" | "quit", source = snapshot) {
    if (!source) return; setMessage("");
    // New/Load are deliberate replacement requests. Merely opening either
    // chooser leaves the current document installed until admission.
    if (intent === "new") { setChoice({ source, game: source.game, playMode: source.play_mode, side: source.human.first_mover ? "first" : "second", tenjikuProfile: source.game === "tenjiku" ? source.profile : tenjikuProfiles[0].id }); return; }
    if (intent === "open") { void session.file(source, "open"); return; }
    if (source.close_requires_save) setPending({ source });
    else void session.finishClose(source).then(closed => { if (!closed) void session.cancelClose(); });
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
    if (pending && !(await session.cancelClose())) return;
    setPending(null); setMessage("");
  }
  async function decide(save: boolean) {
    if (!pending || !snapshot || stale || deciding.current) return;
    deciding.current = true; setMessage("");
    try {
      let source = snapshot;
      if (save) {
        const saved = await session.file(source, "save");
        if (!saved || saved.document.dirty || saved.token.document !== pending.source.token.document) { setMessage("The game was not saved. Your game is still open."); return; }
        source = saved; setPending({ source });
      }
      if (await session.finishClose(source)) setPending(null);
    } finally { deciding.current = false; }
  }
  async function start() {
    if (!choice || !snapshot || session.busy || !same(choice.source.open_token, snapshot.open_token)) return;
    const first = choice.side === "random" ? (crypto.getRandomValues(new Uint8Array(1))[0]! & 1) === 0 : choice.side === "first";
    const { side } = games[choice.game].sides[first ? 0 : 1];
    const human = { game_kind: choice.game, side } as TaggedSide;
    const next = await session.newGame(choice.source.open_token, choice.game, human, choice.game === "tenjiku" ? choice.tenjikuProfile : games[choice.game].profile, choice.playMode);
    if (next) setChoice(null);
  }
  if (!snapshot) return <main className="loading"><h1>mShogi</h1><p role="status">Opening your game…</p>{session.error && <p role="alert">The snapshot application could not connect.</p>}</main>;
  const blocked = pending !== null || choice !== null;
  const selectedGame = games[choice?.game ?? snapshot.game];
  return <>{verificationControls?.({ snapshot, session, blocked })}<div inert={blocked}><ApplicationView key={snapshot.token.document} snapshot={snapshot} session={session} onNewGame={() => begin("new")} onOpen={() => begin("open")} blocked={blocked} /></div>
    {pending && <Dialog title="Save Game" className="document-guard" cancel={() => { if (!session.busy) void cancelGuard(); }}>{stale && <p role="alert">The game changed. Cancel and review the current position.</p>}{message && <p role="alert">{message}</p>}<div className="dialog-actions"><button className="secondary" disabled={session.busy || stale || !snapshot.close_requires_save} onClick={() => { void decide(true); }}>Save</button><button className="danger" disabled={session.busy || stale} onClick={() => { void decide(false); }}>Discard</button><button data-cancel className="quiet" disabled={session.busy} onClick={() => { void cancelGuard(); }}>Cancel</button></div></Dialog>}
    {choice && <Dialog title="New game" cancel={() => { if (!session.busy) setChoice(null); }} className="new-game-dialog"><label className="game-select-label">Game<select data-game-select value={choice.game} disabled={session.busy} onChange={event => setChoice({ ...choice, game: event.target.value as GameKind })}>{Object.entries(games).map(([kind, game]) => <option key={kind} value={kind}>{game.name}</option>)}</select></label>{choice.game === "tenjiku" && <label className="game-select-label">Rules profile<select data-profile-select value={choice.tenjikuProfile} disabled={session.busy} onChange={event => setChoice({ ...choice, tenjikuProfile: event.target.value as TenjikuProfile })}>{tenjikuProfiles.map(profile => <option key={profile.id} value={profile.id}>{profile.label}</option>)}</select></label>}<label className="game-select-label">Players<select aria-label="Players" value={choice.playMode} disabled={session.busy} onChange={event => setChoice({ ...choice, playMode: event.target.value as PlayMode, side: choice.side === "random" ? "first" : choice.side })}><option value="human_engine">Human against mShogi</option><option value="two_humans">Two humans</option></select></label><h3>{choice.playMode === "two_humans" ? "Bottom side" : "Your side"}</h3><div className="side-choices" role="radiogroup" aria-label={choice.playMode === "two_humans" ? "Bottom side" : "Your side"} onKeyDown={radioKeys}>{(choice.playMode === "two_humans" ? ["first", "second"] as const : ["first", "second", "random"] as const).map((side) => <button key={side} role="radio" tabIndex={choice.side === side ? 0 : -1} aria-checked={choice.side === side} data-side={side === "random" ? "random" : selectedGame.sides[side === "first" ? 0 : 1].side} className={choice.side === side ? "chosen" : ""} disabled={session.busy} onClick={() => setChoice({ ...choice, side })}>{side === "random" ? "Random" : selectedGame.sides[side === "first" ? 0 : 1].label}</button>)}</div>{!same(choice.source.open_token, snapshot.open_token) && <p role="alert">The current game changed. Cancel and review it before starting another game.</p>}{session.error && <p role="alert">The new game could not start. Your current game is retained.</p>}<div className="dialog-actions"><button className="quiet" data-cancel onClick={() => setChoice(null)} disabled={session.busy}>Cancel</button><button className="primary" data-start-game disabled={session.busy || !same(choice.source.open_token, snapshot.open_token)} onClick={() => { void start(); }}>Start game</button></div></Dialog>}
  </>;
}
