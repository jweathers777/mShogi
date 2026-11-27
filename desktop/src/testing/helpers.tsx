import { render } from "@testing-library/react";
import type { ComponentType } from "react";
import { vi } from "vitest";
import type { GameSnapshot } from "../sessionTypes";
import { ApplicationView } from "../product/ApplicationView";
import { GameBoard } from "../product/GameBoard";
import type { GameSession } from "../product/useGameSession";

export function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(done => { resolve = done; });
  return { promise, resolve };
}

export function fakeSession(snapshot: GameSnapshot, overrides: Partial<GameSession> = {}) {
  return {
    snapshot, busy: false, error: "", closeRequested: false,
    command: vi.fn().mockResolvedValue(snapshot),
    file: vi.fn().mockResolvedValue(snapshot),
    newGame: vi.fn().mockResolvedValue(snapshot),
    cancelClose: vi.fn().mockResolvedValue(true),
    finishClose: vi.fn().mockResolvedValue(true),
    requestClose: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  } satisfies GameSession;
}

export function renderApplication(snapshot: GameSnapshot, session = fakeSession(snapshot), blocked = false) {
  const props = { snapshot, session, blocked, onNewGame: vi.fn(), onOpen: vi.fn() };
  return { ...render(<ApplicationView {...props} />), props, session };
}

export function renderGameBoard(snapshot: GameSnapshot, command = vi.fn()) {
  return { ...render(<GameBoard snapshot={snapshot} disabled={false} inspectionDisabled={false} command={command} />), command };
}

export function renderBoard<P>(BoardView: ComponentType<NoInfer<P> & { disabled: boolean; onPlay: (action: string) => void }>, options: P) {
  const onPlay = vi.fn();
  const props = { ...options, disabled: false, onPlay };
  return { ...render(<BoardView {...props} />), props, onPlay };
}

// Coordinate labels are. Use the rendered label directly to avoid
// recomputing hundreds of SVG accessible names per gesture in jsdom.
export function square(key: string, container: ParentNode = document): SVGGElement {
  const element = container.querySelector<SVGGElement>('svg [role="button"][aria-label^="' + key + ',"]');
  if (!element) throw new Error("Missing game board coordinate " + key);
  return element;
}
