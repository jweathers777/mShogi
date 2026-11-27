import type { KeyboardEvent } from "react";

export function cycleTabFocus(event: KeyboardEvent<HTMLElement>, selector = "button:not(:disabled)") {
  const dialog = event.currentTarget;
  const items = [...dialog.querySelectorAll<HTMLElement>(selector)].filter(item => {
    for (let ancestor = item.parentElement; ancestor && ancestor !== dialog; ancestor = ancestor.parentElement) {
      if (ancestor.hidden || ancestor.hasAttribute("inert")) return false;
      if (ancestor instanceof HTMLDetailsElement && !ancestor.open && item !== ancestor.querySelector(":scope > summary")) return false;
    }
    return !item.hidden;
  }).sort((a, b) => a.compareDocumentPosition(b) & Node.DOCUMENT_POSITION_FOLLOWING ? -1 : 1);
  const index = items.indexOf(document.activeElement as HTMLElement);
  event.preventDefault();
  items[(index + (event.shiftKey ? -1 : 1) + items.length) % items.length]?.focus();
}

/** Reveal the whole target and its outward focus ring within one viewport. */
export function revealInViewport(viewport: HTMLElement, element: HTMLElement) {
  const frame = viewport.getBoundingClientRect(), target = element.getBoundingClientRect(), margin = 8;
  if (target.top - margin < frame.top) viewport.scrollTop -= frame.top - target.top + margin;
  else if (target.bottom + margin > frame.bottom) viewport.scrollTop += target.bottom + margin - frame.bottom;
}
