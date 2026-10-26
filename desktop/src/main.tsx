import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ProductApp } from "./product/ProductApp";
import "./style.css";

declare const __MSHOGI_DESKTOP_VERIFICATION__: boolean;

const root = createRoot(document.getElementById("root")!);
if (__MSHOGI_DESKTOP_VERIFICATION__) {
  void import("./verification/bootstrap")
    .then(({ mountVerification }) => mountVerification(root))
    .catch((reason: unknown) => {
      root.render(<main className="loading"><h1>Verification startup failed</h1><p role="alert">{String(reason)}</p></main>);
    });
} else {
  root.render(<StrictMode><ProductApp /></StrictMode>);
}
