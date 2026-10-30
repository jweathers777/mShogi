import { useLayoutEffect, useRef, type FocusEvent } from "react";
// Common non-board focus recovery. Native-choice selection and roving board
// focus remain with the specialized renderers until MSH-120.
export function useApplicationFocus(blocked: boolean) {
  const scope = useRef("board");
  const last = useRef<Element | null>(null), suspended = useRef<Element | null>(null);
  useLayoutEffect(() => {
    const previous = last.current;
    if (blocked || !previous || previous.closest('[role="dialog"]')) return;
    if (previous.isConnected && previous.matches(":disabled")) { suspended.current = previous; return; }
    if (previous.isConnected && suspended.current !== previous) return;
    if (document.activeElement !== document.body && document.activeElement !== document.documentElement) return;
    if (previous.isConnected && (previous instanceof HTMLElement || previous instanceof SVGElement)) {
      previous.focus(); if (document.activeElement === previous) suspended.current = null; return;
    }
    const kind = scope.current;
    const board = '[data-game-slot="board"] [tabindex="0"]';
    const targets: Record<string, string[]> = { analysis: ['[data-analysis-start]:not(:disabled)', '.inspector-switch button[aria-pressed="true"]:not(:disabled)'], engine: ['[data-focus-after="engine"]:not(:disabled)', board], outcome: ['[data-game-status]'], board: [board] };
    for (const selector of [...(targets[kind] ?? targets.board!), '[data-game-status]']) {
      const node = document.querySelector<HTMLElement | SVGElement>(selector); node?.focus();
      if (node && document.activeElement === node) { last.current = node; suspended.current = null; break; }
    }
  });
  return (event: FocusEvent<HTMLElement>) => { if (event.target instanceof Element) { last.current = event.target; scope.current = event.target.closest("[data-focus-after]")?.getAttribute("data-focus-after") ?? (event.target.closest('[data-shell-section="analysis"]') ? "analysis" : "board"); suspended.current = null; } };
}
