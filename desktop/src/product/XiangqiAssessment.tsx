import type { XiangqiSnapshot } from "../native";

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
  </details>;
}
