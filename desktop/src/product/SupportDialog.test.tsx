// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ApplicationView } from "./ApplicationView";
import type { GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import chess from "../__fixtures__/shared-chess-standard.json";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
afterEach(cleanup);
function app(value: unknown) {
 const snapshot = value as GameSnapshot;
 const session: GameSession = { snapshot, busy:false, error:"native failure detail", closeRequested:false, command:vi.fn(), file:vi.fn(), newGame:vi.fn(), cancelClose:vi.fn(), finishClose:vi.fn(), requestClose:vi.fn() };
 const props = {snapshot,session,onNewGame:vi.fn(),onOpen:vi.fn(),blocked:false};
 return {...render(<ApplicationView {...props}/>),props,session};
}
it.each([chess,shogi,xiangqi])("keeps common support outside the sidebar and retains full native evidence and notices", value => {
 const {container,session}=app(value);
 const support=screen.getByRole("button",{name:"About mShogi"});
 expect(support.closest(".shell-header")).toBeTruthy();
 expect(container.querySelector(".shell-sidebar [data-native-diagnostics]")).toBeNull();
 expect(container.textContent).not.toMatch(/UNTIMED|Hollow rings|reconstructed|Native rules/);
 fireEvent.click(support);
 const dialog=screen.getByRole("dialog",{name:"About mShogi"});
 expect(container.querySelector(".shell-content")?.hasAttribute("inert")).toBe(true);
 expect(within(dialog).getByText("Diagnostics")).toBeTruthy();
 const payload=JSON.parse(dialog.querySelector("[data-native-diagnostics]")!.textContent!);
 expect(payload.context).toEqual(session.snapshot!.context);
 expect(payload.token).toEqual(session.snapshot!.token);
 expect(payload.capacity).toEqual(session.snapshot!.capacity);
 expect(payload.moves).toEqual(session.snapshot!.moves);
 expect(payload.error).toBe("native failure detail");
 expect(within(dialog).getByText("Font license")).toBeTruthy();
 expect(dialog.textContent).toContain("SIL OPEN FONT LICENSE");
 expect(dialog.querySelector('a[download="mshogi-pieces-OFL.txt"]')).toBeTruthy();
 fireEvent.click(within(dialog).getByRole("button",{name:"Quit application"}));
 expect(session.requestClose).toHaveBeenCalledExactlyOnceWith();
});
it("traps keyboard navigation through visible disclosures, skips hidden license links and restores About on Escape",async()=>{
 app(chess);const user=userEvent.setup();
 const support=screen.getByRole("button",{name:"About mShogi"}); support.focus();await user.keyboard("{Enter}");
 const dialog=screen.getByRole("dialog");const diagnostics=within(dialog).getByText("Diagnostics"),license=within(dialog).getByText("Font license");
 expect(document.activeElement).toBe(within(dialog).getByRole("button",{name:"Close"}));
 await user.tab();expect(document.activeElement).toBe(diagnostics);
 await user.tab();expect(document.activeElement).toBe(license);
 await user.tab();expect(document.activeElement).toBe(within(dialog).getByRole("button",{name:"Quit application"}));
 await user.tab({shift:true});expect(document.activeElement).toBe(license);
 // jsdom lacks the browser default Enter action for summary; actual package
 // verification exercises the native keyboard activation. Model its open state here.
 fireEvent.click(license);expect(license.parentElement?.hasAttribute("open")).toBe(true);
 license.focus();fireEvent.keyDown(dialog,{key:"Tab"});expect(document.activeElement?.textContent).toBe("Save the font license");
 await user.keyboard("{Escape}");expect(screen.queryByRole("dialog")).toBeNull();
 await waitFor(()=>expect(document.activeElement).toBe(support));
});
it("yields About to the existing outer native close guard without executing a game command",()=>{
 const {props,rerender,session}=app(shogi);fireEvent.click(screen.getByRole("button",{name:"About mShogi"}));
 rerender(<ApplicationView {...props} blocked/>);
 expect(screen.queryByRole("dialog")).toBeNull();expect(session.command).not.toHaveBeenCalled();
});
