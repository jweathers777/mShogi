import { useState } from "react";
import { Board } from "./Board";
import type { Color, Snapshot } from "./native";
import { claimName, colorName, gameStatus } from "./presentation";
import { useNativeSession } from "./useNativeSession";

function EngineObservations({ snapshot }: { snapshot: Snapshot }) {
  const { engine } = snapshot;
  const last = engine.last;
  if (last === null)
    return (
      <p className="muted">
        Final search observations appear after mShogi replies.
      </p>
    );
  if (last.kind === "failure")
    return (
      <p className="failure-detail">
        {last.failure.kind.replaceAll("_", " ")}: {last.failure.message}
      </p>
    );
  const telemetry = last.telemetry;
  const choice =
    last.kind === "move"
      ? `Played ${last.action}`
      : last.kind === "claim"
        ? claimName(last.claim)
        : last.kind === "finished"
          ? "Position already concluded"
          : "No completed search iteration";
  return (
    <>
      <p className="engine-choice">{choice}</p>
      <p className="caption">
        Last searched position · version {engine.searched_version} ·{" "}
        {engine.last_turn === null ? "unknown" : colorName(engine.last_turn)}{" "}
        perspective
      </p>
      <dl className="telemetry">
        <div>
          <dt>Depth</dt>
          <dd>{telemetry.completed_depth}</dd>
        </div>
        <div>
          <dt>Nodes</dt>
          <dd>{telemetry.nodes.toLocaleString()}</dd>
        </div>
        <div>
          <dt>Native score</dt>
          <dd>{telemetry.score ?? "—"}</dd>
        </div>
      </dl>
      <p className="caption">
        Score units: pawn = 100; mate uses native distance encoding.{" "}
        {telemetry.status.replaceAll("_", " ")}.
      </p>
      {telemetry.principal_variation.length > 0 && (
        <p className="variation">
          <span>Continuation</span>{" "}
          {telemetry.principal_variation
            .map((choice) =>
              choice.kind === "move"
                ? choice.action
                : `claim (${choice.claim.reason.replaceAll("_", " ")})`,
            )
            .join(" · ")}
        </p>
      )}
    </>
  );
}
export function App() {
  const { snapshot, error, connected, busy, command } = useNativeSession();
  const [orientation, setOrientation] = useState<Color>("white");
  const [newHuman, setNewHuman] = useState<Color>("white");
  const [claimIndex, setClaimIndex] = useState(0);
  if (snapshot === null)
    return (
      <main className="loading">
        <p className="eyebrow">MSHOGI · NATIVE CHESS</p>
        <h1>Opening the board</h1>
        <p role={error ? "alert" : "status"}>
          {error || "Connecting to the native game session…"}
        </p>
      </main>
    );
  const humanTurn =
    snapshot.human === snapshot.turn && snapshot.conclusion === null;
  const canPlay = connected && !busy && humanTurn;
  const paused = ["cancelled", "failed", "incomplete"].includes(
    snapshot.engine.status,
  );
  const claim = snapshot.claims[claimIndex] ?? snapshot.claims[0];
  const moves = Array.from(
    { length: Math.ceil(snapshot.history.length / 2) },
    (_, index) => ({
      white: snapshot.history[index * 2],
      black: snapshot.history[index * 2 + 1],
    }),
  );
  return (
    <main className="application">
      <header className="app-header">
        <div className="brand">
          <span className="brand-mark" aria-hidden="true">
            ♜
          </span>
          <div>
            <h1>mShogi</h1>
            <p>CHESS · DESKTOP PREVIEW</p>
          </div>
        </div>
        <div className="session-badges">
          <span>
            <i /> Untimed
          </span>
          <span>M2 rules profile</span>
        </div>
      </header>
      <div className="workspace">
        <section className="board-panel" aria-label="Live game">
          <div className="board-heading">
            <div>
              <p className="eyebrow">LIVE GAME</p>
              <h2 role="status" aria-live="polite">
                {gameStatus(snapshot)}
              </h2>
              <p className="player-line">
                You play {colorName(snapshot.human)} <span>against</span> mShogi
              </p>
            </div>
            <button
              className="quiet flip-button"
              onClick={() =>
                setOrientation(orientation === "white" ? "black" : "white")
              }
              aria-label={`Flip board; currently ${orientation} at bottom`}
            >
              ↕ <span>Flip board</span>
            </button>
          </div>
          <Board
            pieces={snapshot.pieces}
            choices={snapshot.legal_choices}
            orientation={orientation}
            version={snapshot.version}
            disabled={!canPlay}
            onPlay={(action) => {
              void command({ kind: "play", version: snapshot.version, action });
            }}
          />
          <div className="game-footnote">
            <span>
              {snapshot.conclusion !== null
                ? "Game concluded"
                : canPlay
                  ? "Your turn"
                  : snapshot.engine.status === "thinking"
                    ? "mShogi is searching within fixed work limits"
                    : "Engine turn paused"}
            </span>
            <span>Live position · {snapshot.history.length} plies</span>
          </div>
        </section>
        <aside className="sidebar" aria-label="Game controls and inspection">
          <section className="card new-game">
            <p className="eyebrow">PLAY</p>
            <label htmlFor="human-color">Your color for a new game</label>
            <div className="new-game-controls">
              <select
                id="human-color"
                value={newHuman}
                onChange={(event) => setNewHuman(event.target.value as Color)}
              >
                <option value="white">White</option>
                <option value="black">Black</option>
              </select>
              <button
                className="primary"
                disabled={!connected || busy}
                onClick={() => {
                  setClaimIndex(0);
                  void command({
                    kind: "new_game",
                    version: snapshot.version,
                    human: newHuman,
                  });
                }}
              >
                New game
              </button>
            </div>
            <p className="caption">
              Games are unsaved. New game or closing the app discards this game.
            </p>
          </section>
          {(error || snapshot.engine.message) && (
            <section
              className={`notice ${snapshot.engine.status === "failed" || error ? "error" : ""}`}
              role={
                snapshot.engine.status === "failed" || error
                  ? "alert"
                  : "status"
              }
            >
              <strong>
                {error
                  ? "Command or connection problem"
                  : snapshot.engine.status === "failed"
                    ? "Engine failure · game unscored"
                    : snapshot.engine.status === "incomplete"
                      ? "Search incomplete"
                      : "Engine work cancelled"}
              </strong>
              <p>{error || snapshot.engine.message}</p>
            </section>
          )}
          <section className="card engine-card">
            <div className="card-title">
              <h2>Native engine</h2>
              <span className={`engine-phase ${snapshot.engine.status}`}>
                {snapshot.engine.status === "thinking"
                  ? "Searching"
                  : snapshot.engine.status}
              </span>
            </div>
            <p className="caption">
              Play limit: depth {snapshot.limits.depth} ·{" "}
              {snapshot.limits.nodes.toLocaleString()} nodes
            </p>
            <EngineObservations snapshot={snapshot} />
            <div className="engine-actions">
              {snapshot.engine.status === "thinking" && (
                <button
                  className="secondary"
                  disabled={busy || !connected}
                  onClick={() => {
                    void command({ kind: "cancel", version: snapshot.version });
                  }}
                >
                  Cancel search
                </button>
              )}
              {paused && !humanTurn && snapshot.conclusion === null && (
                <button
                  className="secondary"
                  disabled={busy || !connected}
                  onClick={() => {
                    void command({ kind: "retry", version: snapshot.version });
                  }}
                >
                  Retry engine turn
                </button>
              )}
            </div>
          </section>
          {snapshot.claims.length > 0 && snapshot.conclusion === null && (
            <section className="card claims">
              <h2>Draw claim available</h2>
              <p className="caption">
                An opportunity to claim, while the game remains ongoing. An
                intended move is not played by claiming.
              </p>
              {humanTurn && claim !== undefined ? (
                <>
                  <label className="sr-only" htmlFor="draw-claim">
                    Native draw claim
                  </label>
                  <select
                    id="draw-claim"
                    value={snapshot.claims[claimIndex] ? claimIndex : 0}
                    onChange={(event) =>
                      setClaimIndex(Number(event.target.value))
                    }
                  >
                    {snapshot.claims.map((claim, index) => (
                      <option
                        key={`${claim.reason}-${claim.intended}`}
                        value={index}
                      >
                        {claimName(claim)}
                      </option>
                    ))}
                  </select>
                  <button
                    className="secondary"
                    disabled={!canPlay}
                    onClick={() => {
                      void command({
                        kind: "claim",
                        version: snapshot.version,
                        claim,
                      });
                    }}
                  >
                    Claim selected draw
                  </button>
                </>
              ) : (
                <p>mShogi may choose a valid native claim.</p>
              )}
            </section>
          )}
          <section className="card history-card">
            <div className="card-title">
              <h2>Move history</h2>
              <span className="caption">Native actions</span>
            </div>
            <div
              className="history-scroll"
              tabIndex={0}
              aria-label="Played move history"
            >
              {moves.length === 0 ? (
                <p className="empty-history">
                  Moves appear here as the game is played.
                </p>
              ) : (
                <table>
                  <thead>
                    <tr>
                      <th scope="col">Move</th>
                      <th scope="col">White</th>
                      <th scope="col">Black</th>
                    </tr>
                  </thead>
                  <tbody>
                    {moves.map((move, index) => (
                      <tr key={index}>
                        <th scope="row">{index + 1}.</th>
                        <td>{move.white}</td>
                        <td>{move.black ?? "…"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </div>
            <p className="caption">
              Viewing earlier positions and saving arrive in a later desktop
              batch.
            </p>
          </section>
        </aside>
      </div>
      <footer className="app-footer">
        <span>One native rules engine. Complete actions. Optional claims.</span>
        <span>Session version {snapshot.version}</span>
      </footer>
    </main>
  );
}
