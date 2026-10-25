import { useRef, useState } from "react";
import { OutcomeCard, RecoveryNotice, ResignConfirmation } from "./Recovery";
import { Board } from "./Board";
import { AnalysisPanel } from "./Analysis";
import { scoreDescription } from "./analysisPresentation";
import type { Color, Snapshot } from "./native";
import { claimName, colorName, gameStatus } from "./presentation";
import { useNativeSession } from "./useNativeSession";

function EngineObservations({ snapshot }: { snapshot: Snapshot }) {
  const { engine } = snapshot;
  const last = engine.last;
  if (last === null)
    return (
      <p className="muted">No engine search has completed in this game.</p>
    );
  if (last.kind === "failure")
    return (
      <details className="failure-detail">
        <summary>Engine diagnostic details</summary>
        <p>
          {last.failure.kind.replaceAll("_", " ")}: {last.failure.message}
        </p>
      </details>
    );
  if (last.kind === "reachability")
    return (
      <p className="failure-detail">
        An advanced analysis result cannot be used as an engine-turn result. No
        move is implied.
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
      <p className="analysis-score">
        {scoreDescription(telemetry.score, engine.last_turn)}
      </p>
      <p className="caption">
        Last engine-turn position ·{" "}
        {engine.last_turn === null
          ? "unknown side"
          : colorName(engine.last_turn)}{" "}
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
      </dl>
      <details className="technical-analysis">
        <summary>Search technical details</summary>
        <p>
          Last searched position · version {engine.searched_version} ·{" "}
          {engine.last_turn === null ? "unknown" : colorName(engine.last_turn)}{" "}
          perspective
        </p>
        <pre>{JSON.stringify(telemetry, null, 2)}</pre>
      </details>
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
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const resignButton = useRef<HTMLButtonElement>(null);
  const [resigning, setResigning] = useState<{
    version: number;
    human: Color;
  } | null>(null);
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
  const canPlay =
    connected &&
    !busy &&
    humanTurn &&
    resigning === null &&
    !snapshot.play_paused;
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
                : snapshot.play_paused
                  ? "Play paused · analysis cannot change the game"
                  : humanTurn
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
            {snapshot.conclusion === null && (
              <button
                className="quiet resign-button"
                ref={resignButton}
                disabled={!connected || busy}
                onClick={() =>
                  setResigning({
                    version: snapshot.version,
                    human: snapshot.human,
                  })
                }
              >
                Resign game…
              </button>
            )}
          </section>
          {snapshot.conclusion !== null && (
            <OutcomeCard conclusion={snapshot.conclusion} />
          )}
          <RecoveryNotice snapshot={snapshot} error={error} />
          {snapshot.play_paused && (
            <section className="notice play-paused" role="status">
              <strong>Play paused for analysis</strong>
              <p>
                {snapshot.conclusion === null
                  ? "Board moves and draw claims are paused. Resuming retires analysis and continues play."
                  : "The game remains concluded. Finishing analysis does not reopen it."}
              </p>
              {snapshot.analysis.status === "thinking" && (
                <button
                  className="quiet"
                  disabled={busy || !connected}
                  onClick={() => {
                    void command({
                      kind: "cancel_analysis",
                      version: snapshot.version,
                    });
                  }}
                >
                  Cancel analysis
                </button>
              )}
              <button
                className="secondary"
                disabled={busy || !connected}
                onClick={() => {
                  void command({ kind: "resume", version: snapshot.version });
                }}
              >
                {snapshot.conclusion === null
                  ? "Resume play"
                  : "Finish analysis"}
              </button>
            </section>
          )}
          <div
            className="inspector-switch"
            role="group"
            aria-label="Inspector view"
          >
            <button
              aria-pressed={inspector === "play"}
              onClick={() => setInspector("play")}
            >
              Play details
            </button>
            <button
              aria-pressed={inspector === "analysis"}
              onClick={() => setInspector("analysis")}
            >
              Analyze position
            </button>
          </div>
          {inspector === "analysis" ? (
            <AnalysisPanel
              snapshot={snapshot}
              busy={busy}
              connected={connected}
              command={command}
            />
          ) : (
            <section className="card engine-card">
              <div className="card-title">
                <h2>Native engine</h2>
                <span className={`engine-phase ${snapshot.engine.status}`}>
                  {snapshot.engine.status === "thinking"
                    ? "Searching"
                    : snapshot.engine.status === "paused"
                      ? "Play paused"
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
                      void command({
                        kind: "cancel",
                        version: snapshot.version,
                      });
                    }}
                  >
                    Cancel search
                  </button>
                )}
                {paused &&
                  !snapshot.play_paused &&
                  !humanTurn &&
                  snapshot.conclusion === null && (
                    <button
                      className="secondary"
                      disabled={busy || !connected}
                      onClick={() => {
                        void command({
                          kind: "retry",
                          version: snapshot.version,
                        });
                      }}
                    >
                      Retry engine turn
                    </button>
                  )}
              </div>
            </section>
          )}
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
                  <p className="claim-scope">
                    {claim.intended === null
                      ? "Current-position claim"
                      : `Intended-move claim · ${claim.intended}`}
                  </p>
                  <p className="caption">
                    {claim.intended === null
                      ? "The required threshold is already established in the current native history."
                      : "This action is an unplayed intention. Claiming does not add this move to the board or move history."}
                  </p>
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
                  {snapshot.conclusion === null
                    ? "Moves appear here as the game is played."
                    : "No board moves were played."}
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
      {resigning !== null && (
        <ResignConfirmation
          human={resigning.human}
          openedVersion={resigning.version}
          currentVersion={snapshot.version}
          returnFocus={() => {
            if (resignButton.current && !resignButton.current.disabled)
              resignButton.current.focus();
            else
              document
                .querySelector<SVGGElement>('.square[tabindex="0"]')
                ?.focus();
          }}
          busy={busy}
          connected={connected}
          onCancel={() => setResigning(null)}
          onConfirm={(version) => {
            setResigning(null);
            void command({ kind: "resign", version });
          }}
        />
      )}
      <footer className="app-footer">
        <span>One native rules engine. Complete actions. Optional claims.</span>
        <details className="footer-details">
          <summary>Session details</summary>
          <span>
            Session version {snapshot.version} · snapshot revision{" "}
            {snapshot.revision}
          </span>
        </details>
      </footer>
    </main>
  );
}
