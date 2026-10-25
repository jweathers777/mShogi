import { useLayoutEffect, useRef } from "react";
import type { Snapshot } from "./native";

export function CapacityNotice({ capacity }: Pick<Snapshot, "capacity">) {
  const notice = useRef<HTMLElement>(null);
  useLayoutEffect(() => {
    // A rejected operation can disable the focused analysis control while this
    // notice is above the scrolled inspector. Reveal each new native pause once;
    // ordinary snapshot polls must preserve the user's subsequent focus.
    if (capacity.status === "paused") notice.current?.focus();
  }, [capacity.id, capacity.status]);
  if (capacity.status === "available") return null;
  return (
    <section
      className="notice capacity-notice"
      ref={notice}
      tabIndex={-1}
      role={capacity.status === "paused" ? "alert" : "status"}
      aria-label="Document capacity"
    >
      <strong>
        {capacity.status === "checking"
          ? "Checking complete save capacity"
          : "Application limit reached"}
      </strong>
      <p>
        {capacity.message ??
          "The native host is checking this operation before adding it to the saved document."}
      </p>
      {capacity.status === "paused" && (
        <>
          <p>
            This is an application resource limit. No chess result or illegal
            move is implied. Your complete accepted game and observations are
            preserved.
          </p>
          <p>
            Save or Save as to keep this document, then start a new game or open
            another document to continue. Saving and reopening retains all
            recorded history and observations; it does not reset their limits.
          </p>
        </>
      )}
    </section>
  );
}
