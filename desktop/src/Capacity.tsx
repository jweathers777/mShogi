import type { Snapshot } from "./native";

export function CapacityNotice({ capacity }: Pick<Snapshot, "capacity">) {
  if (capacity.status === "available") return null;
  return (
    <section
      className="notice capacity-notice"
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
