import { cycleTabFocus } from "../interaction/focus";
import { useLayoutEffect, useRef, type ReactNode } from "react";
export function Dialog({ title, children, cancel, className = "" }: { title: string; children: ReactNode; cancel: () => void; className?: string }) {
  const ref = useRef<HTMLDivElement>(null);
  // Capture before this commit makes the background inert (and blurs it).
  const invoker = useRef(document.activeElement);
  useLayoutEffect(() => {
    (ref.current?.querySelector<HTMLElement>("[data-cancel]") ?? ref.current?.querySelector<HTMLElement>("button, select"))?.focus();
    return () => {
      const previous = invoker.current;
      queueMicrotask(() => {
        // Wait until inert has been removed; do not steal focus from a dialog
        // which immediately replaces this one (for example Save → New game).
        if (!document.activeElement?.closest('[role="dialog"]') && (previous instanceof HTMLElement || previous instanceof SVGElement) && previous.isConnected) previous.focus();
      });
    };
  }, []);
  return <div className="modal-backdrop product-backdrop"><div ref={ref} className={`product-dialog ${className}`} role="dialog" aria-modal="true" aria-label={title} onKeyDown={(e) => {
    if (e.key === "Escape") { e.preventDefault(); cancel(); }
    if (e.key === "Tab") cycleTabFocus(e, 'button:not(:disabled), select:not(:disabled), input:not(:disabled):not([type="hidden"]), a[href], summary');
  }}><h2>{title}</h2>{children}</div></div>;
}
