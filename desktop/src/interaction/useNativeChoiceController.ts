import { useLayoutEffect, useRef, useState, type FocusEvent, type KeyboardEvent } from "react";
import type { NativeChoice } from "../boardPresentation";
import type { InspectMovement, MovementInspection } from "../product/movementInspection";

export type Direction = "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown";
export interface SelectionOrigin { key: string; owner: string | undefined; inspectable: boolean }
export interface ChoiceControllerOptions {
  contextKey: string;
  choices: NativeChoice[];
  origins: SelectionOrigin[];
  humanOwner?: string | undefined;
  disabled: boolean;
  inspectionDisabled: boolean;
  onInspect?: InspectMovement | undefined;
  onPlay: (action: string) => unknown;
  initialFocus: string;
  boardKeys: string[];
  neighbor: (key: string, direction: Direction) => string | undefined;
  handGroups?: string[][];
}
type State = {
  context: string;
  selected: string | null;
  prefix: string[];
  completions: NativeChoice[];
  inspection: MovementInspection | null;
  inspectionPending: boolean;
  announcement: string;
};
const empty = (context: string, announcement = ""): State => ({ context, selected: null, prefix: [], completions: [], inspection: null, inspectionPending: false, announcement });

// Geometry helper, not movement rules. A renderer may supply a different neighbor
// function; opaque coordinates are never decoded by the controller.
export function gridNeighbor(keys: string[], columns: number, origin: string, direction: Direction): string | undefined {
  const index = keys.indexOf(origin);
  if (index < 0) return;
  const delta = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -columns, ArrowDown: columns }[direction];
  const next = index + delta;
  if (next < 0 || next >= keys.length || (Math.abs(delta) !== columns && Math.floor(next / columns) !== Math.floor(index / columns))) return;
  return keys[next];
}

