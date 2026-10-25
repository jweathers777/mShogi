import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { OutcomeCard, RecoveryNotice, ResignConfirmation } from "./Recovery";
import { CapacityNotice } from "./Capacity";
import { Board } from "./Board";
import { AnalysisPanel } from "./Analysis";
import { DocumentToolbar, UnsavedDialog, useDocumentGuard } from "./Document";
import { HistoryPanel, SavedObservations } from "./History";
import { readableAction, scoreDescription } from "./analysisPresentation";
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
      ? `Move returned: ${readableAction(last.action)}`
      : last.kind === "claim"
        ? `Claim returned: ${claimName(last.claim)}`
        : last.kind === "finished"
          ? "Engine reported a concluded position"
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
  const session = useNativeSession();
  const { snapshot, error, connected, command } = session;
  const busy =
    session.busy ||
    (snapshot !== null &&
      ["saving", "opening", "viewing"].includes(snapshot.io.phase));
  const guard = useDocumentGuard(snapshot, session.closeRequest, session);
  const [newHuman, setNewHuman] = useState<Color>("white");
  const [claimIndex, setClaimIndex] = useState(0);
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const resignButton = useRef<HTMLButtonElement>(null);
  const lastFocused = useRef<Element | null>(null);
  const suspendedFocus = useRef<Element | null>(null);
  const returnLiveFocus = useRef(false);
  const [resigning, setResigning] = useState<{
    version: number;
    human: Color;
  } | null>(null);
  useLayoutEffect(() => {
    const previous = lastFocused.current;
    if (snapshot === null || guard.pending !== null || resigning !== null ||
      previous === null) return;
    // WebKit also blurs a still-connected button when a command disables it.
    // Remember that transition, then recover only after it becomes enabled.
    if (previous.isConnected && previous.matches(":disabled")) {
      suspendedFocus.current = previous;
      return;
    }
    if (previous.isConnected && suspendedFocus.current !== previous) return;
    // Recover focus lost through removal or a recorded disabled transition.
    // Never pull focus from another control or a newly opened dialog.
    if (document.activeElement !== document.body &&
      document.activeElement !== document.documentElement) return;
    if (previous.isConnected) {
      if (previous instanceof HTMLElement || previous instanceof SVGElement) previous.focus();
      if (document.activeElement === previous) suspendedFocus.current = null;
      return;
    }
    const kind = previous.closest("[data-focus-after]")?.getAttribute("data-focus-after") ?? "board";
    const selectors: Record<string, string[]> = {
      analysis: ['[data-analysis-start="moves"]:not(:disabled)', '.inspector-switch button[aria-pressed="true"]:not(:disabled)'],
      engine: ['.engine-actions button:not(:disabled)', '.square[tabindex="0"]'],
      board: ['.square[tabindex="0"]'],
      outcome: ['[data-game-status]'],
    };
    for (const selector of [...(selectors[kind] ?? selectors.board!), '[data-game-status]']) {
      const target = document.querySelector<HTMLElement | SVGGElement>(selector);
      if (target !== null) {
        target.focus();
        if (document.activeElement === target) { lastFocused.current = target; break; }
      }
    }
  });
  useEffect(() => {
    if (guard.pending !== null) setResigning(null);
  }, [guard.pending]);
  useLayoutEffect(() => {
    if (
      returnLiveFocus.current &&
      snapshot !== null &&
      !snapshot.view.historical
    ) {
      returnLiveFocus.current = false;
      if (guard.pending === null)
        document.querySelector<SVGGElement>('.square[tabindex="0"]')?.focus();
    }
  }, [snapshot?.view.historical, snapshot?.view.selection_id, guard.pending]);
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
  const capacityPaused = snapshot.capacity.status === "paused";
  const canPlay =
    connected &&
    !busy &&
    humanTurn &&
    resigning === null &&
    !snapshot.play_paused &&
    !capacityPaused &&
    !snapshot.view.historical &&
    guard.pending === null;
  const paused = ["cancelled", "failed", "incomplete"].includes(
    snapshot.engine.status,
  );
  const claim = snapshot.claims[claimIndex] ?? snapshot.claims[0];
  const orientation = snapshot.document.settings.orientation;
  const historical = snapshot.view.historical;
  return (
    <main className="application" onFocusCapture={(event) => {
      if (event.target instanceof Element) {
        lastFocused.current = event.target;
        suspendedFocus.current = null;
      }
    }}>
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
        <section
          className="board-panel"
          aria-label={historical ? "Historical position" : "Live game"}
        >
          <div className="board-heading">
            <div>
              <p className="eyebrow">
                {historical ? "HISTORICAL · READ ONLY" : "LIVE GAME"}
              </p>
              <h2 role="status" aria-live="polite" tabIndex={-1} data-game-status>
                {historical ? snapshot.view.label : gameStatus(snapshot)}
              </h2>
              <p className="player-line">
                You play {colorName(snapshot.human)} <span>against</span> mShogi
              </p>
            </div>
            <button
              className="quiet flip-button"
              disabled={!connected || busy || guard.pending !== null}
              onClick={() => {
                void command({
                  kind: "preferences",
                  orientation: orientation === "white" ? "black" : "white",
                  analysis_defaults:
                    snapshot.document.settings.analysis_defaults,
                });
              }}
              aria-label={`Flip board; currently ${orientation} at bottom`}
            >
              ↕ <span>Flip board</span>
            </button>
          </div>
          <Board
            key={snapshot.view.selection_id}
            pieces={snapshot.view.pieces}
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
              {historical
                ? "Read-only history · return to live to play"
                : snapshot.conclusion !== null
                  ? "Game concluded"
                  : snapshot.play_paused
                    ? "Play paused · resume explicitly to continue"
                    : humanTurn
                      ? "Your turn"
                      : snapshot.engine.status === "thinking"
                        ? "mShogi is searching within fixed work limits"
                        : "Engine turn paused"}
            </span>
            <span>
              {historical ? "Selected position" : "Live position"} ·{" "}
              {snapshot.view.ply} plies
            </span>
          </div>
        </section>
        <aside className="sidebar" aria-label="Game controls and inspection">
          <DocumentToolbar
            snapshot={snapshot}
            busy={busy || guard.pending !== null}
            connected={connected}
            actions={session}
            onOpen={guard.requestOpen}
          />
          <CapacityNotice capacity={snapshot.capacity} />
          {historical && (
            <section className="notice historical-notice">
              <strong>Viewing history · read only</strong>
              <p>
                {snapshot.view.label} · {colorName(snapshot.view.turn)} to move.
                The live game remains after {snapshot.history.length} plies.
                Analysis uses this selected native history prefix.
              </p>
              {snapshot.view.conclusion !== null && (
                <OutcomeCard conclusion={snapshot.view.conclusion} />
              )}
              <button
                className="secondary"
                disabled={busy || !connected || guard.pending !== null}
                onClick={async () => {
                  returnLiveFocus.current = true;
                  const result = await command({
                    kind: "return_live",
                    version: snapshot.version,
                  });
                  if (result === null) returnLiveFocus.current = false;
                }}
              >
                Return to live game
              </button>
            </section>
          )}
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
                disabled={!connected || busy || guard.pending !== null}
                onClick={() => {
                  setClaimIndex(0);
                  guard.requestNewGame(newHuman);
                }}
              >
                New game
              </button>
            </div>
            <p className="caption">
              Save the native game document to preserve context, events and
              observations.
            </p>
            {snapshot.conclusion === null && !historical && (
              <button
                className="quiet resign-button"
                ref={resignButton}
                data-focus-after="outcome"
                disabled={!connected || busy || guard.pending !== null || capacityPaused}
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
          {snapshot.conclusion !== null && !historical && (
            <OutcomeCard conclusion={snapshot.conclusion} />
          )}
          <RecoveryNotice snapshot={snapshot} error={error.startsWith("Application capacity:") ? "" : error} />
          {snapshot.play_paused && !capacityPaused && (
            <section className="notice play-paused" role="status">
              <strong>Play paused</strong>
              <p>
                {snapshot.conclusion === null
                  ? "Board moves and draw claims are paused. Returning to live and resuming retires analysis and continues play."
                  : "The game remains concluded. Finishing analysis does not reopen it."}
              </p>
              {snapshot.analysis.status === "thinking" && (
                <button
                  className="quiet"
                  data-focus-after="analysis"
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
                data-focus-after={snapshot.conclusion === null ? "board" : "outcome"}
                disabled={
                  busy || !connected || historical || guard.pending !== null
                }
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
              key={session.documentEpoch}
              snapshot={snapshot}
              busy={busy || guard.pending !== null || capacityPaused}
              connected={connected}
              command={command}
            />
          ) : (
            <section className="card engine-card">
              <div className="card-title">
                <h2>Native engine</h2>
                <span className={`engine-phase ${snapshot.engine.status}`}
                  role="status" aria-label="Engine turn status">
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
                    data-focus-after="engine"
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
                      data-focus-after="engine"
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
          {snapshot.claims.length > 0 &&
            snapshot.conclusion === null &&
            !historical && (
              <section className="card claims" data-focus-after="outcome">
                <h2 role="status">Draw claim available</h2>
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
                      data-focus-after="outcome"
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
          <HistoryPanel
            snapshot={snapshot}
            busy={busy || guard.pending !== null}
            connected={connected}
            command={command}
          />
          <SavedObservations observations={snapshot.observations} />
        </aside>
      </div>
      {guard.pending !== null && (
        <UnsavedDialog
          guard={guard}
          dirty={snapshot.document.dirty}
          busy={busy}
        />
      )}
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
