import { useLayoutEffect, useRef, type KeyboardEvent } from "react";
import type { Color, MatchConclusion, Snapshot } from "./native";
import { colorName, conclusionName } from "./presentation";

export function OutcomeCard({ conclusion }: { conclusion: MatchConclusion }) {
  let explanation =
    "Play has ended. The board and played move history are preserved.";
  if ("unresolved" in conclusion) {
    const { actor, trigger } = conclusion.unresolved;
    explanation = `${colorName(actor)} ${trigger === "resignation" ? "resigned" : "ran out of time"}. The M2 profile could not establish whether the opponent can mate. Play has stopped without an established win or draw, and no score is assigned.`;
  } else if ("win" in conclusion) {
    explanation = `Play ended by ${conclusion.win.trigger === "resignation" ? "resignation" : "time expiry"}. Native rules established the opponent's mating possibility; the result does not come from an evaluation score.`;
  } else if ("draw" in conclusion && conclusion.draw.trigger !== null) {
    explanation =
      "Play ended with a draw because native rules established that the opponent cannot mate.";
  }
  return (
    <section
      className={`card outcome-card ${"unresolved" in conclusion ? "unresolved" : ""}`}
      aria-label="Game outcome"
    >
      <p className="eyebrow">PLAY ENDED</p>
      <h2>{conclusionName(conclusion)}</h2>
      <p>{explanation}</p>
      <p className="caption">
        Start a new game to play again. This result remains separate from engine
        process status.
      </p>
    </section>
  );
}
export function RecoveryNotice({
  snapshot,
  error,
}: {
  snapshot: Snapshot;
  error: string;
}) {
  if (error)
    return (
      <section className="notice error" role="alert">
        <strong>Command or connection problem</strong>
        <p>
          The native session could not confirm this request. The board shows the
          latest received position.
        </p>
        <details>
          <summary>Request details</summary>
          <p>{error}</p>
        </details>
      </section>
    );
  if (snapshot.conclusion !== null) return null;
  const phase = snapshot.engine.status;
  if (phase !== "failed" && phase !== "cancelled" && phase !== "incomplete")
    return null;
  const heading =
    phase === "failed"
      ? "Engine failure · play paused"
      : phase === "incomplete"
        ? "Search incomplete · play paused"
        : "Search cancelled · play paused";
  const explanation =
    phase === "failed"
      ? "The engine could not complete this turn."
      : phase === "incomplete"
        ? "No complete search iteration finished."
        : "Engine work was cancelled; returned work cannot change the game.";
  return (
    <section
      className={`notice ${phase === "failed" ? "error" : ""}`}
      role={phase === "failed" ? "alert" : "status"}
    >
      <strong>{heading}</strong>
      <p>
        {explanation} The game is still ongoing. No engine move or chess result
        was added.
      </p>
      <p>
        Retry the engine turn to continue from this position, or start a new
        game.
      </p>
      {snapshot.engine.message && (
        <details>
          <summary>Native status details</summary>
          <p>{snapshot.engine.message}</p>
        </details>
      )}
    </section>
  );
}

interface ResignProps {
  human: Color;
  openedVersion: number;
  currentVersion: number;
  busy: boolean;
  connected: boolean;
  returnFocus: () => void;
  onCancel: () => void;
  onConfirm: (version: number) => void;
}
export function ResignConfirmation({
  human,
  openedVersion,
  currentVersion,
  busy,
  connected,
  returnFocus,
  onCancel,
  onConfirm,
}: ResignProps) {
  const dialog = useRef<HTMLDivElement>(null);
  const stale = openedVersion !== currentVersion;
  useLayoutEffect(() => {
    dialog.current?.querySelector<HTMLButtonElement>("button")?.focus();
    return returnFocus;
  }, []);
  function key(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") {
      event.preventDefault();
      if (!busy) onCancel();
    }
    if (event.key === "Tab") {
      const buttons = [
        ...(dialog.current?.querySelectorAll<HTMLButtonElement>(
          "button:not(:disabled)",
        ) ?? []),
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
    <div className="modal-backdrop">
      <div
        className="promotion-dialog resign-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="resign-title"
        aria-describedby="resign-description"
        ref={dialog}
        onKeyDown={key}
      >
        <p className="eyebrow">END YOUR GAME</p>
        <h2 id="resign-title">Resign as {colorName(human)}?</h2>
        <p id="resign-description">
          Resigning ends play and stops engine work. The native M2 rules
          determine the outcome. If mating possibility cannot be established,
          the result is unresolved and unscored; an engine evaluation does not
          decide it.
        </p>
        {stale && (
          <p className="resign-stale" role="alert">
            The game changed while this confirmation was open. Cancel and review
            the current position before resigning.
          </p>
        )}
        <div className="dialog-actions">
          <button className="secondary" disabled={busy} onClick={onCancel}>
            {stale ? "Return to game" : "Keep playing"}
          </button>
          <button
            className="danger"
            disabled={stale || busy || !connected}
            onClick={() => onConfirm(openedVersion)}
          >
            Confirm resignation
          </button>
        </div>
      </div>
    </div>
  );
}
