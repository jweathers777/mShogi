import type { CompletedActionGeometry } from "./boardPresentation";
import "./completed-action.css";

// Decoration only: centers are the renderer's native-coordinate layout. No
// action parsing, legality, history selection or interaction state lives here.
export function CompletedActionCue({ action, center, radius = 44 }: {
  action: CompletedActionGeometry | null | undefined;
  center: (key: string) => readonly [number, number] | null;
  radius?: number;
}) {
  if (!action || action.steps.length === 0) return null;
  const from = action.origin.kind === "board" ? action.origin.square : action.origin.kind === "intersection" ? action.origin.point : null;
  const destination = action.steps.at(-1)!;
  const route = [...(from ? [from] : []), ...action.steps];
  const effects = action.effects ?? [];
  const originKeys = [...new Set([...(from ? [from] : []), ...effects.map(effect => effect.from)])];
  const destinationKeys = [...new Set([destination, ...effects.map(effect => effect.to)])];
  function corners(key: string, kind: "origin" | "destination") {
    const point = center(key);
    if (!point) return null;
    const [x, y] = point, r = kind === "origin" ? radius - 5 : radius, arm = kind === "destination" ? 21 : 12;
    return <path key={`${kind}-${key}`} data-last-move={kind} data-location={key}
      data-cue-signature={kind === "destination" ? "double-underline" : "dashed-corners"}
      className={`completed-action-${kind}`} d={`M${x-r} ${y-r+arm}v${-arm}h${arm} M${x+r-arm} ${y-r}h${arm}v${arm} M${x+r} ${y+r-arm}v${arm}h${-arm} M${x-r+arm} ${y+r}h${-arm}v${-arm}${kind === "destination" ? ` M${x-r+10} ${y+r-3}h${2*r-20} M${x-r+10} ${y+r-9}h${2*r-20}` : ""}`} />;
  }
  const routePoints = route.map(center);
  return <g className="completed-action-cue" aria-hidden="true" pointerEvents="none" data-native-last-action={action.action} data-last-event={action.event_index}>
    {action.steps.length > 1 && routePoints.every(point => point !== null) && <g className="completed-action-route" data-route={route.join(">")}>
      <polyline points={routePoints.map(point => point!.join(",")).join(" ")} />
      {action.steps.map((step, index) => {
        const point = center(step);
        return point && <g key={index} transform={`translate(${point[0] + radius - 9} ${point[1] - radius + 9})`}><circle r="10" /><text textAnchor="middle" y="4">{index + 1}</text></g>;
      })}
    </g>}
    {originKeys.map(key => corners(key, "origin"))}
    {destinationKeys.map(key => corners(key, "destination"))}
  </g>;
}
