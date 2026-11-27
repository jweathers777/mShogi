import type { AnalysisState, ExecutionObservation, GameSnapshot } from "../native";
import { useEffect, useRef, useState } from "react";
import { choiceDescription, readableAction, resultTelemetry, scoreDescription } from "../analysisPresentation";
import { sideLabel } from "../games";
import { sessionStatus } from "../sessionState";
import { conclusionName } from "../presentation";

function sameTarget(analysis: AnalysisState, snapshot: GameSnapshot) {
  const target = analysis.target;
  const purpose = analysis.context?.purpose;
  const request = purpose && purpose.kind !== "engine_turn" ? purpose.target : null;
  return target && request && analysis.context?.session_version === target.session_version &&
    target.session_version === snapshot.token.version && target.selection_id === snapshot.token.selection &&
    request.session_version === target.session_version && request.selection_id === target.selection_id &&
    request.event_index === target.event_index && request.board_ply === target.board_ply &&
    request.label === target.label && request.turn.game_kind === "chess" && target.turn.game_kind === "chess" && request.turn.side === target.turn.side &&
    purpose?.kind === "move_analysis" && analysis.operation !== null &&
    analysis.context?.limits.depth === analysis.operation.depth && analysis.context.limits.nodes === analysis.operation.nodes;
}
function analysisSummary(analysis: AnalysisState, current: boolean, accepted: boolean, working: boolean): string {
  if (analysis.status === "idle" || analysis.status === "failed") return "";
  if (analysis.status === "cancelled") return "Analysis cancelled. No conclusion was established; play remains paused until you explicitly resume.";
  if (!analysis.target) return "Waiting for the selected analysis position.";
  const side = analysis.target.turn.game_kind === "chess" ? sideLabel("chess", analysis.target.turn.side) : "Unknown side";
  const position = `${analysis.target.label}, ${side} to move`;
  if (!current) return `Analysis for recorded target ${position} belongs to an earlier position or request. It does not describe the current position.`;
  if (working) return `Move analysis started for ${position}. Read-only suggestions will not be played. Waiting for a completed result.`;
  if (!accepted) return `No accepted analysis observation for ${position}. No live-game command was issued.`;
  if (analysis.status === "incomplete") return `Analysis incomplete. A work limit or stop ended the request for ${position}; it does not conclude the live game. Suggestions were not played.`;
  if (analysis.result?.kind === "move") return `Move analysis completed for ${position}. Suggested move: ${readableAction(analysis.result.action)}; not played.`;
  if (analysis.result?.kind === "claim") return `Move analysis completed for ${position}. Suggested ${choiceDescription({ kind: "claim", claim: analysis.result.claim })}; not exercised.`;
  return `Analysis completed for ${position}. No live-game command was issued.`;
}
export function ChessAnalysisContent({ snapshot }: { snapshot: GameSnapshot }) {
  const analysis = snapshot.game === "chess" ? snapshot.chess.analysis : null;
  const [notice, setNotice] = useState({ key: "", summary: "", text: "" });
  const previousRequest = useRef<{ key: string; working: boolean } | null>(null);
  const current = analysis ? Boolean(sameTarget(analysis, snapshot)) : false;
  const accepted = Boolean(analysis && current && (analysis.status === "complete" || analysis.status === "incomplete") && analysis.result !== null &&
    analysis.context?.purpose.kind === "move_analysis" && analysis.result.kind !== "failure");
  const working = analysis !== null && sessionStatus(snapshot) === "analyzing";
  const summary = analysis ? analysisSummary(analysis, current, accepted, working) : "";
  const requestKey = analysis?.context ? `${analysis.context.session_version}:${analysis.context.request_id}` : "";
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
    <p className="analysis-notice" role="status" aria-label="Analysis status" aria-live="polite" aria-atomic="true">{announcement}</p>
    {analysis.status === "failed" && <p role="alert">Analysis process failed. No analysis conclusion or chess outcome was assigned. You may replace the analysis or resume play.</p>}
    {analysis.target && <div className="analysis-result" aria-label="Analysis result"><h3>{analysis.target.label}</h3><p className="analysis-perspective">{turn === null ? "Unknown side" : sideLabel("chess", turn)} perspective · Bounded move search</p><p className="caption">{analysis.operation ? `Recorded limit: depth ${analysis.operation.depth}, ${analysis.operation.nodes.toLocaleString()} nodes.` : ""}</p>
      {!current && analysis.result ? <p>This result belongs to an earlier position or request. It does not describe the current position.</p> : !accepted ? <p>{working ? "Waiting for a completed result." : "No accepted result to display. Rejected responses are available in About mShogi."}</p> : <>
        {analysis.result?.kind === "move" && <p>Suggested move: {readableAction(analysis.result.action)} · not played</p>}
        {analysis.result?.kind === "claim" && <p>Suggested {choiceDescription({ kind: "claim", claim: analysis.result.claim })} · not exercised</p>}
        {analysis.result?.kind === "finished" && <p>{conclusionName({ chess: analysis.result.outcome })}. This analyzed position is concluded; no live-game command was issued.</p>}
        {telemetry && <><p className="analysis-score">{scoreDescription(telemetry.score, turn)}</p><dl className="shogi-telemetry"><div><dt>Completed depth</dt><dd>{telemetry.completed_depth}</dd></div><div><dt>Selective depth</dt><dd>{telemetry.selective_depth}</dd></div><div><dt>Nodes</dt><dd>{telemetry.nodes.toLocaleString()}</dd></div></dl>{telemetry.principal_variation.length > 0 && <p>Suggested continuation · not played: {telemetry.principal_variation.map(choiceDescription).join(" · ")}</p>}</>}
      </>}
    </div>}
  </>;
}
export function ChessSessionEvidence({ observation, rejected = false }: { observation: ExecutionObservation; rejected?: boolean }) {
  if (rejected) return <p>Rejected response. No valid score or proof is presented.</p>;
  const result = observation.result.kind === "chess" ? observation.result.result : null;
  const purpose = observation.context.purpose;
  if (!result) return null;
  return <>{purpose.kind !== "engine_turn" && <p>{purpose.target.label} · historical session observation, not a pending request</p>}{result.kind === "claim" ? <p>{choiceDescription({ kind: "claim", claim: result.claim })} · not exercised by this observation</p> : result.kind === "failure" ? <p>Process failure. No chess outcome is implied.</p> : result.kind === "incomplete" ? <p>Incomplete search · no completed action</p> : null}</>;
}
