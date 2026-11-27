import { inspectionStatus } from "./product/movementInspection";
import { useId } from "react";
import type { NativeChoiceController } from "./interaction/useNativeChoiceController";

// The controls that every board shares: one help line, route controls, live
// announcement, completion dialog and square state wording.

/** Accessible name of a board point: its native label plus the selection state. */
export function pointLabel(key: string, piece: { label: string } | undefined, interaction: NativeChoiceController) {
  const { selected, inspection, inspectionPending, prefix, destinations } = interaction;
  const base = piece?.label ?? `${key}, empty`;
  const target = destinations.includes(key);
  return `${base}${selected === key ? inspection ? `, Inspecting ${inspection.label}` : inspectionPending ? ", loading movement inspection" : ", selected" : ""}${prefix.includes(key) ? ", selected route step" : ""}${target ? inspection ? ", read-only inspection destination" : ", legal destination" : ""}`;
}

/** Help line, partial-route controls and the live announcement under a board. */
export function BoardFooter({ interaction, disabled }: { interaction: NativeChoiceController; disabled: boolean }) {
  const { inspection, inspectionPending, selectedLabel, selected, prefix } = interaction;
  return <div className="shell-board-help">
    <p className={`board-help${inspection || inspectionPending ? " movement-inspection-status" : selectedLabel ? " selected-piece-status" : ""}`}>{inspection ? inspectionStatus(inspection.label) : inspectionPending ? "Loading read-only movement inspection…" : selectedLabel ? `Selected ${selectedLabel} · Esc clears` : "Arrows: move focus · Enter/Space: select · Esc: cancel"}</p>
    {interaction.continuation && <div className="route-selection" role="group" aria-label="Partial route selection" onKeyDown={event => { if (event.key === "Escape") { event.preventDefault(); interaction.cancel(); } }}>
      <p>Selected route: {selected} → {prefix.join(" → ")}. No action submitted.</p>
      {interaction.terminals.length > 0 && !inspection && <button disabled={disabled} onClick={interaction.completeHere}>Complete action here</button>}
      <button className="quiet" onClick={interaction.cancel}>Cancel selection</button>
    </div>}
    <span className="sr-only" role="status" aria-live="polite">{interaction.announcement}</span>
  </div>;
}

/** Choice between complete actions that share a route; a promotion in Chess and Shogi. */
export function CompletionDialog({ interaction, promotion }: { interaction: NativeChoiceController; promotion: boolean }) {
  const title = useId();
  if (!interaction.completions.length) return null;
  return <div className="modal-backdrop"><div className="promotion-dialog" role="dialog" aria-modal="true" aria-labelledby={title}
    ref={interaction.completionRef} onKeyDown={interaction.completionKey}>
    <p className="eyebrow">COMPLETE YOUR MOVE</p><h2 id={title}>{promotion ? "Choose promotion" : "Choose complete action"}</h2>
    <p>The action is applied only after you choose.</p>
    <div className="promotion-choices">{interaction.completions.map(choice => <button key={choice.action} onClick={() => interaction.chooseCompletion(choice.action)}>
      {choice.completion?.glyph && <span aria-hidden="true">{choice.completion.glyph}</span>}{choice.completion?.label ?? choice.action}
    </button>)}</div>
    <button className="quiet" onClick={interaction.cancel}>{promotion ? "Cancel promotion" : "Cancel selection"}</button>
  </div></div>;
}
