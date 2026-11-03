import { useMemo, useState } from "react";

// Bound browser text layout, independently of the complete native archive limits.
export const DIAGNOSTIC_PAGE_CHARACTERS = 64 * 1024;

function pageEnds(text: string): number[] {
  const ends = [0];
  while (ends.at(-1)! < text.length) {
    let end = Math.min(ends.at(-1)! + DIAGNOSTIC_PAGE_CHARACTERS, text.length);
    // A page must not split a Unicode scalar when copied or read separately.
    if (end < text.length && /[\uD800-\uDBFF]/.test(text[end - 1]!) && /[\uDC00-\uDFFF]/.test(text[end]!)) end--;
    ends.push(end);
  }
  if (ends.length === 1) ends.push(0);
  return ends;
}

/** Complete captured text, with only one bounded page in the browser layout. */
export function DiagnosticText({ text }: { text: string }) {
  const ends = useMemo(() => pageEnds(text), [text]);
  const [page, setPage] = useState(0);
  const current = Math.min(page, ends.length - 2), count = ends.length - 1;
  return <>
    <p>Captured when Diagnostics was opened. Close and reopen it to refresh. All diagnostic text is available across these pages; your saved game is unchanged.</p>
    <div role="group" aria-label="Diagnostic pages">
      <button className="quiet" disabled={current === 0} onClick={() => setPage(current - 1)}>Previous diagnostics page</button>
      <span role="status">Page {current + 1} of {count}</span>
      <button className="quiet" disabled={current + 1 === count} onClick={() => setPage(current + 1)}>Next diagnostics page</button>
    </div>
    <pre data-native-diagnostics data-diagnostics-page={current} data-diagnostics-pages={count}>{text.slice(ends[current], ends[current + 1])}</pre>
  </>;
}
