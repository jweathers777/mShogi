import type { XiangqiExactEvidence, XiangqiSnapshot } from "./native";
function savedMeaning(evidence: XiangqiExactEvidence) {
  const result = evidence.report.verdict;
  switch (result.kind) {
    case "neither_can_win": return "Exact proof: neither side can reach a natural board win.";
    case "natural_win": return `${result.winner === "red" ? "Red" : "Black"} has a possible natural board win. This is not a forced win.`;
    case "unknown": return result.reason === "cancelled" ? "Cancelled · Unknown. No conclusion was established." : "Unknown · exploration incomplete. No conclusion was established.";
    case "failure": return "Analysis failed. No conclusion was established.";
  }
}
export function XiangqiAssessment({ snapshot }: { snapshot: XiangqiSnapshot }) {
  const assessment = snapshot.xiangqi.no_win_assessment;
  const label = assessment.kind === "proven_neither_can_win" ? "No natural board win is possible"
    : assessment.kind === "proven_natural_win" ? "A natural board win is possible"
    : assessment.kind === "failure" ? "Assessment unavailable" : "Unknown";
  return <details className="position-assessment"><summary>Board-win assessment: {label}</summary>
    <p>{assessment.kind === "proven_neither_can_win" ? "Sound fast proof: neither side can reach a natural board win."
      : assessment.kind === "proven_natural_win" ? "Fast assessment found a possible winning continuation. This is not a forced win."
      : assessment.kind === "failure" ? "Assessment failed. No game result is implied."
      : "No proof is available from ordinary search. Unknown does not establish a win or draw."}</p>
    {snapshot.xiangqi.exact_observations.length > 0 && <details><summary>Saved exact reports ({snapshot.xiangqi.exact_observations.length})</summary>
      <p>Historical results for their recorded positions. They do not change your current game.</p>
      {snapshot.xiangqi.exact_observations.map((evidence, index) => <article key={index}><h4>Saved analysis · event {evidence.event_count}</h4>
        <p>{savedMeaning(evidence)}</p><p>{evidence.report.work.states.toLocaleString()} positions · {evidence.report.work.transitions.toLocaleString()} transitions</p>
        {evidence.report.verdict.kind === "natural_win" && <details><summary>Possible route · not played</summary><p>{evidence.report.verdict.witness.length ? evidence.report.verdict.witness.join(" · ") : "The target already has a natural board win."}</p></details>}
      </article>)}
    </details>}
  </details>;
}
