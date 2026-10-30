import { useEffect, useRef, useState, type FormEvent } from "react";
import { AnalysisActions } from "./product/ApplicationShell";
import type { DesktopCommand, ExecutionTarget, ReachabilityTarget, Snapshot } from "./native";
import {
  choiceDescription,
  readableAction,
  reachabilityDescription,
  resultTelemetry,
  scoreDescription,
} from "./analysisPresentation";
import { colorName, conclusionName } from "./presentation";

function targetColor(target: ExecutionTarget) {
  return target.turn.game_kind === "chess" ? target.turn.side : null;
}
function targetTurnLabel(target: ExecutionTarget): string {
  const color = targetColor(target);
  return color === null ? "Unknown Chess side" : colorName(color);
}
function budget(value: string, max: number, allowZero: boolean): number | null {
  if (!/^\d+$/.test(value)) return null;
  const parsed = Number(value);
  return Number.isSafeInteger(parsed) &&
    parsed >= (allowZero ? 0 : 1) &&
    parsed <= max
    ? parsed
    : null;
}
function analysisSummary(snapshot: Snapshot, current: boolean, accepted: boolean): string {
  const { analysis } = snapshot;
  if (analysis.status === "idle" || analysis.status === "failed") return "";
  if (analysis.status === "cancelled")
    return "Analysis cancelled. No conclusion was established; play remains paused until you explicitly resume.";
  if (analysis.target === null) return "Waiting for an identified native analysis target.";
  const position = `${analysis.target.label}, ${targetTurnLabel(analysis.target)} to move`;
  if (!current)
    return `Analysis for recorded target ${position} belongs to an earlier position or request. It does not describe the current position.`;
  const full = analysis.operation?.kind === "full_reachability";
  const operation = full ? "Full cooperative mating query" : "Move analysis";
  if (analysis.status === "thinking")
    return `${operation} started for ${position}. ${full ? "This is not forced-win search or a live-game outcome. " : "Read-only suggestions will not be played. "}Waiting for a final native result.`;
  if (!accepted)
    return `No accepted analysis observation for ${position}. No live-game command was issued.`;
  if (full && analysis.result?.kind === "reachability" && analysis.operation?.kind === "full_reachability") {
    const description = reachabilityDescription(analysis.result.result, analysis.operation.target);
    return `${operation} ${analysis.status === "incomplete" ? "incomplete" : "completed"} for ${position}. ${description.title}. ${"unknown" in analysis.result.result ? "No mating conclusion was established. " : ""}This is not forced-win search or a live-game outcome.`;
  }
  if (analysis.status === "incomplete")
    return `Analysis incomplete. A work limit or stop ended the request for ${position}; it does not conclude the live game. Suggestions were not played.`;
  if (analysis.result?.kind === "move")
    return `Move analysis completed for ${position}. Suggested move: ${readableAction(analysis.result.action)}; not played.`;
  if (analysis.result?.kind === "claim")
    return `Move analysis completed for ${position}. Suggested ${choiceDescription({ kind: "claim", claim: analysis.result.claim })}; not exercised.`;
  if (analysis.result?.kind === "finished")
    return `Move analysis completed for ${position}. The analyzed position is concluded; no live-game command was issued.`;
  return `Analysis completed for ${position}. No live-game command was issued.`;
}
export function AnalysisPanel({
  snapshot,
  busy,
  connected,
  command,
}: {
  snapshot: Snapshot;
  busy: boolean;
  connected: boolean;
  command: (command: DesktopCommand) => Promise<Snapshot | null>;
}) {
  const [depth, setDepth] = useState("4");
  const [nodes, setNodes] = useState("100000");
  const [positions, setPositions] = useState("1000");
  const [transitions, setTransitions] = useState("10000");
  const [mate, setMate] = useState<ReachabilityTarget>("either");
  const [validation, setValidation] = useState("");
  const [notice, setNotice] = useState<{ key: string | null; summary: string; text: string }>({ key: null, summary: "", text: "" });
  const previousRequest = useRef<{ key: string | null; working: boolean } | null>(null);
  const defaults = snapshot.document.settings.analysis_defaults;
  const settingsKey = JSON.stringify(defaults);
  useEffect(() => {
    if (defaults.kind === "moves") {
      setDepth(String(defaults.depth));
      setNodes(String(defaults.nodes));
    } else {
      setMate(defaults.target);
      setPositions(String(defaults.max_positions));
      setTransitions(String(defaults.max_transitions));
    }
    setValidation("");
  }, [settingsKey, snapshot.document.path]);
  const analysis = snapshot.analysis;
  const working = analysis.status === "thinking";
  const disabled = busy || !connected;
  const telemetry = resultTelemetry(analysis.result);
  function moves(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const d = budget(depth, 8, false);
    const n = budget(nodes, 5_000_000, true);
    if (d === null || n === null) {
      setValidation(
        "Move analysis requires a whole-number depth from 1 to 8 and a node budget from 0 to 5,000,000.",
      );
      return;
    }
    setValidation("");
    void command({
      kind: "analyze",
      version: snapshot.version,
      operation: { kind: "moves", depth: d, nodes: n },
    });
  }
  function full(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const p = budget(positions, 100_000, true);
    const t = budget(transitions, 1_000_000, true);
    if (p === null || t === null) {
      setValidation(
        "Full reachability requires whole-number budgets: up to 100,000 positions and 1,000,000 transitions (zero is allowed for an immediate incomplete result).",
      );
      return;
    }
    setValidation("");
    void command({
      kind: "analyze",
      version: snapshot.version,
      operation: {
        kind: "full_reachability",
        target: mate,
        max_positions: p,
        max_transitions: t,
      },
    });
  }
  const purpose = analysis.context?.purpose;
  const requestTarget = purpose !== undefined && purpose.kind !== "engine_turn" ? purpose.target : null;
  const requestMatchesTarget = analysis.target !== null && requestTarget !== null &&
    analysis.context?.session_version === analysis.target.session_version &&
    requestTarget.session_version === analysis.target.session_version &&
    requestTarget.selection_id === analysis.target.selection_id &&
    requestTarget.board_ply === analysis.target.board_ply &&
    requestTarget.event_index === analysis.target.event_index &&
    requestTarget.turn.game_kind === "chess" && analysis.target.turn.game_kind === "chess" &&
    requestTarget.turn.side === analysis.target.turn.side &&
    requestTarget.label === analysis.target.label &&
    (analysis.operation?.kind === "moves" ? purpose?.kind === "move_analysis"
      : analysis.operation?.kind === "full_reachability" && purpose?.kind === "chess_full_reachability");
  const provenanceCurrent = requestMatchesTarget &&
    analysis.target !== null &&
    analysis.target.session_version === snapshot.version &&
    analysis.target.selection_id === snapshot.view.selection_id;
  const resultMatchesOperation =
    analysis.result === null || analysis.operation === null
      ? false
      : analysis.operation.kind === "full_reachability"
        ? analysis.result.kind === "reachability"
        : analysis.result.kind !== "reachability" &&
          analysis.result.kind !== "failure";
  const showResult =
    (analysis.status === "complete" || analysis.status === "incomplete") &&
    resultMatchesOperation && requestMatchesTarget;
  const targetLabel = analysis.target?.label ?? snapshot.view.label;
  const summary = analysisSummary(snapshot, provenanceCurrent, showResult);
  const requestKey = analysis.context === null ? null
    : `${analysis.context.session_version}:${analysis.context.request_id}:${analysis.context.process_generation}`;
  // Retire old text immediately when its native request or provenance changes.
  const announcement = notice.key === requestKey && notice.summary === summary ? notice.text : "";
  useEffect(() => {
    const replaced = working && previousRequest.current?.working === true &&
      previousRequest.current.key !== null && previousRequest.current.key !== requestKey;
    const next = replaced ? summary.replace(" started for ", " replacement started for ") : summary;
    previousRequest.current = { key: requestKey, working };
    setNotice((current) => current.key === requestKey && current.summary === summary && current.text === next
      ? current : { key: requestKey, summary, text: next });
    // Summary excludes changing counters/PVs. Identical native polls stay quiet.
  }, [summary, requestKey, working]);
  return (
    <section className="card analysis-card" aria-label="Read-only analysis">
      <div className="card-title">
        <h2>Read-only analysis</h2>
        <span className={`engine-phase ${analysis.status}`}>
          {working ? "Analyzing" : analysis.status}
        </span>
      </div>
      <p className="caption">
        Starting analysis pauses play. Results suggest or inspect; they never
        play a move, claim a draw or change the live game.
      </p>
      <p className="analysis-selection">
        Selected: {snapshot.view.label}
        {snapshot.view.historical ? " · historical prefix" : " · live position"}
      </p>
      <form className="analysis-form" onSubmit={moves} noValidate>
        <h3>Bounded move search</h3>
        <div className="analysis-fields">
          <label>
            Search depth
            <input
              inputMode="numeric"
              type="number"
              min="1"
              max="8"
              step="1"
              value={depth}
              onChange={(event) => setDepth(event.target.value)}
            />
          </label>
          <label>
            Node budget
            <input
              inputMode="numeric"
              type="number"
              min="0"
              max="5000000"
              step="1"
              value={nodes}
              onChange={(event) => setNodes(event.target.value)}
            />
          </label>
        </div>
        <AnalysisActions><button className="secondary" disabled={disabled} type="submit" data-analysis-start="moves">
          {working ? "Replace analysis" : "Start move analysis"}
        </button>
        {working && <button type="button" className="quiet" data-focus-after="analysis" disabled={disabled} onClick={() => {
          void command({ kind: "cancel_analysis", version: snapshot.version });
        }}>Cancel analysis</button>}</AnalysisActions>
      </form>
      <details className="advanced-analysis">
        <summary>Advanced: Full mating reachability</summary>
        <p className="caption">
          Cooperative legal play, not forced-win search. Both sides may
          cooperate to reach mate. Future draw claims and repetition/move-count
          endings are outside this geometric query; the live game keeps M2
          adjudication.
        </p>
        <form className="analysis-form" onSubmit={full} noValidate>
          <label>
            Mating side
            <select
              value={mate}
              onChange={(event) =>
                setMate(event.target.value as ReachabilityTarget)
              }
            >
              <option value="either">Either side</option>
              <option value="white">White</option>
              <option value="black">Black</option>
            </select>
          </label>
          <div className="analysis-fields">
            <label>
              Position budget
              <input
                type="number"
                inputMode="numeric"
                min="0"
                max="100000"
                step="1"
                value={positions}
                onChange={(event) => setPositions(event.target.value)}
              />
            </label>
            <label>
              Transition budget
              <input
                type="number"
                inputMode="numeric"
                min="0"
                max="1000000"
                step="1"
                value={transitions}
                onChange={(event) => setTransitions(event.target.value)}
              />
            </label>
          </div>
          <button type="submit" className="secondary" disabled={disabled} data-analysis-start="full">
            Run Full reachability
          </button>
        </form>
      </details>
      <p className="caption">
        Final native observations only. Every request has work budgets and a
        15-second process watchdog; these are not hard realtime guarantees.
      </p>
      {validation && (
        <p role="alert" className="validation-error">
          {validation}
        </p>
      )}
      <p className={announcement ? "analysis-notice" : "sr-only"} role="status"
        aria-label="Analysis status" aria-live="polite" aria-atomic="true">{announcement}</p>
      {analysis.status === "failed" && (
        <p className="analysis-notice" role="alert">
          Analysis process failed. No analysis conclusion or chess outcome was
          assigned. You may replace the analysis or resume play.
        </p>
      )}
      {analysis.target !== null && (
        <div className="analysis-result" aria-label="Analysis result">
          <h3>{targetLabel}</h3>
          <p className="caption">
            {targetTurnLabel(analysis.target)} to move ·{" "}
            {analysis.operation?.kind === "full_reachability"
              ? "Full cooperative reachability"
              : "Bounded move search"}
          </p>
          <p className="caption">
            {analysis.operation?.kind === "moves"
              ? `Recorded limit: depth ${analysis.operation.depth}, ${analysis.operation.nodes.toLocaleString()} nodes.`
              : analysis.operation?.kind === "full_reachability"
                ? `Recorded limit: ${analysis.operation.max_positions.toLocaleString()} positions, ${analysis.operation.max_transitions.toLocaleString()} transitions.`
                : ""}
          </p>
          {!provenanceCurrent && analysis.result !== null ? (
            <p className="analysis-notice">
              This result belongs to an earlier position or request. It does not
              describe the current position.
            </p>
          ) : !showResult ? (
            <p className="caption">
              {working
                ? "Waiting for a final native result."
                : "No accepted analysis observation to display. Technical details retain any rejected response."}
            </p>
          ) : (
            <>
              {analysis.result?.kind === "move" && (
                <p className="analysis-suggestion">
                  Suggested move: {readableAction(analysis.result.action)} · not
                  played
                </p>
              )}
              {analysis.result?.kind === "claim" && (
                <p className="analysis-suggestion">
                  Suggested{" "}
                  {choiceDescription({
                    kind: "claim",
                    claim: analysis.result.claim,
                  })}{" "}
                  · not exercised
                </p>
              )}
              {analysis.result?.kind === "finished" && (
                <p className="analysis-notice">
                  {conclusionName({ chess: analysis.result.outcome })}. This
                  analyzed position is concluded; no live-game command was
                  issued.
                </p>
              )}
              {telemetry !== null && (
                <>
                  <p className="analysis-score">
                    {scoreDescription(telemetry.score, targetColor(analysis.target))}
                  </p>
                  <dl className="telemetry">
                    <div>
                      <dt>Completed depth</dt>
                      <dd>{telemetry.completed_depth}</dd>
                    </div>
                    <div>
                      <dt>Visited nodes</dt>
                      <dd>{telemetry.nodes.toLocaleString()}</dd>
                    </div>
                  </dl>
                  {telemetry.principal_variation.length > 0 && (
                    <p className="variation">
                      <span>Suggested continuation · not played</span>
                      <br />
                      {telemetry.principal_variation
                        .map(choiceDescription)
                        .join(" · ")}
                    </p>
                  )}
                </>
              )}
              {analysis.result?.kind === "reachability" && (
                <ReachabilityResult
                  result={analysis.result.result}
                  target={
                    analysis.operation?.kind === "full_reachability"
                      ? analysis.operation.target
                      : "either"
                  }
                />
              )}
            </>
          )}
        </div>
      )}
      {(analysis.context !== null || analysis.message !== null) && (
        <details className="technical-analysis">
          <summary>Analysis technical details</summary>
          {analysis.message && <p>{analysis.message}</p>}
          <pre>
            {JSON.stringify(
              {
                target: analysis.target,
                operation: analysis.operation,
                context: analysis.context,
                result: analysis.result,
              },
              null,
              2,
            )}
          </pre>
        </details>
      )}
    </section>
  );
}
import type { MateReachability } from "./native";
function ReachabilityResult({
  result,
  target,
}: {
  result: MateReachability;
  target: ReachabilityTarget;
}) {
  const description = reachabilityDescription(result, target);
  const work =
    "possible" in result
      ? result.possible.work
      : "impossible" in result
        ? result.impossible.work
        : result.unknown.work;
  return (
    <div className="reachability-result">
      <h4>{description.title}</h4>
      <p>{description.explanation}</p>
      <dl className="telemetry">
        <div>
          <dt>Positions</dt>
          <dd>{work.positions.toLocaleString()}</dd>
        </div>
        <div>
          <dt>Transitions</dt>
          <dd>{work.transitions.toLocaleString()}</dd>
        </div>
      </dl>
      {"possible" in result && (
        <details>
          <summary>Cooperative route · never played</summary>
          <p className="variation">
            {result.possible.witness.length === 0
              ? "The target mate is already present."
              : result.possible.witness.map(readableAction).join(" · ")}
          </p>
        </details>
      )}
    </div>
  );
}
