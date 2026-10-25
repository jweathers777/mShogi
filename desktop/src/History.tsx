import type { ArchivedObservation, DesktopCommand, Snapshot } from "./native";
import {
  choiceDescription,
  readableAction,
  reachabilityDescription,
  resultTelemetry,
  scoreDescription,
} from "./analysisPresentation";
import { claimName, conclusionName } from "./presentation";
export function HistoryPanel({
  snapshot,
  busy,
  connected,
  command,
}: {
  snapshot: Snapshot;
  busy: boolean;
  connected: boolean;
  command: (input: DesktopCommand) => Promise<Snapshot | null>;
}) {
  return (
    <section className="card history-card" aria-label="Game history">
      <div className="card-title">
        <h2>Game history</h2>
        <span className="caption">Native event timeline</span>
      </div>
      <p className="caption">
        Inspect an event without changing the live game. Claims and conclusions
        remain separate events; there is no branching.
      </p>
      <ol className="event-timeline" aria-label="Recorded game events">
        {snapshot.view.timeline.map((event) => (
          <li key={event.index}>
            <button
              className="quiet"
              aria-current={
                snapshot.view.event_index === event.index ? "step" : undefined
              }
              disabled={busy || !connected}
              onClick={() => {
                void command({
                  kind: "inspect",
                  version: snapshot.version,
                  event_index: event.index,
                });
              }}
            >
              <span>{event.label}</span>
              <small>{event.ply} plies</small>
            </button>
          </li>
        ))}
      </ol>
      <details>
        <summary>Played move identifiers</summary>
        {snapshot.history.length === 0 ? (
          <p className="empty-history">
            {snapshot.conclusion === null
              ? "Moves appear here as the game is played."
              : "No board moves were played."}
          </p>
        ) : (
          <ol className="played-actions">
            {snapshot.history.map((action, index) => (
              <li key={index}>{action}</li>
            ))}
          </ol>
        )}
      </details>
    </section>
  );
}
function Observation({ observation }: { observation: ArchivedObservation }) {
  const { result, context } = observation.response;
  const telemetry = resultTelemetry(result);
  const turn = observation.analyzed_turn;
  const full =
    context.purpose.kind === "full_reachability" ? context.purpose : null;
  return (
    <article className="archived-observation">
      <h3>{observation.label}</h3>
      <p className="caption">
        {observation.historical
          ? "Saved historical observation"
          : "Recorded observation"}{" "}
        · not a pending request or a current-position result
      </p>
      <p className="caption">
        {full !== null
          ? `Full cooperative reachability · ${full.limits.max_positions.toLocaleString()} positions / ${full.limits.max_transitions.toLocaleString()} transitions`
          : `Bounded search · depth ${context.limits.depth} / ${context.limits.nodes.toLocaleString()} nodes`}
      </p>
      {observation.application_error !== null ? (
        <p>
          Rejected response: {observation.application_error}. No valid score or
          proof is presented.
        </p>
      ) : result.kind === "failure" ? (
        <p>
          Process failure: {result.failure.message}. No chess outcome is
          implied.
        </p>
      ) : result.kind === "reachability" ? (
        full === null ? (
          <p>
            Unexpected proof response retained for inspection; no analysis
            conclusion is presented.
          </p>
        ) : (
          <>
            <p>{reachabilityDescription(result.result, full.mate).title}</p>
            <p className="caption">
              {reachabilityDescription(result.result, full.mate).explanation}
            </p>
          </>
        )
      ) : (
        <>
          <p>
            {result.kind === "move"
              ? `${context.purpose.kind === "engine_turn" ? "Engine-turn action" : "Read-only suggestion"}: ${readableAction(result.action)}`
              : result.kind === "claim"
                ? claimName(result.claim)
                : result.kind === "finished"
                  ? conclusionName({ chess: result.outcome })
                  : "Incomplete search · no completed action"}
          </p>
          {telemetry !== null && (
            <>
              <p>{scoreDescription(telemetry.score, turn)}</p>
              <p className="caption">
                Completed depth {telemetry.completed_depth} ·{" "}
                {telemetry.nodes.toLocaleString()} visited nodes.{" "}
                {context.purpose.kind !== "engine_turn" &&
                  "No action was applied by this analysis."}
              </p>
              {telemetry.principal_variation.length > 0 && (
                <p className="variation">
                  Recorded continuation:{" "}
                  {telemetry.principal_variation
                    .map(choiceDescription)
                    .join(" · ")}
                </p>
              )}
            </>
          )}
        </>
      )}
      <details className="technical-analysis">
        <summary>Recorded context, modes and limits</summary>
        <pre>{JSON.stringify(observation.response, null, 2)}</pre>
      </details>
    </article>
  );
}
export function SavedObservations({
  observations,
}: {
  observations: ArchivedObservation[];
}) {
  if (observations.length === 0) return null;
  return (
    <section
      className="card saved-observations"
      aria-label="Recorded search observations"
    >
      <details>
        <summary>Recorded search observations ({observations.length})</summary>
        <p className="caption">
          Exact request context and work limits are preserved. Reopening does
          not resume saved searches or apply their results.
        </p>
        {observations.map((observation) => (
          <Observation key={observation.sequence} observation={observation} />
        ))}
      </details>
    </section>
  );
}
