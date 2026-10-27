// Disk identity and native dirty state are separate: a pristine new document
// still has no save on disk. This changes labels, never native save/guard policy.
export function documentState({ path, dirty }: { path: string | null; dirty: boolean }) {
  if (path === null) return { kind: "unsaved", label: "Unsaved" } as const;
  return dirty
    ? { kind: "modified", label: "Unsaved changes" } as const
    : { kind: "unmodified", label: "Unmodified" } as const;
}
