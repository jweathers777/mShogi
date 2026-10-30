import { useEffect, useRef, useState } from "react";
import { choiceDescription, readableAction, reachabilityDescription, resultTelemetry, scoreDescription } from "../analysisPresentation";
import { conclusionName } from "../presentation";
import type { AnalysisOperation, AnalysisState, ExecutionObservation, MateReachability, ReachabilityTarget } from "../native";
import type { GameSnapshot, Operation } from "./native";

export function ReachabilityEvidence({ result, target }: { result: MateReachability; target: ReachabilityTarget }) {
  const text = reachabilityDescription(result, target);
  const work = "possible" in result ? result.possible.work : "impossible" in result ? result.impossible.work : result.unknown.work;
  return <div className="reachability-result"><h4>{text.title}</h4><p>{text.explanation}</p><dl className="telemetry"><div><dt>Positions</dt><dd>{work.positions.toLocaleString()}</dd></div><div><dt>Transitions</dt><dd>{work.transitions.toLocaleString()}</dd></div></dl>{"possible" in result && <details><summary>Cooperative route · never played</summary><p>{result.possible.witness.length ? result.possible.witness.map(readableAction).join(" · ") : "The target mate is already present."}</p></details>}</div>;
}
function sameTarget(analysis: AnalysisState, snapshot: GameSnapshot) {
  const target = analysis.target;
  const purpose = analysis.context?.purpose;
  const request = purpose && purpose.kind !== "engine_turn" ? purpose.target : null;
  return target && request && analysis.context?.session_version === target.session_version &&
    target.session_version === snapshot.token.version && target.selection_id === snapshot.token.selection &&
    request.session_version === target.session_version && request.selection_id === target.selection_id &&
    request.event_index === target.event_index && request.board_ply === target.board_ply &&
    request.label === target.label && request.turn.game_kind === "chess" && target.turn.game_kind === "chess" && request.turn.side === target.turn.side &&
    (analysis.operation?.kind === "moves" ? purpose?.kind === "move_analysis" : analysis.operation?.kind === "full_reachability" && purpose?.kind === "chess_full_reachability" && purpose.mate === analysis.operation.target && purpose.limits.max_positions === analysis.operation.max_positions && purpose.limits.max_transitions === analysis.operation.max_transitions) &&
    (analysis.operation?.kind !== "moves" || (analysis.context?.limits.depth === analysis.operation.depth && analysis.context.limits.nodes === analysis.operation.nodes));
}
function analysisSummary(analysis: AnalysisState, current: boolean, accepted: boolean): string {
  if (analysis.status === "idle" || analysis.status === "failed") return "";
  if (analysis.status === "cancelled") return "Analysis cancelled. No conclusion was established; play remains paused until you explicitly resume.";
  if (!analysis.target) return "Waiting for an identified native analysis target.";
  const side = analysis.target.turn.game_kind === "chess" ? (analysis.target.turn.side === "white" ? "White" : "Black") : "Unknown side";
  const position = `${analysis.target.label}, ${side} to move`;
  if (!current) return `Analysis for recorded target ${position} belongs to an earlier position or request. It does not describe the current position.`;
  const full = analysis.operation?.kind === "full_reachability";
  const operation = full ? "Full cooperative mating query" : "Move analysis";
  if (analysis.status === "thinking") return `${operation} started for ${position}. ${full ? "This is not forced-win search or a live-game outcome." : "Read-only suggestions will not be played."} Waiting for a final native result.`;
  if (!accepted) return `No accepted analysis observation for ${position}. No live-game command was issued.`;
  if (analysis.result?.kind === "reachability" && analysis.operation?.kind === "full_reachability") {
    const text = reachabilityDescription(analysis.result.result, analysis.operation.target);
    return `${operation} ${analysis.status === "incomplete" ? "incomplete" : "completed"} for ${position}. ${text.title}. ${"unknown" in analysis.result.result ? "No mating conclusion was established. " : ""}This is not forced-win search or a live-game outcome.`;
  }
  if (analysis.status === "incomplete") return `Analysis incomplete. A work limit or stop ended the request for ${position}; it does not conclude the live game. Suggestions were not played.`;
  if (analysis.result?.kind === "move") return `Move analysis completed for ${position}. Suggested move: ${readableAction(analysis.result.action)}; not played.`;
  if (analysis.result?.kind === "claim") return `Move analysis completed for ${position}. Suggested ${choiceDescription({ kind: "claim", claim: analysis.result.claim })}; not exercised.`;
  return `Analysis completed for ${position}. No live-game command was issued.`;
}
export function ChessAnalysisContent({ snapshot, disabled, command, defaults }: { snapshot: GameSnapshot; disabled: boolean; command: (operation: Operation) => unknown; defaults: AnalysisOperation }) {
  const [target, setTarget] = useState<ReachabilityTarget>("either"), [positions, setPositions] = useState("1000"), [transitions, setTransitions] = useState("10000");
  useEffect(() => { if (defaults.kind === "full_reachability") { setTarget(defaults.target); setPositions(String(defaults.max_positions)); setTransitions(String(defaults.max_transitions)); } }, [snapshot.token.document]);
  const analysis = snapshot.game === "chess" ? snapshot.chess.analysis : null;
  const [notice, setNotice] = useState({ key: "", summary: "", text: "" });
  const previousRequest = useRef<{ key: string; working: boolean } | null>(null);
  const current = analysis ? Boolean(sameTarget(analysis, snapshot)) : false;
  const full = analysis?.operation?.kind === "full_reachability";
  const purpose = analysis?.context?.purpose;
  const accepted = Boolean(analysis && current && (analysis.status === "complete" || analysis.status === "incomplete") && analysis.result !== null &&
    (full ? purpose?.kind === "chess_full_reachability" && analysis.result.kind === "reachability" : purpose?.kind === "move_analysis" && analysis.result.kind !== "reachability" && analysis.result.kind !== "failure"));
  const summary = analysis ? analysisSummary(analysis, current, accepted) : "";
  const requestKey = analysis?.context ? `${analysis.context.session_version}:${analysis.context.request_id}:${analysis.context.process_generation}` : "";
  const working = analysis?.status === "thinking";
  useEffect(() => {
    const replaced = working && previousRequest.current?.working && previousRequest.current.key !== requestKey;
    previousRequest.current = { key: requestKey, working };
    const text = replaced ? summary.replace(" started for ", " replacement started for ") : summary;
    setNotice((current) => current.key === requestKey && current.summary === summary && current.text === text ? current : { key: requestKey, summary, text });
  }, [summary, requestKey, working]);
  const announcement = notice.key === requestKey && notice.summary === summary ? notice.text : "";
  if (!analysis) return null;
  const telemetry = accepted ? resultTelemetry(analysis.result) : null;
  const turn = analysis.target?.turn.game_kind === "chess" ? analysis.target.turn.side : null;
  return <>
    <details className="advanced-analysis"><summary>Advanced: Full mating reachability</summary><p className="caption">Cooperative legal play, not forced-win search. Both sides may cooperate to reach mate. Future draw claims and repetition/move-count endings are outside this geometric query; the live game keeps M2 adjudication.</p><form onSubmit={(event) => { event.preventDefault(); if (event.currentTarget.checkValidity() && !disabled) void command({ kind: "analyze", operation: { kind: "full_reachability", target, max_positions: Number(positions), max_transitions: Number(transitions) } }); }}><label>Mating side<select value={target} onChange={(e) => setTarget(e.target.value as ReachabilityTarget)}><option value="either">Either side</option><option value="white">White</option><option value="black">Black</option></select></label><div className="shogi-analysis-fields"><label>Position budget<input type="number" min="0" max="100000" step="1" required value={positions} onChange={(e) => setPositions(e.target.value)} /></label><label>Transition budget<input type="number" min="0" max="1000000" step="1" required value={transitions} onChange={(e) => setTransitions(e.target.value)} /></label></div><button className="secondary" data-analysis-start="full" disabled={disabled}>Run Full reachability</button></form></details>
    <p className="analysis-notice" role="status" aria-label="Analysis status" aria-live="polite" aria-atomic="true">{announcement}</p>
    {analysis.status === "failed" && <p role="alert">Analysis process failed. No analysis conclusion or chess outcome was assigned. You may replace the analysis or resume play.</p>}
    {analysis.target && <div className="analysis-result" aria-label="Analysis result"><h3>{analysis.target.label}</h3><p className="analysis-perspective">{turn === "white" ? "White" : turn === "black" ? "Black" : "Unknown side"} perspective · {full ? "Full cooperative reachability" : "Bounded move search"}</p><p className="caption">{analysis.operation?.kind === "moves" ? `Recorded limit: depth ${analysis.operation.depth}, ${analysis.operation.nodes.toLocaleString()} nodes.` : analysis.operation?.kind === "full_reachability" ? `Recorded limit: ${analysis.operation.max_positions.toLocaleString()} positions, ${analysis.operation.max_transitions.toLocaleString()} transitions.` : ""}</p>
      {!current && analysis.result ? <p>This result belongs to an earlier position or request. It does not describe the current position.</p> : !accepted ? <p>{analysis.status === "thinking" ? "Waiting for a final native result." : "No accepted analysis observation to display. Technical details retain any rejected response."}</p> : <>
        {analysis.result?.kind === "move" && <p>Suggested move: {readableAction(analysis.result.action)} · not played</p>}
        {analysis.result?.kind === "claim" && <p>Suggested {choiceDescription({ kind: "claim", claim: analysis.result.claim })} · not exercised</p>}
        {analysis.result?.kind === "finished" && <p>{conclusionName({ chess: analysis.result.outcome })}. This analyzed position is concluded; no live-game command was issued.</p>}
        {telemetry && <><p className="analysis-score">{scoreDescription(telemetry.score, turn)}</p><dl className="shogi-telemetry"><div><dt>Completed depth</dt><dd>{telemetry.completed_depth}</dd></div><div><dt>Nodes</dt><dd>{telemetry.nodes.toLocaleString()}</dd></div></dl>{telemetry.principal_variation.length > 0 && <p>Suggested continuation · not played: {telemetry.principal_variation.map(choiceDescription).join(" · ")}</p>}</>}
        {analysis.result?.kind === "reachability" && analysis.operation?.kind === "full_reachability" && <ReachabilityEvidence result={analysis.result.result} target={analysis.operation.target} />}
      </>}
    </div>}
  </>;
}
export function ChessSavedEvidence({ observation, rejected = false }: { observation: ExecutionObservation; rejected?: boolean }) {
  if (rejected) return <p>Rejected response. No valid score or proof is presented.</p>;
  const result = observation.result.kind === "chess" ? observation.result.result : null;
  const purpose = observation.context.purpose;
  if (!result) return null;
  return <>{purpose.kind !== "engine_turn" && <p>{purpose.target.label} · saved historical observation, not a pending request</p>}{result.kind === "reachability" && purpose.kind === "chess_full_reachability" ? <ReachabilityEvidence result={result.result} target={purpose.mate} /> : result.kind === "claim" ? <p>{choiceDescription({ kind: "claim", claim: result.claim })} · not exercised by this observation</p> : result.kind === "failure" ? <p>Process failure. No chess outcome is implied.</p> : result.kind === "incomplete" ? <p>Incomplete search · no completed action</p> : null}</>;
}
