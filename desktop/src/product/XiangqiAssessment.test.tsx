// @vitest-environment jsdom
import type { XiangqiSnapshot } from "../sessionTypes";
import { afterEach, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { XiangqiAssessment } from "./XiangqiAssessment";

import standard from "../__fixtures__/shared-xiangqi-standard.json";
import fastProof from "../__fixtures__/shared-xiangqi-fast-proof.json";
afterEach(cleanup);
it.each([[standard,"Unknown","Unknown does not establish a win or draw"],[fastProof,"No natural board win is possible","Sound fast proof: neither side"]])("keeps the supplied fast assessment understandable outside raw diagnostics",(value,label,meaning)=>{
 render(<XiangqiAssessment snapshot={value as unknown as XiangqiSnapshot}/>);
 expect(screen.getByText(`Board-win assessment: ${label}`)).toBeTruthy();
 expect(document.body.textContent).toContain(meaning);
});
