import { useState, type FormEvent } from "react";
import type { DesktopCommand, ReachabilityTarget, Snapshot } from "./native";
import {
  choiceDescription,
  readableAction,
  reachabilityDescription,
  resultTelemetry,
  scoreDescription,
} from "./analysisPresentation";
import { colorName, conclusionName } from "./presentation";

function budget(value: string, max: number, allowZero: boolean): number | null {
  if (!/^\d+$/.test(value)) return null;
  const parsed = Number(value);
  return Number.isSafeInteger(parsed) &&
    parsed >= (allowZero ? 0 : 1) &&
    parsed <= max
    ? parsed
    : null;
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
  command: (command: DesktopCommand) => Promise<void>;
}) {
  const [depth, setDepth] = useState("4");
  const [nodes, setNodes] = useState("100000");
  const [positions, setPositions] = useState("1000");
  const [transitions, setTransitions] = useState("10000");
  const [mate, setMate] = useState<ReachabilityTarget>("either");
  const [validation, setValidation] = useState("");
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
  const provenanceCurrent =
    analysis.target !== null &&
    analysis.target.session_version === snapshot.version;
  const resultMatchesOperation =
    analysis.result === null || analysis.operation === null
      ? false
      : analysis.operation.kind === "full_reachability"
        ? analysis.result.kind === "reachability"
        : analysis.result.kind !== "reachability" &&
          analysis.result.kind !== "failure";
  const showResult =
    (analysis.status === "complete" || analysis.status === "incomplete") &&
    resultMatchesOperation;
  const targetLabel =
    analysis.target?.label ??
    `Live position after ${snapshot.history.length} plies`;
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
        <button className="secondary" disabled={disabled} type="submit">
          {working ? "Replace with move analysis" : "Start move analysis"}
        </button>
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
          <button type="submit" className="secondary" disabled={disabled}>
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
      {analysis.status === "cancelled" && (
        <p className="analysis-notice" role="status">
          Analysis cancelled. No conclusion was established; play remains paused
          until you explicitly resume.
        </p>
      )}
      {analysis.status === "incomplete" && (
        <p className="analysis-notice" role="status">
          Analysis incomplete. A work limit or stop ended this request; it does
          not conclude the live game.
        </p>
      )}
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
            {colorName(analysis.target.turn)} to move ·{" "}
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
                    {scoreDescription(telemetry.score, analysis.target.turn)}
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
