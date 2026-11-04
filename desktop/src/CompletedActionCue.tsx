import type { CompletedActionGeometry } from "./boardPresentation";
import "./completed-action.css";

// Decoration only: centers and ordered phases are native-provided facts. No
// action parsing, legality, victim inference or interaction state lives here.
export function CompletedActionCue({ action, center, radius = 44 }: {
  action: CompletedActionGeometry | null | undefined;
  center: (key: string) => readonly [number, number] | null;
  radius?: number;
}) {
  if (!action || action.steps.length === 0 && !action.native_effects) return null;
  const from = action.origin.kind === "board" ? action.origin.square : action.origin.kind === "intersection" ? action.origin.point : null;
  const destination = action.steps.at(-1);
  const route = [...(from ? [from] : []), ...action.steps];
  const effects = action.effects ?? [], native = action.native_effects;
  const originKeys = [...new Set([...(from ? [from] : []), ...effects.map(effect => effect.from)])];
  const destinationKeys = [...new Set([...(destination ? [destination] : []), ...effects.map(effect => effect.to)])];
  function corners(key: string, kind: "origin" | "destination") {
    const point = center(key);
    if (!point) return null;
    const [x, y] = point, r = kind === "origin" ? radius - 5 : radius, arm = kind === "destination" ? 21 : 12;
    return <path key={`${kind}-${key}`} data-last-move={kind} data-location={key}
      data-cue-signature={kind === "destination" ? "double-underline" : "dashed-corners"}
      className={`completed-action-${kind}`} d={`M${x-r} ${y-r+arm}v${-arm}h${arm} M${x+r-arm} ${y-r}h${arm}v${arm} M${x+r} ${y+r-arm}v${arm}h${-arm} M${x-r+arm} ${y+r}h${-arm}v${-arm}${kind === "destination" ? ` M${x-r+10} ${y+r-3}h${2*r-20} M${x-r+10} ${y+r-9}h${2*r-20}` : ""}`} />;
  }
  function marker(key: string, phase: string, order: number, victim: number, pieceKind: string) {
    const point = center(key);
    if (!point) return null;
    return <g key={`${phase}-${order}-${victim}`} data-native-effect-phase={phase} data-effect-phase-index={order + 1}
      data-victim-index={victim + 1} data-location={key} data-effect-piece={pieceKind}
      {...(phase === "burn" ? { "data-native-burn": key } : phase === "capture" ? { "data-native-capture": key } : {})}
      transform={`translate(${point[0]} ${point[1]})`}>
      {phase === "burn" ? <><circle className="completed-native-burn" r="31" /><path className="completed-native-burn" d="M0 -24L11 -7L5 7L-7 15L-12 0Z" /></>
        : phase === "mover_dies" ? <path className="completed-native-destruction" d="M0 -31L31 0L0 31L-31 0ZM-12 -12L12 12M12 -12L-12 12" />
        : phase === "promotion" ? <circle className="completed-native-promotion" r="37" />
        : <path className="completed-action-capture" d="M-9 -9L9 9M9 -9L-9 9" />}
    </g>;
  }
  const routePoints = route.map(center);
  const stationary = native?.stationary_target ? center(native.stationary_target) : null;
  return <g className="completed-action-cue" aria-hidden="true" pointerEvents="none" data-native-last-action={action.action} data-last-event={action.event_index}>
    {action.steps.length > 1 && routePoints.every(point => point !== null) && <g className="completed-action-route" data-route={route.join(">")}>
      <polyline points={routePoints.map(point => point!.join(",")).join(" ")} />
      {action.steps.map((step, index) => {
        const point = center(step);
        return point && <g key={index} transform={`translate(${point[0] + radius - 9} ${point[1] - radius + 9})`}><circle r="10" /><text textAnchor="middle" y="4">{index + 1}</text></g>;
      })}
    </g>}
    {!native && action.captures?.map((capture, index) => {
      const point = center(capture.square);
      return point && <g key={`capture-${index}`} data-native-capture={capture.square} data-capture-order={index + 1} data-captured-kind={capture.original_kind}
        transform={`translate(${point[0]} ${point[1]})`}><path className="completed-action-capture" d="M-9 -9L9 9M9 -9L-9 9" /></g>;
    })}
    {native?.phases.map((phase, index) => phase.phase === "burn" ? phase.victims.map((victim, i) => marker(victim.square,"burn",index,i,victim.piece.base))
      : phase.phase === "capture" ? marker(phase.victim.square,"capture",index,0,phase.victim.piece.base)
      : phase.phase === "mover_dies" ? marker(phase.mover.square,"mover_dies",index,0,phase.mover.piece.base)
      : marker(phase.after.square,"promotion",index,0,phase.after.piece.base))}
    {stationary && <circle data-native-stationary-target={native!.stationary_target} className="completed-native-stationary" cx={stationary[0]} cy={stationary[1]} r="42" />}
    {originKeys.map(key => corners(key, "origin"))}
    {destinationKeys.map(key => corners(key, "destination"))}
  </g>;
}