/** One selection/route/inspection/completion/focus lifetime for every renderer. */
export function useNativeChoiceController(options: ChoiceControllerOptions) {
  const { contextKey, choices, origins, disabled, inspectionDisabled, onInspect, humanOwner, initialFocus, boardKeys, handGroups = [] } = options;
  // Native snapshots may be freshly allocated on every poll. Semantic equality
  // preserves selection; any changed capability/complete choice retires it.
  const context = JSON.stringify([contextKey, choices, origins, humanOwner, disabled, inspectionDisabled, initialFocus, boardKeys, handGroups]);
  const [stored, setStored] = useState<State>(() => empty(context));
  const [focused, setFocused] = useState(initialFocus);
  const state = stored.context === context ? stored : empty(context);
  const stateRef = useRef(state);
  stateRef.current = state;
  function transition(next: State) { stateRef.current = next; setStored(next); }
  const lifetime = useRef({ context, mounted: false, generation: 0, submitted: false });
  const elements = useRef(new Map<string, SVGGElement | HTMLButtonElement>());
  const completionRef = useRef<HTMLDivElement>(null);
  const returnFocus = useRef(initialFocus), hadFocus = useRef(false);
  // Also invalidate old handlers/results during a changed render, before effects.
  if (lifetime.current.context !== context) {
    lifetime.current = { ...lifetime.current, context, generation: lifetime.current.generation + 1, submitted: false };
  }
  function current() { return lifetime.current.mounted && lifetime.current.context === context; }
  function currentSelection() { return current() && stateRef.current === state; }
  function focus(key: string) {
    if (!current()) return;
    const element = elements.current.get(key);
    if (!element || (element instanceof HTMLButtonElement && element.disabled)) return;
    setFocused(key); element.focus(); hadFocus.current = true;
  }
  useLayoutEffect(() => {
    const lifetimeNow = lifetime.current;
    lifetimeNow.mounted = true; lifetimeNow.generation++;
    transition(empty(context));
    const target = elements.current.get(returnFocus.current);
    const focusedElement = document.activeElement;
    // Recover a disabled/removed board or completion control, without stealing
    // focus from a sidebar, file dialog or any unrelated user-selected control.
    const owned = [...elements.current.values()].includes(focusedElement as SVGGElement | HTMLButtonElement)
      || completionRef.current?.contains(focusedElement);
    if (hadFocus.current && (focusedElement === document.body || owned)) {
      if (target && !(target instanceof HTMLButtonElement && target.disabled)) focus(returnFocus.current);
      else focus(initialFocus);
    }
    if (!boardKeys.includes(focused) && !handGroups.some(group => group.includes(focused))) setFocused(initialFocus);
    return () => { lifetimeNow.mounted = false; lifetimeNow.generation++; };
  // Fresh callback identity can replace the native query provider as well.
  }, [context, onInspect]);
  useLayoutEffect(() => {
    if (state.completions.length) completionRef.current?.querySelector<HTMLButtonElement>("button:not(:disabled)")?.focus();
  }, [state.completions]);

  const activeChoices = state.inspection?.choices ?? (disabled ? [] : choices);
  const candidates = activeChoices.filter(choice => choice.from === state.selected && state.prefix.every((step, index) => choice.steps[index] === step));
  const terminals = state.prefix.length ? candidates.filter(choice => choice.steps.length === state.prefix.length) : [];
  const continuation = state.prefix.length > 0 && candidates.some(choice => choice.steps.length > state.prefix.length);
  const destinations = candidates.flatMap(choice => choice.steps[state.prefix.length] === undefined ? [] : [choice.steps[state.prefix.length]!]);
  function clear(announcement = "Selection cleared.") {
    if (!current()) return;
    lifetime.current.generation++;
    transition(empty(context, announcement));
  }
  function cancel() {
    if (!currentSelection()) return;
    const target = state.completions.length ? returnFocus.current : state.selected ?? returnFocus.current;
    clear(); focus(target);
  }
  function submit(choice: NativeChoice, route: string[], origin: string | null) {
    if (!currentSelection() || lifetime.current.submitted || disabled || state.inspection || state.inspectionPending
      || choice.from !== origin || choice.steps.length !== route.length || !route.every((step, i) => step === choice.steps[i])
      || !choices.some(native => JSON.stringify(native) === JSON.stringify(choice))) return;
    lifetime.current.submitted = true;
    clear("Complete native action submitted."); focus(returnFocus.current);
    options.onPlay(choice.action);
  }
  function finish(matches: NativeChoice[], route = state.prefix, origin = state.selected) {
    if (!currentSelection() || state.inspection || disabled || !matches.length) return;
    returnFocus.current = route.at(-1) ?? origin ?? initialFocus;
    if (matches.length === 1) submit(matches[0]!, route, origin);
    else transition({ ...state, prefix: route, completions: matches });
  }
  function chooseCompletion(action: string) {
    const choice = state.completions.find(item => item.action === action);
    if (choice) submit(choice, state.prefix, state.selected);
  }
  function canSelectOrigin(key: string) {
    const origin = origins.find(item => item.key === key);
    return !disabled && (humanOwner === undefined || origin?.owner === humanOwner) && choices.some(choice => choice.from === key);
  }
  function canInteractOrigin(key: string) {
    return canSelectOrigin(key) || (!!onInspect && !inspectionDisabled && !!origins.find(origin => origin.key === key)?.inspectable);
  }
  function select(key: string) {
    if (!currentSelection() || lifetime.current.submitted || state.completions.length || (disabled && (inspectionDisabled || !onInspect))) return;
    focus(key);
    const actionable = canSelectOrigin(key);
    function selectActionOrigin() {
      clear(); returnFocus.current = key;
      transition({ ...empty(context), selected: key, announcement: `${key} selected. Choose a highlighted destination.` });
    }
    // Complete native routes take precedence over occupied enemy inspection and
    // toggling the origin (a valid route may return to that very origin).
    const next = candidates.filter(choice => choice.steps[state.prefix.length] === key);
    if (next.length) {
      const route = [...state.prefix, key];
      returnFocus.current = key;
      const terminal = next.filter(choice => choice.steps.length === route.length);
      if (next.some(choice => choice.steps.length > route.length)) {
        transition({ ...state, prefix: route, announcement: `${state.selected} → ${route.join(" → ")} selected. Continue to a highlighted square${terminal.length && !state.inspection ? ", or choose Complete action here" : ""}. No action has been submitted.` });
      } else if (state.inspection) {
        transition({ ...state, prefix: route, announcement: `Inspection route complete. ${terminal.map(choice => choice.completion?.label ?? "Complete route").join(" or ")}. Read-only; no action submitted.` });
      } else finish(terminal, route);
      return;
    }
    if (state.selected === key) { clear(); return; }
    if (actionable) { selectActionOrigin(); return; }
    if (onInspect && !inspectionDisabled && origins.find(origin => origin.key === key)?.inspectable) {
      clear(); returnFocus.current = key;
      transition({ ...empty(context), selected: key, inspectionPending: true, announcement: "Loading read-only movement inspection." });
      const generation = ++lifetime.current.generation;
      void (async () => onInspect(key))().then(result => {
        if (!current() || lifetime.current.generation !== generation) return;
        if (result?.read_only === true && result.origin === key && result.choices.every(choice => choice.from === key)) {
          transition({ ...empty(context), selected: key, inspection: result, announcement: `Inspecting ${result.label}. Read-only movement choices; no action can be played.` });
        } else transition(empty(context, "Movement inspection is unavailable for this context. Select the piece again to query the current position."));
      }).catch(() => {
        if (current() && lifetime.current.generation === generation) transition(empty(context, "Movement inspection is unavailable for this context."));
      });
    } else transition({ ...state, announcement: "Choose a piece or one of its marked destinations. Escape cancels." });
  }
  function keyDown(event: KeyboardEvent, key: string) {
    if (!current()) return;
    if (["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"].includes(event.key)) {
      event.preventDefault();
      const direction = event.key as Direction;
      const group = handGroups.find(group => group.includes(key));
      if (group) {
        if (direction === "ArrowUp" || direction === "ArrowDown") focus(initialFocus);
        else {
          const available = group.filter(canInteractOrigin), index = available.indexOf(key);
          const next = available[(index + (direction === "ArrowLeft" ? -1 : 1) + available.length) % available.length];
          if (next) focus(next);
        }
      } else {
        const next = options.neighbor(key, direction);
        if (next) focus(next);
      }
    } else if (event.key === "Enter" || event.key === " ") { event.preventDefault(); select(key); }
    else if (event.key === "Escape") { event.preventDefault(); cancel(); }
  }
  function completionKey(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") { event.preventDefault(); event.stopPropagation(); cancel(); }
    else if (event.key === "Tab") {
      event.preventDefault();
      const buttons = [...(completionRef.current?.querySelectorAll<HTMLButtonElement>("button:not(:disabled)") ?? [])];
      const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
      buttons[(index + (event.shiftKey ? -1 : 1) + buttons.length) % buttons.length]?.focus();
    }
  }
  function onFocusCapture() { hadFocus.current = true; }
  function onBlurCapture(event: FocusEvent) {
    if (!event.currentTarget.contains(event.relatedTarget as Node | null)) {
      const target = event.target;
      // WebKit may blur disabled/removed hand controls to body before a commit.
      hadFocus.current = event.relatedTarget === null && target instanceof HTMLButtonElement && (target.disabled || !target.isConnected);
    }
  }
  return {
    ...state, focused, destinations, terminals, continuation, completionRef,
    completionTarget: returnFocus.current, interactionDisabled: (disabled && (inspectionDisabled || !onInspect)) || state.completions.length > 0,
    register: (key: string, element: SVGGElement | HTMLButtonElement | null) => { if (element) elements.current.set(key, element); else elements.current.delete(key); },
    onFocus: (key: string) => { setFocused(key); hadFocus.current = true; },
    onFocusCapture, onBlurCapture, select, keyDown, completionKey, cancel, chooseCompletion, completeHere: () => finish(terminals),
    canInteractOrigin,
  };
}
