import { useState } from "react";

// QA-only, read-only observation of the actual production DOM. It neither
// dispatches commands nor supplies choices, effects or engine observations.
function measure() {
  const svg = document.querySelector<SVGSVGElement>(".tenjiku-board-frame svg.chess-board");
  if (!svg) return { error: "No production Tenjiku board" };
  const rect = (element: Element) => {
    const r = element.getBoundingClientRect();
    return { x: r.x, y: r.y, width: r.width, height: r.height };
  };
  const cells = [...svg.querySelectorAll<SVGRectElement>(".square-fill")];
  const visible = (element: Element) => {
    const r = element.getBoundingClientRect();
    if (r.width <= 0 || r.height <= 0) return false;
    if (r.left < 0 || r.top < 0 || r.right > innerWidth + .5 || r.bottom > innerHeight + .5) return false;
    for (let parent = element.parentElement; parent; parent = parent.parentElement) {
      const style = getComputedStyle(parent), p = parent.getBoundingClientRect();
      if (/(hidden|clip|auto|scroll)/.test(style.overflowX) && (r.left < p.left - .5 || r.right > p.right + .5)) return false;
      if (/(hidden|clip|auto|scroll)/.test(style.overflowY) && (r.top < p.top - .5 || r.bottom > p.bottom + .5)) return false;
    }
    return true;
  };
  const glyphs = [...svg.querySelectorAll<SVGTextElement>(".tenjiku-face text")].map(glyph => {
    const matrix = glyph.getScreenCTM();
    return { text: glyph.textContent, font_family: getComputedStyle(glyph).fontFamily, box: rect(glyph), logical_em: parseFloat(getComputedStyle(glyph).fontSize) * (matrix ? Math.hypot(matrix.a, matrix.b) : 0) };
  });
  const ancestors = [];
  for (let parent: Element | null = svg; parent; parent = parent.parentElement) {
    const style = getComputedStyle(parent);
    ancestors.push({ tag: parent.tagName, class: parent.getAttribute("class"), box: rect(parent),
      overflow_x: style.overflowX, overflow_y: style.overflowY, scroll_left: parent.scrollLeft, scroll_top: parent.scrollTop });
  }
  return { schema: "mshogi.tenjiku.actual-layout.v1", content: { width: innerWidth, height: innerHeight }, device_pixel_ratio: devicePixelRatio,
    board: rect(svg), cells: cells.length, visible_cells: cells.filter(visible).length, cell: cells[0] ? rect(cells[0]) : null,
    font_loaded: [...document.fonts].some(face => face.family.replaceAll('"', '') === "mShogi Tenjiku Pieces" && face.status === "loaded"),
    font_check: document.fonts.check('47px "mShogi Tenjiku Pieces"', glyphs.map(g => g.text).join("")), glyph_count: glyphs.length,
    minimum_glyph_em: Math.min(...glyphs.map(g => g.logical_em)), maximum_glyph_em: Math.max(...glyphs.map(g => g.logical_em)),
    glyphs, ancestors };
}
export function TenjikuLayout() {
  const [result, setResult] = useState<ReturnType<typeof measure> | null>(null);
  return <details className="tenjiku-layout-measurements">
    <summary onClick={() => setResult(measure())}>Layout measurements</summary>
    <textarea aria-label="Actual Tenjiku layout measurements" readOnly value={result ? JSON.stringify(result, (_key, value: unknown) => typeof value === "number" ? Math.round(value * 1000) / 1000 : value) : ""}
      style={{ width: "70vw", height: "45vh", color: "#fff", background: "#172d36" }} />
  </details>;
}
