import { useLayoutEffect, useRef } from "react";

// Presentation only: native admission owns the decision point and transaction.
export function TakeBackControl({ available, disabled, contextKey, restore }: {
  available: boolean; disabled: boolean; contextKey: string; restore: () => Promise<unknown>;
}) {
  const focusAfterRestore = useRef<string | null>(null);
  useLayoutEffect(() => {
    if (focusAfterRestore.current !== null && focusAfterRestore.current !== contextKey && !disabled) {
      focusAfterRestore.current = null;
      document.querySelector<HTMLElement>("[data-game-status]")?.focus();
    }
  }, [contextKey, disabled]);
  return <button className="quiet take-back-button" disabled={disabled || !available} onClick={() => {
    focusAfterRestore.current = contextKey;
    void Promise.resolve(restore()).then((result) => { if (!result) focusAfterRestore.current = null; }, () => { focusAfterRestore.current = null; });
  }}>Take back</button>;
}
