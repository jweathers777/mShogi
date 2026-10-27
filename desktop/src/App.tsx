import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { OutcomeCard, RecoveryNotice, ResignConfirmation } from "./Recovery";
import { CapacityNotice } from "./Capacity";
import { Board } from "./Board";
import { AnalysisPanel } from "./Analysis";
import { UnsavedDialog, useDocumentGuard } from "./Document";
import { HistoryPanel, SavedObservations } from "./History";
import { readableAction, scoreDescription } from "./analysisPresentation";
import type { Color, Snapshot } from "./native";
import { claimName, colorName, gameStatus } from "./presentation";
import { useNativeSession } from "./useNativeSession";
import { ApplicationShell, DocumentState, GameArea, GamePositionStatus, HistoricalNotice, InspectorSwitch, LiveGameHeading, NativeEnginePanel, PlayerArea } from "./product/ApplicationShell";
import type { GameSnapshot } from "./product/native";
import { useMovementInspection } from "./product/useMovementInspection";

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
export function App({ onChooseGame, externalCloseGuard = false, sharedSnapshot }: { onChooseGame?: (human: Color) => void; externalCloseGuard?: boolean; sharedSnapshot?: GameSnapshot } = {}) {
  const session = useNativeSession();
  const { snapshot: legacySnapshot, error, connected, command } = session;
  const snapshot = sharedSnapshot?.chess ?? legacySnapshot;
  const onInspect = useMovementInspection(sharedSnapshot ? { token: sharedSnapshot.token, game: sharedSnapshot.game } : null);
  const busy =
    session.busy ||
    (snapshot !== null &&
      ["saving", "opening", "viewing"].includes(snapshot.io.phase));
  const guard = useDocumentGuard(snapshot, externalCloseGuard ? null : session.closeRequest, session);
  const [newHuman, setNewHuman] = useState<Color>("white");
  const [claimIndex, setClaimIndex] = useState(0);
  const [inspector, setInspector] = useState<"play" | "analysis">("play");
  const [boardHelpTarget, setBoardHelpTarget] = useState<HTMLDivElement | null>(null);
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
  return <ApplicationShell game="chess" runtime={!connected ? "Connecting…" : snapshot.engine.status === "thinking" ? "mShogi is thinking" : snapshot.analysis.status === "thinking" ? "Analyzing selected position" : "Local engine"} historical={historical}
    onFocusCapture={(event) => { if (event.target instanceof Element) { lastFocused.current = event.target; suspendedFocus.current = null; } }}
    document={<><DocumentState document={snapshot.document} /><h2 title={snapshot.document.path ?? undefined}>{snapshot.document.path?.split(/[\\/]/).at(-1) ?? "Untitled game"}</h2>{snapshot.io.phase !== "idle" && <p role={snapshot.io.phase === "failed" ? "alert" : "status"}>{snapshot.io.message ?? "File operation in progress…"}</p>}</>}
    actions={<><button className="primary" disabled={!connected || busy || guard.pending !== null || resigning !== null} onClick={() => { setClaimIndex(0); if (onChooseGame) onChooseGame(snapshot.human); else guard.requestNewGame(newHuman); }}>New game</button><button className="quiet" disabled={!connected || busy || guard.pending !== null || resigning !== null} onClick={() => { void session.save(); }}>Save</button><button className="quiet" disabled={!connected || busy || guard.pending !== null || resigning !== null} onClick={() => { void session.saveAs(); }}>Save as…</button><button className="quiet" disabled={!connected || busy || guard.pending !== null || resigning !== null} onClick={guard.requestOpen}>Open…</button></>}
    heading={<LiveGameHeading eyebrow={historical ? "HISTORY · READ ONLY" : snapshot.conclusion ? "GAME CONCLUDED" : "YOUR GAME"} status={historical ? snapshot.view.label : gameStatus(snapshot)} player={<>You play {colorName(snapshot.human)} against mShogi</>} flip={<>            <button
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
</>} />}
    board={<GameArea
      upperPlayer={<PlayerArea side={colorName(orientation === "white" ? "black" : "white")} playerKey={orientation === "white" ? "black" : "white"} human={snapshot.human !== orientation} toMove={snapshot.view.turn !== orientation && snapshot.view.conclusion === null} />}
      lowerPlayer={<PlayerArea side={colorName(orientation)} playerKey={orientation} human={snapshot.human === orientation} toMove={snapshot.view.turn === orientation && snapshot.view.conclusion === null} />}
      board={          <Board
            helpTarget={boardHelpTarget}
            key={snapshot.view.selection_id}
            pieces={snapshot.view.pieces}
            choices={snapshot.legal_choices}
            orientation={orientation}
            version={snapshot.version}
            disabled={!canPlay}
            onInspect={sharedSnapshot ? onInspect : undefined}
            humanOwner={snapshot.human}
            inspectionDisabled={!connected || busy || guard.pending !== null || resigning !== null}
            inspectionContextKey={JSON.stringify(sharedSnapshot?.token ?? null)}
            onPlay={(action) => {
              void command({ kind: "play", version: snapshot.version, action });
            }}
          />
} footer={<><div ref={setBoardHelpTarget} className="shell-board-help" /><GamePositionStatus historical={historical} concluded={snapshot.conclusion !== null}
            paused={snapshot.play_paused} humanTurn={humanTurn} thinking={snapshot.engine.status === "thinking"} ply={snapshot.view.ply} />
          </>
} />}
    positionNotice={<>          {historical && (
            <HistoricalNotice action={<button
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
                Return to live
              </button>}>
              <p>
                {snapshot.view.label} · {colorName(snapshot.view.turn)} to move.
                The live game remains after {snapshot.history.length} plies.
                Analysis uses this selected native history prefix.
              </p>
              {snapshot.view.conclusion !== null && (
                <OutcomeCard conclusion={snapshot.view.conclusion} />
              )}
            </HistoricalNotice>
          )}
</>}
    notices={<>          <CapacityNotice capacity={snapshot.capacity} />
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
</>}
    analysis={<><InspectorSwitch value={inspector} onChange={setInspector} />
          {inspector === "analysis" ? (
            <AnalysisPanel
              key={session.documentEpoch}
              snapshot={snapshot}
              busy={busy || guard.pending !== null || capacityPaused}
              connected={connected}
              command={command}
            />
          ) : (
            <NativeEnginePanel limits={snapshot.limits} status={snapshot.engine.status === "thinking" ? "Searching" : snapshot.engine.status === "paused" ? "Play paused" : snapshot.engine.status}>
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
            </NativeEnginePanel>
          )}
          <SavedObservations observations={snapshot.observations} />
</>}
    history={<>          <HistoryPanel
            snapshot={snapshot}
            busy={busy || guard.pending !== null}
            connected={connected}
            command={command}
          />
</>}
    gameActions={<>          <section className="card new-game">
            {!onChooseGame && <><label htmlFor="human-color">Your color for a new game</label><select id="human-color" value={newHuman} onChange={(event) => setNewHuman(event.target.value as Color)}><option value="white">White</option><option value="black">Black</option></select></>}
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
</>}
    technical={<><p>mShogi · untimed chess · native rules and complete game records.</p><details><summary>Session details</summary><p>Session version {snapshot.version} · snapshot revision {snapshot.revision}</p></details><details className="document-details"><summary>File details</summary><p>{snapshot.document.path ?? "No file selected"}</p><p>Document revision {snapshot.document.revision}. Modes, limits, match events and observations are saved by the native host.</p></details><button className="quiet" disabled={!connected || busy} onClick={() => { void session.requestClose(); }}>Quit application</button></>}
    overlays={<>      {guard.pending !== null && (
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
</>}
  />;
}
