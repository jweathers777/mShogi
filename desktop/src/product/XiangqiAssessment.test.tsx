// @vitest-environment jsdom
import { afterEach, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { XiangqiAssessment } from "./XiangqiAssessment";
import type { XiangqiExactEvidence, XiangqiSnapshot } from "./native";
import standard from "../__fixtures__/shared-xiangqi-standard.json";
import fastProof from "../__fixtures__/shared-xiangqi-fast-proof.json";
afterEach(cleanup);
it.each([[standard,"Unknown","Unknown does not establish a win or draw"],[fastProof,"No natural board win is possible","Sound fast proof: neither side"]])("keeps the supplied fast assessment understandable outside raw diagnostics",(value,label,meaning)=>{
 render(<XiangqiAssessment snapshot={value as unknown as XiangqiSnapshot}/>);
 expect(screen.getByText(`Board-win assessment: ${label}`)).toBeTruthy();
 expect(document.body.textContent).toContain(meaning);
});
it("distinguishes saved exact proof, positive witness, unknown, cancellation and failure without changing the current game",()=>{
 // Presentation-only observations: native replay/validation remains covered by archive tests.
 const verdicts: XiangqiExactEvidence["report"]["verdict"][]=[{kind:"neither_can_win"},{kind:"natural_win",winner:"black",reason:"stalemate",witness:["a0a1"]},{kind:"unknown",reason:"states"},{kind:"unknown",reason:"cancelled"},{kind:"failure",reason:"original native detail"}];
 const records=verdicts.map((verdict,index)=>({event_count:index+4,report:{verdict,work:{states:3,transitions:2}}} as XiangqiExactEvidence));
 const snapshot={...standard,xiangqi:{...standard.xiangqi,exact_observations:records}} as unknown as XiangqiSnapshot;
 const original=JSON.stringify(snapshot);render(<XiangqiAssessment snapshot={snapshot}/>);
 expect(screen.getByText("Exact proof: neither side can reach a natural board win.")).toBeTruthy();
 expect(screen.getByText("Black has a possible natural board win. This is not a forced win.")).toBeTruthy();
 expect(screen.getByText("Unknown · exploration incomplete. No conclusion was established.")).toBeTruthy();
 expect(screen.getByText("Cancelled · Unknown. No conclusion was established.")).toBeTruthy();
 expect(screen.getByText("Analysis failed. No conclusion was established.")).toBeTruthy();
 expect(screen.getByText("Historical results for their recorded positions. They do not change your current game.")).toBeTruthy();
 expect(screen.getByText("Saved analysis · event 5")).toBeTruthy();expect(screen.getByText("Possible route · not played")).toBeTruthy();
 expect(JSON.stringify(snapshot)).toBe(original);
});
